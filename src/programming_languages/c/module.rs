use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{error::ErrorAt, programming_languages::c::module_element::CModuleElement, traits::{ast_node::AstNode, module::Module}};

#[derive(Debug)]
pub struct CModule {
	_elements: Vec<CModuleElement>,
}

impl CModule {
	pub fn new() -> Self {
		Self {
			_elements: Vec::new(),
		}
	}
}

impl Module for CModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut crate::Main) -> Result<(), ErrorAt> {
		unimplemented!()
	}
}

impl AstNode for CModule {
	fn get_start_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_end_line(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_start_column(&self) -> NonZeroUsize {
		todo!()
	}

	fn get_end_column(&self) -> NonZeroUsize {
		todo!()
	}

	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}