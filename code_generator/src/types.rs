use ai_dsl2_compiler::Type;

pub fn convert_type_name(type_name: &str) -> Type {
	match type_name {
		"int" => Type::Integer,
		"string" => Type::CString,
		&_ => todo!(),
	}
}