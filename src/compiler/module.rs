use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::compiler::{ FunctionTable, Type };
use crate::utility::strings::StringTable;

#[derive(Debug)]
pub struct Module {
	builder: LLVMBuilderRef,
	context: LLVMContextRef,
	pub function_table: FunctionTable,
	module: LLVMModuleRef,
	pub string_table: StringTable, // keep the strings alive for as long as we are using LLVM resources
}

impl Module {
	pub fn new(name: &str) -> Self {
		let mut string_table = StringTable::default();

		let context;
		let module;
		let builder;
		unsafe {
			context = LLVMContextCreate();
			module = LLVMModuleCreateWithName(string_table.to_llvm_string(name));
			builder = LLVMCreateBuilderInContext(context);
		}

		Module {
			builder,
			context,
			function_table: FunctionTable::default(),
			module,
			string_table,
		}
	}

	pub fn to_llvm_type(&self, type_enum: Type) -> LLVMTypeRef {
		unsafe {
			match type_enum {
				Type::CString => {
					let i8_type = LLVMIntTypeInContext(self.context, 8);
					LLVMPointerType(i8_type, 0)
				},
				Type::Float => LLVMDoubleTypeInContext(self.context),
				Type::Integer => LLVMIntTypeInContext(self.context, 64),
				Type::Void => LLVMVoidTypeInContext(self.context),
			}
		}
	}

	pub fn write_bitcode(&mut self, filename: &str) {
		unsafe {
			LLVMWriteBitcodeToFile(self.module, self.string_table.to_llvm_string(filename));
		}
	}

	pub fn create_global_string(&mut self, string: &str) -> LLVMValueRef {
		unsafe {
			// TODO for this to not seg fault, we need to have the builder positioned at the end of a block? what if its positioned anywhere?
			LLVMBuildGlobalStringPtr(
				self.builder,
				self.string_table.to_llvm_string(string),
				self.string_table.to_llvm_string("") // TODO what is this for?
			)
		}
	}

	pub fn get_builder(&self) -> LLVMBuilderRef {
		self.builder
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
			LLVMDisposeBuilder(self.builder);
			LLVMDisposeModule(self.module);
			LLVMContextDispose(self.context);
		}
	}
}
