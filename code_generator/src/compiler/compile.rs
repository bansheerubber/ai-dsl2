use ai_dsl2_compiler::{ Block, Module, Value };
use pest::iterators::{ Pair, Pairs };

use crate::compiler::{ Function, Math, VariableDeclaration };
use crate::parser::{ self, DSLParser };

pub struct CompilationContext<'a> {
	pub current_block: Option<Block>,
	pub module: Module,
	pub parser: DSLParser<'a>,
}

pub fn compile_pair(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Option<Value> {
	match pair.as_rule() {
		parser::Rule::function => {
			Function::compile(context, pair);
			return None;
		},
		parser::Rule::math => {
			Some(Math::compile(context, pair))
		},
		parser::Rule::return_statement => {
			println!("return statement not implemented");
			return None;
		},
		parser::Rule::variable_declaration => {
			VariableDeclaration::compile(context, pair);
			return None;
		},
		parser::Rule::EOI => {
			println!("end of input");
			return None;
		}
		rule => todo!("{:?} not implemented", rule),
	}
}

pub fn compile_pairs(context: &mut CompilationContext, pairs: Pairs<'_, parser::Rule>) {
	for pair in pairs {
		compile_pair(context, pair);
	}
}
