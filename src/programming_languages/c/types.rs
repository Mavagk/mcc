use std::fmt::{self, Formatter};

use crate::traits::{ast_node::AstNode, types::Type};

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
}