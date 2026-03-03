use std::{fmt::{self, Formatter}, path::Path, u64};

use num::{BigInt, Signed};

use crate::{error::Error, programming_languages::tanuki::t_type::TanukiType, traits::ast_node::AstNode};

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
	//Function(Box<str>, Box<Path>),
	FunctionPointer(Box<str>, Box<Path>, Box<TanukiType>, Box<[TanukiType]>),
}

impl TanukiCompileTimeValue {
	pub fn get_type(&self) -> TanukiType {
		match self {
			Self::CompileTimeInt(_)                                                                         => TanukiType::CompileTimeInt,
			Self::CompileTimeFloat(_)                                                                       => TanukiType::CompileTimeFloat,
			Self::CompileTimeBool(_)                                                                        => TanukiType::CompileTimeBool,
			Self::CompileTimeChar(_)                                                                        => TanukiType::CompileTimeChar,
			Self::CompileTimeString(_)                                                                      => TanukiType::CompileTimeString,
			Self::Void                                                                                      => TanukiType::Void,
			Self::U(bit_width, _)                                                                      => TanukiType::U(*bit_width),
			Self::I(bit_width, _)                                                                      => TanukiType::I(*bit_width),
			Self::F(bit_width, _)                                                                      => TanukiType::F(*bit_width),
			Self::Type(_)                                                                                   => TanukiType::Type,
			//Self::Function(_, _)                                                                            => TanukiType::Function,
			Self::FunctionPointer(_, _, return_type, parameter_types) => TanukiType::FunctionPointer(return_type.clone(), parameter_types.clone()),
		}
	}

	pub fn is_of_type(&self, t_type: &TanukiType) -> bool {
		match t_type {
			TanukiType::Any => true,
			_ => &self.get_type() == t_type,
		}
	}

	pub fn cast_to(self, type_to: &TanukiType, is_explicit: bool) -> Result<Self, Error> {
		let type_from = self.get_type();
		match (type_from, type_to, is_explicit) {
			(type_from, type_to, _) if &type_from == type_to => Ok(self),
			(_, TanukiType::Any, _) => Ok(self),
			(TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_), TanukiType::CompileTimeInt | TanukiType::U(_) | TanukiType::I(_), _) => {
				let value = match self {
					TanukiCompileTimeValue::CompileTimeInt(value) => value,
					TanukiCompileTimeValue::U(_, value) => value.into(),
					TanukiCompileTimeValue::I(_, value) => value.into(),
					_ => unreachable!(),
				};
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
			_ => {
				println!("{self:?} {type_to:?}");
				todo!()
			}
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
			Self::Type(_)                                                         => write!(f, "Type"),
			//Self::Function(name, module_path)              => write!(f, "Function {name} of {module_path:?}"),
			Self::FunctionPointer(name, module_path, _, _) => write!(f, "Function Pointer {name} of {module_path:?}"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompileTimeInt(_) | Self::CompileTimeFloat(_) | Self::CompileTimeBool(_) | Self::CompileTimeChar(_) | Self::CompileTimeString(_) |
			Self::Void | Self::U(_, _) | Self::I(_, _) | Self::F(_, _)/* | Self::Function(_, _)*/ => Ok(()),
			Self::Type(type_t) => type_t.print(level, f),
			Self::FunctionPointer(_, _, return_type, parameter_types) => {
				return_type.print(level, f)?;
				for parameter_type in parameter_types.iter() {
					parameter_type.print(level, f)?;
				}
				Ok(())
			},
		}
	}
}

/*impl Neg for TanukiCompileTimeValue {
	type Output = Result<Option<TanukiCompileTimeValue>, Error>;

	fn neg(self) -> Self::Output {
		match self {
			TanukiCompileTimeValue::CompileTimeInt(value) => Ok(Some(TanukiCompileTimeValue::CompileTimeInt(-value))),
			_ => Ok(None), //Err(Error::CannotUseUnaryOperatorForType { type_t: format!("{:?}", self.get_type()), operator: "-".to_string() }),
		}
	}
}

impl Add for TanukiCompileTimeValue {
	type Output = Result<Option<TanukiCompileTimeValue>, Error>;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(TanukiCompileTimeValue::CompileTimeInt(lhs_value), TanukiCompileTimeValue::CompileTimeInt(rhs_value)) => Ok(Some(TanukiCompileTimeValue::CompileTimeInt(lhs_value + rhs_value))),
			//(lhs, rhs)
			//	=> Err(Error::CannotUseBinaryOperatorForType { lhs_type_t: format!("{:?}", lhs.get_type()), rhs_type_t: format!("{:?}", rhs.get_type()), operator: "+".into() }),
			_ => Ok(None),
		}
	}
}

impl Sub for TanukiCompileTimeValue {
	type Output = Result<Option<TanukiCompileTimeValue>, Error>;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(TanukiCompileTimeValue::CompileTimeInt(lhs_value), TanukiCompileTimeValue::CompileTimeInt(rhs_value)) => Ok(Some(TanukiCompileTimeValue::CompileTimeInt(lhs_value - rhs_value))),
			//(lhs, rhs)
			//	=> Err(Error::CannotUseBinaryOperatorForType { lhs_type_t: format!("{:?}", lhs.get_type()), rhs_type_t: format!("{:?}", rhs.get_type()), operator: "-".into() }),
			_ => Ok(None),
		}
	}
}

impl Mul for TanukiCompileTimeValue {
	type Output = Result<Option<TanukiCompileTimeValue>, Error>;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(TanukiCompileTimeValue::CompileTimeInt(lhs_value), TanukiCompileTimeValue::CompileTimeInt(rhs_value)) => Ok(Some(TanukiCompileTimeValue::CompileTimeInt(lhs_value * rhs_value))),
			//(lhs, rhs)
			//	=> Err(Error::CannotUseBinaryOperatorForType { lhs_type_t: format!("{:?}", lhs.get_type()), rhs_type_t: format!("{:?}", rhs.get_type()), operator: "*".into() }),
			_ => Ok(None),
		}
	}
}

impl Div for TanukiCompileTimeValue {
	type Output = Result<Option<TanukiCompileTimeValue>, Error>;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(TanukiCompileTimeValue::CompileTimeInt(lhs_value), _) if lhs_value.is_zero() => Err(Error::DivisionByZero),
			(TanukiCompileTimeValue::CompileTimeInt(lhs_value), TanukiCompileTimeValue::CompileTimeInt(rhs_value)) => Ok(Some(TanukiCompileTimeValue::CompileTimeInt(lhs_value / rhs_value))),
			//(lhs, rhs)
			//	=> Err(Error::CannotUseBinaryOperatorForType { lhs_type_t: format!("{:?}", lhs.get_type()), rhs_type_t: format!("{:?}", rhs.get_type()), operator: "/".into() }),
			_ => Ok(None),
		}
	}
}*/