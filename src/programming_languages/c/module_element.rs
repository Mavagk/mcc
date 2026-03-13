use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{statement::CCompoundStatement, types::CType}, traits::{ast_node::AstNode, module_element::ModuleElement}};

#[derive(Debug)]
pub enum CModuleElement {
	FunctionDefinition { return_type: CType, name: Box<str>, parameters: Box<[CFunctionParameter]>, body: Box<CCompoundStatement> },
	FunctionDeclaration { return_type: CType, name: Box<str>, parameters: Box<[CFunctionParameter]> },
	AngleIncludeInHeader(Box<str>),
	DoubleQuotesIncludeInHeader(Box<str>),
	AngleIncludeInMain(Box<str>),
	DoubleQuotesIncludeInMain(Box<str>),
}

impl ModuleElement for CModuleElement {

}

impl AstNode for CModuleElement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::FunctionDefinition { name, .. } => write!(f, "Function Definition \"{name}\""),
			Self::FunctionDeclaration { name, .. } => write!(f, "Function Declaration \"{name}\""),
			Self::AngleIncludeInHeader(name) => write!(f, "Include <{name}> in Header"),
			Self::DoubleQuotesIncludeInHeader(name) => write!(f, "Include \"{name}\" in Header"),
			Self::AngleIncludeInMain(name) => write!(f, "Include <{name}> in Main"),
			Self::DoubleQuotesIncludeInMain(name) => write!(f, "Include \"{name}\" in Main"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::FunctionDefinition { return_type, parameters: arguments, body, .. } => {
				return_type.print(level, f)?;
				writeln!(f)?;
				for argument in arguments {
					argument.print(level, f)?;
				}
				body.print(level, f)?;
				Ok(())
			}
			Self::FunctionDeclaration { return_type, parameters: arguments, .. } => {
				return_type.print(level, f)?;
				writeln!(f)?;
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			Self::AngleIncludeInHeader(_) | Self::DoubleQuotesIncludeInHeader(_) | Self::AngleIncludeInMain(_) | Self::DoubleQuotesIncludeInMain(_) => Ok(()),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::FunctionDefinition { return_type, name, parameters, body } => {
				return_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_parameter = true;
				for parameter in parameters {
					if !is_first_parameter {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					parameter.write_to_file(writer, indentation_level)?;
					is_first_parameter = false;
				}
				writer.write_all(b") ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				body.write_to_file(writer, indentation_level)
			}
			Self::AngleIncludeInMain(name) => {
				writer.write_all(b"#include <").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b">").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::DoubleQuotesIncludeInMain(name) => {
				writer.write_all(b"#include \"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::FunctionDeclaration { .. } | Self::AngleIncludeInHeader(..) | Self::DoubleQuotesIncludeInHeader(..) => Ok(())
		}
	}

	fn write_header_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::FunctionDeclaration { return_type, name, parameters } | Self::FunctionDefinition { return_type, name, parameters, .. } => {
				return_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_parameter = true;
				for parameter in parameters {
					if !is_first_parameter {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					parameter.write_to_file(writer, indentation_level)?;
					is_first_parameter = false;
				}
				writer.write_all(b");").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AngleIncludeInHeader(name) => {
				writer.write_all(b"#include <").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b">").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::DoubleQuotesIncludeInHeader(name) => {
				writer.write_all(b"#include \"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AngleIncludeInMain(..) | Self::DoubleQuotesIncludeInMain(..) => Ok(())
		}
	}
}

#[derive(Debug)]
pub struct CFunctionParameter {
	param_type: CType,
	name: Box<str>,
}

impl CFunctionParameter {
	pub fn new(param_type: CType, name: Box<str>) -> Self {
		Self { param_type, name }
	}
}

impl AstNode for CFunctionParameter {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Function Parameter \"{}\"", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		self.param_type.print(level, f)
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		if self.name.is_empty() {
			self.param_type.write_to_file(writer, indentation_level)
		}
		else {
			self.param_type.write_to_file_with_name(writer, indentation_level, &self.name)
		}
	}
}