use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

pub trait Token: Debug {
	fn get_start_line(&self) -> NonZeroUsize;
	fn get_end_line(&self) -> NonZeroUsize;
	fn get_start_column(&self) -> NonZeroUsize;
	fn get_end_column(&self) -> NonZeroUsize;
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result;

	fn print(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:03}:{:03}: ", self.get_start_line(), self.get_start_column())?;
		self.print_name(f)?;
		writeln!(f)
	}
}