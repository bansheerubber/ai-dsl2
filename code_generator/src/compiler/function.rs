use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pairs };
use crate::parser;
use crate::types::convert_type_name;

pub struct Function;

impl Function {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut name = "";
		let mut return_type = "";

		let pairs = pair.into_inner();
		for pair in pairs.clone() {
			if pair.as_rule() == parser::Rule::token {
				name = pair.as_str();
			} else if pair.as_rule() == parser::Rule::type_token {
				return_type = pair.as_str();
			} else if pair.as_rule() == parser::Rule::function_body {
				break;
			}
		}

		context.module.create_function(name, &vec![], convert_type_name(return_type));
		let block = context.module.new_block_in_function(name, name);
		context.current_block = Some(block);

		compile_pairs(context, pairs.last().unwrap().into_inner());

		context.module.add_return_void(context.current_block.unwrap());
	}
}
