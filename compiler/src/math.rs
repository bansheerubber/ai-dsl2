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
				Type::Float(0) => LLVMBuildFAdd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("addftmp")
				),
				Type::Integer(0, _) => LLVMBuildAdd(
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
				Type::Float(0) => LLVMBuildFSub(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("subftmp")
				),
				Type::Integer(0, _) => LLVMBuildSub(
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
				Type::Float(0) => LLVMBuildFMul(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("mulftmp")
				),
				Type::Integer(0, _) => LLVMBuildMul(
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
		if let Type::Integer(_, bits1) = type1 {
			if let Type::Integer(_, bits2) = type2 {
				return Ok(Type::Integer(0, std::cmp::max(bits1, bits2)));
			}
		} else if let Type::Integer(_, bits2) = type2 {
			if let Type::Integer(_, bits1) = type1 {
				return Ok(Type::Integer(0, std::cmp::max(bits1, bits2)));
			}
		}

		if type1.zero_pointer_number() == type2.zero_pointer_number() {
			Ok(type1.zero_pointer_number())
		} else if type1.zero_pointer_number() == Type::Float(0) || type2.zero_pointer_number() == Type::Float(0) {
			Ok(Type::Float(0))
		} else {
			Err(MathError::IncompatibleTypes(type1, type2))
		}
	}

	pub fn create_immediate_integer(&self, number: u64) -> Value {
		unsafe {
			Value {
				type_enum: Type::Integer(0, 64),
				value: LLVMConstInt(self.to_llvm_type(Type::Integer(0, 64)), number, 0),
			}
		}
	}

	pub fn add_global_integer(&mut self, block: Block, number: u64) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let global = LLVMAddGlobal(
				self.get_module(),
				self.to_llvm_type(Type::Integer(0, 64)),
				self.string_table.to_llvm_string(&format!("c{}", number))
			);

			LLVMSetInitializer(global, LLVMConstInt(self.to_llvm_type(Type::Integer(0, 64)), number, 0));

			Value {
				type_enum: Type::Integer(1, 64),
				value: global,
			}
		}
	}

	pub fn create_immediate_float(&self, number: f64) -> Value {
		unsafe {
			Value {
				type_enum: Type::Float(0),
				value: LLVMConstReal(self.to_llvm_type(Type::Float(0)), number),
			}
		}
	}

	pub fn add_global_float(&mut self, block: Block, number: f64) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let global = LLVMAddGlobal(
				self.get_module(),
				self.to_llvm_type(Type::Float(0)),
				self.string_table.to_llvm_string(&format!("cf{}", number as u64))
			);

			LLVMSetInitializer(global, LLVMConstReal(self.to_llvm_type(Type::Float(0)), number));

			Value {
				type_enum: Type::Float(1),
				value: global,
			}
		}
	}

	pub fn upgrade_type(&mut self, type_enum: Type) -> Type {
		match type_enum {
			Type::Float(pointer_number) => Type::Float(pointer_number + 1),
			Type::Integer(pointer_number, bits) => Type::Integer(pointer_number + 1, bits),
			_ => todo!(),
		}
	}

	pub fn downgrade_type(&mut self, type_enum: Type) -> Type {
		match type_enum {
			Type::Float(pointer_number) => Type::Float(pointer_number - 1),
			Type::Integer(pointer_number, bits) => Type::Integer(pointer_number - 1, bits),
			_ => todo!(),
		}
	}

	// resolves pointers into temporary variables
	pub fn resolve_value(&mut self, block: Block, value: Value) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			match value.type_enum { // TODO modularize
				Type::Float(pointer_number) => {
					if pointer_number == 0 {
						value
					} else {
						Value {
							type_enum: Type::Float(0),
							value: LLVMBuildLoad2(
								builder.get_builder(),
								self.to_llvm_type(Type::Float(0)),
								value.value,
								self.string_table.to_llvm_string("fdereference")
							),
						}
					}
				},
				Type::Integer(pointer_number, bits) => {
					if pointer_number == 0 {
						value
					} else {
						Value {
							type_enum: Type::Integer(0, bits),
							value: LLVMBuildLoad2(
								builder.get_builder(),
								self.to_llvm_type(Type::Integer(0, bits)),
								value.value,
								self.string_table.to_llvm_string("idereference")
							),
						}
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

			match value.type_enum {
				Type::Float(pointer_number) => match result_type {
					Type::Integer(_, bits) => Value {
						type_enum: Type::Integer(pointer_number, bits),
						value: LLVMBuildFPToUI(
							builder.get_builder(),
							value.value,
							self.to_llvm_type(Type::Integer(0, bits)),
							self.string_table.to_llvm_string("icast")
						),
					},
					_ => todo!(),
				},
				Type::Integer(pointer_number, bits1) => match result_type {
					Type::Integer(_, bits2) => {
						if bits1 > bits2 { // no bit truncating yet
							todo!();
						}

						Value {
							type_enum: Type::Integer(pointer_number, bits2),
							value: LLVMBuildIntCast2(
								builder.get_builder(),
								value.value,
								self.to_llvm_type(Type::Integer(pointer_number, bits2)), // upgrade bits, retain value pointer number
								0,
								self.string_table.to_llvm_string("iupgrade")
							),
						}
					},
					Type::Float(_) =>	Value {
						type_enum: Type::Float(pointer_number),
						value: LLVMBuildUIToFP(
							builder.get_builder(),
							value.value,
							self.to_llvm_type(Type::Float(0)),
							self.string_table.to_llvm_string("fcast")
						),
					},
					_ => todo!(),
				}
				_ => todo!(),
			}
		}
	}
}
