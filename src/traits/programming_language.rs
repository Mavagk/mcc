use std::{fmt::Debug, path::Path};

use crate::{Main, arguments::Arguments, error::ErrorAt, source_file_reader::SourceFileReader, traits::{module::Module, token::Token}};

pub trait ProgrammingLanguage<T, M>: Debug where T: Token, M: Module {
	fn tokenize_next_token(main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<T>, ErrorAt>;
	fn parse_tokens(main: &mut Main, tokens: &[T]) -> Result<M, ErrorAt>;

	fn tokenize(main: &mut Main, reader: &mut SourceFileReader) -> Result<Box<[T]>, ErrorAt> {
		let mut tokens = Vec::new();
		loop {
			match Self::tokenize_next_token(main, reader)? {
				Some(token) => tokens.push(token),
				None => return Ok(tokens.into_boxed_slice()),
			}
		}
	}

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
		let module = Self::parse_tokens(main, &tokens)?;
		if args.print_ast {
			println!("{module:?}");
		}
		// Return
		Ok(module)
	}
}