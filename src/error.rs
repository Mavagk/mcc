use std::fmt::Display;

pub enum Error {
	InvalidSourcePath,
	NotYetImplemented(String),
	Unimplemented(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotYetImplemented(feature) => writeln!(f, "{feature} not yet implemented"),
			Self::Unimplemented(feature) => writeln!(f, "{feature} unimplemented"),
			Self::InvalidSourcePath => writeln!(f, "Invalid source path"),
		}
	}
}