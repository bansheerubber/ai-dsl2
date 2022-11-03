use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;

use crate::{ Block, Module, TerminalInstruction, Type };

#[derive(Clone, Debug)]
pub struct Function {
	pub blocks: HashMap<LLVMBasicBlockRef, Block>,

	// blocks are not self-terminating. what this means is that the code_generator may compile a block, add all sorts of
	// instructions to it, but not add a terminating instruction in the context the block was created in. whenever the
	// next block in the same function is created by the code_generator, that new block has control over how the original
	// block terminates.
	//
	// for compilation safety purposes, all blocks are created with a "return void" terminating instruction. this is only
	// to assist with debugging, since the disassembler formats block locations incorrectly if blocks are not correctly
	// terminated.
	pub(crate) block_terminals: HashMap<Block, TerminalInstruction>,

	function: LLVMValueRef,
	pub name: String,
	pub return_type: LLVMTypeRef,
}

impl Function {
	pub fn get_function(&self) -> LLVMValueRef {
		return self.function;
	}

	pub fn add_block(&mut self, block: Block) {
		self.blocks.insert(block.get_block(), block);
	}

	pub fn set_block_terminal(&mut self, block: Block, terminal: TerminalInstruction) {
		self.block_terminals.insert(block, terminal);
	}

	pub fn get_block_terminal(&self, block: Block) -> Option<&TerminalInstruction> {
		self.block_terminals.get(&block)
	}

	pub fn get_block_terminal_mut(&mut self, block: Block) -> Option<&mut TerminalInstruction> {
		self.block_terminals.get_mut(&block)
	}

	pub fn delete_block_terminal(&mut self, block: Block) {
		self.block_terminals.remove(&block);
	}
}

impl Module {
	pub fn create_function(&mut self, name: &str, arg_types: &Vec<Type>, return_type: Type) -> FunctionKey {
		let mut arguments = Vec::new();
		for &arg_type in arg_types {
			arguments.push(self.to_llvm_type(arg_type));
		}

		let function_type;
		let function;
		unsafe {
			function_type = LLVMFunctionType(self.to_llvm_type(return_type), arguments.as_mut_ptr(), arguments.len() as u32, 0);
			function = LLVMAddFunction(self.get_module(), self.string_table.to_llvm_string(name), function_type);
		}

		let function = Function {
			blocks: HashMap::new(),
			block_terminals: HashMap::new(),
			function,
			name: String::from(name),
			return_type: function_type,
		};

		self.function_table.add_function(name, function)
	}
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FunctionKey {
	name: String,
}

#[derive(Debug, Default)]
pub struct FunctionTable {
	functions: HashMap<FunctionKey, Function>,
	functions_by_ref: HashMap<LLVMValueRef, FunctionKey>,
}

impl FunctionTable {
	pub fn add_function(&mut self, name: &str, function: Function) -> FunctionKey {
		let key = FunctionKey {
			name: String::from(name),
		};

		self.functions_by_ref.insert(function.get_function(), key.clone());
		self.functions.insert(key.clone(), function);

		return key;
	}

	pub fn get_function(&self, key: &FunctionKey) -> Option<&Function> {
		self.functions.get(key)
	}

	pub fn get_function_mut(&mut self, key: &FunctionKey) -> Option<&mut Function> {
		self.functions.get_mut(key)
	}

	pub fn get_function_by_ref(&self, function: LLVMValueRef) -> Option<FunctionKey> {
		if let Some(key) = self.functions_by_ref.get(&function) {
			return Some(key.clone());
		} else {
			return None;
		}
	}

	pub fn get_function_by_ref_mut(&self, function: LLVMValueRef) -> Option<FunctionKey> {
		if let Some(key) = self.functions_by_ref.get(&function) {
			return Some(key.clone());
		} else {
			return None;
		}
	}
}
