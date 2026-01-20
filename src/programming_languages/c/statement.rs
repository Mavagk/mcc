use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::traits::{ast_node::AstNode, statement::Statement};

#[derive(Debug)]
pub enum CStatement {
	CompoundStatement(CCompoundStatement),
}

impl Statement for CStatement {
	
}

impl AstNode for CStatement {
	fn start_line(&self) -> Option<NonZeroUsize> {
		None
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		None
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		None
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		None
	}

	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

#[derive(Debug)]
pub struct CCompoundStatement {
	sub_statements: Vec<CStatement>,
}

impl CCompoundStatement {
	pub fn new() -> Self {
		Self {
			sub_statements: Vec::new(),
		}
	}

	pub fn push_statement(&mut self, statement: CStatement) {
		self.sub_statements.push(statement);
	}
}