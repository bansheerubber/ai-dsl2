use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::{ rc::Rc, cell::RefCell };

use crate::compiler::Module;

#[derive(Debug)]
pub struct Block {
	block: LLVMBasicBlockRef,
	module: Rc<RefCell<Module>>,
}

impl Block {
	pub fn new(module: Rc<RefCell<Module>>, name: &str, value: LLVMValueRef) -> Self {
		let block;
		unsafe {
			let mut module_mut = module.borrow_mut();
			block = LLVMAppendBasicBlockInContext(
				module_mut.get_context(),
				value,
				module_mut.string_table.to_llvm_string(name)
			);
			LLVMPositionBuilderAtEnd(module_mut.get_builder(), block);
		}

		Block {
			block,
			module,
		}
	}

	pub fn add_function_call(&self, name: &str, args: &mut [LLVMValueRef]) {
		let mut module = self.module.borrow_mut();
		let function = module.function_table.get_function(name).unwrap();
		unsafe {
			LLVMPositionBuilderAtEnd(module.get_builder(), self.block);

			LLVMBuildCall2(
				module.get_builder(),
				function.borrow().return_type,
				function.borrow().function,
				args.as_mut_ptr(),
				args.len() as u32,
				module.string_table.to_llvm_string("") // TODO what is this for?
			);
		}
	}
}
