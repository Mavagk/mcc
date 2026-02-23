use std::{collections::HashSet, mem::take};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{constant_value::TanukiConstantValue, export::TanukiExport, expression::{TanukiExpression, TanukiExpressionVariant}, function::{TanukiFunction, TanukiFunctionArgument}, global_constant::TanukiGlobalConstant, import::TanukiImport, link::TanukiLink, module::TanukiModule}};

pub struct TanukiModulePostParseData<'a> {
	pub functions: &'a mut Vec<TanukiFunction>,
	pub global_constants: &'a mut Vec<Option<TanukiGlobalConstant>>,
	pub exports: &'a mut Vec<TanukiExport>,
	pub imports: &'a mut Vec<TanukiImport>,
	pub links: &'a mut Vec<TanukiLink>,
}

impl TanukiModule {
	pub fn post_parse(&mut self, main: &mut Main) -> Result<(), ErrorAt> {
		let mut post_parse_data = TanukiModulePostParseData {
			functions: &mut self.functions,
			global_constants: &mut self.global_constants,
			exports: &mut self.exports, imports: &mut self.imports, links: &mut self.links
		};
		for expression in self.parsed_expressions.iter_mut() {
			expression.post_parse(main, &mut post_parse_data, false, None, false, &mut HashSet::new(), &mut Vec::new())?;
		}
		self.parsed_expressions = Default::default();
		Ok(())
	}
}

impl TanukiExpression {
	pub fn post_parse(
		&mut self, main: &mut Main, post_parse_data: &mut TanukiModulePostParseData, is_inside_function_or_block: bool, assigned_to_name: Option<&str>, is_l_value: bool,
		global_variables_dependent_on: &mut HashSet<Box<str>>, local_variables: &mut Vec<HashSet<Box<str>>>
	) -> Result<(), ErrorAt> {
		match (&mut self.variant, is_inside_function_or_block, is_l_value) {
			// Assignment
			(TanukiExpressionVariant::Assignment(lhs, rhs), false, _) => {
				let start_line = lhs.start_line;
				let start_column = lhs.start_column;
				let end_line = rhs.end_line;
				let end_column = rhs.end_column;
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, &mut HashSet::new(), &mut Vec::new())?;
				let lhs = take(lhs);
				let (name, t_type) = match lhs.clone().variant {
					TanukiExpressionVariant::Variable(name) => {
						(name, None)
					},
					TanukiExpressionVariant::TypeAndValue(type_expression, value_expression) => match value_expression.variant {
						TanukiExpressionVariant::Variable(name) => (name, Some(type_expression)),
						_ => return Err(Error::ExpectedVariable.at(Some(value_expression.start_line), Some(value_expression.end_column), None)),
					},
					_ => return Err(Error::ExpectedVariable.at(Some(lhs.start_line), Some(lhs.end_column), None)),
				};
				*global_variables_dependent_on = HashSet::new();
				global_variables_dependent_on.insert(name.clone());
				let mut global_variables_dependent_on = HashSet::new();
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, Some(&name), false, &mut global_variables_dependent_on, local_variables)?;
				let rhs = take(rhs);
				if !matches!(rhs.variant, TanukiExpressionVariant::Import(..) | TanukiExpressionVariant::Link(..)) {
					let global_constant = TanukiGlobalConstant {
						value_expression: *rhs, name, t_type: t_type.map(|t_type| *t_type), start_line, start_column, end_line, end_column, depends_on: global_variables_dependent_on,
						has_been_const_compiled: false,
					};
					post_parse_data.global_constants.push(Some(global_constant));
				}
				*self = *lhs.clone();
			}
			// Unary operators taking a argument of the same l/r rating
			(TanukiExpressionVariant::Percent(sub_expression) | TanukiExpressionVariant::Factorial(sub_expression) |
			TanukiExpressionVariant::SaturatingFactorial(sub_expression) | TanukiExpressionVariant::WrappingFactorial(sub_expression) |
			TanukiExpressionVariant::TryFactorial(sub_expression) | TanukiExpressionVariant::TryPropagate(sub_expression) |
			TanukiExpressionVariant::Unwrap(sub_expression) |
			TanukiExpressionVariant::Not(sub_expression) | TanukiExpressionVariant::Reciprocal(sub_expression) |
			TanukiExpressionVariant::BitshiftRightOne(sub_expression) | TanukiExpressionVariant::ComplexConjugate(sub_expression) |
			TanukiExpressionVariant::Signum(sub_expression) | TanukiExpressionVariant::Negation(sub_expression) |
			TanukiExpressionVariant::SaturatingNegation(sub_expression) | TanukiExpressionVariant::WrappingNegation(sub_expression) |
			TanukiExpressionVariant::TryNegation(sub_expression) | TanukiExpressionVariant::Square(sub_expression) |
			TanukiExpressionVariant::SaturatingSquare(sub_expression) | TanukiExpressionVariant::WrappingSquare(sub_expression) |
			TanukiExpressionVariant::TrySquare(sub_expression) | TanukiExpressionVariant::BitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::SaturatingBitshiftLeftOne(sub_expression) | TanukiExpressionVariant::WrappingBitshiftLeftOne(sub_expression) |
			TanukiExpressionVariant::TryBitshiftLeftOne(sub_expression) | TanukiExpressionVariant::NthToLast(sub_expression) |
			TanukiExpressionVariant::RangeToInclusive(sub_expression) | TanukiExpressionVariant::RangeToExclusive(sub_expression), _, _)
				=> sub_expression.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?,
			// L-value unary operators taking a l-value
			(TanukiExpressionVariant::Read(sub_expression), _, false)
				=> sub_expression.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?,
			// R-value unary operators taking a l-value
			(TanukiExpressionVariant::AddressOf(sub_expression), _, false)
				=> sub_expression.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, global_variables_dependent_on, local_variables)?,
			// Unary operators taking a l-value
			(TanukiExpressionVariant::PostfixIncrement(sub_expression) |
			TanukiExpressionVariant::PostfixSaturatingIncrement(sub_expression) | TanukiExpressionVariant::PostfixWrappingIncrement(sub_expression) |
			TanukiExpressionVariant::PostfixDecrement(sub_expression) | TanukiExpressionVariant::PostfixSaturatingDecrement(sub_expression) |
			TanukiExpressionVariant::PostfixWrappingDecrement(sub_expression) | TanukiExpressionVariant::PrefixIncrement(sub_expression) |
			TanukiExpressionVariant::PrefixSaturatingIncrement(sub_expression) | TanukiExpressionVariant::PrefixWrappingIncrement(sub_expression) |
			TanukiExpressionVariant::PrefixDecrement(sub_expression) | TanukiExpressionVariant::PrefixSaturatingDecrement(sub_expression) |
			TanukiExpressionVariant::PrefixWrappingDecrement(sub_expression), _, _)
				=> sub_expression.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, global_variables_dependent_on, local_variables)?,
			// Unary operators taking a r-value
			(TanukiExpressionVariant::Dereference(sub_expression), _, _)
				=> sub_expression.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, global_variables_dependent_on, local_variables)?,
			// Invalid unary operators as a l-value
			(TanukiExpressionVariant::AddressOf(_) | TanukiExpressionVariant::Read(_), _, true)
				=> return Err(Error::ExpressionCannotBeLValue.at(Some(self.start_line), Some(self.start_column), None)),
			// Binary operators that take arguments of the same l/r value rating
			(TanukiExpressionVariant::Exponent(lhs, rhs) |
			TanukiExpressionVariant::SaturatingExponent(lhs, rhs) | TanukiExpressionVariant::WrappingExponent(lhs, rhs) |
			TanukiExpressionVariant::TryExponent(lhs, rhs) | TanukiExpressionVariant::Multiplication(lhs, rhs) |
			TanukiExpressionVariant::SaturatingMultiplication(lhs, rhs) | TanukiExpressionVariant::WrappingMultiplication(lhs, rhs) |
			TanukiExpressionVariant::TryMultiplication(lhs, rhs) | TanukiExpressionVariant::Division(lhs, rhs) |
			TanukiExpressionVariant::SaturatingDivision(lhs, rhs) | TanukiExpressionVariant::WrappingDivision(lhs, rhs) |
			TanukiExpressionVariant::TryDivision(lhs, rhs) | TanukiExpressionVariant::Modulo(lhs, rhs) |
			TanukiExpressionVariant::SaturatingModulo(lhs, rhs) | TanukiExpressionVariant::WrappingModulo(lhs, rhs) |
			TanukiExpressionVariant::TryModulo(lhs, rhs) | TanukiExpressionVariant::Addition(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAddition(lhs, rhs) | TanukiExpressionVariant::WrappingAddition(lhs, rhs) |
			TanukiExpressionVariant::TryAddition(lhs, rhs) | TanukiExpressionVariant::Subtraction(lhs, rhs) |
			TanukiExpressionVariant::SaturatingSubtraction(lhs, rhs) | TanukiExpressionVariant::WrappingSubtraction(lhs, rhs) |
			TanukiExpressionVariant::TrySubtraction(lhs, rhs) | TanukiExpressionVariant::Concatenate(lhs, rhs) |
			TanukiExpressionVariant::Append(lhs, rhs) | TanukiExpressionVariant::BitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::SaturatingBitshiftLeft(lhs, rhs) | TanukiExpressionVariant::WrappingBitshiftLeft(lhs, rhs) |
			TanukiExpressionVariant::TryBitshiftLeft(lhs, rhs) | TanukiExpressionVariant::BitshiftRight(lhs, rhs) |
			TanukiExpressionVariant::ThreeWayCompare(lhs, rhs) | TanukiExpressionVariant::LessThan(lhs, rhs) |
			TanukiExpressionVariant::LessThanOrEqualTo(lhs, rhs) | TanukiExpressionVariant::GreaterThan(lhs, rhs) |
			TanukiExpressionVariant::GreaterThanOrEqualTo(lhs, rhs) | TanukiExpressionVariant::Equality(lhs, rhs) |
			TanukiExpressionVariant::Inequality(lhs, rhs) | TanukiExpressionVariant::ReferenceEquality(lhs, rhs) |
			TanukiExpressionVariant::ReferenceInequality(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitAnd(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNand(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitXor(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXnor(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitOr(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNor(lhs, rhs) | TanukiExpressionVariant::Minimum(lhs, rhs) |
			TanukiExpressionVariant::Maximum(lhs, rhs) | TanukiExpressionVariant::Pipe(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitAnd(lhs, rhs) | TanukiExpressionVariant::ShortCircuitNand(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXor(lhs, rhs) | TanukiExpressionVariant::ShortCircuitXnor(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitOr(lhs, rhs) | TanukiExpressionVariant::ShortCircuitNor(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitingNullCoalescing(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitingNullCoalescing(lhs, rhs) | TanukiExpressionVariant::ExclusiveRange(lhs, rhs) |
			TanukiExpressionVariant::InclusiveRange(lhs, rhs), _, _) => {
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
			}
			// Binary operators that take a left argument of the same l/r value rating and a right r-value argument
			(TanukiExpressionVariant::As(lhs, rhs) | TanukiExpressionVariant::TypeAndValue(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAs(lhs, rhs) | TanukiExpressionVariant::WrappingAs(lhs, rhs) |
			TanukiExpressionVariant::TryAs(lhs, rhs) | TanukiExpressionVariant::Index(lhs, rhs) |
			TanukiExpressionVariant::MemberAccess(lhs, rhs), _, _) => {
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?;
			}
			// Ternary
			(TanukiExpressionVariant::NonShortCircuitingConditional(lhs, mhs, rhs) | TanukiExpressionVariant::ShortCircuitingConditional(lhs, mhs, rhs), _, _) => {
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?;
				mhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
			}
			// Augmented assignments cannot be used outside a block or function
			(TanukiExpressionVariant::ExponentAssignment(_, _) | TanukiExpressionVariant::SaturatingExponentAssignment(_, _) | TanukiExpressionVariant::WrappingExponentAssignment(_, _) |
			TanukiExpressionVariant::MultiplicationAssignment(_, _) | TanukiExpressionVariant::SaturatingMultiplicationAssignment(_, _) |
			TanukiExpressionVariant::WrappingMultiplicationAssignment(_, _) | TanukiExpressionVariant::DivisionAssignment(_, _) |
			TanukiExpressionVariant::SaturatingDivisionAssignment(_, _) | TanukiExpressionVariant::WrappingDivisionAssignment(_, _) | TanukiExpressionVariant::ModuloAssignment(_, _) |
			TanukiExpressionVariant::SaturatingModuloAssignment(_, _) | TanukiExpressionVariant::WrappingModuloAssignment(_, _) | TanukiExpressionVariant::AdditionAssignment(_, _) |
			TanukiExpressionVariant::SaturatingAdditionAssignment(_, _) | TanukiExpressionVariant::WrappingAdditionAssignment(_, _) | TanukiExpressionVariant::SubtractionAssignment(_, _) |
			TanukiExpressionVariant::SaturatingSubtractionAssignment(_, _) | TanukiExpressionVariant::WrappingSubtractionAssignment(_, _) |
			TanukiExpressionVariant::ConcatenateAssignment(_, _) | TanukiExpressionVariant::AppendAssignment(_, _) | TanukiExpressionVariant::BitshiftLeftAssignment(_, _) |
			TanukiExpressionVariant::SaturatingBitshiftLeftAssignment(_, _) | TanukiExpressionVariant::WrappingBitshiftLeftAssignment(_, _) |
			TanukiExpressionVariant::BitshiftRightAssignment(_, _) | TanukiExpressionVariant::ThreeWayCompareAssignment(_, _) | TanukiExpressionVariant::NonShortCircuitAndAssignment(_, _) |
			TanukiExpressionVariant::NonShortCircuitNandAssignment(_, _) | TanukiExpressionVariant::NonShortCircuitXorAssignment(_, _) |
			TanukiExpressionVariant::NonShortCircuitXnorAssignment(_, _) | TanukiExpressionVariant::NonShortCircuitOrAssignment(_, _) |
			TanukiExpressionVariant::NonShortCircuitNorAssignment(_, _) | TanukiExpressionVariant::MinimumAssignment(_, _) | TanukiExpressionVariant::MaximumAssignment(_, _) |
			TanukiExpressionVariant::PipeAssignment(_, _) | TanukiExpressionVariant::ShortCircuitAndAssignment(_, _) | TanukiExpressionVariant::ShortCircuitNandAssignment(_, _) |
			TanukiExpressionVariant::ShortCircuitXorAssignment(_, _) | TanukiExpressionVariant::ShortCircuitXnorAssignment(_, _) | TanukiExpressionVariant::ShortCircuitOrAssignment(_, _) |
			TanukiExpressionVariant::ShortCircuitNorAssignment(_, _) | TanukiExpressionVariant::NonShortCircuitingNullCoalescingAssignment(_, _) |
			TanukiExpressionVariant::ShortCircuitingNullCoalescingAssignment(_, _), false, _)
				=> return Err(Error::AugmentedAssignmentUsedOnGlobalVariable.at(Some(self.start_line), Some(self.start_column), None)),
			// Augmented assignments
			(TanukiExpressionVariant::ExponentAssignment(lhs, rhs) | TanukiExpressionVariant::SaturatingExponentAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingExponentAssignment(lhs, rhs) |
			TanukiExpressionVariant::MultiplicationAssignment(lhs, rhs) | TanukiExpressionVariant::SaturatingMultiplicationAssignment(lhs, rhs) |
			TanukiExpressionVariant::WrappingMultiplicationAssignment(lhs, rhs) | TanukiExpressionVariant::DivisionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingDivisionAssignment(lhs, rhs) | TanukiExpressionVariant::WrappingDivisionAssignment(lhs, rhs) |
			TanukiExpressionVariant::ModuloAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingModuloAssignment(lhs, rhs) | TanukiExpressionVariant::WrappingModuloAssignment(lhs, rhs) |
			TanukiExpressionVariant::AdditionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingAdditionAssignment(lhs, rhs) | TanukiExpressionVariant::WrappingAdditionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SubtractionAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingSubtractionAssignment(lhs, rhs) | TanukiExpressionVariant::WrappingSubtractionAssignment(lhs, rhs) |
			TanukiExpressionVariant::ConcatenateAssignment(lhs, rhs) | TanukiExpressionVariant::AppendAssignment(lhs, rhs) |
			TanukiExpressionVariant::BitshiftLeftAssignment(lhs, rhs) |
			TanukiExpressionVariant::SaturatingBitshiftLeftAssignment(lhs, rhs) | TanukiExpressionVariant::WrappingBitshiftLeftAssignment(lhs, rhs) |
			TanukiExpressionVariant::BitshiftRightAssignment(lhs, rhs) | TanukiExpressionVariant::ThreeWayCompareAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitAndAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNandAssignment(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitXorAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitXnorAssignment(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitOrAssignment(lhs, rhs) |
			TanukiExpressionVariant::NonShortCircuitNorAssignment(lhs, rhs) | TanukiExpressionVariant::MinimumAssignment(lhs, rhs) |
			TanukiExpressionVariant::MaximumAssignment(lhs, rhs) |
			TanukiExpressionVariant::PipeAssignment(lhs, rhs) | TanukiExpressionVariant::ShortCircuitAndAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNandAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitXorAssignment(lhs, rhs) | TanukiExpressionVariant::ShortCircuitXnorAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitOrAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitNorAssignment(lhs, rhs) | TanukiExpressionVariant::NonShortCircuitingNullCoalescingAssignment(lhs, rhs) |
			TanukiExpressionVariant::ShortCircuitingNullCoalescingAssignment(lhs, rhs) | TanukiExpressionVariant::Assignment(lhs, rhs), true, _) => {
				lhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, true, global_variables_dependent_on, local_variables)?;
				rhs.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?;
			}
			(TanukiExpressionVariant::Block { sub_expressions, has_return_value: _ }, _, _) => {
				let sub_expressions_length = sub_expressions.len();
				local_variables.push(HashSet::new());
				for (index, sub_expression) in sub_expressions.iter_mut().enumerate() {
					if index == sub_expressions_length - 1 {
						sub_expression.post_parse(main, post_parse_data, true, None, is_l_value, global_variables_dependent_on, local_variables)?;
					}
					else {
						sub_expression.post_parse(main, post_parse_data, true, None, false, global_variables_dependent_on, local_variables)?;
					}
				}
				local_variables.pop();
			}
			(TanukiExpressionVariant::FunctionCall { function_pointer, arguments }, _, _) => {
				if is_l_value {
					return Err(Error::NotYetImplemented("Functions that return l-values".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				function_pointer.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?;
				for argument in arguments {
					argument.post_parse(main, post_parse_data, is_inside_function_or_block, None, false, global_variables_dependent_on, local_variables)?;
				}
			}
			(TanukiExpressionVariant::Variable(name), _, _) => {
				if name.starts_with("_tnk_") {
					return Err(Error::VariableStartsWithTnk.at(Some(self.start_line), Some(self.start_column), None));
				}
				let mut local_variable_exists = false;
				for local_variable_level in local_variables.iter() {
					if local_variable_level.contains(name) {
						local_variable_exists = true;
						break;
					}
				}
				if !local_variable_exists {
					if is_l_value && is_inside_function_or_block {
						local_variables.last_mut().unwrap().insert(name.clone());
					}
					if !is_l_value {
						global_variables_dependent_on.insert(name.clone());
					}
				}
			}
			(TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression }, _, false) => {
				let mut function_depends_on_globals_for_execution = HashSet::new();
				let mut function_local_variables = Vec::new();
				function_local_variables.push(HashSet::new());
				// Parse sub-expressions
				for parameter in parameters.iter_mut() {
					parameter.post_parse(
						main, post_parse_data, true, None, true, &mut function_depends_on_globals_for_execution, &mut function_local_variables
					)?;
				}
				if let Some(return_type) = return_type {
					return_type.post_parse(
						main, post_parse_data, true, None, false, &mut function_depends_on_globals_for_execution, &mut function_local_variables
					)?;
				}
				body_expression.post_parse(main, post_parse_data, true, None, false, &mut function_depends_on_globals_for_execution, &mut function_local_variables)?;
				//
				let mut new_parameters = Vec::new();
				for parameter in take(parameters) {
					new_parameters.push(match parameter.variant {
						TanukiExpressionVariant::Variable(name) => TanukiFunctionArgument {
							t_type: None, name, start_line: parameter.start_line, start_column: parameter.start_column, end_line: parameter.end_line, end_column: parameter.end_column
						},
						TanukiExpressionVariant::TypeAndValue(t_type, name_expression) => match name_expression.variant {
							TanukiExpressionVariant::Variable(name) => TanukiFunctionArgument {
								t_type: Some(*t_type), name, start_line: parameter.start_line, start_column: parameter.start_column, end_line: parameter.end_line, end_column: parameter.end_column
							},
							_ => return Err(Error::ExpectedVariable.at(Some(parameter.start_line), Some(parameter.start_column), None)),
						}
						_ => return Err(Error::ExpectedVariable.at(Some(parameter.start_line), Some(parameter.start_column), None)),
					});
				}
				let module_function_index = post_parse_data.functions.len();
				let mangled_function_name = format!("_tnk_fn_{module_function_index}").into_boxed_str();
				global_variables_dependent_on.insert(mangled_function_name.clone());
				post_parse_data.functions.push(TanukiFunction {
					name: mangled_function_name, parameters: new_parameters.into_boxed_slice(),
					return_type: take(return_type).map(|return_type| *return_type),
					body: take(body_expression), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column,
					depends_on_for_execution: function_depends_on_globals_for_execution, is_pure: false,
				});
				*self = TanukiExpression {
					variant: TanukiExpressionVariant::ModuleFunction { module_function_index }, start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				};
			}
			(TanukiExpressionVariant::FunctionDefinition { .. }, _, true) =>
				return Err(Error::ExpressionCannotBeLValue.at(Some(self.start_line), Some(self.start_column), None)),
			(TanukiExpressionVariant::Constant(..), _, _) => {},
			(TanukiExpressionVariant::ModuleFunction { .. }, _, _) => unreachable!(),
			(TanukiExpressionVariant::Export(to_export), false, _) => {
				to_export.post_parse(main, post_parse_data, is_inside_function_or_block, None, is_l_value, global_variables_dependent_on, local_variables)?;
				match &mut to_export.variant {
					TanukiExpressionVariant::Variable(name) => {
						post_parse_data.exports.push(TanukiExport { name: name.clone(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column });
						*self = take(to_export);
					},
					_ => return Err(Error::ExpectedVariable.at(Some(self.start_line), Some(self.start_column), None)),
				}
			},
			(TanukiExpressionVariant::Import(arguments), false, _) => {
				if arguments.len() != 1 {
					return Err(Error::Unimplemented("@import with argument count that is not one".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				let assigned_to_name = assigned_to_name.unwrap();
				let argument = &arguments[0];
				let argument = match &argument.variant {
					TanukiExpressionVariant::Constant(TanukiConstantValue::CompileTimeString(path)) => &**path,
					_ => return Err(Error::Unimplemented("@import with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
				};
				let mut module = main.module_being_processed.parent().unwrap().to_path_buf();
				module.push(argument);
				main.add_module_to_compile((module.clone().into_boxed_path(), false));
				post_parse_data.imports.push(TanukiImport {
					name: assigned_to_name.into(), module_from: module.into_boxed_path(), start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				});
			},
			(TanukiExpressionVariant::Link(arguments), false, _) => {
				if arguments.len() != 1 {
					return Err(Error::Unimplemented("@link with argument count that is not one".into()).at(Some(self.start_line), Some(self.start_column), None));
				}
				let assigned_to_name = assigned_to_name.unwrap();
				let argument = &arguments[0];
				let argument = match &argument.variant {
					TanukiExpressionVariant::Constant(TanukiConstantValue::CompileTimeString(path)) => &**path,
					_ => return Err(Error::Unimplemented("@link with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
				};
				let mut dynamic_library_path = main.module_being_processed.parent().unwrap().to_path_buf();
				dynamic_library_path.push(argument);
				post_parse_data.links.push(TanukiLink {
					name: assigned_to_name.into(), dynamic_library_path: dynamic_library_path.into_boxed_path(),
					start_line: self.start_line, start_column: self.start_column, end_line: self.end_line, end_column: self.end_column
				});
			},
			(TanukiExpressionVariant::Export(..) | TanukiExpressionVariant::Import(..) | TanukiExpressionVariant::Link(..), true, _)
				=> return Err(Error::CannotBeInsideBlockOrFunction.at(Some(self.start_line), Some(self.start_column), None)),
		}
		Ok(())
	}
}