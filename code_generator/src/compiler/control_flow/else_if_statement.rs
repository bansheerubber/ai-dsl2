use ai_dsl2_compiler::{ Block, Value, };
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair, compile_pairs };
use crate::parser;

use super::ControlFlow;

pub struct ElseIfStatement {
	body_block: Block,
	conditional_block: Block,
	conditional_value: Value,
}

impl ElseIfStatement {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> ElseIfStatement {
		let mut pairs = pair.into_inner();

		let conditional_block = context.module.new_block("else_if_condition", &context.current_function.as_ref().unwrap());
		let body_block = context.module.new_block("else_if_body", &context.current_function.as_ref().unwrap());

		context.current_block = Some(conditional_block);
		let conditional_value = compile_pair(context, pairs.next().unwrap()).unwrap(); // compile conditional

		context.current_block = Some(body_block);
		compile_pairs(context, pairs.next().unwrap().into_inner());

		return ElseIfStatement {
			body_block,
			conditional_block,
			conditional_value,
		}
	}
}

impl ControlFlow for ElseIfStatement {
	fn get_start_block(&self) -> Block {
		self.body_block
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
