use std::collections::HashMap;
use llvm_sys::core::*;

use crate::{ Block, Builder, Module, Type, Value };

#[derive(Debug, Default)]
pub struct Variable {
	type_enum: Type,
	is_mutable: bool,
	name: String,
}

#[derive(Debug, Default)]
pub struct VariableTable {
	variables: HashMap<Block, HashMap<String, Variable>>,
}

impl VariableTable {
	pub fn add(&mut self, block: Block, variable: Variable) {
		if !self.variables.contains_key(&block) {
			self.variables.insert(block, HashMap::new());
		}

		self.variables.get_mut(&block).unwrap().insert(variable.name.clone(), variable);
	}
}

impl Module {
	pub fn add_immutable_variable(&mut self, block: Block, name: &str, type_enum: Type) -> Value {
		self.variable_table.add(
			block,
			Variable {
				type_enum: Type::Void,
				is_mutable: false,
				name: String::from(name),
			}
		);

		// TODO we're going to use the stack, even for immutable variables, for ease of design
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			Value {
				type_enum: Type::Void,
				value: LLVMBuildAlloca(
					builder.get_builder(),
					self.to_llvm_type(type_enum),
					self.string_table.to_llvm_string(name)
				),
			}
		}
	}

	pub fn add_mutable_variable(&mut self, block: Block, name: &str, type_enum: Type) -> Value {
		self.variable_table.add(
			block,
			Variable {
				type_enum: Type::Void,
				is_mutable: true,
				name: String::from(name),
			}
		);

		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			Value {
				type_enum: Type::Void,
				value: LLVMBuildAlloca(
					builder.get_builder(),
					self.to_llvm_type(type_enum),
					self.string_table.to_llvm_string(name)
				),
			}
		}
	}
}
