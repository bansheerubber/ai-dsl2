use ai_dsl2_compiler::{ Block, FunctionKey, Module, Type, Value, };
use pest::iterators::{ Pair, Pairs, };

use crate::compiler::{
	ForLoop,
	Function,
	FunctionCall,
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
	pub airt_handle_function_call: FunctionKey,
	pub airt_finish_function_call: FunctionKey,
	pub current_block: Option<Block>,
	pub current_function: Option<FunctionKey>,
	pub module: Module,
	pub parser: DSLParser<'a>,
	pub placeholder_evaluation_float: FunctionKey,
	pub placeholder_evaluation_int: FunctionKey,
	// used to determine if we should insert an airt function call before every return in a function. TODO rethink how
	// this is implemented?
	pub prediction_index: Option<Value>,
}

impl CompilationContext<'_> {
	pub fn new<'a>(input_filename: &str, state: &'a mut parser::ParserState<'a>) -> CompilationContext<'a> {
		let mut module = Module::new("main");

		module.create_extern_function(
			"_airt_print_float", &vec![Type::Float(0)], Type::Void
		);

		module.create_extern_function(
			"_airt_print_int", &vec![Type::Integer(0, 64)], Type::Void
		);

		module.create_extern_function(
			"_airt_random_float", &vec![Type::Float(0), Type::Float(0)], Type::Float(0)
		);

		module.create_extern_function(
			"_airt_log_simulation", &vec![Type::Float(0), Type::Float(0)], Type::Void
		);

		CompilationContext {
			airt_handle_function_call: module.create_extern_function(
				"airt_handle_function_call", &vec![Type::CString(0), Type::Float(1)], Type::Integer(0, 64)
			),
			airt_finish_function_call: module.create_extern_function(
				"airt_finish_function_call", &vec![Type::CString(0), Type::Integer(0, 64)], Type::Void
			),
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
			prediction_index: None,
		}
	}

	pub fn add_finish_function_call(&mut self) {
		if let Some(prediction_index) = self.prediction_index {
			let function_name = &self.current_function.as_ref().unwrap().name;

			// TODO cache the name
			let allocated_name = self.module.create_global_string(self.current_block.unwrap(), function_name);

			self.module.add_function_call(
				self.current_block.unwrap(),
				&self.airt_finish_function_call,
				&mut [allocated_name, prediction_index]
			);
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
		parser::Rule::function_call => {
			return Some(FunctionCall::compile(context, pair));
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
