use pest::iterators::Pair;

use crate::compiler::CompilationContext;
use crate::parser;
use crate::types::convert_type_name;

pub struct StructDeclaration;

impl StructDeclaration {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) {
		let mut pairs = pair.into_inner();

		let struct_name = pairs.next().unwrap().as_str();
		let struct_fields = pairs.map(|pair| {
			let mut field_pairs = pair.into_inner();

			(
				field_pairs.next().unwrap().as_str().to_string(), // property name
				convert_type_name(&context.module, field_pairs.next().unwrap().as_str()), // property type
			)
		}).collect();

		context.module.create_struct_type(struct_name, struct_fields);
	}
}
