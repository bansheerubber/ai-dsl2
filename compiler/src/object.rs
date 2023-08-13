use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::LLVMGetModuleDataLayout;
use llvm_sys::target::LLVMSizeOfTypeInBits;
use std::collections::HashMap;

use crate::{ Module, Type, };

// provides everything that is needed to talk to LLVM concerning this struct type
// 1. property types & indices
// 2. object size
// 3. struct typeref
// 4. struct name
#[derive(Debug)]
pub(crate) struct StructType {
 pub(crate) name: String,
 pub(crate) property_to_index: HashMap<String, usize>,
 pub(crate) size: usize,
 pub(crate) type_ref: LLVMTypeRef,
}

#[derive(Debug, Default)]
pub struct TypeTable {
	structs: HashMap<String, StructType>,
}

impl Module {
	// creates a struct type by calculating its size and then throwing it into the type table
	pub fn create_struct_type(&mut self, name: &str, properties: HashMap<String, Type>) {
		unsafe {
			let type_ref = LLVMStructCreateNamed(self.get_context(), self.string_table.to_llvm_string(&name));

			let mut property_to_index = HashMap::new();
			let mut arguments = Vec::new();
			let mut counter = 0;
			for (name, &arg_type) in properties.iter() {
				arguments.push(self.to_llvm_type(arg_type));
				property_to_index.insert(name.to_string(), counter);

				counter += 1;
			}

			LLVMStructSetBody(type_ref, arguments.as_mut_ptr(), arguments.len() as u32, 0);

			let data_layout = LLVMGetModuleDataLayout(self.get_module());

			self.type_table.structs.insert(
				name.to_string(),
				StructType {
					name: name.to_string(),
					property_to_index,
					size: (LLVMSizeOfTypeInBits(data_layout, type_ref) / 8) as usize,
					type_ref,
				},
			);
		}

		/*
			LLVMStructCreateNamed() - creates a struct with a name, returns type ref
			LLVMStructSetBody() - sets the properties of the struct(?)

			other useful functions:
			- LLVMGetStructElementTypes
			- LLVMCountStructElementTypes
			- LLVMStructGetTypeAtIndex
			- LLVMBuildStructGEP2 (GEP = get element pointer)
		*/
	}
}
