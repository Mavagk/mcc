use crate::{error::ErrorAt, source_file_reader::SourceFileReader, traits::{programming_language::ProgrammingLanguage, token::Token}};

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	pub const fn new() -> Self {
		Self
	}
}

impl ProgrammingLanguage<BranFlakesToken> for Branflakes {
	fn get_extensions(&self) -> &'static [&'static str] {
		&["bf"]
	}

	fn tokenize_next_token(&self, _main: &mut crate::Main, reader: &mut SourceFileReader) -> Result<Option<BranFlakesToken>, ErrorAt> {
		loop {
			match reader.peek_char()? {
				Some('+' | '-' | '>' | '<' | '.' | ',' | '[' | ']') => {},
				Some(_) => {
					reader.read_char()?;
					continue;
				}
				None => return Ok(None),
			}
			match reader.read_char()? {
				Some(chr) => return Ok(Some(match chr {
					'+' => BranFlakesToken::Increment,
					'-' => BranFlakesToken::Decrement,
					'>' => BranFlakesToken::IncrementPointer,
					'<' => BranFlakesToken::DecrementPointer,
					'.' => BranFlakesToken::Print,
					',' => BranFlakesToken::Input,
					'[' => BranFlakesToken::LoopStart,
					']' => BranFlakesToken::LoopEnd,
					_ => unreachable!(),
				})),
				None => return Ok(None),
			}
		}
	}
}

#[derive(Debug)]
pub enum BranFlakesToken {
	Increment,
	Decrement,
	IncrementPointer,
	DecrementPointer,
	Print,
	Input,
	LoopStart,
	LoopEnd,
}

impl Token for BranFlakesToken {

}