#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::bit_writer::*;
use std::ffi::CString;
use std::os::raw::{c_uint, c_ulonglong};
use std::ptr;

mod compiler;
mod utility;

macro_rules! c_str {
	($s:expr) => (
		concat!($s, "\0").as_ptr() as *const i8
	);
}

unsafe fn function_call(
	builder: LLVMBuilderRef,
	function: LLVMValueRef,
	function_type: LLVMTypeRef,
	args: &mut [LLVMValueRef]
) {
	LLVMBuildCall2(builder, function_type, function, args.as_mut_ptr(), args.len() as c_uint, c_str!(""));
}

fn main() {
	let program = std::fs::read_to_string("test.ai").unwrap();
	let result = DSLParser::parse(Rule::program, &program);
	println!("{:?}", result);

	let module = compiler::Module::new("main");

	module.borrow_mut().create_function("log", &vec![compiler::Type::CString], compiler::Type::Void);
	module.borrow_mut().create_function("main", &vec![], compiler::Type::Void);

	let function = module.borrow_mut().function_table.get_function("main").unwrap();
	let mut function = function.borrow_mut();
	let block = function.get_block_mut();
	let mut log_args = vec![module.borrow_mut().create_global_string("hey there")];
	block.add_function_call(
		"log",
		&mut log_args
	);

	module.borrow_mut().write_bitcode("main.bc");
}
