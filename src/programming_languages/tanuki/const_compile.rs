use std::{collections::HashSet, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{expression::TanukiExpressionVariant, module::TanukiModule}, traits::module::Module};

impl TanukiModule {
	pub fn get_global_items_for_module(&self, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>) -> Result<(), ErrorAt> {
		for global_constant in self.global_constants.iter() {
			global_items_to_const_compile_for_this_module.insert(global_constant.name.clone());
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
		'a: for global_constant in self.global_constants.iter_mut() {
			// Make sure the constant has not already been const-compiled
			if global_constant.has_been_const_compiled {
				continue 'a;
			}
			// Make sure all dependencies have been const-compiled first
			for dependency in global_constant.depends_on.iter() {
				if global_items_to_const_compile_for_this_module.contains(dependency) {
					continue 'a;
				}
			}
			// Const-compile
			global_constant.const_compile()?;
			if !matches!(global_constant.value_expression.variant, TanukiExpressionVariant::Constant(_)) {
				return Err(Error::UnableToConstCompile.at(Some(global_constant.start_line), Some(global_constant.start_column), None));
			}
			// Update lists
			global_constant.has_been_const_compiled = true;
			global_items_to_const_compile_for_this_module.remove(&global_constant.name);
			global_items_const_compiled.insert((global_constant.name.clone(), module_path.into()));
		}
		// Check for duplicates without the same value
		if global_items_to_const_compile_for_this_module.is_empty() {
			for (x, global_constant_x) in self.global_constants.iter().enumerate() {
				for (y, global_constant_y) in self.global_constants.iter().enumerate() {
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