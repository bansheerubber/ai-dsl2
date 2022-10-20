use ai_dsl2_compiler::Value;
use pest::iterators::Pair;
use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::CompilationContext;
use crate::parser::{ self, configure_pratt };

pub struct Math;

impl Math {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		// rust is cool and all but god is it annoying sometimes
		let current_block = context.current_block.unwrap();
		let reference1 = Rc::new(RefCell::new(context));
		let reference2 = reference1.clone();

		let value = configure_pratt()
			.map_primary(|primary| match primary.as_rule() {
				parser::Rule::integer => reference1.borrow().module.create_immediate_int(primary.as_str().parse::<u64>().unwrap()),
				parser::Rule::float => reference1.borrow().module.create_immediate_float(primary.as_str().parse::<f64>().unwrap()),
				_ => todo!(),
			})
			.map_infix(|lhs, op, rhs| match op.as_rule() {
				parser::Rule::addition  => reference2.borrow_mut().module.add_addition(current_block, lhs, rhs),
				parser::Rule::multiplication  => reference2.borrow_mut().module.add_multiplication(current_block, lhs, rhs),
				parser::Rule::subtraction  => reference2.borrow_mut().module.add_subtraction(current_block, lhs, rhs),
				_ => todo!(),
			})
			.parse(pair.into_inner());

		return value;
	}
}
