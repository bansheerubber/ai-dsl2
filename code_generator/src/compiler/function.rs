use ai_dsl2_compiler::Type;
use pest::iterators::{Pair, Pairs};

use crate::compiler::{ CompilationContext, compile_pairs };
use crate::parser;
use crate::types::convert_type_name;

pub struct Function;

impl Function {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut name = "";
		let mut return_type = "";

		let mut argument_names = Vec::new();
		let mut argument_types = Vec::new();
		let pairs = pair.into_inner();
		for pair in pairs.clone() {
			if pair.as_rule() == parser::Rule::token {
				name = pair.as_str();
			} else if pair.as_rule() == parser::Rule::type_token {
				return_type = pair.as_str();
			} else if pair.as_rule() == parser::Rule::function_body {
				break;
			} else if pair.as_rule() == parser::Rule::function_declaration_args { // interpret arguments
				for argument_pair in pair.into_inner() {
					if argument_pair.as_rule() == parser::Rule::type_token {
						argument_types.push(convert_type_name(&context.module, argument_pair.as_str()));
					} else if argument_pair.as_rule() == parser::Rule::token {
						argument_names.push(String::from(argument_pair.as_str()));
					}
				}
			}
		}

		context.current_function = Some(context.module.create_function(
			name,
			&argument_types,
			convert_type_name(&context.module, return_type)
		));

		let block = context.module.new_block(name, &context.current_function.as_ref().unwrap());
		context.current_block = Some(block);

		let mut values = Vec::new();
		for i in 0..argument_names.len() {
			let argument_name = &argument_names[i];
			let argument_type = argument_types[i];
			let function = context.module.function_table.get_function(&context.current_function.as_ref().unwrap()).unwrap();
			let argument_value = function.get_argument(i);
			values.push(argument_value);

			context.module.add_argument(block, &argument_name, argument_type, argument_value);
		}

		let has_learned_values = Function::look_for_learned_values(
			pairs.clone().last().unwrap().into_inner()
		);

		let prediction_index = if values.len() > 0 && has_learned_values { // call `airt_handle_function_call`
			// create array of arguments
			let array = context.module.add_immutable_array(
				context.current_block.unwrap(),
				Type::Float(0),
				values.len()
			);

			for i in 0..values.len() {
				let value = values[i];
				context.module.add_store_to_array(block, array, i, value).unwrap();
			}

			// TODO cache the name
			let allocated_name = context.module.create_global_string(
				context.current_block.unwrap(), &context.module.transform_function_name(name)
			);

			Some(context.module.add_function_call(
				context.current_block.unwrap(),
				&context.airt_handle_function_call,
				&mut [allocated_name, array]
			))
		} else {
			None
		};

		context.prediction_index = prediction_index;

		compile_pairs(context, pairs.last().unwrap().into_inner());

		let function = context.module.function_table.get_function(&context.current_function.as_ref().unwrap()).unwrap();
		if function.has_default_block_terminal(context.current_block.unwrap()) {
			context.add_finish_function_call();
			context.module.add_return_void(context.current_block.unwrap());
		}

		context.current_function = None;
	}

	// looks for learned values recursively, determines whether or not we process a function in the airt
	fn look_for_learned_values(pairs: Pairs<parser::Rule>) -> bool {
		for pair in pairs {
			if pair.as_rule() == parser::Rule::learned_value {
				return true;
			}

			let result = Function::look_for_learned_values(pair.clone().into_inner());
			if result {
				return true;
			}
		}

		return false;
	}
}
