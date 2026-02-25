use crate::{Main, error::ErrorAt, programming_languages::{c::module::CModule, tanuki::module::TanukiModule}};

impl TanukiModule {
	pub fn compile_to_c_module(&self, _main: &mut Main, _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		todo!()
	}
}