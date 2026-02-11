use std::fmt::{self, Formatter};

use crate::{Main, error::ErrorAt, programming_languages::{c::module::CModule, tanuki::{expression::TanukiExpression, token::TanukiToken}}, token_reader::TokenReader, traits::{ast_node::AstNode, module::Module}};

#[derive(Debug)]
pub struct TanukiModule {
	pub expressions: Box<[TanukiExpression]>,
}

impl TanukiModule {
	/// Parse tokens received from tokenizing a file into a `TanukiModule` containing an AST.
	pub fn parse(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		let mut expressions = Vec::new();
		while !token_reader.is_empty() {
			expressions.push(TanukiExpression::parse(main, token_reader)?);
		}
		Ok(Self {
			expressions: expressions.into_boxed_slice(),
		})
		// TODO: Remove , ;
	}
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