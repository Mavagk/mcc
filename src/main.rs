use std::{env::args_os, ffi::OsString};

use crate::arguments::parse_arguments;

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
	// Print version if commanded to do so
	if args.print_version {
		println!("Version: {}", env!("CARGO_PKG_VERSION"));
	}
	// Print help if commanded to do so
	if args.print_help {
		println!("Help:");
		println!("<filename>\t\t\t\t\t\tCompiles the file, path will be <home path>/<source path>/<filename>.");
		println!("-h <directory path>, --home-dir <directory path>\tSets the home path.");
		println!("-s <directory path>, --source-dir <directory path>\tSets the source path.");
		println!("-O <directory path>, --output-dir <directory path>\tSets the output path.");
		println!("-o <filename>, --output-file <directory path>\t\tSets the filepath of the compiled binary, path will be <home path>/<output path>/<filename>.");
	}
	// TODO
	println!("{args:?}");
}