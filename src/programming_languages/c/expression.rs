use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{l_value::CLValue, statement::CStatement, types::CType}, traits::{ast_node::AstNode, expression::Expression}};

#[derive(Debug)]
pub enum CExpression {
	Assignment(Box<CLValue>, Box<CExpression>),
	LValueRead(Box<CLValue>),
	FunctionCall(Box<str>, Box<[CExpression]>),
	IntConstant(i128),
	StringConstant(Box<str>),
	Sizeof(CType),
	GreaterThanOrEqual(Box<CExpression>, Box<CExpression>),
	Equal(Box<CExpression>, Box<CExpression>),
	Multiply(Box<CExpression>, Box<CExpression>),
	Add(Box<CExpression>, Box<CExpression>),
	Subtract(Box<CExpression>, Box<CExpression>),
	PostfixIncrement(Box<CLValue>),
	PostfixDecrement(Box<CLValue>),
}

impl Expression for CExpression {
	
}

impl AstNode for CExpression {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Assignment(_, _) => write!(f, "Assignment"),
			Self::LValueRead(_) => write!(f, "Read L-Value"),
			Self::FunctionCall(name, _) => write!(f, "Function Call \"{name}\""),
			Self::IntConstant(value) => write!(f, "Int Constant {value}"),
			Self::StringConstant(value) => write!(f, "String Constant \"{value}\""),
			Self::Sizeof(_) => write!(f, "Sizeof"),
			Self::Equal(_, _) => write!(f, "Equal"),
			Self::GreaterThanOrEqual(_, _) => write!(f, "Greater than or Equal"),
			Self::Multiply(_, _) => write!(f, "Multiply"),
			Self::Add(_, _) => write!(f, "Add"),
			Self::Subtract(_, _) => write!(f, "Subtract"),
			Self::PostfixIncrement(_) => write!(f, "Postfix Increment"),
			Self::PostfixDecrement(_) => write!(f, "Postfix Decrement"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Assignment(l_value, sub_expression) => {
				l_value.print(level, f)?;
				sub_expression.print(level, f)
			}
			Self::LValueRead(l_value) => l_value.print(level, f),
			Self::FunctionCall(_, arguments) => {
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			Self::IntConstant(_) | Self::StringConstant(_) => Ok(()),
			Self::Sizeof(sub_type) => sub_type.print(level, f),
			Self::Equal(lhs, rhs) | Self::GreaterThanOrEqual(lhs, rhs) | Self::Multiply(lhs, rhs) | Self::Add(lhs, rhs) | Self::Subtract(lhs, rhs) => {
				lhs.print(level, f)?;
				rhs.print(level, f)
			}
			Self::PostfixIncrement(l_value) | Self::PostfixDecrement(l_value) => l_value.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Assignment(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" = ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::LValueRead(l_value) => l_value.write_to_file(writer, indentation_level),
			Self::FunctionCall(name, arguments) => {
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_argument = true;
				for argument in arguments {
					if !is_first_argument {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					argument.write_to_file(writer, indentation_level)?;
					is_first_argument = false;
				}
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::IntConstant(value) => writer.write_all(format!("{value}").as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::StringConstant(value) => {
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(value.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Sizeof(sub_type) => {
				writer.write_all(b"sizeof(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Equal(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" == ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::GreaterThanOrEqual(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" >= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Multiply(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" * ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Add(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" + ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Subtract(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" - ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PostfixIncrement(l_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b"++)").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PostfixDecrement(l_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b"--)").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}

impl Into<CStatement> for CExpression {
	fn into(self) -> CStatement {
		CStatement::Expression(self)
	}
}