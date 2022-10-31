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

pub fn from_llvm_string(string: *const i8) -> String {
	unsafe {
		let mut output = String::new();
		let mut index = 0;
		while *string.offset(index) != 0 {
			output.push((*string.offset(index) as u8) as char);
			index += 1;
		}
		return output;
	}
}
