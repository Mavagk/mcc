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
	Block(Box<[TanukiExpression]>)
}

impl TanukiExpression {
	pub fn parse(_main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		let mut maybe_parsed_tokens = Vec::new();
		let mut bracket_depth = 0usize;
		// Loop through all tokens until we reach the end of the expression
		while matches!(token_reader.peek().map(|token| &token.variant), Some(..)) {
			// If we reach a separator that is'int an opening separator or nested, break
			let token = &token_reader.peek().unwrap().variant;
			if matches!(token, TanukiTokenVariant::LeftParenthesis | TanukiTokenVariant::LeftCurlyParenthesis | TanukiTokenVariant::LeftSquareParenthesis) {
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
			// Take the token from the reader and parse it if it is a simple token
			let token = token_reader.next().unwrap();
			let token_start_line = token.start_line;
			let token_end_line = token.end_line;
			let token_start_column = token.start_column;
			let token_end_column = token.end_column;
			let expression_variant = match &token.variant {
				TanukiTokenVariant::NumericLiteral(None, Some(float_value)) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Float(*float_value))),
				TanukiTokenVariant::NumericLiteral(Some(int_value), _) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Integer(int_value.clone().into()))),
				TanukiTokenVariant::NumericLiteral(None, None) => unreachable!(),
				TanukiTokenVariant::CharacterLiteral(value) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::Character(*value))),
				TanukiTokenVariant::StringLiteral(value) => Some(TanukiExpressionVariant::Constant(TanukiConstantValue::String(value.clone()))),
				_ => None
			};
			maybe_parsed_tokens.push(match expression_variant {
				Some(expression_variant) => {
					MaybeParsedToken::Parsed(TanukiExpression { variant: expression_variant, start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column })
				}
				None => MaybeParsedToken::Unparsed(token.clone()),
			});
		}
		if bracket_depth > 0 {
			return Err(Error::MoreOpeningParenthesesThanClosingParentheses.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None));
		}
		println!("{maybe_parsed_tokens:?}");
		// TODO
		Ok(Self {
			variant: TanukiExpressionVariant::Block([].into()),
			end_column: 1.try_into().unwrap(),
			end_line: 1.try_into().unwrap(),
			start_column: 1.try_into().unwrap(),
			start_line: 1.try_into().unwrap(),
		})
	}
}

impl Expression for TanukiExpression {
	
}

impl AstNode for TanukiExpression {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(value) => write!(f, "Constant {value:?}"),
			TanukiExpressionVariant::Block(..) => write!(f, "Block"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(..) => Ok(()),
			TanukiExpressionVariant::Block(sub_expressions) => {
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