use std::{collections::HashMap, fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::tanuki::{expression::TanukiExpression, t_type::TanukiType}, traits::ast_node::AstNode};

/// A Tanuki function.
pub struct TanukiFunction {
	pub name: Box<str>,
	pub parameters: Box<[Option<TanukiFunctionParameter>]>,
	pub return_type: Option<TanukiExpression>,
	pub body: Option<TanukiExpression>,
	pub bodies_for_concrete_types: Option<HashMap<(Box<[TanukiType]>, Box<TanukiType>),TanukiExpression>>,
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
			parameter.as_ref().unwrap().print(level, f)?;
		}
		if let Some(return_type) = &self.return_type {
			return_type.print(level, f)?;
		}
		if let Some(body) = &self.body {
			body.print(level, f)?;
		}
		Ok(())
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