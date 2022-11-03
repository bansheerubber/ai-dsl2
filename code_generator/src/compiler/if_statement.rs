use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair, compile_pairs };
use crate::parser;

pub struct IfStatement;

impl IfStatement {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut pairs = pair.into_inner();
		let conditional = compile_pair(context, pairs.next().unwrap()).unwrap(); // compile conditional

		let if_true_block = context.module.new_block("iftrue", &context.current_function.as_ref().unwrap());
		let if_false_block = context.module.new_block("iffalse", &context.current_function.as_ref().unwrap());

		context.module.add_branch_if_true(context.current_block.unwrap(), conditional, if_true_block, if_false_block);

		context.current_block = Some(if_true_block);
		compile_pairs(context, pairs.next().unwrap().into_inner());

		context.current_block = Some(if_false_block);
	}
}
