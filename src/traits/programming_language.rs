use std::fmt::Debug;

pub trait ProgrammingLanguage: Debug {
	fn get_extensions(&self) -> &'static [&'static str];
}