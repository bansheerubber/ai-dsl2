use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair, compile_pairs };
use crate::parser;

pub struct ForLoop;

impl ForLoop {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut pairs = pair.into_inner();

		// compile the variable declaration
		compile_pair(context, pairs.next().unwrap());

		let conditional_block = context.module.new_block("for_condition", &context.current_function.as_ref().unwrap());
		let body_block = context.module.new_block("for_body", &context.current_function.as_ref().unwrap());
		let increment_block = context.module.new_block("for_increment", &context.current_function.as_ref().unwrap());

		// branch into the conditional block
		context.module.add_branch(context.current_block.unwrap(), conditional_block);

		// split the block
		context.module.split_block_in_place(context.current_block.as_mut().unwrap());
		let continued_block = context.current_block.unwrap();

		// compile the conditional expression
		context.current_block = Some(conditional_block);
		let conditional = compile_pair(context, pairs.next().unwrap()).unwrap();
		context.module.add_branch_if_true(conditional_block, conditional, body_block, continued_block);

		// compile the increment expression
		context.current_block = Some(increment_block);
		compile_pair(context, pairs.next().unwrap());
		context.module.add_branch(increment_block, conditional_block);

		// compile the body
		context.current_block = Some(body_block);
		compile_pairs(context, pairs.next().unwrap().into_inner());

		// jump into increment
		context.module.add_branch(context.current_block.unwrap(), increment_block);

		context.current_block = Some(continued_block);
	}
}
