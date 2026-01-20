use std::{fmt::{self, Formatter}, num::NonZeroUsize};

use crate::{programming_languages::c::{statement::CCompoundStatement, types::CType}, traits::{ast_node::AstNode, module_element::ModuleElement}};

#[derive(Debug)]
pub enum CModuleElement {
	FunctionDefinition { return_type: CType, name: Box<str>, arguments: Box<[(CType, Box<str>)]>, body: Box<CCompoundStatement> }
}

impl ModuleElement for CModuleElement {

}

impl AstNode for CModuleElement {
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