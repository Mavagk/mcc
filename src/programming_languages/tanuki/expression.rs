use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::tanuki::constant_value::TanukiConstantValue, traits::{ast_node::AstNode, expression::Expression}};

#[derive(Debug, Clone)]
pub struct TanukiExpression {
	pub variant: TanukiExpressionVariant,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub enum TanukiExpressionVariant {
	Constant(TanukiConstantValue),
	Block { sub_expressions: Box<[TanukiExpression]>, has_return_value: bool },
	Variable(Box<str>),
	FunctionCall { function_pointer: Box<TanukiExpression>, arguments: Box<[TanukiExpression]> },
	FunctionDefinition { parameters: Box<[TanukiExpression]>, return_type: Option<Box<TanukiExpression>>, body_expression: Box<TanukiExpression> },
	Index(Box<TanukiExpression>, Box<TanukiExpression>),
	TypeAndValue(Box<TanukiExpression>, Box<TanukiExpression>),
	Import(Box<[TanukiExpression]>),
	Export(Box<TanukiExpression>),
	Link(Box<[TanukiExpression]>),
	// Unary postfix operators
	Percent(Box<TanukiExpression>),
	Factorial(Box<TanukiExpression>),
	SaturatingFactorial(Box<TanukiExpression>),
	WrappingFactorial(Box<TanukiExpression>),
	TryFactorial(Box<TanukiExpression>),
	PostfixIncrement(Box<TanukiExpression>),
	PostfixSaturatingIncrement(Box<TanukiExpression>),
	PostfixWrappingIncrement(Box<TanukiExpression>),
	PostfixDecrement(Box<TanukiExpression>),
	PostfixSaturatingDecrement(Box<TanukiExpression>),
	PostfixWrappingDecrement(Box<TanukiExpression>),
	TryPropagate(Box<TanukiExpression>),
	Unwrap(Box<TanukiExpression>),
	// Unary prefix operators
	Read(Box<TanukiExpression>),
	Not(Box<TanukiExpression>),
	Reciprocal(Box<TanukiExpression>),
	BitshiftRightOne(Box<TanukiExpression>),
	ComplexConjugate(Box<TanukiExpression>),
	Signum(Box<TanukiExpression>),
	Negation(Box<TanukiExpression>),
	SaturatingNegation(Box<TanukiExpression>),
	WrappingNegation(Box<TanukiExpression>),
	TryNegation(Box<TanukiExpression>),
	Square(Box<TanukiExpression>),
	SaturatingSquare(Box<TanukiExpression>),
	WrappingSquare(Box<TanukiExpression>),
	TrySquare(Box<TanukiExpression>),
	BitshiftLeftOne(Box<TanukiExpression>),
	SaturatingBitshiftLeftOne(Box<TanukiExpression>),
	WrappingBitshiftLeftOne(Box<TanukiExpression>),
	TryBitshiftLeftOne(Box<TanukiExpression>),
	PrefixIncrement(Box<TanukiExpression>),
	PrefixSaturatingIncrement(Box<TanukiExpression>),
	PrefixWrappingIncrement(Box<TanukiExpression>),
	PrefixDecrement(Box<TanukiExpression>),
	PrefixSaturatingDecrement(Box<TanukiExpression>),
	PrefixWrappingDecrement(Box<TanukiExpression>),
	AddressOf(Box<TanukiExpression>),
	Dereference(Box<TanukiExpression>),
	NthToLast(Box<TanukiExpression>),
	RangeToExclusive(Box<TanukiExpression>),
	RangeToInclusive(Box<TanukiExpression>),
	// Binary infix operators
	MemberAccess(Box<TanukiExpression>, Box<TanukiExpression>),
	As(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingAs(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingAs(Box<TanukiExpression>, Box<TanukiExpression>),
	TryAs(Box<TanukiExpression>, Box<TanukiExpression>),
	Exponent(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingExponent(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingExponent(Box<TanukiExpression>, Box<TanukiExpression>),
	TryExponent(Box<TanukiExpression>, Box<TanukiExpression>),
	Multiplication(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingMultiplication(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingMultiplication(Box<TanukiExpression>, Box<TanukiExpression>),
	TryMultiplication(Box<TanukiExpression>, Box<TanukiExpression>),
	Division(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingDivision(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingDivision(Box<TanukiExpression>, Box<TanukiExpression>),
	TryDivision(Box<TanukiExpression>, Box<TanukiExpression>),
	Modulo(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingModulo(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingModulo(Box<TanukiExpression>, Box<TanukiExpression>),
	TryModulo(Box<TanukiExpression>, Box<TanukiExpression>),
	Addition(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingAddition(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingAddition(Box<TanukiExpression>, Box<TanukiExpression>),
	TryAddition(Box<TanukiExpression>, Box<TanukiExpression>),
	Subtraction(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingSubtraction(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingSubtraction(Box<TanukiExpression>, Box<TanukiExpression>),
	TrySubtraction(Box<TanukiExpression>, Box<TanukiExpression>),
	Concatenate(Box<TanukiExpression>, Box<TanukiExpression>),
	Append(Box<TanukiExpression>, Box<TanukiExpression>),
	BitshiftLeft(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingBitshiftLeft(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingBitshiftLeft(Box<TanukiExpression>, Box<TanukiExpression>),
	TryBitshiftLeft(Box<TanukiExpression>, Box<TanukiExpression>),
	BitshiftRight(Box<TanukiExpression>, Box<TanukiExpression>),
	ThreeWayCompare(Box<TanukiExpression>, Box<TanukiExpression>),
	LessThan(Box<TanukiExpression>, Box<TanukiExpression>),
	LessThanOrEqualTo(Box<TanukiExpression>, Box<TanukiExpression>),
	GreaterThan(Box<TanukiExpression>, Box<TanukiExpression>),
	GreaterThanOrEqualTo(Box<TanukiExpression>, Box<TanukiExpression>),
	Equality(Box<TanukiExpression>, Box<TanukiExpression>),
	Inequality(Box<TanukiExpression>, Box<TanukiExpression>),
	ReferenceEquality(Box<TanukiExpression>, Box<TanukiExpression>),
	ReferenceInequality(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitAnd(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitNand(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitXor(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitXnor(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitOr(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitNor(Box<TanukiExpression>, Box<TanukiExpression>),
	Minimum(Box<TanukiExpression>, Box<TanukiExpression>),
	Maximum(Box<TanukiExpression>, Box<TanukiExpression>),
	Pipe(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitAnd(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitNand(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitXor(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitXnor(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitOr(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitNor(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitingNullCoalescing(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitingNullCoalescing(Box<TanukiExpression>, Box<TanukiExpression>),
	ExclusiveRange(Box<TanukiExpression>, Box<TanukiExpression>),
	InclusiveRange(Box<TanukiExpression>, Box<TanukiExpression>),
	// Ternary operators
	NonShortCircuitingConditional(Box<TanukiExpression>, Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitingConditional(Box<TanukiExpression>, Box<TanukiExpression>, Box<TanukiExpression>),
	// Assignment binary operators
	Assignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ExponentAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingExponentAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingExponentAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	MultiplicationAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingMultiplicationAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingMultiplicationAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	DivisionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingDivisionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingDivisionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ModuloAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingModuloAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingModuloAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	AdditionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingAdditionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingAdditionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SubtractionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingSubtractionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingSubtractionAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ConcatenateAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	AppendAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	BitshiftLeftAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	SaturatingBitshiftLeftAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	WrappingBitshiftLeftAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	BitshiftRightAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ThreeWayCompareAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitAndAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitNandAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitXorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitXnorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitOrAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitNorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	MinimumAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	MaximumAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	PipeAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitAndAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitNandAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitXorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitXnorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitOrAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitNorAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	NonShortCircuitingNullCoalescingAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
	ShortCircuitingNullCoalescingAssignment(Box<TanukiExpression>, Box<TanukiExpression>),
}

impl Expression for TanukiExpression {}

impl AstNode for TanukiExpression {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(value) => write!(f, "Constant {value:?}"),
			TanukiExpressionVariant::Block { has_return_value, .. } => {
				write!(f, "Block")?;
				if *has_return_value {
					write!(f, ", has return value")?;
				}
				Ok(())
			},
			TanukiExpressionVariant::FunctionCall { .. }                            => write!(f, "Function Call"),
			TanukiExpressionVariant::FunctionDefinition { return_type, .. } => {
				write!(f, "Function Definition")?;
				if return_type.is_some() {
					write!(f, ", Has Return Type")?;
				}
				Ok(())
			},
			TanukiExpressionVariant::Index { .. }                                   => write!(f, "Index"),
			TanukiExpressionVariant::Variable(name)                      => write!(f, "Variable {name}"),
			TanukiExpressionVariant::TypeAndValue(..)                               => write!(f, "Type and Value"),
			TanukiExpressionVariant::Import(..)                                     => write!(f, "Import"),
			TanukiExpressionVariant::Export(..)                                     => write!(f, "Export"),
			TanukiExpressionVariant::Link(..)                                       => write!(f, "Link"),

			TanukiExpressionVariant::Percent(..)                                    => write!(f, "Percent"),
			TanukiExpressionVariant::Factorial(..)                                  => write!(f, "Factorial"),
			TanukiExpressionVariant::SaturatingFactorial(..)                        => write!(f, "Saturating Factorial"),
			TanukiExpressionVariant::WrappingFactorial(..)                          => write!(f, "Wrapping Factorial"),
			TanukiExpressionVariant::TryFactorial(..)                               => write!(f, "Try Factorial"),
			TanukiExpressionVariant::PostfixIncrement(..)                           => write!(f, "Postfix Increment"),
			TanukiExpressionVariant::PostfixSaturatingIncrement(..)                 => write!(f, "Postfix Saturating Increment"),
			TanukiExpressionVariant::PostfixWrappingIncrement(..)                   => write!(f, "Postfix Wrapping Increment"),
			TanukiExpressionVariant::PostfixDecrement(..)                           => write!(f, "Postfix Decrement"),
			TanukiExpressionVariant::PostfixSaturatingDecrement(..)                 => write!(f, "Postfix Saturating Decrement"),
			TanukiExpressionVariant::PostfixWrappingDecrement(..)                   => write!(f, "Postfix Wrapping Decrement"),
			TanukiExpressionVariant::TryPropagate(..)                               => write!(f, "Try Propagate"),
			TanukiExpressionVariant::Unwrap(..)                                     => write!(f, "Unwrap"),
			TanukiExpressionVariant::Read(..)                                       => write!(f, "Read"),
			TanukiExpressionVariant::Not(..)                                        => write!(f, "Not"),
			TanukiExpressionVariant::Reciprocal(..)                                 => write!(f, "Reciprocal"),
			TanukiExpressionVariant::BitshiftRightOne(..)                           => write!(f, "Bitshift Right One"),
			TanukiExpressionVariant::ComplexConjugate(..)                           => write!(f, "ComplexConjugate"),
			TanukiExpressionVariant::Signum(..)                                     => write!(f, "Signum"),
			TanukiExpressionVariant::Negation(..)                                   => write!(f, "Negation"),
			TanukiExpressionVariant::SaturatingNegation(..)                         => write!(f, "Saturating Negation"),
			TanukiExpressionVariant::WrappingNegation(..)                           => write!(f, "Wrapping Negation"),
			TanukiExpressionVariant::TryNegation(..)                                => write!(f, "Try Negation"),
			TanukiExpressionVariant::Square(..)                                     => write!(f, "Square"),
			TanukiExpressionVariant::SaturatingSquare(..)                           => write!(f, "Saturating Square"),
			TanukiExpressionVariant::WrappingSquare(..)                             => write!(f, "Wrapping Square"),
			TanukiExpressionVariant::TrySquare(..)                                  => write!(f, "TrySquare"),
			TanukiExpressionVariant::BitshiftLeftOne(..)                            => write!(f, "Bitshift Left One"),
			TanukiExpressionVariant::SaturatingBitshiftLeftOne(..)                  => write!(f, "Saturating Bitshift Left One"),
			TanukiExpressionVariant::WrappingBitshiftLeftOne(..)                    => write!(f, "Wrapping Bitshift Left One"),
			TanukiExpressionVariant::TryBitshiftLeftOne(..)                         => write!(f, "Try Bitshift Left One"),
			TanukiExpressionVariant::PrefixIncrement(..)                            => write!(f, "Prefix Increment"),
			TanukiExpressionVariant::PrefixSaturatingIncrement(..)                  => write!(f, "Prefix Saturating Increment"),
			TanukiExpressionVariant::PrefixWrappingIncrement(..)                    => write!(f, "Prefix Wrapping Increment"),
			TanukiExpressionVariant::PrefixDecrement(..)                            => write!(f, "Prefix Decrement"),
			TanukiExpressionVariant::PrefixSaturatingDecrement(..)                  => write!(f, "Prefix Saturating Decrement"),
			TanukiExpressionVariant::PrefixWrappingDecrement(..)                    => write!(f, "Prefix Wrapping Decrement"),
			TanukiExpressionVariant::AddressOf(..)                                  => write!(f, "Address of"),
			TanukiExpressionVariant::Dereference(..)                                => write!(f, "Dereference"),
			TanukiExpressionVariant::NthToLast(..)                                  => write!(f, "Nth to Last"),
			TanukiExpressionVariant::RangeToExclusive(..)                           => write!(f, "Range to Exclusive"),
			TanukiExpressionVariant::RangeToInclusive(..)                           => write!(f, "Range to Inclusive"),
			//TanukiExpressionVariant::RangeFrom(..)                  => write!(f, "Range From"),
			TanukiExpressionVariant::MemberAccess(..)                               => write!(f, "Member Access"),
			TanukiExpressionVariant::As(..)                                         => write!(f, "As"),
			TanukiExpressionVariant::SaturatingAs(..)                               => write!(f, "Saturating As"),
			TanukiExpressionVariant::WrappingAs(..)                                 => write!(f, "Wrapping As"),
			TanukiExpressionVariant::TryAs(..)                                      => write!(f, "Try As"),
			TanukiExpressionVariant::Exponent(..)                                   => write!(f, "Exponent"),
			TanukiExpressionVariant::SaturatingExponent(..)                         => write!(f, "Saturating Exponent"),
			TanukiExpressionVariant::WrappingExponent(..)                           => write!(f, "Wrapping Exponent"),
			TanukiExpressionVariant::TryExponent(..)                                => write!(f, "Try Exponent"),
			TanukiExpressionVariant::Multiplication(..)                             => write!(f, "Multiplication"),
			TanukiExpressionVariant::SaturatingMultiplication(..)                   => write!(f, "Saturating Multiplication"),
			TanukiExpressionVariant::WrappingMultiplication(..)                     => write!(f, "Wrapping Multiplication"),
			TanukiExpressionVariant::TryMultiplication(..)                          => write!(f, "Try Multiplication"),
			TanukiExpressionVariant::Division(..)                                   => write!(f, "Division"),
			TanukiExpressionVariant::SaturatingDivision(..)                         => write!(f, "Saturating Division"),
			TanukiExpressionVariant::WrappingDivision(..)                           => write!(f, "Wrapping Division"),
			TanukiExpressionVariant::TryDivision(..)                                => write!(f, "Try Division"),
			TanukiExpressionVariant::Modulo(..)                                     => write!(f, "Modulo"),
			TanukiExpressionVariant::SaturatingModulo(..)                           => write!(f, "Saturating Modulo"),
			TanukiExpressionVariant::WrappingModulo(..)                             => write!(f, "Wrapping Modulo"),
			TanukiExpressionVariant::TryModulo(..)                                  => write!(f, "Try Modulo"),
			TanukiExpressionVariant::Addition(..)                                   => write!(f, "Addition"),
			TanukiExpressionVariant::SaturatingAddition(..)                         => write!(f, "Saturating Addition"),
			TanukiExpressionVariant::WrappingAddition(..)                           => write!(f, "Wrapping Addition"),
			TanukiExpressionVariant::TryAddition(..)                                => write!(f, "Try Addition"),
			TanukiExpressionVariant::Subtraction(..)                                => write!(f, "Subtraction"),
			TanukiExpressionVariant::SaturatingSubtraction(..)                      => write!(f, "Saturating Subtraction"),
			TanukiExpressionVariant::WrappingSubtraction(..)                        => write!(f, "Wrapping Subtraction"),
			TanukiExpressionVariant::TrySubtraction(..)                             => write!(f, "Try Subtraction"),
			TanukiExpressionVariant::Concatenate(..)                                => write!(f, "Concatenate"),
			TanukiExpressionVariant::Append(..)                                     => write!(f, "Append"),
			TanukiExpressionVariant::BitshiftLeft(..)                               => write!(f, "Bitshift Left"),
			TanukiExpressionVariant::SaturatingBitshiftLeft(..)                     => write!(f, "Saturating Bitshift Left"),
			TanukiExpressionVariant::WrappingBitshiftLeft(..)                       => write!(f, "Wrapping Bitshift Left"),
			TanukiExpressionVariant::TryBitshiftLeft(..)                            => write!(f, "Try Bitshift Left"),
			TanukiExpressionVariant::BitshiftRight(..)                              => write!(f, "Bitshift Right"),
			TanukiExpressionVariant::ThreeWayCompare(..)                            => write!(f, "Three Way Compare"),
			TanukiExpressionVariant::LessThan(..)                                   => write!(f, "Less Than"),
			TanukiExpressionVariant::LessThanOrEqualTo(..)                          => write!(f, "Less Than or Equal to"),
			TanukiExpressionVariant::GreaterThan(..)                                => write!(f, "Greater Than"),
			TanukiExpressionVariant::GreaterThanOrEqualTo(..)                       => write!(f, "Greater Than or Equal to"),
			TanukiExpressionVariant::Equality(..)                                   => write!(f, "Equality"),
			TanukiExpressionVariant::Inequality(..)                                 => write!(f, "Inequality"),
			TanukiExpressionVariant::ReferenceEquality(..)                          => write!(f, "Reference Equality"),
			TanukiExpressionVariant::ReferenceInequality(..)                        => write!(f, "Reference Inequality"),
			TanukiExpressionVariant::NonShortCircuitAnd(..)                         => write!(f, "Non Short Circuit And"),
			TanukiExpressionVariant::NonShortCircuitNand(..)                        => write!(f, "Non Short Circuit Nand"),
			TanukiExpressionVariant::NonShortCircuitXor(..)                         => write!(f, "Non Short Circuit Xor"),
			TanukiExpressionVariant::NonShortCircuitXnor(..)                        => write!(f, "Non Short Circuit Xnor"),
			TanukiExpressionVariant::NonShortCircuitOr(..)                          => write!(f, "Non Short Circuit Or"),
			TanukiExpressionVariant::NonShortCircuitNor(..)                         => write!(f, "Non Short Circuit Nor"),
			TanukiExpressionVariant::Minimum(..)                                    => write!(f, "Minimum"),
			TanukiExpressionVariant::Maximum(..)                                    => write!(f, "Maximum"),
			TanukiExpressionVariant::Pipe(..)                                       => write!(f, "Pipe"),
			TanukiExpressionVariant::ShortCircuitAnd(..)                            => write!(f, "Short Circuit And"),
			TanukiExpressionVariant::ShortCircuitNand(..)                           => write!(f, "Short Circuit Nand"),
			TanukiExpressionVariant::ShortCircuitXor(..)                            => write!(f, "Short Circuit Xor"),
			TanukiExpressionVariant::ShortCircuitXnor(..)                           => write!(f, "Short Circuit Xnor"),
			TanukiExpressionVariant::ShortCircuitOr(..)                             => write!(f, "Short Circuit Or"),
			TanukiExpressionVariant::ShortCircuitNor(..)                            => write!(f, "Short Circuit Nor"),
			TanukiExpressionVariant::NonShortCircuitingNullCoalescing(..)           => write!(f, "Non Short Circuiting Null Coalescing"),
			TanukiExpressionVariant::ShortCircuitingNullCoalescing(..)              => write!(f, "Short Circuiting Null Coalescing"),
			TanukiExpressionVariant::ExclusiveRange(..)                             => write!(f, "Exclusive Range"),
			TanukiExpressionVariant::InclusiveRange(..)                             => write!(f, "Inclusive Range"),
			TanukiExpressionVariant::NonShortCircuitingConditional(..)              => write!(f, "Non Short Circuiting Conditional"),
			TanukiExpressionVariant::ShortCircuitingConditional(..)                 => write!(f, "Short Circuiting Conditional"),
			TanukiExpressionVariant::Assignment(..)                                 => write!(f, "Assignment"),
			TanukiExpressionVariant::ExponentAssignment(..)                         => write!(f, "Exponent Assignment"),
			TanukiExpressionVariant::SaturatingExponentAssignment(..)               => write!(f, "Saturating Exponent Assignment"),
			TanukiExpressionVariant::WrappingExponentAssignment(..)                 => write!(f, "Wrapping Exponent Assignment"),
			TanukiExpressionVariant::MultiplicationAssignment(..)                   => write!(f, "Multiplication Assignment"),
			TanukiExpressionVariant::SaturatingMultiplicationAssignment(..)         => write!(f, "Saturating Multiplication Assignment"),
			TanukiExpressionVariant::WrappingMultiplicationAssignment(..)           => write!(f, "Wrapping Multiplication Assignment"),
			TanukiExpressionVariant::DivisionAssignment(..)                         => write!(f, "Division Assignment"),
			TanukiExpressionVariant::SaturatingDivisionAssignment(..)               => write!(f, "Saturating Division Assignment"),
			TanukiExpressionVariant::WrappingDivisionAssignment(..)                 => write!(f, "Wrapping Division Assignment"),
			TanukiExpressionVariant::ModuloAssignment(..)                           => write!(f, "Modulo Assignment"),
			TanukiExpressionVariant::SaturatingModuloAssignment(..)                 => write!(f, "Saturating Modulo Assignment"),
			TanukiExpressionVariant::WrappingModuloAssignment(..)                   => write!(f, "Wrapping Modulo Assignment"),
			TanukiExpressionVariant::AdditionAssignment(..)                         => write!(f, "Addition Assignment"),
			TanukiExpressionVariant::SaturatingAdditionAssignment(..)               => write!(f, "Saturating Addition Assignment"),
			TanukiExpressionVariant::WrappingAdditionAssignment(..)                 => write!(f, "Wrapping Addition Assignment"),
			TanukiExpressionVariant::SubtractionAssignment(..)                      => write!(f, "Subtraction Assignment"),
			TanukiExpressionVariant::SaturatingSubtractionAssignment(..)            => write!(f, "Saturating Subtraction Assignment"),
			TanukiExpressionVariant::WrappingSubtractionAssignment(..)              => write!(f, "Wrapping Subtraction Assignment"),
			TanukiExpressionVariant::ConcatenateAssignment(..)                      => write!(f, "Concatenate Assignment"),
			TanukiExpressionVariant::AppendAssignment(..)                           => write!(f, "Append Assignment"),
			TanukiExpressionVariant::BitshiftLeftAssignment(..)                     => write!(f, "Bitshift Left Assignment"),
			TanukiExpressionVariant::SaturatingBitshiftLeftAssignment(..)           => write!(f, "Saturating Bitshift Left Assignment"),
			TanukiExpressionVariant::WrappingBitshiftLeftAssignment(..)             => write!(f, "Wrapping Bitshift Left Assignment"),
			TanukiExpressionVariant::BitshiftRightAssignment(..)                    => write!(f, "Bitshift Right Assignment"),
			TanukiExpressionVariant::ThreeWayCompareAssignment(..)                  => write!(f, "Three Way CompareAssignment"),
			TanukiExpressionVariant::NonShortCircuitAndAssignment(..)               => write!(f, "Non Short Circuit And Assignment"),
			TanukiExpressionVariant::NonShortCircuitNandAssignment(..)              => write!(f, "Non Short Circuit Nand Assignment"),
			TanukiExpressionVariant::NonShortCircuitXorAssignment(..)               => write!(f, "Non Short Circuit Xor Assignment"),
			TanukiExpressionVariant::NonShortCircuitXnorAssignment(..)              => write!(f, "Non Short Circuit Xnor Assignment"),
			TanukiExpressionVariant::NonShortCircuitOrAssignment(..)                => write!(f, "Non Short Circuit Or Assignment"),
			TanukiExpressionVariant::NonShortCircuitNorAssignment(..)               => write!(f, "Non Short Circuit Nor Assignment"),
			TanukiExpressionVariant::MinimumAssignment(..)                          => write!(f, "Minimum Assignment"),
			TanukiExpressionVariant::MaximumAssignment(..)                          => write!(f, "Maximum Assignment"),
			TanukiExpressionVariant::PipeAssignment(..)                             => write!(f, "Pipe Assignment"),
			TanukiExpressionVariant::ShortCircuitAndAssignment(..)                  => write!(f, "Short Circuit And Assignment"),
			TanukiExpressionVariant::ShortCircuitNandAssignment(..)                 => write!(f, "Short Circuit Nand Assignment"),
			TanukiExpressionVariant::ShortCircuitXorAssignment(..)                  => write!(f, "Short Circuit Xor Assignment"),
			TanukiExpressionVariant::ShortCircuitXnorAssignment(..)                 => write!(f, "Short Circuit Xnor Assignment"),
			TanukiExpressionVariant::ShortCircuitOrAssignment(..)                   => write!(f, "Short Circuit Or Assignment"),
			TanukiExpressionVariant::ShortCircuitNorAssignment(..)                  => write!(f, "Short Circuit Nor Assignment"),
			TanukiExpressionVariant::NonShortCircuitingNullCoalescingAssignment(..) => write!(f, "Non Short Circuiting Null Coalescing Assignment"),
			TanukiExpressionVariant::ShortCircuitingNullCoalescingAssignment(..)    => write!(f, "Short Circuiting Null Coalescing Assignment"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(..) | TanukiExpressionVariant::Variable(..) => Ok(()),
			TanukiExpressionVariant::Block { sub_expressions, ..} => {
				for sub_expression in sub_expressions {
					sub_expression.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
				function_pointer.print(level, f)?;
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression } => {
				for parameter in parameters {
					parameter.print(level, f)?;
				}
				if let Some(return_type) = return_type {
					return_type.print(level, f)?;
				}
				body_expression.print(level, f)
			}
			TanukiExpressionVariant::Import(arguments) | TanukiExpressionVariant::Link(arguments) => {
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::Export(argument) => argument.print(level, f),
			TanukiExpressionVariant::Percent(sub_expression) |
			TanukiExpressionVariant::Factorial(sub_expression) |
			TanukiExpressionVariant::SaturatingFactorial(sub_expression) |
			TanukiExpressionVariant::WrappingFactorial(sub_expression) |
			TanukiExpressionVariant::TryFactorial(sub_expression) |
			TanukiExpressionVariant::PostfixIncrement(sub_expression) |
			TanukiExpressionVariant::PostfixSaturatingIncrement(sub_expression) |
			TanukiExpressionVariant::PostfixWrappingIncrement(sub_expression) |
			TanukiExpressionVariant::PostfixDecrement(sub_expression) |
			TanukiExpressionVariant::PostfixSaturatingDecrement(sub_expression) |
			TanukiExpressionVariant::PostfixWrappingDecrement(sub_expression) |
			TanukiExpressionVariant::TryPropagate(sub_expression) |
			TanukiExpressionVariant::Unwrap(sub_expression) |
			TanukiExpressionVariant::Read(sub_expression) |
			TanukiExpressionVariant::Not(sub_expression) |
			TanukiExpressionVariant::Reciprocal(sub_expression) |
			TanukiExpressionVariant::BitshiftRightOne(sub_expression) |
			TanukiExpressionVariant::ComplexConjugate(sub_expression) |
			TanukiExpressionVariant::Signum(sub_expression) |
			TanukiExpressionVariant::Negation(sub_expression) |
			TanukiExpressionVariant::SaturatingNegation(sub_expression) |
			TanukiExpressionVariant::WrappingNegation(sub_expression) |
			TanukiExpressionVariant::TryNegation(sub_expression) |
			TanukiExpressionVariant::Square(sub_expression) |
			TanukiExpressionVariant::SaturatingSquare(sub_expression) |
			TanukiExpressionVariant::WrappingSquare(sub_expression) |
			TanukiExpressionVariant::TrySquare(sub_expression) |
			TanukiExpressionVariant::BitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::SaturatingBitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::WrappingBitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::TryBitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::PrefixIncrement(sub_expression) |
			TanukiExpressionVariant::PrefixSaturatingIncrement(sub_expression) |
			TanukiExpressionVariant::PrefixWrappingIncrement(sub_expression) |
			TanukiExpressionVariant::PrefixDecrement(sub_expression) |
			TanukiExpressionVariant::PrefixSaturatingDecrement(sub_expression) |
			TanukiExpressionVariant::PrefixWrappingDecrement(sub_expression) |
			TanukiExpressionVariant::AddressOf(sub_expression) |
			TanukiExpressionVariant::Dereference(sub_expression) |
			TanukiExpressionVariant::NthToLast(sub_expression) |
			TanukiExpressionVariant::RangeToExclusive(sub_expression) |
			TanukiExpressionVariant::RangeToInclusive(sub_expression) => sub_expression.print(level, f),
			TanukiExpressionVariant::Index(lhs, rhs) |
			TanukiExpressionVariant::TypeAndValue(lhs, rhs) |
			TanukiExpressionVariant::MemberAccess(lhs, rhs) |
			TanukiExpressionVariant::As(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAs(lhs, rhs) |
			TanukiExpressionVariant::WrappingAs(lhs, rhs) |
			TanukiExpressionVariant::TryAs(lhs, rhs) |
			TanukiExpressionVariant::Exponent(lhs, rhs) |
			TanukiExpressionVariant::SaturatingExponent(lhs, rhs) |
			TanukiExpressionVariant::WrappingExponent(lhs, rhs) |
			TanukiExpressionVariant::TryExponent(lhs, rhs) |
			TanukiExpressionVariant::Multiplication(lhs, rhs) |
			TanukiExpressionVariant::SaturatingMultiplication(lhs, rhs) |
			TanukiExpressionVariant::WrappingMultiplication(lhs, rhs) |
			TanukiExpressionVariant::TryMultiplication(lhs, rhs) |
			TanukiExpressionVariant::Division(lhs, rhs) |
			TanukiExpressionVariant::SaturatingDivision(lhs, rhs) |
			TanukiExpressionVariant::WrappingDivision(lhs, rhs) |
			TanukiExpressionVariant::TryDivision(lhs, rhs) |
			TanukiExpressionVariant::Modulo(lhs, rhs) |
			TanukiExpressionVariant::SaturatingModulo(lhs, rhs) |
			TanukiExpressionVariant::WrappingModulo(lhs, rhs) |
			TanukiExpressionVariant::TryModulo(lhs, rhs) |
			TanukiExpressionVariant::Addition(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAddition(lhs, rhs) |
			TanukiExpressionVariant::WrappingAddition(lhs, rhs) |
			TanukiExpressionVariant::TryAddition(lhs, rhs) |
			TanukiExpressionVariant::Subtraction(lhs, rhs) |
			TanukiExpressionVariant::SaturatingSubtraction(lhs, rhs) |
			TanukiExpressionVariant::WrappingSubtraction(lhs, rhs) |
			TanukiExpressionVariant::TrySubtraction(lhs, rhs) |
			TanukiExpressionVariant::Concatenate(lhs, rhs) |
			TanukiExpressionVariant::Append(lhs, rhs) |
			TanukiExpressionVariant::BitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::SaturatingBitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::WrappingBitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::TryBitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::BitshiftRight(lhs, rhs) |
			TanukiExpressionVariant::ThreeWayCompare(lhs, rhs) |
			TanukiExpressionVariant::LessThan(lhs, rhs) |
			TanukiExpressionVariant::LessThanOrEqualTo(lhs, rhs) |
			TanukiExpressionVariant::GreaterThan(lhs, rhs) |
			TanukiExpressionVariant::GreaterThanOrEqualTo(lhs, rhs) |
			TanukiExpressionVariant::Equality(lhs, rhs) |
			TanukiExpressionVariant::Inequality(lhs, rhs) |
			TanukiExpressionVariant::ReferenceEquality(lhs, rhs) |
			TanukiExpressionVariant::ReferenceInequality(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitAnd(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNand(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXor(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXnor(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitOr(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNor(lhs, rhs) |
			TanukiExpressionVariant::Minimum(lhs, rhs) |
			TanukiExpressionVariant::Maximum(lhs, rhs) |
			TanukiExpressionVariant::Pipe(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitAnd(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNand(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXor(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXnor(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitOr(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNor(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitingNullCoalescing(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitingNullCoalescing(lhs, rhs) |
			TanukiExpressionVariant::ExclusiveRange(lhs, rhs) |
			TanukiExpressionVariant::InclusiveRange(lhs, rhs) |
			TanukiExpressionVariant::Assignment(lhs, rhs) |
			TanukiExpressionVariant::ExponentAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingExponentAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingExponentAssignment(lhs, rhs) |
			TanukiExpressionVariant::MultiplicationAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingMultiplicationAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingMultiplicationAssignment(lhs, rhs) |
			TanukiExpressionVariant::DivisionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingDivisionAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingDivisionAssignment(lhs, rhs) |
			TanukiExpressionVariant::ModuloAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingModuloAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingModuloAssignment(lhs, rhs) |
			TanukiExpressionVariant::AdditionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAdditionAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingAdditionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SubtractionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingSubtractionAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingSubtractionAssignment(lhs, rhs) |
			TanukiExpressionVariant::ConcatenateAssignment(lhs, rhs) |
			TanukiExpressionVariant::AppendAssignment(lhs, rhs) |
			TanukiExpressionVariant::BitshiftLeftAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingBitshiftLeftAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingBitshiftLeftAssignment(lhs, rhs) |
			TanukiExpressionVariant::BitshiftRightAssignment(lhs, rhs) |
			TanukiExpressionVariant::ThreeWayCompareAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitAndAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNandAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXorAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXnorAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitOrAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNorAssignment(lhs, rhs) |
			TanukiExpressionVariant::MinimumAssignment(lhs, rhs) |
			TanukiExpressionVariant::MaximumAssignment(lhs, rhs) |
			TanukiExpressionVariant::PipeAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitAndAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNandAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXorAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXnorAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitOrAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNorAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitingNullCoalescingAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitingNullCoalescingAssignment(lhs, rhs) => {
				lhs.print(level, f)?;
				rhs.print(level, f)
			}
			TanukiExpressionVariant::NonShortCircuitingConditional(lhs, mhs, rhs) |
			TanukiExpressionVariant::ShortCircuitingConditional(lhs, mhs, rhs) => {
				lhs.print(level, f)?;
				mhs.print(level, f)?;
				rhs.print(level, f)
			}
		}
	}

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
}