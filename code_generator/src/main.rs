mod compiler;
mod parser;
mod types;

fn main() {
	let mut state = parser::ParserState::default();
	let mut context = compiler::CompilationContext::new("test.ai", &mut state);

	let pairs = context.parser.pairs.clone();
	compiler::compile_pairs(&mut context, pairs);

	context.module.write_bitcode("main.bc");
}
