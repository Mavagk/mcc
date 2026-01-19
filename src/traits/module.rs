use std::fmt::Debug;

use crate::{Main, error::ErrorAt, traits::ast_node::AstNode};

/// A module that has been parsed from one or more files.
pub trait Module: Debug + AstNode {
	/// Execute the module in interpreted mode from the module entrypoint.
	fn interpreted_execute_entrypoint(&self, main: &mut Main) -> Result<(), ErrorAt>;
}