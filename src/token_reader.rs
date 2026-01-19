use std::num::NonZeroUsize;

use crate::traits::token::Token;

#[derive(Clone)]
pub struct TokenReader<'a, T: Token> {
	tokens: &'a [T],
	last_taken_token_end_line: NonZeroUsize,
	last_taken_token_end_column: NonZeroUsize,
}

impl<'a, T: Token> TokenReader<'a, T> {
	pub fn new(tokens: &'a [T]) -> Self {
		Self {
			tokens,
			last_taken_token_end_line: NonZeroUsize::new(1).unwrap(),
			last_taken_token_end_column: NonZeroUsize::new(1).unwrap(),
		}
	}
	
	pub fn peek(&self) -> Option<&T> {
		self.tokens.get(0)
	}

	pub fn next(&mut self) -> Option<&T> {
		let out = self.tokens.get(0)?;
		self.last_taken_token_end_line = out.get_end_line();
		self.last_taken_token_end_column = out.get_end_line();
		self.tokens = &self.tokens[1..];
		Some(out)
	}

	pub fn last_token_end_line(&self) -> NonZeroUsize {
		self.last_taken_token_end_line
	}

	pub fn last_token_end_column(&self) -> NonZeroUsize {
		self.last_taken_token_end_column
	}
}