use std::fmt::{self, Formatter};

use crate::{error::ErrorAt, programming_languages::c::module::CModule, traits::{ast_node::AstNode, module::Module}};

#[derive(Debug)]
pub struct TanukiModule {

}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut crate::Main) -> Result<(), ErrorAt> {
		todo!()
	}

	fn to_c_module(&self, _main: &mut crate::Main, _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		todo!()
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")
	}

	fn print_sub_nodes(&self, _level: usize, _f: &mut Formatter<'_>) -> fmt::Result {
		Ok(())
	}
}