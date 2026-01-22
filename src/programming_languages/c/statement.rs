use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{expression::CExpression, types::CType}, traits::{ast_node::AstNode, statement::Statement}};

#[derive(Debug)]
pub enum CStatement {
	CompoundStatement(CCompoundStatement),
	VariableDeclaration(CType, Box<str>, Option<Box<CInitializer>>),
	Expression(CExpression),
}

impl Statement for CStatement {
	
}

impl AstNode for CStatement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompoundStatement(_) => write!(f, "Compound Statement"),
			Self::VariableDeclaration(_, name, _) => write!(f, "Variable Declaration \"{name}\""),
			Self::Expression(_) => write!(f, "Expression"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompoundStatement(compound_statement) => compound_statement.print(level, f),
			Self::VariableDeclaration(var_type, _, initializer) => {
				var_type.print(level, f)?;
				if let Some(initializer) = initializer {
					initializer.print(level, f)?;
				}
				Ok(())
			}
			Self::Expression(expression) => expression.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>) -> Result<(), ErrorAt> {
		match self {
			Self::CompoundStatement(compound_statement) => compound_statement.write_to_file(writer),
			Self::Expression(expression) => {
				expression.write_to_file(writer)?;
				writer.write_all(b";").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::VariableDeclaration(variable_type, name, initializer) => {
				variable_type.write_to_file(writer)?;
				writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if let Some(initializer) = initializer {
					initializer.write_to_file(writer)?;
				}
				writer.write_all(b";").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}

#[derive(Debug)]
pub struct CCompoundStatement {
	sub_statements: Vec<CStatement>,
}

impl CCompoundStatement {
	pub fn new() -> Self {
		Self {
			sub_statements: Vec::new(),
		}
	}

	pub fn push_statement(&mut self, statement: CStatement) {
		self.sub_statements.push(statement);
	}
}

impl AstNode for CCompoundStatement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Compound Statement")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for sub_statement in self.sub_statements.iter() {
			sub_statement.print(level, f)?;
		}
		Ok(())
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>) -> Result<(), ErrorAt> {
		writer.write_all(b"{").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		for sub_statement in self.sub_statements.iter() {
			sub_statement.write_to_file(writer)?;
		}
		writer.write_all(b"}").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
	}
}

#[derive(Debug)]
pub enum CInitializer {
	Expression(CExpression)
}

impl AstNode for CInitializer {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Expression(_) => write!(f, "Expression"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Expression(expression) => expression.print(level, f),
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>) -> Result<(), ErrorAt> {
		match self {
			Self::Expression(expression) => expression.write_to_file(writer)
		}
	}
}