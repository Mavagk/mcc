use std::{fmt::{self, Debug, Formatter}, fs::File, io::{BufWriter, Write}, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::c::module_element::CModuleElement, traits::{ast_node::AstNode, module::Module}};

pub struct CModule {
	elements: Vec<CModuleElement>,
}

impl CModule {
	pub fn new() -> Self {
		Self {
			elements: Vec::new(),
		}
	}

	pub fn push_element(&mut self, element: CModuleElement) {
		self.elements.push(element);
	}
}

impl Module for CModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		unimplemented!()
	}

	fn to_c_module(&self, _main: &mut Main, _modules: &[(Box<Path>, bool, Option<Box<dyn Module>>)],  _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		unimplemented!()
	}
}

impl AstNode for CModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "C Module")
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for sub_element in self.elements.iter() {
			sub_element.print(level, f)?;
		}
		Ok(())
	}

	fn write_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		let mut is_first_element = true;
		for module_element in self.elements.iter() {
			if !is_first_element && !matches!(module_element, CModuleElement::FunctionDeclaration { .. } | CModuleElement::AngleInclude(..) | CModuleElement::DoubleQuotesInclude(..)) {
				writer.write_all(b"\n\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
			}
			module_element.write_to_file(writer, indentation_level)?;
			if !matches!(module_element, CModuleElement::FunctionDeclaration { .. } | CModuleElement::AngleInclude(..) | CModuleElement::DoubleQuotesInclude(..)) {
				is_first_element = false;
			}
		}
		Ok(())
	}

	fn write_header_to_file(&self, writer: &mut BufWriter<File>, indentation_level: usize) -> Result<(), ErrorAt> {
		let mut is_first_element = true;
		for module_element in self.elements.iter() {
			if !is_first_element {
				writer.write_all(b"\n").map_err(|err| Error::UnableToWriteToFile(err.to_string()).at(None, None, None))?;
			}
			module_element.write_header_to_file(writer, indentation_level)?;
			is_first_element = false;
		}
		Ok(())
	}
}

impl Debug for CModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}