use std::fmt::Debug;

use crate::{Main, error::ErrorAt};

pub trait Module: Debug {
	fn execute_interpreted(&self, main: &mut Main) -> Result<(), ErrorAt>;
}