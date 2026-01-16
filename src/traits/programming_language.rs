use std::fmt::Debug;

use crate::{Main, error::ErrorAt, source_file_reader::SourceFileReader, traits::token::Token};

pub trait ProgrammingLanguage<T>: Debug where T: Token {
	fn get_extensions(&self) -> &'static [&'static str];
	fn tokenize_next_token(&self, main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<T>, ErrorAt>;

	fn tokenize(&self, main: &mut Main, reader: &mut SourceFileReader) -> Result<Box<[T]>, ErrorAt> {
		let mut tokens = Vec::new();
		loop {
			match self.tokenize_next_token(main, reader)? {
				Some(token) => tokens.push(token),
				None => return Ok(tokens.into_boxed_slice()),
			}
		}
	}
}