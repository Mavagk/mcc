use std::fmt::Display;

use crate::traits::types::Type;

pub trait Expression: Display {
	type TypeType: Type;

	fn get_result_type<'a>(&'a self) -> &'a Self::TypeType;
}