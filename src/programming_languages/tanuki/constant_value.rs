use std::fmt::{self, Formatter};

use num::BigInt;

use crate::traits::ast_node::AstNode;

#[derive(Debug, Clone)]
pub enum TanukiConstantValue {
	CompileTimeInt(BigInt),
	CompileTimeFloat(f64),
	CompileTimeBool(bool),
	CompileTimeChar(char),
	CompileTimeString(Box<str>),
	Void,
	U(u8, u64),
	I(u8, i64),
	F(u8, f64),
}

impl TanukiConstantValue {
	pub fn get_type(&self) -> TanukiType {
		match self {
			Self::CompileTimeInt(_)     => TanukiType::CompileTimeInt,
			Self::CompileTimeFloat(_)   => TanukiType::CompileTimeFloat,
			Self::CompileTimeBool(_)    => TanukiType::CompileTimeBool,
			Self::CompileTimeChar(_)    => TanukiType::CompileTimeChar,
			Self::CompileTimeString(_)  => TanukiType::CompileTimeString,
			Self::Void                  => TanukiType::Void,
			Self::U(bit_width, _)  => TanukiType::U(*bit_width),
			Self::I(bit_width, _)  => TanukiType::I(*bit_width),
			Self::F(bit_width, _)  => TanukiType::F(*bit_width),
		}
	}
}

impl AstNode for TanukiConstantValue {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(value)      => write!(f, "Compile Time Integer {value}"),
			Self::CompileTimeFloat(value)       => write!(f, "Compile Time Float {value}"),
			Self::CompileTimeBool(value)       => write!(f, "Compile Time Bool {value}"),
			Self::CompileTimeChar(value)       => write!(f, "Compile Time Char {value:?}"),
			Self::CompileTimeString(value) => write!(f, "Compile Time String {value:?}"),
			Self::Void                                => write!(f, "Void"),
			Self::U(bit_width, value)      => write!(f, "U{bit_width} {value}"),
			Self::I(bit_width, value)      => write!(f, "I{bit_width} {value}"),
			Self::F(bit_width, value)      => write!(f, "F{bit_width} {value}"),
		}
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(_) | Self::CompileTimeFloat(_) | Self::CompileTimeBool(_) | Self::CompileTimeChar(_) | Self::CompileTimeString(_) | Self::Void | Self::U(_, _) | Self::I(_, _) | Self::F(_, _) => Ok(()),
		}
	}
}

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
	Any,
}

impl AstNode for TanukiType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt    => write!(f, "Compile Time Integer"),
			Self::CompileTimeFloat  => write!(f, "Compile Time Float"),
			Self::CompileTimeBool   => write!(f, "Compile Time Bool"),
			Self::CompileTimeChar   => write!(f, "Compile Time Char"),
			Self::CompileTimeString => write!(f, "Compile Time String"),
			Self::Void              => write!(f, "Void"),
			Self::U(bit_width) => write!(f, "U{bit_width}"),
			Self::I(bit_width) => write!(f, "I{bit_width}"),
			Self::F(bit_width) => write!(f, "F{bit_width}"),
			Self::Any               => write!(f, "Any"),
		}
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt | Self::CompileTimeFloat | Self::CompileTimeBool | Self::CompileTimeChar | Self::CompileTimeString | Self::Void | Self::U(_) | Self::I(_) | Self::F(_) | Self::Any => Ok(()),
		}
	}
}