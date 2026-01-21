use std::fmt::{self, Formatter};

use crate::{programming_languages::c::{statement::CCompoundStatement, types::CType}, traits::{ast_node::AstNode, module_element::ModuleElement}};

#[derive(Debug)]
pub enum CModuleElement {
	FunctionDefinition { return_type: CType, name: Box<str>, parameters: Box<[CFunctionParameter]>, body: Box<CCompoundStatement> }
}

impl ModuleElement for CModuleElement {

}

impl AstNode for CModuleElement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::FunctionDefinition { name, .. } => write!(f, "Function Definition \"{name}\""),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::FunctionDefinition { return_type, parameters: arguments, body, .. } => {
				return_type.print(level, f)?;
				writeln!(f)?;
				for argument in arguments {
					argument.print(level, f)?;
				}
				body.print(level, f)?;
				Ok(())
			}
		}
	}
}

#[derive(Debug)]
pub struct CFunctionParameter {
	param_type: CType,
	name: Box<str>,
}

impl AstNode for CFunctionParameter {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Function Parameter \"{}\"", self.name)
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		self.param_type.print(level, f)
	}
}