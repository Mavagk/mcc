use std::{collections::HashSet, fmt::{self, Debug, Formatter}, num::NonZeroUsize, path::Path};

use crate::{Main, error::ErrorAt, programming_languages::{c::module::CModule, tanuki::{export::TanukiExport, expression::TanukiExpression, function::TanukiFunction, global_constant::TanukiGlobalConstant, import::TanukiImport, link::TanukiLink}}, traits::{ast_node::AstNode, module::Module}};

pub struct TanukiModule {
	pub parsed_expressions: Box<[TanukiExpression]>,
	pub functions: Vec<Option<TanukiFunction>>,
	pub global_constants: Vec<Option<TanukiGlobalConstant>>,
	pub exports: Vec<TanukiExport>,
	pub imports: Vec<TanukiImport>,
	pub links: Vec<TanukiLink>,
}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		todo!()
	}

	fn to_c_module(&self, main: &mut Main, is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		self.compile_to_c_module(main, is_entrypoint)
	}

	fn get_global_items(&self, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>) -> Result<(), ErrorAt> {
		self.get_global_items_for_module(global_items_to_const_compile_for_this_module)
	}

	fn const_compile(
		&mut self, main: &mut Main, global_items_const_compiled: &mut HashSet<(Box<str>, Box<Path>)>, global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], module_path: &Path,
	) -> Result<bool, ErrorAt> {
		self.const_compile_globals(main, global_items_const_compiled, global_items_to_const_compile_for_this_module, modules, module_path)
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for export in self.exports.iter() {
			export.print(level, f)?;
		}
		for import in self.imports.iter() {
			import.print(level, f)?;
		}
		for link in self.links.iter() {
			link.print(level, f)?;
		}
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