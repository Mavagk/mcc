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
	/// Tokenized from a single `(` char.
	LeftParenthesis,
	/// Tokenized from a single `)` char.
	RightParenthesis,
	/// Tokenized from a single `{` char.
	LeftCurlyParenthesis,
	/// Tokenized from a single `}` char.
	RightCurlyParenthesis,
	/// Tokenized from a single `[` char.
	LeftSquareParenthesis,
	/// Tokenized from a single `]` char.
	RightSquareParenthesis,
	/// Tokenized from a single `,` char.
	Comma,
	/// Tokenized from a single `;` char.
	Semicolon,
	/// An identifier for naming variables, consists of letters, digits, underscores and all but the first char can be digits.
	Identifier(Box<str>),
	/// Tokenized from a keyword that started with an `@` sign.
	Keyword(Keyword),
	/// A label for naming block expressions that started with a `'` char, contained string is the source code literal without the leading `'`.
	BlockLabel(Box<str>),
	/// Contains a numeric literal tokenized to int and float types if they are valid for each types.
	NumericLiteral(Option<BigUint>, Option<f64>),
	/// Tokenized from a string literal, contains the content without the surrounding `"` chars and escape sequences have been escaped.
	StringLiteral(Box<str>),
	/// Contains the char that has been parsed from a char literal.
	CharacterLiteral(char),
	/// Tokenized from an operator literal.
	Operator(Option<PrefixUnaryOperator>, Option<InfixBinaryOperator>, Option<PostfixUnaryOperator>, Option<InfixTernaryOperator>, Option<NullaryOperator>, Box<str>),
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
			TanukiTokenVariant::Operator(prefix_unary_operator, infix_binary_operator, postfix_unary_operator, infix_ternary_operator, nullary_operator, _symbol) => {
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
				if let Some(nullary_operator) = nullary_operator {
					write!(f, " Nullary ")?;
					nullary_operator.print_name(f)?;
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
	// Reads an l-value and converts it to an r-value. This operator is a no-op when used on r-values.
	Read,             // +
	Not,              // !
	Roll,             // ?
	Reciprocal,       // /
	BitshiftRightOne, // <<
	ComplexConjugate, // |

	Negation,           // -
	SaturatingNegation, // -|
	WrappingNegation,   // -%
	TryNegation,        // -?

	Square,           // **
	SaturatingSquare, // **|
	WrappingSquare,   // **%
	TrySquare,        // **?

	BitshiftLeftOne,           // <<
	SaturatingBitshiftLeftOne, // <<|
	WrappingBitshiftLeftOne,   // <<%
	TryBitshiftLeftOne,        // <<?

	Increment,           // ++
	SaturatingIncrement, // ++|
	WrappingIncrement,   // ++%

	Decrement,           // --
	SaturatingDecrement, // --|
	WrappingDecrement,   // --%

	AddressOf,   // &
	Dereference, // *
	NthToLast,   // ^
}

impl PrefixUnaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Read             => write!(f, "Read +"),
			Self::Not              => write!(f, "Not !"),
			Self::Roll             => write!(f, "Roll ?"),
			Self::Reciprocal       => write!(f, "Reciprocal /"),
			Self::BitshiftRightOne => write!(f, "Bitshift Right One >>"),
			Self::ComplexConjugate => write!(f, "Complex Conjugate |"),

			Self::Negation           => write!(f, "Negation -"),
			Self::SaturatingNegation => write!(f, "Saturating Negation -|"),
			Self::WrappingNegation   => write!(f, "Wrapping Negation -%"),
			Self::TryNegation        => write!(f, "Try Negation -?"),

			Self::Square           => write!(f, "Square **"),
			Self::SaturatingSquare => write!(f, "Saturating Square **|"),
			Self::WrappingSquare   => write!(f, "Wrapping Square **%"),
			Self::TrySquare        => write!(f, "Try Square **?"),

			Self::BitshiftLeftOne           => write!(f, "Bitshift Left One <<"),
			Self::SaturatingBitshiftLeftOne => write!(f, "Saturating Bitshift Left One <<|"),
			Self::WrappingBitshiftLeftOne   => write!(f, "Wrapping Bitshift Left One <<%"),
			Self::TryBitshiftLeftOne        => write!(f, "Try Bitshift Left One <<?"),

			Self::Increment           => write!(f, "Increment ++"),
			Self::SaturatingIncrement => write!(f, "Saturating Increment ++|"),
			Self::WrappingIncrement   => write!(f, "Wrapping Increment ++%"),

			Self::Decrement           => write!(f, "Decrement --"),
			Self::SaturatingDecrement => write!(f, "Saturating Decrement --|"),
			Self::WrappingDecrement   => write!(f, "Wrapping Decrement --%"),

			Self::AddressOf           => write!(f, "Address of &"),
			Self::Dereference         => write!(f, "Dereference *"),
			Self::NthToLast           => write!(f, "Nth to Last ^"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"+"  => Self::Read,
			"!"  => Self::Not,
			"?"  => Self::Roll,
			"/"  => Self::Reciprocal,
			">>" => Self::BitshiftRightOne,
			"|"  => Self::ComplexConjugate,

			"-"  => Self::Negation,
			"-|" => Self::SaturatingNegation,
			"-%" => Self::WrappingNegation,
			"-?" => Self::TryNegation,

			"**"  => Self::Square,
			"**|" => Self::SaturatingSquare,
			"**%" => Self::WrappingSquare,
			"**?" => Self::TrySquare,

			"<<"  => Self::BitshiftLeftOne,
			"<<|" => Self::SaturatingBitshiftLeftOne,
			"<<%" => Self::WrappingBitshiftLeftOne,
			"<<?" => Self::TryBitshiftLeftOne,

			"++"  => Self::Increment,
			"++|" => Self::SaturatingIncrement,
			"++%" => Self::WrappingIncrement,

			"--"  => Self::Decrement,
			"--|" => Self::SaturatingDecrement,
			"--%" => Self::WrappingDecrement,

			"&" => Self::AddressOf,
			"*" => Self::Dereference,
			"^" => Self::NthToLast,

			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InfixBinaryOperator {
	Addition,
	Subtraction,
}

impl InfixBinaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Addition    => write!(f, "Addition +"),
			Self::Subtraction => write!(f, "Subtraction -"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"+" => Self::Addition,
			"-" => Self::Subtraction,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixUnaryOperator {
	TryPropagate
}

impl PostfixUnaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::TryPropagate => write!(f, "Try Propagate ?"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"?" => Self::TryPropagate,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InfixTernaryOperator {
	NonShortCircuitingConditional,
}

impl InfixTernaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::NonShortCircuitingConditional => write!(f, "Conditional ?"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"?" => Self::NonShortCircuitingConditional,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum NullaryOperator {
	Last,
}

impl NullaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Last => write!(f, "Last ^"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"^" => Self::Last,
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