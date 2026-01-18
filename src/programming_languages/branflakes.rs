use std::{fmt::{self, Debug, Formatter}, io::{self, Write}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, source_file_reader::SourceFileReader, traits::{module::Module, programming_language::ProgrammingLanguage, statement::Statement, token::Token, virtual_machine::VirtualMachine}};

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

#[derive(Debug, Clone)]
pub enum BranflakesStatementVariant {
	Increment,
	Decrement,
	IncrementPointer,
	DecrementPointer,
	Print,
	Input,
	Loop(Box<[BranflakesStatement]>),
}

#[derive(Clone)]
pub struct BranflakesStatement {
	variant: BranflakesStatementVariant,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl BranflakesStatement {
	fn execute_interpreted(&self, virtual_machine: &mut BranflakesVirtualMachine) -> Result<(), ErrorAt> {
		match self.variant.clone() {
			BranflakesStatementVariant::IncrementPointer => virtual_machine.data_pointer = virtual_machine.data_pointer.checked_add(1)
				.ok_or_else(|| Error::IntegerOverflow.at(Some(self.line), Some(self.column), None))?,
			BranflakesStatementVariant::DecrementPointer => virtual_machine.data_pointer = virtual_machine.data_pointer.checked_sub(1)
				.ok_or_else(|| Error::IntegerUnderflow.at(Some(self.line), Some(self.column), None))?,
			BranflakesStatementVariant::Increment => virtual_machine.write(virtual_machine.read().wrapping_add(1)),
			BranflakesStatementVariant::Decrement => virtual_machine.write(virtual_machine.read().wrapping_sub(1)),
			BranflakesStatementVariant::Print => {
				let value = virtual_machine.read();
				if value > 127 {
					return Err(Error::InvalidAsciiValue.at(Some(self.line), Some(self.column), None));
				}
				print!("{}", value as char);
				io::stdout().flush().unwrap();
			}
			BranflakesStatementVariant::Loop(sub_expressions) => {
				while virtual_machine.read() != 0 {
					for sub_expression in sub_expressions.iter() {
						sub_expression.execute_interpreted(virtual_machine)?;
					}
				}
			}
			BranflakesStatementVariant::Input => {
				if virtual_machine.input.is_none() {
					let mut buffer = String::new();
					io::stdin().read_line(&mut buffer).unwrap();
					virtual_machine.input = Some(buffer.chars().rev().collect());
				}
				let chr = match &mut virtual_machine.input {
					Some(chr) => chr.pop(),
					None => panic!(),
				};
				let result = match chr {
					None => {
						virtual_machine.input = None;
						0xFF
					}
					Some(chr) => {
						if chr as u32 > 127 {
							0x01
						}
						else {
							chr as u8
						}
					}
				};
				virtual_machine.write(result);

			}
		}
		//println!("{self:?}");
		Ok(())
	}
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

impl Module for BranflakesModule {
	fn execute_interpreted(&self, main: &mut Main) -> Result<(), ErrorAt> {
		let mut virtual_machine = BranflakesVirtualMachine::new(main);
		for statement in self.statements.iter() {
			statement.execute_interpreted(&mut virtual_machine)?;
		}
		Ok(())
	}
}

struct BranflakesVirtualMachine {
	memory: Vec<u8>,
	data_pointer: usize,
	input: Option<String>,
}

impl BranflakesVirtualMachine {
	pub fn read(&self) -> u8 {
		self.memory.get(self.data_pointer).map(|value| *value).unwrap_or(0)
	}

	pub fn write(&mut self, value: u8) {
		if self.memory.len() <= self.data_pointer {
			self.memory.resize(self.data_pointer + 1, 0);
		}
		self.memory[self.data_pointer] = value;
	}
}

impl VirtualMachine for BranflakesVirtualMachine {
	fn new(_main: &mut Main) -> Self {
		Self {
			memory: Vec::new(),
			data_pointer: 0,
			input: None,
		}
	}
}