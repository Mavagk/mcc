use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{expression::CExpression, types::CType}, traits::{ast_node::AstNode, statement::Statement}};

#[derive(Debug)]
pub enum CStatement {
	CompoundStatement(CCompoundStatement),
	VariableDeclaration(CType, Box<str>, Option<Box<CInitializer>>),
	Expression(CExpression),
	Comment(Box<str>),
	If(Box<CExpression>, Box<CStatement>),
	While(Box<CExpression>, Box<CStatement>),
	Return(Option<Box<CExpression>>),
}

impl Statement for CStatement {
	
}

impl AstNode for CStatement {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::CompoundStatement(_) => write!(f, "Compound Statement"),
			Self::VariableDeclaration(_, name, _) => write!(f, "Variable Declaration \"{name}\""),
			Self::Expression(_) => write!(f, "Expression"),
			Self::Comment(comment) => write!(f, "Comment \"{comment}\""),
			Self::If(_, _) => write!(f, "If"),
			Self::While(_, _) => write!(f, "While"),
			Self::Return(_) => write!(f, "Return"),
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
			Self::Comment(_) => Ok(()),
			Self::If(condition_expression, sub_statement) | Self::While(condition_expression, sub_statement) => {
				condition_expression.print(level, f)?;
				sub_statement.print(level, f)
			}
			Self::Return(return_value) => {
				if let Some(return_value) = return_value {
					return_value.print(level, f)?;
				}
				Ok(())
			}
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::CompoundStatement(compound_statement) => compound_statement.write_to_file(writer, indentation_level),
			Self::Expression(expression) => {
				expression.write_to_file(writer, indentation_level)?;
				writer.write_all(b";").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::VariableDeclaration(variable_type, name, initializer) => {
				//variable_type.write_to_file(writer, indentation_level)?;
				//writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				//writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				variable_type.write_to_file_with_name(writer, indentation_level, name)?;
				if let Some(initializer) = initializer {
					writer.write_all(b" = ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					initializer.write_to_file(writer, indentation_level)?;
				}
				writer.write_all(b";").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Comment(comment) => {
				writer.write_all(b"// ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(comment.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::If(condition_expression, sub_statement) => {
				writer.write_all(b"if (").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				condition_expression.write_to_file(writer, indentation_level)?;
				writer.write_all(b") ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_statement.write_to_file(writer, indentation_level)
			}
			Self::While(condition_expression, sub_statement) => {
				writer.write_all(b"while (").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				condition_expression.write_to_file(writer, indentation_level)?;
				writer.write_all(b") ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_statement.write_to_file(writer, indentation_level)
			}
			Self::Return(return_value) => {
				writer.write_all(b"return").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if let Some(return_value) = return_value {
					writer.write_all(b" ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					return_value.write_to_file(writer, indentation_level)?;
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

impl Into<CStatement> for CCompoundStatement {
	fn into(self) -> CStatement {
		CStatement::CompoundStatement(self)
	}
}

impl Into<Box<CStatement>> for CCompoundStatement {
	fn into(self) -> Box<CStatement> {
		CStatement::CompoundStatement(self).into()
	}
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

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		writer.write_all(b"{").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		if !self.sub_statements.is_empty() {
			writer.write_all(b"\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		}
		for sub_statement in self.sub_statements.iter() {
			for _ in 0..indentation_level + 1 {
				writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
			}
			sub_statement.write_to_file(writer, indentation_level + 1)?;
			writer.write_all(b"\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
		}
		if !self.sub_statements.is_empty() {
			for _ in 0..indentation_level {
				writer.write_all(b"\t").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
			}
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

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Expression(expression) => expression.write_to_file(writer, indentation_level)
		}
	}
}