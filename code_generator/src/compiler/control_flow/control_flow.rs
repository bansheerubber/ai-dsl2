use ai_dsl2_compiler::{ Block, Value, };
use pest::iterators::Pair;

use crate::compiler::CompilationContext;
use crate::parser;

pub trait ControlFlow {
	fn get_body_block(&self) -> Block;
	fn get_conditional_block(&self) -> Option<Block>;
	fn get_conditional_value(&self) -> Option<Value>;
}
