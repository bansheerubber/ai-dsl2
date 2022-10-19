use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::Block;

pub struct Builder {
	builder: LLVMBuilderRef,
}

impl Builder {
	pub fn new() -> Self {
		unsafe {
			Builder {
				builder: LLVMCreateBuilder(),
			}
		}
	}

	pub fn seek_to_end(&self, block: Block) {
		unsafe {
			LLVMPositionBuilderAtEnd(self.builder, block.get_block());
		}
	}

	pub fn get_builder(&self) -> LLVMBuilderRef {
		self.builder
	}
}

impl Drop for Builder {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeBuilder(self.builder);
		}
	}
}
