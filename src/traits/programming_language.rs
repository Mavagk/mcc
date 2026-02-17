use std::{fmt::Debug, path::Path};

use crate::{Main, arguments::Arguments, error::ErrorAt, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::{module::Module, token::Token}};

pub trait ProgrammingLanguage<T, M>: Debug where T: Token, M: Module {
	/// takes chars from the file reader and returns the next token if it exists.
	fn tokenize_next_token(main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<T>, ErrorAt>;
	/// Parses tokens into a module with its abstract syntax tree.
	fn parse_tokens(main: &mut Main, token_reader: TokenReader<T>) -> Result<M, ErrorAt>;

	fn post_parse(_main: &mut Main, _module: &mut M) -> Result<(), ErrorAt> {
		Ok(())
	}

	/// Reads from a file reader and tokenizes it to tokens.
	fn tokenize(main: &mut Main, reader: &mut SourceFileReader) -> Result<Box<[T]>, ErrorAt> {
		let mut tokens = Vec::new();
		loop {
			match Self::tokenize_next_token(main, reader)? {
				Some(token) => tokens.push(token),
				None => return Ok(tokens.into_boxed_slice()),
			}
		}
	}

	/// Takes in a filepath, opens it, parses its chars to tokens which are then parsed into a module.
	fn tokenize_parse(main: &mut Main, args: &Arguments, filepath: &Path) -> Result<M, ErrorAt> {
		// Open file
		let mut source_file_reader = SourceFileReader::new(&filepath).map_err(|error| error.at(None, None, None))?;
		// Tokenize
		let tokens = Self::tokenize(main, &mut source_file_reader)?;
		if args.print_tokens {
			println!("Tokens of {}", filepath.as_os_str().to_string_lossy());
			for token in tokens.iter() {
				println!("{token:?}");
			}
		}
		// Parse
		let token_reader = TokenReader::new(&tokens);
		let mut module = Self::parse_tokens(main, token_reader)?;
		if args.print_ast_after_parse {
			println!("AST of {} after parse", filepath.as_os_str().to_string_lossy());
			println!("{module:?}");
		}
		// Post parse
		Self::post_parse(main, &mut module)?;
		if args.print_ast_after_post_parse {
			println!("AST of {} after post-parse", filepath.as_os_str().to_string_lossy());
			println!("{module:?}");
		}
		// Return
		Ok(module)
	}
}