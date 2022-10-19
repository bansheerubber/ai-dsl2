use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, Module, Type };

#[derive(Clone, Copy, Debug)]
pub struct Value {
	pub value: LLVMValueRef,
}

impl Module {
	pub fn add_addition(&mut self, block: Block, lhs: Value, rhs: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			Value {
				value: LLVMBuildAdd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("addtmp")
				),
			}
		}
	}

	pub fn create_immediate_int(&self, number: u64) -> Value {
		unsafe {
			Value {
				value: LLVMConstInt(self.to_llvm_type(Type::Integer), number, 0),
			}
		}
	}
}
