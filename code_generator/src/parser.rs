use pest::Parser;
use pest::iterators::Pairs;
use pest::pratt_parser::{ Assoc, Op, PrattParser };
use pest_derive::Parser;

pub fn configure_pratt() -> PrattParser<Rule> {
	PrattParser::new()
		.op(Op::infix(Rule::logical_or, Assoc::Left))
		.op(Op::infix(Rule::logical_and, Assoc::Left))
		.op(Op::infix(Rule::bitwise_and, Assoc::Left) | Op::infix(Rule::bitwise_or, Assoc::Left) | Op::infix(Rule::bitwise_xor, Assoc::Left))
		.op(Op::infix(Rule::equals, Assoc::Left) | Op::infix(Rule::not_equals, Assoc::Left))
		.op(Op::infix(Rule::less_than_equal_to, Assoc::Left) | Op::infix(Rule::greater_than_equal_to, Assoc::Left))
		.op(Op::infix(Rule::less_than, Assoc::Left) | Op::infix(Rule::greater_than, Assoc::Left))
		.op(Op::infix(Rule::addition, Assoc::Left) | Op::infix(Rule::subtraction, Assoc::Left))
		.op(Op::infix(Rule::multiplication, Assoc::Left) | Op::infix(Rule::division, Assoc::Left))
		.op(Op::prefix(Rule::negative) | Op::prefix(Rule::logical_not) | Op::prefix(Rule::bitwise_not) | Op::prefix(Rule::learned_value))
}

#[derive(Default)]
pub struct ParserState<'a> {
	programs: Vec<String>,
	parsers: Vec<DSLParser<'a>>,
}

impl ParserState<'_> {
	pub fn parse_file(&mut self, file_name: &str) -> DSLParser<'_> {
		self.programs.push(std::fs::read_to_string(file_name).unwrap());
		DSLParser {
			pairs: DSLParser::parse(Rule::program, &self.programs.iter().last().unwrap()).unwrap(),
			pratt: configure_pratt(),
		}
	}
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DSLParser<'a> {
	pub pairs: Pairs<'a, Rule>,
	pub pratt: PrattParser<Rule>,
}
