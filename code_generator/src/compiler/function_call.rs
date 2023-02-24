use ai_dsl2_compiler::{ FunctionKey, Value, };
use pest::iterators::Pair;

use crate::compiler::CompilationContext;
use crate::parser;

use super::compile_pair;

pub struct FunctionCall;

impl FunctionCall {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let mut pairs = pair.into_inner();

		let function_name = context.module.transform_function_name(pairs.next().unwrap().as_str());

		// compile the arguments
		let mut args = Vec::new();
		for pair in pairs.clone().next().unwrap().into_inner() {
			let Some(value) = compile_pair(context, pair) else {
				unreachable!();
			};

			args.push(value);
		}

		let key = FunctionKey::new(&function_name);
		context.module.add_function_call(
			context.current_block.unwrap(), &key, &mut args
		)
	}
}
