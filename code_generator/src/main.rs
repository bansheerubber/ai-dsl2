use ai_dsl2_compiler::Module;

mod compiler;
mod parser;
mod types;

fn main() {
	let mut state = parser::ParserState::default();

	let mut context = compiler::CompilationContext {
		current_block: None,
		module: Module::new("main"),
		parser: state.parse_file("test.ai"),
	};

	let pairs = context.parser.pairs.clone();
	compiler::compile_pairs(&mut context, pairs);

	context.module.write_bitcode("main.bc");
}
