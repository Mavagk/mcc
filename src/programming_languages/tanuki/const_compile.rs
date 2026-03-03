use std::{any::Any, collections::HashMap, mem::take, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, global_constant::TanukiGlobalConstant, module::TanukiModule, t_type::TanukiType}, traits::module::Module};

impl TanukiModule {
	/*pub fn get_global_items_for_module(&self, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>) -> Result<(), ErrorAt> {
		for global_constant in self.global_constants.iter() {
			global_items_to_const_compile_for_this_module.insert(global_constant.as_ref().unwrap().name.clone());
		}
		for function in self.functions.iter() {
			if !global_items_to_const_compile_for_this_module.insert(function.as_ref().unwrap().name.clone()) {
				return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(function.as_ref().unwrap().start_line), Some(function.as_ref().unwrap().start_column), None));
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
	}*/

	pub fn const_compile_globals(
		//&mut self, _main: &mut Main, global_items_const_compiled: &mut HashSet<(Box<str>, Box<Path>)>, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		//modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], module_path: &Path,
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], module_path: &Path, was_complication_done: &mut bool,
	//) -> Result<bool, ErrorAt> {
	) -> Result<(), ErrorAt> {
		// Return if all global items have been const-compiled
		//if global_items_to_const_compile_for_this_module.is_empty() {
		//	return Ok(true);
		//}
		// Const-compile globals that we can
		for x in 0..self.global_constants.len() {
			let global_constant = &mut self.global_constants[x];
			// Make sure the constant has not already been const-compiled
			//if global_constant.as_ref().unwrap().has_been_const_compiled {
			//	continue 'a;
			//}
			// Make sure all dependencies have been const-compiled first
			//for dependency in global_constant.as_ref().unwrap().depends_on.iter() {
			//	if global_items_to_const_compile_for_this_module.contains(dependency) {
			//		continue 'a;
			//	}
			//}
			let mut global_constant_removed = take(global_constant).unwrap();
			// Const-compile
			//let mut where_extra_dependencies_found = false;
			//global_constant_removed.const_compile(&mut self.global_constants, global_items_to_const_compile_for_this_module, &mut where_extra_dependencies_found)?;
			//global_constant_removed.const_compile(modules, self, global_items_to_const_compile_for_this_module, &mut where_extra_dependencies_found)?;
			global_constant_removed.const_compile(main, modules, self, module_path, was_complication_done)?;
			let global_constant = &mut self.global_constants[x];
			*global_constant = Some(global_constant_removed);
			//if where_extra_dependencies_found {
			//	continue 'a;
			//}
			if !matches!(global_constant.as_ref().unwrap().value_expression.variant, TanukiExpressionVariant::Constant(_)) {
				return Err(Error::UnableToConstCompile.at(Some(global_constant.as_ref().unwrap().start_line), Some(global_constant.as_ref().unwrap().start_column), None));
			}
			// Update lists
			//global_constant.as_mut().unwrap().has_been_const_compiled = true;
			//global_items_to_const_compile_for_this_module.remove(&global_constant.as_ref().unwrap().name);
			//global_items_const_compiled.insert((global_constant.as_ref().unwrap().name.clone(), module_path.into()));
		}
		// Const-compile functions that we can
		for x in 0..self.functions.len() {
			let function = &mut self.functions[x];
			// Make sure the function has not already been const-compiled
			//if function.as_ref().unwrap().is_const_compiled {
			//	continue 'a;
			//}
			// Make sure all dependencies have been const-compiled first
			//for dependency in function.as_ref().unwrap().depends_on_for_execution.iter() {
			//	if global_items_to_const_compile_for_this_module.contains(dependency) {
			//		continue 'a;
			//	}
			//}
			let mut function_removed = take(function).unwrap();
			// Const-compile
			//let mut where_extra_dependencies_found = false;
			//function_removed.const_compile(modules, self, global_items_to_const_compile_for_this_module, &mut where_extra_dependencies_found)?;
			function_removed.const_compile(main, modules, self, module_path, was_complication_done)?;
			let function = &mut self.functions[x];
			*function = Some(function_removed);
			//if where_extra_dependencies_found {
			//	continue 'a;
			//}
			//if !matches!(global_constant.as_ref().unwrap().value_expression.variant, TanukiExpressionVariant::Constant(_)) {
			//	return Err(Error::UnableToConstCompile.at(Some(global_constant.as_ref().unwrap().start_line), Some(global_constant.as_ref().unwrap().start_column), None));
			//}
			// Update lists
			//function.as_mut().unwrap().is_const_compiled = true;
			//global_items_to_const_compile_for_this_module.remove(&function.as_ref().unwrap().name);
			//global_items_const_compiled.insert((function.as_ref().unwrap().name.clone(), module_path.into()));
		}
		// Check for duplicate constants without the same value
		// TODO: Redo
		/*if global_items_to_const_compile_for_this_module.is_empty() {
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
		}*/
		// Return
		Ok(())
		//Ok(global_items_to_const_compile_for_this_module.is_empty())
	}
}

impl TanukiGlobalConstant {
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path, was_complication_done: &mut bool,
		//&mut self, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule,
		//global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>, where_extra_dependencies_found: &mut bool,
	) -> Result<(), ErrorAt> {
		self.value_expression.const_compile_r_value(
			main, modules, this_module, module_path, was_complication_done, &mut Vec::new(), &TanukiType::Any
			//modules, this_module, global_items_to_const_compile_for_this_module,&mut Vec::new(), &TanukiType::Any, where_extra_dependencies_found
		)?;
		Ok(())
	}
}

impl TanukiFunction {
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path, was_complication_done: &mut bool,
		//&mut self, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>, where_extra_dependencies_found: &mut bool
	) -> Result<(), ErrorAt> {
		let mut local_variables = Vec::new();
		local_variables.push(HashMap::new());
		for parameter in self.parameters.iter_mut() {
			if let Some(t_type) = &mut parameter.t_type {
				let t_type = t_type.const_compile_r_value(
					main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type
					//modules, this_module, global_items_to_const_compile_for_this_module, &mut local_variables, &TanukiType::Type, where_extra_dependencies_found
				)?;
				//if *where_extra_dependencies_found {
				//	return Ok(());
				//}
				let t_type = match t_type.unwrap() {
					TanukiCompileTimeValue::Type(t_type) => t_type,
					_ => return Ok(()),
				};
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), (t_type, None));
			}
			else {
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), (TanukiType::Any, None));
			}
		}
		let return_type = match &mut self.return_type {
			Some(return_type) => return_type.const_compile_r_value(
				main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type
				//modules, this_module, global_items_to_const_compile_for_this_module, &mut Vec::new(), &TanukiType::Type, where_extra_dependencies_found
			)?,
			None => None,
		};
		//if *where_extra_dependencies_found {
		//	return Ok(())
		//}
		let return_type = match return_type {
			Some(TanukiCompileTimeValue::Type(return_type)) => return_type,
			None => TanukiType::Any,
			_ => return Ok(()),
		};
		//self.body.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, &mut local_variables, &return_type, where_extra_dependencies_found)?;
		self.body.const_compile_r_value(main, modules, this_module, module_path, was_complication_done, &mut local_variables, &return_type)?;
		Ok(())
	}
}

impl TanukiExpression {
	pub fn const_compile_r_value(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType
		//&mut self, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		//local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, where_extra_dependencies_found: &mut bool,
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		// Unpack
		let Self { variant, start_line, start_column, .. } = self;
		// Try to const-compile
		let const_compiled_value = match variant {
			TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Function(target_function_name, target_module_path)) => 'a: {
				for (module_path, _, module) in modules.iter() {
					if module_path != target_module_path {
						continue;
					}
					let module: &TanukiModule = if &**module_path == this_module_path {
						this_module
					}
					else {
						match ((&**module.as_ref().unwrap()) as &dyn Any).downcast_ref() {
							Some(module) => module,
							None => return Err(Error::Unimplemented("Linking to non-Tanuki modules".into()).at(None, None, None)),
						}
					};
					for function in module.functions.iter() {
						// TODO: this function
						let function = match function {
							Some(function) => function,
							None => continue,
						};
						if &function.name != target_function_name {
							continue;
						}
						let return_type = match &function.return_type {
							Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => value,
							_ => return Ok(None),
						};
						let mut parameter_types = Vec::new();
						for parameter in function.parameters.iter() {
							match &parameter.t_type {
								Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => parameter_types.push(value.clone()),
								_ => return Ok(None),
							};
						}
						*was_complication_done = true;
						break 'a Some(TanukiCompileTimeValue::FunctionPointer(target_function_name.clone(), target_module_path.clone(), return_type.clone().into(), parameter_types.into()));
					}
					return Ok(None);
				}
				return Ok(None);
			},
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
					Some((_, value)) => value.clone(),
					// Else read the global variable
					None => 'a: {
						for global_constant in this_module.global_constants.iter() {
							if let Some(global_constant) = global_constant && global_constant.name == *name {
								match &global_constant.value_expression.variant {
									TanukiExpressionVariant::Constant(value) => break 'a Some(value.clone()),
									_ => return Ok(None),
								}
							}
						}
						//if global_items_to_const_compile_for_this_module.contains(name) {
						//	*where_extra_dependencies_found = true;
						//	return Ok(None);
						//}
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
				//let argument = sub_expression.const_compile_r_value_forced(
				//	modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::CompileTimeInt, where_extra_dependencies_found
				//)?;
				let argument = sub_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::CompileTimeInt,
				)?;
				let argument = match argument {
					Some(TanukiCompileTimeValue::CompileTimeInt(argument)) => argument,
					_ => return Ok(None),
				};
				*was_complication_done = true;
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
				//let type_expression_parsed = match type_expression.const_compile_r_value_forced(
				//	modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Type, where_extra_dependencies_found
				//)
				let type_expression_parsed = match type_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Type,
					//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Type, where_extra_dependencies_found
				)? {
					Some(TanukiCompileTimeValue::Type(type_expression_parsed)) => type_expression_parsed,
					_ => return Ok(None),
				};
				let castee_expression_parsed = castee_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any
					//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found
				)?;
				match castee_expression_parsed {
					Some(castee_expression_parsed) => {
						*was_complication_done = true;
						Some(
							castee_expression_parsed.cast_to(&type_expression_parsed, false).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?
						)
					},
					None => None,
				}
			}
			// TODO: Operators
			/*TanukiExpressionVariant::Negation(operand) => {
				match operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)? {
					Some(operand) => (-operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					None => None,
				}
			}
			TanukiExpressionVariant::Addition(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
					rhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand + rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Subtraction(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
					rhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand - rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Multiplication(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
					rhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand * rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}
			TanukiExpressionVariant::Division(lhs_operand, rhs_operand) => {
				match (
					lhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
					rhs_operand.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found)?,
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand / rhs_operand).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?,
					_ => None,
				}
			}*/
			TanukiExpressionVariant::Assignment(l_value, r_value) => {
				let (l_value, l_value_type) = l_value.const_compile_l_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any
					//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found
				)?;
				if l_value.is_none() {
					return Ok(None);
				}
				//if *where_extra_dependencies_found {
				//	return Ok(None);
				//}
				//r_value.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &l_value_type, where_extra_dependencies_found)?;
				r_value.const_compile_r_value(main, modules, this_module, this_module_path, was_complication_done, local_variables, &l_value_type)?;
				None
			}
			TanukiExpressionVariant::Block { sub_expressions, has_return_value } => {
				let sub_expressions_len = sub_expressions.len();
				let mut block_result = None;
				local_variables.push(HashMap::new());
				for (x, sub_expression) in sub_expressions.iter_mut().enumerate() {
					if x == sub_expressions_len - 1 && *has_return_value {
						let sub_expression_result = sub_expression.const_compile_r_value(
							main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type
							//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, result_type, where_extra_dependencies_found
						)?;
						//if *where_extra_dependencies_found {
						//	return Ok(None);
						//}
						if sub_expressions_len == 1 {
							block_result = sub_expression_result;
							*was_complication_done = true;
						}
					}
					else {
						let result = sub_expression.const_compile_r_value(
							main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any
							//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Any, where_extra_dependencies_found
						)?;
						if result.is_none() {
							return Ok(None);
						}
						//if *where_extra_dependencies_found {
						//	return Ok(None);
						//}
					}
				}
				local_variables.pop();
				block_result
			}
			_ => None,
		};
		//if *where_extra_dependencies_found {
		//	return Ok(None);
		//}
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

	pub fn const_compile_l_value(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType
		//&mut self, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		//local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, where_extra_dependencies_found: &mut bool,
	) -> Result<(Option<CompileTimeLValue>, TanukiType), ErrorAt> {
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
						local_variables.last_mut().unwrap().insert(name.clone(), (result_type.clone(), None));
						(Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level: local_variables.len() }), result_type.clone())
					}
					Some((type_t, _)) => (Some(CompileTimeLValue::LocalVariable { name: name.clone(), block_level }), type_t.clone())
				}
			}
			TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => {
				let type_t = match type_expression.const_compile_r_value(
					main, modules, this_module, module_path, was_complication_done, local_variables, &TanukiType::Type
					//modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &TanukiType::Type, where_extra_dependencies_found
				)? {
					Some(TanukiCompileTimeValue::Type(type_t)) => type_t,
					Some(_) => return Ok((None, TanukiType::Any)),
					None => TanukiType::Any,
				};
				//value_expression.const_compile_l_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, &type_t, where_extra_dependencies_found)?;
				value_expression.const_compile_l_value(main, modules, this_module, module_path, was_complication_done, local_variables, &type_t)?;
				(None, type_t)
			},
			_ => (None, TanukiType::Any),
		})
	}

	/*pub fn const_compile_r_value_forced(
		&mut self, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, where_extra_dependencies_found: &mut bool,
	) -> Result<TanukiCompileTimeValue, ErrorAt> {
		match self.const_compile_r_value(modules, this_module, global_items_to_const_compile_for_this_module, local_variables, result_type, where_extra_dependencies_found)? {
			Some(value) => Ok(value),
			None => Err(Error::UnableToConstCompile.at(Some(self.start_line), Some(self.start_column), None)),
		}
	}*/
}

pub enum CompileTimeLValue {
	LocalVariable { name: Box<str>, block_level: usize },
}