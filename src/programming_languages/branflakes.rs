use crate::{error::ErrorAt, traits::{programming_language::ProgrammingLanguage, statement::Statement, token::Token}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	pub const fn new() -> Self {
		Self
	}
}

impl ProgrammingLanguage<BrainFlakesStatement> for Branflakes {
	//type TokenType = BrainFlakesStatement;

	fn get_extensions(&self) -> &'static [&'static str] {
		&["bf"]
	}

	fn parse_next_token(&self, main: &mut crate::Main, reader: &mut crate::source_file_reader::SourceFileReader) -> Result<ErrorAt, Option<BrainFlakesStatement>> {
		todo!()
	}
}

#[derive(Debug)]
pub enum BrainFlakesStatement {

}

impl Token for BrainFlakesStatement {

}

impl Statement for BrainFlakesStatement {

}