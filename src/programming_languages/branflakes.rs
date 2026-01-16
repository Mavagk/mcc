use crate::{error::ErrorAt, traits::{programming_language::ProgrammingLanguage, token::Token}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	pub const fn new() -> Self {
		Self
	}
}

impl ProgrammingLanguage<BrainFlakesToken> for Branflakes {
	//type TokenType = BrainFlakesStatement;

	fn get_extensions(&self) -> &'static [&'static str] {
		&["bf"]
	}

	fn tokenize_next_token(&self, main: &mut crate::Main, reader: &mut crate::source_file_reader::SourceFileReader) -> Result<Option<BrainFlakesToken>, ErrorAt> {
		todo!()
	}
}

#[derive(Debug)]
pub enum BrainFlakesToken {
	Increment,
	Decrement,
	IncrementPointer,
	DecrementPointer,
	Print,
	Input,
	LoopStart,
	LoopEnd,
}

impl Token for BrainFlakesToken {

}