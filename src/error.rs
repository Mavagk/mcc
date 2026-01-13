use std::fmt::Display;

pub enum Error {
	InvalidSourcePath(String),
	InvalidCommandLineArgument(String),
	NotYetImplemented(String),
	Unimplemented(String),
	MultipleSourcePaths,
	MultipleOutputPaths,
	MultipleHomePaths,
	MultipleOutputFiles,
	RepeatedArgument(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotYetImplemented(feature) => writeln!(f, "{feature} not yet implemented"),
			Self::Unimplemented(feature) => writeln!(f, "{feature} unimplemented"),
			Self::InvalidSourcePath(path) => writeln!(f, "Invalid source path: {path}"),
			Self::InvalidCommandLineArgument(argument) => writeln!(f, "Invalid command line argument: {argument}"),
			Self::MultipleSourcePaths => writeln!(f, "Multiple source paths"),
			Self::MultipleOutputPaths => writeln!(f, "Multiple output paths"),
			Self::MultipleHomePaths => writeln!(f, "Multiple home paths"),
			Self::MultipleOutputFiles => writeln!(f, "Multiple output files"),
			Self::RepeatedArgument(argument) => writeln!(f, "Repeated argument {argument}"),
		}
	}
}