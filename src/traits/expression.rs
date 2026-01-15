use std::fmt::Debug;

use crate::traits::types::Type;

pub trait Expression: Debug {
	type TypeType: Type;

	fn get_result_type<'a>(&'a self) -> &'a Self::TypeType;
}