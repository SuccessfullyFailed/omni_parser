use crate::{ NestedCodeSegment, NestedCodeParser, LazyMatchSource };
use std::error::Error;



// NestedCodeParser Settings.
type DefinitionSet<'a> = (&'a str, bool, &'a str, Option<&'a str>, &'a str, Option<&'a str>);
const DICT_DEFINITION:DefinitionSet = ("dict", true, "{", None, "}", None);
const ARRAY_DEFINITION:DefinitionSet = ("array", true, "[", None, "]", None);
const STRING_DEFINITION:DefinitionSet = ("string", false, "\"", None, "\"", Some("\\"));
const STRING_DEFINITION_LITERAL:DefinitionSet = ("string", false, "'", None, "'", None);
const STRING_DEFINITION_MOD:DefinitionSet = ("string", false, "`", None, "`", None);
const DEFINITION_SETS:&[DefinitionSet] = &[DICT_DEFINITION, ARRAY_DEFINITION, STRING_DEFINITION, STRING_DEFINITION_LITERAL, STRING_DEFINITION_MOD];



// JSON NestedCodeParser.
static mut JSON_PARSER:Option<NestedCodeParser> = None;
fn json_parser() -> &'static NestedCodeParser {
	unsafe {
		match JSON_PARSER.as_ref() {
			Some(parser) => &parser,
			None => {
				JSON_PARSER = Some(
					NestedCodeParser::new(
						DEFINITION_SETS.iter().map(|set| set as &dyn LazyMatchSource).collect::<Vec<&dyn LazyMatchSource>>()
					).ignore_white_space_segments()
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
	String(String)
}
impl Json {

	/* CONSTRUCTOR METHODS */

	/// Create a new json object from contents.
	pub fn new(contents:&str) -> Result<Json, Box<dyn Error>> {

		// Parse and validate contents.
		let parsed_contents:NestedCodeSegment = json_parser().parse(contents.trim())?;
		if parsed_contents.sub_segments().is_empty() || !parsed_contents[0].matched() {
			return Err(format!("Could not parse JSON from contents:\n\n{}", contents).into());
		}

		// Return json nodes from contents.
		Self::nested_code_to_json_node(&parsed_contents.sub_segments()[0])
	}

	/// Turn a parsed contents node into a json node.
	fn nested_code_to_json_node(parsed_code:&NestedCodeSegment) -> Result<Json, Box<dyn Error>> {

		// Parse dictionary.
		if parsed_code.type_name() == DICT_DEFINITION.0 {
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 1).all(|(_, sub_content)| sub_content.outer_contents().trim() == ":") {
				return Err(format!("Could not create json dictionary from contents. Expected ':' in:\n\n{}", parsed_code.outer_contents()).into());
			}
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 4 == 3).all(|(_, sub_content)| sub_content.outer_contents().trim() == ",") {
				return Err(format!("Could not create json dictionary from contents. Expected ',' in:\n\n{}", parsed_code.outer_contents()).into());
			}
			if parsed_code.sub_segments().len() % 4 != 3 {
				return Err(format!("Could not create json dictionary from contents. Unexpected end in:\n\n{}", parsed_code.outer_contents()).into());
			}
			let items:Vec<Result<Json, Box<dyn Error>>> = parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 2 == 0).map(|(_, item)| Self::nested_code_to_json_node(item)).collect();
			if items.iter().all(|item| item.is_ok()) {
				return Ok(Json::Dict((0..items.len() / 2).map(|item_index| (items[item_index * 2].as_ref().unwrap().clone(), items[item_index * 2 + 1].as_ref().unwrap().clone())).collect::<Vec<(Json, Json)>>()));
			}
		}

		// Parse array.
		if parsed_code.type_name() == ARRAY_DEFINITION.0 {
			if !parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 2 == 1).all(|(_, sub_content)| sub_content.outer_contents().trim() == ",") {
				return Err(format!("Could not create json array from contents. Each odd index should be a comma, but is not in:\n\n{}", parsed_code.outer_contents()).into());
			}
			let items:Vec<Result<Json, Box<dyn Error>>> = parsed_code.sub_segments().iter().enumerate().filter(|(index, _)| index % 2 == 0).map(|(_, item)| Self::nested_code_to_json_node(item)).collect();
			if items.iter().all(|item| item.is_ok()) {
				return Ok(Json::Array(items.iter().flatten().cloned().collect()));
			}
		}
		
		// Parse string.
		if parsed_code.type_name() == STRING_DEFINITION.0 {
			return Ok(Json::String(parsed_code.outer_contents().to_string()));
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
		}
	}
}