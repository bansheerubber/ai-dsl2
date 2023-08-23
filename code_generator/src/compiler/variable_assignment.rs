use ai_dsl2_compiler::Value;
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair };
use crate::parser;

pub struct VariableAssignment;

impl VariableAssignment {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let property_assignment = pair.as_rule() == parser::Rule::property_assignment;
		let mut pairs = pair.into_inner();

		if property_assignment {
			let mut property_chain = pairs.next().unwrap().into_inner();
			let mut value = context.module.get_variable(
				context.current_block.unwrap(),
				property_chain.next().unwrap().as_str()
			).unwrap();
			let assignment_value = compile_pair(context, pairs.last().unwrap()).unwrap();

			let property_chain = property_chain.collect::<Vec<Pair<parser::Rule>>>();
			let length = property_chain.len();
			for i in 0..length {
				let property_name = property_chain.get(i).unwrap().as_str();
				if i == length - 1 {
					value = context.module.add_store_to_obj(
						context.current_block.unwrap(),
						value, // use last value in the chain, should always be an object
						property_name,
						assignment_value,
					).unwrap();
				} else {
					value = context.module.get_obj_property( // TODO this hasn't been tested b/c nested objects aren't even implemented yet
						context.current_block.unwrap(),
						value, // use last valeu in the chain, should always be an object
						property_name,
					).unwrap();
				}
			}

			value
		} else {
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
}