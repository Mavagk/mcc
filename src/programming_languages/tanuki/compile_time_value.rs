use std::{fmt::{self, Debug, Formatter}, path::Path, u64};

use num::{BigInt, Signed};

use crate::{error::Error, programming_languages::tanuki::t_type::TanukiType, traits::ast_node::AstNode};

#[derive(Clone, PartialEq)]
/// A value that is known at compile time.
pub enum TanukiCompileTimeValue {
	// Values that can only exist an compile time
	CompileTimeInt(BigInt),
	CompileTimeFloat(f64),
	CompileTimeBool(bool),
	CompileTimeChar(char),
	CompileTimeString(Box<str>),
	Type(TanukiType),
	Void,
	// Values that can exist at compile time
	U(u8, u64),
	I(u8, i64),
	F(u8, f64),
	Bool(bool),
	FunctionPointer(Box<str>, Box<Path>, Box<TanukiType>, Box<[TanukiType]>),
	LinkedFunctionPointer(Box<str>, Box<TanukiType>, Box<[TanukiType]>),
}

impl TanukiCompileTimeValue {
	/// Returns the type of this value.
	pub fn get_type(&self) -> TanukiType {
		match self {
			Self::CompileTimeInt(_)                                                                            => TanukiType::CompileTimeInt,
			Self::CompileTimeFloat(_)                                                                          => TanukiType::CompileTimeFloat,
			Self::CompileTimeBool(_)                                                                           => TanukiType::CompileTimeBool,
			Self::CompileTimeChar(_)                                                                           => TanukiType::CompileTimeChar,
			Self::CompileTimeString(_)                                                                         => TanukiType::CompileTimeString,
			Self::Void                                                                                         => TanukiType::Void,
			Self::U(bit_width, _)                                                                         => TanukiType::U(*bit_width),
			Self::I(bit_width, _)                                                                         => TanukiType::I(*bit_width),
			Self::F(bit_width, _)                                                                         => TanukiType::F(*bit_width),
			Self::Bool(_)                                                                                      => TanukiType::Bool,
			Self::Type(_)                                                                                      => TanukiType::Type,
			Self::FunctionPointer(_, _, return_type, parameter_types)    => TanukiType::FunctionPointer(return_type.clone(), parameter_types.clone()),
			Self::LinkedFunctionPointer(_, return_type, parameter_types) => TanukiType::FunctionPointer(return_type.clone(), parameter_types.clone()),
		}
	}

	/// Can this be contained in a variable of a type.
	pub fn is_of_type(&self, t_type: &TanukiType) -> bool {
		match t_type {
			TanukiType::Any => true,
			_ => &self.get_type() == t_type,
		}
	}

	/// Cast this value to a given type.
	/// If `can_be_lossy` is `true` data can be lost in the cast, eg. 3.4 as @int will give 3.
	/// If `can_be_lossy` is `false` data cannot be lost in the cast, eg. 3.4 as @int will throw an error.
	pub fn cast_to(self, type_to: &TanukiType, can_be_lossy: bool) -> Result<Self, Error> {
		let type_from = self.get_type();
		match (type_from, type_to, can_be_lossy) {
			// No cast should happen if we are casting a value to it's own type or @any.
			(type_from, type_to, _) if &type_from == type_to => Ok(self),
			(_, TanukiType::Any, _) => Ok(self),
			// Integer casts
			(TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_), TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_), _) => {
				// Convert the value to a big-int
				let value = match self {
					TanukiCompileTimeValue::CompileTimeInt(value) => value,
					TanukiCompileTimeValue::U(_, value) => value.into(),
					TanukiCompileTimeValue::I(_, value) => value.into(),
					_ => unreachable!(),
				};
				// Convert value to target type
				Ok(match type_to {
					TanukiType::CompileTimeInt => TanukiCompileTimeValue::CompileTimeInt(value),
					TanukiType::U(bit_width) => TanukiCompileTimeValue::U(*bit_width, {
						let max = !(u64::MAX.wrapping_shl(*bit_width as u32));
						let value_u64 = match (&value).try_into() {
							Ok(value) => value,
							Err(_) => return Err(match value.is_positive() {
								true => Error::IntegerTooLargeForType(value, format!("{type_to:?}")),
								false => Error::IntegerTooSmallForType(value, format!("{type_to:?}")),
							}),
						};
						if value_u64 > max {
							return Err(Error::IntegerTooLargeForType(value, format!("{type_to:?}")));
						}
						value_u64
					}),
					TanukiType::I(bit_width) => TanukiCompileTimeValue::I(*bit_width, {
						let max = (!(u64::MAX.wrapping_shl(bit_width.saturating_sub(1) as u32))) as i64;
						let min = (-max) - 1;
						let value_i64 = match (&value).try_into() {
							Ok(value) => value,
							Err(_) => return Err(match value.is_positive() {
								true => Error::IntegerTooLargeForType(value, format!("{type_to:?}")),
								false => Error::IntegerTooSmallForType(value, format!("{type_to:?}")),
							}),
						};
						if value_i64 < min {
							return Err(Error::IntegerTooSmallForType(value, format!("{type_to:?}")));
						}
						if value_i64 > max {
							return Err(Error::IntegerTooLargeForType(value, format!("{type_to:?}")));
						}
						value_i64
					}),
					_ => unreachable!(),
				})
			}
			_ => return Err(Error::NotYetImplemented(format!("Casting value {self:?} to type {type_to:?}"))),
		}
	}
}

impl AstNode for TanukiCompileTimeValue {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(value)                                  => write!(f, "Compile Time Integer {value}"),
			Self::CompileTimeFloat(value)                                   => write!(f, "Compile Time Float {value}"),
			Self::CompileTimeBool(value)                                   => write!(f, "Compile Time Bool {value}"),
			Self::CompileTimeChar(value)                                   => write!(f, "Compile Time Char {value:?}"),
			Self::CompileTimeString(value)                             => write!(f, "Compile Time String {value:?}"),
			Self::Void                                                            => write!(f, "Void"),
			Self::U(bit_width, value)                                  => write!(f, "U{bit_width} {value}"),
			Self::I(bit_width, value)                                  => write!(f, "I{bit_width} {value}"),
			Self::F(bit_width, value)                                  => write!(f, "F{bit_width} {value}"),
			Self::Bool(value)                                              => write!(f, "Bool {value}"),
			Self::Type(_)                                                         => write!(f, "Type"),
			Self::FunctionPointer(name, module_path, _, _) => write!(f, "Function Pointer {name} of {module_path:?}"),
			Self::LinkedFunctionPointer(name, _, _)                    => write!(f, "Linked Function Pointer {name}"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(_) | Self::CompileTimeFloat(_) | Self::CompileTimeBool(_) | Self::CompileTimeChar(_) | Self::CompileTimeString(_) |
			Self::Void | Self::U(_, _) | Self::I(_, _) | Self::F(_, _) | Self::Bool(_) => Ok(()),
			Self::Type(type_t) => type_t.print(level, f),
			Self::FunctionPointer(_, _, return_type, parameter_types) | Self::LinkedFunctionPointer(_, return_type, parameter_types) => {
				return_type.print(level, f)?;
				for parameter_type in parameter_types.iter() {
					parameter_type.print(level, f)?;
				}
				Ok(())
			},
		}
	}
}

impl Debug for TanukiCompileTimeValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print_name(f)
	}
}