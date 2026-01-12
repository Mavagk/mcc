use std::{env::args_os, ffi::OsString};

use crate::arguments::parse_arguments;

pub mod traits;
pub mod arguments;
pub mod error;

fn main() {
	let args: Box<[OsString]> = args_os().skip(1).collect();
	let args = parse_arguments(&args);
}