use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

use num::BigUint;

use crate::traits::token::Token;

pub struct TanukiToken {
	pub variant: TanukiTokenVariant,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

#[derive(Clone, Debug)]
pub enum TanukiTokenVariant {
	LeftParenthesis,
	RightParenthesis,
	LeftCurlyParenthesis,
	RightCurlyParenthesis,
	LeftSquareParenthesis,
	RightSquareParenthesis,
	Comma,
	Semicolon,
	Identifier(Box<str>),
	Keyword(Keyword),
	BlockLabel(Box<str>),
	NumericLiteral(Option<BigUint>, Option<f64>),
	StringLiteral(Box<str>),
	CharacterLiteral(char),
	Operator(Option<PrefixUnaryOperator>, Option<InfixBinaryOperator>, Option<PostfixUnaryOperator>, Option<InfixTernaryOperator>),
}

impl Token for TanukiToken {
	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.start_line)
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.start_column)
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.end_line)
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.end_column)
	}

	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiTokenVariant::LeftParenthesis             => write!(f, "Left Parenthesis"),
			TanukiTokenVariant::RightParenthesis            => write!(f, "Right Parenthesis"),
			TanukiTokenVariant::LeftCurlyParenthesis        => write!(f, "Left Curly Parenthesis"),
			TanukiTokenVariant::RightCurlyParenthesis       => write!(f, "Right Curly Parenthesis"),
			TanukiTokenVariant::LeftSquareParenthesis       => write!(f, "Left Square Parenthesis"),
			TanukiTokenVariant::RightSquareParenthesis      => write!(f, "Right Square Parenthesis"),
			TanukiTokenVariant::Comma                       => write!(f, "Comma"),
			TanukiTokenVariant::Semicolon                   => write!(f, "Semicolon"),
			TanukiTokenVariant::Identifier(name) => write!(f, "Identifier \"{name}\""),
			TanukiTokenVariant::Keyword(keyword)  => {
				write!(f, "Keyword ")?;
				keyword.print_name(f)
			}
			TanukiTokenVariant::BlockLabel(name) => write!(f, "Block Label \"{name}\""),
			TanukiTokenVariant::NumericLiteral(int_value, float_value) => {
				write!(f, "Numeric Literal")?;
				if let Some(int_value) = int_value {
					write!(f, " Integer {int_value}")?;
				}
				if let Some(float_value) = float_value {
					write!(f, " Float {float_value}")?;
				}
				Ok(())
			}
			TanukiTokenVariant::StringLiteral(value) => write!(f, "String Literal {value:?}"),
			TanukiTokenVariant::CharacterLiteral(value) => write!(f, "Character Literal {value:?}"),
			TanukiTokenVariant::Operator(prefix_unary_operator, infix_binary_operator, postfix_unary_operator, infix_ternary_operator) => {
				write!(f, "Operator")?;
				if let Some(prefix_unary_operator) = prefix_unary_operator {
					write!(f, " Prefix Unary ")?;
					prefix_unary_operator.print_name(f)?;
				}
				if let Some(infix_binary_operator) = infix_binary_operator {
					write!(f, " Infix Binary ")?;
					infix_binary_operator.print_name(f)?;
				}
				if let Some(postfix_unary_operator) = postfix_unary_operator {
					write!(f, " Postfix Unary ")?;
					postfix_unary_operator.print_name(f)?;
				}
				if let Some(infix_ternary_operator) = infix_ternary_operator {
					write!(f, " Infix Ternary ")?;
					infix_ternary_operator.print_name(f)?;
				}
				Ok(())
			}
		}
	}
}

impl Debug for TanukiToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(f)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixUnaryOperator {

}

impl PrefixUnaryOperator {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			_ => todo!()
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InfixBinaryOperator {
	
}

impl InfixBinaryOperator {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			_ => todo!()
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixUnaryOperator {
	
}

impl PostfixUnaryOperator {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			_ => todo!()
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InfixTernaryOperator {
	
}

impl InfixTernaryOperator {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			_ => todo!()
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
	
}

impl Keyword {
	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			_ => todo!()
		}
	}

	pub fn from_name(name: &str) -> Option<Self> {
		match name {
			_ => None,
		}
	}
}