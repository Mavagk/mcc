use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::module_element::CTypeAndName, traits::ast_node::AstNode};

#[derive(Debug)]
pub enum CType {
	Void,
	Int,
	Bool,
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
	FunctionPointer(Box<CType>, Box<[CType]>),
	Struct(Box<[CTypeAndName]>),
	NamedType(Box<str>),
}

impl AstNode for CType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void                        => write!(f, "Void"),
			Self::Int                         => write!(f, "Int"),
			Self::Bool                        => write!(f, "Bool"),
			Self::U8                          => write!(f, "U8"),
			Self::U16                         => write!(f, "U16"),
			Self::U32                         => write!(f, "U32"),
			Self::U64                         => write!(f, "U64"),
			Self::USize                       => write!(f, "USize"),
			Self::I8                          => write!(f, "I8"),
			Self::I16                         => write!(f, "I16"),
			Self::I32                         => write!(f, "I32"),
			Self::I64                         => write!(f, "I64"),
			Self::PointerTo(_)               => write!(f, "Pointer To"),
			Self::FunctionPointer(_, _)      => write!(f, "Function Pointer"),
			Self::Struct(_)                  => write!(f, "Struct"),
			Self::NamedType(name) => write!(f, "Named \"{name}\""),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => Ok(()),
			Self::Int => Ok(()),
			Self::Bool => Ok(()),
			Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::I8 | Self::I16 | Self::I32 | Self::I64 => Ok(()),
			Self::USize => Ok(()),
			Self::PointerTo(pointee_type) => pointee_type.print(level, f),
			Self::FunctionPointer(return_type, parameter_types) => {
				return_type.print(level, f)?;
				for parameter_type in parameter_types.iter() {
					parameter_type.print(level, f)?;
				}
				Ok(())
			}
			Self::Struct(members) => {
				for member in members {
					member.print(level, f)?;
				}
				Ok(())
			}
			Self::NamedType(_) => Ok(()),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Void => writer.write_all(b"void").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::Int => writer.write_all(b"int").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::Bool => writer.write_all(b"bool").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U8 => writer.write_all(b"uint8_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U16 => writer.write_all(b"uint16_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U32 => writer.write_all(b"uint32_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::U64 => writer.write_all(b"uint64_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::USize => writer.write_all(b"size_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I8 => writer.write_all(b"int8_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I16 => writer.write_all(b"int16_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I32 => writer.write_all(b"int32_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::I64 => writer.write_all(b"int64_t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::PointerTo(pointee) => 'a: {
				if let CType::FunctionPointer(return_type, parameter_types) = &**pointee {
					return_type.write_to_file(writer, indentation_level)?;
					writer.write_all(b" (**)(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					let mut is_first_parameter = true;
					for parameter in parameter_types {
						if !is_first_parameter {
							writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
						}
						parameter.write_to_file(writer, indentation_level)?;
						is_first_parameter = false;
					}
					break 'a writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None));
				}
				pointee.write_to_file(writer, indentation_level)?;
				writer.write_all(b"*").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::FunctionPointer(return_type, parameter_types) => {
				return_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b" (*)(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_parameter = true;
				for parameter in parameter_types {
					if !is_first_parameter {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					parameter.write_to_file(writer, indentation_level)?;
					is_first_parameter = false;
				}
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Struct(members) => {
				writer.write_all(b"struct {").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if !members.is_empty() {
					writer.write_all(b"\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				}
				for member in members {
					for _ in 0..indentation_level + 1 {
						writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					member.write_to_file(writer, indentation_level)?;
				}
				if !members.is_empty() {
					for _ in 0..indentation_level {
						writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
				}
				writer.write_all(b"}").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				Ok(())
			}
			Self::NamedType(name) => writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
		}
	}
}

impl CType {
	pub fn write_to_file_with_name(&self, writer: &mut BufWriter<File>, indentation_level: usize, name: &str) -> Result<(), ErrorAt> {
		match self {
			Self::Void | Self::Int | Self::Bool | Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::USize | Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::NamedType(_) => {
				self.write_to_file(writer, indentation_level)?;
				writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PointerTo(pointee) => 'a: {
				if let CType::PointerTo(pointee_pointee) = &**pointee {
					pointee_pointee.write_to_file(writer, indentation_level)?;
					writer.write_all(b" **").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					break 'a writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None));
				}
				if let CType::FunctionPointer(return_type, parameter_types) = &**pointee {
					return_type.write_to_file(writer, indentation_level)?;
					writer.write_all(b" (**").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					writer.write_all(b")(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					let mut is_first_parameter = true;
					for parameter in parameter_types {
						if !is_first_parameter {
							writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
						}
						parameter.write_to_file(writer, indentation_level)?;
						is_first_parameter = false;
					}
					break 'a writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None));
				}
				pointee.write_to_file(writer, indentation_level)?;
				writer.write_all(b" *").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::FunctionPointer(return_type, parameter_types) => {
				return_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b" (*").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b")(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_parameter = true;
				for parameter in parameter_types {
					if !is_first_parameter {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					parameter.write_to_file(writer, indentation_level)?;
					is_first_parameter = false;
				}
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Struct(members) => {
				writer.write_all(b"struct {").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if !members.is_empty() {
					writer.write_all(b"\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				}
				for member in members {
					for _ in 0..indentation_level + 1 {
						writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					member.write_to_file(writer, indentation_level)?;
					writer.write_all(b";\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				}
				if !members.is_empty() {
					for _ in 0..indentation_level {
						writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
				}
				writer.write_all(b"} ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}