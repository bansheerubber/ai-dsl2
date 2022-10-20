use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, MathError, Module, Type };

#[derive(Clone, Copy, Debug)]
pub struct Value {
	pub type_enum: Type,
	pub value: LLVMValueRef,
}

impl Module {
	pub fn add_addition(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;
			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Float => LLVMBuildFAdd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("addftmp")
				),
				Type::Integer => LLVMBuildAdd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("addtmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_subtraction(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;
			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Float => LLVMBuildFSub(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("subftmp")
				),
				Type::Integer => LLVMBuildSub(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("subtmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_multiplication(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;
			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Float => LLVMBuildFMul(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("mulftmp")
				),
				Type::Integer => LLVMBuildMul(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("multmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn math_type_aliasing(&self, type1: Type, type2: Type) -> Result<Type, MathError> {
		if type1 == type2 {
			Ok(type1)
		}
		else if type1 == Type::Float || type2 == Type::Float {
			Ok(Type::Float)
		} else {
			Err(MathError::IncompatibleTypes)
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

	pub fn math_resolve_value(&mut self, block: Block, value: Value, result_type: Type) -> Value {
		let resolved = self.resolve_value(block, value);
		self.convert_to_type(block, resolved, result_type)
	}

	pub fn convert_to_type(&mut self, block: Block, value: Value, result_type: Type) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			if value.type_enum == result_type {
				return value;
			}

			let value = match value.type_enum {
				Type::Float => {
					match result_type {
						Type::Integer => {
							LLVMBuildFPToUI(
								builder.get_builder(),
								value.value,
								self.to_llvm_type(Type::Integer),
								self.string_table.to_llvm_string("t")
							)
						},
						_ => todo!(),
					}
				},
				Type::Integer => {
					match result_type {
						Type::Float => {
							LLVMBuildUIToFP(
								builder.get_builder(),
								value.value,
								self.to_llvm_type(Type::Float),
								self.string_table.to_llvm_string("t")
							)
						},
						_ => todo!(),
					}
				},
				_ => todo!(),
			};

			Value {
				type_enum: result_type,
				value,
			}
		}
	}
}
