use crate::{Main, error::ErrorAt, programming_languages::{c::{expression::CExpression, l_value::CLValue, module::CModule, module_element::{CFunctionParameter, CModuleElement}, statement::{CCompoundStatement, CInitializer, CStatement}, types::CType}, tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, module::TanukiModule, t_type::TanukiType}}};

impl TanukiModule {
	pub fn compile_to_c_module(&self, main: &mut Main) -> Result<CModule, ErrorAt> {
		// Create module
		let mut c_module = CModule::new();
		// Add imports needed for the language
		c_module.push_element(CModuleElement::AngleInclude("stdint.h".into()));
		// Compile functions
		for function in self.functions.iter() {
			c_module.push_element(function.as_ref().unwrap().compile_to_c(main)?);
		}
		// Return
		Ok(c_module)
	}
}

impl TanukiFunction {
	pub fn compile_to_c(&self, main: &mut Main) -> Result<CModuleElement, ErrorAt> {
		// Get return type
		let return_type = match &self.return_type {
			Some(return_type) => match return_type {
				TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(return_type)), .. } => return_type,
				_ => unreachable!(),
			}
			None => &TanukiType::Void,
		};
		// Compile parameters
		let mut c_parameters = Vec::new();
		for parameter in self.parameters.iter() {
			let c_type = match &parameter.t_type {
				Some(TanukiExpression { variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(t_type)), .. }) => t_type,
				Some(_) => unreachable!(),
				None => todo!(),
			}.compile_to_c(main)?;
			c_parameters.push(CFunctionParameter::new(c_type, parameter.name.clone()));
		}
		// Compile body
		let mut function_temp_variable_count = 0;
		let mut c_compound_statement = CCompoundStatement::new();
		let body_result_value_name = self.body.compile_to_c(main, &mut c_compound_statement, &mut function_temp_variable_count)?;
		// Return body value if it is not a void value
		if let Some(body_result_value_name) = body_result_value_name {
			c_compound_statement.push_statement(CStatement::Return(Some(CLValue::Variable(body_result_value_name).into())));
		}
		// Pack into struct
		Ok(CModuleElement::FunctionDefinition {
			return_type: return_type.compile_to_c(main)?, name: self.name.clone(), parameters: c_parameters.into(), body: Box::new(c_compound_statement)
		})
	}
}

impl TanukiType {
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
	pub fn compile_to_c(&self, main: &mut Main, insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize) -> Result<Option<Box<str>>, ErrorAt> {
		match &self.variant {
			TanukiExpressionVariant::Constant(constant) => constant.compile_to_c(main, insert_into, function_temp_variable_count),
			_ => todo!(),
		}
	}
}

impl TanukiCompileTimeValue {
	pub fn compile_to_c(&self, main: &mut Main, insert_into: &mut CCompoundStatement, function_temp_variable_count: &mut usize) -> Result<Option<Box<str>>, ErrorAt> {
		let c_type = self.get_type().compile_to_c(main)?;
		let c_expression = match self {
			TanukiCompileTimeValue::U(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::I(_, value) => CExpression::IntConstant(*value as i128),
			TanukiCompileTimeValue::Void => return Ok(None),
			_ => todo!(),
		};
		let name = format!("_tnk_temp_var_{function_temp_variable_count}");
		*function_temp_variable_count += 1;
		insert_into.push_statement(CStatement::VariableDeclaration(c_type, name.clone().into(), Some(CInitializer::Expression(c_expression).into())));
		Ok(Some(name.into()))
	}
}