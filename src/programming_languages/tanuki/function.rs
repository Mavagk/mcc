use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::tanuki::{constant_value::TanukiType, expression::TanukiExpression}, traits::ast_node::AstNode};

pub struct TanukiFunction {
	pub name: Box<str>,
	pub arguments: Box<[TanukiFunctionArgument]>,
	pub return_type: TanukiType,
	pub body: TanukiExpression,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

impl AstNode for TanukiFunction {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Function {}", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for argument in self.arguments.iter() {
			argument.print(level, f)?;
		}
		self.return_type.print(level, f)?;
		self.body.print(level, f)
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

pub struct TanukiFunctionArgument {
	t_type: TanukiType,
	name: Box<str>,
}

impl AstNode for TanukiFunctionArgument {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Argument {}", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		self.t_type.print(level, f)
	}
}