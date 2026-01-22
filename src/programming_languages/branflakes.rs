use std::{fmt::{self, Debug, Formatter}, io::{self, Write}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::c::{expression::CExpression, module::CModule, module_element::CModuleElement, statement::{CCompoundStatement, CInitializer, CStatement}, types::CType}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::{ast_node::AstNode, module::Module, programming_language::ProgrammingLanguage, statement::Statement, token::Token, virtual_machine::VirtualMachine}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	/// Parses a single statement from tokens including a loop containing other statements.
	fn parse_statement(token_reader: &mut TokenReader<BranflakesToken>) -> Result<Option<BranflakesStatement>, ErrorAt> {
		// Read the token or return if it is a loop end or the end of the tokens
		let token = match token_reader.next().cloned() {
			None | Some(BranflakesToken { variant: BranflakesTokenVariant::LoopEnd, .. }) => return Ok(None),
			Some(token) => token,
		};
		// Convert the token to a statement if it is not a loop start
		// If it is a loop start, parse the content tokens to statements and wrap them up in a loop statement
		let statement_variant = match token.variant {
			BranflakesTokenVariant::Increment => BranflakesStatementVariant::Increment,
			BranflakesTokenVariant::Decrement => BranflakesStatementVariant::Decrement,
			BranflakesTokenVariant::IncrementPointer => BranflakesStatementVariant::IncrementPointer,
			BranflakesTokenVariant::DecrementPointer => BranflakesStatementVariant::DecrementPointer,
			BranflakesTokenVariant::Print => BranflakesStatementVariant::Print,
			BranflakesTokenVariant::Input => BranflakesStatementVariant::Input,
			BranflakesTokenVariant::LoopStart => {
				BranflakesStatementVariant::Loop(Self::parse_statements(token_reader, true)?)
			},
			BranflakesTokenVariant::LoopEnd => unreachable!(),
		};
		let statement = BranflakesStatement {
			variant: statement_variant,
			start_line: token.line,
			start_column: token.column,
			end_line: token_reader.last_token_end_line(),
			end_column: token_reader.last_token_end_column(),
		};
		// Return
		Ok(Some(statement))
	}

	/// Parses statements from tokens until a loop end token is found or the token buffer end is reached.
	fn parse_statements(token_reader: &mut TokenReader<BranflakesToken>, is_parenthesised: bool) -> Result<Box<[BranflakesStatement]>, ErrorAt> {
		// Parse single statements until the end is reached
		let mut statements = Vec::new();
		loop {
			match Self::parse_statement(token_reader)? {
				None => break,
				Some(statement) => statements.push(statement),
			};
		}
		// Throw an error if there were un-equal amounts of opening and closing parentheses
		match token_reader.peek() {
			None if is_parenthesised => return Err(Error::MoreOpeningParenthesesThanClosingParentheses.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			Some(BranflakesToken { line, column, .. }) if !is_parenthesised =>
				return Err(Error::MoreClosingParenthesesThanOpeningParentheses.at(Some(*line), Some(*column), None)),
			_ => {}
		}
		// Return parsed statements
		Ok(statements.into())
	}
}

impl ProgrammingLanguage<BranflakesToken, BranflakesModule> for Branflakes {
	fn tokenize_next_token(_main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<BranflakesToken>, ErrorAt> {
		// Read chars until we get a valid BF one
		loop {
			// Get token position in file
			let line = reader.get_line();
			let column = reader.get_column();
			// Read a char, convert it to a token and return it if it is a BF token, else skip it and go to the next char.
			return match reader.read_char()? {
				Some(chr) => Ok(Some(BranflakesToken {
					variant: match chr {
						'+' => BranflakesTokenVariant::Increment,
						'-' => BranflakesTokenVariant::Decrement,
						'>' => BranflakesTokenVariant::IncrementPointer,
						'<' => BranflakesTokenVariant::DecrementPointer,
						'.' => BranflakesTokenVariant::Print,
						',' => BranflakesTokenVariant::Input,
						'[' => BranflakesTokenVariant::LoopStart,
						']' => BranflakesTokenVariant::LoopEnd,
						_ => continue,
					},
					line,
					column,
				})),
				None => Ok(None),
			}
		}
	}

	fn parse_tokens(_main: &mut Main, mut tokens_reader: TokenReader<BranflakesToken>) -> Result<BranflakesModule, ErrorAt> {
		let statements = Self::parse_statements(&mut tokens_reader, false)?;
		Ok(BranflakesModule { statements })
	}
}

#[derive(Debug, Clone)]
pub enum BranflakesTokenVariant {
	/// Wrapping increment the byte pointed to by the data pointer.
	Increment,
	/// Wrapping decrement the byte pointed to by the data pointer.
	Decrement,
	/// Increment the data pointer.
	IncrementPointer,
	/// Decrement the data pointer.
	DecrementPointer,
	/// Print out the byte pointed to by the data pointer.
	Print,
	/// Get a single char from the input buffer, write it to the byte pointed to by the data pointer.
	Input,
	/// The start of a loop. Will jump to the matching loop end if the byte pointed to by the data pointer is zero.
	LoopStart,
	/// The end of a loop. Will jump to the matching loop start if the byte pointed to by the data pointer is non-zero.
	LoopEnd,
}

/// A single BF token parsed from a single source char.
#[derive(Clone)]
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

impl Token for BranflakesToken {
	fn start_line(&self) -> NonZeroUsize {
		self.line
	}

	fn end_line(&self) -> NonZeroUsize {
		self.line
	}

	fn start_column(&self) -> NonZeroUsize {
		self.column
	}

	fn end_column(&self) -> NonZeroUsize {
		self.column.saturating_add(1)
	}

	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self.variant {
			BranflakesTokenVariant::Increment        => write!(f, "Increment"),
			BranflakesTokenVariant::Decrement        => write!(f, "Decrement"),
			BranflakesTokenVariant::IncrementPointer => write!(f, "Increment Pointer"),
			BranflakesTokenVariant::DecrementPointer => write!(f, "Decrement Pointer"),
			BranflakesTokenVariant::Print            => write!(f, "Print"),
			BranflakesTokenVariant::Input            => write!(f, "Input"),
			BranflakesTokenVariant::LoopStart        => write!(f, "Loop Start"),
			BranflakesTokenVariant::LoopEnd          => write!(f, "Loop End"),
		}
	}
}

#[derive(Clone)]
pub enum BranflakesStatementVariant {
	/// Wrapping increment the byte pointed to by the data pointer.
	Increment,
	/// Wrapping decrement the byte pointed to by the data pointer.
	Decrement,
	/// Increment the data pointer.
	IncrementPointer,
	/// Decrement the data pointer.
	DecrementPointer,
	/// Print out the byte pointed to by the data pointer.
	Print,
	/// Get a single char from the input buffer, write it to the byte pointed to by the data pointer.
	Input,
	/// A loop. The loop's content will be executed repeatedly until the byte pointed to by the data pointer is zero.
	Loop(Box<[BranflakesStatement]>),
}

/// A BF statement.
#[derive(Clone)]
pub struct BranflakesStatement {
	variant: BranflakesStatementVariant,
	start_line: NonZeroUsize,
	start_column: NonZeroUsize,
	end_line: NonZeroUsize,
	end_column: NonZeroUsize,
}

impl BranflakesStatement {
	fn execute_interpreted(&self, virtual_machine: &mut BranflakesVirtualMachine) -> Result<(), ErrorAt> {
		match self.variant.clone() {
			BranflakesStatementVariant::IncrementPointer => virtual_machine.data_pointer = virtual_machine.data_pointer.checked_add(1)
				.ok_or_else(|| Error::IntegerOverflow.at(Some(self.start_line), Some(self.start_column), None))?,
			BranflakesStatementVariant::DecrementPointer => virtual_machine.data_pointer = virtual_machine.data_pointer.checked_sub(1)
				.ok_or_else(|| Error::IntegerUnderflow.at(Some(self.start_line), Some(self.start_column), None))?,
			BranflakesStatementVariant::Increment => virtual_machine.write(virtual_machine.read().wrapping_add(1)),
			BranflakesStatementVariant::Decrement => virtual_machine.write(virtual_machine.read().wrapping_sub(1)),
			BranflakesStatementVariant::Print => {
				let value = virtual_machine.read();
				if value > 127 {
					return Err(Error::InvalidAsciiValue.at(Some(self.start_line), Some(self.start_column), None));
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
				// Request user input if there is no stored input string. Store it in the input string with its chars reversed
				if virtual_machine.input.is_none() {
					let mut buffer = String::new();
					io::stdin().read_line(&mut buffer).unwrap();
					virtual_machine.input = Some(buffer.chars().rev().collect());
				}
				// Pop the last char off the input string
				let chr = match &mut virtual_machine.input {
					Some(chr) => chr.pop(),
					None => panic!(),
				};
				let result = match chr {
					// If we have popped off the last char, read value is 0xFF and the input string is deleted
					None => {
						virtual_machine.input = None;
						0xFF
					}
					// Else it is the popped char or 0x01 if it is non-ASCII
					Some(chr) => match chr {
						'\u{0080}'.. => 0x01,
						ascii_char => ascii_char as u8,
					}
				};
				// Store the read char at the byte pointed to by the data pointer
				virtual_machine.write(result);

			}
		}
		Ok(())
	}
}

impl Statement for BranflakesStatement {}

impl AstNode for BranflakesStatement {
	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.start_line)
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.end_line)
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.start_column)
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.end_column)
	}

	fn print_name(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		match self.variant {
			BranflakesStatementVariant::Increment        => write!(f, "Increment"),
			BranflakesStatementVariant::Decrement        => write!(f, "Decrement"),
			BranflakesStatementVariant::IncrementPointer => write!(f, "Increment Pointer"),
			BranflakesStatementVariant::DecrementPointer => write!(f, "Decrement Pointer"),
			BranflakesStatementVariant::Print            => write!(f, "Print"),
			BranflakesStatementVariant::Input            => write!(f, "Input"),
			BranflakesStatementVariant::Loop(_)          => write!(f, "Loop"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		match &self.variant {
			BranflakesStatementVariant::Loop(sub_nodes) => {
				for sub_node in sub_nodes {
					sub_node.print(level, f)?;
				}
			}
			_ => {}
		}
		Ok(())
	}
}

/// An entire parsed file
pub struct BranflakesModule {
	statements: Box<[BranflakesStatement]>,
}

impl Module for BranflakesModule {
	fn interpreted_execute_entrypoint(&self, main: &mut Main) -> Result<(), ErrorAt> {
		// Create the BF VM
		let mut virtual_machine = BranflakesVirtualMachine::new(main);
		// Execute all statements
		for statement in self.statements.iter() {
			statement.execute_interpreted(&mut virtual_machine)?;
		}
		Ok(())
	}

	fn to_c_module(&self, _main: &mut Main, is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		if !is_entrypoint {
			return Err(Error::NotYetImplemented("BF to C not entrypoint".into()).at(None, None, None));
		}
		// Create main function body
		let mut main_function_body = CCompoundStatement::new();
		// Add memory allocation for memory buffer
		let function_call = CInitializer::Expression(CExpression::FunctionCall("calloc".into(), [CExpression::IntConstant(30000), CExpression::Sizeof(CType::U8)].into()));
		main_function_body.push_statement(CStatement::VariableDeclaration(CType::PointerTo(CType::U8.into()), "memory".into(), Some(function_call.into())));
		// Create main function and a C module to add it to
		let main_function = CModuleElement::FunctionDefinition { return_type: CType::Int, name: "main".into(), parameters: Default::default(), body: Box::new(main_function_body) };
		let mut c_module = CModule::new();
		c_module.push_element(main_function);
		Ok(Some(c_module))
	}
}

impl AstNode for BranflakesModule {
	fn start_line(&self) -> Option<NonZeroUsize> {
		match self.statements.first() {
			Some(first_statement) => first_statement.start_line(),
			None => Some(NonZeroUsize::new(1).unwrap()),
		}
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		match self.statements.last() {
			Some(last_statement) => last_statement.end_line(),
			None => Some(NonZeroUsize::new(1).unwrap()),
		}
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		match self.statements.first() {
			Some(first_statement) => first_statement.start_column(),
			None => Some(NonZeroUsize::new(1).unwrap()),
		}
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		match self.statements.last() {
			Some(last_statement) => last_statement.end_column(),
			None => Some(NonZeroUsize::new(1).unwrap()),
		}
	}

	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "BF Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for statement in &self.statements {
			statement.print(level, f)?;
		}
		Ok(())
	}
}

impl Debug for BranflakesModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}

/// The virtual machine that will execute BF code in interpreted mode.
struct BranflakesVirtualMachine {
	/// The array of cells.
	memory: Vec<u8>,
	/// The data pointer, is an index for `memory`.
	data_pointer: usize,
	/// The input string. Chars are reversed from what is entered.
	input: Option<String>,
}

impl BranflakesVirtualMachine {
	/// Reads the byte pointed to by the data pointer if it exists. Else return 0 if the data buffer is too small.
	pub fn read(&self) -> u8 {
		self.memory.get(self.data_pointer).map(|value| *value).unwrap_or(0)
	}

	/// Writes to the byte pointed to by the data pointer. Will expand the data buffer first with zero bytes if the buffer is too small.
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