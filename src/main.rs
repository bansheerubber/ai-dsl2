#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

fn main() {
	let program = std::fs::read_to_string("test.ai").unwrap();
	let result = DSLParser::parse(Rule::program, &program);
	println!("{:?}", result);
}
