pub type Bits = u32;
pub type Pointers = u8;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Type {
	CString(Pointers),
	Float(Pointers),
	FloatArray(usize),
	Integer(Pointers, Bits),
	#[default]
	Void,
}

impl Type {
	pub fn increment_pointer_number(self) -> Self {
		match self {
			Type::CString(p) => Type::CString(p + 1),
			Type::Float(p) => Type::Float(p + 1),
			Type::FloatArray(size) => Type::FloatArray(size),
			Type::Integer(p, bits) => Type::Integer(p + 1, bits),
			Type::Void => Type::Void,
		}
	}

	pub fn get_pointer_number(self) -> Pointers {
		match self {
			Type::CString(number) => number,
			Type::Float(number) => number,
			Type::FloatArray(_) => 1,
			Type::Integer(number, _) => number,
			Type::Void => 0,
		}
	}

	pub fn zero_pointer_number(self) -> Self {
		match self {
			Type::CString(_) => Type::CString(0),
			Type::Float(_) => Type::Float(0),
			Type::FloatArray(size) => Type::FloatArray(size),
			Type::Integer(_, bits) => Type::Integer(0, bits),
			Type::Void => Type::Void,
		}
	}

	pub fn zero_bits(self) -> Self {
		match self {
			Type::CString(pointer_number) => Type::CString(pointer_number),
			Type::Float(pointer_number) => Type::Float(pointer_number),
			Type::FloatArray(size) => Type::FloatArray(size),
			Type::Integer(pointer_number, _) => Type::Integer(pointer_number, 0),
			Type::Void => Type::Void,
		}
	}

	pub fn to_array(self, size: usize) -> Self {
		match self {
			Type::Float(_) => Type::FloatArray(size),
			_ => todo!(),
		}
	}

	pub fn to_scalar(self) -> Self {
		match self {
			Type::FloatArray(_) => Type::Float(0),
			_ => todo!(),
		}
	}

	/// Whether or not this type can be converted to another type.
	pub fn is_compatible(&self, other: &Type) -> bool {
		if self == other {
			return true;
		}

		match *self {
			Type::Float(_) => match *other {
				Type::Float(_) => {
					return true;
				},
				_ => todo!("{:?} {:?}", self, other),
			},
			Type::FloatArray(_) => match *other {
				Type::Float(pointer) => {
					pointer == 1
				},
				_ => todo!("{:?} {:?}", self, other),
			},
			Type::Integer(_, _) => match *other {
				Type::Integer(_, _) => {
					self.zero_pointer_number() == other.zero_pointer_number()
				},
				_ => todo!("{:?} {:?}", self, other),
			},
			_ => todo!("{:?} {:?}", self, other),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MathError {
	IncompatibleTypes(Type, Type),
	UndefinedVariable(String),
	UnsupportedOperation,
}
