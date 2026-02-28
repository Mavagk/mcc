use std::{any::Any, collections::HashSet, fmt::Debug, path::Path};

use crate::{Main, error::ErrorAt, programming_languages::c::module::CModule, traits::ast_node::AstNode};

/// A module that has been parsed from one or more files.
pub trait Module: Debug + AstNode + Any {
	/// Execute the module in interpreted mode from the module entrypoint.
	fn interpreted_execute_entrypoint(&self, main: &mut Main) -> Result<(), ErrorAt>;
	/// Source to source compiles the module to a C module. Returns `Ok(None)` if this is not supported.
	fn to_c_module(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt>;

	fn const_compile(
		&mut self, _main: &mut Main, _global_items_const_compiled: &mut HashSet<(Box<str>, Box<Path>)>, _global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>,
		_modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)], _module_path: &Path,
	) -> Result<bool, ErrorAt> {
		Ok(true)
	}

	fn get_global_items(&self, _global_items_to_const_compile_for_this_module: &mut HashSet<Box<str>>) -> Result<(), ErrorAt> {
		Ok(())
	}
}