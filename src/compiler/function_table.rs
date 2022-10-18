use llvm_sys::prelude::*;
use std::{ collections::HashMap, rc::Rc, cell::RefCell };

use crate::compiler::{ Block, Module };

#[derive(Debug)]
pub struct Function {
	block: Option<Block>,
	pub function: LLVMValueRef,
	module: Rc<RefCell<Module>>,
	pub name: String,
	pub return_type: LLVMTypeRef,
}

impl Function {
	fn create_block(&mut self) {
		if let None = self.block {
			self.block = Some(Block::new(self.module.clone(), &self.name, self.function));
		}
	}

	pub fn get_block(&mut self) -> &Block {
		self.create_block();
		&self.block.as_ref().unwrap()
	}

	pub fn get_block_mut(&mut self) -> &mut Block {
		self.create_block();
		self.block.as_mut().unwrap()
	}
}

#[derive(Debug, Default)]
pub struct FunctionTable {
	functions: HashMap<String, Rc<RefCell<Function>>>,
}

impl FunctionTable {
	pub fn add_function(&mut self, module: Rc<RefCell<Module>>, name: &str, function: LLVMValueRef, return_type: LLVMTypeRef) {
		self.functions.insert(String::from(name), Rc::new(RefCell::new(Function {
			block: None,
			function,
			module,
			name: String::from(name),
			return_type,
		})));
	}

	pub fn get_function(&self, name: &str) -> Option<Rc<RefCell<Function>>> {
		if let Some(function) = self.functions.get(&String::from(name)) {
			Some(function.clone())
		} else {
			return None;
		}
	}
}
