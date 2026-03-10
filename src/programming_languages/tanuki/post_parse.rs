use std::{collections::HashSet, mem::take};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, global_constant::TanukiGlobalConstant, module::TanukiModule}};

pub struct TanukiModulePostParseData<'a> {
	pub functions: &'a mut Vec<Option<TanukiFunction>>,
	pub global_constants: &'a mut Vec<Option<TanukiGlobalConstant>>,
	pub entrypoint: &'a mut Option<Box<str>>,
}

impl TanukiModule {
	pub fn post_parse(&mut self, main: &mut Main) -> Result<(), ErrorAt> {
		let mut post_parse_data = TanukiModulePostParseData {
			functions: &mut self.functions, global_constants: &mut self.global_constants, entrypoint: &mut self.entrypoint
		};
		for expression in self.parsed_expressions.iter_mut() {
			expression.post_parse(main, &mut post_parse_data, false, false, &mut HashSet::new(), &mut Vec::new())?;
		}
		self.parsed_expressions = Default::default();
		Ok(())
	}
}

impl TanukiExpression {
	pub fn post_parse(
		&mut self, main: &mut Main, post_parse_data: &mut TanukiModulePostParseData, is_inside_function_or_block: bool, /*assigned_to_name: Option<&str>, */is_l_value: bool,
		global_variables_dependent_on: &mut HashSet<Box<str>>, local_variables: &mut Vec<HashSet<Box<str>>>
	) -> Result<(), ErrorAt> {
		match &mut self.variant {
			// Assignment
			TanukiExpressionVariant::Assignment(lhs, rhs) => {
				let start_line = lhs.start_line;
				let start_column = lhs.start_column;
				let end_line = rhs.end_line;
				let end_column = rhs.end_column;
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, /*None, */true, &mut HashSet::new(), &mut Vec::new())?;
				let lhs = take(lhs);
				let (name, t_type, is_exported) = match lhs.clone().variant {
					TanukiExpressionVariant::Variable(name) => (name, None, false),
					TanukiExpressionVariant::Export(sub_expression) => match sub_expression.variant {
						TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => match value_expression.variant {
							TanukiExpressionVariant::Variable(name) => (name, Some(type_expression), true),
							_ => return Err(Error::ExpectedVariable.at(Some(value_expression.start_line), Some(value_expression.end_column), None)),
						},
						TanukiExpressionVariant::Variable(name) => (name, None, true),
						_ => return Err(Error::ExpectedVariable.at(Some(sub_expression.start_line), Some(sub_expression.end_column), None)),
					},
					TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => match value_expression.variant {
						TanukiExpressionVariant::Variable(name) => (name, Some(type_expression), false),
						_ => return Err(Error::ExpectedVariable.at(Some(value_expression.start_line), Some(value_expression.end_column), None)),
					},
					_ => return Err(Error::ExpectedVariable.at(Some(lhs.start_line), Some(lhs.end_column), None)),
				};
				*global_variables_dependent_on = HashSet::new();
				global_variables_dependent_on.insert(name.clone());
				let mut global_variables_dependent_on = HashSet::new();
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, false, &mut global_variables_dependent_on, local_variables)?;
				let rhs = take(rhs);
				let global_constant = TanukiGlobalConstant {
					value_expression: *rhs, name, t_type: t_type.map(|t_type| *t_type), start_line, start_column, end_line, end_column, export: is_exported
				};
				post_parse_data.global_constants.push(Some(global_constant));
				*self = *lhs.clone();
			}
			TanukiExpressionVariant::Entrypoint(to_be_entrypoint) => {
				to_be_entrypoint.post_parse(main, post_parse_data, is_inside_function_or_block, /*None, */is_l_value, global_variables_dependent_on, local_variables)?;
				match &mut to_be_entrypoint.variant {
					TanukiExpressionVariant::Variable(name) => {
						if let Some(current_entrypoint) = post_parse_data.entrypoint && current_entrypoint != name {
							return Err(Error::MultipleEntrypoints.at(Some(self.start_line), Some(self.start_column), None));
						}
						*post_parse_data.entrypoint = Some(name.clone());
						*self = take(to_be_entrypoint);
					},
					_ => return Err(Error::ExpectedVariable.at(Some(self.start_line), Some(self.start_column), None)),
				}
			},
			_ => {}
		}
		Ok(())
	}
}