use pest::Parser;
use pest::iterators::Pairs;
use pest::pratt_parser::{ Assoc, Op, PrattParser };
use pest_derive::Parser;

fn configure_pratt() -> PrattParser<Rule> {
	PrattParser::new()
		.op(Op::infix(Rule::addition, Assoc::Left) | Op::infix(Rule::subtraction, Assoc::Left))
		.op(Op::infix(Rule::multiplication, Assoc::Left) | Op::infix(Rule::division, Assoc::Left))
		.op(Op::prefix(Rule::negative))
}

#[derive(Default)]
pub struct ParserState<'a> {
	programs: Vec<String>,
	parsers: Vec<DSLParser<'a>>,
}

impl ParserState<'_> {
	pub fn parse_file(&mut self, file_name: &str) -> DSLParser<'_> {
		self.programs.push(std::fs::read_to_string(file_name).unwrap());
		let mut parser = DSLParser {
			pairs: DSLParser::parse(Rule::program, &self.programs.iter().last().unwrap()).unwrap(),
			pratt: configure_pratt(),
		};

		return parser;
	}
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DSLParser<'a> {
	pub pairs: Pairs<'a, Rule>,
	pub pratt: PrattParser<Rule>,
}
