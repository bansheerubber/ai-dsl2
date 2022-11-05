use ai_dsl2_compiler::{ Block, Value, };
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pairs };
use crate::parser;

use super::ControlFlow;

pub struct ElseStatement {
	body: Block,
}

impl ElseStatement {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> ElseStatement {
		// deconstruct `else_statement` into `if_statement_body` then get the `body` elements to compile
		let pairs = pair.into_inner().next().unwrap().into_inner();

		let body_block = context.module.new_block("else_body", &context.current_function.as_ref().unwrap());

		context.current_block = Some(body_block);
		compile_pairs(context, pairs);

		return ElseStatement {
			body: body_block,
		}
	}
}

impl ControlFlow for ElseStatement {
	fn get_body_block(&self) -> Block {
		self.body
	}

	fn get_conditional_block(&self) -> Option<Block> {
		None
	}

	fn get_conditional_value(&self) -> Option<Value> {
		None
	}
}
