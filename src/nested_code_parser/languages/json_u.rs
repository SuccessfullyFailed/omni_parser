



#[cfg(test)]
mod test {
	use crate::nested_code_parser::languages::json::Json;



	#[test]
	fn test_json_string() {
		assert_eq!(
			Json::new(r#""test_str""#).unwrap(),
			Json::String(r#""test_str""#.to_string())
		);
	}

	#[test]
	fn test_json_array() {
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
	fn test_json_dict() {
		assert_eq!(
			Json::new(r#"{ "a": "1", "b": "2", "c": "3" }"#).unwrap(),
			Json::Dict(vec![
				(Json::String(r#""a""#.to_string()), Json::String(r#""1""#.to_string())),
				(Json::String(r#""b""#.to_string()), Json::String(r#""2""#.to_string())),
				(Json::String(r#""c""#.to_string()), Json::String(r#""3""#.to_string()))
			])
		);
	}
}