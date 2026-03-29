use std::{collections::HashSet, fmt::{self, Debug, Formatter}, num::NonZeroUsize, path::Path};

use crate::{Main, error::{Error, ErrorAt}, programming_languages::{c::module::CModule, tanuki::{expression::TanukiExpression, function::TanukiFunction, global_constant::TanukiGlobalConstant, t_type::TanukiType}}, traits::{ast_node::AstNode, module::Module}};

/// A Tanuki module that has been parsed from a single file.
pub struct TanukiModule {
	pub parsed_expressions: Box<[TanukiExpression]>,
	pub functions: Vec<Option<TanukiFunction>>,
	pub global_constants: Vec<Option<TanukiGlobalConstant>>,
	/// A list of types used that can exist at runtime.
	pub run_time_types_used: HashSet<TanukiType>,
	pub run_time_types_used_list: Vec<TanukiType>,
	pub entrypoint: Option<Box<str>>,
	pub mangled_module_names_to_include_in_c: HashSet<Box<str>>,
}

impl TanukiModule {
	pub fn add_run_time_type(&mut self, t_type: &TanukiType) {
		if self.run_time_types_used.insert(t_type.clone()) {
			match t_type {
				TanukiType::ConcreteFunctionPointer(types) => {
					for parameter_type in &types.parameter_types {
						self.add_run_time_type(parameter_type);
					}
					self.add_run_time_type(&types.return_type);
				}
				TanukiType::Struct { ordered_members, named_members } => {
					for parameter_type in ordered_members {
						self.add_run_time_type(parameter_type);
					}
					for parameter_type in named_members {
						self.add_run_time_type(parameter_type.1);
					}
				}
				TanukiType::Pointer(pointee_type) => {
					self.add_run_time_type(pointee_type);
				}
				TanukiType::TypeEnum(types) => {
					for t_type in types {
						self.add_run_time_type(t_type);
					}
				}
				TanukiType::Bool | TanukiType::U(_) | TanukiType::I(_) | TanukiType::F(_) | TanukiType::Void => {},
				TanukiType::CompileTimeChar | TanukiType::CompileTimeFloat | TanukiType::CompileTimeInt | TanukiType::CompileTimeString | TanukiType::Any | TanukiType::Type |
				TanukiType::FunctionPointer(_) | TanukiType::FunctionPointerEnum(_) => unreachable!(),
			}
			self.run_time_types_used_list.push(t_type.clone());
		}
	}
}

impl Module for TanukiModule {
	fn interpreted_execute_entrypoint(&self, _main: &mut Main) -> Result<(), ErrorAt> {
		Err(Error::Unimplemented("Interpreting Tanuki code".into()).at(None, None, None))
	}

	fn to_c_module(&self, main: &mut Main, modules: &[(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], _is_entrypoint: bool) -> Result<Option<CModule>, ErrorAt> {
		Ok(Some(self.compile_to_c_module(main, modules)?))
	}

	fn const_compile(&mut self, main: &mut Main, modules: &mut [(Box<Path>, bool, Option<Box<dyn Module>>, Box<str>)], module_path: &Path, was_complication_done: &mut bool) -> Result<(), ErrorAt> {
		self.const_compile_globals(main, modules, module_path, was_complication_done)
	}
}

impl AstNode for TanukiModule {
	fn print_name(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tanuki Module")?;
		if let Some(entrypoint) = &self.entrypoint {
			write!(f, ", Entrypoint: {entrypoint}")?;
		}
		Ok(())
	}

	fn print_sub_nodes(&self, level: usize, f: &mut Formatter<'_>) -> fmt::Result {
		for run_time_type_used in &self.run_time_types_used_list {
			run_time_type_used.print(level, f)?;
		}
		for expression in &self.parsed_expressions {
			expression.print(level, f)?;
		}
		for function in &self.functions {
			function.as_ref().unwrap().print(level, f)?;
		}
		for global_constant in &self.global_constants {
			global_constant.as_ref().unwrap().print(level, f)?;
		}
		Ok(())
	}

	fn start_line(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.first().map(|expression| expression.start_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn start_column(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.first().map(|expression| expression.start_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_line(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.last().map(|expression| expression.end_line).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}

	fn end_column(&self) -> Option<NonZeroUsize> {
		Some(self.parsed_expressions.last().map(|expression| expression.end_column).unwrap_or(NonZeroUsize::new(1).unwrap()))
	}
}

impl Debug for TanukiModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.print(0, f)
	}
}