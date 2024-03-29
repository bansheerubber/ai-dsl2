use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Builder, FunctionKey, Module, Type, Value, strings };

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

	pub fn add_function_call(&mut self, block: Block, function: &FunctionKey, args: &mut [Value]) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let check_arguments = self.function_table.get_function(&function).unwrap().check_arguments;
			let function_argument_types = self.function_table.get_function(&function).unwrap().argument_types.iter()
				.map(|x| *x)
				.collect::<Vec<Type>>();

			if args.len() != function_argument_types.len() && check_arguments {
				panic!("Incorrect number of function arguments");
			}

			let arg_types = args.iter().map(|x| x.type_enum).collect::<Vec<Type>>();

			// test argument types
			for (arg, arg_type) in arg_types.iter().zip(function_argument_types.iter()) {
				if !arg.is_compatible(arg_type) && check_arguments {
					panic!("Incorrect function argument types {:?} {:?}", arg_types, function_argument_types);
				}
			}

			let mut llvm_args = vec![];

			if check_arguments {
				for (arg, arg_type) in args.iter().zip(function_argument_types.iter()) {
					llvm_args.push(self.math_resolve_value(block, *arg, *arg_type).value);
				}
			} else {
				for arg in args.iter() {
					llvm_args.push(arg.value);
				}
			}


			let function = self.function_table.get_function(&function).unwrap();

			let value = LLVMBuildCall2(
				builder.get_builder(),
				function.get_function_type(),
				function.get_function(),
				llvm_args.as_mut_ptr(),
				llvm_args.len() as u32,
				self.string_table.to_llvm_string("") // TODO what is this for?
			);

			Value {
				type_enum: function.return_type,
				value,
			}
		}
	}
}
