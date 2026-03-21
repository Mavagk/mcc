use std::{collections::HashMap, fmt::{self, Debug, Formatter}};

use crate::{error::Error, traits::ast_node::AstNode};

#[derive(Clone, PartialEq, Eq)]
/// A type of a Tanuki value.
pub enum TanukiType {
	CompileTimeInt,
	CompileTimeFloat,
	CompileTimeChar,
	CompileTimeString,
	Type,
	Any,
	Void,
	U(u8),
	I(u8),
	F(u8),
	Bool,
	Pointer(Box<TanukiType>),
	FunctionPointer(Box<TanukiType>, Box<[TanukiType]>),
	FunctionPointerEnum(Box<[(Box<TanukiType>, Box<[TanukiType]>)]>),
	Struct { ordered_members: Box<[TanukiType]>, named_members: HashMap<Box<str>, TanukiType> },
	TypeEnum(Box<[TanukiType]>),
}

impl TanukiType {
	/// Gives the type of a value of this type after it has been cast to another type.
	pub fn cast_to(&self, type_to: &TanukiType) -> Result<Self, Error> {
		Ok(match (self, type_to) {
			// Cast a struct of types to a type of struct
			(TanukiType::Struct { ordered_members, named_members }, TanukiType::Type) => TanukiType::Struct {
				ordered_members: ordered_members.clone(), named_members: named_members.clone()
			},
			(type_t, TanukiType::Any) => type_t.clone(),
			(TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_), TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_)) => type_to.clone(),
			(TanukiType::Any, type_t) => type_t.clone(),
			(cast_from, cast_to) if cast_from == cast_to => cast_from.clone(),
			_ => return Err(Error::NotYetImplemented(format!("Casting value {self:?} to type {type_to:?}"))),
		})
	}
}

impl AstNode for TanukiType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt         => write!(f, "Compile Time Integer"),
			Self::CompileTimeFloat       => write!(f, "Compile Time Float"),
			Self::CompileTimeChar        => write!(f, "Compile Time Char"),
			Self::CompileTimeString      => write!(f, "Compile Time String"),
			Self::Void                   => write!(f, "Void"),
			Self::U(bit_width)      => write!(f, "U{bit_width}"),
			Self::I(bit_width)      => write!(f, "I{bit_width}"),
			Self::F(bit_width)      => write!(f, "F{bit_width}"),
			Self::Bool                   => write!(f, "Bool"),
			Self::Any                    => write!(f, "Any"),
			Self::Type                   => write!(f, "Type"),
			Self::Pointer(_)             => write!(f, "Pointer"),
			Self::FunctionPointer(_, _)  => write!(f, "Function Pointer"),
			Self::FunctionPointerEnum(_) => write!(f, "Function Pointer Enum"),
			Self::Struct { .. }          => write!(f, "Struct"),
			Self::TypeEnum(_)            => write!(f, "Type Enum"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt | Self::CompileTimeFloat | Self::CompileTimeChar | Self::CompileTimeString | Self::Void | Self::U(_) | Self::I(_) | Self::F(_) |
			Self::Any | Self::Type | Self::Bool => Ok(()),
			Self::FunctionPointer(return_type, parameter_types) => {
				for parameter_type in parameter_types.iter() {
					parameter_type.print(level, f)?;
				}
				return_type.print(level, f)
			},
			Self::FunctionPointerEnum(types) => {
				for (return_type, parameter_types) in types.iter() {
					for parameter_type in parameter_types.iter() {
						parameter_type.print(level, f)?;
					}
					return_type.print(level, f)?;
					writeln!(f)?;
				}
				Ok(())
			},
			Self::Pointer(pointee_type) => pointee_type.print(level, f),
			Self::Struct { ordered_members, named_members } => {
				for ordered_member in ordered_members.iter() {
					ordered_member.print(level, f)?;
				}
				for (_named_member_name, named_member) in named_members.iter() {
					named_member.print(level, f)?;
				}
				Ok(())
			}
			Self::TypeEnum(types) => {
				for t_type in types.iter() {
					t_type.print(level, f)?;
				}
				Ok(())
			}
		}
	}
}

impl Debug for TanukiType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}