use ai_dsl2_compiler::Module;
use pest::iterators::{ Pair, Pairs };

use crate::compiler::Function;
use crate::parser;

pub fn compile_pair(module: &mut Module, pair: Pair<parser::Rule>) {
	match pair.as_rule() {
		parser::Rule::function => {
			Function::compile(module, pair);
		},
		parser::Rule::return_statement => {
			println!("return statement not implemented");
		},
		parser::Rule::variable_declaration => {
			println!("variable declaration not implemented");
		},
		parser::Rule::EOI => {
			println!("end of input");
		}
		_ => todo!(),
	}
}

pub fn compile_pairs(module: &mut Module, pairs: Pairs<'_, parser::Rule>) {
	for pair in pairs {
		compile_pair(module, pair);
	}
}
