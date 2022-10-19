use std::ffi::CString;

#[derive(Debug, Default)]
pub struct StringTable {
	strings: Vec<CString>,
}

impl StringTable {
	pub fn to_llvm_string(&mut self, string: &str) -> *const i8 {
		let string = CString::new(string).unwrap();
		let pointer = string.as_ptr();
		self.strings.push(string);
		return pointer;
	}

	pub fn to_mut_llvm_string(&mut self, string: &str) -> *mut i8 {
		let string = CString::new(string).unwrap();
		let pointer = string.as_ptr() as *mut i8;
		self.strings.push(string);
		return pointer;
	}
}
