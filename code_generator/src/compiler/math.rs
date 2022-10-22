use ai_dsl2_compiler::Value;
use pest::iterators::Pair;
use std::cell::RefCell;
use std::rc::Rc;

use crate::compiler::CompilationContext;
use crate::parser::{ self, configure_pratt };

pub struct Math;

impl Math {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let reference = Rc::new(RefCell::new(context));
		return Math::_compile(reference, pair);
	}

	fn _compile(context: Rc<RefCell<&mut CompilationContext>>, pair: Pair<parser::Rule>) -> Value {
		let reference1 = context.clone();
		let reference2 = context.clone();
		let reference3 = context.clone();
		let reference4 = context.clone();

		let current_block = reference4.borrow().current_block.unwrap();

		let value = configure_pratt()
			.map_primary(|primary| match primary.as_rule() {
				parser::Rule::integer => reference1.borrow().module.create_immediate_integer(primary.as_str().parse::<u64>().unwrap()),
				parser::Rule::float => reference1.borrow().module.create_immediate_float(primary.as_str().parse::<f64>().unwrap()),
				parser::Rule::math => Math::_compile(reference1.clone(), primary),
				rule => todo!("{:?} not implemented", rule),
			})
			.map_prefix(|op, value| match op.as_rule() {
				parser::Rule::bitwise_not => reference2.borrow_mut().module.add_bitwise_not(current_block, value).unwrap(),
				parser::Rule::logical_not => reference2.borrow_mut().module.add_logical_not(current_block, value).unwrap(),
				parser::Rule::negative => reference2.borrow_mut().module.add_negate(current_block, value).unwrap(),
				rule => todo!("{:?} not implemented", rule),
			})
			.map_infix(|lhs, op, rhs| match op.as_rule() {
				parser::Rule::addition => reference3.borrow_mut().module.add_addition(current_block, lhs, rhs).unwrap(),
				parser::Rule::bitwise_and => reference3.borrow_mut().module.add_bitwise_and(current_block, lhs, rhs).unwrap(),
				parser::Rule::bitwise_or => reference3.borrow_mut().module.add_bitwise_or(current_block, lhs, rhs).unwrap(),
				parser::Rule::bitwise_xor => reference3.borrow_mut().module.add_bitwise_xor(current_block, lhs, rhs).unwrap(),
				parser::Rule::division => reference3.borrow_mut().module.add_division(current_block, lhs, rhs).unwrap(),
				parser::Rule::equals => reference3.borrow_mut().module.add_equals(current_block, lhs, rhs).unwrap(),
				parser::Rule::greater_than => reference3.borrow_mut().module.add_greater_than(current_block, lhs, rhs).unwrap(),
				parser::Rule::greater_than_equal_to => reference3.borrow_mut().module.add_greater_than_equal_to(current_block, lhs, rhs).unwrap(),
				parser::Rule::less_than => reference3.borrow_mut().module.add_less_than(current_block, lhs, rhs).unwrap(),
				parser::Rule::less_than_equal_to => reference3.borrow_mut().module.add_less_than_equal_to(current_block, lhs, rhs).unwrap(),
				parser::Rule::multiplication => reference3.borrow_mut().module.add_multiplication(current_block, lhs, rhs).unwrap(),
				parser::Rule::not_equals => reference3.borrow_mut().module.add_not_equals(current_block, lhs, rhs).unwrap(),
				parser::Rule::subtraction => reference3.borrow_mut().module.add_subtraction(current_block, lhs, rhs).unwrap(),
				rule => todo!("{:?} not implemented", rule),
			})
			.parse(pair.into_inner());

		return value;
	}
}
