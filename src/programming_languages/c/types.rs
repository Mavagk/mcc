use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, traits::{ast_node::AstNode, types::Type}};

#[derive(Debug)]
pub enum CType {
	Void,
	Int,
	U8,
	PointerTo(Box<CType>),
}

impl Type for CType {

}

impl AstNode for CType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => write!(f, "Void"),
			Self::Int => write!(f, "Int"),
			Self::U8 => write!(f, "U8"),
			CType::PointerTo(_) => write!(f, "Pointer To"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => Ok(()),
			Self::Int => Ok(()),
			Self::U8 => Ok(()),
			Self::PointerTo(pointee_type) => pointee_type.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Void => writer.write_all(b"void").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::Int => writer.write_all(b"int").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U8 => writer.write_all(b"uint8_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::PointerTo(pointee) => {
				//writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				pointee.write_to_file(writer, indentation_level)?;
				writer.write_all(b"*").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}