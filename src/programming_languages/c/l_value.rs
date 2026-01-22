use std::fmt::{self, Formatter};

use crate::traits::ast_node::AstNode;

#[derive(Debug)]
pub enum LValue {
	Variable(Box<str>),
}

impl AstNode for LValue {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(name) => write!(f, "Variable \"{name}\""),
		}
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(_) => Ok(())
		}
	}
}