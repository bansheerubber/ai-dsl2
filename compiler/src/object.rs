use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use std::collections::HashMap;

use crate::FunctionKey;
use crate::MathError;
use crate::{ Block, Builder, Module, Type, Value, };

// provides everything that is needed to talk to LLVM concerning this struct type
// 1. property types & indices
// 2. object size
// 3. struct typeref
// 4. struct name
#[derive(Debug)]
pub(crate) struct StructType {
 pub(crate) name: String,
 pub(crate) property_to_index: HashMap<String, usize>,
 pub(crate) property_to_type: HashMap<String, Type>,
 pub(crate) size: usize,
 pub(crate) type_index: usize,
 pub(crate) type_ref: LLVMTypeRef,
}

#[derive(Debug, Default)]
pub struct TypeTable {
	index_to_struct: Vec<String>,
	structs: HashMap<String, StructType>,
}

impl Module {
	// creates a struct type by calculating its size and then throwing it into the type table
	pub fn create_struct_type(&mut self, name: &str, properties: HashMap<String, Type>) {
		unsafe {
			let type_ref = LLVMStructCreateNamed(self.get_context(), self.string_table.to_llvm_string(&format!("struct.{}", name)));

			let mut property_to_index = HashMap::new();
			let mut property_to_type = HashMap::new();
			let mut arguments = Vec::new();
			let mut counter = 0;
			for (name, &arg_type) in properties.iter() {
				arguments.push(self.to_llvm_type(arg_type));
				property_to_index.insert(name.to_string(), counter);
				property_to_type.insert(name.to_string(), arg_type);

				counter += 1;
			}

			LLVMStructSetBody(type_ref, arguments.as_mut_ptr(), arguments.len() as u32, 0);

			let data_layout = LLVMGetModuleDataLayout(self.get_module());

			self.type_table.structs.insert(
				name.to_string(),
				StructType {
					name: name.to_string(),
					property_to_index,
					property_to_type,
					size: (LLVMSizeOfTypeInBits(data_layout, type_ref) / 8) as usize,
					type_index: self.type_table.index_to_struct.len(),
					type_ref,
				},
			);

			self.type_table.index_to_struct.push(name.to_string());
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

	// allocates a struct
	pub fn add_struct_malloc(&mut self, block: Block, struct_type_name: &str) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let struct_size = self.type_table.structs.get(struct_type_name).unwrap().size;

			let malloc = self.add_function_call(
				block,
				&FunctionKey {
					name: String::from("malloc"),
				},
				&mut [Value {
					type_enum: Type::Integer(0, 32),
					value: LLVMConstInt(LLVMIntType(32), struct_size as u64, 0),
				}],
			);

			let struct_type = self.type_table.structs.get(struct_type_name).unwrap();

			Value {
				type_enum: Type::Struct(1, struct_type.type_index),
				value: malloc.value,
			}
		}
	}

	pub fn add_store_to_obj(
		&mut self,
		block: Block,
		obj: Value,
		property: &str,
		property_value: Value
	) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let Type::Struct(_, type_index) = obj.type_enum else {
				return Err(MathError::UnsupportedOperation);
			};

			let type_name = self.type_table.index_to_struct.get(type_index).unwrap();

			let struct_type = self.type_table.structs
				.get(type_name)
				.unwrap();

			let property_type = struct_type
				.property_to_type
				.get(property)
				.unwrap();

			let property_index = *struct_type
				.property_to_index
				.get(property)
				.unwrap();

			let property_location = LLVMBuildStructGEP2(
				builder.get_builder(),
				struct_type.type_ref,
				obj.value,
				property_index as u32,
				self.string_table.to_llvm_string(&format!("{}.{}", type_name, property))
			);

			Ok(Value {
				type_enum: *property_type,
				value: LLVMBuildStore(builder.get_builder(), property_value.value, property_location),
			})
		}
	}

	// looks up the struct type index from struct name
	pub fn lookup_struct_type_index(&self, type_name: &str) -> usize {
		self.type_table.structs.get(type_name).unwrap().type_index
	}

	pub(crate) fn lookup_struct_type(&self, type_index: usize) -> &StructType {
		let type_name = &self.type_table.index_to_struct[type_index];
		self.type_table.structs.get(type_name).unwrap()
	}
}
