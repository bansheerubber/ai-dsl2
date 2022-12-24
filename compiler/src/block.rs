use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Builder, FunctionKey, Module, strings };

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalInstruction {
	Branch {
		instruction: LLVMValueRef,
		target: LLVMBasicBlockRef,
	},
	ConditionalBranch {
		instruction: LLVMValueRef,
		target_if_false: LLVMBasicBlockRef,
		target_if_true: LLVMBasicBlockRef,
	},
	Default {
		instruction: LLVMValueRef,
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

impl TerminalInstruction {
	pub fn get_instruction_ref(&self) -> LLVMValueRef {
		match *self {
			TerminalInstruction::Branch { instruction, target: _, } => instruction,
			TerminalInstruction::ConditionalBranch { instruction, target_if_true: _, target_if_false: _, } => instruction,
			TerminalInstruction::Default { instruction, } => instruction,
			TerminalInstruction::Return { instruction, value: _, } => instruction,
			TerminalInstruction::ReturnVoid { instruction, } => instruction,
			TerminalInstruction::None => panic!("Could not get terminal ref"),
			TerminalInstruction::Unknown { instruction, } => instruction,
		}
	}
}

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
	pub fn new_block(&mut self, name: &str, function: &FunctionKey) -> Block {
		let function = self.function_table.get_function_mut(&function).unwrap();

		// had to inline function b/c non-lexical lifetimes do not extend through functions
		unsafe {
			let block = LLVMAppendBasicBlock(
				function.get_function(),
				self.string_table.to_llvm_string(name)
			);

			let builder = Builder::new();
			LLVMPositionBuilderAtEnd(builder.get_builder(), block);
			let instruction = LLVMBuildRetVoid(builder.get_builder());

			let block = Block {
				block,
			};

			function.add_block(block);
			function.set_block_terminal(block, TerminalInstruction::Default { instruction, });

			return block;
		};
	}

	// modifies current block, returns reference to old block
	pub fn split_block_in_place(&mut self, block: &mut Block) -> Block {
		let name = block.get_name() + "cont";
		let parent = block.get_parent();

		let old_block = block.clone();

		*block = self.new_block(&name, &self.function_table.get_function_by_ref(parent).unwrap());

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

	pub(crate) fn set_block_terminal(&mut self, block: Block, terminal: TerminalInstruction) {
		self.delete_block_terminal(block);

		let key = self.function_table.get_function_by_ref(block.get_parent()).unwrap();
		self.function_table.get_function_mut(&key).unwrap().set_block_terminal(block, terminal);
	}

	pub(crate) fn delete_block_terminal(&mut self, block: Block) {
		let key = self.function_table.get_function_by_ref(block.get_parent()).unwrap();
		let terminal = if let Some(terminal) = self.function_table.get_function_mut(&key).unwrap().get_block_terminal(block) {
			terminal
		} else {
			return;
		};

		unsafe {
			LLVMInstructionEraseFromParent(terminal.get_instruction_ref());
			self.function_table.get_function_mut(&key).unwrap().delete_block_terminal(block);
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
