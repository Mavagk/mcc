use std::fmt::{self, Formatter};

use crate::{programming_languages::c::{l_value::LValue, types::CType}, traits::{ast_node::AstNode, expression::Expression}};

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
}