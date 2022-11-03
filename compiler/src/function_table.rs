use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;

use crate::{ Block, Module, Type };

#[derive(Clone, Debug)]
pub struct Function {
	pub blocks: HashMap<LLVMBasicBlockRef, Block>,
	function: LLVMValueRef,
	pub name: String,
	pub return_type: LLVMTypeRef,
}

impl Function {
	pub fn get_function(&self) -> LLVMValueRef {
		return self.function;
	}
}

impl Module {
	pub fn create_function(&mut self, name: &str, arg_types: &Vec<Type>, return_type: Type) -> &Function {
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
			function,
			name: String::from(name),
			return_type: function_type,
		};

		self.function_table.add_function(name, function);

		self.function_table.get_function(name).unwrap()
	}
}

#[derive(Debug, Default)]
pub struct FunctionTable {
	functions: HashMap<String, Function>,
}

impl FunctionTable {
	pub fn add_function(&mut self, name: &str, function: Function) {
		self.functions.insert(String::from(name), function);
	}

	pub fn get_function(&self, name: &str) -> Option<&Function> {
		self.functions.get(&String::from(name))
	}
}
