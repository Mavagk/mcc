use crate::traits::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MaybeParsedToken<P, Q, U> where U: Token {
	Unparsed(U),
	PartiallyParsed(Q),
	Parsed(P),
}

impl<P, Q, U: Token> MaybeParsedToken<P, Q, U> {
	pub fn unwrap_parsed(self) -> P {
		match self {
			Self::Parsed(parsed) => parsed,
			Self::PartiallyParsed(..) => panic!("Unwrapped partially parsed token as parsed."),
			Self::Unparsed(..) => panic!("Unwrapped unparsed token as parsed."),
		}
	}

	pub fn unwrap_unparsed(self) -> U {
		match self {
			Self::Unparsed(unparsed) => unparsed,
			Self::PartiallyParsed(..) => panic!("Unwrapped partially parsed token as unparsed."),
			Self::Parsed(..) => panic!("Unwrapped parsed token as unparsed."),
		}
	}

	pub fn is_parsed(&self) -> bool {
		matches!(self, Self::Parsed(..))
	}

	pub fn is_unparsed(&self) -> bool {
		matches!(self, Self::Unparsed(..))
	}
}