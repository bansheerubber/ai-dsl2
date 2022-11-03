pub type Bits = u32;
pub type Pointers = u8;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Type {
	CString(Pointers),
	Float(Pointers),
	Integer(Pointers, Bits),
	#[default]
	Void,
}

impl Type {
	pub fn get_pointer_number(&self) -> Pointers {
		match *self {
			Type::CString(number) => number,
			Type::Float(number) => number,
			Type::Integer(number, _) => number,
			Type::Void => 0,
		}
	}

	pub fn zero_pointer_number(&self) -> Self {
		match *self {
			Type::CString(_) => Type::CString(0),
			Type::Float(_) => Type::Float(0),
			Type::Integer(_, bits) => Type::Integer(0, bits),
			Type::Void => Type::Void,
		}
	}

	pub fn zero_bits(&self) -> Self {
		match *self {
			Type::CString(pointer_number) => Type::CString(pointer_number),
			Type::Float(pointer_number) => Type::Float(pointer_number),
			Type::Integer(pointer_number, _) => Type::Integer(pointer_number, 0),
			Type::Void => Type::Void,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MathError {
	IncompatibleTypes(Type, Type),
	UndefinedVariable(String),
	UnsupportedOperation,
}
