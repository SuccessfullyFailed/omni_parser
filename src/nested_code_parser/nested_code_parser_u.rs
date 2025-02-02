#[cfg(test)]
mod tests {
	use crate::{ NestedCodeParser, NestedSegment, ROOT_NAME, nested_code_parser::{ CONTENTS_NAME, WHITESPACE_NAME } };

	/* HELPER FUNCTIONS */

	const EXAMPLE_TEXT:&str = r#"
	if necessary {
		// Makes the program do the expected thing.
		let thing_result = do_the_thing();
		if thing_result.is_ok() {
			println!("Successful thinging complete! Exited with error code \"{}\".", get_code());
		}
	}
	if	 weirdly_spaced_bool {
		// This comment contains white-space, but is not split up despite it's white-space end-tag.
	}
	confusing footer?
	"#;
	fn example_parser() -> NestedCodeParser {
		NestedCodeParser::new(vec![
			&("comment", false, "//", "\n"),
			&("scope", true, "{", "}"),
			&("if-statement", true, "if ", " "),
			&("string", false, "\"", None, "\"", Some("\\")),
			&("print-statement", true, "println!(", ");")
		])
	}


	/* TESTS */

	#[test]
	fn test_nesting_structure() {
		let parser:NestedCodeParser = example_parser();
		let result:NestedSegment = parser.parse(EXAMPLE_TEXT);
		println!("{:?}", result);
		
		assert_eq!(result.sub_segments().len(), 6);
		assert_eq!(result[0].type_name(), WHITESPACE_NAME);
		assert_eq!(result[1].type_name(), "if-statement");
		assert_eq!(result[2].type_name(), "scope");
		assert_eq!(result[2][0].type_name(), WHITESPACE_NAME);
		assert_eq!(result[2][1].type_name(), "comment");
		assert_eq!(result[2][2].type_name(), CONTENTS_NAME);
		assert_eq!(result[2][3].type_name(), "if-statement");
		assert_eq!(result[2][4].type_name(), "scope");
		assert_eq!(result[2][4][0].type_name(), WHITESPACE_NAME);
		assert_eq!(result[2][4][1].type_name(), "print-statement");
		assert_eq!(result[2][4][1][0].type_name(), "string");
		assert_eq!(result[3].type_name(), CONTENTS_NAME);
		assert_eq!(result[4].type_name(), "scope");
		assert_eq!(result[4][0].type_name(), WHITESPACE_NAME);
		assert_eq!(result[4][1].type_name(), "comment");
		assert_eq!(result[5].type_name(), CONTENTS_NAME);
		assert_eq!(
		  	result.flat().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
		 	vec![ROOT_NAME, WHITESPACE_NAME, "if-statement", CONTENTS_NAME, "scope", WHITESPACE_NAME, "comment", CONTENTS_NAME, CONTENTS_NAME, "if-statement", CONTENTS_NAME, "scope", WHITESPACE_NAME, "print-statement", "string", CONTENTS_NAME, CONTENTS_NAME, WHITESPACE_NAME, WHITESPACE_NAME, CONTENTS_NAME, "scope", WHITESPACE_NAME, "comment", CONTENTS_NAME, WHITESPACE_NAME, CONTENTS_NAME]
		);
	}

	#[test]
	fn test_ignore_white_space_segments() {
		let parser:NestedCodeParser = example_parser().ignore_white_space_segments();
		let result:NestedSegment = parser.parse(EXAMPLE_TEXT);
		println!("{:?}", result);
		
		assert_eq!(result.sub_segments().len(), 5);
		assert_eq!(result[0].type_name(), "if-statement");
		assert_eq!(result[1].type_name(), "scope");
		assert_eq!(result[1][0].type_name(), "comment");
		assert_eq!(result[1][1].type_name(), CONTENTS_NAME);
		assert_eq!(result[1][2].type_name(), "if-statement");
		assert_eq!(result[1][3].type_name(), "scope");
		assert_eq!(result[1][3][0].type_name(), "print-statement");
		assert_eq!(result[1][3][0][0].type_name(), "string");
		assert_eq!(result[2].type_name(), CONTENTS_NAME);
		assert_eq!(result[3].type_name(), "scope");
		assert_eq!(result[3][0].type_name(), "comment");
		assert_eq!(result[4].type_name(), CONTENTS_NAME);
		assert_eq!(
		 	result.flat().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
			vec![ROOT_NAME, "if-statement", CONTENTS_NAME, "scope", "comment", CONTENTS_NAME, CONTENTS_NAME, "if-statement", CONTENTS_NAME, "scope", "print-statement", "string", CONTENTS_NAME, CONTENTS_NAME, CONTENTS_NAME, "scope", "comment", CONTENTS_NAME, CONTENTS_NAME]
		);
	}

	#[test]
	fn test_double_escape() {
		let parser:NestedCodeParser = example_parser();
		assert_eq!(parser.parse(r#"- "test" -"#).sub_segments()[1].to_string(), r#""test""#);
		assert_eq!(parser.parse(r#"- "test\"" -"#).sub_segments()[1].to_string(), r#""test\"""#);
		assert_eq!(parser.parse(r#"- "test\\" -"#).sub_segments()[1].to_string(), r#""test\\""#);
		assert_eq!(parser.parse(r#"- "test\\\"" -"#).sub_segments()[1].to_string(), r#""test\\\"""#);
	}

	#[test]
	fn test_identification_types() {

		// CharSet match.
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, "//", "\n")]);
		assert_eq!(parser.parse("-- // test\n --").sub_segments()[1].to_string(), "// test\n");
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, "//", Some("P"), "\n", Some("\\"))]);
		assert_eq!(parser.parse("-- P// // test\\\n \n --").sub_segments()[1].to_string(), "// test\\\n \n");

		// Method match.
		const OPEN:&'static dyn Fn(&str) -> Option<usize> = &|contents| if contents.len() >= 2 && &contents[..2] == "//" { Some(2) } else { None };
		const CLOSE:&'static dyn Fn(&str) -> Option<usize> = &|contents| if contents.len() >= 1 && &contents[..1] == "\n" { Some(1) } else { None };
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, OPEN, CLOSE)]);
		assert_eq!(parser.parse("-- // test\n --").sub_segments()[1].to_string(), "// test\n");

		// Regex match.
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", "^//.+\n")]);
		assert_eq!(parser.parse("-- // test\n --").sub_segments()[1].to_string(), "// test\n");
	}

	#[test]
	fn test_flatten_and_inflate() {
		let parser:NestedCodeParser = example_parser();
		let result:NestedSegment = parser.parse(EXAMPLE_TEXT);
		println!("{:?}", result);

		let flat:Vec<(usize, NestedSegment)> = result.clone().to_flat();
		let validation:NestedSegment = NestedSegment::from_flat(flat).unwrap();
		assert_eq!(result, validation);
	}
}