use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, FunctionTable, TerminalInstruction, Type, Value, VariableTable, };
use crate::strings::StringTable;

#[derive(Debug)]
pub struct Module {
	context: LLVMContextRef,
	pub function_table: FunctionTable,
	module: LLVMModuleRef,
	pub string_table: StringTable, // keep the strings alive for as long as we are using LLVM resources
	pub variable_table: VariableTable,
}

impl Module {
	pub fn new(name: &str) -> Self {
		let mut string_table = StringTable::default();

		let context;
		let module;
		unsafe {
			context = LLVMContextCreate();
			module = LLVMModuleCreateWithName(string_table.to_llvm_string(name));
		}

		Module {
			context,
			function_table: FunctionTable::default(),
			module,
			string_table,
			variable_table: VariableTable::default(),
		}
	}

	pub fn to_llvm_type(&self, type_enum: Type) -> LLVMTypeRef {
		unsafe {
			match type_enum {
				Type::CString(0) => {
					let i8_type = LLVMIntType(8);
					LLVMPointerType(i8_type, 0)
				},
				Type::Float(0) => LLVMDoubleType(),
				Type::Float(1) => LLVMPointerType(LLVMDoubleType(), 64),
				Type::Integer(0, bits) => LLVMIntType(bits),
				Type::Integer(1, bits) => LLVMPointerType(LLVMIntType(bits), 64),
				Type::Void => LLVMVoidType(),
				_ => LLVMVoidType(),
			}
		}
	}

	pub fn write_bitcode(&mut self, filename: &str) {
		unsafe {
			LLVMWriteBitcodeToFile(self.module, self.string_table.to_llvm_string(filename));
		}
	}

	pub fn create_global_string(&mut self, block: Block, string: &str) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			Value {
				type_enum: Type::CString(0),
				value: LLVMBuildGlobalStringPtr(
					builder.get_builder(),
					self.string_table.to_llvm_string(string),
					self.string_table.to_llvm_string("") // TODO what is this for?
				),
			}
		}
	}

	pub fn add_return(&mut self, block: Block, value: Value) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			self.set_block_terminal(
				block,
				TerminalInstruction::Return {
					instruction: LLVMBuildRet(builder.get_builder(), value.value),
					value: value.value,
				}
			);
		}
	}

	pub fn add_return_void(&mut self, block: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			self.set_block_terminal(
				block,
				TerminalInstruction::ReturnVoid {
					instruction: LLVMBuildRetVoid(builder.get_builder()),
				}
			);
		}
	}

	pub fn add_branch(&mut self, block: Block, target: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			self.set_block_terminal(
				block,
				TerminalInstruction::Branch {
					instruction: LLVMBuildBr(builder.get_builder(), target.get_block()),
					target: target.get_block(),
				}
			);
		}
	}

	pub fn add_branch_if_true(&mut self, block: Block, value: Value, if_true: Block, if_false: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let instruction = LLVMBuildCondBr(
				builder.get_builder(),
				self.math_resolve_value(block, value, Type::Integer(0, 1)).value,
				if_true.get_block(),
				if_false.get_block()
			);

			self.set_block_terminal(
				block,
				TerminalInstruction::ConditionalBranch {
					instruction,
					target_if_false: if_false.get_block(),
					target_if_true: if_true.get_block(),
				}
			);
		}
	}

	pub fn get_context(&self) -> LLVMContextRef {
		self.context
	}

	pub fn get_module(&self) -> LLVMModuleRef {
		self.module
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeModule(self.module);
			LLVMContextDispose(self.context);
		}
	}
}
