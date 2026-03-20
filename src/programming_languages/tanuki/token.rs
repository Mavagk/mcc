use std::{fmt::{self, Debug, Display, Formatter}, num::NonZeroUsize};

use num::BigUint;

use crate::traits::token::Token;

#[derive(Clone)]
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
	Keyword(TanukiKeyword),
	/// A label for naming block expressions that started with a `'` char, contained string is the source code literal without the leading `'`.
	BlockLabel(Box<str>),
	/// Contains a numeric literal tokenized to int and float types if they are valid for each types.
	NumericLiteral(Option<BigUint>, Option<f64>),
	/// Tokenized from a string literal, contains the content without the surrounding `"` chars and escape sequences have been escaped.
	StringLiteral(Box<str>),
	/// Contains the char that has been parsed from a char literal.
	CharacterLiteral(char),
	/// Tokenized from an operator literal.
	Operator {
		prefix_unary_operator: Option<TanukiPrefixUnaryOperator>,
		infix_binary_operator: Option<TanukiInfixBinaryOperator>,
		postfix_unary_operator: Option<TanukiPostfixUnaryOperator>,
		infix_ternary_operator: Option<TanukiInfixTernaryOperator>, 
		nullary_operator: Option<TanukiNullaryOperator>,
		is_colon: bool,
		is_assignment: bool,
		symbol: Box<str>,
	},
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
			TanukiTokenVariant::Operator {
				prefix_unary_operator, infix_binary_operator, postfix_unary_operator, infix_ternary_operator, nullary_operator, is_assignment, is_colon, symbol: _
			} => {
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
				if *is_colon {
					write!(f, " Colon")?;
				}
				if *is_assignment {
					write!(f, " Assignment")?;
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
pub enum TanukiPrefixUnaryOperator {
	/// Reads an l-value and converts it to an r-value. This operator is a no-op when used on r-values.
	Read,             // +
	Not,              // !
	MemberAccess,     // .
	/// Returns a random number in the 0..x range.
	//Roll,             // ?
	/// Returns 1. / x, floats only.
	Reciprocal,       // /
	BitshiftRightOne, // <<
	/// Returns x with it's imaginary part sign flipped, complex numbers only.
	ComplexConjugate, // |
	/// Returns -1 if x is negative, 0 if x is 0, 1 if x is positive.
	Signum,           // <=>

	/// Returns 0 - x, underflow terminates the program.
	Negation,           // -
	/// Negation but underflow results in the minimum value for the type, integers (non-big) only.
	SaturatingNegation, // -|
	/// Negation but can underflow, integers (non-big) only.
	WrappingNegation,   // -%
	TryNegation,        // -?

	/// Returns x ** 2, underflow terminates the program.
	Square,           // **
	/// Square but overflow results in the maximum value for the type, integers (non-big) only.
	SaturatingSquare, // **|
	/// Square but can overflow, integers (non-big) only.
	WrappingSquare,   // **%
	TrySquare,        // **?

	/// Returns x << 1, underflow/underflow terminates the program, integers only.
	BitshiftLeftOne,           // <<
	/// Bitshift left but overflow/underflow results in the maximum/minimum value for the type, integers (non-big) only.
	SaturatingBitshiftLeftOne, // <<|
	/// Square but can overflow/underflow, integers (non-big) only.
	WrappingBitshiftLeftOne,   // <<%
	TryBitshiftLeftOne,        // <<?

	/// Used on an l-value. Increments the l-value's value then returns the same l-value that was the input.
	Increment,           // ++
	SaturatingIncrement, // ++|
	WrappingIncrement,   // ++%

	/// Used on an l-value. Decrements the l-value's value then returns the same l-value that was the input.
	Decrement,           // --
	SaturatingDecrement, // --|
	WrappingDecrement,   // --%

	AddressOf,        // &
	Dereference,      // *
	/// Wraps the input n value in a nth to last wrapper. Indexing using the result gives the n-th to last value of the container being indexed.
	NthToLast,        // ^
	/// Gives a half open range with the last value in the range being x-1.
	RangeToExclusive, // ..
	/// Gives a half open range with the last value in the range being x.
	RangeToInclusive, // ..=
}

impl Display for TanukiPrefixUnaryOperator {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}

impl TanukiPrefixUnaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Read             => write!(f, "Read +"),
			Self::Not              => write!(f, "Not !"),
			Self::MemberAccess     => write!(f, "Member Access ."),
			//Self::Roll             => write!(f, "Roll ?"),
			Self::Reciprocal       => write!(f, "Reciprocal /"),
			Self::BitshiftRightOne => write!(f, "Bitshift Right One >>"),
			Self::ComplexConjugate => write!(f, "Complex Conjugate |"),
			Self::Signum           => write!(f, "Signum <=>"),

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

			Self::AddressOf         => write!(f, "Address of &"),
			Self::Dereference       => write!(f, "Dereference *"),
			Self::NthToLast         => write!(f, "Nth to Last ^"),
			Self::RangeToExclusive  => write!(f, "Range to Exclusive .."),
			Self::RangeToInclusive  => write!(f, "Range to Inclusive ..="),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"+"  => Self::Read,
			"!"  => Self::Not,
			"."  => Self::MemberAccess,
			//"?"  => Self::Roll,
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

			"&"   => Self::AddressOf,
			"*"   => Self::Dereference,
			"^"   => Self::NthToLast,
			".."  => Self::RangeToExclusive,
			"..=" => Self::RangeToInclusive,

			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TanukiInfixBinaryOperator {
	/// The operator variant of the `operator`
	None,

	// 100
	MemberAccess, // .

	// 101

	As,           // ->
	SaturatingAs, // ->|
	WrappingAs,   // ->%
	TryAs,        // ->?

	// 102

	Exponent,           // **
	SaturatingExponent, // **|
	WrappingExponent,   // **%
	TryExponent,        // **?

	// 103

	Multiplication,           // *
	SaturatingMultiplication, // *|
	WrappingMultiplication,   // *%
	TryMultiplication,        // *?

	Division,           // /
	SaturatingDivision, // /|
	WrappingDivision,   // /%
	TryDivision,        // /?

	Modulo,           // %
	SaturatingModulo, // %|
	WrappingModulo,   // %%
	TryModulo,        // %?

	// 104

	Addition,           // +
	SaturatingAddition, // +|
	WrappingAddition,   // +%
	TryAddition,        // +?

	Subtraction,           // -
	SaturatingSubtraction, // -|
	WrappingSubtraction,   // -%
	TrySubtraction,        // -?

	// 105 // TODO: Maybe different precedence

	//Concatenate, // ++
	Append,      // +++

	// 106

	BitshiftLeft,           // <<
	SaturatingBitshiftLeft, // <<|
	WrappingBitshiftLeft,   // <<%
	TryBitshiftLeft,        // <<?

	BitshiftRight, // >>

	// 107

	/// Returns -1 if lhs < rhs, 0 if lhs == rhs, 1 if lhs > rhs.
	ThreeWayCompare, // <=>

	// 108

	LessThan,             // <
	LessThanOrEqualTo,    // <=
	GreaterThan,          // >
	GreaterThanOrEqualTo, // >=

	// 109

	Equality,            // ==
	Inequality,          // !=
	ReferenceEquality,   // ===
	ReferenceInequality, // !==

	// 110

	NonShortCircuitAnd,  // &
	NonShortCircuitNand, // !&

	// 111

	NonShortCircuitXor,  // ^
	NonShortCircuitXnor, // !^

	// 112

	NonShortCircuitOr,  // |
	NonShortCircuitNor, // !|

	// 113

	Minimum, // <<<
	Maximum, // >>>

	// 114

	Pipe, // |>

	// 115

	ShortCircuitAnd,  // &&
	ShortCircuitNand, // !&&

	// 116

	ShortCircuitXor,  // ^^
	ShortCircuitXnor, // !^^

	// 117

	ShortCircuitOr,  // ||
	ShortCircuitNor, // !||

	// 118

	NonShortCircuitingNullCoalescing,
	ShortCircuitingNullCoalescing,

	// 119

	ExclusiveRange,
	InclusiveRange,
}

impl Display for TanukiInfixBinaryOperator {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}

impl TanukiInfixBinaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::None => write!(f, "None"),

			Self::MemberAccess => write!(f, "member Access ."),

			Self::As           => write!(f, "As ->"),
			Self::SaturatingAs => write!(f, "Saturating As ->|"),
			Self::WrappingAs   => write!(f, "Wrapping As ->%"),
			Self::TryAs        => write!(f, "Try As ->?"),

			Self::Exponent           => write!(f, "Exponent **"),
			Self::SaturatingExponent => write!(f, "Saturating Exponent **|"),
			Self::WrappingExponent   => write!(f, "Wrapping Exponent **%"),
			Self::TryExponent        => write!(f, "Try Exponent **?"),

			Self::Multiplication           => write!(f, "Multiplication *"),
			Self::SaturatingMultiplication => write!(f, "Saturating Multiplication *|"),
			Self::WrappingMultiplication   => write!(f, "Wrapping Multiplication *%"),
			Self::TryMultiplication        => write!(f, "Try Multiplication *?"),

			Self::Division           => write!(f, "Division /"),
			Self::SaturatingDivision => write!(f, "Saturating Division /|"),
			Self::WrappingDivision   => write!(f, "Wrapping Division /%"),
			Self::TryDivision        => write!(f, "Try Division /?"),

			Self::Modulo           => write!(f, "Modulo %"),
			Self::SaturatingModulo => write!(f, "Saturating Modulo %|"),
			Self::WrappingModulo   => write!(f, "Wrapping Modulo %%"),
			Self::TryModulo        => write!(f, "Try Modulo %?"),

			Self::Addition           => write!(f, "Addition +"),
			Self::SaturatingAddition => write!(f, "Saturating Addition +|"),
			Self::WrappingAddition   => write!(f, "Wrapping Addition +%"),
			Self::TryAddition        => write!(f, "Try Addition +?"),

			Self::Subtraction           => write!(f, "Subtraction -"),
			Self::SaturatingSubtraction => write!(f, "Saturating Subtraction -|"),
			Self::WrappingSubtraction   => write!(f, "Wrapping Subtraction -%"),
			Self::TrySubtraction        => write!(f, "Try Subtraction -?"),

			//Self::Concatenate => write!(f, "Concatenate ++"),
			Self::Append      => write!(f, "Append +++"),

			Self::BitshiftLeft           => write!(f, "Bitshift Left <<"),
			Self::SaturatingBitshiftLeft => write!(f, "Saturating Bitshift Left <<|"),
			Self::WrappingBitshiftLeft   => write!(f, "Wrapping Bitshift Left <<%"),
			Self::TryBitshiftLeft        => write!(f, "Try Bitshift Left <<?"),

			Self::BitshiftRight => write!(f, "Bitshift Left >>"),

			Self::ThreeWayCompare => write!(f, "Three Way Compare <=>"),

			Self::LessThan             => write!(f, "Less than <"),
			Self::LessThanOrEqualTo    => write!(f, "Less than or Equal to <="),
			Self::GreaterThan          => write!(f, "Greater than >"),
			Self::GreaterThanOrEqualTo => write!(f, "Greater than or Equal to >="),

			Self::Equality            => write!(f, "Equality =="),
			Self::Inequality          => write!(f, "Inequality !="),
			Self::ReferenceEquality   => write!(f, "Reference Equality ==="),
			Self::ReferenceInequality => write!(f, "Reference Inequality !=="),

			Self::NonShortCircuitAnd  => write!(f, "Non-Short Circuit AND &"),
			Self::NonShortCircuitNand => write!(f, "Non-Short Circuit NAND !&"),

			Self::NonShortCircuitXor  => write!(f, "Non-Short Circuit XOR ^"),
			Self::NonShortCircuitXnor => write!(f, "Non-Short Circuit NAND !^"),

			Self::NonShortCircuitOr  => write!(f, "Non-Short Circuit OR |"),
			Self::NonShortCircuitNor => write!(f, "Non-Short Circuit NOR !|"),

			Self::Minimum => write!(f, "Minimum <<<"),
			Self::Maximum => write!(f, "Maximum >>>"),

			Self::Pipe => write!(f, "Pipe |>"),

			Self::ShortCircuitAnd  => write!(f, "Short Circuit AND &&"),
			Self::ShortCircuitNand => write!(f, "Short Circuit NAND !&&"),

			Self::ShortCircuitXor  => write!(f, "Short Circuit XOR ^^"),
			Self::ShortCircuitXnor => write!(f, "Short Circuit NAND !^^"),

			Self::ShortCircuitOr  => write!(f, "Short Circuit OR ||"),
			Self::ShortCircuitNor => write!(f, "Short Circuit NOR !||"),

			Self::NonShortCircuitingNullCoalescing => write!(f, "Non-Short Circuiting Null Coalescing ?"),
			Self::ShortCircuitingNullCoalescing    => write!(f, "Short Circuiting Null Coalescing ??"),

			Self::ExclusiveRange => write!(f, "Exclusive Range .."),
			Self::InclusiveRange => write!(f, "Inclusive Range ..="),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"" => Self::None,

			"." => Self::MemberAccess,

			"->"  => Self::As,
			"->|" => Self::SaturatingAs,
			"->%" => Self::WrappingAs,
			"->?" => Self::TryAs,

			"**"  => Self::Exponent,
			"**|" => Self::SaturatingExponent,
			"**%" => Self::WrappingExponent,
			"**?" => Self::TryExponent,

			"*"  => Self::Multiplication,
			"*|" => Self::SaturatingMultiplication,
			"*%" => Self::WrappingMultiplication,
			"*?" => Self::TryMultiplication,

			"/"  => Self::Division,
			"/|" => Self::SaturatingDivision,
			"/%" => Self::WrappingDivision,
			"/?" => Self::TryDivision,

			"%"  => Self::Modulo,
			"%|" => Self::SaturatingModulo,
			"%%" => Self::WrappingModulo,
			"%?" => Self::TryModulo,

			"+"  => Self::Addition,
			"+|" => Self::SaturatingAddition,
			"+%" => Self::WrappingAddition,
			"+?" => Self::TryAddition,

			"-"  => Self::Subtraction,
			"-|" => Self::SaturatingSubtraction,
			"-%" => Self::WrappingSubtraction,
			"-?" => Self::TrySubtraction,

			//"++"  => Self::Concatenate,
			"+++" => Self::Append,

			"<<"  => Self::BitshiftLeft,
			"<<|" => Self::SaturatingBitshiftLeft,
			"<<%" => Self::WrappingBitshiftLeft,
			"<<?" => Self::TryBitshiftLeft,

			">>" => Self::BitshiftRight,

			"<=>" => Self::ThreeWayCompare,

			"<"  => Self::LessThan,
			"<=" => Self::LessThanOrEqualTo,
			">"  => Self::GreaterThan,
			">=" => Self::GreaterThanOrEqualTo,

			"&"  => Self::NonShortCircuitAnd,
			"!&" => Self::NonShortCircuitNand,

			"^"  => Self::NonShortCircuitXor,
			"!^" => Self::NonShortCircuitXnor,

			"|"  => Self::NonShortCircuitOr,
			"!|" => Self::NonShortCircuitNor,

			"<<<" => Self::Minimum,
			">>>" => Self::Maximum,

			"|>" => Self::Pipe,

			"&&"  => Self::ShortCircuitAnd,
			"!&&" => Self::ShortCircuitNand,

			"^^"  => Self::ShortCircuitXor,
			"!^^" => Self::ShortCircuitXnor,

			"||"  => Self::ShortCircuitOr,
			"!||" => Self::ShortCircuitNor,

			"?"  => Self::NonShortCircuitingNullCoalescing,
			"??" => Self::ShortCircuitingNullCoalescing,

			".."  => Self::ExclusiveRange,
			"..=" => Self::InclusiveRange,

			_ => return None,
		})
	}

	pub const PRECEDENCE_LEVELS: &'static[&'static[Self]; 19] = &[
		&[Self::As, Self::SaturatingAs, Self::WrappingAs, Self::TryAs],
		&[Self::Exponent, Self::SaturatingExponent, Self::WrappingExponent, Self::TryExponent],
		&[
			Self::Multiplication, Self::SaturatingMultiplication, Self::WrappingMultiplication, Self::TryMultiplication,
			Self::Division,       Self::SaturatingDivision,       Self::WrappingDivision,       Self::TryDivision,
			Self::Modulo,         Self::SaturatingModulo,         Self::WrappingModulo,         Self::TryModulo,
		],
		&[Self::Addition, Self::SaturatingAddition, Self::WrappingAddition, Self::TryAddition, Self::Subtraction, Self::SaturatingSubtraction, Self::WrappingSubtraction, Self::Subtraction],
		&[Self::Append],
		&[Self::BitshiftLeft, Self::SaturatingBitshiftLeft, Self::WrappingBitshiftLeft, Self::TryBitshiftLeft, Self::BitshiftRight],
		&[Self::ThreeWayCompare],
		&[Self::LessThan, Self::LessThanOrEqualTo, Self::GreaterThan, Self::GreaterThanOrEqualTo],
		&[Self::Minimum, Self::Maximum],
		&[Self::Pipe],
		&[Self::Equality, Self::Inequality, Self::ReferenceEquality, Self::ReferenceInequality],
		&[Self::NonShortCircuitAnd, Self::NonShortCircuitNand],
		&[Self::NonShortCircuitXor, Self::NonShortCircuitXnor],
		&[Self::NonShortCircuitOr, Self::NonShortCircuitNor],
		&[Self::ShortCircuitAnd, Self::ShortCircuitNand],
		&[Self::ShortCircuitXor, Self::ShortCircuitXnor],
		&[Self::ShortCircuitOr, Self::ShortCircuitNor],
		&[Self::NonShortCircuitingNullCoalescing, Self::ShortCircuitingNullCoalescing],
		&[Self::ExclusiveRange, Self::InclusiveRange],
	];
}

#[derive(Debug, Clone, Copy)]
pub enum TanukiPostfixUnaryOperator {
	/// Returns x / 100, floats only.
	Percent, // %

	/// Returns the factorial of x if it is an integer or gamma(x + 1.) if the input is a float. Integer overflow will result in program termination.
	Factorial,           // !
	/// The factorial of x, returns the maximum value if x overflows, integers (non-big) only.
	SaturatingFactorial, // !|
	/// The factorial of x, can overflow, integers (non-big) only.
	WrappingFactorial,   // !%
	TryFactorial,        // !?

	/// Used on an l-value. Returns the same l-value that was the input. The l-value will be incremented after the expression the operator is contained in is evaluated.
	Increment,           // ++
	SaturatingIncrement, // ++|
	WrappingIncrement,   // ++%

	/// Used on an l-value. Returns the same l-value that was the input. The l-value will be decremented after the expression the operator is contained in is evaluated.
	Decrement,           // --
	SaturatingDecrement, // --|
	WrappingDecrement,   // --%

	TryPropagate, // ?
	Unwrap,       // .?
	// /// Returns a half open range starting from x.
	//RangeFrom,    // ..
}

impl Display for TanukiPostfixUnaryOperator {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}

impl TanukiPostfixUnaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Percent => write!(f, "Percent %"),

			Self::Factorial           => write!(f, "Factorial !"),
			Self::SaturatingFactorial => write!(f, "Saturating Factorial !|"),
			Self::WrappingFactorial   => write!(f, "Wrapping Factorial !%"),
			Self::TryFactorial        => write!(f, "Try Factorial !?"),

			Self::Increment           => write!(f, "Increment ++"),
			Self::SaturatingIncrement => write!(f, "Saturating Increment ++|"),
			Self::WrappingIncrement   => write!(f, "Wrapping Increment ++%"),

			Self::Decrement           => write!(f, "Decrement --"),
			Self::SaturatingDecrement => write!(f, "Saturating Decrement --|"),
			Self::WrappingDecrement   => write!(f, "Wrapping Decrement --%"),

			Self::TryPropagate => write!(f, "Try Propagate ?"),
			Self::Unwrap       => write!(f, "Unwrap .?"),
			//Self::RangeFrom    => write!(f, "Range From .."),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"%" => Self::Percent,

			"!" => Self::Factorial,
			"!|" => Self::SaturatingFactorial,
			"!%" => Self::WrappingFactorial,
			"!?" => Self::TryFactorial,

			"++"  => Self::Increment,
			"++|" => Self::SaturatingIncrement,
			"++%" => Self::WrappingIncrement,

			"--"  => Self::Decrement,
			"--|" => Self::SaturatingDecrement,
			"--%" => Self::WrappingDecrement,

			"?" => Self::TryPropagate,
			".?" => Self::Unwrap,
			//".." => Self::RangeFrom,
			
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TanukiInfixTernaryOperator {
	NonShortCircuitingConditional,
	ShortCircuitingConditional,
}

impl TanukiInfixTernaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::NonShortCircuitingConditional => write!(f, "Non-Short Circuiting Conditional ?"),
			Self::ShortCircuitingConditional    => write!(f, "Short Circuiting Conditional ??"),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"?" => Self::NonShortCircuitingConditional,
			"??" => Self::ShortCircuitingConditional,

			_ => return None,
		})
	}
}

impl Display for TanukiInfixTernaryOperator {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TanukiNullaryOperator {
	/// Returns a random number value from the output type.
	Roll, // ?

	/// Returns a last-value constant. Indexing using the result gives the last value of the container being indexed.
	Last, // ^

	FullRange, // ..
}

impl TanukiNullaryOperator {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Roll => write!(f, "Roll ?"),

			Self::Last => write!(f, "Last ^"),

			Self::FullRange => write!(f, "Full Range .."),
		}
	}

	pub fn from_source(source: &str) -> Option<Self> {
		Some(match source {
			"?" => Self::Roll,

			"^" => Self::Last,

			".." => Self::FullRange,

			_ => return None,
		})
	}
}

impl Display for TanukiNullaryOperator {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TanukiKeyword {
	Import,
	ImportStd,
	Export,
	Link,
	LinkIf,
	U,
	I,
	F,
	True,
	False,
	Void,
	Break,
	Continue,
	Redo,
	Entrypoint,
	Bool,
	Int,
	Info,
	Transmute,
	Type,
}

impl TanukiKeyword {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Import     => write!(f, "Import"),
			Self::ImportStd  => write!(f, "Import STD"),
			Self::Export     => write!(f, "Export"),
			Self::Link       => write!(f, "Link"),
			Self::LinkIf     => write!(f, "Link If"),
			Self::U          => write!(f, "U"),
			Self::I          => write!(f, "I"),
			Self::F          => write!(f, "F"),
			Self::True       => write!(f, "True"),
			Self::False      => write!(f, "False"),
			Self::Void       => write!(f, "Void"),
			Self::Break      => write!(f, "Break"),
			Self::Continue   => write!(f, "Continue"),
			Self::Redo       => write!(f, "Redo"),
			Self::Entrypoint => write!(f, "Entrypoint"),
			Self::Bool       => write!(f, "Bool"),
			Self::Int        => write!(f, "Int"),
			Self::Info       => write!(f, "Info"),
			Self::Transmute  => write!(f, "Transmute"),
			Self::Type       => write!(f, "Type"),
		}
	}

	pub fn from_name(name: &str) -> Option<Self> {
		match name {
			"import"     => Some(Self::Import),
			"import_std" => Some(Self::ImportStd),
			"export"     => Some(Self::Export),
			"link"       => Some(Self::Link),
			"link_if"    => Some(Self::LinkIf),
			"u"          => Some(Self::U),
			"i"          => Some(Self::I),
			"f"          => Some(Self::F),
			"true"       => Some(Self::True),
			"false"      => Some(Self::False),
			"void"       => Some(Self::Void),
			"break"      => Some(Self::Break),
			"continue"   => Some(Self::Continue),
			"redo"       => Some(Self::Redo),
			"entrypoint" => Some(Self::Entrypoint),
			"bool"       => Some(Self::Bool),
			"int"        => Some(Self::Int),
			"_info"      => Some(Self::Info),
			"transmute"  => Some(Self::Transmute),
			"type"       => Some(Self::Type),
			_ => None,
		}
	}
}