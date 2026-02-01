use std::{fmt::{self, Formatter}, fs::File, io::{BufWriter, Write}};

use crate::{error::{Error, ErrorAt}, programming_languages::c::{l_value::CLValue, statement::CStatement, types::CType}, traits::{ast_node::AstNode, expression::Expression}};

#[derive(Debug)]
pub enum CExpression {
	// Non-operators
	LValueRead(Box<CLValue>),
	FunctionCall(Box<str>, Box<[CExpression]>),
	IntConstant(i128),
	StringConstant(Box<str>),
	// Pointer operators
	TakeReference(Box<CLValue>),
	// Assignments
	Assignment(Box<CLValue>, Box<CExpression>),
	AssignmentAdd(Box<CLValue>, Box<CExpression>),
	AssignmentSubtract(Box<CLValue>, Box<CExpression>),
	AssignmentMultiply(Box<CLValue>, Box<CExpression>),
	AssignmentDivide(Box<CLValue>, Box<CExpression>),
	AssignmentModulo(Box<CLValue>, Box<CExpression>),
	AssignmentBitwiseAnd(Box<CLValue>, Box<CExpression>),
	AssignmentBitwiseOr(Box<CLValue>, Box<CExpression>),
	AssignmentBitwiseXor(Box<CLValue>, Box<CExpression>),
	AssignmentBitshiftLeft(Box<CLValue>, Box<CExpression>),
	AssignmentBitshiftRight(Box<CLValue>, Box<CExpression>),
	// Type operators
	Sizeof(CType),
	// Unary l-value operators
	PostfixIncrement(Box<CLValue>),
	PostfixDecrement(Box<CLValue>),
	PrefixIncrement(Box<CLValue>),
	PrefixDecrement(Box<CLValue>),
	// Basic binary operators
	Add(Box<CExpression>, Box<CExpression>),
	Subtract(Box<CExpression>, Box<CExpression>),
	Multiply(Box<CExpression>, Box<CExpression>),
	Divide(Box<CExpression>, Box<CExpression>),
	Modulo(Box<CExpression>, Box<CExpression>),
	// Bitwise binary operators
	BitwiseAnd(Box<CExpression>, Box<CExpression>),
	BitwiseOr(Box<CExpression>, Box<CExpression>),
	BitwiseXor(Box<CExpression>, Box<CExpression>),
	BitshiftLeft(Box<CExpression>, Box<CExpression>),
	BitshiftRight(Box<CExpression>, Box<CExpression>),
	// Logical binary operators
	LogicalAnd(Box<CExpression>, Box<CExpression>),
	LogicalOr(Box<CExpression>, Box<CExpression>),
	// Comparison
	Equal(Box<CExpression>, Box<CExpression>),
	NotEqual(Box<CExpression>, Box<CExpression>),
	GreaterThan(Box<CExpression>, Box<CExpression>),
	LessThan(Box<CExpression>, Box<CExpression>),
	GreaterThanOrEqual(Box<CExpression>, Box<CExpression>),
	LessThanOrEqual(Box<CExpression>, Box<CExpression>),
	// Unary operators
	Negate(Box<CExpression>),
	UnaryPlus(Box<CExpression>),
	BitwiseNot(Box<CExpression>),
	LogicalNot(Box<CExpression>),
	// Ternary
	Ternary(Box<CExpression>, Box<CExpression>, Box<CExpression>),
}

impl CExpression {
	pub fn equal(self, rhs: Self) -> Self {
		Self::Equal(self.into(), rhs.into())
	}

	pub fn not_equal(self, rhs: Self) -> Self {
		Self::NotEqual(self.into(), rhs.into())
	}

	pub fn add(self, rhs: Self) -> Self {
		Self::Add(self.into(), rhs.into())
	}

	pub fn if_statement(self, block: CStatement) -> CStatement {
		CStatement::If(self.into(), block.into())
	}

	pub fn while_statement(self, block: CStatement) -> CStatement {
		CStatement::If(self.into(), block.into())
	}

	pub fn function_call(name: &str, args: Box<[CExpression]>) -> Self {
		Self::FunctionCall(name.into(), args)
	}

	pub fn string_constant(value: &str) -> Self {
		Self::StringConstant(value.into())
	}
}

impl Expression for CExpression {
	
}

impl AstNode for CExpression {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::LValueRead(_) => write!(f, "Read L-Value"),
			Self::FunctionCall(name, _) => write!(f, "Function Call \"{name}\""),
			Self::IntConstant(value) => write!(f, "Int Constant {value}"),
			Self::StringConstant(value) => write!(f, "String Constant \"{value}\""),

			Self::Assignment(_, _) => write!(f, "Assignment"),
			Self::AssignmentAdd(_, _) => write!(f, "Assignment Add"),
			Self::AssignmentSubtract(_, _) => write!(f, "Assignment Subtract"),
			Self::AssignmentMultiply(_, _) => write!(f, "Assignment Multiply"),
			Self::AssignmentDivide(_, _) => write!(f, "Assignment Divide"),
			Self::AssignmentModulo(_, _) => write!(f, "Assignment Modulo"),
			Self::AssignmentBitwiseAnd(_, _) => write!(f, "Assignment Bitwise And"),
			Self::AssignmentBitwiseOr(_, _) => write!(f, "Assignment Bitwise Or"),
			Self::AssignmentBitwiseXor(_, _) => write!(f, "Assignment Bitwise Xor"),
			Self::AssignmentBitshiftLeft(_, _) => write!(f, "Assignment Bitshift Left"),
			Self::AssignmentBitshiftRight(_, _) => write!(f, "Assignment Bitshift Right"),

			Self::Sizeof(_) => write!(f, "Sizeof"),

			Self::PostfixIncrement(_) => write!(f, "Postfix Increment"),
			Self::PostfixDecrement(_) => write!(f, "Postfix Decrement"),
			Self::PrefixIncrement(_) => write!(f, "Postfix Increment"),
			Self::PrefixDecrement(_) => write!(f, "Postfix Decrement"),

			Self::Add(_, _) => write!(f, "Add"),
			Self::Subtract(_, _) => write!(f, "Subtract"),
			Self::Multiply(_, _) => write!(f, "Multiply"),
			Self::Divide(_, _) => write!(f, "Divide"),
			Self::Modulo(_, _) => write!(f, "Modulo"),

			Self::BitwiseAnd(_, _) => write!(f, "Bitwise And"),
			Self::BitwiseOr(_, _) => write!(f, "Bitwise Or"),
			Self::BitwiseXor(_, _) => write!(f, "Bitwise Xor"),
			Self::BitshiftLeft(_, _) => write!(f, "Bitshift Left"),
			Self::BitshiftRight(_, _) => write!(f, "Bitshift Right"),

			Self::LogicalAnd(_, _) => write!(f, "Logical And"),
			Self::LogicalOr(_, _) => write!(f, "Logical Or"),

			Self::Equal(_, _) => write!(f, "Equal"),
			Self::NotEqual(_, _) => write!(f, "Not Equal"),
			Self::LessThan(_, _) => write!(f, "Less than"),
			Self::GreaterThan(_, _) => write!(f, "Greater than"),
			Self::LessThanOrEqual(_, _) => write!(f, "Less than or Equal"),
			Self::GreaterThanOrEqual(_, _) => write!(f, "Greater than or Equal"),

			Self::UnaryPlus(_) => write!(f, "Unary Plus"),
			Self::Negate(_) => write!(f, "Negate"),
			Self::BitwiseNot(_) => write!(f, "Bitwise Not"),
			Self::LogicalNot(_) => write!(f, "Logical Not"),
			
			Self::TakeReference(_) => write!(f, "Take Reference"),
			Self::Ternary(_, _, _) => write!(f, "Ternary"),
		}
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Assignment(l_value, sub_expression) | Self::AssignmentAdd(l_value, sub_expression) | Self::AssignmentSubtract(l_value, sub_expression) |
			Self::AssignmentMultiply(l_value, sub_expression) | Self::AssignmentDivide(l_value, sub_expression) | Self::AssignmentModulo(l_value, sub_expression) |
			Self::AssignmentBitwiseAnd(l_value, sub_expression) | Self::AssignmentBitwiseOr(l_value, sub_expression) | Self::AssignmentBitwiseXor(l_value, sub_expression) |
			Self::AssignmentBitshiftLeft(l_value, sub_expression) | Self::AssignmentBitshiftRight(l_value, sub_expression) => {
				l_value.print(level, f)?;
				sub_expression.print(level, f)
			}
			Self::FunctionCall(_, arguments) => {
				for argument in arguments {
					argument.print(level, f)?;
				}
				Ok(())
			}
			Self::IntConstant(_) | Self::StringConstant(_) => Ok(()),
			Self::Sizeof(sub_type) => sub_type.print(level, f),
			Self::Add(lhs, rhs) | Self::Subtract(lhs, rhs) | Self::Multiply(lhs, rhs) | Self::Divide(lhs, rhs) | Self::Modulo(lhs, rhs) |
			Self::BitwiseAnd(lhs, rhs) | Self::BitwiseOr(lhs, rhs) | Self::BitwiseXor(lhs, rhs) | Self::BitshiftLeft(lhs, rhs) | Self::BitshiftRight(lhs, rhs) |
			Self::LogicalAnd(lhs, rhs) | Self::LogicalOr(lhs, rhs) |
			Self::Equal(lhs, rhs) | Self::NotEqual(lhs, rhs) |
			Self::GreaterThan(lhs, rhs) | Self::LessThan(lhs, rhs) | Self::GreaterThanOrEqual(lhs, rhs) | Self::LessThanOrEqual(lhs, rhs) => {
				lhs.print(level, f)?;
				rhs.print(level, f)
			}
			Self::PostfixIncrement(l_value) | Self::PostfixDecrement(l_value) | Self::PrefixIncrement(l_value) | Self::PrefixDecrement(l_value) |
			Self::TakeReference(l_value) | Self::LValueRead(l_value) => l_value.print(level, f),
			Self::UnaryPlus(sub_value) | Self::Negate(sub_value) | Self::LogicalNot(sub_value) | Self::BitwiseNot(sub_value) => sub_value.print(level, f),
			Self::Ternary(condition, if_true, if_false) => {
				condition.print(level, f)?;
				if_true.print(level, f)?;
				if_false.print(level, f)
			}
		}
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		match self {
			Self::Assignment(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" = ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentAdd(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" += ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentSubtract(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" -= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentMultiply(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" *= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentDivide(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" /= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentModulo(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" %= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentBitwiseAnd(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" &= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentBitwiseOr(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" |= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentBitwiseXor(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" ^= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentBitshiftLeft(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" <<= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::AssignmentBitshiftRight(l_value, r_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b" >>= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				r_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::LValueRead(l_value) => l_value.write_to_file(writer, indentation_level),
			Self::FunctionCall(name, arguments) => {
				writer.write_all(name.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				let mut is_first_argument = true;
				for argument in arguments {
					if !is_first_argument {
						writer.write_all(b", ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
					}
					argument.write_to_file(writer, indentation_level)?;
					is_first_argument = false;
				}
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::IntConstant(value) => writer.write_all(format!("{value}").as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None)),
			Self::StringConstant(value) => {
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(value.as_bytes()).map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				writer.write_all(b"\"").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Sizeof(sub_type) => {
				writer.write_all(b"sizeof(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_type.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::PostfixIncrement(l_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b"++)").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PostfixDecrement(l_value) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b"--)").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PrefixIncrement(l_value) => {
				writer.write_all(b"(++").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::PrefixDecrement(l_value) => {
				writer.write_all(b"(--").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::Add(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" + ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Subtract(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" - ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Multiply(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" * ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Divide(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" / ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Modulo(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" % ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::BitwiseAnd(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" & ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::BitwiseOr(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" | ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::BitwiseXor(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" ^ ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::BitshiftLeft(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" << ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::BitshiftRight(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" >> ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::LogicalAnd(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" && ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::LogicalOr(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" || ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::Equal(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" == ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::NotEqual(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" != ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::LessThan(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" < ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::GreaterThan(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" > ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::LessThanOrEqual(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" <= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::GreaterThanOrEqual(lhs, rhs) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				lhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b" >= ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				rhs.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}

			Self::UnaryPlus(sub_value) => {
				writer.write_all(b"(+").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Negate(sub_value) => {
				writer.write_all(b"(-").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::BitwiseNot(sub_value) => {
				writer.write_all(b"(~").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::LogicalNot(sub_value) => {
				writer.write_all(b"(!").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				sub_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			
			Self::TakeReference(l_value) => {
				writer.write_all(b"(&").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				l_value.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
			Self::Ternary(condition, if_true, if_false) => {
				writer.write_all(b"(").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				condition.write_to_file(writer, indentation_level)?;
				writer.write_all(b"? ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if_true.write_to_file(writer, indentation_level)?;
				writer.write_all(b": ").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
				if_false.write_to_file(writer, indentation_level)?;
				writer.write_all(b")").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))
			}
		}
	}
}

impl Into<CStatement> for CExpression {
	fn into(self) -> CStatement {
		CStatement::Expression(self)
	}
}

impl Into<Box<CStatement>> for CExpression {
	fn into(self) -> Box<CStatement> {
		CStatement::Expression(self).into()
	}
}