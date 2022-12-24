use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair, compile_pairs };
use crate::parser;

pub struct WhileLoop;

impl WhileLoop {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut pairs = pair.into_inner();

		let conditional_block = context.module.new_block("while_condition", &context.current_function.as_ref().unwrap());
		let body_block = context.module.new_block("while_body", &context.current_function.as_ref().unwrap());

		// branch into the conditional block
		context.module.add_branch(context.current_block.unwrap(), conditional_block);

		// split the block
		context.module.split_block_in_place(context.current_block.as_mut().unwrap());
		let continued_block = context.current_block.unwrap();

		// compile the conditional expression
		context.current_block = Some(conditional_block);
		let conditional = compile_pair(context, pairs.next().unwrap()).unwrap();
		context.module.add_branch_if_true(conditional_block, conditional, body_block, continued_block);

		// compile the body
		context.current_block = Some(body_block);
		compile_pairs(context, pairs.next().unwrap().into_inner());

		// jump into conditional, only if another terminal hasn't been assigned
		let function = context.module.function_table.get_function(&context.current_function.as_ref().unwrap()).unwrap();
		if function.has_default_block_terminal(context.current_block.unwrap()) {
			context.module.add_branch(context.current_block.unwrap(), conditional_block);
		}

		context.current_block = Some(continued_block);
	}
}
