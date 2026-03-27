use std::{collections::HashSet, num::NonZeroUsize, path::PathBuf};

use num::ToPrimitive;

use crate::{Main, Os, error::{Error, ErrorAt}, maybe_parsed_token::MaybeParsedToken, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, module::TanukiModule, t_type::TanukiType, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiKeyword, TanukiPrefixUnaryOperator, TanukiToken, TanukiTokenVariant}}, token_reader::TokenReader};

#[derive(Debug, Clone)]
pub struct TanukiPartiallyParsedToken {
	pub variant: TanukiPartiallyParsedTokenVariant,
	pub start_line: NonZeroUsize,
	pub start_column: NonZeroUsize,
	pub end_line: NonZeroUsize,
	pub end_column: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub enum TanukiPartiallyParsedTokenVariant {
	/// Expressions enclosed between parentheses containing the parameter/argument expressions and the return type expression.
	/// From `(expression, expression; return_type)` or `(expression, expression)` or `(;return_type)`, ect..
	FunctionArgumentsOrParameters(Box<[TanukiExpression]>, Option<Box<TanukiExpression>>),
	/// An expression between square parentheses.
	SquareParenthesised(Box<TanukiExpression>),
	/// A ternary operator, the matching colon and the expression in between.
	TernaryOperatorCenter(TanukiInfixTernaryOperator, Box<TanukiExpression>),
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
			parsed_expressions: expressions.into_boxed_slice(), functions: Vec::new(), global_constants: Vec::new(),
			entrypoint: None, mangled_module_names_to_include_in_c: HashSet::new()
		})
	}
}

impl TanukiExpression {
	pub fn parse(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Option<Self>, ErrorAt> {
		if token_reader.peek().is_none() {
			return Ok(None);
		}
		let mut maybe_parsed_tokens = Vec::new();
		// Loop through all tokens until we reach the end of the expression
		while matches!(token_reader.peek().map(|token| &token.variant), Some(..)) {
			// If we reach a separator that is'int an opening separator, break
			let token = &token_reader.peek().unwrap().variant;
			if matches!(token, TanukiTokenVariant::RightParenthesis | TanukiTokenVariant::RightCurlyParenthesis | TanukiTokenVariant::RightSquareParenthesis | TanukiTokenVariant::Comma | TanukiTokenVariant::Semicolon) {
				break;
			}
			// Get and unpack first token
			let token = token_reader.next().unwrap().clone();
			let token_start_line = token.start_line;
			let token_start_column = token.start_column;
			let token_end_line = token.end_line;
			let token_end_column = token.end_column;
			// Parse depending on first token
			maybe_parsed_tokens.push(match &token.variant {
				// Single token expressions
				TanukiTokenVariant::NumericLiteral(None, Some(float_value)) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeFloat(*float_value)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::NumericLiteral(Some(int_value), _) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeInt(int_value.clone().into())),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::NumericLiteral(None, None) => unreachable!(),
				TanukiTokenVariant::CharacterLiteral(value) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeChar(*value)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::StringLiteral(value) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(value.clone())),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Identifier(name) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Variable(name.clone()),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::Bool) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(TanukiType::Bool)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::Void) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Void),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::True) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Bool(true)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::False) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Bool(false)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::Int) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(TanukiType::CompileTimeInt)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				TanukiTokenVariant::Keyword(TanukiKeyword::Type) => MaybeParsedToken::Parsed(TanukiExpression {
					variant: TanukiExpressionVariant::Constant(TanukiCompileTimeValue::Type(TanukiType::Type)),
					start_line: token_start_line, start_column: token_start_column, end_line: token_end_line, end_column: token_end_column
				}),
				// If there is a block
				TanukiTokenVariant::LeftCurlyParenthesis => {
					// Parse each sub-expression
					let mut sub_expressions = Vec::new();
					let mut return_expressions = Vec::new();
					let mut is_parsing_return_expressions = false;
					loop {
						// Parse expression
						let sub_expression = Self::parse(main, token_reader)?;
						// Next token should be a separator
						let mut end_pos = None;
						match token_reader.next() {
							// Right curly bracket ends the block expression
							Some(TanukiToken { variant: TanukiTokenVariant::RightCurlyParenthesis, end_line, end_column, .. }) => {
								is_parsing_return_expressions = true;
								end_pos = Some((end_line, end_column));
							},
							// The token stream should not just stop
							None => return Err(Error::ExpectedCurlyClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Comma separates return expression
							Some(TanukiToken { variant: TanukiTokenVariant::Comma, .. }) => {
								is_parsing_return_expressions = true;
							},
							// Semicolon separates non-return expression
							Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, .. }) => {
								if is_parsing_return_expressions {
									return Err(Error::ExpectedComma.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None));
								}
							},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. }) if !is_parsing_return_expressions
								=> return Err(Error::ExpectedSemicolon.at(Some(*start_column), Some(*end_column), None)),
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedComma.at(Some(*start_column), Some(*end_column), None)),
						}
						// Push sub-expression
						if is_parsing_return_expressions {
							let sub_expression = match sub_expression {
								Some(sub_expression) => sub_expression,
								None => return Err(Error::ExpectedExpression.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							};
							match sub_expression {
								TanukiExpression {
									variant: TanukiExpressionVariant::Assignment(lhs_expression, rhs_expression), ..
								} if matches!(lhs_expression.variant, TanukiExpressionVariant::PrefixUnaryOperator(TanukiPrefixUnaryOperator::MemberAccess, _)) => {
									let name_expression = match lhs_expression.variant {
										TanukiExpressionVariant::PrefixUnaryOperator(_, name_expression) => name_expression,
										_ => unreachable!(),
									};
									let name = match name_expression.variant {
										TanukiExpressionVariant::Variable(name) => name,
										_ => return Err(Error::ExpectedVariable.at(Some(name_expression.start_line), Some(name_expression.start_column), None)),
									};
									return_expressions.push((Some(name), *rhs_expression));
								}
								_ => return_expressions.push((None, sub_expression)),
							}
						}
						else if let Some(sub_expression) = sub_expression {
							sub_expressions.push(sub_expression);
						}
						// Return
						if let Some((end_line, end_column)) = end_pos {
							break MaybeParsedToken::Parsed(TanukiExpression {
								variant: TanukiExpressionVariant::Block { sub_expressions: sub_expressions.into(), return_expressions: return_expressions.into() },
								start_line: token_start_line, start_column: token_start_column, end_line: *end_line, end_column: *end_column,
							})
						}
					}
				},
				// Function arguments or parameters
				TanukiTokenVariant::LeftParenthesis => 'a: {
					// Parse each sub-expression
					let mut sub_expressions = Vec::new();
					let mut return_type_expression = None;
					let mut is_return_type_expression = false;
					loop {
						// Parse expression
						let mut expression_is_empty = false;
						if let Some(sub_expression) = Self::parse(main, token_reader)? {
							if !is_return_type_expression {
								sub_expressions.push(sub_expression);
							}
							else {
								return_type_expression = Some(Box::new(sub_expression));
							}
						}
						else {
							if !is_return_type_expression {
								expression_is_empty = true;
							}
							else {
								return Err(Error::ExpectedExpression.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None));
							}
						}
						// Next token should be a ) token
						match token_reader.next() {
							Some(token) if is_return_type_expression && !matches!(token, TanukiToken { variant: TanukiTokenVariant::RightParenthesis, .. }) =>
								return Err(Error::ExpectedClosingParenthesis.at(Some(token.start_line), Some(token.start_column), None)),
							// Right bracket ends the block expression
							Some(TanukiToken { variant: TanukiTokenVariant::RightParenthesis, end_line, end_column, .. })
								=> break 'a MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken
							{
								variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(sub_expressions.into(), return_type_expression),
								start_line: token_start_line, start_column: token_start_column, end_line: *end_line, end_column: *end_column,
							}),
							// The token stream should not just stop
							None => return Err(Error::ExpectedClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Move on to the next sub-expression if we read a comma
							Some(TanukiToken { variant: TanukiTokenVariant::Comma, start_line, start_column, .. }) => {
								if expression_is_empty {
									return Err(Error::ExpectedExpression.at(Some(*start_line), Some(*start_column), None));
								}
							},
							// Move on to reading the return type if we reach a semicolon
							Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, start_line, start_column, .. }) => {
								if expression_is_empty && !sub_expressions.is_empty() {
									return Err(Error::ExpectedExpression.at(Some(*start_line), Some(*start_column), None));
								}
								is_return_type_expression = true;
							},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedComma.at(Some(*start_column), Some(*end_column), None)),
						}
					}
				},
				// Square parentheses
				TanukiTokenVariant::LeftSquareParenthesis => {
					// Parse expression
					let sub_expression = Self::parse_expected(main, token_reader)?;
					// Take closing square parenthesis
					match token_reader.next() {
						Some(TanukiToken { variant: TanukiTokenVariant::RightSquareParenthesis, .. }) => {},
						Some(TanukiToken { start_line, start_column, .. })
							=> return Err(Error::ExpectedSquareClosingParenthesis.at(Some(*start_line), Some(*start_column), None)),
						None => return Err(Error::ExpectedSquareClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
					}
					// Assemble into value
					MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						start_line: token_start_line, start_column: token_end_column, end_line: token_reader.last_token_end_line(), end_column: token_reader.last_token_end_line(),
						variant: TanukiPartiallyParsedTokenVariant::SquareParenthesised(Box::new(sub_expression)),
					})
				},
				// Else we will need to do more complex parsing later
				_ => MaybeParsedToken::Unparsed(token),
			});
		}
		// Return if there are no tokens parsed
		if maybe_parsed_tokens.is_empty() {
			return Ok(None);
		}
		// Do more complex parsing
		Ok(Some(Self::parse_maybe_parsed_tokens(main, maybe_parsed_tokens)?))
	}

	/// Do more complex parsing, eg. 1 + 2 * -a.
	pub fn parse_maybe_parsed_tokens(main: &mut Main, mut maybe_parsed_tokens: Vec<MaybeParsedToken<TanukiExpression, TanukiPartiallyParsedToken, TanukiToken>>) -> Result<TanukiExpression, ErrorAt> {
		// Loop until we have parsed all tokens or parsing has stalled
		loop {
			let maybe_parsed_tokens_at_start = maybe_parsed_tokens.len();
			// Parse postfix operators and function calls
			let mut x = 0;
			'a: while x < maybe_parsed_tokens.len() - 1 {
				// Skip if this is not in the order (parsed expression, operator, non-parsed_expression) or (parsed expression function arguments)
				if (
					!maybe_parsed_tokens[x].is_parsed() || (!matches!(
						maybe_parsed_tokens[x + 1], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { postfix_unary_operator: Some(..), is_assignment: false, .. }, .. })
					) || matches!(maybe_parsed_tokens.get(x + 2), Some(token) if token.is_parsed()))
				) && (!maybe_parsed_tokens[x].is_parsed() || (!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
					variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..) | TanukiPartiallyParsedTokenVariant::SquareParenthesised(..), ..
				})))) && (!matches!(maybe_parsed_tokens[x], 
					MaybeParsedToken::Unparsed(
						TanukiToken { variant: TanukiTokenVariant::Keyword(
							TanukiKeyword::Import | TanukiKeyword::ImportStd | TanukiKeyword::Link | TanukiKeyword::LinkIf | TanukiKeyword::U | TanukiKeyword::I | TanukiKeyword::F | TanukiKeyword::Info |
							TanukiKeyword::Transmute
						), .. }
					)) || !matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..), .. }))
				) {
					x += 1;
					continue;
				}
				// Parse builtin functions
				if matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Keyword(
					TanukiKeyword::Import | TanukiKeyword::ImportStd | TanukiKeyword::Link | TanukiKeyword::LinkIf | TanukiKeyword::U | TanukiKeyword::I | TanukiKeyword::F | TanukiKeyword::Info |
					TanukiKeyword::Transmute
				), .. })) {
					let operand = maybe_parsed_tokens[x].clone().unwrap_unparsed();
					let (keyword, start_line, start_column) = match operand {
						TanukiToken { variant: TanukiTokenVariant::Keyword(keyword), start_line, start_column, .. } => (keyword, start_line, start_column),
						_ => unreachable!()
					};
					let (arguments, end_line, end_column) = match maybe_parsed_tokens.remove(x + 1).unwrap_partially_parsed() {
						TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(arguments, return_type), end_line, end_column, .. } => {
							if let Some(return_type) = return_type {
								return Err(Error::UnexpectedReturnType.at(Some(return_type.start_line), Some(return_type.start_column), None));
							}
							(arguments, end_line, end_column)
						}
						_ => unreachable!(),
					};
					// Parse depending on function
					maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression { variant: match keyword {
						TanukiKeyword::Import | TanukiKeyword::ImportStd => {
							if arguments.len() != 1 {
								return Err(Error::Unimplemented("Import with argument count that is not one".into()).at(Some(start_line), Some(start_column), None));
							}
							let argument = &arguments[0];
							let argument = match &argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
								_ => return Err(Error::Unimplemented("Import with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
							};
							let mut module_path = match keyword {
								TanukiKeyword::Import    => main.module_being_processed.parent().unwrap().to_path_buf(),
								TanukiKeyword::ImportStd => main.tnk_std_directory.clone().into_path_buf(),
								_ => unreachable!(),
							};
							module_path.push(argument);
							let mut new_path = PathBuf::new();
							for component in module_path.components() {
								let component = component.as_os_str();
								if component.to_string_lossy() == ".." {
									new_path.pop();
								}
								else {
									new_path.push(component);
								}
							}
							main.add_module_to_compile((new_path.clone().into_boxed_path(), false));
							TanukiExpressionVariant::ImportConstant { name: None, module_path: new_path.clone().into_boxed_path() }
						},
						TanukiKeyword::Link => {
							let arguments_len = arguments.len();
							if arguments_len < 2 {
								return Err(Error::Unimplemented("@link with argument count that is less than two".into()).at(Some(start_line), Some(start_column), None));
							}
							let mut arguments = arguments.into_iter();
							// Get the library path
							let library_path_argument = arguments.next().unwrap();
							let library_path_argument = match &library_path_argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
								_ => return Err(
									Error::Unimplemented("@link with library path argument that is not a string".into())
										.at(Some(library_path_argument.start_line), Some(library_path_argument.start_column), None)
								),
							};
							let library_path: PathBuf = library_path_argument.into();
							// Get the arguments
							let mut argument_types = Vec::new();
							for _ in 0..arguments_len - 2 {
								argument_types.push(arguments.next().unwrap());
							}
							// Get the return type
							let return_type = arguments.next().unwrap();
							// Return
							TanukiExpressionVariant::Link {
								name: None, library_path: library_path.into_boxed_path(), parameter_types: argument_types.into_boxed_slice(), return_type: Some(return_type.into()), link_if: None
							}
						},
						TanukiKeyword::LinkIf => {
							let arguments_len = arguments.len();
							if arguments_len < 3 {
								return Err(Error::Unimplemented("@link_if with argument count that is less than three".into()).at(Some(start_line), Some(start_column), None));
							}
							let mut arguments = arguments.into_iter();
							// Get the library path
							let library_path_argument = arguments.next().unwrap();
							let library_path_argument = match &library_path_argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
								_ => return Err(
									Error::Unimplemented("@link with library path argument that is not a string".into())
										.at(Some(library_path_argument.start_line), Some(library_path_argument.start_column), None)
								),
							};
							let library_path: PathBuf = library_path_argument.into();
							// Get the arguments
							let mut argument_types = Vec::new();
							for _ in 0..arguments_len - 3 {
								argument_types.push(arguments.next().unwrap());
							}
							// Get the return type
							let return_type = arguments.next().unwrap();
							// Get the linking condition
							let link_if = arguments.next().unwrap();
							// Return
							TanukiExpressionVariant::Link {
								name: None, library_path: library_path.into_boxed_path(), parameter_types: argument_types.into_boxed_slice(), return_type: Some(return_type.into()), link_if: Some(link_if.into())
							}
						},
						TanukiKeyword::Info => {
							if arguments.len() != 1 {
								return Err(Error::Unimplemented("@_info with argument count that is not one".into()).at(Some(start_line), Some(start_column), None));
							}
							let argument = &arguments[0];
							let argument_value = match &argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeInt(argument_value)) => argument_value,
								_ => return Err(Error::Unimplemented("@_info with argument that is not an integer".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
							};
							let argument_value = match argument_value.to_u8() {
								Some(0) => TanukiCompileTimeValue::Bool(main.os == Os::Windows), // Is windows
								Some(1) => TanukiCompileTimeValue::Bool(main.os == Os::Unix), // Is unix
								_ => return Err(Error::Unimplemented("This info constant".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
							};
							TanukiExpressionVariant::Constant(argument_value)
						}
						TanukiKeyword::Transmute => {
							if arguments.len() != 2 {
								return Err(Error::Unimplemented("@transmute with argument count that is not two".into()).at(Some(start_line), Some(start_column), None));
							}
							TanukiExpressionVariant::Transmute { to_transmute: arguments[0].clone().into(), transmute_to_type: arguments[1].clone().into() }
						}
						TanukiKeyword::U => TanukiExpressionVariant::U(arguments),
						TanukiKeyword::I => TanukiExpressionVariant::I(arguments),
						TanukiKeyword::F => TanukiExpressionVariant::F(arguments),
						_ => unreachable!(),
					}, start_line, start_column, end_column, end_line });
					continue 'a;
				}
				// Else parse postfix operators and non-builtin function calls
				let operand = maybe_parsed_tokens[x].clone().unwrap_parsed();
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match maybe_parsed_tokens.remove(x + 1) {
					// Postfix operators
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Operator { postfix_unary_operator, symbol, .. }, start_line, start_column, end_line, end_column
					}) => TanukiExpression { start_line: operand.start_line, start_column: operand.start_column, variant: match postfix_unary_operator {
						Some(operator) => TanukiExpressionVariant::PostfixUnaryOperator(operator, Box::new(operand)),
						None => return Err(Error::InvalidPostfixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
					}, end_line, end_column },
					// Function calls
					MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(arguments, return_type), end_line, end_column, ..
					}) => {
						if let Some(return_type) = return_type {
							return Err(Error::UnexpectedReturnType.at(Some(return_type.start_line), Some(return_type.start_column), None));
						}
						TanukiExpression {
							start_line: operand.start_line, start_column: operand.start_column, variant: TanukiExpressionVariant::FunctionCall { function_pointer: Box::new(operand), arguments },
							end_line, end_column,
						}
					},
					// Index operator
					MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						variant: TanukiPartiallyParsedTokenVariant::SquareParenthesised(index), end_line, end_column, ..
					}) => TanukiExpression {
						start_line: operand.start_line, start_column: operand.start_column, variant: TanukiExpressionVariant::Index(Box::new(operand), index),
						end_line, end_column,
					},
					// We already checked that it's not these
					MaybeParsedToken::PartiallyParsed(..) | MaybeParsedToken::Parsed(..) | MaybeParsedToken::Unparsed(TanukiToken { .. }) => unreachable!(),
				});
			}
			// Partially parse ternary conditional operators
			let mut x = maybe_parsed_tokens.len().saturating_sub(1);
			loop {
				// Skip if this is not a colon
				if !matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_colon: true, .. }, .. })) {
					x = match x.checked_sub(1) {
						Some(x) => x,
						None => break,
					};
					continue;
				}
				// Remove colon
				let colon_token = maybe_parsed_tokens.remove(x).unwrap_unparsed();
				// Make sure we are not at the end or start of the tokens
				if x == maybe_parsed_tokens.len() {
					return Err(Error::ColonAtExpressionEnd.at(Some(colon_token.start_line), Some(colon_token.start_column), None))
				}
				if x == 0 {
					return Err(Error::ColonWithoutMatchingTernaryOperator.at(Some(colon_token.start_line), Some(colon_token.start_column), None))
				}
				//
				let mut y = x - 1;
				let mut depth = 0usize;
				loop {
					match maybe_parsed_tokens[y] {
						MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_colon: true, .. }, ..}) => depth += 1,
						MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { infix_ternary_operator: Some(..), .. }, ..}) if depth > 0 => depth -= 1,
						MaybeParsedToken::Unparsed(TanukiToken {
							variant: TanukiTokenVariant::Operator { infix_ternary_operator: Some(ternary_operator), .. }, start_line, start_column, ..
						}) => {
							let middle_expression_maybe_parsed_tokens = maybe_parsed_tokens.drain(y + 1..x).collect();
							let middle_expression = Self::parse_maybe_parsed_tokens(main, middle_expression_maybe_parsed_tokens)?;
							x = y;
							maybe_parsed_tokens[x] = MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
								variant: TanukiPartiallyParsedTokenVariant::TernaryOperatorCenter(ternary_operator, Box::new(middle_expression)),
								start_line, start_column, end_line: colon_token.end_line, end_column: colon_token.end_column
							});
							break;
						}
						_ => {},
					}
					y = match y.checked_sub(1) {
						Some(y) => y,
						None => return Err(Error::ColonWithoutMatchingTernaryOperator.at(Some(colon_token.start_line), Some(colon_token.start_column), None)),
					};
				}
			}
			// Parse prefix operators
			let mut x = maybe_parsed_tokens.len().saturating_sub(2);
			loop {
				// Skip if this is not in the order parsed expression, operator, non-parsed expression
				if (
					!matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_assignment: false, .. }, .. })) ||
					((!maybe_parsed_tokens.get(x + 1).is_some_and(|token| token.is_parsed()) ||
					(x > 0 && !maybe_parsed_tokens[x - 1].is_unparsed()) || x >= maybe_parsed_tokens.len() - 1))
				) && (
					x >= maybe_parsed_tokens.len() - 1 || (!maybe_parsed_tokens[x].is_parsed() || !maybe_parsed_tokens[x + 1].is_parsed())
				) && (
					!matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Keyword(TanukiKeyword::Export | TanukiKeyword::Entrypoint), .. })) ||
					!maybe_parsed_tokens.get(x + 1).is_some_and(|token| token.is_parsed())
				) {
					x = match x.checked_sub(1) {
						Some(x) => x,
						None => break,
					};
					continue;
				}
				// Parse
				let operand = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match maybe_parsed_tokens[x].clone() {
					// Prefix operators
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Operator { prefix_unary_operator, symbol, .. }, start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, variant: match prefix_unary_operator {
						Some(operator) => TanukiExpressionVariant::PrefixUnaryOperator(operator, Box::new(operand)),
						None => return Err(Error::InvalidPrefixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
					}, start_line, start_column },
					// Operator-like keywords
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Keyword(TanukiKeyword::Export), start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, start_line, start_column, variant: TanukiExpressionVariant::Export(Box::new(operand))},
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Keyword(TanukiKeyword::Entrypoint), start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, start_line, start_column, variant: TanukiExpressionVariant::Entrypoint(Box::new(operand))},
					// Two expressions next to one another are a type-value pair
					MaybeParsedToken::Parsed(type_expression) => TanukiExpression {
						end_line: operand.end_line, end_column: operand.end_column, start_line: operand.start_line, start_column: operand.start_column,
						variant: TanukiExpressionVariant::TypeAndValue(Box::new(type_expression), Box::new(operand)),
					},
					// We already checked that it's not these
					MaybeParsedToken::Unparsed(TanukiToken { variant: _, .. }) | MaybeParsedToken::PartiallyParsed(..) => unreachable!(),
				});
			}
			// Parse infix binary operators
			for precedence_level in TanukiInfixBinaryOperator::PRECEDENCE_LEVELS {
				let mut x = 0;
				while x < maybe_parsed_tokens.len().saturating_sub(2) {
					// Skip if this is not in the order parsed expression, operator, parsed expression
					if !maybe_parsed_tokens[x].is_parsed() ||
						!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_assignment: false, .. }, .. })) ||
						!maybe_parsed_tokens[x + 2].is_parsed()
					{
						x += 1;
						continue;
					}
					// Skip if the operator should not be parsed for this precedence level
					match &maybe_parsed_tokens[x + 1] {
						MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { infix_binary_operator, symbol, .. }, start_line, start_column, .. }) => {
							if infix_binary_operator.is_none() {
								return Err(Error::InvalidInfixBinaryOperator(symbol.clone().into_string()).at(Some(*start_line), Some(*start_column), None));
							}
							if !precedence_level.contains(&infix_binary_operator.unwrap()) {
								x += 1;
								continue;
							}
						}
						_ => unreachable!(),
					}
					// Parse
					let lhs = maybe_parsed_tokens[x].clone().unwrap_parsed();
					let operator = maybe_parsed_tokens.remove(x + 1).unwrap_unparsed();
					let rhs = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
					maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match operator {
						TanukiToken {
							variant: TanukiTokenVariant::Operator { infix_binary_operator, symbol, .. }, start_line: operator_start_line, start_column: operator_start_column, ..
						} => TanukiExpression { start_line: lhs.start_line, start_column: lhs.start_column, end_line: rhs.end_line, end_column: rhs.end_column, variant: match infix_binary_operator {
							Some(operator) => TanukiExpressionVariant::InfixBinaryOperator(operator, Box::new(lhs), Box::new(rhs)),
							None => return Err(Error::InvalidInfixBinaryOperator(symbol.into_string()).at(Some(operator_start_line), Some(operator_start_column), None)),
						}},
						_ => unreachable!()
					});
				}
			}
			// Parse ternary conditional operators
			let mut x = maybe_parsed_tokens.len().saturating_sub(1);
			loop {
				// Skip if this is not a partially parsed ternary conditional operator
				if x >= maybe_parsed_tokens.len() ||
					!matches!(maybe_parsed_tokens[x], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::TernaryOperatorCenter(..), .. }))
				{
					x = match x.checked_sub(1) {
						Some(x) => x,
						None => break,
					};
					continue;
				}
				// Get operator and operands
				let operator = maybe_parsed_tokens.remove(x).unwrap_partially_parsed();
				if x == maybe_parsed_tokens.len() {
					return Err(Error::ColonAtExpressionEnd.at(Some(operator.end_line), Some(operator.end_column), None));
				}
				let rhs = maybe_parsed_tokens.remove(x);
				if x == 0 {
					return Err(Error::ExpectedExpression.at(Some(operator.start_line), Some(operator.start_column), None));
				}
				x -= 1;
				let lhs = maybe_parsed_tokens[x].clone();
				// Make sure the operands are correct
				let lhs = match lhs {
					MaybeParsedToken::Parsed(lhs) => lhs,
					_ => return Err(Error::ExpectedExpression.at(Some(operator.start_line), Some(operator.start_column), None)),
				};
				let rhs = match rhs {
					MaybeParsedToken::Parsed(rhs) => rhs,
					_ => return Err(Error::ExpectedExpression.at(Some(operator.end_line), Some(operator.end_column), None)),
				};
				// Parse
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression { start_line: lhs.start_line, start_column: lhs.start_column, end_line: rhs.end_line, end_column: rhs.end_column, variant: match operator {
					TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::TernaryOperatorCenter(ternary_operator, middle_operand), .. } =>
						TanukiExpressionVariant::InfixTernaryOperator(ternary_operator, Box::new(lhs), middle_operand, Box::new(rhs)),
					_ => unreachable!(),
				} })
			}
			// Parse function definitions
			let mut x = maybe_parsed_tokens.len().saturating_sub(2);
			loop {
				// Skip if this is not in the order parsed expression, operator, non-parsed expression
				if x >= maybe_parsed_tokens.len().saturating_sub(1) ||
					!matches!(maybe_parsed_tokens[x], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..), .. })) ||
					!maybe_parsed_tokens[x + 1].is_parsed()
				{
					x = match x.checked_sub(1) {
						Some(x) => x,
						None => break,
					};
					continue;
				}
				// Parse
				let function_body_expression = maybe_parsed_tokens.remove(x + 1).unwrap_parsed();
				let function_parameters = maybe_parsed_tokens[x].clone().unwrap_partially_parsed();
				let (parameters, return_type) = match function_parameters {
					TanukiPartiallyParsedToken { variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(parameters, return_type), .. } =>
						(parameters, return_type) ,
					_ => unreachable!(),
				};
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression {
					start_line: function_parameters.start_line, start_column: function_parameters.start_column, end_line: function_body_expression.end_line, end_column: function_body_expression.end_column,
					variant: TanukiExpressionVariant::FunctionDefinition { parameters, return_type, body_expression: Box::new(function_body_expression) }
				});
			}
			// Parse assignments
			let mut x = maybe_parsed_tokens.len().saturating_sub(1);
			loop {
				// Skip if this is not a partially parsed ternary conditional operator
				if x >= maybe_parsed_tokens.len() ||
					!matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { is_assignment: true, .. }, .. })) ||
					x == 0 || !maybe_parsed_tokens[x - 1].is_parsed() || x == maybe_parsed_tokens.len().strict_sub(1) || !maybe_parsed_tokens[x + 1].is_parsed()
				{
					x = match x.checked_sub(1) {
						Some(x) => x,
						None => break,
					};
					continue;
				}
				// Get operator and operands
				let operator = maybe_parsed_tokens.remove(x).unwrap_unparsed();
				if x == maybe_parsed_tokens.len() {
					return Err(Error::ExpectedExpression.at(Some(operator.end_line), Some(operator.end_column), None));
				}
				let rhs = maybe_parsed_tokens.remove(x);
				if x == 0 {
					return Err(Error::ExpectedExpression.at(Some(operator.start_line), Some(operator.start_column), None));
				}
				x -= 1;
				let lhs = maybe_parsed_tokens[x].clone();
				// Make sure the operands are correct
				let lhs = match lhs {
					MaybeParsedToken::Parsed(lhs) => lhs,
					_ => return Err(Error::ExpectedExpression.at(Some(operator.start_line), Some(operator.start_column), None)),
				};
				let rhs = match rhs {
					MaybeParsedToken::Parsed(rhs) => rhs,
					_ => return Err(Error::ExpectedExpression.at(Some(operator.end_line), Some(operator.end_column), None)),
				};
				// Parse
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression { start_line: lhs.start_line, start_column: lhs.start_column, end_line: rhs.end_line, end_column: rhs.end_column, variant: match operator {
					TanukiToken { variant: TanukiTokenVariant::Operator { infix_binary_operator, symbol, .. }, .. } => match infix_binary_operator {
						_ if symbol.as_ref() == "=" => TanukiExpressionVariant::Assignment(Box::new(lhs), Box::new(rhs)),
						Some(operator) => TanukiExpressionVariant::AugmentedBinaryAssignment(operator, Box::new(lhs), Box::new(rhs)),
						_ => return Err(Error::InvalidAssignmentOperator(symbol.into_string()).at(Some(operator.end_line), Some(operator.end_column), None)),
					}
					_ => unreachable!(),
				} })
			}
			// Break the parse loop if there is only one token left or if we haven't parsed any tokens
			if maybe_parsed_tokens.len() == 1 || maybe_parsed_tokens_at_start == maybe_parsed_tokens.len() {
				break;
			}
		}
		// There should only be one `MaybeParsedToken`, it should be parsed into an expression
		if maybe_parsed_tokens.len() == 1 && maybe_parsed_tokens[0].is_parsed() {
			return Ok(maybe_parsed_tokens.pop().unwrap().unwrap_parsed())
		}
		println!("{maybe_parsed_tokens:?}");
		Err(Error::NotYetImplemented("Parsing some expressions".into())
			.at(Some(match maybe_parsed_tokens.first().unwrap() {
				MaybeParsedToken::Parsed(TanukiExpression { start_line, .. }) => *start_line,
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { start_line, .. }) => *start_line,
				MaybeParsedToken::Unparsed(TanukiToken { start_line, .. }) => *start_line,
			}), Some(match maybe_parsed_tokens.first().unwrap() {
				MaybeParsedToken::Parsed(TanukiExpression { start_column, .. }) => *start_column,
				MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken { start_column, .. }) => *start_column,
				MaybeParsedToken::Unparsed(TanukiToken { start_column, .. }) => *start_column,
			}), None)
		)
	}

	/// Parses an expression or returns an error if none is found.
	pub fn parse_expected(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		match Self::parse(main, token_reader)? {
			None => Err(Error::ExpectedExpression.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			Some(expression) => Ok(expression),
		}
	}
}