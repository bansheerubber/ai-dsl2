use ai_dsl2_compiler::Module;
use pest::Parser;

mod compiler;
mod parser;
mod types;

fn main() {
	let program = std::fs::read_to_string("test.ai").unwrap();
	let pairs = parser::DSLParser::parse(parser::Rule::program, &program).unwrap();

	let mut context = compiler::CompilationContext {
		current_block: None,
		module: Module::new("main"),
	};

	compiler::compile_pairs(&mut context, pairs);

	context.module.write_bitcode("main.bc");
}
