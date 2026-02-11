
use crate::{Main, error::ErrorAt, programming_languages::tanuki::{module::TanukiModule, token::TanukiToken, tokenize::tokenize_token}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::programming_language::ProgrammingLanguage};

pub mod module;
pub mod token;
pub mod tokenize;
pub mod parse;
pub mod expression;
pub mod constant_value;

#[derive(Debug)]
pub struct Tanuki {}

impl Tanuki {
	pub fn new() -> Self {
		Self {}
	}
}

impl ProgrammingLanguage<TanukiToken, TanukiModule> for Tanuki {
	fn tokenize_next_token(main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<TanukiToken>, ErrorAt> {
		tokenize_token(main, reader)
	}

	fn parse_tokens(main: &mut Main, mut token_reader: TokenReader<TanukiToken>) -> Result<TanukiModule, ErrorAt> {
		TanukiModule::parse(main, &mut token_reader)
	}
}