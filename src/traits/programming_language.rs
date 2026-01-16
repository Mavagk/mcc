use std::fmt::Debug;

use crate::{Main, error::ErrorAt, source_file_reader::SourceFileReader, traits::token::Token};

pub trait ProgrammingLanguage<T>: Debug where T: Token {
	fn get_extensions(&self) -> &'static [&'static str];
	fn parse_next_token(&self, main: &mut Main, reader: &mut SourceFileReader) -> Result<ErrorAt, Option<T>>;
}