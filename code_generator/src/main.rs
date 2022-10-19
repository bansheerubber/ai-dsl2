#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

fn main() {
	let program = std::fs::read_to_string("test.ai").unwrap();
	let result = DSLParser::parse(Rule::program, &program).unwrap();

	for pair in result {
		println!("{:?}", pair.as_rule());
	}

	// let mut module = Module::new("main");

	// module.create_function("log", &vec![Type::CString], Type::Void);
	// module.create_function("main", &vec![], Type::Void);

	// module.seek_to_block(&module.function_table.get_function("main").unwrap().block);
	// let mut log_args = vec![module.create_global_string("hey there")];
	// module.add_function_call("log", &mut log_args);

	// module.write_bitcode("main.bc");
}
