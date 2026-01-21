use std::fmt::{self, Formatter};

use crate::traits::{ast_node::AstNode, types::Type};

#[derive(Debug)]
pub enum CType {
	Void
}

impl Type for CType {

}

impl AstNode for CType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => write!(f, "Void")
		}
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => Ok(())
		}
	}
}