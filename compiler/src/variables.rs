use std::collections::HashMap;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, MathError, Module, Type, Value };

#[derive(Debug)]
pub struct Variable {
	type_enum: Type,
	is_mutable: bool,
	name: String,
	value: Value,
}

#[derive(Debug, Default)]
pub struct VariableTable {
	variables: HashMap<LLVMValueRef, HashMap<String, Variable>>,
}

impl VariableTable {
	pub fn add(&mut self, function: LLVMValueRef, variable: Variable) {
		if !self.variables.contains_key(&function) {
			self.variables.insert(function, HashMap::new());
		}

		self.variables.get_mut(&function).unwrap().insert(variable.name.clone(), variable);
	}

	pub fn get(&mut self, function: LLVMValueRef, name: &str) -> Option<&Variable> {
		if !self.variables.contains_key(&function) {
			None
		} else {
			self.variables.get(&function).unwrap().get(name)
		}
	}
}

impl Module {
	pub fn add_immutable_variable(&mut self, block: Block, name: &str, type_enum: Type) -> Value {
		// TODO we're going to use the stack, even for immutable variables, for ease of design
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let value = Value {
				type_enum: self.upgrade_type(type_enum),
				value: LLVMBuildAlloca(
					builder.get_builder(),
					self.to_llvm_type(type_enum),
					self.string_table.to_llvm_string(name)
				),
			};

			self.variable_table.add(
				block.get_parent(),
				Variable {
					type_enum,
					is_mutable: false,
					name: String::from(name),
					value,
				}
			);

			return value;
		}
	}

	pub fn add_immutable_array(&mut self, block: Block, type_enum: Type, element_count: usize) -> Value {
		// TODO we're going to use the stack, even for immutable variables, for ease of design
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let value = Value {
				type_enum: type_enum.to_array(element_count),
				value: LLVMBuildAlloca(
					builder.get_builder(),
					LLVMArrayType(self.to_llvm_type(type_enum), element_count as u32),
					self.string_table.to_llvm_string("array")
				),
			};

			return value;
		}
	}

	pub fn add_mutable_variable(&mut self, block: Block, name: &str, type_enum: Type) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let value = Value {
				type_enum: self.upgrade_type(type_enum),
				value: LLVMBuildAlloca(
					builder.get_builder(),
					self.to_llvm_type(type_enum),
					self.string_table.to_llvm_string(name)
				),
			};


			self.variable_table.add(
				block.get_parent(),
				Variable {
					type_enum,
					is_mutable: true,
					name: String::from(name),
					value,
				}
			);

			return value;
		}
	}

	pub fn add_global_variable(&mut self, name: &str, type_enum: Type) -> Value {
		unsafe {
			let value = Value {
				type_enum: self.upgrade_type(type_enum),
				value: LLVMAddGlobal(
					self.get_module(),
					self.to_llvm_type(type_enum),
					self.string_table.to_llvm_string(name)
				),
			};

			self.variable_table.add(
				std::ptr::null_mut(),
				Variable {
					type_enum,
					is_mutable: true,
					name: String::from(name),
					value,
				}
			);

			match type_enum {
				Type::Float(_) => {
					LLVMSetInitializer(value.value, LLVMConstReal(self.to_llvm_type(Type::Float(0)), 0.0));
				}
				Type::Integer(_, bits) => {
					LLVMSetInitializer(value.value, LLVMConstInt(self.to_llvm_type(Type::Integer(0, bits)), 0, 0));
				},
				_ => todo!(),
			}

			return value;
		}
	}

	pub fn get_variable(&mut self, block: Block, name: &str) -> Result<Value, MathError> {
		if let Some(variable) = self.variable_table.get(block.get_parent(), name) {
			return Ok(variable.value);
		}

		if let Some(variable) = self.variable_table.get(std::ptr::null_mut(), name) {
			return Ok(variable.value);
		}

		Err(MathError::UndefinedVariable(String::from(name)))
	}

	pub fn add_store(&mut self, block: Block, location: Value, value: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let value = self.math_resolve_value(block, value, location.type_enum); // resolve & convert type
			if value.type_enum != self.downgrade_type(location.type_enum) {
				return Err(MathError::IncompatibleTypes(location.type_enum, value.type_enum));
			}

			Ok(Value {
				type_enum: Type::Void,
				value: LLVMBuildStore(builder.get_builder(), value.value, location.value),
			})
		}
	}

	pub fn add_store_to_array(
		&mut self, block: Block, array: Value, index: usize, value: Value
	) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let scalar = array.type_enum.to_scalar();

			let value = self.math_resolve_value(block, value, scalar); // resolve & convert type
			if value.type_enum != scalar {
				return Err(MathError::IncompatibleTypes(scalar, value.type_enum));
			}

			let mut args = [
				LLVMConstInt(self.to_llvm_type(Type::Integer(0, 64)), 0, 0),
				LLVMConstInt(self.to_llvm_type(Type::Integer(0, 64)), index as u64, 0),
			];

			let array_element = LLVMBuildGEP2(
				builder.get_builder(),
				self.to_llvm_type(array.type_enum),
				array.value,
				args.as_mut_ptr(),
				2,
				self.string_table.to_llvm_string("element")
			);

			Ok(Value {
				type_enum: Type::Void(0),
				value: LLVMBuildStore(builder.get_builder(), value.value, array_element),
			})
		}
	}

	pub fn add_argument(&mut self, block: Block, name: &str, type_enum: Type, value: Value) {
		self.variable_table.add(
			block.get_parent(),
			Variable {
				type_enum,
				is_mutable: true,
				name: String::from(name),
				value,
			}
		);
	}
}
