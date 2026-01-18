use crate::Main;

pub trait VirtualMachine {
	fn new(main: &mut Main) -> Self;
}