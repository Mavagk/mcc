use std::num::NonZeroUsize;

use crate::programming_languages::tanuki::{expression::TanukiExpression, token::InfixTernaryOperator};

#[derive(Debug, Clone)]
pub struct TanukiPartiallyParsedToken {
	pub variant: TanukiPartiallyParsedTokenVariant,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub enum TanukiPartiallyParsedTokenVariant {
	FunctionArgumentsOrParameters(Box<[TanukiExpression]>),
	SquareParenthesised(Box<TanukiExpression>),
	/// A ternary operator, the matching colon and the expression in between.
	TernaryOperatorCenter(InfixTernaryOperator, Box<TanukiExpression>),
}