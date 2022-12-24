use ai_dsl2_compiler::Value;
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair };
use crate::parser;

pub struct Return {
	value: Value,
}

impl Return {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Return {
		let pairs = pair.into_inner();
		let value = compile_pair(context, pairs.last().unwrap()).unwrap();

		context.module.add_return(context.current_block.unwrap(), value);
		println!("hey there");

		Return {
			value,
		}
	}
}
