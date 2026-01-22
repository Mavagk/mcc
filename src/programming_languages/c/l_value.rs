use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, traits::ast_node::AstNode};

#[derive(Debug)]
pub enum LValue {
	Variable(Box<str>),
}

impl AstNode for LValue {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(name) => write!(f, "Variable \"{name}\""),
		}
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(_) => Ok(())
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, _indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Variable(name) => writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
		}
	}
}