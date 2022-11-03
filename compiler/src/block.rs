use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Builder, FunctionKey, Module, strings };

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalInstruction {
	ConditionalBranch {
		instruction: LLVMValueRef,
		target_if_false: LLVMBasicBlockRef,
		target_if_true: LLVMBasicBlockRef,
	},
	Branch {
		instruction: LLVMValueRef,
		target: LLVMBasicBlockRef,
	},
	None, // we should never be in this state unless only temporarily
	Return {
		instruction: LLVMValueRef,
		value: LLVMValueRef,
	},
	ReturnVoid {
		instruction: LLVMValueRef,
	},
	Unknown {
		instruction: LLVMValueRef,
	},
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Block {
	block: LLVMBasicBlockRef,

	// blocks are not self-terminating. what this means is that the code_generator may compile a block, add all sorts of
	// instructions to it, but not add a terminating instruction in the context the block was created in. whenever the
	// next block in the same function is created by the code_generator, that new block has control over how the original
	// block terminates.
	//
	// for compilation safety purposes, all blocks are created with a "return void" terminating instruction. this is only
	// to assist with debugging, since the disassembler formats block locations incorrectly if blocks are not correctly
	// terminated.
	terminal: TerminalInstruction,
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

	pub fn get_terminal_ref(&self) -> LLVMValueRef {
		match self.terminal {
			TerminalInstruction::ConditionalBranch { instruction, target_if_true: _, target_if_false: _, } => instruction,
			TerminalInstruction::Branch { instruction, target: _, } => instruction,
			TerminalInstruction::Return { instruction, value: _, } => instruction,
			TerminalInstruction::ReturnVoid { instruction, } => instruction,
			TerminalInstruction::None => panic!("Could not get terminal ref"),
			TerminalInstruction::Unknown { instruction, } => instruction,
		}
	}
}

impl Module {
	pub fn new_block(&mut self, name: &str, function: &FunctionKey) -> Block {
		let function = self.function_table.get_function_mut(&function).unwrap();

		// had to inline function b/c non-lexical lifetimes do not extend through functions
		let block = unsafe {
			let block = LLVMAppendBasicBlock(
				function.get_function(),
				self.string_table.to_llvm_string(name)
			);

			let builder = Builder::new();
			LLVMPositionBuilderAtEnd(builder.get_builder(), block);
			let instruction = LLVMBuildRetVoid(builder.get_builder());

			Block {
				block,
				terminal: TerminalInstruction::ReturnVoid {
					instruction,
				},
			}
		};

		function.add_block(block);

		return block;
	}

	pub(crate) fn new_block_from_llvm_ref(&mut self, name: &str, function: LLVMValueRef) -> Block {
		// TODO generate terminating instruction
		unsafe {
			let block = LLVMAppendBasicBlock(
				function,
				self.string_table.to_llvm_string(name)
			);

			let builder = Builder::new();
			LLVMPositionBuilderAtEnd(builder.get_builder(), block);
			let instruction = LLVMBuildRetVoid(builder.get_builder());

			Block {
				block,
				terminal: TerminalInstruction::ReturnVoid {
					instruction,
				},
			}
		}
	}

	// modifies current block, returns reference to old block
	pub fn split_block_in_place(&mut self, block: &mut Block) -> Block {
		let name = block.get_name() + "cont";
		let parent = block.get_parent();

		let old_block = block.clone();
		*block = self.new_block_from_llvm_ref(&name, parent);

		return old_block;
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

	pub(crate) fn set_block_terminal(&mut self, mut block: Block, terminal: TerminalInstruction) {
		self.delete_block_terminal(block);
		block.terminal = terminal;
	}

	pub(crate) fn delete_block_terminal(&mut self, mut block: Block) {
		if block.terminal == TerminalInstruction::None {
			return;
		}

		unsafe {
			LLVMInstructionEraseFromParent(block.get_terminal_ref());
			block.terminal = TerminalInstruction::None;
		}
	}

	pub fn add_function_call(&mut self, block: Block, function: &FunctionKey, args: &mut [LLVMValueRef]) {
		let function = self.function_table.get_function(&function).unwrap();
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			LLVMBuildCall2(
				builder.get_builder(),
				function.return_type,
				function.get_function(),
				args.as_mut_ptr(),
				args.len() as u32,
				self.string_table.to_llvm_string("") // TODO what is this for?
			);
		}
	}
}
