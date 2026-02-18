use crate::{Main, error::ErrorAt, programming_languages::tanuki::module::TanukiModule};

impl TanukiModule {
	pub fn post_parse(&mut self, _main: &mut Main) -> Result<(), ErrorAt> {
		Ok(())
	}
}