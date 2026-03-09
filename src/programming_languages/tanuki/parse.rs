use std::num::NonZeroUsize;

use crate::{Main, error::{Error, ErrorAt}, maybe_parsed_token::MaybeParsedToken, programming_languages::tanuki::{compile_time_value::TanukiCompileTimeValue, expression::{TanukiExpression, TanukiExpressionVariant}, module::TanukiModule, token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiKeyword, TanukiToken, TanukiTokenVariant}}, token_reader::TokenReader};

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
	FunctionArgumentsOrParameters(Box<[TanukiExpression]>, Option<Box<TanukiExpression>>),
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
			parsed_expressions: expressions.into_boxed_slice(), functions: Vec::new(), global_constants: Vec::new(), exports: Vec::new(),//, imports: Vec::new(), links: Vec::new(),
			entrypoint: None,
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
			// First parse round
			let token = token_reader.next().unwrap().clone();
			let token_start_line = token.start_line;
			let token_start_column = token.start_column;
			let token_end_line = token.end_line;
			let token_end_column = token.end_column;
			maybe_parsed_tokens.push(match &token.variant {
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
				// If there is a block
				TanukiTokenVariant::LeftCurlyParenthesis => 'a: {
					// Parse each sub-expression
					let mut sub_expressions = Vec::new();
					loop {
						// Parse expression
						let expression_is_empty;
						if let Some(sub_expression) = Self::parse(main, token_reader)? {
							sub_expressions.push(sub_expression);
							expression_is_empty = false;
						}
						else {
							expression_is_empty = true;
						}
						// Next token should be a } or ; token
						match token_reader.next() {
							// Right curly bracket ends the block expression
							Some(TanukiToken { variant: TanukiTokenVariant::RightCurlyParenthesis, end_line, end_column, .. }) => break 'a MaybeParsedToken::Parsed(TanukiExpression {
								variant: TanukiExpressionVariant::Block { sub_expressions: sub_expressions.into(), has_return_value: !expression_is_empty },
								start_line: token_start_line, start_column: token_start_column, end_line: *end_line, end_column: *end_column,
							}),
							// The token stream should not just stop
							None => return Err(Error::ExpectedCurlyClosingParenthesis.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
							// Move on to the next sub-expression if we read a semicolon
							Some(TanukiToken { variant: TanukiTokenVariant::Semicolon, .. }) => {},
							// Else an error
							Some(TanukiToken { start_column, end_column, .. })
								=> return Err(Error::ExpectedSemicolon.at(Some(*start_column), Some(*end_column), None)),
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
				_ => MaybeParsedToken::Unparsed(token),
			});
		}
		if maybe_parsed_tokens.is_empty() {
			return Ok(None);
		}
		Ok(Some(Self::parse_maybe_parsed_tokens(main, maybe_parsed_tokens)?))
	}

	pub fn parse_maybe_parsed_tokens(main: &mut Main, mut maybe_parsed_tokens: Vec<MaybeParsedToken<TanukiExpression, TanukiPartiallyParsedToken, TanukiToken>>) -> Result<TanukiExpression, ErrorAt> {
		// Loop until we have parsed all tokens or parsing has stalled
		loop {
			let maybe_parsed_tokens_at_start = maybe_parsed_tokens.len();
			// Parse postfix operators
			let mut x = 0;
			'a: while x < maybe_parsed_tokens.len() - 1 {
				// Skip if this is not in the order (parsed expression, operator, non-parsed_expression) or (parsed expression function arguments)
				if (
						!maybe_parsed_tokens[x].is_parsed() || 
						(!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Operator { postfix_unary_operator: Some(..), is_assignment: false, .. }, .. })) ||
						matches!(maybe_parsed_tokens.get(x + 2), Some(token) if token.is_parsed()))
					) && (!maybe_parsed_tokens[x].is_parsed() || (!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..) | TanukiPartiallyParsedTokenVariant::SquareParenthesised(..), ..
					})))) && (!matches!(maybe_parsed_tokens[x], 
						MaybeParsedToken::Unparsed(TanukiToken { variant: TanukiTokenVariant::Keyword(TanukiKeyword::Import | TanukiKeyword::Link | TanukiKeyword::U | TanukiKeyword::I | TanukiKeyword::F), .. })) ||
						!matches!(maybe_parsed_tokens[x + 1], MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken
						{
							variant: TanukiPartiallyParsedTokenVariant::FunctionArgumentsOrParameters(..), ..
						})))
				{
					x += 1;
					continue;
				}
				// Parse
				if matches!(maybe_parsed_tokens[x], MaybeParsedToken::Unparsed(TanukiToken {
					variant: TanukiTokenVariant::Keyword(TanukiKeyword::Import | TanukiKeyword::Link | TanukiKeyword::U | TanukiKeyword::I | TanukiKeyword::F), ..
				})) {
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
					maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(TanukiExpression { variant: match keyword {
						TanukiKeyword::Import => {
							if arguments.len() != 1 {
								return Err(Error::Unimplemented("@import with argument count that is not one".into()).at(Some(start_line), Some(start_column), None));
							}
							let argument = &arguments[0];
							let argument = match &argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
								_ => return Err(Error::Unimplemented("@import with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
							};
							let mut module_path = main.module_being_processed.parent().unwrap().to_path_buf();
							module_path.push(argument);
							main.add_module_to_compile((module_path.clone().into_boxed_path(), false));
							TanukiExpressionVariant::ImportConstant { name: None, module_path: module_path.into_boxed_path() }
						},
						TanukiKeyword::Link => {
							if arguments.len() != 1 {
								return Err(Error::Unimplemented("@link with argument count that is not one".into()).at(Some(start_line), Some(start_column), None));
							}
							let argument = &arguments[0];
							let argument = match &argument.variant {
								TanukiExpressionVariant::Constant(TanukiCompileTimeValue::CompileTimeString(path)) => &**path,
								_ => return Err(Error::Unimplemented("@link with argument that is not a string".into()).at(Some(argument.start_line), Some(argument.start_column), None)),
							};
							let mut library_path = main.module_being_processed.parent().unwrap().to_path_buf();
							library_path.push(argument);
							main.add_module_to_compile((library_path.clone().into_boxed_path(), false));
							TanukiExpressionVariant::Link { name: None, library_path: library_path.into_boxed_path() }
						},
						TanukiKeyword::U => TanukiExpressionVariant::U(arguments),
						TanukiKeyword::I => TanukiExpressionVariant::I(arguments),
						TanukiKeyword::F => TanukiExpressionVariant::F(arguments),
						_ => unreachable!(),
					}, start_line, start_column, end_column, end_line });
					continue 'a;
				}
				let operand = maybe_parsed_tokens[x].clone().unwrap_parsed();
				maybe_parsed_tokens[x] = MaybeParsedToken::Parsed(match maybe_parsed_tokens.remove(x + 1) {
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Operator { postfix_unary_operator, symbol, .. }, start_line, start_column, end_line, end_column
					}) => TanukiExpression { start_line: operand.start_line, start_column: operand.start_column, variant: match postfix_unary_operator {
						Some(operator) => TanukiExpressionVariant::PostfixUnaryOperator(operator, Box::new(operand)),
						None => return Err(Error::InvalidPostfixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
					}, end_line, end_column },
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: _, ..
					}) => unreachable!(),
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
					MaybeParsedToken::PartiallyParsed(TanukiPartiallyParsedToken {
						variant: TanukiPartiallyParsedTokenVariant::SquareParenthesised(index), end_line, end_column, ..
					}) => TanukiExpression {
						start_line: operand.start_line, start_column: operand.start_column, variant: TanukiExpressionVariant::Index(Box::new(operand), index),
						end_line, end_column,
					},
					MaybeParsedToken::PartiallyParsed(..) => unreachable!(),
					MaybeParsedToken::Parsed(..) => unreachable!(),
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
				) && (x >= maybe_parsed_tokens.len() - 1 || (!maybe_parsed_tokens[x].is_parsed() || !maybe_parsed_tokens[x + 1].is_parsed())) && (
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
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Operator { prefix_unary_operator, symbol, .. }, start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, variant: match prefix_unary_operator {
						Some(operator) => TanukiExpressionVariant::PrefixUnaryOperator(operator, Box::new(operand)),
						None => return Err(Error::InvalidPrefixUnaryOperator(symbol.into_string()).at(Some(start_line), Some(start_column), None)),
					}, start_line, start_column },
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Keyword(TanukiKeyword::Export), start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, start_line, start_column, variant: TanukiExpressionVariant::Export(Box::new(operand))},
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: TanukiTokenVariant::Keyword(TanukiKeyword::Entrypoint), start_line, start_column, ..
					}) => TanukiExpression { end_line: operand.end_line, end_column: operand.end_column, start_line, start_column, variant: TanukiExpressionVariant::Entrypoint(Box::new(operand))},
					MaybeParsedToken::Unparsed(TanukiToken {
						variant: _, ..
					}) => unreachable!(),
					MaybeParsedToken::PartiallyParsed(..) => unreachable!(),
					MaybeParsedToken::Parsed(type_expression) => TanukiExpression {
						end_line: operand.end_line, end_column: operand.end_column, start_line: operand.start_line, start_column: operand.start_column,
						variant: TanukiExpressionVariant::TypeAndValue(Box::new(type_expression), Box::new(operand)),
					},
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

	pub fn parse_expected(main: &mut Main, token_reader: &mut TokenReader<TanukiToken>) -> Result<Self, ErrorAt> {
		match Self::parse(main, token_reader)? {
			None => Err(Error::ExpectedExpression.at(Some(token_reader.last_token_end_line()), Some(token_reader.last_token_end_column()), None)),
			Some(expression) => Ok(expression),
		}
	}
}