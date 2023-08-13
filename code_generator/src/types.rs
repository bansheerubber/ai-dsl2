use ai_dsl2_compiler::{ Module, Type, };

pub fn convert_type_name(module: &Module, type_name: &str) -> Type {
	match type_name {
		"float" => Type::Float(0),
		"int" => Type::Integer(0, 64),
		"string" => Type::CString(0),
		name => { // handling structs
			Type::Struct(1, module.lookup_struct_type_index(name))
		}
	}
}