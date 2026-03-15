use core::fmt;
use std::{fmt::{Display, Formatter}, num::NonZeroUsize};

use num::BigInt;

use crate::{programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, t_type::TanukiType}, traits::ast_node::AstNode};

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
	ExpectedOpeningParenthesis,
	ExpectedClosingParenthesis,
	ExpectedCurlyOpeningParenthesis,
	ExpectedCurlyClosingParenthesis,
	ExpectedSquareOpeningParenthesis,
	ExpectedSquareClosingParenthesis,
	ExpectedSemicolon,
	ExpectedComma,
	ExpectedExpression,
	ExpectedVariable,
	InvalidEscapeChars(String),
	InvalidUnicodeCodePoint,
	InvalidOperatorSymbol(String),
	InvalidCharStartingToken(char),
	InvalidPostfixUnaryOperator(String),
	InvalidPrefixUnaryOperator(String),
	InvalidInfixBinaryOperator(String),
	ColonAtExpressionEnd,
	ColonWithoutMatchingTernaryOperator,
	InvalidAssignmentOperator(String),
	UnexpectedReturnType,
	AugmentedAssignmentUsedOnGlobalVariable,
	VariableStartsWithTnk,
	CannotBeInsideBlockOrFunction,
	ExpressionCannotBeLValue,
	ExpressionCannotBeRValue,
	DuplicateGlobalVariableWithDifferentValues,
	UnableToConstCompile,
	VariableNotFound,
	UnexpectedBuiltinFunctionArgumentCount { expected_min: Option<usize>, expected_max: Option<usize>, got: usize },
	UnexpectedValueType { value: TanukiCompileTimeValue, expected_type: Option<TanukiType> },
	InvalidIntegerBitWidth(BigInt),
	InvalidFloatBitWidth(BigInt),
	IntegerTooLargeForType(BigInt, String),
	IntegerTooSmallForType(BigInt, String),
	CannotUseUnaryOperatorForType{ type_t: String, operator: String },
	CannotUseBinaryOperatorForType{ lhs_type_t: String, rhs_type_t: String, operator: String },
	DivisionByZero,
	ModuloByZero,
	MultipleEntrypoints,
	EntrypointOnNonFunction,
	NegativeFactorial,
	ExpectedType,
	TypeMismatch((String, String)),
	ArgumentCountMismatch((usize, usize)),
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
			Self::ExpectedOpeningParenthesis => write!(f, "Expected opening parenthesis"),
			Self::ExpectedClosingParenthesis => write!(f, "Expected closing parenthesis"),
			Self::ExpectedCurlyOpeningParenthesis => write!(f, "Expected curly opening parenthesis"),
			Self::ExpectedCurlyClosingParenthesis => write!(f, "Expected curly closing parenthesis"),
			Self::ExpectedSquareOpeningParenthesis => write!(f, "Expected square opening parenthesis"),
			Self::ExpectedSquareClosingParenthesis => write!(f, "Expected square closing parenthesis"),
			Self::ExpectedSemicolon => write!(f, "Expected semicolon"),
			Self::ExpectedComma => write!(f, "Expected comma"),
			Self::ExpectedExpression => write!(f, "Expected an expression"),
			Self::ExpectedVariable => write!(f, "Expected a variable"),
			Self::InvalidOperatorSymbol(name) => write!(f, "Invalid operator symbol \"{name}\""),
			Self::InvalidCharStartingToken(chr) => write!(f, "Invalid char '{chr}' starting token"),
			Self::InvalidPostfixUnaryOperator(symbol) => write!(f, "Invalid postfix unary operator {symbol}"),
			Self::InvalidPrefixUnaryOperator(symbol) => write!(f, "Invalid prefix unary operator {symbol}"),
			Self::InvalidInfixBinaryOperator(symbol) => write!(f, "Invalid infix binary operator {symbol}"),
			Self::InvalidAssignmentOperator(symbol) => write!(f, "Invalid assignment operator {symbol}"),
			Self::ColonAtExpressionEnd => write!(f, "Colon at expression end, expected expression after colon"),
			Self::ColonWithoutMatchingTernaryOperator => write!(f, "Colon without matching ternary operator"),
			Self::UnexpectedReturnType => write!(f, "Unexpected return type"),
			Self::AugmentedAssignmentUsedOnGlobalVariable => write!(f, "Augmented assignment used on global"),
			Self::VariableStartsWithTnk => write!(f, "Variable starts with \"_tnk_\""),
			Self::CannotBeInsideBlockOrFunction => write!(f, "Cannot be inside block or function"),
			Self::ExpressionCannotBeLValue => write!(f, "Expression cannot be l-value"),
			Self::ExpressionCannotBeRValue => write!(f, "Expression cannot be r-value"),
			Self::DuplicateGlobalVariableWithDifferentValues => write!(f, "Duplicate global variable with different values"),
			Self::UnableToConstCompile => write!(f, "Unable to const-compile"),
			Self::VariableNotFound => write!(f, "Variable not found"),
			Self::UnexpectedBuiltinFunctionArgumentCount { expected_min, expected_max, got } => {
				write!(f, "Unexpected built-in function argument count of {got}")?;
				if let Some(expected) = expected_min && expected_min == expected_max {
					write!(f, ", expected: {expected}")?;
				}
				else {
					if let Some(expected_min) = expected_min {
						write!(f, ", expected min: {expected_min}")?;
					}
					if let Some(expected_max) = expected_max {
						write!(f, ", expected max: {expected_max}")?;
					}
				}
				Ok(())
			}
			Self::UnexpectedValueType { value, expected_type } => {
				write!(f, "Unexpected value type for value: ")?;
				value.print_name(f)?;
				write!(f, ", of type:")?;
				value.get_type().print_name(f)?;
				if let Some(expected_type) = expected_type {
					write!(f, ", expected type:")?;
					expected_type.print_name(f)?;
				}
				Ok(())
			}
			Self::InvalidIntegerBitWidth(width) => write!(f, "Invalid integer bit width {width}, must be 8, 16, 32 or 64"),
			Self::InvalidFloatBitWidth(width) => write!(f, "Invalid float bit width {width}, must be 32 or 64"),
			Self::IntegerTooLargeForType(value, type_t) => write!(f, "Integer overflow, value {value} is too large for type {type_t}"),
			Self::IntegerTooSmallForType(value, type_t) => write!(f, "Integer underflow, value {value} is too small for type {type_t}"),
			Self::CannotUseUnaryOperatorForType { type_t, operator } => write!(f, "Cannot use the unary {operator} operator on a value of type {type_t}"),
			Self::CannotUseBinaryOperatorForType { lhs_type_t, rhs_type_t, operator }
				=> write!(f, "Cannot use the binary {operator} operator on values of type {lhs_type_t} and {rhs_type_t}"),
			Self::DivisionByZero => write!(f, "Division by zero"),
			Self::ModuloByZero => write!(f, "Modulo by zero"),
			Self::MultipleEntrypoints => write!(f, "Multiple entrypoints"),
			Self::EntrypointOnNonFunction => write!(f, "@entrypoint used on non-function"),
			Self::NegativeFactorial => write!(f, "Factorial of negative"),
			Self::ExpectedType => write!(f, "Expected type"),
			Self::TypeMismatch((got, expected)) => write!(f, "Type mismatch, got {got}, expected {expected}"),
			Self::ArgumentCountMismatch((got, expected)) => write!(f, "Argument count mismatch, got {got}, expected {expected}"),
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
			(Some(line), Some(column), Some(file)) => write!(f, " at {file}:{line}:{column}")?,
			(Some(line), None,                         Some(file)) => write!(f, " in {file} on line {line}")?,
			(None,                       None,                         Some(file)) => write!(f, " in {file}")?,
			(Some(line), Some(column), None              ) => write!(f, " at {line}:{column}")?,
			(Some(line), None,                         None              ) => write!(f, " on line {line}")?,
			(None,                       None,                         None              ) => write!(f, "")?,
			(None,                       Some(column), Some(file)) => write!(f, " in {file} in column {column}")?,
			(None,                       Some(column), None              ) => write!(f, " in column {column}")?,
		}
		Ok(())
	}
}