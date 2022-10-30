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
	last_value: Option<Value>,
	operation: LogicOperation,
	short: Block,
}

impl LogicBlock {
	pub fn get_current_block(&self) -> Block {
		self.blocks[self.block_index]
	}

	pub fn get_next_block(&self) -> Option<Block> {
		if self.block_index + 1 >= self.blocks.len() {
			None
		} else {
			Some(self.blocks[self.block_index + 1])
		}
	}
}

impl Module {
	pub fn new_logic_block(&mut self, parent: Block, operation: LogicOperation, count: usize) -> LogicBlock {
		let logic = unsafe {
			let function = LLVMGetBasicBlockParent(parent.get_block());

			let mut blocks = vec![];
			for _ in 0..count {
				blocks.push(self.new_block("logicstep", function));
			}

			let short = self.new_block("short", function);
			let end = self.new_block("end", function);

			LogicBlock {
				blocks,
				block_index: 0,
				end,
				last_value: None,
				operation,
				short,
			}
		};

		// build the short circuit block
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(logic.short);
			LLVMBuildBr(builder.get_builder(), logic.end.get_block());
		}

		return logic;
	}

	pub fn add_logic(&mut self, logic: LogicBlock, value: Value) -> Result<LogicBlock, MathError> {
		let mut logic = logic;

		// keep track of last_value so we can use the result of the logic operation in the end block
		let compare = self.add_equals(
			logic.get_current_block(),
			value,
			self.create_immediate_integer(
				if let LogicOperation::And = logic.operation {
					0
				} else {
					1
				}
			)
		)?;

		logic.last_value = Some(value);

		let builder = Builder::new();
		builder.seek_to_end(logic.get_current_block());

		if let Some(next_block) = logic.get_next_block() {
			self.add_branch_if_true(logic.get_current_block(), compare, logic.short, next_block);
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

			let mut incoming_values = vec![
				LLVMConstInt(self.to_llvm_type(Type::Integer(0, 64)), 0, 0), // TODO handle type conversion
				logic.last_value.unwrap().value // TODO handle type conversion
			];

			let mut incoming_blocks = vec![logic.short.get_block(), logic.blocks[logic.blocks.len() - 1].get_block()];

			LLVMAddIncoming(phi, incoming_values.as_mut_ptr(), incoming_blocks.as_mut_ptr(), 2); // TODO is this gonna crash?

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
