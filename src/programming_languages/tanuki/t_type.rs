use std::{collections::BTreeMap, fmt::{self, Debug, Formatter}, hash::{DefaultHasher, Hash, Hasher}};

use crate::{error::{Error, ErrorAt}, traits::ast_node::AstNode};

#[derive(Clone, PartialEq, Eq, Hash)]
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
	FunctionPointer(FunctionPointerType),
	FunctionPointerEnum(Box<[FunctionPointerType]>),
	ConcreteFunctionPointer(FunctionPointerType),
	Struct { ordered_members: Box<[TanukiType]>, named_members: BTreeMap<Box<str>, TanukiType> },
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

	/// Returns if the type is concrete, that is, it is not a generic type such as @any.
	pub fn is_concrete(&self) -> bool {
		match self {
			Self::CompileTimeChar | Self::CompileTimeFloat | Self::CompileTimeInt | Self::CompileTimeString | Self::Type |
			Self::U(_) | Self::F(_) | Self::I(_) | Self::Bool | Self::Void | Self::ConcreteFunctionPointer(_) => true,
			Self::Any => false,
			Self::FunctionPointer(types) =>
				types.parameter_types.iter().all(|parameter_type| parameter_type.is_concrete()) && types.return_type.is_concrete(),
			Self::FunctionPointerEnum(types) =>
				types.iter().all(|types| types.parameter_types.iter().all(|parameter_type| parameter_type.is_concrete()) && types.return_type.is_concrete()),
			Self::Pointer(pointee_type) => pointee_type.is_concrete(),
			Self::Struct { ordered_members, named_members, .. } =>
				ordered_members.iter().all(|t_type| t_type.is_concrete()) && named_members.iter().all(|(_, t_type)| t_type.is_concrete()),
			Self::TypeEnum(types) => types.iter().all(|t_type| t_type.is_concrete()),
		}
	}

	/// Returns if the type can exist at run time.
	pub fn can_exist_at_run_time(&self) -> bool {
		match self {
			Self::U(_) | Self::F(_) | Self::I(_) | Self::Bool | Self::Void => true,
			Self::Any | Self::CompileTimeChar | Self::CompileTimeFloat | Self::CompileTimeInt | Self::CompileTimeString | Self::Type | Self::FunctionPointer(_) | Self::FunctionPointerEnum(_) => false,
			Self::ConcreteFunctionPointer(types) => types.parameter_types.iter().all(|parameter_type| parameter_type.can_exist_at_run_time()) && types.return_type.can_exist_at_run_time(),
			Self::Pointer(pointee_type) => pointee_type.can_exist_at_run_time(),
			Self::Struct { ordered_members, named_members, .. } =>
				ordered_members.iter().all(|t_type| t_type.can_exist_at_run_time()) && named_members.iter().all(|(_, t_type)| t_type.can_exist_at_run_time()),
			Self::TypeEnum(types) => types.iter().all(|t_type| t_type.can_exist_at_run_time()),
		}
	}

	pub fn get_c_name(&self) -> Result<Box<str>, ErrorAt> {
		Ok(match self {
			Self::U(bit_width) => format!("u{bit_width}").into(),
			Self::I(bit_width) => format!("i{bit_width}").into(),
			Self::F(bit_width) => format!("f{bit_width}").into(),
			Self::Bool => "bool".into(),
			Self::Void => "void".into(),
			Self::Pointer(pointee_type) => format!("p_{}", pointee_type.get_c_name()?).into(),
			Self::Struct { ordered_members, named_members } => {
				let mut hash = DefaultHasher::new();
				ordered_members.hash(&mut hash);
				named_members.hash(&mut hash);
				format!("struct_{}", hash.finish()).into()
			},
			Self::TypeEnum(types) => {
				let mut hash = DefaultHasher::new();
				types.hash(&mut hash);
				format!("enum_{}", hash.finish()).into()
			},
			Self::ConcreteFunctionPointer(types) | Self::FunctionPointer(types) => {
				let mut hash = DefaultHasher::new();
				types.parameter_types.hash(&mut hash);
				types.return_type.hash(&mut hash);
				format!("fnp_{}", hash.finish()).into()
			},
			Self::Any | Self::CompileTimeChar | Self::CompileTimeFloat | Self::CompileTimeInt | Self::CompileTimeString | Self::Type | Self::FunctionPointer(_) | Self::FunctionPointerEnum(_) =>
				{
					println!("{:?}", self);
					return Err(Error::TypeCannotExistAtRunTime.at(None, None, None))
				},
		})
	}
}

impl AstNode for TanukiType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt             => write!(f, "Compile Time Integer"),
			Self::CompileTimeFloat           => write!(f, "Compile Time Float"),
			Self::CompileTimeChar            => write!(f, "Compile Time Char"),
			Self::CompileTimeString          => write!(f, "Compile Time String"),
			Self::Void                       => write!(f, "Void"),
			Self::U(bit_width)          => write!(f, "U{bit_width}"),
			Self::I(bit_width)          => write!(f, "I{bit_width}"),
			Self::F(bit_width)          => write!(f, "F{bit_width}"),
			Self::Bool                       => write!(f, "Bool"),
			Self::Any                        => write!(f, "Any"),
			Self::Type                       => write!(f, "Type"),
			Self::Pointer(_)                 => write!(f, "Pointer"),
			Self::FunctionPointer(_)         => write!(f, "Function Pointer"),
			Self::FunctionPointerEnum(_)     => write!(f, "Function Pointer Enum"),
			Self::ConcreteFunctionPointer(_) => write!(f, "Concrete Function Pointer"),
			Self::Struct { .. }              => write!(f, "Struct"),
			Self::TypeEnum(_)                => write!(f, "Type Enum"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt | Self::CompileTimeFloat | Self::CompileTimeChar | Self::CompileTimeString | Self::Void | Self::U(_) | Self::I(_) | Self::F(_) |
			Self::Any | Self::Type | Self::Bool => Ok(()),
			Self::FunctionPointer(function_pointer_type) | Self::ConcreteFunctionPointer(function_pointer_type) => function_pointer_type.print(level, f),
			Self::FunctionPointerEnum(function_pointer_types) => {
				for function_pointer_type in function_pointer_types.iter() {
					function_pointer_type.print(level, f)?;
				}
				Ok(())
			},
			Self::Pointer(pointee_type) => pointee_type.print(level, f),
			Self::Struct { ordered_members, named_members, .. } => {
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionPointerType {
	pub return_type: Box<TanukiType>,
	pub parameter_types: Box<[TanukiType]>,
}

impl AstNode for FunctionPointerType {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Function Pointer")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for parameter_type in self.parameter_types.iter() {
			parameter_type.print(level, f)?;
		}
		self.return_type.print(level, f)
	}
}