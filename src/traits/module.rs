use std::fmt::Display;

use crate::traits::{module_element::ModuleElement, programming_language::ProgrammingLanguage};

pub trait Module: Display {
	type ModuleElementType: ModuleElement;
	type ProgrammingLanguageType: ProgrammingLanguage;

	fn get_elements<'a>(&'a self) -> &'a [Self::ModuleElementType];
	fn get_programming_language<'a>(&'a self) -> &'a Self::ProgrammingLanguageType;
}