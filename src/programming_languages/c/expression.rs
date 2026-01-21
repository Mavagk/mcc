use std::fmt::{self, Formatter};

use crate::traits::{ast_node::AstNode, expression::Expression};

#[derive(Debug)]
pub enum CExpression {

}

impl Expression for CExpression {
	
}

impl AstNode for CExpression {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}