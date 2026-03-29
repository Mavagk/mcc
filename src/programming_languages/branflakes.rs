use std::{fmt::{self, Debug, Formatter}, io::{self, Write}, num::NonZeroUsize, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::c::{expression::CExpression, l_value::CLValue, module::CModule, module_element::{CTypeAndName, CModuleElement}, statement::{CCompoundStatement, CInitializer, CStatement}, types::CType}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::{ast_node::AstNode, module::Module, programming_language::ProgrammingLanguage, statement::Statement, token::Token, virtual_machine::VirtualMachine}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	/// Parses a single statement from tokens including a loop containing other statements.
	fn parse_statement(token_reader: &mut TokenReader<BranflakesToken>, is_parenthesised: bool) -> Result<Option<BranflakesStatement>, ErrorAt> {
		// Read the token or return if it is a loop end or the end of the tokens
		let token = match token_reader.next().cloned() {
			None if is_parenthesised => return Err(Error::MoreOpeningParenthesesThanClosingParentheses.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			None => return Ok(None),
			Some(BranflakesToken { variant: BranflakesTokenVariant::LoopEnd, line, column, .. }) if !is_parenthesised =>
				return Err(Error::MoreClosingParenthesesThanOpeningParentheses.at(Some(line), Some(column), None)),
			Some(BranflakesToken { variant: BranflakesTokenVariant::LoopEnd, .. }) => return Ok(None),
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
			match Self::parse_statement(token_reader, is_parenthesised)? {
				None => break,
				Some(statement) => statements.push(statement),
			};
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
	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.line)
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.line)
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.column)
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.column.saturating_add(1))
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

	fn to_c(&self, main: &mut Main, compound_statement: &mut CCompoundStatement) -> Result<(), ErrorAt> {
		match &self.variant {
			BranflakesStatementVariant::IncrementPointer => {
				compound_statement.push_statement(CLValue::variable("data_pointer").postfix_increment().into());
				let mut error_compound_statement = CCompoundStatement::new();
				error_compound_statement.push_statement(CExpression::function_call("puts", [CExpression::string_constant("Error: Data pointer overflow.")].into()).into());
				error_compound_statement.push_statement(CExpression::function_call("exit", [CExpression::IntConstant(1)].into()).into());
				compound_statement.push_statement(CExpression::IntConstant(0).equal(CLValue::variable("data_pointer").into()).if_statement(error_compound_statement.into()));
			}
			BranflakesStatementVariant::DecrementPointer => {
				let mut error_compound_statement = CCompoundStatement::new();
				error_compound_statement.push_statement(CExpression::function_call("puts", [CExpression::string_constant("Error: Data pointer underflow.")].into()).into());
				error_compound_statement.push_statement(CExpression::function_call("exit", [CExpression::IntConstant(1)].into()).into());
				compound_statement.push_statement(CExpression::IntConstant(0).equal(CLValue::variable("data_pointer").into()).if_statement(error_compound_statement.into()));
				compound_statement.push_statement(CLValue::variable("data_pointer").postfix_decrement().into())
			}
			BranflakesStatementVariant::Increment => {
				compound_statement.push_statement(CExpression::function_call("expand_memory", [
					CLValue::variable("memory_buffer").take_reference(),
					CLValue::variable("memory_buffer_size").take_reference(),
					CExpression::IntConstant(1).add(CLValue::variable("data_pointer").into())
				].into()).into());
				compound_statement.push_statement(CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).postfix_increment().into());
			}
			BranflakesStatementVariant::Decrement => {
				compound_statement.push_statement(CExpression::function_call("expand_memory", [
					CLValue::variable("memory_buffer").take_reference(),
					CLValue::variable("memory_buffer_size").take_reference(),
					CExpression::IntConstant(1).add(CLValue::variable("data_pointer").into())
				].into()).into());
				compound_statement.push_statement(CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).postfix_decrement().into());
			}
			BranflakesStatementVariant::Loop(sub_statements) => {
				compound_statement.push_statement(CExpression::function_call("expand_memory", [
					CLValue::variable("memory_buffer").take_reference(),
					CLValue::variable("memory_buffer_size").take_reference(),
					CExpression::IntConstant(1).add(CLValue::variable("data_pointer").into())
				].into()).into());
				let mut sub_compound_statement = CCompoundStatement::new();
				for sub_statement in sub_statements.iter() {
					sub_statement.to_c(main, &mut sub_compound_statement)?;
				}
				compound_statement.push_statement(CExpression::IntConstant(0).not_equal(CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).into())
					.while_statement(sub_compound_statement.into())
				);
			}
			BranflakesStatementVariant::Print => {
				compound_statement.push_statement(CExpression::function_call("expand_memory", [
					CLValue::variable("memory_buffer").take_reference(),
					CLValue::variable("memory_buffer_size").take_reference(),
					CExpression::IntConstant(1).add(CLValue::variable("data_pointer").into())
				].into()).into());
				compound_statement.push_statement(CExpression::function_call("putchar", [
					CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).into()
				].into()).into());
			}
			BranflakesStatementVariant::Input => {
				compound_statement.push_statement(
					CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into())
						.assign(CExpression::function_call("getchar", [].into()).into()
				).into());
				compound_statement.push_statement(
					CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).read().equal(CLValue::variable("EOF").into())
					.if_statement(CLValue::variable("memory_buffer").array_subscript(CLValue::variable("data_pointer").into()).assign(CExpression::IntConstant(-1).into()).into()
				).into());
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

	fn to_c_module(&self, main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		if !is_entrypoint {
			return Err(Error::NotYetImplemented("BF to C not entrypoint".into()).at(None, None, None));
		}
		// Create module
		let mut c_module = CModule::new();
		// Add includes
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdint.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdlib.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("stdio.h".into()));
		c_module.push_element(CModuleElement::AngleIncludeInHeader("string.h".into()));
		// Add a function the expands the memory buffer that makes it have at least X cells
		let mut expand_memory_function_body = CCompoundStatement::new();
		expand_memory_function_body.push_statement(CStatement::If(
			CExpression::GreaterThanOrEqual(
				CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer_size".into()).into()).into()).into()).into(),
				CExpression::LValueRead(CLValue::Variable("resize_to_at_least".into()).into()).into()
			).into(), CStatement::Return(None).into()
		));
		expand_memory_function_body.push_statement(CStatement::VariableDeclaration(CType::USize.into(), "old_size".into(), Some(
			CInitializer::Expression(CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer_size".into()).into()).into()).into()).into()).into()
		)).into());
		expand_memory_function_body.push_statement(CStatement::Expression(CExpression::Assignment(
			CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer_size".into()).into()).into()).into(),
			CExpression::Multiply(CExpression::LValueRead(CLValue::Variable("resize_to_at_least".into()).into()).into(), CExpression::IntConstant(2).into()).into()
		).into()));
		expand_memory_function_body.push_statement(CStatement::Expression(CExpression::Assignment(
			CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer".into()).into()).into()).into(),
			CExpression::FunctionCall("realloc".into(), [
				CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer".into()).into()).into()).into()),
				CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer_size".into()).into()).into()).into())
			].into()).into()
		).into()));

		let mut allocation_error_compound_statement = CCompoundStatement::new();
		allocation_error_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("puts".into(), [CExpression::StringConstant("Error: Out of memory.".into())].into())));
		allocation_error_compound_statement.push_statement(CStatement::Expression(CExpression::FunctionCall("exit".into(), [CExpression::IntConstant(1)].into())));
		expand_memory_function_body.push_statement(CStatement::If(CExpression::Equal(
			CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer".into()).into()).into()).into()).into(),
			CExpression::LValueRead(CLValue::Variable("NULL".into()).into()).into()
		).into(), CStatement::CompoundStatement(allocation_error_compound_statement.into()).into()));

		expand_memory_function_body.push_statement(CStatement::Expression(CExpression::FunctionCall("memset".into(), [
			CExpression::Add(
				CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer".into()).into()).into()).into()).into(),
				CExpression::LValueRead(CLValue::Variable("old_size".into()).into()).into()
			),
			CExpression::IntConstant(0),
			CExpression::Subtract(
				CExpression::LValueRead(CLValue::Dereference(CExpression::LValueRead(CLValue::Variable("memory_buffer_size".into()).into()).into()).into()).into(),
				CExpression::LValueRead(CLValue::Variable("old_size".into()).into()).into()
			)
		].into()).into()));

		c_module.push_element(CModuleElement::FunctionDefinition {
			return_type: CType::Void, name: "expand_memory".into(), parameters: [
				CTypeAndName::new(CType::PointerTo(CType::PointerTo(CType::U8.into()).into()), "memory_buffer".into()),
				CTypeAndName::new(CType::PointerTo(CType::USize.into()), "memory_buffer_size".into()),
				CTypeAndName::new(CType::USize, "resize_to_at_least".into())
			].into(), body: expand_memory_function_body.into()
		});
		// Create main function body
		let mut main_function_body = CCompoundStatement::new();
		// Add memory buffer variables
		main_function_body.push_statement(CStatement::Comment("The BF memory state".into()));
		let memory_buffer_init = CInitializer::Expression(CExpression::LValueRead(CLValue::Variable("NULL".into()).into()));
		main_function_body.push_statement(CStatement::VariableDeclaration(CType::PointerTo(CType::U8.into()), "memory_buffer".into(), Some(memory_buffer_init.into())));
		main_function_body.push_statement(CStatement::VariableDeclaration(CType::USize, "memory_buffer_size".into(), Some(CInitializer::Expression(CExpression::IntConstant(0)).into())));
		main_function_body.push_statement(CStatement::VariableDeclaration(CType::USize, "data_pointer".into(), Some(CInitializer::Expression(CExpression::IntConstant(0)).into())));
		// Convert modules statements to C
		for statement in self.statements.iter() {
			statement.to_c(main, &mut main_function_body)?;
		}
		// Create main function and add to the module
		let main_function = CModuleElement::FunctionDefinition { return_type: CType::Int, name: "main".into(), parameters: Default::default(), body: main_function_body.into() };
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