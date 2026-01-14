use std::{ffi::OsString, path::{Path, PathBuf}};

use crate::error::Error;

pub fn parse_arguments(args: &[OsString]) -> Result<Arguments, Error> {
	let mut parse_state = ParseState::Normal;
	let mut source_files = Vec::new();
	let mut home_directory = None;
	let mut source_directory = None;
	let mut output_directory = None;
	let mut output_file = None;
	let mut print_help = false;
	let mut print_version = false;
	let mut print_source = false;
	// Process each argument
	for arg in args {
		match parse_state {
			ParseState::Normal => {
				let arg_str = arg.to_string_lossy();
				let arg_slice = arg.as_os_str();
				// If the argument starts with a dash
				if arg_str.starts_with('-') {
					let arg_str = &arg_str[1..];
					match arg_str {
						"h" | "-home-dir" => parse_state = ParseState::HomeDirectory,
						"O" | "-output-dir" => parse_state = ParseState::OutputDirectory,
						"o" | "-output-file" => parse_state = ParseState::OutputFile,
						"s" | "-source-dir" => parse_state = ParseState::SourceDirectory,
						"-help" => {
							if print_help {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_help = true;
						}
						"-version" => {
							if print_version {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_version = true;
						}
						"-print-source" => {
							if print_source {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_source = true;
						}
						_ => return Err(Error::InvalidCommandLineArgument(arg_str.into()))
					}
					continue;
				}
				// Else it is a source filepath to add to the list of module main file paths
				let filepath = PathBuf::from(arg_slice).clone().into_boxed_path();
				source_files.push(filepath);
			}
			ParseState::HomeDirectory => {
				if home_directory.is_some() {
					return Err(Error::MultipleHomePaths);
				}
				home_directory = Some(PathBuf::from(arg).into_boxed_path());
				parse_state = ParseState::Normal;
			}
			ParseState::OutputDirectory => {
				if output_directory.is_some() {
					return Err(Error::MultipleOutputPaths);
				}
				output_directory = Some(PathBuf::from(arg).into_boxed_path());
				parse_state = ParseState::Normal;
			}
			ParseState::SourceDirectory => {
				if source_directory.is_some() {
					return Err(Error::MultipleSourcePaths);
				}
				source_directory = Some(PathBuf::from(arg).into_boxed_path());
				parse_state = ParseState::Normal;
			}
			ParseState::OutputFile => {
				if output_file.is_some() {
					return Err(Error::MultipleOutputFiles);
				}
				output_file = Some(PathBuf::from(arg).into_boxed_path());
				parse_state = ParseState::Normal;
			}
		}
	}
	// Assemble into arguments struct
	Ok(Arguments { source_files: source_files.into_boxed_slice(), home_directory, output_directory, source_directory, output_file, print_help, print_version, print_source })
}

#[derive(Debug)]
pub struct Arguments {
	pub source_files: Box<[Box<Path>]>,
	pub home_directory: Option<Box<Path>>,
	pub source_directory: Option<Box<Path>>,
	pub output_directory: Option<Box<Path>>,
	pub output_file: Option<Box<Path>>,
	pub print_help: bool,
	pub print_version: bool,
	pub print_source: bool,
}

enum ParseState {
	Normal,
	HomeDirectory,
	SourceDirectory,
	OutputDirectory,
	OutputFile,
}