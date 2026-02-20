use std::{collections::HashSet, fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::tanuki::expression::TanukiExpression, traits::ast_node::AstNode};

pub struct TanukiGlobalConstant {
	pub value_expression: TanukiExpression,
	pub name: Box<str>,
	pub t_type: Option<TanukiExpression>,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
	pub depends_on: HashSet<Box<str>>,
}


impl AstNode for TanukiGlobalConstant {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Global Constant {} depending on {:?}", self.name, self.depends_on)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		self.value_expression.print(level, f)
	}

	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.start_line)
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.start_column)
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.end_line)
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.end_column)
	}
}
