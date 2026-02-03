use crate::{error::ErrorAt, programming_languages::tanuki::{module::TanukiModule, token::TanukiToken}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::programming_language::ProgrammingLanguage};

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
	fn tokenize_next_token(main: &mut crate::Main, reader: &mut SourceFileReader) -> Result<Option<TanukiToken>, ErrorAt> {
		todo!()
	}

	fn parse_tokens(main: &mut crate::Main, token_reader: TokenReader<TanukiToken>) -> Result<TanukiModule, ErrorAt> {
		todo!()
	}
}