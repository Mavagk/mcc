use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, maybe_parsed_token::MaybeParsedToken, programming_languages::tanuki::{constant_value::TanukiConstantValue, token::{TanukiToken, TanukiTokenVariant}}, token_reader::TokenReader, traits::{ast_node::AstNode, expression::Expression}};

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
}

impl TanukiExpression {
	pub fn parse(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Option<Self>, ErrorAt> {
		let mut maybe_parsed_tokens: Vec<MaybeParsedToken<TanukiExpression, (), TanukiToken>> = Vec::new();
		let mut bracket_depth = 0usize;
		// Loop through all tokens until we reach the end of the expression
		while matches!(token_reader.peek().map(|token| &token.variant), Some(..)) {
			// If we reach a separator that is'int an opening separator or nested, break
			let token = &token_reader.peek().unwrap().variant;
			if matches!(token, TanukiTokenVariant::LeftParenthesis/* | TanukiTokenVariant::LeftCurlyParenthesis*/ | TanukiTokenVariant::LeftSquareParenthesis) {
				bracket_depth += 1;
			}
			if matches!(token, TanukiTokenVariant::RightParenthesis | TanukiTokenVariant::RightCurlyParenthesis | TanukiTokenVariant::RightSquareParenthesis) {
				bracket_depth = match bracket_depth.checked_sub(1) {
					Some(bracket_depth) => bracket_depth,
					None => break,
				}
			}
			if matches!(token, TanukiTokenVariant::Comma | TanukiTokenVariant::Semicolon) && bracket_depth == 0 {
				break;
			}
			// First parse round
			let token = token_reader.next().unwrap().clone();
			let token_start_line = token.start_line;
			let token_start_column = token.start_column;
			let expression_variant = match &token.variant {
				TanukiTokenVariant::NumericLiteral(None, Some(float_value)) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Float(*float_value))),
				TanukiTokenVariant::NumericLiteral(Some(int_value), _) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Integer(int_value.clone().into()))),
				TanukiTokenVariant::NumericLiteral(None, None) => unreachable!(),
				TanukiTokenVariant::CharacterLiteral(value) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Character(*value))),
				TanukiTokenVariant::StringLiteral(value) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::String(value.clone()))),
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
							Some(TanukiToken { variant: TanukiTokenVariant::RightCurlyParenthesis, .. }) => {
								break 'a Some(TanukiExpressionVariant::Block { sub_expressions: sub_expressions.into(), has_return_value: !expression_is_empty });
							}
							// The token stream should not just stop
							None => return Err(Error::ExpectedCurlyClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Move on to the next sub-expression if we read a semicolon
							Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, .. }) => {},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedSemicolon.at(Some(*start_column), Some(*end_column), None)),
						}
					}
				}
				_ => None
			};
			maybe_parsed_tokens.push(match expression_variant {
				Some(expression_variant) => {
					MaybeParsedToken::Parsed(TanukiExpression {
						variant: expression_variant, start_line: token_start_line, start_column: token_start_column, end_line: token_reader.last_token_end_line(), end_column: token_reader.last_token_end_column(), 
					})
				}
				None => MaybeParsedToken::Unparsed(token.clone()),
			});
		}
		if bracket_depth > 0 {
			return Err(Error::MoreOpeningParenthesesThanClosingParentheses.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None));
		}
		if maybe_parsed_tokens.is_empty() {
			return Ok(None);
		}
		if maybe_parsed_tokens.len() == 1 && maybe_parsed_tokens[0].is_parsed() {
			return Ok(Some(maybe_parsed_tokens.pop().unwrap().unwrap_parsed()))
		}
		println!("{maybe_parsed_tokens:?}");
		// TODO
		Ok(Some(Self {
			variant: TanukiExpressionVariant::Block { sub_expressions: [].into(), has_return_value: false},
			end_column: 1.try_into().unwrap(),
			end_line: 1.try_into().unwrap(),
			start_column: 1.try_into().unwrap(),
			start_line: 1.try_into().unwrap(),
		}))
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
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(..) => Ok(()),
			TanukiExpressionVariant::Block { sub_expressions, ..} => {
				for sub_expression in sub_expressions {
					sub_expression.print(level, f)?;
				}
				Ok(())
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