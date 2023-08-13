use ai_dsl2_compiler::Value;
use pest::iterators::Pair;

use crate::compiler::CompilationContext;
use crate::parser;

pub struct NewStruct;

impl NewStruct {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let mut pairs = pair.into_inner();

		let struct_name = pairs.next().unwrap().as_str();

		context.module.add_struct_malloc(
			context.current_block.unwrap(),
			struct_name,
		)
	}
}
