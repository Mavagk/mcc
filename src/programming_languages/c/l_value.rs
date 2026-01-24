use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::expression::CExpression, traits::ast_node::AstNode};

#[derive(Debug)]
pub enum CLValue {
	Variable(Box<str>),
	Dereference(Box<CExpression>),
}

impl AstNode for CLValue {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(name) => write!(f, "Variable \"{name}\""),
			Self::Dereference(_) => write!(f, "Dereference"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Variable(_) => Ok(()),
			Self::Dereference(pointer) => pointer.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Variable(name) => writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::Dereference(pointer) => {
				writer.write_all(b"(*").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				pointer.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}