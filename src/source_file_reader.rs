use std::{fs::File, io::{BufReader, Bytes, Read}, iter::Peekable, num::NonZeroUsize, path::Path};

use crate::error::{Error, ErrorAt};

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
			path: path,
			line: NonZeroUsize::new(1).unwrap(),
			column: NonZeroUsize::new(1).unwrap(),
		})
	}

	pub fn get_line(&self) -> NonZeroUsize {
		self.line
	}

	pub fn get_column(&self) -> NonZeroUsize {
		self.column
	}

	pub fn peek_char(&mut self) -> Result<Option<char>, ErrorAt> {
		match self.reader.peek().cloned() {
			None => Ok(None),
			Some(Err(err)) => Err(Error::UnableToReadFile(err.to_string()).at(Some(self.line), Some(self.column), Some(self.path.as_os_str().to_string_lossy().into()))),
			Some(Ok(chr)) => Ok(Some(chr)),
		}
	}

	pub fn read_char(&mut self) -> Result<Option<char>, ErrorAt> {
		let chr = match self.reader.next() {
			None => return Ok(None),
			Some(Err(err)) => return Err(Error::UnableToReadFile(err.to_string()).at(Some(self.line), Some(self.column), Some(self.path.as_os_str().to_string_lossy().into()))),
			Some(Ok(chr)) => chr,
		};
		if chr == '\n' {
			self.column = NonZeroUsize::new(1).unwrap();
			self.line = self.line.checked_add(1).unwrap();
		}
		else {
			self.column = self.column.checked_add(1).unwrap();
		}
		Ok(Some(chr))
	}

	pub fn skip_leading_ascii_whitespaces(&mut self) -> Result<(), ErrorAt> {
		while matches!(self.peek_char()?, Some(chr) if chr.is_ascii_whitespace()) {
			self.read_char()?;
		}
		Ok(())
	}

	pub fn skip(&mut self, n: usize) -> Result<(), ErrorAt> {
		for _ in 0..n {
			self.read_char()?;
		}
		Ok(())
	}

	pub fn read_string_while<P>(&mut self, p: P) -> Result<String, ErrorAt> where P: Fn(char) -> bool {
		let mut out = String::new();
		while let Some(chr) = self.peek_char()? {
			if !p(chr) {
				break;
			}
			out.push(chr);
			self.read_char()?;
		}
		Ok(out)
	}

	pub fn read_string_while_and_skip<P, S>(&mut self, read_while: P, skip_if: S) -> Result<String, ErrorAt> where P: Fn(char) -> bool, S: Fn(char) -> bool {
		let mut out = String::new();
		while let Some(chr) = self.peek_char()? {
			let do_skip = skip_if(chr);
			if !(read_while(chr) || do_skip) {
				break;
			}
			if !do_skip {
				out.push(chr);
			}
			self.read_char()?;
		}
		Ok(out)
	}

	pub fn read_and_expect_char(&mut self, char_to_expect: char) -> Result<bool, ErrorAt> {
		if self.peek_char()? == Some(char_to_expect) {
			self.read_char()?;
			return Ok(true);
		}
		Ok(false)
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