use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, FunctionTable, Type, Value, VariableTable, };
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
				Type::CString => {
					let i8_type = LLVMIntType(8);
					LLVMPointerType(i8_type, 0)
				},
				Type::Float => LLVMDoubleType(),
				Type::Integer => LLVMIntType(64),
				Type::Void => LLVMVoidType(),
			}
		}
	}

	pub fn write_bitcode(&mut self, filename: &str) {
		unsafe {
			LLVMWriteBitcodeToFile(self.module, self.string_table.to_llvm_string(filename));
		}
	}

	pub fn create_global_string(&mut self, block: Block, string: &str) -> LLVMValueRef {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			// TODO for this to not seg fault, we need to have the builder positioned at the end of a block? what if its positioned anywhere?
			LLVMBuildGlobalStringPtr(
				builder.get_builder(),
				self.string_table.to_llvm_string(string),
				self.string_table.to_llvm_string("") // TODO what is this for?
			)
		}
	}

	pub fn add_return(&mut self, block: Block, value: Value) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			LLVMBuildRet(builder.get_builder(), value.value);
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