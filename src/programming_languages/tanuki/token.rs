use std::fmt::{self, Formatter};

use crate::traits::{ast_node::AstNode, token::Token};

#[derive(Debug)]
pub struct TanukiToken {

}

impl Token for TanukiToken {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

impl AstNode for TanukiToken {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}