#[derive(Clone, Copy, Debug, Default)]
pub enum Type {
	CString,
	Float,
	Integer,
	#[default]
	Void,
}
