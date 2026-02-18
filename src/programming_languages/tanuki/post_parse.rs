use std::mem::take;

use crate::{Main, error::ErrorAt, programming_languages::tanuki::{expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, global_constant::TanukiGlobalConstant, module::TanukiModule}};

pub struct TanukiModulePostParseData<'a> {
	pub functions: &'a mut Vec<TanukiFunction>,
	pub global_constants: &'a mut Vec<TanukiGlobalConstant>,
}

impl TanukiModule {
	pub fn post_parse(&mut self, main: &mut Main) -> Result<(), ErrorAt> {
		let mut post_parse_data = TanukiModulePostParseData {
			functions: &mut self.functions,
			global_constants: &mut self.global_constants,
		};
		for expression in self.parsed_expressions.iter_mut() {
			expression.post_parse(main, &mut post_parse_data, false)?;
		}
		self.parsed_expressions = Default::default();
		Ok(())
	}
}

impl TanukiExpression {
	pub fn post_parse(&mut self, main: &mut Main, post_parse_data: &mut TanukiModulePostParseData, is_inside_function: bool) -> Result<(), ErrorAt> {
		match (&mut self.variant, is_inside_function) {
			(TanukiExpressionVariant::Assignment(lhs, rhs), false) => {
				let start_line = lhs.start_line;
				let start_column = lhs.start_column;
				let end_line = rhs.end_line;
				let end_column = rhs.end_column;
				lhs.post_parse(main, post_parse_data, is_inside_function)?;
				rhs.post_parse(main, post_parse_data, is_inside_function)?;
				let lhs = take(lhs);
				let (name, t_type) = match lhs.clone().variant {
					TanukiExpressionVariant::Variable(name) => {
						(name, None)
					},
					TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => match value_expression.variant {
						TanukiExpressionVariant::Variable(name) => (name, Some(type_expression)),
						_ => todo!(),
					},
					_ => todo!()
				};
				let rhs = take(rhs);
				let global_constant = TanukiGlobalConstant {
					value: *rhs, name, t_type: t_type.map(|t_type| *t_type), start_line, start_column, end_line, end_column,
				};
				*self = *lhs.clone();
				post_parse_data.global_constants.push(global_constant);
			}
			_ => {}
		}
		Ok(())
	}
}