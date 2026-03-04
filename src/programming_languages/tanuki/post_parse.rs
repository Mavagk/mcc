use std::{collections::HashSet, mem::take};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, export::TanukiExport, expression::{TanukiExpression, TanukiExpressionVariant}, function::TanukiFunction, global_constant::TanukiGlobalConstant, import::TanukiImport, link::TanukiLink, module::TanukiModule}};

pub struct TanukiModulePostParseData<'a> {
	pub functions: &'a mut Vec<Option<TanukiFunction>>,
	pub global_constants: &'a mut Vec<Option<TanukiGlobalConstant>>,
	pub exports: &'a mut Vec<TanukiExport>,
	pub imports: &'a mut Vec<TanukiImport>,
	pub links: &'a mut Vec<TanukiLink>,
	pub entrypoint: &'a mut Option<Box<str>>,
}

impl TanukiModule {
	pub fn post_parse(&mut self, main: &mut Main) -> Result<(), ErrorAt> {
		let mut post_parse_data = TanukiModulePostParseData {
			functions: &mut self.functions,
			global_constants: &mut self.global_constants,
			exports: &mut self.exports, imports: &mut self.imports, links: &mut self.links, entrypoint: &mut self.entrypoint
		};
		for expression in self.parsed_expressions.iter_mut() {
			expression.post_parse(main, &mut post_parse_data, false, None, false, &mut HashSet::new(), &mut Vec::new())?;
		}
		self.parsed_expressions = Default::default();
		Ok(())
	}
}

impl TanukiExpression {
	pub fn post_parse(
		&mut self, main: &mut Main, post_parse_data: &mut TanukiModulePostParseData, is_inside_function_or_block: bool, assigned_to_name: Option<&str>, is_l_value: bool,
		global_variables_dependent_on: &mut HashSet<Box<str>>, local_variables: &mut Vec<HashSet<Box<str>>>
	) -> Result<(), ErrorAt> {
		match &mut self.variant {
			// Assignment
			TanukiExpressionVariant::Assignment(lhs, rhs) => {
				let start_line = lhs.start_line;
				let start_column = lhs.start_column;
				let end_line = rhs.end_line;
				let end_column = rhs.end_column;
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, &mut HashSet::new(), &mut Vec::new())?;
				let lhs = take(lhs);
				let (name, t_type) = match lhs.clone().variant {
					TanukiExpressionVariant::Variable(name) => {
						(name, None)
					},
					TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => match value_expression.variant {
						TanukiExpressionVariant::Variable(name) => (name, Some(type_expression)),
						_ => return Err(Error::ExpectedVariable.at(Some(value_expression.start_line), Some(value_expression.end_column), None)),
					},
					_ => return Err(Error::ExpectedVariable.at(Some(lhs.start_line), Some(lhs.end_column), None)),
				};
				*global_variables_dependent_on = HashSet::new();
				global_variables_dependent_on.insert(name.clone());
				let mut global_variables_dependent_on = HashSet::new();
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, Some(&name), false, &mut global_variables_dependent_on, local_variables)?;
				let rhs = take(rhs);
				if !matches!(rhs.variant, TanukiExpressionVariant::Import(..) | TanukiExpressionVariant::Link(..)) {
					let global_constant = TanukiGlobalConstant {
						value_expression: *rhs, name, t_type: t_type.map(|t_type| *t_type), start_line, start_column, end_line, end_column,
					};
					post_parse_data.global_constants.push(Some(global_constant));
				}
				*self = *lhs.clone();
			}
			TanukiExpressionVariant::Export(to_export) => {
				to_export.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
				match &mut to_export.variant {
					TanukiExpressionVariant::Variable(name) => {
						post_parse_data.exports.push(TanukiExport { name: name.clone(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column });
						*self = take(to_export);
					},
					_ => return Err(Error::ExpectedVariable.at(Some(self.start_line), Some(self.start_column), None)),
				}
			},
			TanukiExpressionVariant::Entrypoint(to_be_entrypoint) => {
				to_be_entrypoint.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
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
			TanukiExpressionVariant::Import(arguments) => {
				if arguments.len() != 1 {
					return Err(Error::Unimplemented("@import with argument count that is not one".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				let assigned_to_name = assigned_to_name.unwrap();
				let argument = &arguments[0];
				let argument = match &argument.variant {
					TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
					_ => return Err(Error::Unimplemented("@import with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
				};
				let mut module = main.module_being_processed.parent().unwrap().to_path_buf();
				module.push(argument);
				main.add_module_to_compile((module.clone().into_boxed_path(), false));
				post_parse_data.imports.push(TanukiImport {
					name: assigned_to_name.into(), module_from: module.into_boxed_path(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				});
			},
			TanukiExpressionVariant::Link(arguments) => {
				if arguments.len() != 1 {
					return Err(Error::Unimplemented("@link with argument count that is not one".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				let assigned_to_name = assigned_to_name.unwrap();
				let argument = &arguments[0];
				let argument = match &argument.variant {
					TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
					_ => return Err(Error::Unimplemented("@link with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
				};
				let mut dynamic_library_path = main.module_being_processed.parent().unwrap().to_path_buf();
				dynamic_library_path.push(argument);
				post_parse_data.links.push(TanukiLink {
					name: assigned_to_name.into(), dynamic_library_path: dynamic_library_path.into_boxed_path(),
					start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				});
			},
			_ => {}
		}
		Ok(())
	}
}