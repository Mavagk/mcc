use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, maybe_parsed_token::MaybeParsedToken, programming_languages::tanuki::{constant_value::TanukiConstantValue, parse::{TanukiPartiallyParsedToken, TanukiPartiallyParsedTokenVariant}, token::{InfixBinaryOperator, PostfixUnaryOperator, PrefixUnaryOperator, TanukiToken, TanukiTokenVariant}}, token_reader::TokenReader, traits::{ast_node::AstNode, expression::Expression}};

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
	FunctionDefinition { parameters: Box<[TanukiExpression]>, body_expression: Box<TanukiExpression> },
	Index(Box<TanukiExpression>, Box<TanukiExpression>),
	TypeAndValue(Box<TanukiExpression>, Box<TanukiExpression>),
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
}

impl TanukiExpression {
	pub fn parse(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Option<Self>, ErrorAt> {
		if token_reader.peek().is_none() {
			return Ok(None);
		}
		//let expression_start_line = token_reader.peek().unwrap().start_line;
		//let expression_start_column = token_reader.peek().unwrap().start_column;
		let mut maybe_parsed_tokens = Vec::new();
		// Loop through all tokens until we reach the end of the expression
		while matches!(token_reader.peek().map(|token| &token.variant), Some(..)) {
			// If we reach a separator that is'int an opening separator, break
			let token = &token_reader.peek().unwrap().variant;
			if matches!(token, TanukiTokenVariant::RightParenthesis | TanukiTokenVariant::RightCurlyParenthesis | TanukiTokenVariant::RightSquareParenthesis | TanukiTokenVariant::Comma | TanukiTokenVariant::Semicolon) {
				break;
			}
			// First parse round
			let token = token_reader.next().unwrap().clone();
			let token_start_line = token.start_line;
			let token_start_column = token.start_column;
			let token_end_line = token.end_line;
			let token_end_column = token.end_column;
			maybe_parsed_tokens.push(match &token.variant {
				TanukiTokenVariant::NumericLiteral(None, Some(float_value)) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiConstantValue::Float(*float_value)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::NumericLiteral(Some(int_value), _) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiConstantValue::Integer(int_value.clone().into())),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::NumericLiteral(None, None) => unreachable!(),
				TanukiTokenVariant::CharacterLiteral(value) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiConstantValue::Character(*value)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::StringLiteral(value) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiConstantValue::String(value.clone())),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Identifier(name) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Variable(name.clone()),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				// If there is a block
				TanukiTokenVariant::LeftCurlyParenthesis => 'a: {
					// Parse each sub-expression
					let mut sub_expressions = Vec::new();
					loop {
						// Parse expression
						let expression_is_empty;
						if let Some(sub_expression) = Self::parse(main, token_reader)? {
							sub_expressions.push(sub_expression);
							expression_is_empty = false;
						}
						else {
							expression_is_empty = true;
						}
						// Next token should be a } or ; token
						match token_reader.next() {
							// Right curly bracket ends the block expression
							Some(TanukiToken { variant: TanukiTokenVariant::RightCurlyParenthesis, end_line, end_column, .. }) => break 'a MaybeParsedToken::Parsed(TanukiExpression {
								variant: TanukiExpressionVariant::Block { sub_expressions: sub_expressions.into(), has_return_value: !expression_is_empty },
								start_line: token_start_line, start_column: token_start_column, end_line: *end_line, end_column: *end_column,
							}),
							// The token stream should not just stop
							None => return Err(Error::ExpectedCurlyClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Move on to the next sub-expression if we read a semicolon
							Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, .. }) => {},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedSemicolon.at(Some(*start_column), Some(*end_column), None)),
						}
					}
				},
				// Function arguments or parameters
				// If there is a block
				TanukiTokenVariant::LeftParenthesis => 'a: {
					// Parse each sub-expression
					let mut sub_expressions = Vec::new();
					loop {
						// Parse expression
						let expression_is_empty;
						if let Some(sub_expression) = Self::parse(main, token_reader)? {
							sub_expressions.push(sub_expression);
							expression_is_empty = false;
						}
						else {
							expression_is_empty = true;
						}
						// Next token should be a } or ; token
						match token_reader.next() {
							// Right curly bracket ends the block expression
							Some(TanukiToken { variant: TanukiTokenVariant::RightParenthesis, end_line, end_column, .. })
								=> break 'a MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken
							{
								variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(sub_expressions.into()),
								start_line: token_start_line, start_column: token_start_column, end_line: *end_line, end_column: *end_column,
							}),
							// The token stream should not just stop
							None => return Err(Error::ExpectedCurlyClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Move on to the next sub-expression if we read a semicolon
							Some(TanukiToken { variant: TanukiTokenVariant::Comma, start_line, start_column, .. }) => {
								if expression_is_empty {
									return Err(Error::ExpectedExpression.at(Some(*start_line), Some(*start_column), None));
								}
							},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedComma.at(Some(*start_column), Some(*end_column), None)),
						}
					}
				},
				// Square parentheses
				TanukiTokenVariant::LeftSquareParenthesis => {
					// Parse expression
					let sub_expression = Self::parse_expected(main, token_reader)?;
					// Take closing square parenthesis
					match token_reader.next() {
						Some(TanukiToken { variant: TanukiTokenVariant::RightSquareParenthesis, .. }) => {},
						Some(TanukiToken { start_line, start_column, .. })
							=> return Err(Error::ExpectedSquareClosingParenthesis.at(Some(*start_line), Some(*start_column), None)),
						None => return Err(Error::ExpectedSquareClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
					}
					// Assemble into value
					MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						start_line: token_start_line, start_column: token_end_column, end_line: token_reader.last_token_end_line(), end_column: token_reader.last_token_end_line(),
						variant: TanukiPartiallyParsedTokenVariant::SquareParenthesised(Box::new(sub_expression)),
					})
				},
				_ => MaybeParsedToken::Unparsed(token),
			});
		}
		if maybe_parsed_tokens.is_empty() {
			return Ok(None);
		}
		Ok(Some(Self::parse_maybe_parsed_tokens(main, maybe_parsed_tokens)?))
	}

	pub fn parse_maybe_parsed_tokens(_main: &mut Main, mut maybe_parsed_tokens: Vec<MaybeParsedToken<TanukiExpression, TanukiPartiallyParsedToken, TanukiToken>>) -> Result<TanukiExpression, ErrorAt> {
		// Parse postfix operators
		let mut x = 0;
		while x < maybe_parsed_tokens.len() - 1 {
			// Skip if this is not in the order (parsed expression, operator, non-parsed_expression) or (parsed expression function arguments)
			if !maybe_parsed_tokens[x].is_parsed() ||
				(
					(!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { postfix_unary_operator: Some(..), is_assignment: false, .. }, .. })) ||
					matches!(maybe_parsed_tokens.get(x + 2), Some(token) if token.is_parsed()))
				) && !matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
					variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..) | TanukiPartiallyParsedTokenVariant::SquareParenthesised(..), ..
				}))
			{
				x += 1;
				continue;
			}
			// Parse
			let operand = maybe_parsed_tokens[x].clone().unwrap_parsed();
			maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match maybe_parsed_tokens.remove(x + 1) {
				MaybeParsedToken::Unparsed(TanukiToken {
					variant: TanukiTokenVariant::Operator { postfix_unary_operator, symbol, .. }, start_line, start_column, end_line, end_column
				}) => TanukiExpression { start_line: operand.start_line, start_column: operand.start_column, variant: match postfix_unary_operator {
					Some(PostfixUnaryOperator::Percent) => TanukiExpressionVariant::Percent(Box::new(operand)),
					Some(PostfixUnaryOperator::Factorial) => TanukiExpressionVariant::Factorial(Box::new(operand)),
					Some(PostfixUnaryOperator::SaturatingFactorial) => TanukiExpressionVariant::SaturatingFactorial(Box::new(operand)),
					Some(PostfixUnaryOperator::WrappingFactorial) => TanukiExpressionVariant::WrappingFactorial(Box::new(operand)),
					Some(PostfixUnaryOperator::TryFactorial) => TanukiExpressionVariant::TryFactorial(Box::new(operand)),
					Some(PostfixUnaryOperator::Increment) => TanukiExpressionVariant::PostfixIncrement(Box::new(operand)),
					Some(PostfixUnaryOperator::SaturatingIncrement) => TanukiExpressionVariant::PostfixSaturatingIncrement(Box::new(operand)),
					Some(PostfixUnaryOperator::WrappingIncrement) => TanukiExpressionVariant::PostfixWrappingIncrement(Box::new(operand)),
					Some(PostfixUnaryOperator::Decrement) => TanukiExpressionVariant::PostfixDecrement(Box::new(operand)),
					Some(PostfixUnaryOperator::SaturatingDecrement) => TanukiExpressionVariant::PostfixSaturatingDecrement(Box::new(operand)),
					Some(PostfixUnaryOperator::WrappingDecrement) => TanukiExpressionVariant::PostfixWrappingDecrement(Box::new(operand)),
					Some(PostfixUnaryOperator::TryPropagate) => TanukiExpressionVariant::TryPropagate(Box::new(operand)),
					Some(PostfixUnaryOperator::Unwrap) => TanukiExpressionVariant::Unwrap(Box::new(operand)),
					//Some(PostfixUnaryOperator::RangeFrom) => TanukiExpressionVariant::RangeFrom(Box::new(operand)),
					None => return Err(Error::InvalidPostfixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
				}, end_line, end_column },
				MaybeParsedToken::Unparsed(TanukiToken {
					variant: _, ..
				}) => unreachable!(),
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
					variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(arguments), end_line, end_column, ..
				}) => TanukiExpression {
					start_line: operand.start_line, start_column: operand.start_column, variant: TanukiExpressionVariant::FunctionCall { function_pointer: Box::new(operand), arguments },
					end_line, end_column,
				},
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
					variant: TanukiPartiallyParsedTokenVariant::SquareParenthesised(index), end_line, end_column, ..
				}) => TanukiExpression {
					start_line: operand.start_line, start_column: operand.start_column, variant: TanukiExpressionVariant::Index(Box::new(operand), index),
					end_line, end_column,
				},
				MaybeParsedToken::Parsed(..) => unreachable!(),
			});
		}
		// Parse prefix operators
		let mut x = maybe_parsed_tokens.len().saturating_sub(2);
		loop {
			// Skip if this is not in the order parsed expression, operator, non-parsed expression
			if (
				!matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_assignment: false, .. }, .. })) ||
				((!maybe_parsed_tokens.get(x + 1).is_some_and(|token| token.is_parsed()) ||
				(x > 0 && !maybe_parsed_tokens[x - 1].is_unparsed()) || x >= maybe_parsed_tokens.len() - 1))
			) && (x >= maybe_parsed_tokens.len() - 1 || (!maybe_parsed_tokens[x].is_parsed() || !maybe_parsed_tokens[x + 1].is_parsed()))
			{
				x = match x.checked_sub(1) {
					Some(x) => x,
					None => break,
				};
				continue;
			}
			// Parse
			let operand = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
			maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match maybe_parsed_tokens[x].clone() {
				MaybeParsedToken::Unparsed(TanukiToken {
					variant: TanukiTokenVariant::Operator { prefix_unary_operator, symbol, .. }, start_line, start_column, ..
				}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, variant: match prefix_unary_operator {
					Some(PrefixUnaryOperator::Read) => TanukiExpressionVariant::Read(Box::new(operand)),
					Some(PrefixUnaryOperator::Not) => TanukiExpressionVariant::Not(Box::new(operand)),
					Some(PrefixUnaryOperator::Reciprocal) => TanukiExpressionVariant::Reciprocal(Box::new(operand)),
					Some(PrefixUnaryOperator::BitshiftRightOne) => TanukiExpressionVariant::BitshiftRightOne(Box::new(operand)),
					Some(PrefixUnaryOperator::ComplexConjugate) => TanukiExpressionVariant::ComplexConjugate(Box::new(operand)),
					Some(PrefixUnaryOperator::Signum) => TanukiExpressionVariant::Signum(Box::new(operand)),
					Some(PrefixUnaryOperator::Negation) => TanukiExpressionVariant::Negation(Box::new(operand)),
					Some(PrefixUnaryOperator::SaturatingNegation) => TanukiExpressionVariant::SaturatingNegation(Box::new(operand)),
					Some(PrefixUnaryOperator::WrappingNegation) => TanukiExpressionVariant::WrappingNegation(Box::new(operand)),
					Some(PrefixUnaryOperator::TryNegation) => TanukiExpressionVariant::TryNegation(Box::new(operand)),
					Some(PrefixUnaryOperator::Square) => TanukiExpressionVariant::Square(Box::new(operand)),
					Some(PrefixUnaryOperator::SaturatingSquare) => TanukiExpressionVariant::SaturatingSquare(Box::new(operand)),
					Some(PrefixUnaryOperator::WrappingSquare) => TanukiExpressionVariant::WrappingSquare(Box::new(operand)),
					Some(PrefixUnaryOperator::TrySquare) => TanukiExpressionVariant::TrySquare(Box::new(operand)),
					Some(PrefixUnaryOperator::BitshiftLeftOne) => TanukiExpressionVariant::BitshiftLeftOne(Box::new(operand)),
					Some(PrefixUnaryOperator::SaturatingBitshiftLeftOne) => TanukiExpressionVariant::SaturatingBitshiftLeftOne(Box::new(operand)),
					Some(PrefixUnaryOperator::WrappingBitshiftLeftOne) => TanukiExpressionVariant::WrappingBitshiftLeftOne(Box::new(operand)),
					Some(PrefixUnaryOperator::TryBitshiftLeftOne) => TanukiExpressionVariant::TryBitshiftLeftOne(Box::new(operand)),
					Some(PrefixUnaryOperator::Increment) => TanukiExpressionVariant::PrefixIncrement(Box::new(operand)),
					Some(PrefixUnaryOperator::SaturatingIncrement) => TanukiExpressionVariant::PrefixSaturatingIncrement(Box::new(operand)),
					Some(PrefixUnaryOperator::WrappingIncrement) => TanukiExpressionVariant::PrefixWrappingIncrement(Box::new(operand)),
					Some(PrefixUnaryOperator::Decrement) => TanukiExpressionVariant::PrefixDecrement(Box::new(operand)),
					Some(PrefixUnaryOperator::SaturatingDecrement) => TanukiExpressionVariant::PrefixSaturatingDecrement(Box::new(operand)),
					Some(PrefixUnaryOperator::WrappingDecrement) => TanukiExpressionVariant::PrefixWrappingDecrement(Box::new(operand)),
					Some(PrefixUnaryOperator::AddressOf) => TanukiExpressionVariant::AddressOf(Box::new(operand)),
					Some(PrefixUnaryOperator::Dereference) => TanukiExpressionVariant::Dereference(Box::new(operand)),
					Some(PrefixUnaryOperator::NthToLast) => TanukiExpressionVariant::NthToLast(Box::new(operand)),
					Some(PrefixUnaryOperator::RangeToExclusive) => TanukiExpressionVariant::RangeToExclusive(Box::new(operand)),
					Some(PrefixUnaryOperator::RangeToInclusive) => TanukiExpressionVariant::RangeToInclusive(Box::new(operand)),
					None => return Err(Error::InvalidPrefixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
				}, start_line, start_column },
				MaybeParsedToken::Unparsed(TanukiToken {
					variant: _, ..
				}) => unreachable!(),
				MaybeParsedToken::PartiallyParsed(..) => unreachable!(),
				MaybeParsedToken::Parsed(type_expression) => TanukiExpression {
					end_line: operand.end_line, end_column: operand.end_column, start_line: operand.start_line, start_column: operand.start_column,
					variant: TanukiExpressionVariant::TypeAndValue(Box::new(type_expression), Box::new(operand)),
				},
			});
		}
		// Parse infix binary operators
		for precedence_level in InfixBinaryOperator::PRECEDENCE_LEVELS {
			let mut x = 0;
			while x < maybe_parsed_tokens.len().saturating_sub(2) {
				// Skip if this is not in the order parsed expression, operator, parsed expression
				if !maybe_parsed_tokens[x].is_parsed() ||
					!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_assignment: false, .. }, .. })) ||
					!maybe_parsed_tokens[x + 2].is_parsed()
				{
					x += 1;
					continue;
				}
				// Skip if the operator should not be parsed for this precedence level
				match &maybe_parsed_tokens[x + 1] {
					MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { infix_binary_operator, symbol, .. }, start_line, start_column, .. }) => {
						if infix_binary_operator.is_none() {
							return Err(Error::InvalidInfixBinaryOperator(symbol.clone().into_string()).at(Some(*start_line), Some(*start_column), None));
						}
						if !precedence_level.contains(&infix_binary_operator.unwrap()) {
							x += 1;
							continue;
						}
					}
					_ => unreachable!(),
				}
				// Parse
				let lhs = maybe_parsed_tokens[x].clone().unwrap_parsed();
				let operator = maybe_parsed_tokens.remove(x + 1).unwrap_unparsed();
				let rhs = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match operator {
					TanukiToken {
						variant: TanukiTokenVariant::Operator { infix_binary_operator, symbol, .. }, start_line: operator_start_line, start_column: operator_start_column, ..
					} => TanukiExpression { start_line: lhs.start_line, start_column: lhs.start_column, end_line: rhs.end_line, end_column: rhs.end_column, variant: match infix_binary_operator {
						Some(InfixBinaryOperator::MemberAccess) => TanukiExpressionVariant::MemberAccess(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::As) => TanukiExpressionVariant::As(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingAs) => TanukiExpressionVariant::SaturatingAs(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingAs) => TanukiExpressionVariant::WrappingAs(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryAs) => TanukiExpressionVariant::TryAs(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Exponent) => TanukiExpressionVariant::Exponent(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingExponent) => TanukiExpressionVariant::SaturatingExponent(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingExponent) => TanukiExpressionVariant::WrappingExponent(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryExponent) => TanukiExpressionVariant::TryExponent(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Multiplication) => TanukiExpressionVariant::Multiplication(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingMultiplication) => TanukiExpressionVariant::SaturatingMultiplication(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingMultiplication) => TanukiExpressionVariant::WrappingMultiplication(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryMultiplication) => TanukiExpressionVariant::TryMultiplication(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Division) => TanukiExpressionVariant::Division(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingDivision) => TanukiExpressionVariant::SaturatingDivision(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingDivision) => TanukiExpressionVariant::WrappingDivision(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryDivision) => TanukiExpressionVariant::TryDivision(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Modulo) => TanukiExpressionVariant::Modulo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingModulo) => TanukiExpressionVariant::SaturatingModulo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingModulo) => TanukiExpressionVariant::WrappingModulo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryModulo) => TanukiExpressionVariant::TryModulo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Addition) => TanukiExpressionVariant::Addition(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingAddition) => TanukiExpressionVariant::SaturatingAddition(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingAddition) => TanukiExpressionVariant::WrappingAddition(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryAddition) => TanukiExpressionVariant::TryAddition(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Subtraction) => TanukiExpressionVariant::Subtraction(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingSubtraction) => TanukiExpressionVariant::SaturatingSubtraction(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingSubtraction) => TanukiExpressionVariant::WrappingSubtraction(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TrySubtraction) => TanukiExpressionVariant::TrySubtraction(Box::new(lhs), Box::new(rhs)),
						//Some(InfixBinaryOperator::Concatenate) => TanukiExpressionVariant::Concatenate(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Append) => TanukiExpressionVariant::Append(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::BitshiftLeft) => TanukiExpressionVariant::BitshiftLeft(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::SaturatingBitshiftLeft) => TanukiExpressionVariant::SaturatingBitshiftLeft(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::WrappingBitshiftLeft) => TanukiExpressionVariant::WrappingBitshiftLeft(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::TryBitshiftLeft) => TanukiExpressionVariant::TryBitshiftLeft(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::BitshiftRight) => TanukiExpressionVariant::BitshiftRight(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ThreeWayCompare) => TanukiExpressionVariant::ThreeWayCompare(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::LessThan) => TanukiExpressionVariant::LessThan(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::LessThanOrEqualTo) => TanukiExpressionVariant::LessThanOrEqualTo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::GreaterThan) => TanukiExpressionVariant::GreaterThan(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::GreaterThanOrEqualTo) => TanukiExpressionVariant::GreaterThanOrEqualTo(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Equality) => TanukiExpressionVariant::Equality(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Inequality) => TanukiExpressionVariant::Inequality(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ReferenceEquality) => TanukiExpressionVariant::ReferenceEquality(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ReferenceInequality) => TanukiExpressionVariant::ReferenceInequality(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitAnd) => TanukiExpressionVariant::NonShortCircuitAnd(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitNand) => TanukiExpressionVariant::NonShortCircuitNand(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitXor) => TanukiExpressionVariant::NonShortCircuitXor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitXnor) => TanukiExpressionVariant::NonShortCircuitXnor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitOr) => TanukiExpressionVariant::NonShortCircuitOr(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitNor) => TanukiExpressionVariant::NonShortCircuitNor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Minimum) => TanukiExpressionVariant::Minimum(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Maximum) => TanukiExpressionVariant::Maximum(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::Pipe) => TanukiExpressionVariant::Pipe(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitAnd) => TanukiExpressionVariant::ShortCircuitAnd(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitNand) => TanukiExpressionVariant::ShortCircuitNand(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitXor) => TanukiExpressionVariant::ShortCircuitXor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitXnor) => TanukiExpressionVariant::ShortCircuitXnor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitOr) => TanukiExpressionVariant::ShortCircuitOr(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitNor) => TanukiExpressionVariant::ShortCircuitNor(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::NonShortCircuitingNullCoalescing) => TanukiExpressionVariant::NonShortCircuitingNullCoalescing(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ShortCircuitingNullCoalescing) => TanukiExpressionVariant::ShortCircuitingNullCoalescing(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::ExclusiveRange) => TanukiExpressionVariant::ExclusiveRange(Box::new(lhs), Box::new(rhs)),
						Some(InfixBinaryOperator::InclusiveRange) => TanukiExpressionVariant::InclusiveRange(Box::new(lhs), Box::new(rhs)),
						None => return Err(Error::InvalidInfixBinaryOperator(symbol.into_string()).at(Some(operator_start_line), Some(operator_start_column), None)),
						_ => todo!(),
					}},
					_ => unreachable!()
				});
			}
		}
		// TODO: Ternary conditional
		// Parse function definitions
		let mut x = maybe_parsed_tokens.len().saturating_sub(2);
		loop {
			// Skip if this is not in the order parsed expression, operator, non-parsed expression
			if x >= maybe_parsed_tokens.len().saturating_sub(1) ||
				!matches!(maybe_parsed_tokens[x], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..), .. })) ||
				!maybe_parsed_tokens[x + 1].is_parsed()
			{
				x = match x.checked_sub(1) {
					Some(x) => x,
					None => break,
				};
				continue;
			}
			// Parse
			let function_body_expression = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
			let function_parameters = maybe_parsed_tokens[x].clone().unwrap_partially_parsed();
			maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression {
				start_line: function_parameters.start_line, start_column: function_parameters.start_column, end_line: function_body_expression.end_line, end_column: function_body_expression.end_column,
				variant: TanukiExpressionVariant::FunctionDefinition { parameters: match function_parameters {
					TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(parameters), .. } => parameters,
					_ => unreachable!(),
				}, body_expression: Box::new(function_body_expression) }
			});
		}
		// There should only be one `MaybeParsedToken`, it should be parsed into an expression
		if maybe_parsed_tokens.len() == 1 && maybe_parsed_tokens[0].is_parsed() {
			return Ok(maybe_parsed_tokens.pop().unwrap().unwrap_parsed())
		}
		println!("{maybe_parsed_tokens:?}");
		Err(Error::NotYetImplemented("Parsing some expressions".into())
			.at(Some(match maybe_parsed_tokens.first().unwrap() {
				MaybeParsedToken::Parsed(TanukiExpression { start_line, .. }) => *start_line,
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { start_line, .. }) => *start_line,
				MaybeParsedToken::Unparsed(TanukiToken { start_line, .. }) => *start_line,
			}), Some(match maybe_parsed_tokens.first().unwrap() {
				MaybeParsedToken::Parsed(TanukiExpression { start_column, .. }) => *start_column,
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { start_column, .. }) => *start_column,
				MaybeParsedToken::Unparsed(TanukiToken { start_column, .. }) => *start_column,
			}), None)
		)
	}

	pub fn parse_expected(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		match Self::parse(main, token_reader)? {
			None => Err(Error::ExpectedExpression.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			Some(expression) => Ok(expression),
		}
	}
}

impl Expression for TanukiExpression {
	
}

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
			TanukiExpressionVariant::FunctionCall { .. }                  => write!(f, "Function Call"),
			TanukiExpressionVariant::FunctionDefinition { .. }            => write!(f, "Function Definition"),
			TanukiExpressionVariant::Index { .. }                         => write!(f, "Index"),
			TanukiExpressionVariant::Variable(name)            => write!(f, "Variable {name}"),
			TanukiExpressionVariant::TypeAndValue(..)                     => write!(f, "Type and Value"),
			TanukiExpressionVariant::Percent(..)                          => write!(f, "Percent"),
			TanukiExpressionVariant::Factorial(..)                        => write!(f, "Factorial"),
			TanukiExpressionVariant::SaturatingFactorial(..)              => write!(f, "Saturating Factorial"),
			TanukiExpressionVariant::WrappingFactorial(..)                => write!(f, "Wrapping Factorial"),
			TanukiExpressionVariant::TryFactorial(..)                     => write!(f, "Try Factorial"),
			TanukiExpressionVariant::PostfixIncrement(..)                 => write!(f, "Postfix Increment"),
			TanukiExpressionVariant::PostfixSaturatingIncrement(..)       => write!(f, "Postfix Saturating Increment"),
			TanukiExpressionVariant::PostfixWrappingIncrement(..)         => write!(f, "Postfix Wrapping Increment"),
			TanukiExpressionVariant::PostfixDecrement(..)                 => write!(f, "Postfix Decrement"),
			TanukiExpressionVariant::PostfixSaturatingDecrement(..)       => write!(f, "Postfix Saturating Decrement"),
			TanukiExpressionVariant::PostfixWrappingDecrement(..)         => write!(f, "Postfix Wrapping Decrement"),
			TanukiExpressionVariant::TryPropagate(..)                     => write!(f, "Try Propagate"),
			TanukiExpressionVariant::Unwrap(..)                           => write!(f, "Unwrap"),
			TanukiExpressionVariant::Read(..)                             => write!(f, "Read"),
			TanukiExpressionVariant::Not(..)                              => write!(f, "Not"),
			TanukiExpressionVariant::Reciprocal(..)                       => write!(f, "Reciprocal"),
			TanukiExpressionVariant::BitshiftRightOne(..)                 => write!(f, "Bitshift Right One"),
			TanukiExpressionVariant::ComplexConjugate(..)                 => write!(f, "ComplexConjugate"),
			TanukiExpressionVariant::Signum(..)                           => write!(f, "Signum"),
			TanukiExpressionVariant::Negation(..)                         => write!(f, "Negation"),
			TanukiExpressionVariant::SaturatingNegation(..)               => write!(f, "Saturating Negation"),
			TanukiExpressionVariant::WrappingNegation(..)                 => write!(f, "Wrapping Negation"),
			TanukiExpressionVariant::TryNegation(..)                      => write!(f, "Try Negation"),
			TanukiExpressionVariant::Square(..)                           => write!(f, "Square"),
			TanukiExpressionVariant::SaturatingSquare(..)                 => write!(f, "Saturating Square"),
			TanukiExpressionVariant::WrappingSquare(..)                   => write!(f, "Wrapping Square"),
			TanukiExpressionVariant::TrySquare(..)                        => write!(f, "TrySquare"),
			TanukiExpressionVariant::BitshiftLeftOne(..)                  => write!(f, "Bitshift Left One"),
			TanukiExpressionVariant::SaturatingBitshiftLeftOne(..)        => write!(f, "Saturating Bitshift Left One"),
			TanukiExpressionVariant::WrappingBitshiftLeftOne(..)          => write!(f, "Wrapping Bitshift Left One"),
			TanukiExpressionVariant::TryBitshiftLeftOne(..)               => write!(f, "Try Bitshift Left One"),
			TanukiExpressionVariant::PrefixIncrement(..)                  => write!(f, "Prefix Increment"),
			TanukiExpressionVariant::PrefixSaturatingIncrement(..)        => write!(f, "Prefix Saturating Increment"),
			TanukiExpressionVariant::PrefixWrappingIncrement(..)          => write!(f, "Prefix Wrapping Increment"),
			TanukiExpressionVariant::PrefixDecrement(..)                  => write!(f, "Prefix Decrement"),
			TanukiExpressionVariant::PrefixSaturatingDecrement(..)        => write!(f, "Prefix Saturating Decrement"),
			TanukiExpressionVariant::PrefixWrappingDecrement(..)          => write!(f, "Prefix Wrapping Decrement"),
			TanukiExpressionVariant::AddressOf(..)                        => write!(f, "Address of"),
			TanukiExpressionVariant::Dereference(..)                      => write!(f, "Dereference"),
			TanukiExpressionVariant::NthToLast(..)                        => write!(f, "Nth to Last"),
			TanukiExpressionVariant::RangeToExclusive(..)                 => write!(f, "Range to Exclusive"),
			TanukiExpressionVariant::RangeToInclusive(..)                 => write!(f, "Range to Inclusive"),
			//TanukiExpressionVariant::RangeFrom(..)                  => write!(f, "Range From"),
			TanukiExpressionVariant::MemberAccess(..)                     => write!(f, "Member Access"),
			TanukiExpressionVariant::As(..)                               => write!(f, "As"),
			TanukiExpressionVariant::SaturatingAs(..)                     => write!(f, "Saturating As"),
			TanukiExpressionVariant::WrappingAs(..)                       => write!(f, "Wrapping As"),
			TanukiExpressionVariant::TryAs(..)                            => write!(f, "Try As"),
			TanukiExpressionVariant::Exponent(..)                         => write!(f, "Exponent"),
			TanukiExpressionVariant::SaturatingExponent(..)               => write!(f, "Saturating Exponent"),
			TanukiExpressionVariant::WrappingExponent(..)                 => write!(f, "Wrapping Exponent"),
			TanukiExpressionVariant::TryExponent(..)                      => write!(f, "Try Exponent"),
			TanukiExpressionVariant::Multiplication(..)                   => write!(f, "Multiplication"),
			TanukiExpressionVariant::SaturatingMultiplication(..)         => write!(f, "Saturating Multiplication"),
			TanukiExpressionVariant::WrappingMultiplication(..)           => write!(f, "Wrapping Multiplication"),
			TanukiExpressionVariant::TryMultiplication(..)                => write!(f, "Try Multiplication"),
			TanukiExpressionVariant::Division(..)                         => write!(f, "Division"),
			TanukiExpressionVariant::SaturatingDivision(..)               => write!(f, "Saturating Division"),
			TanukiExpressionVariant::WrappingDivision(..)                 => write!(f, "Wrapping Division"),
			TanukiExpressionVariant::TryDivision(..)                      => write!(f, "Try Division"),
			TanukiExpressionVariant::Modulo(..)                           => write!(f, "Modulo"),
			TanukiExpressionVariant::SaturatingModulo(..)                 => write!(f, "Saturating Modulo"),
			TanukiExpressionVariant::WrappingModulo(..)                   => write!(f, "Wrapping Modulo"),
			TanukiExpressionVariant::TryModulo(..)                        => write!(f, "Try Modulo"),
			TanukiExpressionVariant::Addition(..)                         => write!(f, "Addition"),
			TanukiExpressionVariant::SaturatingAddition(..)               => write!(f, "Saturating Addition"),
			TanukiExpressionVariant::WrappingAddition(..)                 => write!(f, "Wrapping Addition"),
			TanukiExpressionVariant::TryAddition(..)                      => write!(f, "Try Addition"),
			TanukiExpressionVariant::Subtraction(..)                      => write!(f, "Subtraction"),
			TanukiExpressionVariant::SaturatingSubtraction(..)            => write!(f, "Saturating Subtraction"),
			TanukiExpressionVariant::WrappingSubtraction(..)              => write!(f, "Wrapping Subtraction"),
			TanukiExpressionVariant::TrySubtraction(..)                   => write!(f, "Try Subtraction"),
			TanukiExpressionVariant::Concatenate(..)                      => write!(f, "Concatenate"),
			TanukiExpressionVariant::Append(..)                           => write!(f, "Append"),
			TanukiExpressionVariant::BitshiftLeft(..)                     => write!(f, "Bitshift Left"),
			TanukiExpressionVariant::SaturatingBitshiftLeft(..)           => write!(f, "Saturating Bitshift Left"),
			TanukiExpressionVariant::WrappingBitshiftLeft(..)             => write!(f, "Wrapping Bitshift Left"),
			TanukiExpressionVariant::TryBitshiftLeft(..)                  => write!(f, "Try Bitshift Left"),
			TanukiExpressionVariant::BitshiftRight(..)                    => write!(f, "Bitshift Right"),
			TanukiExpressionVariant::ThreeWayCompare(..)                  => write!(f, "Three Way Compare"),
			TanukiExpressionVariant::LessThan(..)                         => write!(f, "Less Than"),
			TanukiExpressionVariant::LessThanOrEqualTo(..)                => write!(f, "Less Than or Equal to"),
			TanukiExpressionVariant::GreaterThan(..)                      => write!(f, "Greater Than"),
			TanukiExpressionVariant::GreaterThanOrEqualTo(..)             => write!(f, "Greater Than or Equal to"),
			TanukiExpressionVariant::Equality(..)                         => write!(f, "Equality"),
			TanukiExpressionVariant::Inequality(..)                       => write!(f, "Inequality"),
			TanukiExpressionVariant::ReferenceEquality(..)                => write!(f, "Reference Equality"),
			TanukiExpressionVariant::ReferenceInequality(..)              => write!(f, "Reference Inequality"),
			TanukiExpressionVariant::NonShortCircuitAnd(..)               => write!(f, "Non Short Circuit And"),
			TanukiExpressionVariant::NonShortCircuitNand(..)              => write!(f, "Non Short Circuit Nand"),
			TanukiExpressionVariant::NonShortCircuitXor(..)               => write!(f, "Non Short Circuit Xor"),
			TanukiExpressionVariant::NonShortCircuitXnor(..)              => write!(f, "Non Short Circuit Xnor"),
			TanukiExpressionVariant::NonShortCircuitOr(..)                => write!(f, "Non Short Circuit Or"),
			TanukiExpressionVariant::NonShortCircuitNor(..)               => write!(f, "Non Short Circuit Nor"),
			TanukiExpressionVariant::Minimum(..)                          => write!(f, "Minimum"),
			TanukiExpressionVariant::Maximum(..)                          => write!(f, "Maximum"),
			TanukiExpressionVariant::Pipe(..)                             => write!(f, "Pipe"),
			TanukiExpressionVariant::ShortCircuitAnd(..)                  => write!(f, "Short Circuit And"),
			TanukiExpressionVariant::ShortCircuitNand(..)                 => write!(f, "Short Circuit Nand"),
			TanukiExpressionVariant::ShortCircuitXor(..)                  => write!(f, "Short Circuit Xor"),
			TanukiExpressionVariant::ShortCircuitXnor(..)                 => write!(f, "Short Circuit Xnor"),
			TanukiExpressionVariant::ShortCircuitOr(..)                   => write!(f, "Short Circuit Or"),
			TanukiExpressionVariant::ShortCircuitNor(..)                  => write!(f, "Short Circuit Nor"),
			TanukiExpressionVariant::NonShortCircuitingNullCoalescing(..) => write!(f, "Non Short Circuiting Null Coalescing"),
			TanukiExpressionVariant::ShortCircuitingNullCoalescing(..)    => write!(f, "Short Circuiting Null Coalescing"),
			TanukiExpressionVariant::ExclusiveRange(..)                   => write!(f, "Exclusive Range"),
			TanukiExpressionVariant::InclusiveRange(..)                   => write!(f, "Inclusive Range"),
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
			TanukiExpressionVariant::FunctionDefinition { parameters, body_expression } => {
				for parameter in parameters {
					parameter.print(level, f)?;
				}
				body_expression.print(level, f)
			}
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
			TanukiExpressionVariant::InclusiveRange(lhs, rhs) => {
				lhs.print(level, f)?;
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