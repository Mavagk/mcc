use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, source_file_reader::SourceFileReader, traits::{module::Module, programming_language::ProgrammingLanguage, statement::Statement, token::Token}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	pub const fn new() -> Self {
		Self
	}

	fn parse_statement(tokens: &mut &[BranflakesToken]) -> Result<Option<BranflakesStatement>, ErrorAt> {
		let token = match tokens.get(0) {
			None | Some(BranflakesToken { variant: BranflakesTokenVariant::LoopEnd, .. }) => return Ok(None),
			Some(token) => token,
		};
		let statement_variant = match token.variant {
			BranflakesTokenVariant::Increment => BranflakesStatementVariant::Increment,
			BranflakesTokenVariant::Decrement => BranflakesStatementVariant::Decrement,
			BranflakesTokenVariant::IncrementPointer => BranflakesStatementVariant::IncrementPointer,
			BranflakesTokenVariant::DecrementPointer => BranflakesStatementVariant::DecrementPointer,
			BranflakesTokenVariant::Print => BranflakesStatementVariant::Print,
			BranflakesTokenVariant::Input => BranflakesStatementVariant::Input,
			BranflakesTokenVariant::LoopStart => {
				*tokens = &tokens[1..];
				BranflakesStatementVariant::Loop(Self::parse_statements(tokens, true)?)
			},
			BranflakesTokenVariant::LoopEnd => unreachable!(),
		};
		let statement = BranflakesStatement {
			variant: statement_variant,
			line: token.line,
			column: token.column,
		};
		*tokens = &tokens[1..];
		Ok(Some(statement))
	}

	fn parse_statements(tokens: &mut &[BranflakesToken], is_parenthesised: bool) -> Result<Box<[BranflakesStatement]>, ErrorAt> {
		let mut statements = Vec::new();
		loop {
			let statement = match Self::parse_statement(tokens)? {
				None => break,
				Some(statement) => statement,
			};
			statements.push(statement);
		}
		match tokens.get(0) {
			None if is_parenthesised => return Err(Error::MoreOpeningParenthesesThanClosingParentheses.at(None, None, None)),
			Some(BranflakesToken { line, column, .. }) if !is_parenthesised =>
				return Err(Error::MoreClosingParenthesesThanOpeningParentheses.at(Some(*line), Some(*column), None)),
			_ => {}
		}
		Ok(statements.into())
	}
}

impl ProgrammingLanguage<BranflakesToken, BranflakesModule> for Branflakes {
	fn get_extensions(&self) -> &'static [&'static str] {
		&["bf"]
	}

	fn tokenize_next_token(&self, _main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<BranflakesToken>, ErrorAt> {
		loop {
			match reader.peek_char()? {
				Some('+' | '-' | '>' | '<' | '.' | ',' | '[' | ']') => {},
				Some(_) => {
					reader.read_char()?;
					continue;
				}
				None => return Ok(None),
			}
			let line = reader.get_line();
			let column = reader.get_column();
			match reader.read_char()? {
				Some(chr) => return Ok(Some(BranflakesToken {
					variant: match chr {
						'+' => BranflakesTokenVariant::Increment,
						'-' => BranflakesTokenVariant::Decrement,
						'>' => BranflakesTokenVariant::IncrementPointer,
						'<' => BranflakesTokenVariant::DecrementPointer,
						'.' => BranflakesTokenVariant::Print,
						',' => BranflakesTokenVariant::Input,
						'[' => BranflakesTokenVariant::LoopStart,
						']' => BranflakesTokenVariant::LoopEnd,
						_ => unreachable!(),
					},
					line,
					column,
				})),
				None => return Ok(None),
			}
		}
	}

	fn parse_tokens(&self, _main: &mut Main, mut tokens: &[BranflakesToken]) -> Result<BranflakesModule, ErrorAt> {
		let statements = Self::parse_statements(&mut tokens, false)?;
		Ok(BranflakesModule { statements })
	}
}

#[derive(Debug)]
pub enum BranflakesTokenVariant {
	Increment,
	Decrement,
	IncrementPointer,
	DecrementPointer,
	Print,
	Input,
	LoopStart,
	LoopEnd,
}

pub struct BranflakesToken {
	variant: BranflakesTokenVariant,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl Debug for BranflakesToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}: {:?}", self.line, self.column, self.variant)
	}
}

impl Token for BranflakesToken {}

#[derive(Debug)]
pub enum BranflakesStatementVariant {
	Increment,
	Decrement,
	IncrementPointer,
	DecrementPointer,
	Print,
	Input,
	Loop(Box<[BranflakesStatement]>),
}

pub struct BranflakesStatement {
	variant: BranflakesStatementVariant,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl Debug for BranflakesStatement {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}: {:?}", self.line, self.column, self.variant)
	}
}

impl Statement for BranflakesStatement {}

#[derive(Debug)]
pub struct BranflakesModule {
	statements: Box<[BranflakesStatement]>,
}

impl Module for BranflakesModule {}