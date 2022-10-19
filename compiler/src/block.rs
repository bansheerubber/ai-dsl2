use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Builder, Module };

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Block {
	block: LLVMBasicBlockRef,
}

impl Block {
	pub fn get_block(&self) -> LLVMBasicBlockRef {
		self.block
	}
}

impl Module {
	pub fn new_block(&mut self, name: &str, value: LLVMValueRef) -> Block {
		let block;
		unsafe {
			block = LLVMAppendBasicBlock(
				value,
				self.string_table.to_llvm_string(name)
			);
		}

		Block {
			block,
		}
	}

	pub fn add_function_call(&mut self, name: &str, args: &mut [LLVMValueRef]) {
		let function = self.function_table.get_function(name).unwrap();
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(function.block);

			LLVMBuildCall2(
				builder.get_builder(),
				function.return_type,
				function.function,
				args.as_mut_ptr(),
				args.len() as u32,
				self.string_table.to_llvm_string("") // TODO what is this for?
			);
		}
	}
}
