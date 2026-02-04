use crate::{Main, error::ErrorAt, programming_languages::tanuki::{module::TanukiModule, token::TanukiToken}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::programming_language::ProgrammingLanguage};

pub mod module;
pub mod token;

#[derive(Debug)]
pub struct Tanuki {}

impl Tanuki {
	pub fn new() -> Self {
		Self {}
	}
}

impl ProgrammingLanguage<TanukiToken, TanukiModule> for Tanuki {
	fn tokenize_next_token(main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<TanukiToken>, ErrorAt> {
		// Strip leading whitespaces
		reader.skip_leading_ascii_whitespaces()?;
		// Peek first char
		let first_char = match reader.peek_char()? {
			Some(first_char) => first_char,
			None => return Ok(None),
		};
		// Match depending on first char
		match first_char {
			_ => todo!()
		}
		todo!()
	}

	fn parse_tokens(main: &mut Main, token_reader: TokenReader<TanukiToken>) -> Result<TanukiModule, ErrorAt> {
		todo!()
	}
}