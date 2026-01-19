use crate::{Main, error::ErrorAt, source_file_reader::SourceFileReader, traits::{module::Module, programming_language::ProgrammingLanguage, token::Token}};

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

	fn parse_tokens(_main: &mut Main, _tokens: &[CToken]) -> Result<CModule, ErrorAt> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct CToken {

}

impl Token for CToken {
	
}

#[derive(Debug)]
pub struct CModule {

}

impl Module for CModule {
	fn execute_interpreted(&self, _main: &mut crate::Main) -> Result<(), ErrorAt> {
		unimplemented!()
	}
}