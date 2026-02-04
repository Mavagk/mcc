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
				let mut base = 10u8;
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
					reader.read_char()?;
				}
				// Read numbers chars
				let mut numeric_literal_without_base = String::new();
				while matches!(reader.peek_char()?, Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9' | '.')) {
					numeric_literal_without_base.push(reader.read_char()?.unwrap());
				}
				// Parse
				todo!()
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