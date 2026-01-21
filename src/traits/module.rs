use std::fmt::Debug;

use crate::{Main, error::ErrorAt, programming_languages::c::module::CModule, traits::ast_node::AstNode};

/// A module that has been parsed from one or more files.
pub trait Module: Debug + AstNode {
	/// Execute the module in interpreted mode from the module entrypoint.
	fn interpreted_execute_entrypoint(&self, main: &mut Main) -> Result<(), ErrorAt>;
	/// Source to source compiles the module to a C module. Returns `Ok(None)` if this is not supported.
	fn to_c_module(&self, main: &mut Main, is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt>;
}