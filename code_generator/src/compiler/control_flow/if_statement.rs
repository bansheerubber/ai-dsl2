use ai_dsl2_compiler::{ Block, Value, };
use pest::iterators::Pair;

use crate::compiler::control_flow::else_statement::ElseStatement;
use crate::compiler::{ CompilationContext, compile_pair, compile_pairs };
use crate::parser;

use super::ControlFlow;
use super::else_if_statement::ElseIfStatement;

pub struct IfStatement {
	body_block: Block,
	conditional_block: Block,
	conditional_value: Value,
	start_block: Block,
}

impl IfStatement {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> IfStatement {
		let mut pairs = pair.into_inner();

		let conditional_value = compile_pair(context, pairs.next().unwrap()).unwrap();

		// create body block, and also the block we jump to once we're done evaluating a control flow's body
		let body_block = context.module.new_block("if_body", &context.current_function.as_ref().unwrap());
		let conditional_block = context.module.split_block_in_place(context.current_block.as_mut().unwrap());
		let continued_block = context.current_block.unwrap();

		// compile the if statement body
		context.current_block = Some(body_block);
		compile_pairs(context, pairs.next().unwrap().into_inner());

		let mut chain: Vec<Box<dyn ControlFlow>> = vec![Box::new(
			IfStatement {
				body_block: context.current_block.unwrap(),
				conditional_block,
				conditional_value,
				start_block: body_block,
			}
		)];

		// try to find any else/else if pairs in the control flow chain
		while let Some(pair) = pairs.next() {
			match pair.as_rule() {
				parser::Rule::else_statement => {
					chain.push(Box::new(ElseStatement::compile(context, pair)));
				},
				parser::Rule::else_if_statement => {
					chain.push(Box::new(ElseIfStatement::compile(context, pair)));
				},
				_ => unreachable!()
			};
		}

		// stitch together the branch logic for the control flow chain
		for i in 0..chain.len() {
			let control_flow = &chain[i];
			let next = chain.get(i + 1);

			if let Some(conditional_block) = control_flow.get_conditional_block() {
				// if the next block has one, jump to its conditional block. if it doesn't have one, jump straight into its
				// body block. if there is no next block, jump to the continued block
				let jump_if_false = if let Some(next) = next {
					if let Some(next_conditional_block) = next.get_conditional_block() {
						next_conditional_block
					} else {
						next.get_body_block()
					}
				} else {
					continued_block
				};

				context.module.add_branch_if_true(
					conditional_block,
					control_flow.get_conditional_value().unwrap(),
					control_flow.get_start_block(),
					jump_if_false
				);
			}

			// add branch to each control flow's body block that jumps to a point after the control flow chain
			let function = context.module.function_table.get_function(&context.current_function.as_ref().unwrap()).unwrap();
			if function.has_default_block_terminal(control_flow.get_body_block()) {
				context.module.add_branch(control_flow.get_body_block(), continued_block);
			}

			context.module.move_block_after(continued_block, control_flow.get_body_block());
		}

		context.current_block = Some(continued_block);

		IfStatement {
			body_block,
			conditional_block,
			conditional_value,
			start_block: body_block, // TODO does this break stuff?
		}
	}
}

impl ControlFlow for IfStatement {
	fn get_start_block(&self) -> Block {
		self.start_block
	}

	fn get_body_block(&self) -> Block {
		self.body_block
	}

	fn get_conditional_block(&self) -> Option<Block> {
		Some(self.conditional_block)
	}

	fn get_conditional_value(&self) -> Option<Value> {
		Some(self.conditional_value)
	}
}
