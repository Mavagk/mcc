use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize, path::Path};

use crate::{Main, error::ErrorAt, programming_languages::{c::module::CModule, tanuki::{expression::TanukiExpression, function::TanukiFunction, global_constant::TanukiGlobalConstant}}, traits::{ast_node::AstNode, module::Module}};

/// A Tanuki module that has been parsed from a single file.
pub struct TanukiModule {
	pub parsed_expressions: Box<[TanukiExpression]>,
	pub functions: Vec<Option<TanukiFunction>>,
	pub global_constants: Vec<Option<TanukiGlobalConstant>>,
	pub entrypoint: Option<Box<str>>,
}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		todo!()
	}

	fn to_c_module(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		Ok(Some(self.compile_to_c_module(main, modules)?))
	}

	fn const_compile(&mut self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], module_path: &Path, was_complication_done: &mut bool) -> Result<(), ErrorAt> {
		self.const_compile_globals(main, modules, module_path, was_complication_done)
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")?;
		if let Some(entrypoint) = &self.entrypoint {
			write!(f, ", Entrypoint: {entrypoint}")?;
		}
		Ok(())
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for expression in &self.parsed_expressions {
			expression.print(level, f)?;
		}
		for function in self.functions.iter() {
			function.as_ref().unwrap().print(level, f)?;
		}
		for global_constant in self.global_constants.iter() {
			global_constant.as_ref().unwrap().print(level, f)?;
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