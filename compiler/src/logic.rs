use llvm_sys::core::*;

use crate::{ Block, Builder, MathError, Module, Type, Value };

#[derive(Debug)]
pub enum LogicOperation { // determines how we short
	And,
	Or
}

#[derive(Debug)]
pub struct LogicBlock {
	// the block that we append compares + conditional branch instructions to
	blocks: Vec<Block>,
	block_index: usize,
	end: Block,
	operation: LogicOperation,
	values: Vec<Value>,
}

impl LogicBlock {
	pub fn get_current_block(&self) -> Block {
		self.blocks[self.block_index]
	}

	pub fn get_current_block_ref(&self) -> &Block {
		&self.blocks[self.block_index]
	}

	pub fn get_current_block_mut(&mut self) -> &mut Block {
		&mut self.blocks[self.block_index]
	}

	pub fn get_next_block(&self) -> Option<Block> {
		if self.block_index + 1 >= self.blocks.len() {
			None
		} else {
			Some(self.blocks[self.block_index + 1])
		}
	}

	pub fn get_first_block(&self) -> Block {
		self.blocks[0]
	}

	pub fn get_end(&self) -> Block {
		self.end
	}
}

impl Module {
	pub fn new_logic_block(&mut self, parent: Block, operation: LogicOperation, count: usize) -> LogicBlock {
		let function = self.function_table.get_function_by_ref(parent.get_parent()).unwrap();

		let mut blocks = vec![];
		for _ in 0..count {
			blocks.push(
				self.new_block(&format!("logicstep_{:?}", operation), &function)
			);
		}

		let end = self.new_block("end", &function);

		LogicBlock {
			blocks,
			block_index: 0,
			end,
			operation,
			values: vec![],
		}
	}

	pub fn add_logic(&mut self, logic: LogicBlock, value: Value) -> Result<LogicBlock, MathError> {
		let mut logic = logic;

		let compare = if let LogicOperation::And = logic.operation {
			self.add_equals(
				logic.get_current_block(),
				value,
				self.create_immediate_integer(0)
			)?
		} else {
			self.add_not_equals(
				logic.get_current_block(),
				value,
				self.create_immediate_integer(0)
			)?
		};

		// keep track of values so we can use the result of the logic operations in the end block
		logic.values.push(value);

		let builder = Builder::new();
		builder.seek_to_end(logic.get_current_block());

		if let Some(next_block) = logic.get_next_block() {
			self.add_branch_if_true(logic.get_current_block(), compare, logic.end, next_block);
		} else {
			self.add_branch(logic.get_current_block(), logic.end);
		}

		logic.block_index += 1;

		return Ok(logic);
	}

	pub fn commit_logic_block(&mut self, logic: LogicBlock) -> Result<(Value, Block), MathError> {
		// build the end block
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(logic.end);

			let phi = LLVMBuildPhi(
				builder.get_builder(),
				self.to_llvm_type(Type::Integer(0, 64)), // TODO handle type conversion
				self.string_table.to_llvm_string("phiend")
			);

			let mut incoming_values = Vec::new();
			for value in logic.values {
				incoming_values.push(value.value); // TODO handle type conversion
			}

			let mut incoming_blocks = Vec::new();
			for block in logic.blocks {
				incoming_blocks.push(block.get_block());
			}

			LLVMAddIncoming(phi, incoming_values.as_mut_ptr(), incoming_blocks.as_mut_ptr(), incoming_values.len() as u32);

			Ok((
				Value {
					type_enum: Type::Integer(0, 64),
					value: phi,
				},
				logic.end,
			))
		}
	}
}
