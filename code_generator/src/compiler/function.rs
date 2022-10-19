use ai_dsl2_compiler::Module;
use pest::iterators::Pair;

use crate::compiler::compile_pairs;
use crate::parser;
use crate::types::convert_type_name;

pub struct Function;

impl Function {
	pub fn compile(module: &mut Module, pair: Pair<parser::Rule>) {
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

		module.create_function(name, &vec![], convert_type_name(return_type));

		compile_pairs(module, pairs.last().unwrap().into_inner());
	}
}
