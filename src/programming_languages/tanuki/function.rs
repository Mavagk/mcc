use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::tanuki::expression::TanukiExpression, traits::ast_node::AstNode};

pub struct TanukiFunction {
	pub name: Box<str>,
	pub parameters: Box<[TanukiFunctionParameter]>,
	pub return_type: Option<TanukiExpression>,
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
		for parameter in self.parameters.iter() {
			parameter.print(level, f)?;
		}
		if let Some(return_type) = &self.return_type {
			return_type.print(level, f)?;
		}
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

pub struct TanukiFunctionParameter {
	pub t_type: Option<TanukiExpression>,
	pub name: Box<str>,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

impl AstNode for TanukiFunctionParameter {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Parameter {}", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		if let Some(t_type) = &self.t_type {
			t_type.print(level, f)
		}
		else {
			Ok(())
		}
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