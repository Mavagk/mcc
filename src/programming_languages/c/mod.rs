use crate::{Main, error::ErrorAt, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::{ast_node::AstNode, module::Module, programming_language::ProgrammingLanguage, token::Token}};

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

#[derive(Debug)]
pub struct CModule {

}

impl Module for CModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut crate::Main) -> Result<(), ErrorAt> {
		unimplemented!()
	}
}

impl AstNode for CModule {
	fn get_start_line(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_end_line(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_start_column(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn get_end_column(&self) -> std::num::NonZeroUsize {
		todo!()
	}

	fn print_name(&self, _f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		todo!()
	}
}