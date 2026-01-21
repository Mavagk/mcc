use std::fmt::{self, Debug, Formatter};

use crate::{Main, error::ErrorAt, programming_languages::c::module_element::CModuleElement, traits::{ast_node::AstNode, module::Module}};

pub struct CModule {
	elements: Vec<CModuleElement>,
}

impl CModule {
	pub fn new() -> Self {
		Self {
			elements: Vec::new(),
		}
	}

	pub fn push_element(&mut self, element: CModuleElement) {
		self.elements.push(element);
	}
}

impl Module for CModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		unimplemented!()
	}

	fn to_c_module(&self, _main: &mut Main, _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		unimplemented!()
	}
}

impl AstNode for CModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "C Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for sub_element in self.elements.iter() {
			sub_element.print(level, f)?;
		}
		Ok(())
	}
}

impl Debug for CModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}