use ai_dsl2_compiler::Type;
use pest::iterators::Pair;

use crate::compiler::{ CompilationContext, compile_pair };
use crate::parser;
use crate::types::convert_type_name;

pub struct VariableDeclaration;

// metadata we generate by performing static analysis upon function compilation
pub struct VariableMetadata {
	pub mutable: bool,
	pub name: String,
}

impl VariableDeclaration {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut variable_name = "";
		let mut variable_type = "";

		if context.current_block.is_none() { // compile a global variable declaration
			let pairs = pair.into_inner();
			for pair in pairs.clone() {
				if pair.as_rule() == parser::Rule::token {
					variable_name = pair.as_str();
				} else if pair.as_rule() == parser::Rule::type_token {
					variable_type = pair.as_str();
				}
			}

			context.module.add_global_variable(
				variable_name, convert_type_name(&context.module, variable_type)
			);
		} else { // compile a local variable declaration
			let pairs = pair.into_inner();
			for pair in pairs.clone() {
				if pair.as_rule() == parser::Rule::token {
					variable_name = pair.as_str();
				} else if pair.as_rule() == parser::Rule::type_token {
					variable_type = pair.as_str();
				}
			}

			let variable_type = convert_type_name(&context.module, variable_type);
			let variable = context.module.add_mutable_variable(
				context.current_block.unwrap(), variable_name, variable_type
			);

			let value = compile_pair(context, pairs.last().unwrap()).unwrap();
			context.module.add_store(context.current_block.unwrap(), variable, value).unwrap();
		}
	}
}