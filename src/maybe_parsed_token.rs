use crate::traits::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MaybeParsedToken<P, U> where U: Token {
	Unparsed(U),
	Parsed(P),
}

impl<P, U: Token> MaybeParsedToken<P, U> {
	pub fn unwrap_parsed(self) -> P {
		match self {
			Self::Parsed(parsed) => parsed,
			Self::Unparsed(..) => panic!("Unwrapped unparsed token as parsed."),
		}
	}

	pub fn unwrap_unparsed(self) -> U {
		match self {
			Self::Unparsed(unparsed) => unparsed,
			Self::Parsed(..) => panic!("Unwrapped parsed token as unparsed."),
		}
	}

	pub fn is_parsed(&self) -> bool {
		matches!(self, Self::Parsed(..))
	}
}