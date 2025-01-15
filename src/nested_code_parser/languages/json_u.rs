



#[cfg(test)]
mod test {
	use crate::nested_code_parser::languages::json::Json;



	#[test]
	fn test_bool() {
		assert_eq!(Json::new(r#"true"#).unwrap(), Json::Bool(true));
		assert_eq!(Json::new(r#"True"#).unwrap(), Json::Bool(true));
		assert_eq!(Json::new(r#"TRUE"#).unwrap(), Json::Bool(true));
		assert_eq!(Json::new(r#"false"#).unwrap(), Json::Bool(false));
		assert_eq!(Json::new(r#"False"#).unwrap(), Json::Bool(false));
		assert_eq!(Json::new(r#"FALSE"#).unwrap(), Json::Bool(false));
	}

	#[test]
	fn test_float() {
		assert_eq!(Json::new(r#"000.8"#).unwrap(), Json::Float(0.8));
		assert_eq!(Json::new(r#"0.8"#).unwrap(), Json::Float(0.8));
		assert_eq!(Json::new(r#"10.8"#).unwrap(), Json::Float(10.8));
		assert_eq!(Json::new(r#"-000.8"#).unwrap(), Json::Float(-0.8));
		assert_eq!(Json::new(r#"-0.8"#).unwrap(), Json::Float(-0.8));
		assert_eq!(Json::new(r#"-10.8"#).unwrap(), Json::Float(-10.8));
	}

	#[test]
	fn test_integer() {
		assert_eq!(Json::new(r#"0008"#).unwrap(), Json::Integer(8));
		assert_eq!(Json::new(r#"8"#).unwrap(), Json::Integer(8));
		assert_eq!(Json::new(r#"108"#).unwrap(), Json::Integer(108));
		assert_eq!(Json::new(r#"-0008"#).unwrap(), Json::Integer(-8));
		assert_eq!(Json::new(r#"-8"#).unwrap(), Json::Integer(-8));
		assert_eq!(Json::new(r#"-108"#).unwrap(), Json::Integer(-108));
	}

	#[test]
	fn test_string() {
		assert_eq!(
			Json::new(r#""test_str""#).unwrap(),
			Json::String(r#""test_str""#.to_string())
		);
	}

	#[test]
	fn test_array() {
		assert_eq!(
			Json::new(r#"["a", "b", "c"]"#).unwrap(),
			Json::Array(vec![
				Json::String(r#""a""#.to_string()),
				Json::String(r#""b""#.to_string()),
				Json::String(r#""c""#.to_string())
			])
		);
	}

	#[test]
	fn test_dict() {
		assert_eq!(
			Json::new(r#"{ "a": "A", "b": 2.5, c: "C" }"#).unwrap(),
			Json::Dict(vec![
				(Json::String(r#""a""#.to_string()), Json::String(r#""A""#.to_string())),
				(Json::String(r#""b""#.to_string()), Json::Float(2.5)),
				(Json::String(r#"c"#.to_string()), Json::String(r#""C""#.to_string()))
			])
		);
	}
}