#[cfg(test)]
mod tests {
	use crate::{ NestedCode, NestedCodeParser, UNMATCHED_NAME };

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
		
		assert_eq!(result.children().len(), 3);
		assert_eq!(result[0].type_name(), "if-statement");
		assert_eq!(result[1].type_name(), "scope");
		assert_eq!(result[1][0].type_name(), "comment");
		assert_eq!(result[1][1].type_name(), "if-statement");
		assert_eq!(result[1][2].type_name(), "scope");
		assert_eq!(result[1][2][0].type_name(), "print-statement");
		assert_eq!(result[1][2][0][0].type_name(), "string");
		assert_eq!(result[2].type_name(), "scope");
		assert_eq!(result[2][0].type_name(), "comment");
		assert_eq!(
		 	result.flatten().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
			vec![UNMATCHED_NAME, "if-statement", "scope", "comment", "if-statement", "scope", "print-statement", "string", "scope", "comment"]
		);
	}

	#[test]
	fn test_nesting_structure_match_any_white_space() {
		let parser:NestedCodeParser = example_parser().match_any_white_space();
		let result:NestedCode = parser.parse(EXAMPLE_TEXT).unwrap();
		println!("{:?}", result);
		
		assert_eq!(result.children().len(), 4);
		assert_eq!(result[0].type_name(), "if-statement");
		assert_eq!(result[1].type_name(), "scope");
		assert_eq!(result[1][0].type_name(), "comment");
		assert_eq!(result[1][1].type_name(), "if-statement");
		assert_eq!(result[1][2].type_name(), "scope");
		assert_eq!(result[1][2][0].type_name(), "print-statement");
		assert_eq!(result[1][2][0][0].type_name(), "string");
		assert_eq!(result[2].type_name(), "if-statement");
		assert_eq!(result[3].type_name(), "scope");
		assert_eq!(result[3][0].type_name(), "comment");
		assert_eq!(
			result.flatten().iter().map(|(_, code)| code.type_name()).collect::<Vec<&str>>(),
			vec![UNMATCHED_NAME, "if-statement", "scope", "comment", "if-statement", "scope", "print-statement", "string", "if-statement", "scope", "comment"]
		);
	}
}