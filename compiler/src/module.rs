use std::collections::HashMap;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::object::TypeTable;
use crate::{ Block, Builder, Function, FunctionKey, FunctionTable, TerminalInstruction, Type, Value, VariableTable, };
use crate::strings::StringTable;

#[derive(Debug)]
pub struct Module {
	context: LLVMContextRef,
	pub function_table: FunctionTable,
	module: LLVMModuleRef,
	pub string_table: StringTable, // keep the strings alive for as long as we are using LLVM resources
	pub type_table: TypeTable,
	pub variable_table: VariableTable,
}

impl Module {
	pub fn new(name: &str) -> Self {
		let mut string_table = StringTable::default();

		let context;
		let module;
		unsafe {
			context = LLVMContextCreate();
			module = LLVMModuleCreateWithName(string_table.to_llvm_string(name));
		}

		Module {
			context,
			function_table: FunctionTable::default(),
			module,
			string_table,
			type_table: TypeTable::default(),
			variable_table: VariableTable::default(),
		}
	}

	pub fn to_llvm_type(&self, type_enum: Type) -> LLVMTypeRef {
		unsafe {
			match type_enum {
				Type::CString(0) => {
					let i8_type = LLVMIntType(8);
					LLVMPointerType(i8_type, 0)
				},
				Type::Float(0) => LLVMDoubleType(),
				Type::Float(1) => LLVMPointerType(LLVMDoubleType(), 64),
				Type::FloatArray(size) => LLVMArrayType(LLVMDoubleType(), size as u32),
				Type::Integer(0, bits) => LLVMIntType(bits),
				Type::Integer(1, bits) => LLVMPointerType(LLVMIntType(bits), 64),
				Type::Void => LLVMVoidType(),
				_ => LLVMVoidType(),
			}
		}
	}

	pub fn write_bitcode(&mut self, filename: &str) {
		// add airt function references
		let airt_register_function = self.create_extern_function(
			"airt_register_function",
			&vec![Type::CString(0), Type::Integer(0, 64), Type::Integer(0, 64)],
			Type::Void
		);

		// TODO remove this inline
		let airt_init = {
			let function_type;
			let function;
			unsafe {
				let mut arguments = vec![
					LLVMPointerType(LLVMFunctionType(self.to_llvm_type(Type::Integer(0, 64)), [].as_mut_ptr(), 0 as u32, 0), 64),
					LLVMPointerType(LLVMFunctionType(self.to_llvm_type(Type::Float(0)), [].as_mut_ptr(), 0 as u32, 0), 64),
				];

				function_type = LLVMFunctionType(
					self.to_llvm_type(Type::Void), arguments.as_mut_ptr(), arguments.len() as u32, 0
				);

				function = LLVMAddFunction(
					self.get_module(),
					self.string_table.to_llvm_string("airt_init"),
					function_type
				);
			}

			let function = Function {
				argument_types: Vec::new(),
				argument_values: Vec::new(),
				blocks: HashMap::new(),
				block_terminals: HashMap::new(),
				function,
				function_type,
				learned_values: vec![],
				name: String::from("airt_init"),
				return_type: Type::Void,

				check_arguments: false,
			};

			self.function_table.add_function("airt_init", function)
		};

		let airt_train = self.create_extern_function("airt_train", &vec![], Type::Void);

		// generate main function
		let main_function = self.create_extern_function("main", &vec![], Type::Integer(0, 64));
		let main_block = self.new_block("main", &main_function);

		// add call to `airt_init`
		let reset_function = self.function_table.get_function(&FunctionKey::new("_main")).unwrap().function;
		let tick_function = self.function_table.get_function(&FunctionKey::new("_tick")).unwrap().function;

		let reset_function_value = unsafe {
			let builder = Builder::new();
			builder.seek_to_end(main_block);

			let location = LLVMBuildAlloca(
				builder.get_builder(),
				LLVMPointerType(
					LLVMFunctionType(self.to_llvm_type(Type::Integer(0, 64)), [].as_mut_ptr(), 0 as u32, 0),
					64
				),
				self.string_table.to_llvm_string("reset_function")
			);

			LLVMBuildStore(
				builder.get_builder(),
				reset_function,
				location,
			);

			LLVMBuildLoad2(
				builder.get_builder(),
				LLVMPointerType(
					LLVMFunctionType(self.to_llvm_type(Type::Integer(0, 64)), [].as_mut_ptr(), 0 as u32, 0),
					64
				),
				location,
				self.string_table.to_llvm_string("reset_function_deref")
			)
		};

		let tick_function_value = unsafe {
			let builder = Builder::new();
			builder.seek_to_end(main_block);

			let location = LLVMBuildAlloca(
				builder.get_builder(),
				LLVMPointerType(
					LLVMFunctionType(self.to_llvm_type(Type::Float(0)), [].as_mut_ptr(), 0 as u32, 0),
					64
				),
				self.string_table.to_llvm_string("tick_function")
			);

			LLVMBuildStore(
				builder.get_builder(),
				tick_function,
				location,
			);

			LLVMBuildLoad2(
				builder.get_builder(),
				LLVMPointerType(
					LLVMFunctionType(self.to_llvm_type(Type::Float(0)), [].as_mut_ptr(), 0 as u32, 0),
					64
				),
				location,
				self.string_table.to_llvm_string("tick_function_deref")
			)
		};

		self.add_function_call(
			main_block,
			&airt_init,
			&mut vec![Value {
				type_enum: Type::Void,
				value: reset_function_value,
			}, Value {
				type_enum: Type::Void,
				value: tick_function_value,
			}]
		);

		// TODO figure out how to make this block less messy
		let name_globals = {
			let mut names = Vec::new();
			for (_, function) in self.function_table.iter() {
				names.push(function.name.clone());
			}

			let mut name_globals = Vec::new();
			for name in names {
				name_globals.push(self.create_global_string(main_block, &name));
			}

			name_globals
		};

		// get the name and input/output numbers for `airt_register_function`
		let mut args: Vec<Vec<Value>> = Vec::new();
		for ((_, function), name) in self.function_table.iter().zip(name_globals.iter()) {
			if function.learned_values.len() == 0 {
				continue;
			}

			if function.argument_types.len() == 0 {
				panic!("Learned value container function needs more than 0 arguments");
			}

			args.push(vec![
				*name,
				self.create_immediate_integer(function.argument_types.len() as u64),
				self.create_immediate_integer(function.learned_values.len() as u64),
			]);
		}

		// call `airt_register_function`
		for mut arg in args {
			self.add_function_call(main_block, &airt_register_function, &mut arg);
		}

		// call `_main`
		self.add_function_call(main_block, &FunctionKey::new("_main"), &mut []);

		// call `airt_train`
		self.add_function_call(main_block, &airt_train, &mut []);

		// return 0
		self.add_return(main_block, self.create_immediate_integer(0));

		unsafe {
			LLVMWriteBitcodeToFile(self.module, self.string_table.to_llvm_string(filename));
		}
	}

	pub fn create_global_string(&mut self, block: Block, string: &str) -> Value {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			Value {
				type_enum: Type::CString(0),
				value: LLVMBuildGlobalStringPtr(
					builder.get_builder(),
					self.string_table.to_llvm_string(string),
					self.string_table.to_llvm_string("") // TODO what is this for?
				),
			}
		}
	}

	pub fn add_return(&mut self, block: Block, value: Value) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let value = self.resolve_value(block, value).value;

			self.set_block_terminal(
				block,
				TerminalInstruction::Return {
					instruction: LLVMBuildRet(builder.get_builder(), value),
					value,
				}
			);
		}
	}

	pub fn add_return_void(&mut self, block: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			self.set_block_terminal(
				block,
				TerminalInstruction::ReturnVoid {
					instruction: LLVMBuildRetVoid(builder.get_builder()),
				}
			);
		}
	}

	pub fn add_branch(&mut self, block: Block, target: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			self.set_block_terminal(
				block,
				TerminalInstruction::Branch {
					instruction: LLVMBuildBr(builder.get_builder(), target.get_block()),
					target: target.get_block(),
				}
			);
		}
	}

	pub fn add_branch_if_true(&mut self, block: Block, value: Value, if_true: Block, if_false: Block) {
		unsafe {
			let builder = Builder::new();
			builder.seek_to_end(block);

			let instruction = LLVMBuildCondBr(
				builder.get_builder(),
				self.math_resolve_value(block, value, Type::Integer(0, 1)).value,
				if_true.get_block(),
				if_false.get_block()
			);

			self.set_block_terminal(
				block,
				TerminalInstruction::ConditionalBranch {
					instruction,
					target_if_false: if_false.get_block(),
					target_if_true: if_true.get_block(),
				}
			);
		}
	}

	pub fn get_context(&self) -> LLVMContextRef {
		self.context
	}

	pub fn get_module(&self) -> LLVMModuleRef {
		self.module
	}

	// some function names in LLVM have a special meaning (main, linked functions, etc), so to avoid this problem the DSL
	// will transform them a little to prevent symbol collisions
	pub fn transform_function_name(&self, name: &str) -> String {
		return format!("_{}", name);
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeModule(self.module);
			LLVMContextDispose(self.context);
		}
	}
}
