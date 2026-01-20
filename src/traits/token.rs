use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

pub trait Token: Debug {
	fn start_line(&self) -> NonZeroUsize;
	fn end_line(&self) -> NonZeroUsize;
	fn start_column(&self) -> NonZeroUsize;
	fn end_column(&self) -> NonZeroUsize;
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result;

	fn print(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:03}:{:03}: ", self.start_line(), self.start_column())?;
		self.print_name(f)?;
		writeln!(f)
	}
}