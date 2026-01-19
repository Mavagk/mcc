use core::fmt;
use std::{fmt::Formatter, num::NonZeroUsize};

pub trait AstNode {
	fn get_line(&self) -> NonZeroUsize;
	fn get_column(&self) -> NonZeroUsize;
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result;
	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result;

	fn print(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for _ in 0..level {
			write!(f, "-")?;
		}
		write!(f, "{:03}:{:03}: ", self.get_line(), self.get_column())?;
		self.print_name(f)?;
		writeln!(f)?;
		self.print_sub_nodes(level + 1, f)
	}
}