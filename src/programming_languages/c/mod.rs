pub mod module;
pub mod module_element;
pub mod statement;
pub mod types;
pub mod expression;
pub mod l_value;

use std::{fmt::{self, Formatter}, num::NonZeroUsize};

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
	fn start_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn end_column(&self) -> NonZeroUsize {
		todo!()
	}

	fn start_column(&self) -> NonZeroUsize {
		todo!()
	}

	fn end_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}