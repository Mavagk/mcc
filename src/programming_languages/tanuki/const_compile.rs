use std::{collections::{HashMap, HashSet}, mem::take, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, global_constant::TanukiGlobalConstant, module::TanukiModule, t_type::TanukiType}, traits::module::Module};

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
		self.value_expression.const_compile_r_value(module_global_items_const_compiled, &mut Vec::new(), &TanukiType::Any)?;
		Ok(())
	}
}

impl TanukiExpression {
	pub fn const_compile_r_value(
		&mut self, module_global_items_const_compiled: &mut [Option<TanukiGlobalConstant>], local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, TanukiCompileTimeValue)>>,
		result_type: &TanukiType,
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
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
			TanukiExpressionVariant::U(sub_expressions) | TanukiExpressionVariant::I(sub_expressions) |TanukiExpressionVariant::F(sub_expressions) => {
				if sub_expressions.len() != 1 {
					return Err(Error::UnexpectedBuiltinFunctionArgumentCount {
						expected_min: Some(1), expected_max: Some(1), got: sub_expressions.len()
					}.at(Some(*start_line), Some(*start_column), None));
				}
				let sub_expression = &mut sub_expressions[0];
				let argument = sub_expression.const_compile_r_value_forced(module_global_items_const_compiled, local_variables, &TanukiType::CompileTimeInt)?;
				let argument = match argument {
					TanukiCompileTimeValue::CompileTimeInt(argument) => argument,
					_ => unreachable!(),
				};
				match variant {
					TanukiExpressionVariant::U(_) => {
						let bit_width: u8 = match (&argument).try_into() {
							Ok(argument) => argument,
							Err(_) => return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None)),
						};
						if !matches!(bit_width, 8 | 16 | 32 | 64) {
							return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None));
						}
						Some(TanukiCompileTimeValue::Type(TanukiType::U(bit_width)))
					}
					TanukiExpressionVariant::I(_) => {
						let bit_width: u8 = match (&argument).try_into() {
							Ok(argument) => argument,
							Err(_) => return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None)),
						};
						if !matches!(bit_width, 8 | 16 | 32 | 64) {
							return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None));
						}
						Some(TanukiCompileTimeValue::Type(TanukiType::I(bit_width)))
					}
					TanukiExpressionVariant::F(_) => {
						let bit_width: u8 = match (&argument).try_into() {
							Ok(argument) => argument,
							Err(_) => return Err(Error::InvalidFloatBitWidth(argument).at(Some(*start_line), Some(*start_column), None)),
						};
						if !matches!(bit_width, 32 | 64) {
							return Err(Error::InvalidFloatBitWidth(argument).at(Some(*start_line), Some(*start_column), None));
						}
						Some(TanukiCompileTimeValue::Type(TanukiType::F(bit_width)))
					}
					_ => unreachable!(),
				}
			}
			TanukiExpressionVariant::TypeAndValue(type_expression, castee_expression) => {
				let type_expression_parsed = match type_expression.const_compile_r_value_forced(module_global_items_const_compiled, local_variables, &TanukiType::Type)? {
					TanukiCompileTimeValue::Type(type_expression_parsed) => type_expression_parsed,
					_ => unreachable!(),
				};
				let castee_expression_parsed = castee_expression.const_compile_r_value(module_global_items_const_compiled, local_variables, &TanukiType::Any)?;
				match castee_expression_parsed {
					Some(castee_expression_parsed) => Some(
						castee_expression_parsed.cast_to(&type_expression_parsed, false).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?
					),
					None => None,
				}
			}
			TanukiExpressionVariant::Negation(operand) => {
				match operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)? {
					Some(operand) => (-operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					None => None,
				}
			}
			TanukiExpressionVariant::Addition(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
					rhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand + rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Subtraction(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
					rhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand - rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Multiplication(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
					rhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand * rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Division(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
					rhs_operand.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand / rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			_ => None,
		};
		// Cast
		let const_compiled_value = match const_compiled_value {
			Some(const_compiled_value) => Some(
				const_compiled_value.cast_to(result_type, false).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?
			),
			None => None,
		};
		// If complication was done, replace this with the compiled constant
		if let Some(const_compiled_value) = &const_compiled_value {
			self.variant = TanukiExpressionVariant::Constant(const_compiled_value.clone());
		}
		// Return
		Ok(const_compiled_value)
	}

	pub fn const_compile_l_value(&mut self, local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, TanukiCompileTimeValue)>>) -> Result<Option<CompileTimeLValue>, ErrorAt> {
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
						local_variables.last_mut().unwrap().insert(name.clone(), (TanukiType::Any, TanukiCompileTimeValue::Void));
						Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level: local_variables.len() })
					}
					_ => Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level })
				}
			}
			_ => None,
		})
	}

	pub fn const_compile_r_value_forced(
		&mut self, module_global_items_const_compiled: &mut [Option<TanukiGlobalConstant>], local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, TanukiCompileTimeValue)>>,
		result_type: &TanukiType,
	) -> Result<TanukiCompileTimeValue, ErrorAt> {
		match self.const_compile_r_value(module_global_items_const_compiled, local_variables, result_type)? {
			Some(value) => Ok(value),
			None => Err(Error::UnableToConstCompile.at(Some(self.start_line), Some(self.start_column), None)),
		}
	}
}

pub enum CompileTimeLValue {
	LocalVariable { name: Box<str>, block_level: usize },
}