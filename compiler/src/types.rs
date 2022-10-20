#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Type {
	CString,
	Float,
	Integer,
	IntegerPointer,
	#[default]
	Void,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MathError {
	IncompatibleTypes,
	UnsupportedOperation,
}
