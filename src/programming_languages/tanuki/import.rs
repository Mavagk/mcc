use std::{fmt::{self, Formatter}, num::NonZeroUsize, path::Path};

use crate::traits::ast_node::AstNode;

pub struct TanukiImport {
	pub name: Box<str>,
	pub module_path: Box<Path>,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

impl AstNode for TanukiImport {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Import {:?} {}", self.module_path, self.name)
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
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