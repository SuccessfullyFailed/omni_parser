use crate::{ NestedCodeParser, NestedCodeSegment, AUTO_CLOSE };
use std::{ error::Error, sync::{ Mutex, MutexGuard } };



// Definitions.
const DICT_NAME:&str = "dict";
const ARRAY_NAME:&str = "array";
const STRING_NAME:&str = "string";
const INTEGER_NAME:&str = "integer";
const FLOAT_NAME:&str = "float";
const BOOL_NAME:&str = "bool";

const DICT_DIVIDER_NAME:&str = "dict_divider";
const LIST_DIVIDER_NAME:&str = "list_divider";

const BOOL_CHECK:&'static dyn Fn(&str) -> Option<usize> = &|contents| if contents.len() >= 4 && (contents[..4].to_lowercase().starts_with("true") || contents.to_lowercase().starts_with("false")) { Some(4) } else { None };



// JSON NestedCodeParser.
static mut JSON_PARSER_LOCK:Mutex<()> = Mutex::new(());
static mut JSON_PARSER:Option<NestedCodeParser> = None;
fn json_parser() -> &'static NestedCodeParser {
	unsafe {
		match JSON_PARSER.as_ref() {
			Some(parser) => &parser,
			None => {
				let _lock:MutexGuard<'_, ()> = JSON_PARSER_LOCK.lock().unwrap();
				JSON_PARSER = Some(
					NestedCodeParser::new(vec![
						&(DICT_NAME, true, "{", "}"),
						&(ARRAY_NAME, true, "[", "]"),
						&(STRING_NAME, false, "\"", None, "\"", Some("\\")),
						&(STRING_NAME, false, "'", "'"),
						&(STRING_NAME, false, "`", "`"),
						&(FLOAT_NAME, r#"^-?\d+\.\d+?"#),
						&(INTEGER_NAME, r#"^-?\d+"#),
						&(BOOL_NAME, false, BOOL_CHECK, AUTO_CLOSE),
						
						&(DICT_DIVIDER_NAME, false, ":", ""),
						&(LIST_DIVIDER_NAME, false, ",", "")
					]).ignore_white_space_segments()
				);
				json_parser()
			}
		}
	}
}



#[derive(Clone, PartialEq, Debug)]
pub enum Json {
	Dict(Vec<(Json, Json)>),
	Array(Vec<Json>),
	String(String),
	Integer(i64),
	Float(f64),
	Bool(bool)
}
impl Json {

	/* CONSTRUCTOR METHODS */

	/// Create a new json object from contents.
	pub fn new(contents:&str) -> Result<Json, Box<dyn Error>> {

		// Parse and validate contents.
		let parsed_contents:NestedCodeSegment = json_parser().parse(contents.trim());
		if parsed_contents.sub_segments().is_empty() || !parsed_contents[0].matched() {
			return Err(format!("Could not parse JSON from contents:\n\n{}", contents).into());
		}

		// Return json nodes from contents.
		Self::nested_code_to_json_node(&parsed_contents.sub_segments()[0])
	}

	/// Turn a parsed contents node into a json node.
	fn nested_code_to_json_node(parsed_code:&NestedCodeSegment) -> Result<Json, Box<dyn Error>> {

		// Parse dictionary.
		if parsed_code.type_name() == DICT_NAME {
			println!("{}", parsed_code.sub_segments().iter().map(|s| format!("{}: {}", s.type_name(), s.outer_contents())).collect::<Vec<String>>().join("\n"));

			// Validate parsable dict.
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 1).all(|(_, sub_content)| sub_content.type_name() == DICT_DIVIDER_NAME) {
				return Err(format!("Could not create json dictionary from contents. Expected ':' in:\n\n{}", parsed_code.inner_contents()).into());
			}
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 3).all(|(_, sub_content)| sub_content.type_name() == LIST_DIVIDER_NAME) {
				return Err(format!("Could not create json dictionary from contents. Expected ',' in:\n\n{}", parsed_code.inner_contents()).into());
			}
			if parsed_code.sub_segments().len() % 4 != 3 {
				return Err(format!("Could not create json dictionary from contents. Unexpected end in:\n\n{}", parsed_code.inner_contents()).into());
			}
			
			// Get item keys and values.
			let key_items = parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 0).map(|(_, item)| item);
			let keys:Vec<Json> = key_items.map(|item| Self::nested_code_to_json_node(item).unwrap_or(Json::String(item.inner_contents().trim().to_string()))).collect::<Vec<Json>>();
			let value_items = parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 2).map(|(_, item)| item);
			let values:Vec<Result<Json, Box<dyn Error>>> = value_items.map(|item| Self::nested_code_to_json_node(item)).collect::<Vec<Result<Json, Box<dyn Error>>>>();
			if values.iter().all(|item| item.is_ok()) {
				return Ok(Json::Dict(keys.iter().zip(values).map(|(key, value)| (key.clone(), value.unwrap())).collect::<Vec<(Json, Json)>>()));
			}
		}

		// Parse array.
		if parsed_code.type_name() == ARRAY_NAME {
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 2 == 1).all(|(_, sub_content)| sub_content.type_name() == LIST_DIVIDER_NAME) {
				return Err(format!("Could not create json array from contents. Each odd index should be a comma, but is not in:\n\n{}", parsed_code.outer_contents()).into());
			}
			let items:Vec<Result<Json, Box<dyn Error>>> = parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 2 == 0).map(|(_, item)| Self::nested_code_to_json_node(item)).collect();
			if items.iter().all(|item| item.is_ok()) {
				return Ok(Json::Array(items.iter().flatten().cloned().collect()));
			}
		}
		
		// Parse string.
		if parsed_code.type_name() == STRING_NAME {
			return Ok(Json::String(parsed_code.outer_contents().to_string()));
		}

		// Parse number.
		if parsed_code.type_name() == FLOAT_NAME {
			return Ok(Json::Float(parsed_code.outer_contents().parse::<f64>()?))
		}
		if parsed_code.type_name() == INTEGER_NAME {
			return Ok(Json::Integer(parsed_code.outer_contents().parse::<i64>()?))
		}

		// Parse bool.
		if parsed_code.type_name() == BOOL_NAME {
			return Ok(Json::Bool(parsed_code.outer_contents().to_lowercase() == "true"))
		}

		// Could not parse, return error.
		Err(format!("Could not create json from contents:\n\n{}", parsed_code.outer_contents()).into())
	}
}
impl ToString for Json {
	fn to_string(&self) -> String {
		match self {
			Json::Dict(children) => format!("{} {} {}", '{', children.iter().map(|(key, value)| format!("{}: {}", key.to_string(), value.to_string())).collect::<Vec<String>>().join(", "), '}'),
			Json::Array(children) => format!("[{}]", children.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", ")),
			Json::String(contents) => contents.clone(),
			Json::Integer(contents) => contents.to_string(),
			Json::Float(contents) => contents.to_string(),
			Json::Bool(value) => value.to_string()
		}
	}
}