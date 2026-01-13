use std::{fs::File, io::{BufReader, Read}, num::NonZeroUsize, path::Path};

use crate::error::Error;

pub struct SourceFileReader<'a> {
	reader: BufReader<File>,
	path: &'a Path,
	line: NonZeroUsize,
	column: NonZeroUsize,
}

impl<'a> SourceFileReader<'a> {
	/// Create a new source reader from opening a file at a path.
	pub fn new(path: &'a Path) -> Result<Self, Error> {
		// Open file
		let file = File::open(path).map_err(|err| Error::UnableToOpenFile(path.to_string_lossy().into(), err))?;
		let reader = BufReader::new(file);
		// Pack into struct
		Ok(Self {
			reader,
			path,
			line: 1.try_into().unwrap(),
			column: 1.try_into().unwrap(),
		})
	}

	pub fn peek_char(&mut self) -> Result<Option<char>, Error> {
		// Read to buffer
		let mut buffer: [u8; 4] = [0; 4];
		let bytes_read = self.reader.read(&mut buffer).map_err(|err| Error::UnableToReadFile(self.path.to_string_lossy().into(), err))?;
		// Rewind to before the read
		self.reader.seek_relative(-(bytes_read as i64)).map_err(|err| Error::UnableToReadFile(self.path.to_string_lossy().into(), err))?;
		// Return if we have reached the end of file or a null byte
		if bytes_read == 0 || buffer[0] == 0 {
			return Ok(None);
		}
		// Convert bytes to char and return
		let chr = buffer.utf8_chunks().next().ok_or_else(|| Error::InvalidUtf8(self.path.to_string_lossy().into(), self.line, self.column))?.valid();
		Ok(Some(chr.chars().next().ok_or_else(|| Error::InvalidUtf8(self.path.to_string_lossy().into(), self.line, self.column))?))
	}
}