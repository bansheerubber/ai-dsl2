use ai_dsl2_compiler::Type;

pub fn convert_type_name(type_name: &str) -> Type {
	match type_name {
		"float" => Type::Float(0),
		"int" => Type::Integer(0, 64),
		"string" => Type::CString(0),
		&_ => todo!(),
	}
}