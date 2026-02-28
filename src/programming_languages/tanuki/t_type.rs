use std::fmt::{self, Debug, Formatter};

use crate::traits::ast_node::AstNode;

#[derive(Clone, PartialEq, Eq)]
pub enum TanukiType {
	CompileTimeInt,
	CompileTimeFloat,
	CompileTimeBool,
	CompileTimeChar,
	CompileTimeString,
	Void,
	U(u8),
	I(u8),
	F(u8),
	Type,
	Any,
	Function,
	FunctionPointer(Box<TanukiType>, Box<[TanukiType]>),
}

impl AstNode for TanukiType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt        => write!(f, "Compile Time Integer"),
			Self::CompileTimeFloat      => write!(f, "Compile Time Float"),
			Self::CompileTimeBool       => write!(f, "Compile Time Bool"),
			Self::CompileTimeChar       => write!(f, "Compile Time Char"),
			Self::CompileTimeString     => write!(f, "Compile Time String"),
			Self::Void                  => write!(f, "Void"),
			Self::U(bit_width)     => write!(f, "U{bit_width}"),
			Self::I(bit_width)     => write!(f, "I{bit_width}"),
			Self::F(bit_width)     => write!(f, "F{bit_width}"),
			Self::Any                   => write!(f, "Any"),
			Self::Type                  => write!(f, "Type"),
			Self::Function              => write!(f, "Function"),
			Self::FunctionPointer(_, _) => write!(f, "Function Pointer"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt | Self::CompileTimeFloat | Self::CompileTimeBool | Self::CompileTimeChar | Self::CompileTimeString | Self::Void | Self::U(_) | Self::I(_) | Self::F(_) |
			Self::Any | Self::Type | Self::Function => Ok(()),
			Self::FunctionPointer(return_type, parameter_types) => {
				return_type.print(level, f)?;
				for parameter_type in parameter_types.iter() {
					parameter_type.print(level, f)?;
				}
				Ok(())
			},
		}
	}
}

impl Debug for TanukiType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}