use num::BigInt;

#[derive(Debug, Clone)]
pub enum TanukiConstantValue {
	Integer(BigInt),
	Float(f64),
	Character(char),
	String(Box<str>),
}