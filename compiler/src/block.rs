use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Builder, Module, strings };

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Block {
	block: LLVMBasicBlockRef,
}

impl Block {
	pub fn get_block(&self) -> LLVMBasicBlockRef {
		self.block
	}

	pub fn get_parent(&self) -> LLVMValueRef {
		unsafe {
			LLVMGetBasicBlockParent(self.block)
		}
	}

	pub fn get_name(&self) -> String {
		unsafe {
			strings::from_llvm_string(LLVMGetBasicBlockName(self.block))
		}
	}
}

impl Module {
	pub fn new_block(&mut self, name: &str, value: LLVMValueRef) -> Block {
		Block {
			block: unsafe {
				LLVMAppendBasicBlock(
					value,
					self.string_table.to_llvm_string(name)
				)
			},
		}
	}

	// modifies current block, returns reference to old block
	pub fn split_block_in_place(&mut self, block: &mut Block) -> Block {
		let name = block.get_name() + "cont";
		let parent = block.get_parent();

		let old_block = block.block;
		*block = self.new_block(&name, parent);

		Block {
			block: old_block,
		}
	}

	pub fn move_block_after(&mut self, block: Block, position: Block) {
		unsafe {
			LLVMMoveBasicBlockAfter(block.get_block(), position.get_block());
		}
	}

	pub fn move_block_before(&mut self, block: Block, position: Block) {
		unsafe {
			LLVMMoveBasicBlockAfter(block.get_block(), position.get_block());
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
