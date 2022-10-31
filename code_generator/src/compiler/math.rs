use ai_dsl2_compiler::{ Block, LogicOperation, Value };
use pest::iterators::Pair;

use crate::compiler::CompilationContext;
use crate::parser::{ self, configure_pratt };

#[derive(Debug)]
enum MathIR {
	Constant {
		kind: parser::Rule,
		value: String,
	},
	LogicOperation {
		operation: parser::Rule,
		values: Vec<Box<MathIR>>,
	},
	Operation {
		lhs: Box<MathIR>,
		operation: parser::Rule,
		rhs: Box<MathIR>,
	},
	UnaryOperation {
		operation: parser::Rule,
		value: Box<MathIR>,
	},
}

impl MathIR {
	pub fn get_operation(&self) -> Option<parser::Rule> {
		match *self {
			MathIR::Constant { kind: _, value: _, } => None,
			MathIR::LogicOperation { operation, values: _, } => Some(operation),
			MathIR::Operation { lhs: _, operation, rhs: _ } => Some(operation),
			MathIR::UnaryOperation { operation, value: _ } => Some(operation),
		}
	}
}

pub struct Math;

impl Math {
	pub fn compile(context: &mut CompilationContext, pair: Pair<parser::Rule>) -> Value {
		let math_ir = Math::_compile(pair);

		Math::preorder(context, math_ir).0
	}

	fn preorder(context: &mut CompilationContext, node: Box<MathIR>) -> (Value, Option<(Block, Block)>) {
		match *node {
			MathIR::Constant {
				kind,
				value,
			} => match kind {
				parser::Rule::float => (context.module.create_immediate_float(value.parse::<f64>().unwrap()), None),
				parser::Rule::integer => (context.module.create_immediate_integer(value.parse::<u64>().unwrap()), None),
				_ => unreachable!(),
			},
			MathIR::LogicOperation {
				operation,
				values,
			} => {
				let mut logic = context.module.new_logic_block(
					context.current_block.unwrap(),
					if operation == parser::Rule::logical_and {
						LogicOperation::And
					} else {
						LogicOperation::Or
					},
					values.len()
				);

				// add terminating instruction
				context.module.add_branch(context.current_block.unwrap(), logic.get_current_block());

				for value in values {
					context.current_block = Some(logic.get_current_block());

					let (value, blocks) = Math::preorder(context, value);

					if let Some((first_block, end_block)) = blocks {
						context.module.split_block_in_place(logic.get_current_block_mut());
						context.current_block = Some(logic.get_current_block());
						context.module.add_branch(end_block, context.current_block.unwrap());
					}

					logic = context.module.add_logic(logic, value).unwrap();
				}

				let first = logic.get_first_block();

				let (value, end_block) = context.module.commit_logic_block(logic).unwrap();
				context.current_block = Some(end_block);

				(value, Some((first, end_block)))
			},
			MathIR::Operation {
				lhs,
				operation,
				rhs,
			} => {
				let (lhs, _) = Math::preorder(context, lhs);
				let (rhs, _) = Math::preorder(context, rhs);

				let current_block = context.current_block.unwrap();
				(
					match operation {
						parser::Rule::addition => context.module.add_addition(current_block, lhs, rhs).unwrap(),
						parser::Rule::bitwise_and => context.module.add_bitwise_and(current_block, lhs, rhs).unwrap(),
						parser::Rule::bitwise_or => context.module.add_bitwise_or(current_block, lhs, rhs).unwrap(),
						parser::Rule::bitwise_xor => context.module.add_bitwise_xor(current_block, lhs, rhs).unwrap(),
						parser::Rule::division => context.module.add_division(current_block, lhs, rhs).unwrap(),
						parser::Rule::equals => context.module.add_equals(current_block, lhs, rhs).unwrap(),
						parser::Rule::greater_than => context.module.add_greater_than(current_block, lhs, rhs).unwrap(),
						parser::Rule::greater_than_equal_to => context.module.add_greater_than_equal_to(current_block, lhs, rhs).unwrap(),
						parser::Rule::less_than => context.module.add_less_than(current_block, lhs, rhs).unwrap(),
						parser::Rule::less_than_equal_to => context.module.add_less_than_equal_to(current_block, lhs, rhs).unwrap(),
						parser::Rule::multiplication => context.module.add_multiplication(current_block, lhs, rhs).unwrap(),
						parser::Rule::not_equals => context.module.add_not_equals(current_block, lhs, rhs).unwrap(),
						parser::Rule::subtraction => context.module.add_subtraction(current_block, lhs, rhs).unwrap(),
						rule => todo!("{:?} not implemented", rule),
					},
					None
				)
			},
			MathIR::UnaryOperation {
				operation,
				value,
			} => {
				let (value, _) = Math::preorder(context, value);

				let current_block = context.current_block.unwrap();
				(
					match operation {
						parser::Rule::bitwise_not => context.module.add_bitwise_not(current_block, value).unwrap(),
						parser::Rule::logical_not => context.module.add_logical_not(current_block, value).unwrap(),
						parser::Rule::negative => context.module.add_negate(current_block, value).unwrap(),
						rule => todo!("{:?} not implemented", rule),
					},
					None
				)
			},
		}
	}

	// vectorize operations into intermediate representation that we parse later
	fn _compile(pair: Pair<parser::Rule>) -> Box<MathIR> {
		let value = configure_pratt()
			.map_primary(|primary| match primary.as_rule() {
				parser::Rule::math => Math::_compile(primary),
				kind => Box::new(MathIR::Constant {
					kind,
					value: String::from(primary.as_str()),
				}),
			})
			.map_prefix(|op, value| Box::new(MathIR::UnaryOperation {
					operation: op.as_rule(),
					value,
				})
			)
			.map_infix(|mut lhs, op, rhs| match op.as_rule() {
				parser::Rule::logical_and | parser::Rule::logical_or => {
					if let MathIR::LogicOperation { operation, values } = lhs.as_mut() {
						if operation == &op.as_rule() {
							values.push(rhs);
							return lhs;
						}
					}

					Box::new(MathIR::LogicOperation {
						operation: op.as_rule(),
						values: vec![lhs, rhs],
					})
				},
				operation => Box::new(MathIR::Operation {
					lhs,
					operation,
					rhs,
				}),
			})
			.parse(pair.into_inner());

		return value;
	}
}
