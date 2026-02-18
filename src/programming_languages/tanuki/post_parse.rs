use crate::{Main, error::ErrorAt, programming_languages::tanuki::{constant_value::TanukiConstantValue, expression::TanukiExpression, function::TanukiFunction, module::TanukiModule}};

pub struct TanukiModulePostParseData<'a> {
	pub functions: &'a mut Vec<TanukiFunction>,
	pub global_constants: &'a mut Vec<TanukiConstantValue>,
}

impl TanukiModule {
	pub fn post_parse(&mut self, main: &mut Main) -> Result<(), ErrorAt> {
		let post_parse_data = TanukiModulePostParseData {
			functions: &mut self.functions,
			global_constants: &mut self.global_constants,
		};
		for expression in self.parsed_expressions.iter_mut() {
			expression.post_parse(main, &post_parse_data)?;
		}
		Ok(())
	}
}

impl TanukiExpression {
	pub fn post_parse(&mut self, main: &mut Main, post_parse_data: &TanukiModulePostParseData) -> Result<(), ErrorAt> {
		Ok(())
	}
}