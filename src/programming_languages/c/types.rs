use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, traits::{ast_node::AstNode, types::Type}};

#[derive(Debug)]
pub enum CType {
	Void,
	Int,
	U8,
	U16,
	U32,
	U64,
	USize,
	I8,
	I16,
	I32,
	I64,
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
			Self::U16 => write!(f, "U16"),
			Self::U32 => write!(f, "U32"),
			Self::U64 => write!(f, "U64"),
			Self::USize => write!(f, "USize"),
			Self::I8 => write!(f, "I8"),
			Self::I16 => write!(f, "I16"),
			Self::I32 => write!(f, "I32"),
			Self::I64 => write!(f, "I64"),
			CType::PointerTo(_) => write!(f, "Pointer To"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => Ok(()),
			Self::Int => Ok(()),
			Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::I8 | Self::I16 | Self::I32 | Self::I64 => Ok(()),
			Self::USize => Ok(()),
			Self::PointerTo(pointee_type) => pointee_type.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Void => writer.write_all(b"void").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::Int => writer.write_all(b"int").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U8 => writer.write_all(b"uint8_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U16 => writer.write_all(b"uint16_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U32 => writer.write_all(b"uint32_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U64 => writer.write_all(b"uint64_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::USize => writer.write_all(b"size_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I8 => writer.write_all(b"int8_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I16 => writer.write_all(b"int16_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I32 => writer.write_all(b"int32_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I64 => writer.write_all(b"int64_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::PointerTo(pointee) => {
				//writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				pointee.write_to_file(writer, indentation_level)?;
				writer.write_all(b"*").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}