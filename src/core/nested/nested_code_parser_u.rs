#[cfg(test)]
mod tests {
	use crate::{ NestedCode, NestedCodeParser, UNMATCHED_WHITE_SPACE_NAME };

	/* HELPER FUNCTIONS */

	const EXAMPLE_TEXT:&str = "
	if necessary {
		// Makes the program do the expected thing.
		let thing_result = do_the_thing();
		if thing_result.is_ok() {
			println!(\"Successful thinging complete! Exited with error code {}.\", get_code());
		}
	}
	if	 weirdly_spaced_bool {
		// This comment contains white-space, but is not split up despite it's white-space end-tag.
	}
	";
	fn example_parser() -> NestedCodeParser {
		NestedCodeParser::new(vec![
			&("comment", false, "//", "\n"),
			&("scope", true, "{", "}"),
			&("if-statement", true, "if ", " "),
			&("print-statement", true, "println!(", ");")
		])
	}
	fn example_results() -> Vec<NestedCode<'static>> {
		let parser:NestedCodeParser = example_parser();
		parser.parse(EXAMPLE_TEXT)
	}


	/* TESTS */

	#[test]
	fn test_matching_function() {
		assert_eq!(example_parser().tag_matches_contents("if test {}", 0, "if ", &None), Some(3));
		assert_eq!(example_parser().tag_matches_contents(" if test {}", 0, "if ", &None), None);
		assert_eq!(example_parser().tag_matches_contents(" if test {}", 1, "if ", &None), Some(3));
		assert_eq!(example_parser().tag_matches_contents(" if	test {}", 1, "if ", &None), None);
		assert_eq!(example_parser().tag_matches_contents(" if	 \ntest {}", 1, "if ", &None), None);
		assert_eq!(example_parser().match_any_white_space().tag_matches_contents("if test {}", 0, "if ", &None), Some(3));
		assert_eq!(example_parser().match_any_white_space().tag_matches_contents(" if test {}", 0, "if ", &None), None);
		assert_eq!(example_parser().match_any_white_space().tag_matches_contents(" if test {}", 1, "if ", &None), Some(3));
		assert_eq!(example_parser().match_any_white_space().tag_matches_contents(" if	test {}", 1, "if ", &None), Some(3));
		assert_eq!(example_parser().match_any_white_space().tag_matches_contents(" if	 \ntest {}", 1, "if ", &None), Some(5));
	}

	#[test]
	fn test_basic_parsing() {
		let results:Vec<NestedCode<'_>> = example_results();
		assert_eq!(results[0].type_name(), "if-statement");
		assert_eq!(results[0].depth(), 0);
		assert_eq!(results.iter().filter(|code| code.type_name() == "if-statement").count(), 2);
		assert_eq!(results.iter().filter(|code| code.type_name() == "print-statement").count(), 1);
	}

	#[test]
	fn test_recursive() {
		let results:Vec<NestedCode<'_>> = example_results();
		assert_eq!(results.iter().filter(|code| code.type_name() == "scope").count(), 4);
		assert_eq!(results.iter().filter(|code| code.type_name() == "scope").map(|code| code.depth()).collect::<Vec<usize>>(), vec![0, 1, 3, 0]);
	}

	#[test]
	fn test_include_unmatched() {
		let results:Vec<NestedCode<'_>> = example_parser().include_unmatched(true).parse(EXAMPLE_TEXT);
		assert_eq!(results[0].type_name(), UNMATCHED_WHITE_SPACE_NAME);
		assert_eq!(results[0].depth(), 0);
		assert_eq!(results.iter().filter(|code| code.type_name() == "if-statement").count(), 2);
		assert_eq!(results.iter().filter(|code| code.type_name() == "print-statement").count(), 1);
		assert!(results.iter().find(|code| code.contents().contains("let thing_result = do_the_thing();")).is_some());
	}

	#[test]
	fn test_match_any_whitespace() {
		let results:Vec<NestedCode<'_>> = example_parser().match_any_white_space().parse(EXAMPLE_TEXT);
		assert_eq!(results[0].type_name(), "if-statement");
		assert_eq!(results[0].depth(), 0);
		assert_eq!(results.iter().filter(|code| code.type_name() == "if-statement").count(), 3);
		assert_eq!(results.iter().filter(|code| code.type_name() == "print-statement").count(), 1);
	}

	#[test]
	fn test_full_result() {
		let results:Vec<NestedCode<'_>> = example_parser().include_unmatched(true).match_any_white_space().parse(EXAMPLE_TEXT);
		let expected = [
			(0, UNMATCHED_WHITE_SPACE_NAME, None),
			(0, "if-statement", Some("if necessary ")),
			(0, "scope", None),
			(1, "comment", Some("// Makes the program do the expected thing.\n")),
			(1, "if-statement", Some("if thing_result.is_ok() ")),
			(1, "scope", None),
			(2, "print-statement", Some("println!(\"Successful thinging complete! Exited with error code {}.\", get_code());")),
			(3, "scope", Some("{}")),
			(0, UNMATCHED_WHITE_SPACE_NAME, None),
			(0, "if-statement", Some("if	 weirdly_spaced_bool ")),
			(0, "scope", None),
			(1, "comment", Some("// This comment contains white-space, but is not split up despite it's white-space end-tag.\n"))
		];
		for (index, (real, expected)) in results.iter().map(|result| (result.depth(), result.type_name(), result.contents())).zip(expected).enumerate() {
			println!("Result {index}");
			if let Some(expected_contents) = expected.2 {
				assert_eq!((real.0, real.1, real.2), (expected.0, expected.1, expected_contents));
			} else {
				assert_eq!((real.0, real.1), (expected.0, expected.1));
			}
		}
	}
}