use std::{any::Any, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, mem::take, path::Path};

use num::{BigInt, FromPrimitive, One, Signed, ToPrimitive, Zero};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, function::{TanukiFunction, TanukiFunctionParameter}, global_constant::TanukiGlobalConstant, module::TanukiModule, t_type::TanukiType, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiNullaryOperator, TanukiPostfixUnaryOperator, TanukiPrefixUnaryOperator}}, traits::module::Module};

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
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		self.value_expression.const_compile_r_value(
			main, modules, this_module, module_path, was_complication_done, &mut Vec::new(), &TanukiType::Any, dependencies_need_const_compiling, Some(&self.name)
		)?;
		Ok(())
	}
}

impl TanukiFunction {
	/// Const-compiles a Tanuki function. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		let mut local_variables = Vec::new();
		local_variables.push(HashMap::new());
		// Const-compile each parameter
		for parameter in self.parameters.iter_mut() {
			if let Some(t_type) = &mut parameter.t_type {
				let t_type = t_type.const_compile_r_value(
					main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
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
				main, modules, this_module, module_path, was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
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
		if let Some(body) = &mut self.body {
			body.const_compile_r_value(
				main, modules, this_module, module_path, was_complication_done, &mut local_variables, &return_type, dependencies_need_const_compiling, None
			)?;
		}
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
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, dependencies_need_const_compiling: &mut bool, global_variable_assigned_to_name: Option<&str>,
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		// Unpack
		let Self { variant, start_line, start_column, .. } = self;
		// Try to const-compile depending on the expression variant
		let const_compiled_value = match variant {
			// Do nothing to already const-compiled values
			TanukiExpressionVariant::Constant(value) => Some(value.clone()),
			// Function pointer expressions get converted to a function pointer constant if the parameter and return types of the function have been const-compiled
			TanukiExpressionVariant::Function { name: target_function_name, module_path: target_module_path } => 'a: {
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
			TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression } => {
				let mut new_parameters = Vec::new();
				for parameter in take(parameters) {
					new_parameters.push(match parameter.variant {
						TanukiExpressionVariant::Variable(name) => TanukiFunctionParameter {
							t_type: None, name, start_line: parameter.start_line, start_column: parameter.start_column, end_line: parameter.end_line, end_column: parameter.end_column
						},
						TanukiExpressionVariant::TypeAndValue(t_type, name_expression) => match name_expression.variant {
							TanukiExpressionVariant::Variable(name) => TanukiFunctionParameter {
								t_type: Some(*t_type), name, start_line: parameter.start_line, start_column: parameter.start_column, end_line: parameter.end_line, end_column: parameter.end_column
							},
							_ => return Err(Error::ExpectedVariable.at(Some(parameter.start_line), Some(parameter.start_column), None)),
						}
						_ => return Err(Error::ExpectedVariable.at(Some(parameter.start_line), Some(parameter.start_column), None)),
					});
				}
				let mut module_hash = DefaultHasher::new();
				main.module_being_processed.hash(&mut module_hash);
				let module_function_index = this_module.functions.len();
				let mangled_function_name = match global_variable_assigned_to_name {
					Some(assigned_to_name) => format!("{assigned_to_name}_{}", module_hash.finish()).into_boxed_str(),
					None => format!("_tnk_fn_{module_function_index}_{}", module_hash.finish()).into_boxed_str(),
				};
				this_module.functions.push(Some(TanukiFunction {
					name: mangled_function_name.clone(), parameters: new_parameters.into_boxed_slice(),
					return_type: take(return_type).map(|return_type| *return_type),
					body: Some(take(body_expression)), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column,
				}));
				*self = TanukiExpression {
					variant: TanukiExpressionVariant::Function { name: mangled_function_name, module_path: main.module_being_processed.clone() },
					start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				};
				*was_complication_done = true;
				None
			}
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
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::CompileTimeInt, dependencies_need_const_compiling, None
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
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
				)? {
					Some(TanukiCompileTimeValue::Type(type_expression_parsed)) => type_expression_parsed,
					_ => return Ok(None),
				};
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
				let castee_expression_parsed = castee_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
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
			// Operators
			TanukiExpressionVariant::NullaryOperator(operator) => {
				let result = operator.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?;
				if result.is_some() {
					*was_complication_done = true;
				}
				result
			}
			TanukiExpressionVariant::PrefixUnaryOperator(operator, operand) => {
				let result = operator.const_compile_r_value(
					operand, main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?;
				if result.is_some() {
					*was_complication_done = true;
				}
				result
			}
			TanukiExpressionVariant::PostfixUnaryOperator(operator, operand) => {
				let result = operator.const_compile_r_value(
					operand, main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?;
				if result.is_some() {
					*was_complication_done = true;
				}
				result
			}
			TanukiExpressionVariant::InfixBinaryOperator(operator, lhs_expression, rhs_expression) => {
				let result = operator.const_compile_r_value(
					lhs_expression, rhs_expression, main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?;
				if result.is_some() {
					*was_complication_done = true;
				}
				result
			}
			TanukiExpressionVariant::InfixTernaryOperator(operator, lhs_expression, mhs_expression, rhs_expression) => {
				let result = operator.const_compile_r_value(
					lhs_expression, mhs_expression, rhs_expression, main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?;
				if result.is_some() {
					*was_complication_done = true;
				}
				result
			}
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
				r_value.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &l_value_type, dependencies_need_const_compiling, None
				)?;
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
							main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling, None
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
							main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
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
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(None);
				}
				for argument in arguments.iter_mut() {
					argument.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?;
					if *dependencies_need_const_compiling {
						return Ok(None);
					}
				}
				None
			}
			TanukiExpressionVariant::ImportConstant { name, module_path } => {
				let name = match (name, global_variable_assigned_to_name) {
					(Some(name), _) => &**name,
					(None, Some(name)) => name,
					(None, None) => todo!(),
				};
				let mut module = None;
				for (x_module_path, _, x_module) in modules.iter() {
					if &**module_path == &**x_module_path {
						module = x_module.as_ref();
						break;
					}
				}
				let module = match module {
					Some(module) => &**module,
					None => {
						return Ok(None);
					}
				};
				let module: &TanukiModule = match (module as &dyn Any).downcast_ref() {
					Some(module) => module,
					None => return Err(Error::Unimplemented("Linking to non-Tanuki modules".into()).at(None, None, None)),
				};
				let mut constant = None;
				for module_global_constant in module.global_constants.iter() {
					let module_global_constant = module_global_constant.as_ref().unwrap();
					if !module_global_constant.export {
						todo!()
					}
					if &*module_global_constant.name == name {
						if let TanukiExpressionVariant::Constant(module_global_constant) = &module_global_constant.value_expression.variant {
							constant = Some(module_global_constant);
						}
						else {
							return Ok(None);
						}
					}
				}
				let constant = match constant {
					Some(constant) => constant,
					None => todo!(),
				};
				*was_complication_done = true;
				Some(constant.clone())
			}
			TanukiExpressionVariant::Link { name, library_path, parameter_types, return_type, link_if } => 'a: {
				if name.is_none() {
					// Set the name of the item to link to to that of the variable this expression is being assigned to
					*name = match global_variable_assigned_to_name {
						Some(name) => Some(name.into()),
						None => None,
					};
				}
				// Const-compile link condition
				if let Some(link_if_some) = link_if {
					let link_if_value = link_if_some.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Bool, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(None);
					}
					match link_if_value {
						Some(TanukiCompileTimeValue::Bool(true)) => {
							*link_if = None;
							*was_complication_done = true;
						},
						Some(TanukiCompileTimeValue::Bool(false)) => {
							*was_complication_done = true;
							break 'a Some(TanukiCompileTimeValue::Void);
						}
						Some(_) => unreachable!(),
						None => return Ok(None),
					}
				}
				// Const-compile parameters
				for argument_type in parameter_types.iter_mut() {
					argument_type.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(None);
					}
				}
				// Const-compile return type
				if let Some(return_type) = return_type {
					return_type.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(None);
					}
				}
				// Get name
				let name = match name {
					Some(name) => name,
					None => todo!(),
				};
				// Push function
				let mut parameters = Vec::new();
				for parameter_type in parameter_types.iter() {
					parameters.push(TanukiFunctionParameter {
						t_type: Some(parameter_type.clone()), name: "".into(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
					});
				}
				this_module.functions.push(Some(TanukiFunction {
					name: name.clone(), parameters: parameters.into(), return_type: return_type.clone().map(|return_type| *return_type).into(),
					body: None, start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				}));
				// Convert to constant
				let return_type = match &return_type {
					Some(return_type) => match &**return_type {
						TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)), .. } => t_type.clone(),
						_ => unreachable!(),
					},
					None => TanukiType::Void,
				};
				let mut result_argument_types = Vec::new();
				for argument_type in parameter_types.iter() {
					result_argument_types.push(match &argument_type {
						TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)), .. } => t_type.clone(),
						_ => unreachable!(),
					});
				}
				*was_complication_done = true;
				main.link_to.insert(library_path.clone());
				Some(TanukiCompileTimeValue::LinkedFunctionPointer(name.clone(), return_type.into(), result_argument_types.into()))
			},
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
		&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, module_path: &Path, was_complication_done: &mut bool,
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
					main, modules, this_module, module_path, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
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

impl TanukiNullaryOperator {
	/// Const-compiles a Tanuki nullary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self,
		_main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], _this_module: &mut TanukiModule, _this_module_path: &Path, _was_complication_done: &mut bool,
		_local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, _dependencies_need_const_compiling: &mut bool
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		match self {
			_ => Ok(None),
		}
	}
}

impl TanukiPrefixUnaryOperator {
	/// Const-compiles a Tanuki prefix unary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, operand_expression: &mut TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		match self {
			Self::Read | Self::Not | Self::Reciprocal | Self::BitshiftRightOne | Self::ComplexConjugate | Self::Signum |
			Self::Negation | Self::SaturatingNegation | Self::WrappingNegation | Self::TryNegation |
			Self::Square | Self::SaturatingSquare | Self::WrappingSquare | Self::TrySquare |
			Self::BitshiftLeftOne | Self::SaturatingBitshiftLeftOne | Self::WrappingBitshiftLeftOne | Self::TryBitshiftLeftOne |
			Self::Dereference | Self::NthToLast | Self::RangeToExclusive | Self::RangeToInclusive => {
				let operand = match operand_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)? {
					Some(operand_expression) => operand_expression,
					_ => return Ok(None),
				};
				Ok(match (self, operand) {
					(Self::Read, operand) => Some(operand),
					(Self::Not, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(!operand)),
					(Self::Negation, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(-operand)),
					(Self::Square, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(&operand * &operand)),
					(Self::BitshiftLeftOne, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand << 1)),
					(Self::BitshiftRightOne, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand >> 1)),
					(Self::Signum, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand.signum())),
					_ => None,
				})
			}
			Self::AddressOf | Self::Increment | Self::SaturatingIncrement | Self::WrappingIncrement | Self::Decrement | Self::SaturatingDecrement | Self::WrappingDecrement => {
				let (l_value, _t_type) = match operand_expression.const_compile_l_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)? {
					(Some(l_value), t_type) => (l_value, t_type),
					_ => return Ok(None),
				};
				Ok(match (self, l_value) {
					_ => None,
				})
			}
		}
	}
}

impl TanukiPostfixUnaryOperator {
	/// Const-compiles a Tanuki postfix unary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, operand_expression: &mut TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		match self {
			Self::Percent | Self::Factorial | Self::SaturatingFactorial | Self::WrappingFactorial | Self::TryFactorial |
			Self::TryPropagate | Self::Unwrap => {
				let operand = match operand_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)? {
					Some(operand) => operand,
					_ => return Ok(None),
				};
				Ok(match (self, operand) {
					(Self::Percent, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand / 100)),
					(Self::Factorial, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt({
						if operand.is_negative() {
							return Err(Error::NegativeFactorial.at(Some(operand_expression.start_line), Some(operand_expression.start_column), None))
						}
						let operand: usize = match operand.to_usize() {
							Some(operand) => operand,
							None => return Err(Error::IntegerOverflow.at(Some(operand_expression.start_line), Some(operand_expression.start_column), None)),
						};
						let mut result = BigInt::one();
						for x in 0..= operand {
							result *= x;
						}
						result
					})),
					_ => None,
				})
			}
			Self::Increment | Self::SaturatingIncrement | Self::WrappingIncrement | Self::Decrement | Self::SaturatingDecrement | Self::WrappingDecrement => {
				let (l_value, _t_type) = match operand_expression.const_compile_l_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)? {
					(Some(l_value), t_type) => (l_value, t_type),
					_ => return Ok(None),
				};
				Ok(match (self, l_value) {
					//Self::AddressOf => None,
					_ => None,
				})
			}
		}
	}
}

impl TanukiInfixBinaryOperator {
	/// Const-compiles a Tanuki infix binary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, lhs_expression: &mut TanukiExpression, rhs_expression: &mut TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		match self {
			Self::Addition | Self::Subtraction | Self::Multiplication | Self::Division | Self::Modulo | Self::Exponent |
			Self::WrappingAddition | Self::WrappingSubtraction | Self::WrappingMultiplication | Self::WrappingDivision | Self::WrappingModulo | Self::WrappingExponent |
			Self::SaturatingAddition | Self::SaturatingSubtraction | Self::SaturatingMultiplication | Self::SaturatingDivision | Self::SaturatingModulo | Self::SaturatingExponent |
			Self::TryAddition | Self::TrySubtraction | Self::TryMultiplication | Self::TryDivision | Self::TryModulo | Self::TryExponent |
			Self::BitshiftLeft | Self::SaturatingBitshiftLeft | Self::WrappingBitshiftLeft | Self::TryBitshiftLeft | Self::BitshiftRight |
			Self::NonShortCircuitAnd | Self::NonShortCircuitOr | Self::NonShortCircuitXor | Self::ShortCircuitAnd | Self::ShortCircuitOr | Self::ShortCircuitXor |
			Self::NonShortCircuitNand | Self::NonShortCircuitNor | Self::NonShortCircuitXnor | Self::ShortCircuitNand | Self::ShortCircuitNor | Self::ShortCircuitXnor |
			Self::LessThan | Self::LessThanOrEqualTo | Self::GreaterThan | Self::GreaterThanOrEqualTo | Self::ThreeWayCompare |
			Self::Equality | Self::Inequality | Self::ReferenceEquality | Self::ReferenceInequality | Self::Minimum | Self::Maximum |
			Self::ShortCircuitingNullCoalescing | Self::NonShortCircuitingNullCoalescing | Self::ExclusiveRange | Self::InclusiveRange | Self::Append => {
				let (lhs_operand, rhs_operand) = match (
					lhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?,
					rhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?
				) {
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand, rhs_operand),
					_ => return Ok(None),
				};
				Ok(match (self, lhs_operand, rhs_operand) {
					(Self::Addition, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand + rhs_operand)),
					(Self::Subtraction, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand - rhs_operand)),
					(Self::Multiplication, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand * rhs_operand)),
					(Self::Division, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						if rhs_operand.is_zero() {
							return Err(Error::DivisionByZero.at(Some(lhs_expression.start_line), Some(lhs_expression.start_column), None));
						}
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand / rhs_operand))
					}
					(Self::Modulo, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						if rhs_operand.is_zero() {
							return Err(Error::ModuloByZero.at(Some(lhs_expression.start_line), Some(lhs_expression.start_column), None));
						}
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand % rhs_operand))
					}
					(Self::Exponent, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						let exponent = match rhs_operand.try_into() {
							Err(_) => return Ok(None),
							Ok(exponent) => exponent,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.pow(exponent)))
					}
					(Self::NonShortCircuitAnd | Self::ShortCircuitAnd, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand & rhs_operand)),
					(Self::NonShortCircuitOr | Self::ShortCircuitOr, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand | rhs_operand)),
					(Self::NonShortCircuitXor | Self::ShortCircuitXor, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand ^ rhs_operand)),
					(Self::NonShortCircuitNand | Self::ShortCircuitNand, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(!(lhs_operand & rhs_operand))),
					(Self::NonShortCircuitNor | Self::ShortCircuitNor, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(!(lhs_operand | rhs_operand))),
					(Self::NonShortCircuitXnor | Self::ShortCircuitXnor, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(!(lhs_operand ^ rhs_operand))),
					(Self::BitshiftLeft, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						let rhs_operand: u128 = match rhs_operand.try_into() {
							Err(_) => return Ok(None),
							Ok(rhs_operand) => rhs_operand,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand << rhs_operand))
					}
					(Self::BitshiftRight, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						let rhs_operand: u128 = match rhs_operand.try_into() {
							Err(_) => return Ok(None),
							Ok(rhs_operand) => rhs_operand,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand >> rhs_operand))
					}
					(Self::ThreeWayCompare, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt({
							if lhs_operand == rhs_operand {
								BigInt::ZERO
							}
							else if lhs_operand > rhs_operand {
								BigInt::one()
							}
							else {
								BigInt::from_i8(-1).unwrap()
							}
						})),
					(Self::Minimum, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.min(rhs_operand))),
					(Self::Maximum, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.max(rhs_operand))),
					_ => None
				})
			}
			Self::MemberAccess | Self::As | Self::SaturatingAs | Self::WrappingAs | Self::TryAs | Self::Pipe => Ok(None),
			Self::None => unreachable!(),
		}
	}
}

impl TanukiInfixTernaryOperator {
	/// Const-compiles a Tanuki infix ternary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, lhs_expression: &mut TanukiExpression, mhs_expression: &mut TanukiExpression, rhs_expression: &mut TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], this_module: &mut TanukiModule, this_module_path: &Path, was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<Option<TanukiCompileTimeValue>, ErrorAt> {
		match self {
			Self::NonShortCircuitingConditional | Self::ShortCircuitingConditional => {
				let (lhs_operand, mhs_operand, rhs_operand) = match (
					lhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?,
					mhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?,
					rhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?
				) {
					(Some(lhs_operand), Some(mhs_operand), Some(rhs_operand)) => (lhs_operand, mhs_operand, rhs_operand),
					_ => return Ok(None),
				};
				Ok(match (self, lhs_operand, mhs_operand, rhs_operand) {
					_ => None
				})
			}
		}
	}
}