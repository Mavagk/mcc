use std::fmt::Display;

#[derive(Clone, Debug)]
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
	UnableToOpenFile(String, String),
	UnableToReadFile(String),
	InvalidUtf8,
	NoHomePath,
	InvalidFileExtension(String),
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
			Self::UnableToOpenFile(path, error) => writeln!(f, "Unable to open file at \"{path}\": {error}"),
			Self::UnableToReadFile(error) => writeln!(f, "Unable to read file: {error}"),
			Self::InvalidUtf8 => writeln!(f, "Invalid UTF-8"),
			Self::NoHomePath => writeln!(f, "No home directory specified and could not get the current working directory"),
			Self::InvalidFileExtension(file_path) => writeln!(f, "File {file_path} has an invalid file extension."),
		}
	}
}