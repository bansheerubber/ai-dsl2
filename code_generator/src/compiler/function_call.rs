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

		let mut argument_values = Vec::new();

		let arguments = pairs.next();
		if let Some(arguments) = arguments {
			// compile the arguments
			for pair in arguments.into_inner() {
				let Some(value) = compile_pair(context, pair) else {
					unreachable!();
				};

				argument_values.push(value);
			}
		}

		let key = FunctionKey::new(&function_name);
		context.module.add_function_call(
			context.current_block.unwrap(), &key, &mut argument_values
		)
	}
}
