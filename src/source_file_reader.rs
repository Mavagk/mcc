use std::{fs::File, io::{BufReader, Bytes, Read}, iter::Peekable, num::NonZeroUsize, path::Path};

use crate::error::Error;

pub struct SourceFileReader<'a> {
	reader: Peekable<Utf8Iter>,
	path: &'a Path,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl<'a> SourceFileReader<'a> {
	/// Create a new source reader from opening a file at a path.
	pub fn new(path: &'a Path) -> Result<Self, Error> {
		// Open file
		let file = File::open(path).map_err(|err| Error::UnableToOpenFile(path.to_string_lossy().into(), err.to_string()))?;
		let reader = Utf8Iter {
			bytes: BufReader::new(file).bytes()
		}.peekable();
		// Pack into struct
		Ok(Self {
			reader,
			path,
			line: 1.try_into().unwrap(),
			column: 1.try_into().unwrap(),
		})
	}

	pub fn get_line(&self) -> NonZeroUsize {
		self.line
	}

	pub fn get_column(&self) -> NonZeroUsize {
		self.column
	}

	pub fn peek_char(&mut self) -> Option<Result<char, Error>> {
		self.reader.peek().cloned()
	}
}

struct Utf8Iter {
	bytes: Bytes<BufReader<File>>,
}

impl Iterator for Utf8Iter {
	type Item = Result<char, Error>;

	fn next(&mut self) -> Option<Result<char, Error>> {
		let first_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(Error::UnableToReadFile(error.to_string()))),
			Ok(first_byte) => first_byte,
		};
		if first_byte < 0x80 {
			return Some(Ok(first_byte as char));
		}

		let second_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(Error::UnableToReadFile(error.to_string()))),
			Ok(second_byte) => second_byte,
		};
		if second_byte < 0xE0 {
			return match (((first_byte as u32 & 0b00011111) << 6) | (second_byte as u32 & 0b00111111)).try_into() {
				Ok(chr) => Some(Ok(chr)),
				Err(_) => Some(Err(Error::InvalidUtf8)),
			};
		}

		let third_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(Error::UnableToReadFile(error.to_string()))),
			Ok(third_byte) => third_byte,
		};
		if third_byte < 0xF0 {
			return match (((first_byte as u32 & 0b00001111) << 12) | ((second_byte as u32 & 0b00111111) << 6) | (third_byte as u32 & 0b00111111)).try_into() {
				Ok(chr) => Some(Ok(chr)),
				Err(_) => Some(Err(Error::InvalidUtf8)),
			};
		}

		let fourth_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(Error::UnableToReadFile(error.to_string()))),
			Ok(fourth_byte) => fourth_byte,
		};
		return match (((first_byte as u32 & 0b00000111) << 18) | ((second_byte as u32 & 0b00111111) << 12) | ((third_byte as u32 & 0b00111111) << 6) | (fourth_byte as u32 & 0b00111111)).try_into() {
			Ok(chr) => Some(Ok(chr)),
			Err(_) => Some(Err(Error::InvalidUtf8)),
		};
	}
}