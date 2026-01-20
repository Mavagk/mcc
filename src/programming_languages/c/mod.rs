pub mod module;
pub mod module_element;
pub mod statement;
pub mod types;

use crate::{Main, error::ErrorAt, programming_languages::c::module::CModule, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::{programming_language::ProgrammingLanguage, token::Token}};

#[derive(Debug)]
pub struct C {

}

impl C {
	pub fn new() -> Self {
		Self {}
	}
}

impl ProgrammingLanguage<CToken, CModule> for C {
	fn tokenize_next_token(_main: &mut Main, _reader: &mut SourceFileReader) -> Result<Option<CToken>, ErrorAt> {
		unimplemented!()
	}

	fn parse_tokens(_main: &mut Main, _token_reader: TokenReader<CToken>) -> Result<CModule, ErrorAt> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct CToken {

}

impl Token for CToken {
	fn get_start_line(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_end_column(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_start_column(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_end_line(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn print_name(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}