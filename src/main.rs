use std::{collections::HashSet, env::args_os, ffi::OsString, path::{Path, PathBuf}};

use crate::arguments::{Arguments, parse_arguments};

pub mod traits;
pub mod arguments;
pub mod error;

fn main() {
	// Get and parse program arguments
	let args: Box<[OsString]> = args_os().skip(1).collect();
	let args = match parse_arguments(&args) {
		Ok(args) => args,
		Err(error) => {
			println!("Error while reading arguments: {error}.");
			return;
		}
	};
	//
	let mut main_struct = Main::new(&args);
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
	}
	// TODO
	println!("{args:?}");
}

pub struct Main {
	modules_to_compile: HashSet<Box<Path>>,
	modules_compiled: HashSet<Box<Path>>,
}

impl Main {
	pub fn new(arguments: &Arguments) -> Self {
		Self {
			modules_to_compile: arguments.source_files.iter().cloned().collect(),
			modules_compiled: HashSet::new(),
		}
	}

	pub fn add_module_to_compile(&mut self, module: Box<Path>) {
		if self.modules_compiled.contains(&module) {
			return;
		}
		self.modules_to_compile.insert(module);
	}
}