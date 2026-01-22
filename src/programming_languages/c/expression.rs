use std::{fmt::{self, Formatter}, fs::File, io::BufWriter};

use crate::{error::ErrorAt, programming_languages::c::{l_value::LValue, types::CType}, traits::{ast_node::AstNode, expression::Expression}};

#[derive(Debug)]
pub enum CExpression {
	Assignment(Box<LValue>, Box<CExpression>),
	LValueRead(Box<LValue>),
	FunctionCall(Box<str>, Box<[CExpression]>),
	IntConstant(i128),
	Sizeof(CType),
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
			Self::Sizeof(_) => write!(f, "Sizeof"),
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
			Self::IntConstant(_) => Ok(()),
			Self::Sizeof(sub_type) => sub_type.print(level, f),
		}
	}

	fn write_to_file(&self, _writer: &mut BufWriter<File>) -> Result<(), ErrorAt> {
		Ok(())
		//match self {
		//	Self::FunctionDefinition { return_type, name, parameters, body } => {
		//		return_type.write_to_file(writer)?;
		//		writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		//		writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		//		writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		//		let mut is_first_parameter = true;
		//		for parameter in parameters {
		//			if !is_first_parameter {
		//				writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		//			}
		//			parameter.write_to_file(writer)?;
		//			is_first_parameter = false;
		//		}
		//		writer.write_all(b") ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		//		body.write_to_file(writer)
		//	}
		//}
	}
}