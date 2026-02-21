use std::{collections::{HashMap, HashSet}, mem::take, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{constant_value::{TanukiConstantValue, TanukiType}, expression::{TanukiExpression, TanukiExpressionVariant}, global_constant::TanukiGlobalConstant, module::TanukiModule}, traits::module::Module};

impl TanukiModule {
	pub fn get_global_items_for_module(&self, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>) -> Result<(), ErrorAt> {
		for global_constant in self.global_constants.iter() {
			global_items_to_const_compile_for_this_module.insert(global_constant.as_ref().unwrap().name.clone());
		}
		for function in self.functions.iter() {
			if !global_items_to_const_compile_for_this_module.insert(function.name.clone()) {
				return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(function.start_line), Some(function.start_column), None));
			}
		}
		for import in self.imports.iter() {
			if !global_items_to_const_compile_for_this_module.insert(import.name.clone()) {
				return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(import.start_line), Some(import.start_column), None));
			}
		}
		for link in self.links.iter() {
			if !global_items_to_const_compile_for_this_module.insert(link.name.clone()) {
				return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(link.start_line), Some(link.start_column), None));
			}
		}
		Ok(())
	}

	pub fn const_compile_globals(
		&mut self, _main: &mut Main, global_items_const_compiled: &mut HashSet<(Box<str>, Box<Path>)>, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		_modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], module_path: &Path,
	) -> Result<bool, ErrorAt> {
		// Return if all global items have been const-compiled
		if global_items_to_const_compile_for_this_module.is_empty() {
			return Ok(true);
		}
		// Const-compile globals that we can
		'a: for x in 0..self.global_constants.len() {
			let global_constant = &mut self.global_constants[x];
			// Make sure the constant has not already been const-compiled
			if global_constant.as_ref().unwrap().has_been_const_compiled {
				continue 'a;
			}
			// Make sure all dependencies have been const-compiled first
			for dependency in global_constant.as_ref().unwrap().depends_on.iter() {
				if global_items_to_const_compile_for_this_module.contains(dependency) {
					continue 'a;
				}
			}
			let mut global_constant_removed = take(global_constant).unwrap();
			// Const-compile
			global_constant_removed.const_compile(&mut self.global_constants)?;
			let global_constant = &mut self.global_constants[x];
			*global_constant = Some(global_constant_removed);
			if !matches!(global_constant.as_ref().unwrap().value_expression.variant, TanukiExpressionVariant::Constant(_)) {
				return Err(Error::UnableToConstCompile.at(Some(global_constant.as_ref().unwrap().start_line), Some(global_constant.as_ref().unwrap().start_column), None));
			}
			// Update lists
			global_constant.as_mut().unwrap().has_been_const_compiled = true;
			global_items_to_const_compile_for_this_module.remove(&global_constant.as_ref().unwrap().name);
			global_items_const_compiled.insert((global_constant.as_ref().unwrap().name.clone(), module_path.into()));
		}
		// Check for duplicates without the same value
		if global_items_to_const_compile_for_this_module.is_empty() {
			for (x, global_constant_x) in self.global_constants.iter().enumerate() {
				let global_constant_x = global_constant_x.as_ref().unwrap();
				for (y, global_constant_y) in self.global_constants.iter().enumerate() {
					let global_constant_y = global_constant_y.as_ref().unwrap();
					if x == y || global_constant_x.name != global_constant_y.name {
						continue;
					}
					match (&global_constant_x.value_expression.variant, &global_constant_y.value_expression.variant) {
						(TanukiExpressionVariant::Constant(x_value), TanukiExpressionVariant::Constant(y_value)) => {
							if x_value != y_value {
								return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(global_constant_x.start_line), Some(global_constant_x.start_column), None));
							}
						}
						(_, _) => unreachable!(),
					}
				}
			}
		}
		// Return
		Ok(global_items_to_const_compile_for_this_module.is_empty())
	}
}

impl TanukiGlobalConstant {
	pub fn const_compile(&mut self, module_global_items_const_compiled: &mut [Option<TanukiGlobalConstant>]) -> Result<(), ErrorAt> {
		self.value_expression.const_compile_r_value(module_global_items_const_compiled, &mut Vec::new())?;
		Ok(())
	}
}

impl TanukiExpression {
	pub fn const_compile_r_value(
		&mut self, module_global_items_const_compiled: &mut [Option<TanukiGlobalConstant>], local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, TanukiConstantValue)>>
	) -> Result<Option<TanukiConstantValue>, ErrorAt> {
		// Unpack
		let Self { variant, start_line, start_column, .. } = self;
		// Try to const-compile
		let const_compiled_value = match variant {
			TanukiExpressionVariant::Constant(value) => Some(value.clone()),
			TanukiExpressionVariant::Variable(name) => {
				// Get if there is already a local variable with this name
				let mut variable = None;
				for local_variable_scope in local_variables {
					variable = local_variable_scope.get_mut(name);
					if variable.is_some() {
						break;
					}
				}
				match variable {
					// Return the value if there is
					Some((_, value)) => Some(value.clone()),
					// Else read the global variable
					None => 'a: {
						for global_constant in module_global_items_const_compiled.iter() {
							if let Some(global_constant) = global_constant && global_constant.name == *name {
								match &global_constant.value_expression.variant {
									TanukiExpressionVariant::Constant(value) => break 'a Some(value.clone()),
									_ => {},
								}
							}
						}
						return Err(Error::VariableNotFound.at(Some(*start_line), Some(*start_column), None));
					}
				}
			}
			_ => None,
		};
		// If complication was done, replace this with the compiled constant
		if let Some(const_compiled_value) = &const_compiled_value && !matches!(variant, TanukiExpressionVariant::Constant(..)) {
			self.variant = TanukiExpressionVariant::Constant(const_compiled_value.clone());
		}
		// Return
		Ok(const_compiled_value)
	}

	pub fn const_compile_l_value(&mut self, local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, TanukiConstantValue)>>) -> Result<Option<CompileTimeLValue>, ErrorAt> {
		let Self { variant, .. } = self;
		Ok(match variant {
			TanukiExpressionVariant::Constant(_) => todo!(),
			TanukiExpressionVariant::Variable(name) => {
				// Get if there is already a local variable with this name
				let mut variable = None;
				let mut block_level = 0;
				for (x, local_variable_scope) in local_variables.iter_mut().enumerate() {
					variable = local_variable_scope.get_mut(name);
					block_level = x;
					if variable.is_some() {
						break;
					}
				}
				//
				match variable {
					None => {
						local_variables.last_mut().unwrap().insert(name.clone(), (TanukiType::Any, TanukiConstantValue::Void));
						Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level: local_variables.len() })
					}
					_ => Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level })
				}
			}
			_ => None,
		})
	}
}

pub enum CompileTimeLValue {
	LocalVariable { name: Box<str>, block_level: usize },
}