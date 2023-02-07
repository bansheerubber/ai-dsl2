use ai_dsl2_compiler::{ Block, FunctionKey, Module, Type, Value, };
use pest::iterators::{ Pair, Pairs, };

use crate::compiler::{
	ForLoop,
	Function,
	IfStatement,
	LearnedValue,
	Math,
	Return,
	VariableAssignment,
	VariableDeclaration,
	VoidReturn,
	WhileLoop,
};
use crate::parser::{ self, DSLParser };

pub struct CompilationContext<'a> {
	pub current_block: Option<Block>,
	pub current_function: Option<FunctionKey>,
	pub module: Module,
	pub parser: DSLParser<'a>,
	pub placeholder_evaluation_float: FunctionKey,
	pub placeholder_evaluation_int: FunctionKey,
}

impl CompilationContext<'_> {
	pub fn new<'a>(input_filename: &str, state: &'a mut parser::ParserState<'a>) -> CompilationContext<'a> {
		let mut module = Module::new("main");

		CompilationContext {
			placeholder_evaluation_float: module.create_extern_function(
				"airt_predict_float", &vec![Type::CString(0), Type::Integer(0, 64), Type::Integer(0, 64)], Type::Float(0)
			),
			placeholder_evaluation_int: module.create_extern_function(
				"airt_predict_int", &vec![Type::CString(0), Type::Integer(0, 64), Type::Integer(0, 64)], Type::Integer(0, 64)
			),

			current_block: None,
			current_function: None,
			module,
			parser: state.parse_file(&input_filename),
		}
	}
}

pub fn compile_pair(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Option<Value> {
	match pair.as_rule() {
		parser::Rule::for_loop => {
			ForLoop::compile(context, pair);
			return None;
		},
		parser::Rule::function => {
			Function::compile(context, pair);
			return None;
		},
		parser::Rule::if_statement => {
			IfStatement::compile(context, pair);
			return None;
		},
		parser::Rule::learned_value => {
			return Some(LearnedValue::compile(context));
		},
		parser::Rule::math => {
			Some(Math::compile(context, pair))
		},
		parser::Rule::return_statement => {
			Return::compile(context, pair);
			return None;
		},
		parser::Rule::variable_assignment => {
			Some(VariableAssignment::compile(context, pair))
		},
		parser::Rule::variable_declaration => {
			VariableDeclaration::compile(context, pair);
			return None;
		},
		parser::Rule::void_return_statement => {
			VoidReturn::compile(context);
			return None;
		},
		parser::Rule::while_loop => {
			WhileLoop::compile(context, pair);
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
