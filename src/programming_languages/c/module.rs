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
	fn start_line(&self) -> Option<NonZeroUsize> {
		None
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		None
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		None
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		None
	}

	fn print_name(&self, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}