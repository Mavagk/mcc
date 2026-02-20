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
	let mut print_tokens = false;
	let mut print_ast_after_parse = false;
	let mut print_ast_after_post_parse = false;
	let mut print_ast_after_const_compile = false;
	let mut print_source_to_source_c = false;
	let mut execute_interpreted = false;
	let mut is_entrypoint_module = false;
	let mut optimization_level = None;
	let mut do_stop_after_parse = false;
	let mut do_stop_after_const_compile = false;
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
						"-print-tokens" => {
							if print_tokens {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_tokens = true;
						}
						"-print-ast" => {
							if print_ast_after_parse {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_ast_after_parse = true;
						}
						"-print-ast-post-parse" => {
							if print_ast_after_post_parse {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_ast_after_post_parse = true;
						}
						"-print-ast-after-const-compile" => {
							if print_ast_after_const_compile {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_ast_after_const_compile = true;
						}
						"-execute-interpreted" => {
							if execute_interpreted {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							execute_interpreted = true;
						}
						"-entrypoint-module" => {
							if is_entrypoint_module {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							is_entrypoint_module = true;
						}
						"-print-c" => {
							if print_source_to_source_c {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							print_source_to_source_c = true;
						}
						"-stop-after-parse" => {
							if do_stop_after_parse {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							do_stop_after_parse = true;
						}
						"-stop-after-const-compile" => {
							if do_stop_after_const_compile {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							do_stop_after_const_compile = true;
						}
						_ if matches!(arg_str.chars().next(), Some('O')) => {
							if optimization_level.is_some() {
								return Err(Error::RepeatedArgument(arg_str.into()));
							}
							if arg_str.len() > 2 {
								return Err(Error::InvalidOptimizationLevel);
							}
							let level = match arg_str.chars().skip(1).next() {
								None => return Err(Error::InvalidOptimizationLevel),
								Some(chr) => match chr.to_digit(10) {
									None => return Err(Error::InvalidOptimizationLevel),
									Some(level) => level,
								}
							};
							optimization_level = Some(level as u8);
						}
						_ => return Err(Error::InvalidCommandLineArgument(arg_str.into()))
					}
					continue;
				}
				// Else it is a source filepath to add to the list of module main file paths
				let filepath = PathBuf::from(arg_slice).clone().into_boxed_path();
				source_files.push((filepath, is_entrypoint_module));
				is_entrypoint_module = false;
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
	Ok(Arguments {
		source_files: source_files.into_boxed_slice(), home_directory, output_directory, source_directory, output_file,
		print_help, print_version, print_source, print_tokens, print_ast_after_parse, print_ast_after_post_parse, execute_interpreted, print_source_to_source_c, do_stop_after_parse,
		optimization_level, do_stop_after_const_compile, print_ast_after_const_compile,
	})
}

#[derive(Debug)]
pub struct Arguments {
	/// The filepaths for each module and if it is a entrypoint module.
	pub source_files: Box<[(Box<Path>, bool)]>,
	pub home_directory: Option<Box<Path>>,
	pub source_directory: Option<Box<Path>>,
	pub output_directory: Option<Box<Path>>,
	pub output_file: Option<Box<Path>>,
	pub print_help: bool,
	pub print_version: bool,
	pub print_source: bool,
	pub print_tokens: bool,
	pub print_ast_after_parse: bool,
	pub print_ast_after_post_parse: bool,
	pub execute_interpreted: bool,
	pub print_source_to_source_c: bool,
	pub optimization_level: Option<u8>,
	pub do_stop_after_parse: bool,
	pub print_ast_after_const_compile: bool,
	pub do_stop_after_const_compile: bool,
}

enum ParseState {
	Normal,
	HomeDirectory,
	SourceDirectory,
	OutputDirectory,
	OutputFile,
}