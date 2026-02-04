use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use num::BigUint;

use crate::traits::token::Token;

#[derive(Debug)]
pub struct TanukiToken {
	variant: TanukiTokenVariant,
	start_line: NonZeroUsize,
	start_column: NonZeroUsize,
	end_line: NonZeroUsize,
	end_column: NonZeroUsize,
}

#[derive(Debug)]
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
	Keyword,
	BlockLabel(Box<str>),
	NumericLiteral(Option<BigUint>, Option<f64>),
	StringLiteral(Box<str>),
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
		match self.variant {
			TanukiTokenVariant::LeftParenthesis        => write!(f, "Left Parenthesis"),
			TanukiTokenVariant::RightParenthesis       => write!(f, "Right Parenthesis"),
			TanukiTokenVariant::LeftCurlyParenthesis   => write!(f, "Left Curly Parenthesis"),
			TanukiTokenVariant::RightCurlyParenthesis  => write!(f, "Right Curly Parenthesis"),
			TanukiTokenVariant::LeftSquareParenthesis  => write!(f, "Left Square Parenthesis"),
			TanukiTokenVariant::RightSquareParenthesis => write!(f, "Right Square Parenthesis"),
			TanukiTokenVariant::Comma                  => write!(f, "Comma"),
			TanukiTokenVariant::Semicolon              => write!(f, "Semicolon"),
			_ => todo!()
		}
	}
}

#[derive(Debug)]
pub enum PrefixUnaryOperator {

}

#[derive(Debug)]
pub enum InfixBinaryOperator {
	
}

#[derive(Debug)]
pub enum PostfixUnaryOperator {
	
}

#[derive(Debug)]
pub enum InfixTernaryOperator {
	
}