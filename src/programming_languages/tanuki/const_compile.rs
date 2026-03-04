use std::{any::Any, collections::HashMap, mem::take, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, global_constant::TanukiGlobalConstant, module::TanukiModule, t_type::TanukiType}, traits::module::Module};

impl TanukiModule {
	/// Const-compiles a Tanuki module. Will set `was_complication_done` to `true` if any compilation was done. This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile_globals(&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], module_path: &Path, was_complication_done: &mut bool) -> Result<(), ErrorAt> {
		// Const-compile globals that we can
		for x in 0..self.global_constants.len() {
			let global_constant = &mut self.global_constants[x];
			let mut global_constant_removed = take(global_constant).unwrap();
			global_constant_removed.const_compile(main, modules, self, module_path, was_complication_done, &mut false)?;
			let global_constant = &mut self.global_constants[x];
			*global_constant = Some(global_constant_removed);
		}
		// Const-compile functions that we can
		for x in 0..self.functions.len() {
			let function = &mut self.functions[x];
			let mut function_removed = take(function).unwrap();
			function_removed.const_compile(main, modules, self, module_path, was_complication_done, &mut false)?;
			let function = &mut self.functions[x];
			*function = Some(function_removed);
		}
		// Check for duplicate constants without the same value that have been parsed
		// Search over all constants
		for (x, global_constant_x) in self.global_constants.iter().enumerate() {
			let global_constant_x = global_constant_x.as_ref().unwrap();
			// For any given constant, search over all other constants
			for (y, global_constant_y) in self.global_constants.iter().enumerate() {
				let global_constant_y = global_constant_y.as_ref().unwrap();
				if x == y || global_constant_x.name != global_constant_y.name {
					continue;
				}
				// If the constants has been parsed to a constant value, make sure they are the same
				match (&global_constant_x.value_expression.variant, &global_constant_y.value_expression.variant) {
					(TanukiExpressionVariant::Constant(x_value), TanukiExpressionVariant::Constant(y_value)) => {
						if x_value != y_value {
							return Err(Error::DuplicateGlobalVariableWithDifferentValues.at(Some(global_constant_x.start_line), Some(global_constant_x.start_column), None));
						}
					}
					(_, _) => {},
				}
			}
		}
		// Return
		Ok(())
	}
}

impl TanukiGlobalConstant {
	/// Const-compiles a Tanuki global constant. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if another variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		self.value_expression.const_compile_r_value(
			main, modules, this_module, module_path, was_complication_done, &mut Vec::new(), &TanukiType::Any, dependencies_need_const_compiling
		)?;
		Ok(())
	}
}

impl TanukiFunction {
	/// Const-compiles a Tanuki function. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		let mut local_variables = Vec::new();
		local_variables.push(HashMap::new());
		// Const-compile each parameter
		for parameter in self.parameters.iter_mut() {
			if let Some(t_type) = &mut parameter.t_type {
				let t_type = t_type.const_compile_r_value(
					main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(());
				}
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
		// Const-compile the return type
		let return_type = match &mut self.return_type {
			Some(return_type) => return_type.const_compile_r_value(
				main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling
			)?,
			None => None,
		};
		if *dependencies_need_const_compiling {
			return Ok(());
		}
		let return_type = match return_type {
			Some(TanukiCompileTimeValue::Type(return_type)) => return_type,
			None => TanukiType::Any,
			_ => return Ok(()),
		};
		// Const-compile the function body
		self.body.const_compile_r_value(main, modules, this_module, module_path, was_complication_done, &mut local_variables, &return_type, dependencies_need_const_compiling)?;
		// Return
		Ok(())
	}
}

impl TanukiExpression {
	/// Const-compiles a Tanuki expression as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, dependencies_need_const_compiling: &mut bool,
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		// Unpack
		let Self { variant, start_line, start_column, .. } = self;
		// Try to const-compile depending on the expression variant
		let const_compiled_value = match variant {
			// Do nothing to already const-compiled values
			TanukiExpressionVariant::Constant(value) => Some(value.clone()),
			// Function pointer expressions get converted to a function pointer constant if the parameter and return types of the function have been const-compiled
			TanukiExpressionVariant::Function(target_function_name, target_module_path) => 'a: {
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
			// Any variable will be replaced by the value of the variable if it has been const-compiled
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
					Some((_, value)) => {
						if value.is_some() {
							*was_complication_done = true;
						}
						value.clone()
					},
					// Else read the global variable
					None => 'a: {
						for global_constant in this_module.global_constants.iter() {
							if let Some(global_constant) = global_constant && global_constant.name == *name {
								match &global_constant.value_expression.variant {
									TanukiExpressionVariant::Constant(value) => {
										*was_complication_done = true;
										break 'a Some(value.clone())
									},
									_ => {
										*dependencies_need_const_compiling = true;
										return Ok(None)
									},
								}
							}
						}
						return Err(Error::VariableNotFound.at(Some(*start_line), Some(*start_column), None));
					}
				}
			}
			// @u(x), @i(x), @f(x) get converted to a constant type if their bit-width argument has been const-compiled
			TanukiExpressionVariant::U(sub_expressions) | TanukiExpressionVariant::I(sub_expressions) |TanukiExpressionVariant::F(sub_expressions) => {
				if sub_expressions.len() != 1 {
					return Err(Error::UnexpectedBuiltinFunctionArgumentCount {
						expected_min: Some(1), expected_max: Some(1), got: sub_expressions.len()
					}.at(Some(*start_line), Some(*start_column), None));
				}
				let sub_expression = &mut sub_expressions[0];
				let argument = sub_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::CompileTimeInt, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
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
			// For a type and value, try to convert the value to the type
			TanukiExpressionVariant::TypeAndValue(type_expression, castee_expression) => {
				let type_expression_parsed = match type_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling
				)? {
					Some(TanukiCompileTimeValue::Type(type_expression_parsed)) => type_expression_parsed,
					_ => return Ok(None),
				};
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
				let castee_expression_parsed = castee_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
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
			// For assignments, const-compile the l and r-values
			TanukiExpressionVariant::Assignment(l_value, r_value) => {
				// Const-compile the l-value
				let (_l_value, l_value_type) = l_value.const_compile_l_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
				// Const-compile the r-value
				r_value.const_compile_r_value(main, modules, this_module, this_module_path, was_complication_done, local_variables, &l_value_type, dependencies_need_const_compiling)?;
				// TODO: Assign
				// Return
				None
			}
			// For blocks, const-compile each sub-expression
			TanukiExpressionVariant::Block { sub_expressions, has_return_value } => 'a: {
				let sub_expressions_len = sub_expressions.len();
				let mut _block_result = None;
				local_variables.push(HashMap::new());
				for (x, sub_expression) in sub_expressions.iter_mut().enumerate() {
					if x == sub_expressions_len - 1 && *has_return_value {
						let sub_expression_result = sub_expression.const_compile_r_value(
							main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
						)?;
						if *dependencies_need_const_compiling {
							return Ok(None);
						}
						if sub_expression_result.is_none() {
							return Ok(None);
						}
						if sub_expressions_len == 1 {
							*was_complication_done = true;
							break 'a sub_expression_result
						}
					}
					else {
						sub_expression.const_compile_r_value(
							main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
						)?;
						if *dependencies_need_const_compiling {
							return Ok(None);
						}
					}
				}
				local_variables.pop();
				_block_result
			}
			// For function calls, const-compile the function pointer and the arguments
			TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
				function_pointer.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
				for argument in arguments.iter_mut() {
					argument.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
					)?;
					if *dependencies_need_const_compiling {
						return Ok(None);
					}
				}
				None
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

	/// Const-compiles a Tanuki expression as an l-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// Returns the l-value if it could be const-compiled.
	pub fn const_compile_l_value(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &TanukiModule, module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, dependencies_need_const_compiling: &mut bool,
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
					main, modules, this_module, module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling
				)? {
					Some(TanukiCompileTimeValue::Type(type_t)) => type_t,
					Some(_) => return Ok((None, TanukiType::Any)),
					None => TanukiType::Any,
				};
				if *dependencies_need_const_compiling {
					return Ok((None, TanukiType::Any));
				}
				value_expression.const_compile_l_value(main, modules, this_module, module_path, was_complication_done, local_variables, &type_t, dependencies_need_const_compiling)?;
				(None, type_t)
			},
			_ => (None, TanukiType::Any),
		})
	}
}

/// An l-value.
pub enum CompileTimeLValue {
	/// A local variable in a given block.
	LocalVariable { name: Box<str>, block_level: usize },
}