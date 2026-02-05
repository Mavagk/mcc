use core::fmt;
use std::{fmt::{Display, Formatter}, num::NonZeroUsize};

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
	UnableToWriteToFile(String),
	InvalidUtf8,
	NoHomePath,
	InvalidFileExtension(String),
	MoreOpeningParenthesesThanClosingParentheses,
	MoreClosingParenthesesThanOpeningParentheses,
	IntegerOverflow,
	IntegerUnderflow,
	InvalidAsciiValue,
	InvalidOptimizationLevel,
	InvalidKeyword(String),
	InvalidBaseSpecifier(String),
	InvalidNumericLiteral(String),
	ExpectedClosingQuote,
	InvalidEscapeChars(String),
	InvalidUnicodeCodePoint,
}

impl Error {
	pub fn at(self, line: Option<NonZeroUsize>, column: Option<NonZeroUsize>, file: Option<String>) -> ErrorAt {
		ErrorAt { error: self, line, column, file }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotYetImplemented(feature) => write!(f, "{feature} not yet implemented"),
			Self::Unimplemented(feature) => write!(f, "{feature} unimplemented"),
			Self::InvalidSourcePath(path) => write!(f, "Invalid source path: {path}"),
			Self::InvalidCommandLineArgument(argument) => write!(f, "Invalid command line argument: {argument}"),
			Self::MultipleSourcePaths => write!(f, "Multiple source paths"),
			Self::MultipleOutputPaths => write!(f, "Multiple output paths"),
			Self::MultipleHomePaths => write!(f, "Multiple home paths"),
			Self::MultipleOutputFiles => write!(f, "Multiple output files"),
			Self::RepeatedArgument(argument) => write!(f, "Repeated argument {argument}"),
			Self::UnableToOpenFile(path, error) => write!(f, "Unable to open file at \"{path}\": {error}"),
			Self::UnableToReadFile(error) => write!(f, "Unable to read file: {error}"),
			Self::UnableToWriteToFile(error) => write!(f, "Unable to write to file: {error}"),
			Self::InvalidUtf8 => write!(f, "Invalid UTF-8"),
			Self::NoHomePath => write!(f, "No home directory specified and could not get the current working directory"),
			Self::InvalidFileExtension(file_path) => write!(f, "File {file_path} has an invalid file extension"),
			Self::MoreClosingParenthesesThanOpeningParentheses => write!(f, "More closing parentheses than opening parentheses"),
			Self::MoreOpeningParenthesesThanClosingParentheses => write!(f, "More opening parentheses than closing parentheses"),
			Self::IntegerOverflow => write!(f, "Integer overflow"),
			Self::IntegerUnderflow => write!(f, "Integer underflow"),
			Self::InvalidAsciiValue => write!(f, "Invalid ASCII value"),
			Self::InvalidOptimizationLevel => write!(f, "Invalid optimization level"),
			Self::InvalidKeyword(name) => write!(f, "Invalid keyword {name}"),
			Self::InvalidBaseSpecifier(specifier) => write!(f, "Invalid base specifier {specifier}"),
			Self::InvalidNumericLiteral(literal) => write!(f, "Invalid numeric literal {literal}"),
			Self::ExpectedClosingQuote => write!(f, "Expected closing quote"),
			Self::InvalidEscapeChars(chars) => write!(f, "Invalid escape chars \"{chars}\""),
			Self::InvalidUnicodeCodePoint => write!(f, "Invalid Unicode code point"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ErrorAt {
	error: Error,
	line: Option<NonZeroUsize>,
	column: Option<NonZeroUsize>,
	pub file: Option<String>,
}

impl Display for ErrorAt {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)?;
		match (self.line, self.column, self.file.clone()) {
			(Some(line), Some(column), Some(file)) => write!(f, " at \"{file}\":{line}:{column}")?,
			(Some(line), None,                         Some(file)) => write!(f, " in \"{file}\" on line {line}")?,
			(None,                       None,                         Some(file)) => write!(f, " in \"{file}\"")?,
			(Some(line), Some(column), None              ) => write!(f, " at {line}:{column}")?,
			(Some(line), None,                         None              ) => write!(f, " on line {line}")?,
			(None,                       None,                         None              ) => write!(f, "")?,
			(None,                       Some(column), Some(file)) => write!(f, " in \"{file}\" in column {column}")?,
			(None,                       Some(column), None              ) => write!(f, " in column {column}")?,
		}
		Ok(())
	}
}