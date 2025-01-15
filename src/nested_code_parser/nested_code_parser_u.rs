#[cfg(test)]
mod tests {
	use crate::{ NestedCode, NestedCodeParser, ROOT_NAME, UNMATCHED_NAME, UNMATCHED_WHITESPACE_NAME };

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
		let result:NestedCode = parser.parse(EXAMPLE_TEXT).unwrap();
		println!("{:?}", result);
		
		assert_eq!(result.contents().len(), 6);
		assert_eq!(result[0].type_name(), UNMATCHED_WHITESPACE_NAME);
		assert_eq!(result[1].type_name(), "if-statement");
		assert_eq!(result[2].type_name(), "scope");
		assert_eq!(result[2][0].type_name(), UNMATCHED_WHITESPACE_NAME);
		assert_eq!(result[2][1].type_name(), "comment");
		assert_eq!(result[2][2].type_name(), UNMATCHED_NAME);
		assert_eq!(result[2][3].type_name(), "if-statement");
		assert_eq!(result[2][4].type_name(), "scope");
		assert_eq!(result[2][4][0].type_name(), UNMATCHED_WHITESPACE_NAME);
		assert_eq!(result[2][4][1].type_name(), "print-statement");
		assert_eq!(result[2][4][1][0].type_name(), "string");
		assert_eq!(result[3].type_name(), UNMATCHED_NAME);
		assert_eq!(result[4].type_name(), "scope");
		assert_eq!(result[4][0].type_name(), UNMATCHED_WHITESPACE_NAME);
		assert_eq!(result[4][1].type_name(), "comment");
		assert_eq!(result[5].type_name(), UNMATCHED_NAME);
		assert_eq!(
		 	result.flatten().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
			vec![ROOT_NAME, UNMATCHED_WHITESPACE_NAME, "if-statement", UNMATCHED_NAME, "scope", UNMATCHED_WHITESPACE_NAME, "comment", UNMATCHED_NAME, UNMATCHED_NAME, "if-statement", UNMATCHED_NAME, "scope", UNMATCHED_WHITESPACE_NAME, "print-statement", "string", UNMATCHED_NAME, UNMATCHED_NAME, UNMATCHED_WHITESPACE_NAME, UNMATCHED_WHITESPACE_NAME, UNMATCHED_NAME, "scope", UNMATCHED_WHITESPACE_NAME, "comment", UNMATCHED_NAME, UNMATCHED_WHITESPACE_NAME, UNMATCHED_NAME]
		);
	}

	#[test]
	fn test_ignore_white_space_segments() {
		let parser:NestedCodeParser = example_parser().ignore_white_space_segments();
		let result:NestedCode = parser.parse(EXAMPLE_TEXT).unwrap();
		println!("{:?}", result);
		
		assert_eq!(result.contents().len(), 5);
		assert_eq!(result[0].type_name(), "if-statement");
		assert_eq!(result[1].type_name(), "scope");
		assert_eq!(result[1][0].type_name(), "comment");
		assert_eq!(result[1][1].type_name(), UNMATCHED_NAME);
		assert_eq!(result[1][2].type_name(), "if-statement");
		assert_eq!(result[1][3].type_name(), "scope");
		assert_eq!(result[1][3][0].type_name(), "print-statement");
		assert_eq!(result[1][3][0][0].type_name(), "string");
		assert_eq!(result[2].type_name(), UNMATCHED_NAME);
		assert_eq!(result[3].type_name(), "scope");
		assert_eq!(result[3][0].type_name(), "comment");
		assert_eq!(result[4].type_name(), UNMATCHED_NAME);
		assert_eq!(
		 	result.flatten().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
			vec![ROOT_NAME, "if-statement", UNMATCHED_NAME, "scope", "comment", UNMATCHED_NAME, UNMATCHED_NAME, "if-statement", UNMATCHED_NAME, "scope", "print-statement", "string", UNMATCHED_NAME, UNMATCHED_NAME, UNMATCHED_NAME, "scope", "comment", UNMATCHED_NAME, UNMATCHED_NAME]
		);
	}

	#[test]
	fn test_double_escape() {
		let parser:NestedCodeParser = example_parser();
		assert_eq!(parser.parse(r#"- "test" -"#).unwrap().contents()[1].contents_joined(), r#""test""#);
		assert_eq!(parser.parse(r#"- "test\"" -"#).unwrap().contents()[1].contents_joined(), r#""test\"""#);
		assert_eq!(parser.parse(r#"- "test\\" -"#).unwrap().contents()[1].contents_joined(), r#""test\\""#);
		assert_eq!(parser.parse(r#"- "test\\\"" -"#).unwrap().contents()[1].contents_joined(), r#""test\\\"""#);
	}

	#[test]
	fn test_identification_types() {

		// CharSet match.
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, "//", "\n")]);
		assert_eq!(parser.parse("-- // test\n --").unwrap().contents()[1].contents_joined(), "// test\n");
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, "//", Some("P"), "\n", Some("\\"))]);
		assert_eq!(parser.parse("-- P// // test\\\n \n --").unwrap().contents()[1].contents_joined(), "// test\\\n \n");

		// Method match.
		const OPEN:&'static dyn Fn(&[char]) -> Option<usize> = &|contents:&[char]| if contents[0] == '/' && contents[1] == '/' { Some(2) } else { None };
		const CLOSE:&'static dyn Fn(&[char]) -> Option<usize> = &|contents:&[char]| if contents[0] == '\n' { Some(1) } else { None };
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", false, OPEN, CLOSE)]);
		assert_eq!(parser.parse("-- // test\n --").unwrap().contents()[1].contents_joined(), "// test\n");

		// Regex match.
		let parser:NestedCodeParser = NestedCodeParser::new(vec![&("comment", "^//.+\n")]);
		assert_eq!(parser.parse("-- // test\n --").unwrap().contents()[1].contents_joined(), "// test\n");
	}
}