use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::traits::{ast_node::AstNode, statement::Statement};

#[derive(Debug)]
pub enum CStatement {
	CompoundStatement(CCompoundStatement),
}

impl Statement for CStatement {
	
}

impl AstNode for CStatement {
	fn get_start_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_end_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_start_column(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_end_column(&self) -> NonZeroUsize {
		todo!()
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