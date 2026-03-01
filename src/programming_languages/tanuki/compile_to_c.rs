use std::{any::Any, collections::HashMap, path::Path};

use crate::{Main, Os, error::{Error, ErrorAt}, programming_languages::{c::{expression::CExpression, l_value::CLValue, module::CModule, module_element::{CFunctionParameter, CModuleElement}, statement::{CCompoundStatement, CInitializer, CStatement}, types::CType}, tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, module::TanukiModule, t_type::TanukiType}}, traits::module::Module};

impl TanukiModule {
	pub fn compile_to_c_module(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)]) -> Result<CModule, ErrorAt> {
		// Create module
		let mut c_module = CModule::new();
		// Add imports needed for the language
		c_module.push_element(CModuleElement::AngleInclude("stdint.h".into()));
		c_module.push_element(CModuleElement::AngleInclude("stdlib.h".into()));
		// Compile functions
		for function in self.functions.iter() {
			c_module.push_element(function.as_ref().unwrap().compile_to_c(main, modules)?);
		}
		// Compile entrypoint
		if let Some(entrypoint_name) = &self.entrypoint {
			for global_constant in self.global_constants.iter() {
				let global_constant = global_constant.as_ref().unwrap();
				if &global_constant.name == entrypoint_name {
					let function_name = match &global_constant.value_expression.variant {
						TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Function(name, _module_path)) => name,
						_ => return Err(Error::EntrypointOnNonFunction.at(Some(global_constant.start_line), Some(global_constant.start_column), None)),
					};
					for function in self.functions.iter() {
						let entrypoint_wrapped_function = function.as_ref().unwrap();
						if &entrypoint_wrapped_function.name != function_name {
							continue;
						}
						if entrypoint_wrapped_function.parameters.len() != 0 {
							return Err(
								Error::Unimplemented(
									"Entrypoint with parameters".into()).at(Some(entrypoint_wrapped_function.parameters[0].start_line), Some(entrypoint_wrapped_function.parameters[0].start_column), None
								)
							);
						}
						match main.os {
							Os::Unix => {
								let mut c_compound_statement = CCompoundStatement::new();
								c_compound_statement.push_statement(CStatement::VariableDeclaration(
									CType::U8, "result".into(), Some(CInitializer::Expression(CExpression::FunctionCall(entrypoint_wrapped_function.name.clone().into(), Default::default())).into())
								));
								c_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("exit".into(), [CLValue::Variable("result".into()).into()].into())).into());
								c_module.push_element(CModuleElement::FunctionDefinition { return_type: CType::Void, name: "_start".into(), parameters: Default::default(), body: Box::new(c_compound_statement) });
							}
							Os::Windows => {
								let mut c_compound_statement = CCompoundStatement::new();
								c_compound_statement.push_statement(CStatement::VariableDeclaration(
									CType::U8, "result".into(), Some(CInitializer::Expression(CExpression::FunctionCall(entrypoint_wrapped_function.name.clone().into(), Default::default())).into())
								));
								c_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("exit".into(), [CLValue::Variable("result".into()).into()].into())).into());
								c_module.push_element(CModuleElement::FunctionDefinition { return_type: CType::Void, name: "WinMain".into(), parameters: Default::default(), body: Box::new(c_compound_statement) });
							}
						}
						//let c_compound_statement = CCompoundStatement::new();
						//c_module.push_element(CModuleElement::FunctionDefinition { return_type: CType::Void, name: "_start".into(), parameters: Default::default(), body: Box::new(c_compound_statement) });
						break;
					}
				}
			}
		}
		// Return
		Ok(c_module)
	}
}

impl TanukiFunction {
	/// Compiles the Tanuki module to a C module
	pub fn compile_to_c(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)]) -> Result<CModuleElement, ErrorAt> {
		// Get return type
		let return_type = match &self.return_type {
			Some(return_type) => match return_type {
				TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(return_type)), .. } => return_type,
				_ => unreachable!(),
			}
			None => &TanukiType::Void,
		};
		// Compile parameters
		let mut local_variables = Vec::new();
		local_variables.push(HashMap::new());
		let mut c_parameters = Vec::new();
		for parameter in self.parameters.iter() {
			let t_type = match &parameter.t_type {
				Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)), .. }) => t_type,
				Some(_) => unreachable!(),
				None => todo!(),
			};
			local_variables.last_mut().unwrap().insert(parameter.name.clone(), t_type.clone());
			let c_type = t_type.compile_to_c(main)?;
			c_parameters.push(CFunctionParameter::new(c_type, parameter.name.clone()));
		}
		// Compile body
		let mut function_temp_variable_count = 0;
		let mut c_compound_statement = CCompoundStatement::new();
		let (body_result_value_name, returned_type) = self.body.compile_r_value_to_c(
			main, modules, &mut c_compound_statement, &mut function_temp_variable_count, &mut local_variables
		)?;
		// Return body value if it is not a void value
		if let Some(body_result_value_name) = body_result_value_name {
			c_compound_statement.push_statement(CStatement::Return(Some(CLValue::Variable(body_result_value_name).into())));
		}
		if return_type != &returned_type {
			todo!()
		}
		// Pack into struct
		Ok(CModuleElement::FunctionDefinition {
			return_type: return_type.compile_to_c(main)?, name: self.name.clone(), parameters: c_parameters.into(), body: Box::new(c_compound_statement)
		})
	}
}

impl TanukiType {
	/// Compiles a Tanuki type to a C Type
	pub fn compile_to_c(&self, _main: &mut Main) -> Result<CType, ErrorAt> {
		Ok(match self {
			Self::Void => CType::Void,
			Self::U(bit_width) => match bit_width {
				8 => CType::U8,
				16 => CType::U16,
				32 => CType::U32,
				64 => CType::U64,
				_ => unreachable!(),
			}
			Self::I(bit_width) => match bit_width {
				8 => CType::I8,
				16 => CType::I16,
				32 => CType::I32,
				64 => CType::I64,
				_ => unreachable!(),
			}
			_ => todo!(),
		})
	}
}

impl TanukiExpression {
	/// Compiles an expression that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize, local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match &self.variant {
			// Constants
			TanukiExpressionVariant::Constant(constant) => constant.compile_to_c(main, modules, insert_into, function_temp_variable_count),
			// Binary operators compile the arguments, assign to temp variables, add the variables and assign to a result temp variable that is returned
			TanukiExpressionVariant::Addition(lhs_expression, rhs_expression) => {
				let (lhs_result_variable, lhs_type) = lhs_expression.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				let (rhs_result_variable, rhs_type) = rhs_expression.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				if lhs_type != rhs_type {
					return Err(Error::NotYetImplemented("+ between different types".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				if !matches!(lhs_type, TanukiType::U(_) | TanukiType::I(_)) {
					return Err(Error::NotYetImplemented("Operator overloading".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				let name = format!("_tnk_temp_add_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				let c_expression = CExpression::Add(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
				insert_into.push_statement(CStatement::VariableDeclaration(lhs_type.compile_to_c(main)?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
				Ok((Some(name.into()), lhs_type))
			}
			TanukiExpressionVariant::Assignment(lhs, rhs) => {
				let (lhs_result_variable_name, lhs_type) = lhs.compile_l_value_to_c(main, insert_into, function_temp_variable_count, local_variables, &TanukiType::Any)?;
				let (rhs_result_variable_name, rhs_type) = rhs.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				if lhs_type != rhs_type {
					return Err(Error::NotYetImplemented(format!("= between different types {lhs_type:?} and {rhs_type:?}").into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				if rhs_type != TanukiType::Void {
					insert_into.push_statement(CExpression::Assignment(
						CLValue::Dereference(CLValue::Variable(lhs_result_variable_name.unwrap()).into()).into(),
						CLValue::Variable(rhs_result_variable_name.clone().unwrap()).into(),
					).into());
				}
				Ok((rhs_result_variable_name, rhs_type))
			}
			// For variables, we find the variable and read it
			TanukiExpressionVariant::Variable(name) => 'a: {
				for local_variable_level in local_variables.iter() {
					if let Some(local_variable_type) = local_variable_level.get(name) {
						break 'a Ok((Some(name.clone()), local_variable_type.clone()));
					}
				}
				unreachable!();
			}
			//
			TanukiExpressionVariant::Block { sub_expressions, has_return_value } => {
				let sub_expressions_len = sub_expressions.len();
				let mut return_variable_name = None;
				let mut return_type = TanukiType::Void;
				// Push a local variable scope level
				local_variables.push(HashMap::new());
				// Create c compound statement
				let mut c_compound_statement = CCompoundStatement::new();
				// Compile each variable
				for (x, sub_expression) in sub_expressions.iter().enumerate() {
					let sub_expression_result = sub_expression.compile_r_value_to_c(main, modules, &mut c_compound_statement, function_temp_variable_count, local_variables)?;
					// If this is the block's sub-expression that yields it's result value and the value yielded is not void
					if let Some(sub_expression_return_variable_name) = sub_expression_result.0 && x == sub_expressions_len - 1 && *has_return_value {
						// Create the temp variable to assign the block result to
						return_type = sub_expression_result.1;
						let name = format!("_tnk_temp_block_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						return_variable_name = Some(name.clone().into());
						// Assign the block result to the temp variable
						insert_into.push_statement(CStatement::VariableDeclaration(return_type.compile_to_c(main)?, name.clone().into(), None));
						c_compound_statement.push_statement(CExpression::Assignment(
							CLValue::Variable(name.clone().into()).into(),
							CLValue::Variable(sub_expression_return_variable_name.into()).into(),
						).into());
					}
				}
				// Push compound statement
				insert_into.push_statement(CStatement::CompoundStatement(c_compound_statement));
				// Pop the local variable scope level
				local_variables.pop();
				// Return
				Ok((return_variable_name, return_type))
			}
			//TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
			//	todo!()
			//}
			_ => return Err(Error::NotYetImplemented(format!("{:?} expression", self.variant)).at(Some(self.start_line), Some(self.start_column), None)),
		}
	}

	/// Compiles an expression that is being used as a l-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains a pointer to the l-value's value'.
	pub fn compile_l_value_to_c(
		&self, main: &mut Main, insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize, local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>, t_type: &TanukiType,
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match &self.variant {
			TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => {
				let t_type = match &type_expression.variant {
					TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)) => t_type,
					_ => todo!(),
				};
				value_expression.compile_l_value_to_c(main, insert_into, function_temp_variable_count, local_variables, t_type)
			}
			TanukiExpressionVariant::Variable(name) => 'a: {
				for local_variable_level in local_variables.iter() {
					if let Some(local_variable_type) = local_variable_level.get(name) {
						let temp_var_name = format!("_tnk_temp_l_var_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						insert_into.push_statement(CStatement::VariableDeclaration(
							CType::PointerTo(t_type.compile_to_c(main)?.into()).into(),
							temp_var_name.clone().into(), Some(CInitializer::Expression(CExpression::TakeReference(CLValue::Variable(temp_var_name.clone().into()).into())).into())
						));
						break 'a Ok((Some(temp_var_name.into()), local_variable_type.clone()));
					}
				}
				local_variables.iter_mut().last().unwrap().insert(name.clone(), t_type.clone());
				insert_into.push_statement(CStatement::VariableDeclaration(t_type.compile_to_c(main)?.into(), name.clone(), None));
				let temp_var_name = format!("_tnk_temp_l_var_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				insert_into.push_statement(CStatement::VariableDeclaration(
					CType::PointerTo(t_type.compile_to_c(main)?.into()).into(),
					temp_var_name.clone().into(), Some(CInitializer::Expression(CExpression::TakeReference(CLValue::Variable(temp_var_name.clone().into()).into())).into())
				));
				Ok((Some(temp_var_name.into()), t_type.clone()))
			}
			_ => return Err(Error::NotYetImplemented(format!("{:?} l-value expression", self.variant)).at(Some(self.start_line), Some(self.start_column), None)),
		}
	}
}

impl TanukiCompileTimeValue {
	/// Converts a Tanuki compile time value into a C constant.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_to_c(
		&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		let t_type = self.get_type();
		let c_type = t_type.compile_to_c(main)?;
		let c_expression = match self {
			TanukiCompileTimeValue::U(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::I(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::Void => return Ok((None, TanukiType::Void)),
			TanukiCompileTimeValue::Function(name, target_module_name) => {
				for (module_name, _, module) in modules.iter() {
					if target_module_name != module_name {
						continue;
					}
					let module: &TanukiModule = match ((&**module.as_ref().unwrap()) as &dyn Any).downcast_ref() {
						Some(module) => module,
						None => return Err(Error::Unimplemented("Linking to non-Tanuki modules".into()).at(None, None, None)),
					};
					for function in module.functions.iter() {
						let function = function.as_ref().unwrap();
						if &function.name != name {
							continue;
						}
						let return_type = match &function.return_type {
							None => return Err(Error::Unimplemented("Function without return type".into()).at(None, None, None)),
							Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => value,
							_ => unreachable!(),
						};
						let mut parameter_types = Vec::new();
						for parameter in function.parameters.iter() {
							match &parameter.t_type {
								None => return Err(Error::Unimplemented("Function argument without type".into()).at(None, None, None)),
								Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(value)), .. }) => parameter_types.push(value.clone()),
								_ => unreachable!(),
							};
						}
						let t_type = TanukiType::FunctionPointer(Box::new(return_type.clone()), parameter_types.into());
						let c_expression = CExpression::TakeReference(CLValue::Variable("name".into()).into());
						let temp_name = format!("_tnk_temp_func_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						insert_into.push_statement(CStatement::VariableDeclaration(c_type, temp_name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
						return Ok((Some(temp_name.into()), t_type))
					}
					unreachable!();
				}
				unreachable!()
			},
			_ => todo!(),
		};
		let name = format!("_tnk_temp_var_{function_temp_variable_count}");
		*function_temp_variable_count += 1;
		insert_into.push_statement(CStatement::VariableDeclaration(c_type, name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
		Ok((Some(name.into()), t_type))
	}
}