use std::{ffi::OsString, path::{Path, PathBuf}};

use crate::error::Error;

pub fn parse_arguments(args: &[OsString]) -> Result<Box<[Argument]>, Error> {
	let mut parse_state = ParseState::Normal;
	let mut out = Vec::new();
	for arg in args {
		//if arg.is_empty() {
		//	continue;
		//}
		match parse_state {
			ParseState::Normal => {
				let arg_str = arg.to_string_lossy();
				let arg_slice = arg.as_os_str();
				if arg_str.starts_with('-') {
					todo!()
				}
				let path = PathBuf::from(arg_slice);
				let stem = path.file_stem().ok_or(Error::InvalidSourcePath)?.to_str().ok_or(Error::InvalidSourcePath)?;
				out.push(Argument::SourceFile { name: Box::, full_path: path.into_boxed_path(), full_path_without_extension: () });
			}
		}
	}
	Ok(out.into_boxed_slice())
}

enum ParseState {
	Normal,
}

pub enum Argument {
	SourceFile{ name: Box<str>, full_path: Box<Path>, full_path_without_extension: Box<Path> },
}