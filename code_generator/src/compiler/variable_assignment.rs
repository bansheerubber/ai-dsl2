use ai_dsl2_compiler::Value;
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair };
use crate::parser;

pub struct VariableAssignment;

impl VariableAssignment {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let property_assignment = pair.as_rule() == parser::Rule::property_assignment;

		let mut pairs = pair.into_inner();

		let variable_name = pairs.next().unwrap().as_str();
		let variable = context.module.get_variable(context.current_block.unwrap(), variable_name).unwrap();

		let value = compile_pair(context, pairs.last().unwrap()).unwrap();
		context.module.add_store(
			context.current_block.unwrap(),
			variable,
			value
		).unwrap()
	}
}