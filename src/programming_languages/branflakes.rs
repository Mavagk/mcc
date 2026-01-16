use crate::traits::programming_language::ProgrammingLanguage;

#[derive(Debug)]
pub struct Branflakes;

impl Branflakes {
	pub const fn new() -> Self {
		Self
	}
}

impl ProgrammingLanguage for Branflakes {
	fn get_extensions(&self) -> &'static [&'static str] {
		&["bf"]
	}
}