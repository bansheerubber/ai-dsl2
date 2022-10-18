#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

fn main() {
	let result = DSLParser::parse(Rule::program, "printf(#5);");
	println!("{:?}", result);
}
