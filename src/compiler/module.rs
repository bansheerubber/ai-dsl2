use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use crate::compiler::{ FunctionTable, Type };
use crate::utility::strings::StringTable;

#[derive(Debug)]
pub struct Module {
	builder: LLVMBuilderRef,
	context: LLVMContextRef,
	pub function_table: FunctionTable,
	module: LLVMModuleRef,
	self_reference: Weak<RefCell<Module>>,
	pub string_table: StringTable, // keep the strings alive for as long as we are using LLVM resources
}

impl Module {
	pub fn new(name: &str) -> Rc<RefCell<Self>> {
		let mut string_table = StringTable::default();

		let context;
		let module;
		let builder;
		unsafe {
			context = LLVMContextCreate();
			module = LLVMModuleCreateWithName(string_table.to_llvm_string(name));
			builder = LLVMCreateBuilderInContext(context);
		}


		Rc::new_cyclic(|self_reference| {
			RefCell::new(
				Module {
					builder,
					context,
					function_table: FunctionTable::default(),
					module,
					self_reference: self_reference.clone(),
					string_table,
				}
			)
		})
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

	fn upgrade(&self) -> Rc<RefCell<Module>> {
		self.self_reference.upgrade().unwrap()
	}

	pub fn create_function(&mut self, name: &str, arg_types: &Vec<Type>, return_type: Type) {
		let mut arguments = Vec::new();
		for &arg_type in arg_types {
			arguments.push(self.to_llvm_type(arg_type));
		}

		let function_type;
		let function;
		unsafe {
			function_type = LLVMFunctionType(self.to_llvm_type(return_type), arguments.as_mut_ptr(), arguments.len() as u32, 0);
			function = LLVMAddFunction(self.module, self.string_table.to_llvm_string(name), function_type);
		}

		self.function_table.add_function(self.upgrade(), name, function, function_type);
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
