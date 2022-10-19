use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::Module;

#[derive(Debug)]
pub struct Block {
	block: LLVMBasicBlockRef,
}

impl Module {
	pub fn new_block(&mut self, name: &str, value: LLVMValueRef) -> Block {
		let block;
		unsafe {
			block = LLVMAppendBasicBlockInContext(
				self.get_context(),
				value,
				self.string_table.to_llvm_string(name)
			);
			LLVMPositionBuilderAtEnd(self.get_builder(), block);
		}

		Block {
			block,
		}
	}

	pub fn seek_to_block(&self, block: &Block) {
		unsafe {
			LLVMPositionBuilderAtEnd(self.get_builder(), block.block);
		}
	}

	pub fn add_function_call(&mut self, name: &str, args: &mut [LLVMValueRef]) {
		let function = self.function_table.get_function(name).unwrap();
		unsafe {
			LLVMBuildCall2(
				self.get_builder(),
				function.return_type,
				function.function,
				args.as_mut_ptr(),
				args.len() as u32,
				self.string_table.to_llvm_string("") // TODO what is this for?
			);
		}
	}
}
