use std::{fmt::{self, Debug, Formatter}, num::NonZeroUsize};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::{c::module::CModule, tanuki::{expression::TanukiExpression, token::{TanukiToken, TanukiTokenVariant}}}, token_reader::TokenReader, traits::{ast_node::AstNode, module::Module}};

pub struct TanukiModule {
	pub expressions: Box<[TanukiExpression]>,
}

impl TanukiModule {
	/// Parse tokens received from tokenizing a file into a `TanukiModule` containing an AST.
	pub fn parse(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		// Parse expressions until there are none left
		let mut expressions = Vec::new();
		while !token_reader.is_empty() {
			// Parse expression
			if let Some(expression) = TanukiExpression::parse(main, token_reader)? {
				expressions.push(expression);
			}
			// Expect a semicolon
			match token_reader.next() {
				Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, .. }) => {},
				Some(TanukiToken { start_line, start_column, .. }) => return Err(Error::ExpectedSemicolon.at(Some(*start_line), Some(*start_column), None)),
				None => return Err(Error::ExpectedSemicolon.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			}
		}
		Ok(Self {
			expressions: expressions.into_boxed_slice(),
		})
	}
}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		todo!()
	}

	fn to_c_module(&self, _main: &mut Main, _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		todo!()
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for expression in &self.expressions {
			expression.print(level, f)?;
		}
		Ok(())
	}

	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.expressions.first().map(|expression| expression.start_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.expressions.first().map(|expression| expression.start_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.expressions.last().map(|expression| expression.end_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.expressions.last().map(|expression| expression.end_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}
}

impl Debug for TanukiModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}