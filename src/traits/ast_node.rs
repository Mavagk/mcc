use std::{fmt::{self, Formatter}, num::NonZeroUsize};

pub trait AstNode {
	fn start_line(&self) -> Option<NonZeroUsize>;
	fn end_line(&self) -> Option<NonZeroUsize>;
	fn start_column(&self) -> Option<NonZeroUsize>;
	fn end_column(&self) -> Option<NonZeroUsize>;
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result;
	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result;

	fn print(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for _ in 0..level {
			write!(f, "-")?;
		}
		let start_line = self.start_line();
		let end_line = self.end_line();
		let start_column = self.start_column();
		let end_column = self.end_column();
		match (start_line, end_line, start_column, end_column) {
			(None, None, None, None) => {},
			(Some(start_line), None, None, None) => write!(f, "{start_line:03} ")?,
			(Some(start_line), None, Some(start_column), None) => write!(f, "{start_line:03}:{start_column:03} ")?,
			(Some(start_line), Some(end_line), None, None) if start_line == end_line => write!(f, "{start_line:03} ")?,
			(Some(start_line), Some(end_line), None, None) => write!(f, "{start_line:03}-{end_line:03} ")?,
			(Some(start_line), Some(end_line), Some(start_column), Some(end_column)) if start_line == end_line && start_column == end_column
				=> write!(f, "{start_line:03}:{start_column:03} ")?,
			(Some(start_line), Some(end_line), Some(start_column), Some(end_column)) if start_line == end_line
				=> write!(f, "{start_line:03}:{start_column:03}-{end_column:03} ")?,
			(Some(start_line), Some(end_line), Some(start_column), Some(end_column)) => write!(f, "{start_line:03}:{start_column:03}-{end_line:03}:{end_column:03} ")?,
			(Some(start_line), None, Some(start_column), Some(end_column)) => write!(f, "{start_line:03}:{start_column:03}-{end_column:03} ")?,
			_ => unimplemented!(),
		}
		self.print_name(f)?;
		writeln!(f)?;
		self.print_sub_nodes(level + 1, f)
	}
}