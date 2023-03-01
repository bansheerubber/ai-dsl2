use ai_dsl2_compiler::{ Block, Value, };

pub trait ControlFlow {
	fn get_start_block(&self) -> Block;
	fn get_body_block(&self) -> Block;
	fn get_conditional_block(&self) -> Option<Block>;
	fn get_conditional_value(&self) -> Option<Value>;
}
