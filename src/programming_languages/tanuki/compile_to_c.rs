use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}, num::NonZeroUsize, path::Path};

use crate::{Main, Os, error::{Error, ErrorAt}, programming_languages::{c::{expression::CExpression, l_value::CLValue, module::CModule, module_element::{CFunctionParameter, CModuleElement}, statement::{CCompoundStatement, CInitializer, CStatement}, types::CType}, tanuki::{compile_time_value::{FunctionPointer, TanukiCompileTimeValue}, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, module::TanukiModule, t_type::{FunctionPointerType, TanukiType}, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiNullaryOperator, TanukiPostfixUnaryOperator, TanukiPrefixUnaryOperator}}}, traits::module::Module};

impl TanukiModule {
	pub fn compile_to_c_module(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)]) -> Result<CModule, ErrorAt> {
		// Create module
		let mut c_module = CModule::new();
		// Add imports needed for the language
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdint.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdlib.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stddef.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdbool.h".into()));
		// Declare types used
		// TODO
		// Add imports of other file headers
		for mangled_module_name in self.mangled_module_names_to_include_in_c.iter() {
			c_module.push_element(CModuleElement::DoubleQuotesIncludeInHeader(format!("{mangled_module_name}.h").into()));
		}
		// Compile functions
		for function in self.functions.iter() {
			function.as_ref().unwrap().compile_to_c(main, &mut c_module, modules)?;
		}
		// Compile entrypoint
		if let Some(entrypoint_name) = &self.entrypoint {
			for global_constant in self.global_constants.iter() {
				let global_constant = global_constant.as_ref().unwrap();
				if &global_constant.name == entrypoint_name {
					let (name, _module_path, return_type, parameter_types) = match &global_constant.value_expression.variant {
						TanukiExpressionVariant::Constant(TanukiCompileTimeValue::ConcreteFunctionPointer(FunctionPointer { name, module_path, return_type, parameter_types })) =>
							(name, module_path, &**return_type, parameter_types),
						_ => return Err(Error::EntrypointOnNonFunction.at(Some(global_constant.start_line), Some(global_constant.start_column), None)),
					};
					// Check the parameter and return types
					if return_type != &TanukiType::U(8) {
						return Err(Error::Unimplemented(
							"Non-@u(8) return type".into()).at(Some(global_constant.value_expression.start_line), Some(global_constant.value_expression.start_column), None
						));
					}
					if parameter_types.len() != 0 {
						return Err(Error::Unimplemented(
							"Entrypoint with parameters".into()).at(Some(global_constant.value_expression.start_line), Some(global_constant.value_expression.start_column), None
						));
					}
					// Get name
					let mut hasher = DefaultHasher::new();
					(parameter_types, return_type).hash(&mut hasher);
					let concrete_type_function_name = format!("{}_{}", name, hasher.finish());
					// Create entrypoint wrapper function
					match main.os {
						Os::Unix => {
							let mut c_compound_statement = CCompoundStatement::new();
							c_compound_statement.push_statement(CStatement::VariableDeclaration(
								CType::U8, "result".into(), Some(CInitializer::Expression(CExpression::FunctionCall(concrete_type_function_name.clone().into(), Default::default())).into())
							));
							c_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("exit".into(), [CLValue::Variable("result".into()).into()].into())).into());
							c_module.push_element(CModuleElement::FunctionDefinition { return_type: CType::Void, name: "_start".into(), parameters: Default::default(), body: Box::new(c_compound_statement) });
						}
						Os::Windows => {
							let mut c_compound_statement = CCompoundStatement::new();
							c_compound_statement.push_statement(CStatement::VariableDeclaration(
								CType::U8, "result".into(), Some(CInitializer::Expression(CExpression::FunctionCall(concrete_type_function_name.clone().into(), Default::default())).into())
							));
							c_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("exit".into(), [CLValue::Variable("result".into()).into()].into())).into());
							c_module.push_element(CModuleElement::FunctionDefinition { return_type: CType::Void, name: "WinMain".into(), parameters: Default::default(), body: Box::new(c_compound_statement) });
						}
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
	pub fn compile_to_c(&self, main: &mut Main, insert_into: &mut CModule, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)]) -> Result<(), ErrorAt> {
		// If this is a function definition
		if self.body.is_none() {
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
				let parameter = parameter.as_ref().unwrap();
				let t_type = match &parameter.t_type {
					Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)), .. }) => t_type,
					Some(_) => unreachable!(),
					None => return Err(Error::ExpectedType.at(Some(parameter.start_line), Some(parameter.start_column), None)),
				};
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), t_type.clone());
				let c_type = t_type.compile_to_c(main, Some(parameter.start_line), Some(parameter.start_column))?;
				c_parameters.push(CFunctionParameter::new(c_type, parameter.name.clone()));
			}
			insert_into.push_element(CModuleElement::FunctionDeclaration {
				return_type: return_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?, name: self.name.clone(), parameters: c_parameters.into()
			});
		}
		// Compile concrete type function bodies
		for ((parameter_types, return_type), body_for_concrete_type) in self.bodies_for_concrete_types.as_ref().unwrap().iter() {
			// Do not compile the concrete function body if cannot exist at run time
			if parameter_types.iter().any(|parameter_type| !parameter_type.can_exist_at_run_time()) || !return_type.can_exist_at_run_time() {
				continue;
			}
			// Get name
			let mut hasher = DefaultHasher::new();
			(parameter_types, return_type).hash(&mut hasher);
			let concrete_type_function_name = format!("{}_{}", self.name, hasher.finish());
			// Compile parameters
			let mut local_variables = Vec::new();
			local_variables.push(HashMap::new());
			let mut c_parameters = Vec::new();
			for (x, parameter) in self.parameters.iter().enumerate() {
				let parameter = parameter.as_ref().unwrap();
				local_variables.last_mut().unwrap().insert(parameter.name.clone(), parameter_types[x].clone());
				let c_type = parameter_types[x].compile_to_c(main, Some(parameter.start_line), Some(parameter.start_column))?;
				c_parameters.push(CFunctionParameter::new(c_type, parameter.name.clone()));
			}
			// Compile body
			let mut function_temp_variable_count = 0;
			let mut c_compound_statement = CCompoundStatement::new();
			let (body_result_value_name, returned_type) = body_for_concrete_type.compile_r_value_to_c(
				main, modules, &mut c_compound_statement, &mut function_temp_variable_count, &mut local_variables
			)?;
			// Return body value if it is not a void value
			if let Some(body_result_value_name) = body_result_value_name {
				c_compound_statement.push_statement(CStatement::Return(Some(CLValue::Variable(body_result_value_name).into())));
			}
			if &**return_type != &returned_type {
				return Err(Error::TypeMismatch((format!("{returned_type:?}"), format!("{return_type:?}"))).at(Some(body_for_concrete_type.start_line), Some(body_for_concrete_type.start_column), None));
			}
			// Pack into struct
			let c_function = CModuleElement::FunctionDefinition {
				return_type: return_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?,
				name: concrete_type_function_name.into(), parameters: c_parameters.into(), body: Box::new(c_compound_statement)
			};
			insert_into.push_element(c_function);
		}
		Ok(())
	}
}

impl TanukiType {
	/// Compiles a Tanuki type to a C Type
	pub fn compile_to_c(&self, main: &mut Main, line: Option<NonZeroUsize>, column: Option<NonZeroUsize>) -> Result<CType, ErrorAt> {
		Ok(match self {
			Self::Void => CType::Void,
			Self::U(bit_width) => match bit_width {
				8  => CType::U8,
				16 => CType::U16,
				32 => CType::U32,
				64 => CType::U64,
				_ => unreachable!(),
			}
			Self::I(bit_width) => match bit_width {
				8  => CType::I8,
				16 => CType::I16,
				32 => CType::I32,
				64 => CType::I64,
				_ => unreachable!(),
			}
			Self::FunctionPointer(FunctionPointerType { return_type, parameter_types }) => {
				let mut c_parameter_types = Vec::new();
				for parameter_type in parameter_types.iter() {
					c_parameter_types.push(parameter_type.compile_to_c(main, line, column)?);
				}
				CType::FunctionPointer(return_type.compile_to_c(main, line, column)?.into(), c_parameter_types.into())
			}
			Self::Bool => CType::Bool,
			Self::Pointer(pointee_type) => CType::PointerTo(pointee_type.compile_to_c(main, line, column)?.into()),
			Self::Any | Self::CompileTimeChar | Self::CompileTimeFloat | Self::CompileTimeInt | Self::CompileTimeString | Self::Type =>
				return Err(Error::TypeCannotExistAtRunTime.at(line, column, None)),
			_ => return Err(Error::Unimplemented(format!("Compiling value {self:?}")).at(line, column, None)),
		})
	}
}

impl TanukiExpression {
	/// Compiles an expression that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize, local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match &self.variant {
			// Constants
			TanukiExpressionVariant::Constant(constant) => constant.compile_to_c(main, modules, insert_into, function_temp_variable_count, Some(self.start_line), Some(self.start_column)),
			// Operators
			TanukiExpressionVariant::NullaryOperator(operator) =>
				operator.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables, self.start_line, self.start_column),
			TanukiExpressionVariant::PrefixUnaryOperator(operator, operand) =>
				operator.compile_r_value_to_c(&**operand, main, modules, insert_into, function_temp_variable_count, local_variables, self.start_line, self.start_column),
			TanukiExpressionVariant::PostfixUnaryOperator(operator, operand) =>
				operator.compile_r_value_to_c(&**operand, main, modules, insert_into, function_temp_variable_count, local_variables, self.start_line, self.start_column),
			TanukiExpressionVariant::InfixBinaryOperator(operator, lhs_expression, rhs_expression) =>
				operator.compile_r_value_to_c(&**lhs_expression, &**rhs_expression, main, modules, insert_into, function_temp_variable_count, local_variables, self.start_line, self.start_column),
			TanukiExpressionVariant::InfixTernaryOperator(operator, lhs_expression, mhs_expression, rhs_expression) =>
				operator.compile_r_value_to_c(&**lhs_expression, &**mhs_expression, &**rhs_expression, main, modules, insert_into, function_temp_variable_count, local_variables, self.start_line, self.start_column),
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
				return Err(Error::UnableToConstCompile.at(Some(self.start_line), Some(self.start_column), None));
			}
			// Blocks
			TanukiExpressionVariant::Block { sub_expressions, return_expressions } => {
				//let sub_expressions_len = sub_expressions.len();
				let mut return_variable_name = None;
				let mut return_type = TanukiType::Void;
				// Push a local variable scope level
				local_variables.push(HashMap::new());
				// Create c compound statement
				let mut c_compound_statement = CCompoundStatement::new();
				// Compile each sub-expression
				for sub_expression in sub_expressions.iter() {
					sub_expression.compile_r_value_to_c(main, modules, &mut c_compound_statement, function_temp_variable_count, local_variables)?;
				}
				let return_expressions_len = return_expressions.len();
				for (_x, (return_expression_name, return_expression)) in return_expressions.iter().enumerate() {
					let return_expression_result = return_expression.compile_r_value_to_c(main, modules, &mut c_compound_statement, function_temp_variable_count, local_variables)?;
					// If this is the block's sub-expression that yields it's result value and the value yielded is not void
					if let Some(return_expression_result_variable_name) = return_expression_result.0 && return_expressions_len == 1 && return_expression_name.is_none() {
						// Create the temp variable to assign the block result to
						return_type = return_expression_result.1;
						let name = format!("_tnk_temp_block_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						return_variable_name = Some(name.clone().into());
						// Assign the block result to the temp variable
						insert_into.push_statement(
							CStatement::VariableDeclaration(return_type.compile_to_c(main, Some(return_expression.start_line), Some(return_expression.start_column))?, name.clone().into(), None)
						);
						c_compound_statement.push_statement(CExpression::Assignment(
							CLValue::Variable(name.clone().into()).into(),
							CLValue::Variable(return_expression_result_variable_name).into(),
						).into());
					}
					else {
						return Err(Error::NotYetImplemented("Blocks with struct return result".into()).at(Some(return_expression.start_line), Some(return_expression.start_column), None));
					}
				}
				// Push compound statement
				insert_into.push_statement(CStatement::CompoundStatement(c_compound_statement));
				// Pop the local variable scope level
				local_variables.pop();
				// Return
				Ok((return_variable_name, return_type))
			}
			TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
				let (function_pointer_result_variable, function_pointer_type) = function_pointer.compile_r_value_to_c(
					main, modules, insert_into, function_temp_variable_count, local_variables
				)?;
				let (return_type, parameter_types) = match function_pointer_type {
					TanukiType::FunctionPointer(FunctionPointerType { return_type, parameter_types }) => (return_type, parameter_types),
					_ => return Err(Error::TypeMismatch((format!("{function_pointer_type:?}"), format!("{:?}", TanukiType::FunctionPointer(FunctionPointerType {
						return_type: TanukiType::Any.into(), parameter_types: Default::default()
					})))).at(Some(self.start_line), Some(self.start_column), None)),
				};
				if parameter_types.len() != arguments.len() {
					return Err(Error::ArgumentCountMismatch((arguments.len(), parameter_types.len())).at(Some(self.start_line), Some(self.start_column), None));
				}
				let mut argument_results: Vec<CExpression> = Vec::new();
				let mut argument_types = Vec::new();
				for (x, argument) in arguments.iter().enumerate() {
					let result = argument.compile_r_value_to_c(
						main, modules, insert_into, function_temp_variable_count, local_variables
					)?;
					if &result.1 != &parameter_types[x] {
						return Err(Error::TypeMismatch((format!("{:?}", result.1), format!("{:?}", parameter_types[x]))).at(Some(argument.start_line), Some(argument.start_column), None));
					}
					argument_results.push(CLValue::Variable(result.0.unwrap()).into());
					argument_types.push(result.1);
				}
				let name = format!("_tnk_temp_fn_call_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				let c_expression = CExpression::FunctionPointerCall(CLValue::Variable(function_pointer_result_variable.unwrap().into()).into(), argument_results.into());
				insert_into.push_statement(
					CStatement::VariableDeclaration(return_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into()))
				);
				Ok((Some(name.into()), *return_type))
			}
			TanukiExpressionVariant::Transmute { to_transmute, transmute_to_type } => {
				// Compile value to be transmuted
				let (to_transmute_result_name, to_transmute_type) = to_transmute.compile_r_value_to_c(
					main, modules, insert_into, function_temp_variable_count, local_variables
				)?;
				// Get type to be transmuted to
				let transmute_to_type = match &transmute_to_type.variant {
					TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(type_t)) => type_t,
					TanukiExpressionVariant::Constant(_) => return Err(Error::ExpectedType.at(Some(transmute_to_type.start_line), Some(transmute_to_type.start_column), None)),
					_ => return Err(Error::UnableToConstCompile.at(Some(transmute_to_type.start_line), Some(transmute_to_type.start_column), None)),
				};
				// Check if the type can be transmuted
				match (&to_transmute_type, &transmute_to_type) {
					(TanukiType::Pointer(_), TanukiType::Pointer(_)) => {},
					(TanukiType::Pointer(_), TanukiType::U(_)) => {},
					(TanukiType::U(_), TanukiType::Pointer(_)) => {},
					_ => return Err(Error::Unimplemented(format!("Transmuting from type {to_transmute_type:?} to {transmute_to_type:?}")).at(Some(self.start_line), Some(self.start_column), None)),
				}
				// Create transmute code
				let name = format!("_tnk_temp_transmute_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				insert_into.push_statement(
					CStatement::VariableDeclaration(
						transmute_to_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?, name.clone().into(),
						Some(CInitializer::Expression(CLValue::Variable(to_transmute_result_name.unwrap().into()).into()).into()),
					)
				);
				// Return
				Ok((Some(name.into()), transmute_to_type.clone()))
			}
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
					_ => return Err(Error::UnableToConstCompile.at(Some(type_expression.start_line), Some(type_expression.start_column), None)),
				};
				value_expression.compile_l_value_to_c(main, insert_into, function_temp_variable_count, local_variables, t_type)
			}
			TanukiExpressionVariant::Variable(name) => 'a: {
				for local_variable_level in local_variables.iter() {
					if let Some(local_variable_type) = local_variable_level.get(name) {
						let temp_var_name = format!("_tnk_temp_l_var_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						insert_into.push_statement(CStatement::VariableDeclaration(
							CType::PointerTo(t_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?.into()).into(),
							temp_var_name.clone().into(), Some(CInitializer::Expression(CExpression::TakeReference(CLValue::Variable(name.clone().into()).into())).into())
						));
						break 'a Ok((Some(temp_var_name.into()), local_variable_type.clone()));
					}
				}
				local_variables.iter_mut().last().unwrap().insert(name.clone(), t_type.clone());
				insert_into.push_statement(CStatement::VariableDeclaration(t_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?.into(), name.clone(), None));
				let temp_var_name = format!("_tnk_temp_l_var_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				insert_into.push_statement(CStatement::VariableDeclaration(
					CType::PointerTo(t_type.compile_to_c(main, Some(self.start_line), Some(self.start_column))?.into()).into(),
					temp_var_name.clone().into(), Some(CInitializer::Expression(CExpression::TakeReference(CLValue::Variable(name.clone().into()).into())).into())
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
		&self, main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize,
		line: Option<NonZeroUsize>, column: Option<NonZeroUsize>
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		let c_expression = match self {
			TanukiCompileTimeValue::U(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::I(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::Void => return Ok((None, TanukiType::Void)),
			TanukiCompileTimeValue::Bool(value) => match value {
				true  => CExpression::TrueConstant,
				false => CExpression::FalseConstant,
			}
			//TanukiCompileTimeValue::FunctionPointer(FunctionPointer { name: function_name, module_path: _, return_type, parameter_types }) => {
			//	let t_type = TanukiType::FunctionPointer(FunctionPointerType { return_type: return_type.clone(), parameter_types: parameter_types.clone() });
			//	let c_expression = CExpression::TakeReference(CLValue::Variable(function_name.clone().into()).into());
			//	let temp_name = format!("_tnk_temp_func_var_{function_temp_variable_count}");
			//	*function_temp_variable_count += 1;
			//	insert_into.push_statement(CStatement::VariableDeclaration(t_type.compile_to_c(main, line, column)?, temp_name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
			//	return Ok((Some(temp_name.into()), t_type))
			//}
			TanukiCompileTimeValue::ConcreteFunctionPointer(FunctionPointer { name: function_name, module_path: _, return_type, parameter_types }) => {
				let mut hasher = DefaultHasher::new();
				(parameter_types, return_type).hash(&mut hasher);
				let concrete_type_function_name = format!("{}_{}", function_name, hasher.finish());
				let t_type = TanukiType::FunctionPointer(FunctionPointerType { return_type: return_type.clone(), parameter_types: parameter_types.clone() });
				let c_expression = CExpression::TakeReference(CLValue::Variable(concrete_type_function_name.clone().into()).into());
				let temp_name = format!("_tnk_temp_func_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				insert_into.push_statement(CStatement::VariableDeclaration(t_type.compile_to_c(main, line, column)?, temp_name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
				return Ok((Some(temp_name.into()), t_type))
			}
			TanukiCompileTimeValue::LinkedFunctionPointer(function_name, return_type, parameter_types) => {
				let t_type = TanukiType::FunctionPointer(FunctionPointerType { return_type: return_type.clone(), parameter_types: parameter_types.clone() });
				let c_expression = CExpression::TakeReference(CLValue::Variable(function_name.clone().into()).into());
				let temp_name = format!("_tnk_temp_link_func_var_{function_temp_variable_count}");
				*function_temp_variable_count += 1;
				insert_into.push_statement(CStatement::VariableDeclaration(t_type.compile_to_c(main, line, column)?, temp_name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
				return Ok((Some(temp_name.into()), t_type))
			}
			_ => return Err(Error::NotYetImplemented(format!("{self:?} value")).at(line, column, None)),
		};
		let t_type = self.get_type();
		let c_type = t_type.compile_to_c(main, line, column)?;
		let name = format!("_tnk_temp_var_{function_temp_variable_count}");
		*function_temp_variable_count += 1;
		insert_into.push_statement(CStatement::VariableDeclaration(c_type, name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
		Ok((Some(name.into()), t_type))
	}
}

impl TanukiNullaryOperator {
	/// Compiles a nullary operator that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self,
		_main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _insert_into: &mut CCompoundStatement, _function_temp_variable_count: &mut usize, _local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>,
		start_line: NonZeroUsize, start_column: NonZeroUsize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match self {
			_ => return Err(Error::NotYetImplemented(format!("{self} operator").into()).at(Some(start_line), Some(start_column), None)),
		}
	}
}

impl TanukiPrefixUnaryOperator {
	/// Compiles a prefix unary operator that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, operand: &TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize,
		local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>,
		start_line: NonZeroUsize, start_column: NonZeroUsize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match self {
			Self::WrappingNegation => {
				let (operand_result_variable, operand_type) = operand.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				match (self, &operand_type) {
					(Self::WrappingNegation, TanukiType::U(_)) => {
						let name = format!("_tnk_temp_neg_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::Negate(CLValue::Variable(operand_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							operand_type.compile_to_c(main, Some(operand.start_line), Some(operand.start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), operand_type))
					}
					_ => return Err(Error::NotYetImplemented(format!("{self} operator between on {operand_type:?} types").into()).at(Some(start_line), Some(start_column), None)),
				}
			},
			Self::Not => {
				let (operand_result_variable, operand_type) = operand.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				match (self, &operand_type) {
					(Self::WrappingNegation, TanukiType::U(_)) => {
						let name = format!("_tnk_temp_bit_not_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseNot(CLValue::Variable(operand_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							operand_type.compile_to_c(main, Some(operand.start_line), Some(operand.start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), operand_type))
					}
					_ => return Err(Error::NotYetImplemented(format!("{self} operator on {operand_type:?} types").into()).at(Some(start_line), Some(start_column), None)),
				}
			},
			_ => return Err(Error::NotYetImplemented(format!("{self} operator").into()).at(Some(start_line), Some(start_column), None)),
		}
	}
}

impl TanukiPostfixUnaryOperator {
	/// Compiles a postfix unary operator that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, _operand: &TanukiExpression,
		_main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _insert_into: &mut CCompoundStatement, _function_temp_variable_count: &mut usize, _local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>,
		start_line: NonZeroUsize, start_column: NonZeroUsize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match self {
			_ => return Err(Error::NotYetImplemented(format!("{self} operator").into()).at(Some(start_line), Some(start_column), None)),
		}
	}
}

impl TanukiInfixBinaryOperator {
	/// Compiles a infix binary operator that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, lhs_expression: &TanukiExpression, rhs_expression: &TanukiExpression,
		main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize, local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>,
		start_line: NonZeroUsize, start_column: NonZeroUsize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match self {
			Self::WrappingAddition | Self::WrappingSubtraction | Self::WrappingMultiplication => {
				let (lhs_result_variable, lhs_type) = lhs_expression.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				let (rhs_result_variable, rhs_type) = rhs_expression.compile_r_value_to_c(main, modules, insert_into, function_temp_variable_count, local_variables)?;
				match (self, &lhs_type, &rhs_type) {
					// U + U
					(Self::WrappingAddition, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_add_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::Add(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					// U - U
					(Self::WrappingSubtraction, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_add_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::Subtract(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					// U * U
					(Self::WrappingMultiplication, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_mul_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::Multiply(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					// Bitwise
					(Self::NonShortCircuitAnd, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_and_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseAnd(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					(Self::NonShortCircuitOr, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_or_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseOr(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					(Self::NonShortCircuitXor, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_xor_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseXor(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					(Self::NonShortCircuitNand, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_nand_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseNot(CExpression::BitwiseAnd(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					(Self::NonShortCircuitNor, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_nor_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseNot(CExpression::BitwiseOr(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					(Self::NonShortCircuitXnor, TanukiType::U(lhs_bit_width), TanukiType::U(rhs_bit_width)) if lhs_bit_width == rhs_bit_width => {
						let name = format!("_tnk_temp_bit_xnor_var_{function_temp_variable_count}");
						*function_temp_variable_count += 1;
						let c_expression = CExpression::BitwiseNot(CExpression::BitwiseXor(CLValue::Variable(lhs_result_variable.unwrap()).into(), CLValue::Variable(rhs_result_variable.unwrap()).into()).into());
						insert_into.push_statement(CStatement::VariableDeclaration(
							lhs_type.compile_to_c(main, Some(start_line), Some(start_column))?, name.clone().into(), Some(CInitializer::Expression(c_expression).into())
						));
						Ok((Some(name.into()), lhs_type))
					}
					_ => return Err(Error::NotYetImplemented(format!("{self} operator between {lhs_type:?} and {rhs_type:?} types").into()).at(Some(start_line), Some(start_column), None)),
				}
			},
			_ => return Err(Error::NotYetImplemented(format!("{self} operator").into()).at(Some(start_line), Some(start_column), None)),
		}
	}
}

impl TanukiInfixTernaryOperator {
	/// Compiles a infix ternary operator that is being used as a r-value into C expressions/statements and inserts the compiled C statements into `insert_into`.
	/// Returns a (name, type) tuple where type is the results type and name is a `Some` variant if the result is non-void type and is the name of a C variable that contains the result.
	pub fn compile_r_value_to_c(
		&self, _lhs_expression: &TanukiExpression, _mhs_expression: &TanukiExpression, _rhs_expression: &TanukiExpression,
		_main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _insert_into: &mut CCompoundStatement, _function_temp_variable_count: &mut usize, _local_variables: &mut Vec<HashMap<Box<str>, TanukiType>>,
		start_line: NonZeroUsize, start_column: NonZeroUsize
	) -> Result<(Option<Box<str>>, TanukiType), ErrorAt> {
		match self {
			_ => return Err(Error::NotYetImplemented(format!("{self} operator").into()).at(Some(start_line), Some(start_column), None)),
		}
	}
}