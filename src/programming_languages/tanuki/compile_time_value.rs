use std::fmt::{self, Formatter};

use num::BigInt;

use crate::{error::ErrorAt, programming_languages::tanuki::t_type::TanukiType, traits::ast_node::AstNode};

#[derive(Debug, Clone, PartialEq)]
pub enum TanukiCompileTimeValue {
	CompileTimeInt(BigInt),
	CompileTimeFloat(f64),
	CompileTimeBool(bool),
	CompileTimeChar(char),
	CompileTimeString(Box<str>),
	Void,
	U(u8, u64),
	I(u8, i64),
	F(u8, f64),
	Type(TanukiType),
}

impl TanukiCompileTimeValue {
	pub fn get_type(&self) -> TanukiType {
		match self {
			Self::CompileTimeInt(_)    => TanukiType::CompileTimeInt,
			Self::CompileTimeFloat(_)  => TanukiType::CompileTimeFloat,
			Self::CompileTimeBool(_)   => TanukiType::CompileTimeBool,
			Self::CompileTimeChar(_)   => TanukiType::CompileTimeChar,
			Self::CompileTimeString(_) => TanukiType::CompileTimeString,
			Self::Void                 => TanukiType::Void,
			Self::U(bit_width, _) => TanukiType::U(*bit_width),
			Self::I(bit_width, _) => TanukiType::I(*bit_width),
			Self::F(bit_width, _) => TanukiType::F(*bit_width),
			Self::Type(_)              => TanukiType::Type,
		}
	}

	pub fn is_of_type(&self, t_type: &TanukiType) -> bool {
		match t_type {
			TanukiType::Any => true,
			_ => &self.get_type() == t_type,
		}
	}

	pub fn cast_to(self, t_type: &TanukiType, is_explicit: bool) -> Result<Self, ErrorAt> {
		match (self, t_type, is_explicit) {
			(value, type_t, _) if value.is_of_type(t_type) => Ok(value),
			_ => todo!()
		}
	}
}

impl AstNode for TanukiCompileTimeValue {
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
			Self::Type(_)                             => write!(f, "Type"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(_) | Self::CompileTimeFloat(_) | Self::CompileTimeBool(_) | Self::CompileTimeChar(_) | Self::CompileTimeString(_) | Self::Void | Self::U(_, _) | Self::I(_, _) | Self::F(_, _) => Ok(()),
			Self::Type(type_t) => type_t.print(level, f),
		}
	}
}