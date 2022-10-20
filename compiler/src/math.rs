use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, Module, Type };

#[derive(Clone, Copy, Debug)]
pub struct Value {
	pub type_enum: Type,
	pub value: LLVMValueRef,
}

impl Module {
	pub fn add_addition(&mut self, block: Block, lhs: Value, rhs: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let lhs = self.resolve_value(block, lhs);
			let rhs = self.resolve_value(block, rhs);
			Value {
				type_enum: Type::Integer,
				value: LLVMBuildAdd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("addtmp")
				),
			}
		}
	}

	pub fn add_subtraction(&mut self, block: Block, lhs: Value, rhs: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let lhs = self.resolve_value(block, lhs);
			let rhs = self.resolve_value(block, rhs);
			Value {
				type_enum: Type::Integer,
				value: LLVMBuildSub(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("subtmp")
				),
			}
		}
	}

	pub fn add_multiplication(&mut self, block: Block, lhs: Value, rhs: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let lhs = self.resolve_value(block, lhs);
			let rhs = self.resolve_value(block, rhs);
			Value {
				type_enum: Type::Integer,
				value: LLVMBuildMul(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("multmp")
				),
			}
		}
	}

	pub fn create_immediate_int(&self, number: u64) -> Value {
		unsafe {
			Value {
				type_enum: Type::Integer,
				value: LLVMConstInt(self.to_llvm_type(Type::Integer), number, 0),
			}
		}
	}

	pub fn add_global_int(&mut self, block: Block, number: u64) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let global = LLVMAddGlobal(
				self.get_module(),
				self.to_llvm_type(Type::Integer),
				self.string_table.to_llvm_string(&format!("c{}", number))
			);

			LLVMSetInitializer(global, LLVMConstInt(self.to_llvm_type(Type::Integer), number, 0));

			Value {
				type_enum: Type::IntegerPointer,
				value: global,
			}
		}
	}

	pub fn create_immediate_float(&self, number: f64) -> Value {
		unsafe {
			Value {
				type_enum: Type::Float,
				value: LLVMConstReal(self.to_llvm_type(Type::Float), number),
			}
		}
	}

	// resolves pointers into temporary variables
	pub fn resolve_value(&mut self, block: Block, value: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			match value.type_enum {
				Type::IntegerPointer => {
					Value {
						type_enum: Type::Integer,
						value: LLVMBuildLoad2(
							builder.get_builder(),
							self.to_llvm_type(Type::Integer),
							value.value,
							self.string_table.to_llvm_string("t")
						),
					}
				},
				_ => value,
			}
		}
	}
}
