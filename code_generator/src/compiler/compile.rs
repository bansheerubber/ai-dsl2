use ai_dsl2_compiler::{ Block, Module };
use pest::iterators::{ Pair, Pairs };

use crate::compiler::{ Function, VariableDeclaration };
use crate::parser::{ self, DSLParser };

pub struct CompilationContext<'a> {
	pub current_block: Option<Block>,
	pub module: Module,
	pub parser: DSLParser<'a>,
}

pub fn compile_pair(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
	match pair.as_rule() {
		parser::Rule::function => {
			Function::compile(context, pair);
		},
		parser::Rule::math => {
			println!("math not implemented");
		},
		parser::Rule::return_statement => {
			println!("return statement not implemented");
		},
		parser::Rule::variable_declaration => {
			VariableDeclaration::compile(context, pair);
		},
		parser::Rule::EOI => {
			println!("end of input");
		}
		rule => todo!("{:?} not implemented", rule),
	}
}

pub fn compile_pairs(context: &mut CompilationContext, pairs: Pairs<'_, parser::Rule>) {
	for pair in pairs {
		compile_pair(context, pair);
	}
}
