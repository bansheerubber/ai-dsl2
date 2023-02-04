use ai_dsl2_compiler::Value;

use crate::compiler::CompilationContext;

pub struct LearnedValue;

impl LearnedValue {
	pub fn compile(context: &mut CompilationContext) -> Value {
		let function_name = context.module.function_table.get_function(&context.current_function.as_ref().unwrap())
			.unwrap().name.clone();

		let mut args = vec![
			context.module.create_global_string(context.current_block.unwrap(), &function_name),
			context.module.create_immediate_integer(0),
			context.module.create_immediate_integer(0),
		];
		context.module.add_function_call(
			context.current_block.unwrap(), &context.placeholder_evaluation_int, &mut args
		)
	}
}
