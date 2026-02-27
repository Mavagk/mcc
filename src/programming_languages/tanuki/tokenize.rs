use num::{BigUint, Num};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::token::{TanukiInfixBinaryOperator, TanukiInfixTernaryOperator, TanukiKeyword, TanukiNullaryOperator, TanukiPostfixUnaryOperator, TanukiPrefixUnaryOperator, TanukiToken, TanukiTokenVariant}, source_file_reader::SourceFileReader};

pub fn tokenize_token(_main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<TanukiToken>, ErrorAt> {
	'r: loop {
		// Strip leading whitespaces
		reader.skip_leading_ascii_whitespaces()?;
		// Peek first char
		let first_char = match reader.peek_char()? {
			Some(first_char) => first_char,
			None => return Ok(None),
		};
		let start_line = reader.get_line();
		let start_column = reader.get_column();
		// Match depending on first char
		let token_variant = match first_char {
			// For separator tokens, the token is one char long
			'(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' => {
				reader.read_char()?;
				match first_char {
					'(' => TanukiTokenVariant::LeftParenthesis,
					')' => TanukiTokenVariant::RightParenthesis,
					'{' => TanukiTokenVariant::LeftCurlyParenthesis,
					'}' => TanukiTokenVariant::RightCurlyParenthesis,
					'[' => TanukiTokenVariant::LeftSquareParenthesis,
					']' => TanukiTokenVariant::RightSquareParenthesis,
					',' => TanukiTokenVariant::Comma,
					';' => TanukiTokenVariant::Semicolon,
					_ => unreachable!(),
				}
			}
			// For identifiers, the token consists of letters, digits and underscores but does not start with a digit
			'A'..='Z' | 'a'..='z' | '_' => TanukiTokenVariant::Identifier(reader.read_string_while(|chr| matches!(chr, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))?.into_boxed_str()),
			// For keywords, the token starts with a '@' char, the rest of the token consists of letters, digits and underscores
			'@' => {
				reader.read_char()?;
				let name = reader.read_string_while(|chr| matches!(chr, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))?;
				let keyword = TanukiKeyword::from_name(&name)
					.ok_or_else(|| Error::InvalidKeyword(format!("@{name}")).at(Some(start_line), Some(start_column), None))?;
				TanukiTokenVariant::Keyword(keyword)
			}
			// For numeric literals, the token starts with a digit, the rest of the token consists of letters, digits, underscores and decimal points
			'0'..='9' => {
				// Read base
				let mut base = 10;
				if first_char == '0' {
					reader.read_char()?;
					if matches!(reader.peek_char()?, Some(chr) if chr.is_alphabetic()) {
						base = match reader.read_char()?.unwrap() {
							'b' => 2,
							'o' => 8,
							'x' => 16,
							other => return Err(Error::InvalidBaseSpecifier(format!("0{other}")).at(Some(start_line), Some(start_column), None)),
						};
					}
				}
				// Read numbers chars
				let mut numeric_literal_without_base: String = "0".into();
				let mut last_was_e = false;
				loop {
					let chr = reader.peek_char()?;
					if !(matches!(chr, Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9' | '.')) || matches!(chr, Some('+' | '-') if last_was_e && base == 10)) {
						break;
					}
					let chr = chr.unwrap();
					if chr == '_' {
						continue;
					}
					last_was_e = matches!(chr, 'e' | 'E');
					numeric_literal_without_base.push(chr);
					reader.read_char()?;
				}
				// Parse
				let as_integer = BigUint::from_str_radix(&numeric_literal_without_base, base).ok();
				let as_float = f64::from_str_radix(&numeric_literal_without_base, base).ok();
				if as_integer.is_none() && as_float.is_none() {
					return Err(Error::InvalidNumericLiteral(numeric_literal_without_base.into()).at(Some(start_line), Some(start_column), None));
				}
				// Assemble into token variant
				TanukiTokenVariant::NumericLiteral(as_integer, as_float)
			}
			// For a string literals, the token is enclosed by double quotes and consists of non double quote or backslash chars and escape sequences
			'"' => {
				// Take opening quote
				reader.read_char()?;
				// Read escaped/unescaped chars
				let mut string = String::new();
				while let Some(chr) = parse_literal_char(reader, false)? {
					string.push(chr);
				}
				// Expect a closing quote
				let expected_quote_line = reader.get_line();
				let expected_quote_column = reader.get_column();
				if reader.read_char()? != Some('"') {
					return Err(Error::ExpectedClosingQuote.at(Some(expected_quote_line), Some(expected_quote_column), None));
				}
				// Assemble into token variant
				TanukiTokenVariant::StringLiteral(string.into_boxed_str())
			}
			// For char literals, the token is enclosed by a single quote and consists of a single non backslash char or escape sequence
			// For block labels, the token starts with a single quote, the rest of the token consists of letters, digits and underscores
			'\'' => 'a: {
				// Take opening quote
				reader.read_char()?;
				// Read/parse first char
				let first_source_char = match reader.peek_char()? {
					Some(first_source_char) => first_source_char,
					None => return Err(Error::ExpectedClosingQuote.at(Some(reader.get_line()), Some(reader.get_column()), None)),
				};
				let first_parsed_char = parse_literal_char(reader, true)?.unwrap();
				let second_char = reader.peek_char()?;
				// If this is a block label, parse a block label
				if matches!(first_source_char, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9') && second_char != Some('\'') {
					let mut label_name = String::new();
					label_name.push(first_source_char);
					while matches!(reader.peek_char()?, Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9')) {
						label_name.push(reader.read_char()?.unwrap());
					}
					break 'a TanukiTokenVariant::BlockLabel(label_name.into_boxed_str());
				}
				// Else parse a char literal and expect a closing quote
				if second_char != Some('\'') {
					return Err(Error::ExpectedClosingQuote.at(Some(reader.get_line()), Some(reader.get_column()), None));
				}
				reader.read_char()?;
				TanukiTokenVariant::CharacterLiteral(first_parsed_char)
			}
			// For operators, the token consists of operator chars
			// For comments, the comment starts with two slashes and runs until the next newline
			'+' | '-' | '*' | '/' | '%' | '!' | '|' | '&' | '^' | '<' | '=' | '>' | ':' | '?' | '.' => {
					// Get the operator symbol
					let mut name = String::new();
					while let Some(chr) = reader.peek_char()? {
						if !matches!(chr, '+' | '-' | '*' | '/' | '%' | '!' | '|' | '&' | '^' | '<' | '=' | '>' | ':' | '?' | '.') {
							break;
						}
						name.push(chr);
						reader.read_char()?;
						// If we read two slashes, skip to the next newline
						if name == "//" {
							while matches!(reader.peek_char()?, Some(chr) if chr != '\n') {
								reader.read_char()?;
							}
							continue 'r;
						}
					}
					// Get if this is a colon
					let is_colon = name == ":";
					// Get if this is an assignment operator
					let is_assignment = name.ends_with('=') && !matches!(name.as_str(), "==" | "!=" | "===" | "!==" | "<=" | ">=" | "..=");
					let name_without_assignment: &str = match is_assignment {
						true => name.as_str().strip_suffix('=').unwrap_or(name.as_str()),
						false => name.as_str(),
					};
					// Parse
					let prefix_unary_operator = TanukiPrefixUnaryOperator::from_source(name_without_assignment);
					let infix_binary_operator = TanukiInfixBinaryOperator::from_source(name_without_assignment);
					let postfix_unary_operator = TanukiPostfixUnaryOperator::from_source(name_without_assignment);
					let infix_ternary_operator = TanukiInfixTernaryOperator::from_source(name_without_assignment);
					let nullary_operator = TanukiNullaryOperator::from_source(name_without_assignment);
					if prefix_unary_operator.is_none() && infix_binary_operator.is_none() && postfix_unary_operator.is_none() && infix_ternary_operator.is_none() && nullary_operator.is_none() && !is_colon {
						return Err(Error::InvalidOperatorSymbol(name).at(Some(start_line), Some(start_column), None));
					}
					TanukiTokenVariant::Operator { prefix_unary_operator, infix_binary_operator, postfix_unary_operator, infix_ternary_operator, nullary_operator, is_assignment, is_colon, symbol: name.into_boxed_str() }
				}
			// TODO: Comments
			other => return Err(Error::InvalidCharStartingToken(other).at(Some(start_line), Some(start_column), None)),
		};
		// Assemble into token
		return Ok(Some(TanukiToken {
			variant: token_variant,
			start_line,
			start_column,
			end_line: reader.get_line(),
			end_column: reader.get_column(),
		}))
	}
}

/// Reads and parses a single escaped or unescaped char form the reader.
/// Returns `Ok(None)` if a double quote is encountered and `is_char_literal` is false.
fn parse_literal_char(reader: &mut SourceFileReader, is_char_literal: bool) -> Result<Option<char>, ErrorAt> {
	// TODO: Better errors
	let start_line = reader.get_line();
	let start_column = reader.get_column();
	let first_char = match reader.peek_char()? {
		None => return Ok(None),
		Some(first_char) => first_char,
	};
	if first_char == '"' && !is_char_literal {
		return Ok(None);
	}
	match first_char {
		'\\' => {
			reader.read_char()?;
			let char_after_backslash = match reader.read_char()? {
				Some(char_after_backslash) => char_after_backslash,
				None => return Err(Error::InvalidEscapeChars(format!("\\")).at(Some(start_line), Some(start_column), None)),
			};
			Ok(Some(match char_after_backslash {
				'\'' => '\'',
				'\"' => '\"',
				'\\' => '\\',
				'n' => '\n',
				'r' => '\r',
				't' => '\t',
				'0' => '\0',
				'a' => '\x07',
				'b' => '\x08',
				'f' => '\x0C',
				'v' => '\x0B',
				'x' => {
					match (reader.read_char()?, reader.read_char()?) {
						(Some(high_digit), Some(low_digit)) if high_digit.is_ascii_hexdigit() && low_digit.is_ascii_hexdigit()
							=> (high_digit.to_digit(16).unwrap() * 16 + low_digit.to_digit(16).unwrap()) as u8 as char,
						_ => return Err(Error::InvalidEscapeChars(format!("\\x??")).at(Some(start_line), Some(start_column), None)),
					}
				}
				'o' => {
					expect_opening_curly_parenthesis(reader)?;
					let digits = reader.read_string_while_and_skip(|chr| matches!(chr, '0'..='7'), |chr| chr == '_')?;
					expect_closing_curly_parenthesis(reader)?;
					match u32::from_str_radix(&digits, 8) {
						Ok(escaped_char_value) => match char::from_u32(escaped_char_value) {
							Some(escaped_char_value) => escaped_char_value,
							None => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
						}
						Err(_) => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
					}
				}
				'd' => {
					expect_opening_curly_parenthesis(reader)?;
					let digits = reader.read_string_while_and_skip(|chr| chr.is_ascii_digit(), |chr| chr == '_')?;
					expect_closing_curly_parenthesis(reader)?;
					match u32::from_str_radix(&digits, 10) {
						Ok(escaped_char_value) => match char::from_u32(escaped_char_value) {
							Some(escaped_char_value) => escaped_char_value,
							None => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
						}
						Err(_) => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
					}
				}
				'{' => {
					let digits = reader.read_string_while_and_skip(|chr| chr.is_ascii_hexdigit(), |chr| chr == '_')?;
					expect_closing_curly_parenthesis(reader)?;
					match u32::from_str_radix(&digits, 16) {
						Ok(escaped_char_value) => match char::from_u32(escaped_char_value) {
							Some(escaped_char_value) => escaped_char_value,
							None => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
						}
						Err(_) => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
					}
				}
				'u' => {
					let mut hex_digits = String::new();
					for _ in 0..4 {
						hex_digits.push(match reader.read_char()? {
							Some(chr) if chr.is_ascii_hexdigit() =>chr,
							_ => return Err(Error::InvalidEscapeChars(format!("\\u????")).at(Some(start_line), Some(start_column), None)),
						});
					}
					match u32::from_str_radix(&hex_digits, 16) {
						Ok(escaped_char_value) => match char::from_u32(escaped_char_value) {
							Some(escaped_char_value) => escaped_char_value,
							None => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
						}
						Err(_) => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
					}
				}
				'U' => {
					let mut hex_digits = String::new();
					for _ in 0..6 {
						hex_digits.push(match reader.read_char()? {
							Some(chr) if chr.is_ascii_hexdigit() =>chr,
							_ => return Err(Error::InvalidEscapeChars(format!("\\U??????")).at(Some(start_line), Some(start_column), None)),
						});
					}
					match u32::from_str_radix(&hex_digits, 16) {
						Ok(escaped_char_value) => match char::from_u32(escaped_char_value) {
							Some(escaped_char_value) => escaped_char_value,
							None => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
						}
						Err(_) => return Err(Error::InvalidUnicodeCodePoint.at(Some(start_line), Some(start_column), None)),
					}
				}
				other => return Err(Error::InvalidEscapeChars(format!("\\{other}")).at(Some(start_line), Some(start_column), None)),
			}))
		},
		_ => reader.read_char(),
	}
}

/// Reads a single `{` char, returns an error if it is not said char.
pub fn expect_opening_curly_parenthesis(reader: &mut SourceFileReader) -> Result<(), ErrorAt> {
	match reader.read_and_expect_char('{')? {
		true => Ok(()),
		false => Err(Error::ExpectedCurlyOpeningParenthesis.at(Some(reader.get_line()), Some(reader.get_column()), None)),
	}
}

/// Reads a single `}` char, returns an error if it is not said char.
pub fn expect_closing_curly_parenthesis(reader: &mut SourceFileReader) -> Result<(), ErrorAt> {
	match reader.read_and_expect_char('}')? {
		true => Ok(()),
		false => Err(Error::ExpectedCurlyOpeningParenthesis.at(Some(reader.get_line()), Some(reader.get_column()), None)),
	}
}