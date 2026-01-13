use std::{fs::File, io::{self, BufReader, Bytes, Read}, num::NonZeroUsize, path::Path};

use crate::error::Error;

pub struct SourceFileReader<'a> {
	reader: Bytes<BufReader<File>>,
	path: &'a Path,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl<'a> SourceFileReader<'a> {
	/// Create a new source reader from opening a file at a path.
	pub fn new(path: &'a Path) -> Result<Self, Error> {
		// Open file
		let file = File::open(path).map_err(|err| Error::UnableToOpenFile(path.to_string_lossy().into(), err))?;
		let reader = BufReader::new(file).bytes();
		// Pack into struct
		Ok(Self {
			reader,
			path,
			line: 1.try_into().unwrap(),
			column: 1.try_into().unwrap(),
		})
	}

	pub fn peek_char(&mut self) -> Result<Option<char>, Error> {
		self.reader
		//// Read to buffer
		//let mut buffer: [u8; 4] = [0; 4];
		//let bytes_read = self.reader.read(&mut buffer).map_err(|err| Error::UnableToReadFile(self.path.to_string_lossy().into(), err))?;
		//// Rewind to before the read
		//self.reader.seek_relative(-(bytes_read as i64)).map_err(|err| Error::UnableToReadFile(self.path.to_string_lossy().into(), err))?;
		//// Return if we have reached the end of file or a null byte
		//if bytes_read == 0 || buffer[0] == 0 {
		//	return Ok(None);
		//}
		//// Convert bytes to char and return
		//let chr = buffer.utf8_chunks().next().ok_or_else(|| Error::InvalidUtf8(self.path.to_string_lossy().into(), self.line, self.column))?.valid();
		//Ok(Some(chr.chars().next().ok_or_else(|| Error::InvalidUtf8(self.path.to_string_lossy().into(), self.line, self.column))?))
		todo!()
	}
}

struct Utf8Iter {
	bytes: Bytes<BufReader<File>>,
}

impl Iterator for Utf8Iter {
	type Item = Result<char, io::Error>;

	fn next(&mut self) -> Option<Result<char, io::Error>> {
		let first_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(error)),
			Ok(first_byte) => first_byte,
		};
		if first_byte < 0x80 {
			return Some(Ok(first_byte as char));
		}

		let second_byte = match self.bytes.next()? {
			Err(error) => return Some(Err(error)),
			Ok(second_byte) => second_byte,
		};
		if second_byte < 0x80 {
			return Some(Ok(first_byte as char));
		}
		todo!()
	}
}