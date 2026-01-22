use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{statement::CCompoundStatement, types::CType}, traits::{ast_node::AstNode, module_element::ModuleElement}};

#[derive(Debug)]
pub enum CModuleElement {
	FunctionDefinition { return_type: CType, name: Box<str>, parameters: Box<[CFunctionParameter]>, body: Box<CCompoundStatement> },
	AngleInclude(Box<str>),
	DoubleQuotesInclude(Box<str>),
}

impl ModuleElement for CModuleElement {

}

impl AstNode for CModuleElement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::FunctionDefinition { name, .. } => write!(f, "Function Definition \"{name}\""),
			Self::AngleInclude(name) => write!(f, "Include <{name}>"),
			Self::DoubleQuotesInclude(name) => write!(f, "Include \"{name}\""),
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
			Self::AngleInclude(_) | Self::DoubleQuotesInclude(_) => Ok(()),
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
			Self::AngleInclude(name) => {
				writer.write_all(b"#include <").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b">").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::DoubleQuotesInclude(name) => {
				writer.write_all(b"#include \"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}

#[derive(Debug)]
pub struct CFunctionParameter {
	param_type: CType,
	name: Box<str>,
}

impl AstNode for CFunctionParameter {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Function Parameter \"{}\"", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		self.param_type.print(level, f)
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		self.param_type.write_to_file(writer, indentation_level)?;
		writer.write_all(self.name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
	}
}