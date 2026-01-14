use std::{collections::HashSet, env::{args_os, current_dir}, ffi::OsString, mem::take, path::{Path, PathBuf}};

use crate::{arguments::{Arguments, parse_arguments}, error::Error, source_file_reader::SourceFileReader};

pub mod traits;
pub mod arguments;
pub mod error;
pub mod source_file_reader;

fn main() {
	// Get and parse program arguments
	let args: Box<[OsString]> = args_os().skip(1).collect();
	let mut args = match parse_arguments(&args) {
		Ok(args) => args,
		Err(error) => {
			println!("Error while reading arguments: {error}.");
			return;
		}
	};
	//
	let mut main_struct = match Main::new(&mut args) {
		Err(error) => {
			println!("Error while reading arguments: {error}.");
			return;
		}
		Ok(main_struct) => main_struct,
	};
	// Print version if commanded to do so
	if args.print_version {
		println!("Version: {}", env!("CARGO_PKG_VERSION"));
	}
	// Print help if commanded to do so
	if args.print_help {
		println!("Help:");
		println!("<filename>\t\t\t\t\t\tCompiles a module with the main input file specified, path will be <home path>/<source path>/<filename>.");
		println!("-h <directory path>, --home-dir <directory path>\tSets the home path.");
		println!("-s <directory path>, --source-dir <directory path>\tSets the source path.");
		println!("-O <directory path>, --output-dir <directory path>\tSets the output path.");
		println!("-o <filename>, --output-file <directory path>\t\tSets the filepath of the compiled binary, path will be <home path>/<output path>/<filename>.");
		println!("-print-source\t\t\t\t\t\tPrints out each processed source file.");
	}
	// Process each module.
	loop {
		// Remove module from "to process" and add it to "processed".
		let module_path = match main_struct.modules_to_compile.iter().next() {
			Some(module_path) => module_path,
			None => break,
		}.clone();
		main_struct.modules_to_compile.remove(&module_path);
		main_struct.modules_compiled.insert(module_path.clone());
		// Process
		if let Err(error) = process_module(&mut main_struct, &module_path) {
			println!("Error while compiling file \"{}\": {error}.", module_path.to_string_lossy());
			return;
		}
	}
}

pub struct Main {
	modules_to_compile: HashSet<Box<Path>>,
	/// Modules that have been compiled or are being compiled right now.
	modules_compiled: HashSet<Box<Path>>,
	pub print_source: bool,
	pub home_directory: Box<Path>,
	pub source_directory: Box<Path>,
	pub output_directory: Box<Path>,
}

impl Main {
	fn new(arguments: &mut Arguments) -> Result<Self, Error> {
		// Get directories
		let home_directory = match take(&mut arguments.home_directory) {
			Some(home_directory) => home_directory,
			None => match current_dir() {
				Ok(home_directory) => home_directory,
				Err(_) => return Err(Error::NoHomePath),
			}.into_boxed_path()
		};
		let mut source_directory: PathBuf = home_directory.clone().into();
		match &arguments.source_directory {
			Some(source_directory_argument) => source_directory.push(source_directory_argument),
			None => source_directory.push("src"),
		}
		let mut output_directory: PathBuf = home_directory.clone().into();
		match &arguments.output_directory {
			Some(output_directory_argument) => output_directory.push(output_directory_argument),
			None => output_directory.push("bin"),
		}

		Ok(Self {
			modules_to_compile: arguments.source_files.iter().cloned().collect(),
			modules_compiled: HashSet::new(),
			print_source: arguments.print_source,
			home_directory,
			source_directory: source_directory.into_boxed_path(),
			output_directory: output_directory.into_boxed_path(),
		})
	}

	/// Adds a module to be compiled if it has not already been added to the list or has been compiled.
	pub fn add_module_to_compile(&mut self, module_file_path: Box<Path>) {
		if self.modules_compiled.contains(&module_file_path) {
			return;
		}
		self.modules_to_compile.insert(module_file_path);
	}
}

fn process_module(main: &mut Main, module_path: &Path) -> Result<(), Error> {
	// Get source filepath
	let mut filepath: PathBuf = main.source_directory.clone().into();
	filepath.push(module_path);
	// Open file
	let _file = SourceFileReader::new(&filepath)?;
	// TODO
	Ok(())
}