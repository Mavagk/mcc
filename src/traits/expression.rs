use crate::traits::{ast_node::AstNode, types::Type};

pub trait Expression: AstNode {
	type TypeType: Type;

	fn get_result_type<'a>(&'a self) -> &'a Self::TypeType;
}