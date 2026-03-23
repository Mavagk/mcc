use std::{any::Any, collections::{BTreeMap, HashMap}, hash::{DefaultHasher, Hash, Hasher}, iter::once, mem::take, path::Path};

use num::{BigInt, FromPrimitive, One, Signed, ToPrimitive, Zero};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::{FunctionPointer, TanukiCompileTimeValue}, expression::{TanukiExpression, TanukiExpressionVariant}, function::{TanukiFunction, TanukiFunctionParameter}, global_constant::TanukiGlobalConstant, module::TanukiModule, t_type::TanukiType, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiNullaryOperator, TanukiPostfixUnaryOperator, TanukiPrefixUnaryOperator}}, traits::module::Module};

impl TanukiModule {
	/// Const-compiles a Tanuki module. Will set `was_complication_done` to `true` if any compilation was done. This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile_globals(&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], module_path: &Path, was_complication_done: &mut bool) -> Result<(), ErrorAt> {
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
		&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		if let Some(t_type) = &mut self.t_type {
			t_type.const_compile_r_value(
				main, modules, this_module, module_path, &mut None, was_complication_done, &mut Vec::new(), &TanukiType::Type, dependencies_need_const_compiling, Some(&self.name)
			)?;
		}
		if *dependencies_need_const_compiling {
			return Ok(());
		}
		let return_type = match &self.t_type {
			None => &TanukiType::Any,
			Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(return_type)), .. }) => return_type,
			_ => unreachable!(),
		};
		self.value_expression.const_compile_r_value(
			main, modules, this_module, module_path, &mut None, was_complication_done, &mut Vec::new(), return_type, dependencies_need_const_compiling, Some(&self.name)
		)?;
		Ok(())
	}
}

impl TanukiFunction {
	/// Const-compiles a Tanuki function. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	pub fn const_compile(
		&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, module_path: &Path,
		was_complication_done: &mut bool, dependencies_need_const_compiling: &mut bool,
	) -> Result<(), ErrorAt> {
		let mut local_variables = Vec::new();
		local_variables.push(HashMap::new());
		// Const-compile each parameter
		for x in 0..self.parameters.len() {
			if let Some(mut t_type_parameter) = take(&mut self.parameters[x]) {
				let t_type = t_type_parameter.t_type.as_mut().unwrap().const_compile_r_value(
					main, modules, this_module, module_path, &mut Some(self), was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
				)?.result_value;
				self.parameters[x] = Some(t_type_parameter);
				if *dependencies_need_const_compiling {
					return Ok(());
				}
				let t_type = match t_type.unwrap() {
					TanukiCompileTimeValue::Type(t_type) => t_type,
					_ => return Ok(()),
				};
				let parameter = self.parameters[x].as_mut().unwrap();
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), (t_type, None));
			}
			else {
				let parameter = self.parameters[x].as_mut().unwrap();
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), (TanukiType::Any, None));
			}
		}
		// Const-compile the return type
		let mut return_type = None;
		if let Some(mut return_type_expression) = take(&mut self.return_type) {
			return_type = return_type_expression.const_compile_r_value(
				main, modules, this_module, module_path, &mut Some(self), was_complication_done, &mut local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
			)?.result_value;
			self.return_type = Some(return_type_expression);
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
		if let Some(mut body) = take(&mut self.body) {
			body.const_compile_r_value(
				main, modules, this_module, module_path, &mut Some(self), was_complication_done, &mut local_variables, &return_type, dependencies_need_const_compiling, None
			)?;
			self.body = Some(body);
		}
		// Const-compile the concrete type function bodies
		let mut bodies_for_concrete_types = take(&mut self.bodies_for_concrete_types).unwrap();
		for ((parameter_types, return_type), body) in bodies_for_concrete_types.iter_mut() {
			let mut local_variables = Vec::new();
			local_variables.push(HashMap::new());
			for (x, parameter) in self.parameters.iter().enumerate() {
				let parameter = parameter.as_ref().unwrap();
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), (parameter_types[x].clone(), None));
			}
			body.const_compile_r_value(
				main, modules, this_module, module_path, &mut Some(self), was_complication_done, &mut local_variables, &return_type, dependencies_need_const_compiling, None
			)?;
		}
		self.bodies_for_concrete_types = Some(bodies_for_concrete_types);
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
		&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, this_module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, dependencies_need_const_compiling: &mut bool, global_variable_assigned_to_name: Option<&str>,
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		// Unpack
		let Self { variant, start_line, start_column, .. } = self;
		// Try to const-compile depending on the expression variant
		let mut expression_result: RValueConstComplicationResult = match variant {
			//
			TanukiExpressionVariant::Constant(TanukiCompileTimeValue::FunctionPointer(function_pointer)) => 'a: {
				let is_concrete = function_pointer.return_type.is_concrete() && function_pointer.parameter_types.iter().all(|parameter_type| parameter_type.is_concrete());
				if !is_concrete {
					break 'a RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::FunctionPointer(function_pointer.clone())), result_type: None, is_pure: true }
				}
				// Get the module of the function
				let mut module = None;
				for (x_module_path, _, x_module, _x_mangled_name) in modules.iter_mut() {
					if &*function_pointer.module_path == &**x_module_path {
						module = x_module.as_mut();
						break;
					}
				}
				let module: &mut TanukiModule = match module {
					Some(module) => {
						let module = &mut **module;
						(module as &mut dyn Any).downcast_mut().unwrap()
					},
					None => this_module,
				};
				// Get the function
				let mut function = None;
				for function_x in module.functions.iter_mut() {
					if function_x.is_none() {
						continue;
					}
					if function_x.as_ref().unwrap().name == function_pointer.name {
						function = Some(function_x.as_mut().unwrap());
					}
				}
				let function = match function {
					Some(function) => function,
					None => this_function.as_mut().unwrap(),
				};
				// Push the concrete body
				//let parameter_types: Box<[TanukiType]> = function_pointer.parameter_types;
				let types = (function_pointer.parameter_types.clone(), function_pointer.return_type.clone().into());
				if !function.bodies_for_concrete_types.as_ref().unwrap().contains_key(&types) {
					function.bodies_for_concrete_types.as_mut().unwrap().insert(types, function.body.clone().unwrap());
				}
				// Convert to concrete function pointer
				//function_pointer.variant = TanukiExpressionVariant::Constant(TanukiCompileTimeValue::ConcreteFunctionPointer(FunctionPointer {
				//	name: function_pointer_constant.name.clone(), module_path: function_pointer_constant.module_path.clone(), return_type: result_type.clone().into(), parameter_types: parameter_types.clone().into()
				//}));
				*was_complication_done = true;
				//RValueConstComplicationResult { result_value: Some(value.clone()), result_type: None, is_pure: true }
				RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::ConcreteFunctionPointer(function_pointer.clone())), result_type: None, is_pure: true }
			}
			// Do nothing to already const-compiled values
			TanukiExpressionVariant::Constant(value) => RValueConstComplicationResult { result_value: Some(value.clone()), result_type: None, is_pure: true },
			// Function pointer expressions get converted to a function pointer constant if the parameter and return types of the function have been const-compiled
			TanukiExpressionVariant::Function { name: target_function_name, module_path: target_module_path } => 'a: {
				for (module_path, _, module, _mangled_name) in modules.iter() {
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
						let function = match function {
							Some(function) => function,
							None => match this_function {
								Some(this_function) => this_function,
								None => continue,
							},
						};
						if &function.name != target_function_name {
							continue;
						}
						let return_type = match &function.return_type {
							Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => value,
							_ => return Ok(RValueConstComplicationResult::default()),
						};
						let mut parameter_types = Vec::new();
						for parameter in function.parameters.iter() {
							let parameter = parameter.as_ref().unwrap();
							match &parameter.t_type {
								Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => parameter_types.push(value.clone()),
								_ => return Ok(RValueConstComplicationResult::default()),
							};
						}
						*was_complication_done = true;
						break 'a RValueConstComplicationResult {
							result_value: Some(TanukiCompileTimeValue::FunctionPointer(FunctionPointer {
								name: target_function_name.clone(), module_path: target_module_path.clone(), return_type: return_type.clone().into(), parameter_types: parameter_types.into()
							})),
							result_type: None, is_pure: true
						};
					}
					return Ok(RValueConstComplicationResult::default());
				}
				return Ok(RValueConstComplicationResult::default());
			},
			TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression } => {
				let mut new_parameters = Vec::new();
				for parameter in take(parameters) {
					new_parameters.push(Some(match parameter.variant {
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
					}));
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
					bodies_for_concrete_types: Some(HashMap::new()),
				}));
				*self = TanukiExpression {
					variant: TanukiExpressionVariant::Function { name: mangled_function_name, module_path: main.module_being_processed.clone() },
					start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				};
				*was_complication_done = true;
				RValueConstComplicationResult { result_value: None, result_type: None, is_pure: true }
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
						RValueConstComplicationResult { result_value: value.clone(), result_type: None, is_pure: true }
					},
					// Else read the global variable
					None => 'a: {
						for global_constant in this_module.global_constants.iter() {
							if let Some(global_constant) = global_constant && global_constant.name == *name {
								match &global_constant.value_expression.variant {
									TanukiExpressionVariant::Constant(value) => {
										*was_complication_done = true;
										break 'a RValueConstComplicationResult { result_value: Some(value.clone()), result_type: None, is_pure: true };
										//break 'a Some(value.clone())
									},
									_ => {
										*dependencies_need_const_compiling = true;
										return Ok(RValueConstComplicationResult::default())
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
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::CompileTimeInt, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				let argument = match argument.result_value {
					Some(TanukiCompileTimeValue::CompileTimeInt(argument)) => argument,
					_ => return Ok(RValueConstComplicationResult::default()),
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
						RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::Type(TanukiType::U(bit_width))), result_type: None, is_pure: true }
					}
					TanukiExpressionVariant::I(_) => {
						let bit_width: u8 = match (&argument).try_into() {
							Ok(argument) => argument,
							Err(_) => return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None)),
						};
						if !matches!(bit_width, 8 | 16 | 32 | 64) {
							return Err(Error::InvalidIntegerBitWidth(argument).at(Some(*start_line), Some(*start_column), None));
						}
						//Some(TanukiCompileTimeValue::Type(TanukiType::I(bit_width)))
						RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::Type(TanukiType::I(bit_width))), result_type: None, is_pure: true }
					}
					TanukiExpressionVariant::F(_) => {
						let bit_width: u8 = match (&argument).try_into() {
							Ok(argument) => argument,
							Err(_) => return Err(Error::InvalidFloatBitWidth(argument).at(Some(*start_line), Some(*start_column), None)),
						};
						if !matches!(bit_width, 32 | 64) {
							return Err(Error::InvalidFloatBitWidth(argument).at(Some(*start_line), Some(*start_column), None));
						}
						//Some(TanukiCompileTimeValue::Type(TanukiType::F(bit_width)))
						RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::Type(TanukiType::F(bit_width))), result_type: None, is_pure: true }
					}
					_ => unreachable!(),
				}
			}
			// For a type and value, try to convert the value to the type
			TanukiExpressionVariant::TypeAndValue(type_expression, castee_expression) => {
				let type_expression_parsed = type_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
				)?;
				let type_expression_parsed_type = match type_expression_parsed.result_value {
					Some(TanukiCompileTimeValue::Type(type_expression_parsed)) => type_expression_parsed,
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				if !type_expression_parsed.is_pure || *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				let mut result = castee_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				if let Some(result_value) = &mut result.result_value {
					*result_value = result_value.clone().cast_to(&type_expression_parsed_type, true).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?;
				}
				if let Some(result_type) = &mut result.result_type {
					*result_type = result_type.cast_to(&type_expression_parsed_type).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?;
				}
				result
			}
			// Operators
			TanukiExpressionVariant::NullaryOperator(operator) => {
				operator.const_compile_r_value(
					main, modules, this_module, this_module_path, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?
			}
			TanukiExpressionVariant::PrefixUnaryOperator(operator, operand) => {
				operator.const_compile_r_value(
					operand, main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?
			}
			TanukiExpressionVariant::PostfixUnaryOperator(operator, operand) => {
				operator.const_compile_r_value(
					operand, main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?
			}
			TanukiExpressionVariant::InfixBinaryOperator(operator, lhs_expression, rhs_expression) => {
				operator.const_compile_r_value(
					lhs_expression, rhs_expression, main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, result_type, dependencies_need_const_compiling,
					global_variable_assigned_to_name
				)?
			}
			TanukiExpressionVariant::InfixTernaryOperator(operator, lhs_expression, mhs_expression, rhs_expression) => {
				operator.const_compile_r_value(
					lhs_expression, mhs_expression, rhs_expression, main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, result_type, dependencies_need_const_compiling
				)?
			}
			// For assignments, const-compile the l and r-values
			TanukiExpressionVariant::Assignment(l_value, r_value) => {
				// Const-compile the l-value
				let (_l_value, l_value_type) = l_value.const_compile_l_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				// Const-compile the r-value
				let mut value_result = r_value.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &l_value_type, dependencies_need_const_compiling, None
				)?;
				// TODO: Assign
				// Return
				value_result.is_pure = false;
				value_result
			}
			// For blocks, const-compile each sub-expression
			TanukiExpressionVariant::Block { sub_expressions, return_expressions } => 'a: {
				local_variables.push(HashMap::new());
				let mut is_pure = true;
				let mut x = 0;
				while x < sub_expressions.len() {
					let sub_expression = &mut sub_expressions[x];
					let sub_expression_result = sub_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
					if sub_expression_result.is_pure {
						sub_expressions.remove(x);
					}
					is_pure &= sub_expression_result.is_pure;
					x += 1;
				}
				let return_expressions_len = return_expressions.len();
				let mut ordered_members = Vec::new();
				let mut named_members = HashMap::new();
				let mut ordered_member_types = Vec::new();
				let mut named_member_types = BTreeMap::new();
				let mut are_all_values_known = true;
				let mut are_all_types_known = true;
				for (return_expression_name, return_expression) in return_expressions.iter_mut() {
					let result_type = match return_expressions_len == 1 && return_expression_name.is_none() {
						true => result_type,
						false => &TanukiType::Any,
					};
					let mut return_expression_result = return_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, result_type, dependencies_need_const_compiling, None
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
					if return_expressions_len == 1 && return_expression_name.is_none() {
						if sub_expressions.len() > 0 {
							return_expression_result.is_pure = false;
						}
						break 'a return_expression_result;
					}
					is_pure &= return_expression_result.is_pure;
					// Types
					if return_expression_result.result_type.is_none() {
						are_all_types_known = false;
					}
					if !are_all_types_known {
						continue;
					}
					match return_expression_name.clone() {
						None => ordered_member_types.push(return_expression_result.result_type.unwrap()),
						Some(return_expression_name) => {
							named_member_types.insert(return_expression_name, return_expression_result.result_type.unwrap());
						}
					}
					// Values
					if return_expression_result.result_value.is_none() {
						are_all_values_known = false;
					}
					if !are_all_values_known {
						continue;
					}
					match return_expression_name {
						None => ordered_members.push(return_expression_result.result_value.unwrap()),
						Some(return_expression_name) => {
							named_members.insert(return_expression_name.clone(), return_expression_result.result_value.unwrap());
						}
					}
				}
				local_variables.pop();
				// Return void if there are no return expressions
				if return_expressions.is_empty() {
					break 'a RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::Void), result_type: None, is_pure: false }
				}
				// Else create the return struct
				RValueConstComplicationResult {
					result_value: match are_all_values_known {
						false => None,
						true => Some(TanukiCompileTimeValue::Struct { ordered_members: ordered_members.into(), named_members: named_members })
					},
					result_type: match are_all_types_known {
						false => None,
						true => Some(TanukiType::Struct { ordered_members: ordered_member_types.into(), named_members: named_member_types })
					},
					is_pure
				}
			}
			// For function calls, const-compile the function pointer and the arguments
			TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
				// Const-compile arguments
				let mut argument_types = Vec::new();
				let mut is_concrete = result_type.is_concrete();
				for argument in arguments.iter_mut() {
					let argument_result = argument.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables,
						&TanukiType::Any, dependencies_need_const_compiling, None
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
					match argument_result.result_type {
						Some(result_type) => {
							is_concrete &= result_type.is_concrete();
							argument_types.push(result_type);
						},
						None => is_concrete = false,
					}
				}
				// Const-compile pointer
				function_pointer.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				// If the function pointer is non-concrete but the arguments are
				if is_concrete && matches!(function_pointer.variant, TanukiExpressionVariant::Constant(TanukiCompileTimeValue::FunctionPointer(_))) {
					// Get function pointer constant
					let function_pointer_constant = match &function_pointer.variant {
						TanukiExpressionVariant::Constant(TanukiCompileTimeValue::FunctionPointer(function_pointer)) => function_pointer,
						_ => unreachable!(),
					};
					// Get the module of the function
					let mut module = None;
					for (x_module_path, _, x_module, _x_mangled_name) in modules.iter_mut() {
						if &*function_pointer_constant.module_path == &**x_module_path {
							module = x_module.as_mut();
							break;
						}
					}
					let module: &mut TanukiModule = match module {
						Some(module) => {
							let module = &mut **module;
							(module as &mut dyn Any).downcast_mut().unwrap()
						},
						None => this_module,
					};
					// Get the function
					let mut function = None;
					for function_x in module.functions.iter_mut() {
						if function_x.is_none() {
							continue;
						}
						if function_x.as_ref().unwrap().name == function_pointer_constant.name {
							function = Some(function_x.as_mut().unwrap());
						}
					}
					let function = match function {
						Some(function) => function,
						None => this_function.as_mut().unwrap(),
					};
					// Push the concrete body
					let argument_types: Box<[TanukiType]> = argument_types.into_boxed_slice();
					let types = (argument_types.clone(), result_type.clone().into());
					if !function.bodies_for_concrete_types.as_ref().unwrap().contains_key(&types) {
						function.bodies_for_concrete_types.as_mut().unwrap().insert(types, function.body.clone().unwrap());
					}
					// Convert to concrete function pointer
					function_pointer.variant = TanukiExpressionVariant::Constant(TanukiCompileTimeValue::ConcreteFunctionPointer(FunctionPointer {
						name: function_pointer_constant.name.clone(), module_path: function_pointer_constant.module_path.clone(), return_type: result_type.clone().into(), parameter_types: argument_types.clone().into()
					}));
					*was_complication_done = true;
				}
				// TODO: Return type and evaluate const function
				RValueConstComplicationResult::default()
			}
			TanukiExpressionVariant::ImportConstant { name, module_path } => {
				let name = match (name, global_variable_assigned_to_name) {
					(Some(name), _) => &**name,
					(None, Some(name)) => name,
					(None, None) => return Err(Error::UnknownName.at(Some(self.start_line), Some(self.start_column), None))
				};
				let mut module = None;
				let mut module_mangled_name = None;
				for (x_module_path, _, x_module, x_mangled_name) in modules.iter() {
					if &**module_path == &**x_module_path {
						module = x_module.as_ref();
						module_mangled_name = Some(&**x_mangled_name);
						break;
					}
				}
				let module = match module {
					Some(module) => &**module,
					None => {
						return Ok(RValueConstComplicationResult::default());
					}
				};
				let module_mangled_name = module_mangled_name.unwrap();
				let module: &TanukiModule = match (module as &dyn Any).downcast_ref() {
					Some(module) => module,
					None => return Err(Error::Unimplemented("Linking to non-Tanuki modules".into()).at(None, None, None)),
				};
				let mut constant = None;
				for module_global_constant in module.global_constants.iter() {
					let module_global_constant_expression = module_global_constant.as_ref().unwrap();
					if &*module_global_constant_expression.name == name {
						if let TanukiExpressionVariant::Constant(module_global_constant) = &module_global_constant_expression.value_expression.variant {
							if !module_global_constant_expression.export {
								return Err(Error::GlobalConstantNotExported.at(Some(self.start_line), Some(self.start_column), None))
							}
							constant = Some(module_global_constant);
						}
						else {
							return Ok(RValueConstComplicationResult::default());
						}
					}
				}
				let constant = match constant {
					Some(constant) => constant,
					None => return Err(Error::GlobalConstantNotFound.at(Some(self.start_line), Some(self.start_column), None)),
				};
				*was_complication_done = true;
				this_module.mangled_module_names_to_include_in_c.insert(module_mangled_name.into());
				//Some(constant.clone())
				RValueConstComplicationResult { result_value: Some(constant.clone()), result_type: None, is_pure: true }
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
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables,
						&TanukiType::Bool, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
					match link_if_value.result_value {
						Some(TanukiCompileTimeValue::Bool(true)) => {
							*link_if = None;
							*was_complication_done = true;
						},
						Some(TanukiCompileTimeValue::Bool(false)) => {
							*was_complication_done = true;
							break 'a RValueConstComplicationResult { result_value: Some(TanukiCompileTimeValue::Void), result_type: None, is_pure: true }//Some(TanukiCompileTimeValue::Void);
						}
						Some(_) => unreachable!(),
						None => return Ok(RValueConstComplicationResult::default()),
					}
				}
				// Const-compile parameters
				for argument_type in parameter_types.iter_mut() {
					argument_type.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables,
						&TanukiType::Type, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
				}
				// Const-compile return type
				if let Some(return_type) = return_type {
					return_type.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables,
						&TanukiType::Type, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?;
					if *dependencies_need_const_compiling {
						return Ok(RValueConstComplicationResult::default());
					}
				}
				// Get name
				let name = match name {
					Some(name) => name,
					None => return Err(Error::UnknownName.at(Some(self.start_line), Some(self.start_column), None)),
				};
				// Push function
				let mut parameters = Vec::new();
				for parameter_type in parameter_types.iter() {
					parameters.push(Some(TanukiFunctionParameter {
						t_type: Some(parameter_type.clone()), name: "".into(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
					}));
				}
				this_module.functions.push(Some(TanukiFunction {
					name: name.clone(), parameters: parameters.into(), return_type: return_type.clone().map(|return_type| *return_type).into(),
					body: None, start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column,
					bodies_for_concrete_types: Some(HashMap::new()),
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
				//Some(TanukiCompileTimeValue::LinkedFunctionPointer(name.clone(), return_type.into(), result_argument_types.into()))
				RValueConstComplicationResult {
					result_value: Some(TanukiCompileTimeValue::LinkedFunctionPointer(name.clone(), return_type.into(), result_argument_types.into())), result_type: None, is_pure: true
				}
			},
			TanukiExpressionVariant::Transmute { to_transmute, transmute_to_type } => {
				to_transmute.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				transmute_to_type.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
				)?;
				if *dependencies_need_const_compiling {
					return Ok(RValueConstComplicationResult::default());
				}
				// TODO
				RValueConstComplicationResult::default()
			}
			_ => RValueConstComplicationResult::default(),
		};
		// Cast
		if let Some(result_value) = &expression_result.result_value && expression_result.result_type.is_none() {
			expression_result.result_type = Some(result_value.get_type());
		}
		if expression_result.result_type.is_none() {
			expression_result.result_type = Some(result_type.clone());
		}
		if expression_result.result_value.is_none() {
			expression_result.result_type = Some(
				expression_result.result_type.unwrap().cast_to(result_type).map_err(|err: Error| err.at(Some(self.start_line), Some(self.start_column), None))?
			);
			return Ok(expression_result);
		}
		expression_result.result_value = Some(
			expression_result.result_value.unwrap().cast_to(result_type, false).map_err(|err: Error| err.at(Some(self.start_line), Some(self.start_column), None))?
		);
		expression_result.result_type = Some(expression_result.result_value.as_ref().unwrap().get_type());

		if expression_result.is_pure && !matches!(&self.variant, TanukiExpressionVariant::Constant(constant) if expression_result.result_value.as_ref().unwrap() == constant) {
			self.variant = TanukiExpressionVariant::Constant(expression_result.result_value.as_ref().unwrap().clone());
			*was_complication_done = true;
		}
		//let const_compiled_value = match expression_result {
		//	Some(const_compiled_value) => Some(
		//		const_compiled_value.cast_to(result_type, false).map_err(|err| err.at(Some(self.start_line), Some(self.start_column), None))?
		//	),
		//	None => None,
		//};
		//// If complication was done, replace this with the compiled constant
		//if let Some(const_compiled_value) = &const_compiled_value {
		//	self.variant = TanukiExpressionVariant::Constant(const_compiled_value.clone());
		//}
		// Return
		Ok(expression_result)
		//Ok(RValueConstComplicationResult { return_value: const_compiled_value, is_pure: false, ..Default::default() })
	}

	/// Const-compiles a Tanuki expression as an l-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// Returns the l-value if it could be const-compiled.
	pub fn const_compile_l_value(
		&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, result_type: &TanukiType, dependencies_need_const_compiling: &mut bool,
	) -> Result<(Option<CompileTimeLValue>, TanukiType), ErrorAt> {
		let Self { variant, .. } = self;
		Ok(match variant {
			TanukiExpressionVariant::Constant(_) => return Err(Error::Unimplemented("Using a constant as a l-value".into()).at(Some(self.start_line), Some(self.start_column), None)),
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
					main, modules, this_module, module_path, this_function, was_complication_done, local_variables, &TanukiType::Type, dependencies_need_const_compiling, None
				)?.result_value {
					Some(TanukiCompileTimeValue::Type(type_t)) => type_t,
					Some(_) => return Ok((None, TanukiType::Any)),
					None => TanukiType::Any,
				};
				if *dependencies_need_const_compiling {
					return Ok((None, TanukiType::Any));
				}
				value_expression.const_compile_l_value(
					main, modules, this_module, module_path, this_function, was_complication_done, local_variables, &type_t, dependencies_need_const_compiling
				)?;
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
		_main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _this_module: &mut TanukiModule, _this_module_path: &Path, _was_complication_done: &mut bool,
		_local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, _dependencies_need_const_compiling: &mut bool
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		match self {
			_ => Ok(RValueConstComplicationResult::default()),
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
		main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, this_module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		match self {
			Self::Read | Self::Not | Self::Reciprocal | Self::BitshiftRightOne | Self::ComplexConjugate | Self::Signum |
			Self::Negation | Self::SaturatingNegation | Self::WrappingNegation | Self::TryNegation |
			Self::Square | Self::SaturatingSquare | Self::WrappingSquare | Self::TrySquare |
			Self::BitshiftLeftOne | Self::SaturatingBitshiftLeftOne | Self::WrappingBitshiftLeftOne | Self::TryBitshiftLeftOne |
			Self::Dereference | Self::NthToLast | Self::RangeToExclusive | Self::RangeToInclusive => {
				let operand = operand_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				let operand_value = match operand.result_value {
					Some(operand_value) => operand_value,
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				let value = Ok(match (self, operand_value) {
					(Self::Read, operand) => Some(operand),
					(Self::Not, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(!operand)),
					(Self::Negation, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(-operand)),
					(Self::Square, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(&operand * &operand)),
					(Self::BitshiftLeftOne, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand << 1)),
					(Self::BitshiftRightOne, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand >> 1)),
					(Self::Signum, TanukiCompileTimeValue::CompileTimeInt(operand)) => Some(TanukiCompileTimeValue::CompileTimeInt(operand.signum())),
					(Self::Dereference, TanukiCompileTimeValue::Type(pointee_type)) => Some(TanukiCompileTimeValue::Type(TanukiType::Pointer(pointee_type.into()))),
					_ => None,
				})?;
				Ok(RValueConstComplicationResult { result_value: value, is_pure: operand.is_pure, ..Default::default() })
			}
			Self::AddressOf | Self::Increment | Self::SaturatingIncrement | Self::WrappingIncrement | Self::Decrement | Self::SaturatingDecrement | Self::WrappingDecrement => {
				let (l_value, _t_type) = match operand_expression.const_compile_l_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)? {
					(Some(l_value), t_type) => (l_value, t_type),
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				Ok(match (self, l_value) {
					_ => RValueConstComplicationResult::default(),
				})
			}
			Self::MemberAccess => Ok(RValueConstComplicationResult::default()),
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
		main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, this_module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		match self {
			Self::Percent | Self::Factorial | Self::SaturatingFactorial | Self::WrappingFactorial | Self::TryFactorial |
			Self::TryPropagate | Self::Unwrap => {
				let operand = operand_expression.const_compile_r_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
				)?;
				let operand_value = match operand.result_value {
					Some(operand_value) => operand_value,
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				let value = match (self, operand_value) {
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
				};
				Ok(RValueConstComplicationResult { result_value: value, is_pure: operand.is_pure, ..Default::default() })
			}
			Self::Increment | Self::SaturatingIncrement | Self::WrappingIncrement | Self::Decrement | Self::SaturatingDecrement | Self::WrappingDecrement => {
				// TODO
				let (l_value, _t_type) = match operand_expression.const_compile_l_value(
					main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling
				)? {
					(Some(l_value), t_type) => (l_value, t_type),
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				Ok(match (self, l_value) {
					//Self::AddressOf => None,
					_ => RValueConstComplicationResult::default(),
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
		main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, this_module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool, global_variable_assigned_to_name: Option<&str>
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		Ok(match self {
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
				let global_variable_assigned_to_name = match *self == Self::NonShortCircuitOr {
					true => global_variable_assigned_to_name,
					false => None,
				};
				let (lhs_operand, rhs_operand) = (
					lhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?,
					rhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, global_variable_assigned_to_name
					)?
				);
				let (lhs_operand_value, rhs_operand_value) = match (lhs_operand.result_value, rhs_operand.result_value){
					(Some(lhs_operand), Some(rhs_operand)) => (lhs_operand, rhs_operand),
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				let result = match (self, lhs_operand_value, rhs_operand_value) {
					// Arithmetic with big integers
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
							Err(_) => return Ok(RValueConstComplicationResult::default()),
							Ok(exponent) => exponent,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.pow(exponent)))
					}
					// Bitwise with big integers
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
							Err(_) => return Ok(RValueConstComplicationResult::default()),
							Ok(rhs_operand) => rhs_operand,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand << rhs_operand))
					}
					(Self::BitshiftRight, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) => {
						let rhs_operand: u128 = match rhs_operand.try_into() {
							Err(_) => return Ok(RValueConstComplicationResult::default()),
							Ok(rhs_operand) => rhs_operand,
						};
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand >> rhs_operand))
					}
					// Comparison with big integers
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
					// Min/max with big integers
					(Self::Minimum, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.min(rhs_operand))),
					(Self::Maximum, TanukiCompileTimeValue::CompileTimeInt(lhs_operand), TanukiCompileTimeValue::CompileTimeInt(rhs_operand)) =>
						Some(TanukiCompileTimeValue::CompileTimeInt(lhs_operand.max(rhs_operand))),
					// Enum types/functions
					(Self::NonShortCircuitOr, TanukiCompileTimeValue::Type(lhs_type), TanukiCompileTimeValue::Type(rhs_type)) =>
						Some(TanukiCompileTimeValue::Type({
							let mut types = Vec::new();
							if let TanukiType::TypeEnum(lhs_types) = lhs_type {
								types.extend(lhs_types.into_iter());
							}
							else {
								types.push(lhs_type);
							}
							if let TanukiType::TypeEnum(rhs_types) = rhs_type {
								for rhs_type in rhs_types {
									if !types.contains(&rhs_type) {
										types.push(rhs_type);
									}
								}
							}
							else {
								if !types.contains(&rhs_type) {
									types.push(rhs_type);
								}
							}
							TanukiType::TypeEnum(types.into())
						})),
					(Self::NonShortCircuitOr, TanukiCompileTimeValue::FunctionPointer(lhs_pointer), TanukiCompileTimeValue::FunctionPointer(rhs_pointer)) =>
						Some(TanukiCompileTimeValue::FunctionPointerEnum([lhs_pointer, rhs_pointer].iter().cloned().collect())),
					(Self::NonShortCircuitOr, TanukiCompileTimeValue::FunctionPointerEnum(lhs_pointers), TanukiCompileTimeValue::FunctionPointer(rhs_pointer)) =>
						Some(TanukiCompileTimeValue::FunctionPointerEnum(lhs_pointers.iter().cloned().chain(once(rhs_pointer)).collect())),
					(Self::NonShortCircuitOr, TanukiCompileTimeValue::FunctionPointer(lhs_pointer), TanukiCompileTimeValue::FunctionPointerEnum(rhs_pointers)) =>
						Some(TanukiCompileTimeValue::FunctionPointerEnum(once(lhs_pointer).chain(rhs_pointers.iter().cloned()).collect())),
					(Self::NonShortCircuitOr, TanukiCompileTimeValue::FunctionPointerEnum(lhs_pointers), TanukiCompileTimeValue::FunctionPointerEnum(rhs_pointers)) =>
						Some(TanukiCompileTimeValue::FunctionPointerEnum(lhs_pointers.iter().cloned().chain(rhs_pointers.iter().cloned()).collect())),
					_ => None
				};
				RValueConstComplicationResult { result_value: result, is_pure: lhs_operand.is_pure && rhs_operand.is_pure, ..Default::default() }
			}
			Self::MemberAccess | Self::As | Self::SaturatingAs | Self::WrappingAs | Self::TryAs | Self::Pipe => RValueConstComplicationResult::default(),
			Self::None => unreachable!(),
		})
	}
}

impl TanukiInfixTernaryOperator {
	/// Const-compiles a Tanuki infix ternary operator as an r-value. Will set `was_complication_done` to `true` if any compilation was done.
	/// Will set `dependencies_need_const_compiling` to `true` if a variable this depends on has not fully been compiled.
	/// This function must be repeatedly called until `was_complication_done` is not set to `true`.
	/// If this expression has been compiled to a constant value, it will return it.
	pub fn const_compile_r_value(
		&mut self, lhs_expression: &mut TanukiExpression, mhs_expression: &mut TanukiExpression, rhs_expression: &mut TanukiExpression,
		main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], this_module: &mut TanukiModule, this_module_path: &Path, this_function: &mut Option<&mut TanukiFunction>,
		was_complication_done: &mut bool,
		local_variables: &mut Vec<HashMap<Box<str>, (TanukiType, Option<TanukiCompileTimeValue>)>>, _result_type: &TanukiType, dependencies_need_const_compiling: &mut bool
	) -> Result<RValueConstComplicationResult, ErrorAt> {
		match self {
			Self::NonShortCircuitingConditional | Self::ShortCircuitingConditional => {
				let (lhs_operand, mhs_operand, rhs_operand) = (
					lhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?,
					mhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?,
					rhs_expression.const_compile_r_value(
						main, modules, this_module, this_module_path, this_function, was_complication_done, local_variables, &TanukiType::Any, dependencies_need_const_compiling, None
					)?
				);
				let (lhs_operand_result, mhs_operand_result, rhs_operand_result) = match (lhs_operand.result_value, mhs_operand.result_value, rhs_operand.result_value) {
					(Some(lhs_operand), Some(mhs_operand), Some(rhs_operand)) => (lhs_operand, mhs_operand, rhs_operand),
					_ => return Ok(RValueConstComplicationResult::default()),
				};
				Ok(match (self, lhs_operand_result, mhs_operand_result, rhs_operand_result) {
					_ => RValueConstComplicationResult::default()
				})
			}
		}
	}
}

#[derive(Debug, Default)]
pub struct RValueConstComplicationResult {
	/// Contains a value if this expression always gives the same value.
	pub result_value: Option<TanukiCompileTimeValue>,
	/// Contains a type if this expression always gives the same value type.
	pub result_type: Option<TanukiType>,
	/// The expression can be removed and has no side-effects if this is true.
	pub is_pure: bool,
}