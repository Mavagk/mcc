use std::{fmt::{self, Formatter}, num::NonZeroUsize, path::Path};

use crate::{programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiNullaryOperator, TanukiPostfixUnaryOperator, TanukiPrefixUnaryOperator}}, traits::{ast_node::AstNode, expression::Expression}};

#[derive(Debug, Clone)]
/// A Tanuki expression that cas been parsed from a file.
pub struct TanukiExpression {
	pub variant: TanukiExpressionVariant,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub enum TanukiExpressionVariant {
	Constant(TanukiCompileTimeValue),
	//Block { sub_expressions: Box<[TanukiExpression]>, has_return_value: bool },
	Block { sub_expressions: Vec<TanukiExpression>, return_expressions: Box<[(Option<Box<str>>, TanukiExpression)]> },
	Variable(Box<str>),
	FunctionCall { function_pointer: Box<TanukiExpression>, arguments: Box<[TanukiExpression]> },
	FunctionDefinition { parameters: Box<[TanukiExpression]>, return_type: Option<Box<TanukiExpression>>, body_expression: Box<TanukiExpression> },
	Function { name: Box<str>, module_path: Box<Path> },
	Index(Box<TanukiExpression>, Box<TanukiExpression>),
	TypeAndValue(Box<TanukiExpression>, Box<TanukiExpression>),
	ImportConstant { name: Option<Box<str>>, module_path: Box<Path> },
	Export(Box<TanukiExpression>),
	Link { name: Option<Box<str>>, library_path: Box<Path>, parameter_types: Box<[TanukiExpression]>, return_type: Option<Box<TanukiExpression>>, link_if: Option<Box<TanukiExpression>> },
	Entrypoint(Box<TanukiExpression>),
	U(Box<[TanukiExpression]>),
	I(Box<[TanukiExpression]>),
	F(Box<[TanukiExpression]>),
	NullaryOperator(TanukiNullaryOperator),
	PrefixUnaryOperator(TanukiPrefixUnaryOperator, Box<TanukiExpression>),
	PostfixUnaryOperator(TanukiPostfixUnaryOperator, Box<TanukiExpression>),
	InfixBinaryOperator(TanukiInfixBinaryOperator, Box<TanukiExpression>, Box<TanukiExpression>),
	InfixTernaryOperator(TanukiInfixTernaryOperator, Box<TanukiExpression>, Box<TanukiExpression>, Box<TanukiExpression>),
	Assignment(Box<TanukiExpression>, Box<TanukiExpression>),
	AugmentedBinaryAssignment(TanukiInfixBinaryOperator, Box<TanukiExpression>, Box<TanukiExpression>),
	Transmute { to_transmute: Box<TanukiExpression>, transmute_to_type: Box<TanukiExpression> },
}

impl Default for TanukiExpression {
	fn default() -> Self {
		Self {
			variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Void),
			start_line: 1.try_into().unwrap(), start_column: 1.try_into().unwrap(), end_line: 1.try_into().unwrap(), end_column: 1.try_into().unwrap(),
		}
	}
}

impl Expression for TanukiExpression {}

impl AstNode for TanukiExpression {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(..) => write!(f, "Constant"),
			TanukiExpressionVariant::Block { .. } => {
				write!(f, "Block")
				//if *has_return_value {
				//	write!(f, ", has return value")?;
				//}
				//Ok(())
			},
			TanukiExpressionVariant::FunctionCall { .. }                            => write!(f, "Function Call"),
			TanukiExpressionVariant::FunctionDefinition { return_type, .. } => {
				write!(f, "Function Definition")?;
				if return_type.is_some() {
					write!(f, ", Has Return Type")?;
				}
				Ok(())
			},
			TanukiExpressionVariant::U { .. }                                                              => write!(f, "U"),
			TanukiExpressionVariant::I { .. }                                                              => write!(f, "I"),
			TanukiExpressionVariant::F { .. }                                                              => write!(f, "F"),
			TanukiExpressionVariant::Function { name, module_path }                 => write!(f, "Function {name} from {module_path:?}"),
			TanukiExpressionVariant::Index { .. }                                                          => write!(f, "Index"),
			TanukiExpressionVariant::Variable(name)                                             => write!(f, "Variable {name}"),
			TanukiExpressionVariant::TypeAndValue(..)                                                      => write!(f, "Type and Value"),
			TanukiExpressionVariant::ImportConstant { name, module_path } => {
				write!(f, "Import Constant")?;
				if let Some(name) = name {
					write!(f, " {name}")?;
				}
				write!(f, " from {module_path:?}")
			}
			TanukiExpressionVariant::Export(..)                                                            => write!(f, "Export"),
			TanukiExpressionVariant::Link { name, library_path, .. } => {
				write!(f, "Link")?;
				if let Some(name) = name {
					write!(f, " {name}")?;
				}
				write!(f, " from {library_path:?}")
			}
			TanukiExpressionVariant::Entrypoint(..)                                                        => write!(f, "Entrypoint"),
			TanukiExpressionVariant::NullaryOperator(operator)                     => write!(f, "Nullary Operator {operator}"),
			TanukiExpressionVariant::PrefixUnaryOperator(operator, _)          => write!(f, "Prefix Unary Operator {operator}"),
			TanukiExpressionVariant::PostfixUnaryOperator(operator, _)        => write!(f, "Postfix Unary Operator {operator}"),
			TanukiExpressionVariant::InfixBinaryOperator(operator, _, _)       => write!(f, "Infix Binary Operator {operator}"),
			TanukiExpressionVariant::InfixTernaryOperator(operator, _, _, _)  => write!(f, "Infix Ternary Operator {operator}"),
			TanukiExpressionVariant::Assignment(..)                                                        => write!(f, "Assignment"),
			TanukiExpressionVariant::AugmentedBinaryAssignment(operator, _, _) => write!(f, "Augmented Binary Assignment {operator}="),
			TanukiExpressionVariant::Transmute { .. }                                                      => write!(f, "Transmute"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.variant {
			TanukiExpressionVariant::Constant(value) => value.print(level, f),
			TanukiExpressionVariant::Variable(..) => Ok(()),
			TanukiExpressionVariant::Block { sub_expressions, return_expressions} => {
				for sub_expression in sub_expressions {
					sub_expression.print(level, f)?;
				}
				writeln!(f)?;
				for (_name, return_expression) in return_expressions {
					return_expression.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::Function { .. } | TanukiExpressionVariant::ImportConstant { .. } => Ok(()),
			TanukiExpressionVariant::FunctionCall { function_pointer, arguments } => {
				function_pointer.print(level, f)?;
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression } => {
				for parameter in parameters {
					parameter.print(level, f)?;
				}
				if let Some(return_type) = return_type {
					return_type.print(level, f)?;
				}
				body_expression.print(level, f)
			}
			TanukiExpressionVariant::U(arguments) | TanukiExpressionVariant::I(arguments) | TanukiExpressionVariant::F(arguments) => {
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::Export(operand) | TanukiExpressionVariant::Entrypoint(operand) |
			TanukiExpressionVariant::PrefixUnaryOperator(_, operand) |
			TanukiExpressionVariant::PostfixUnaryOperator(_, operand) => operand.print(level, f),
			TanukiExpressionVariant::Index(lhs, rhs) |
			TanukiExpressionVariant::TypeAndValue(lhs, rhs) |
			TanukiExpressionVariant::Assignment(lhs, rhs) |
			TanukiExpressionVariant::InfixBinaryOperator(_, lhs, rhs) |
			TanukiExpressionVariant::AugmentedBinaryAssignment(_, lhs, rhs) => {
				lhs.print(level, f)?;
				rhs.print(level, f)
			}
			TanukiExpressionVariant::InfixTernaryOperator(_, lhs, mhs, rhs) => {
				lhs.print(level, f)?;
				mhs.print(level, f)?;
				rhs.print(level, f)
			}
			TanukiExpressionVariant::NullaryOperator(_) => Ok(()),
			TanukiExpressionVariant::Link { parameter_types: argument_types, return_type, link_if, .. } => {
				for argument in argument_types.iter() {
					argument.print(level, f)?;
				}
				writeln!(f)?;
				if let Some(return_type) = return_type {
					return_type.print(level, f)?;
				}
				if let Some(link_if) = link_if {
					link_if.print(level, f)?;
				}
				Ok(())
			}
			TanukiExpressionVariant::Transmute { to_transmute, transmute_to_type } => {
				to_transmute.print(level, f)?;
				transmute_to_type.print(level, f)
			}
		}
	}

	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.start_line)
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.start_column)
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.end_line)
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.end_column)
	}
}