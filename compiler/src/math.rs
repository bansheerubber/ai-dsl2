use llvm_sys::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::{ Block, Builder, MathError, Module, Type };

#[derive(Clone, Copy, Debug)]
pub struct Value {
	pub type_enum: Type,
	pub value: LLVMValueRef,
}

enum CompareOperation {
	Equals,
	GreaterThan,
	GreaterThanEqualTo,
	LessThan,
	LessThanEqualTo,
	NotEquals,
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

	pub fn add_division(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let lhs = self.math_resolve_value(block, lhs, Type::Float(0));
			let rhs = self.math_resolve_value(block, rhs, Type::Float(0));

			let value = LLVMBuildFDiv(
				builder.get_builder(),
				lhs.value,
				rhs.value,
				self.string_table.to_llvm_string("divtmp")
			);

			Ok(Value {
				type_enum: Type::Float(0),
				value,
			})
		}
	}

	fn get_float_compare_enum(&self, operation: CompareOperation) -> LLVMRealPredicate {
		// TODO what is ordered vs unordered?
		match operation {
			CompareOperation::Equals => LLVMRealPredicate::LLVMRealOEQ,
			CompareOperation::GreaterThan => LLVMRealPredicate::LLVMRealOGT,
			CompareOperation::GreaterThanEqualTo => LLVMRealPredicate::LLVMRealOGE,
			CompareOperation::LessThan => LLVMRealPredicate::LLVMRealOLT,
			CompareOperation::LessThanEqualTo => LLVMRealPredicate::LLVMRealOLE,
			CompareOperation::NotEquals => LLVMRealPredicate::LLVMRealONE,
		}
	}

	fn get_integer_compare_enum(&self, operation: CompareOperation) -> LLVMIntPredicate {
		// TODO unsigned int problems
		match operation {
			CompareOperation::Equals => LLVMIntPredicate::LLVMIntEQ,
			CompareOperation::GreaterThan => LLVMIntPredicate::LLVMIntUGT,
			CompareOperation::GreaterThanEqualTo => LLVMIntPredicate::LLVMIntUGE,
			CompareOperation::LessThan => LLVMIntPredicate::LLVMIntULT,
			CompareOperation::LessThanEqualTo => LLVMIntPredicate::LLVMIntULE,
			CompareOperation::NotEquals => LLVMIntPredicate::LLVMIntNE,
		}
	}

	fn add_compare(
		&mut self, block: Block, lhs: Value, rhs: Value, operation: CompareOperation
	) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let common_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;
			let lhs = self.math_resolve_value(block, lhs, common_type);
			let rhs = self.math_resolve_value(block, rhs, common_type);

			let value = Value {
				type_enum: Type::Integer(0, 1),
				value: match common_type {
					Type::Float(_) => LLVMBuildFCmp(
						builder.get_builder(),
						self.get_float_compare_enum(operation),
						lhs.value,
						rhs.value,
						self.string_table.to_llvm_string("fcmplt")
					),
					Type::Integer(_, 64) => LLVMBuildICmp(
						builder.get_builder(),
						self.get_integer_compare_enum(operation),
						lhs.value,
						rhs.value,
						self.string_table.to_llvm_string("icmplt")
					),
					_ => return Err(MathError::UnsupportedOperation)
				}
			};

			Ok(value)
		}
	}

	pub fn add_less_than(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::LessThan)
	}

	pub fn add_greater_than(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::GreaterThan)
	}

	pub fn add_less_than_equal_to(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::LessThanEqualTo)
	}

	pub fn add_greater_than_equal_to(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::GreaterThanEqualTo)
	}

	pub fn add_equals(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::Equals)
	}

	pub fn add_not_equals(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		self.add_compare(block, lhs, rhs, CompareOperation::NotEquals)
	}

	pub fn add_bitwise_and(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;

			if result_type.zero_pointer_number().zero_bits() != Type::Integer(0, 0) {
				return Err(MathError::UnsupportedOperation)
			}

			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Integer(0, _) => LLVMBuildAnd(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("and")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_bitwise_or(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;

			if result_type.zero_pointer_number().zero_bits() != Type::Integer(0, 0) {
				return Err(MathError::UnsupportedOperation)
			}

			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Integer(0, _) => LLVMBuildOr(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("ortmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_bitwise_xor(&mut self, block: Block, lhs: Value, rhs: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = self.math_type_aliasing(lhs.type_enum, rhs.type_enum)?;

			if result_type.zero_pointer_number().zero_bits() != Type::Integer(0, 0) {
				return Err(MathError::UnsupportedOperation)
			}

			let lhs = self.math_resolve_value(block, lhs, result_type);
			let rhs = self.math_resolve_value(block, rhs, result_type);

			let value = match result_type {
				Type::Integer(0, _) => LLVMBuildXor(
					builder.get_builder(),
					lhs.value,
					rhs.value,
					self.string_table.to_llvm_string("xortmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_bitwise_not(&mut self, block: Block, value: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = value.type_enum.zero_pointer_number();

			if result_type.zero_pointer_number().zero_bits() != Type::Integer(0, 0) {
				return Err(MathError::UnsupportedOperation)
			}

			let value = self.math_resolve_value(block, value, result_type);

			let value = match result_type {
				Type::Integer(0, bits) => LLVMBuildXor(
					builder.get_builder(),
					LLVMConstInt(self.to_llvm_type(Type::Integer(0, bits)), 0xFFFF_FFFF_FFFF_FFFF, 0), // input tied high
					value.value,
					self.string_table.to_llvm_string("xortmp")
				),
				_ => return Err(MathError::UnsupportedOperation)
			};

			Ok(Value {
				type_enum: result_type,
				value,
			})
		}
	}

	pub fn add_logical_not(&mut self, block: Block, value: Value) -> Result<Value, MathError> {
		match value.type_enum {
			Type::Float(_) =>
				self.add_compare(block, value, self.create_immediate_float(0.0), CompareOperation::Equals),
			Type::Integer(_, _) =>
				self.add_compare(block, value, self.create_immediate_integer(0), CompareOperation::Equals),
			_ => Err(MathError::UnsupportedOperation)
		}
	}

	pub fn add_negate(&mut self, block: Block, value: Value) -> Result<Value, MathError> {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let result_type = value.type_enum.zero_pointer_number();
			let value = self.math_resolve_value(block, value, result_type);

			let value = match result_type {
				Type::Float(0) => LLVMBuildFSub(
					builder.get_builder(),
					LLVMConstReal(self.to_llvm_type(Type::Float(0)), 0.0),
					value.value,
					self.string_table.to_llvm_string("subftmp")
				),
				Type::Integer(0, bits) => LLVMBuildSub(
					builder.get_builder(),
					LLVMConstInt(self.to_llvm_type(Type::Integer(0, bits)), 0, 0),
					value.value,
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
