use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

use crate::{Main, error::ErrorAt, programming_languages::{c::module::CModule, tanuki::{constant_value::TanukiConstantValue, expression::TanukiExpression, function::TanukiFunction}}, traits::{ast_node::AstNode, module::Module}};

pub struct TanukiModule {
	pub parsed_expressions: Box<[TanukiExpression]>,
	pub functions: Vec<TanukiFunction>,
	pub global_constants: Vec<TanukiConstantValue>,
}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		todo!()
	}

	fn to_c_module(&self, _main: &mut Main, _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		todo!()
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for expression in &self.parsed_expressions {
			expression.print(level, f)?;
		}
		for function in self.functions.iter() {
			function.print(level, f)?;
		}
		for global_constant in self.global_constants.iter() {
			global_constant.print(level, f)?;
		}
		Ok(())
	}

	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.first().map(|expression| expression.start_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.first().map(|expression| expression.start_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.last().map(|expression| expression.end_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.last().map(|expression| expression.end_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}
}

impl Debug for TanukiModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}