use crate::compiler::CompilationContext;

pub struct VoidReturn;

impl VoidReturn {
	pub fn compile(context: &mut CompilationContext) -> VoidReturn {
		context.add_finish_function_call();
		context.module.add_return_void(context.current_block.unwrap());
		VoidReturn {}
	}
}
