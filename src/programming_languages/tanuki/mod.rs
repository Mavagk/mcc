use num::{BigUint, Num};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::tanuki::{module::TanukiModule, token::{Keyword, TanukiToken, TanukiTokenVariant}}, source_file_reader::SourceFileReader, token_reader::TokenReader, traits::programming_language::ProgrammingLanguage};

pub mod module;
pub mod token;

#[derive(Debug)]
pub struct Tanuki {}

impl Tanuki {
	pub fn new() -> Self {
		Self {}
	}
}

impl ProgrammingLanguage<TanukiToken, TanukiModule> for Tanuki {
	fn tokenize_next_token(_main: &mut Main, reader: &mut SourceFileReader) -> Result<Option<TanukiToken>, ErrorAt> {
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
			'A'..='Z' | 'a'..='z' | '_' => {
				let mut name = String::new();
				while matches!(reader.peek_char()?, Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9')) {
					name.push(reader.read_char()?.unwrap());
				}
				TanukiTokenVariant::Identifier(name.into_boxed_str())
			}
			'@' => {
				reader.read_char()?;
				let mut name = String::new();
				while matches!(reader.peek_char()?, Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9')) {
					name.push(reader.read_char()?.unwrap());
				}
				let keyword = Keyword::from_name(&name)
					.ok_or_else(|| Error::InvalidKeyword(format!("@{name}")).at(Some(start_line), Some(start_column), None))?;
				TanukiTokenVariant::Keyword(keyword)
			}
			'0'..='9' | '.' => {
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
			_ => todo!()
		};
		// Assemble into token
		Ok(Some(TanukiToken {
			variant: token_variant,
			start_line,
			start_column,
			end_line: reader.get_line(),
			end_column: reader.get_column(),
		}))
	}

	fn parse_tokens(_main: &mut Main, _token_reader: TokenReader<TanukiToken>) -> Result<TanukiModule, ErrorAt> {
		todo!()
	}
}

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
				'x' => {
					match (reader.read_char()?, reader.read_char()?) {
						(Some(high_digit), Some(low_digit)) if high_digit.is_ascii_hexdigit() && low_digit.is_ascii_hexdigit()
							=> (high_digit.to_digit(16).unwrap() * 16 + low_digit.to_digit(16).unwrap()) as u8 as char,
						_ => return Err(Error::InvalidEscapeChars(format!("\\x??")).at(Some(start_line), Some(start_column), None)),
					}
				}
				'{' => {
					let mut hex_digits = String::new();
					while matches!(reader.peek_char()?, Some(chr) if chr.is_ascii_hexdigit() || chr == '_') {
						if reader.peek_char()?.unwrap() != '_' {
							hex_digits.push(reader.read_char()?.unwrap());
						}
						else {
							reader.read_char()?;
						}
					}
					if reader.peek_char()? != Some('}') {
						return Err(Error::InvalidEscapeChars("\\{?".into()).at(Some(start_line), Some(start_column), None));
					}
					reader.read_char()?;
					match u32::from_str_radix(&hex_digits, 16) {
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