use ai_dsl2_compiler::Module;
use pest::Parser;

mod compiler;
mod parser;
mod types;

fn main() {
	let program = std::fs::read_to_string("test.ai").unwrap();
	let pairs = parser::DSLParser::parse(parser::Rule::program, &program).unwrap();
	let mut module = Module::new("main");

	compiler::compile_pairs(&mut module, pairs);

	module.write_bitcode("main.bc");
}
