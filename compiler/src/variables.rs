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

	pub fn get_variable(&mut self, block: Block, name: &str) -> Result<Value, MathError> {
		if let Some(variable) = self.variable_table.get(block.get_parent(), name) {
			Ok(variable.value)
		} else {
			Err(MathError::UndefinedVariable(String::from(name)))
		}
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
}
