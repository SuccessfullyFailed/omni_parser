use std::error::Error;
use file_ref::FileRef;
use super::{NestedCode, SegmentIdentification, SegmentIdentificationSource};



const HTML_REPLACEMENTS:&[(&str, &str)] = &[("<", "&lt;"), (">", "&gt;"), ("\"", "&quot;"), ("\n", "<br>"), ("\t", "<span class=\"spacer\"></span>")];
pub const UNMATCHED_SEGMENT_NAME:&str = "UNNAMED";



pub struct NestedCodeParser {
	identification:Vec<SegmentIdentification>,
	include_unmatched:bool
}
impl NestedCodeParser {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new parser.
	pub fn new(identification:Vec<&dyn SegmentIdentificationSource>) -> NestedCodeParser {
		NestedCodeParser {
			identification: identification.iter().map(|id_source| id_source.to_identification()).collect::<Vec<SegmentIdentification>>(),
			include_unmatched: false
		}
	}

	/// Return a version of self that includes unmatched contents.
	pub fn include_unmatched(mut self) -> Self {
		self.include_unmatched = true;
		self
	}



	/* USAGE METHODS */

	/// Parse a piece of code.
	pub fn parse<'a>(&self, contents:&'a str) -> Vec<NestedCode<'a>> {
		let mut depth:Vec<&SegmentIdentification> = Vec::new();
		let mut results:Vec<NestedCode> = Vec::new();

		// Loop through code.
		let mut last_unmatched_cursor:usize = 0;
		let mut cursor:usize = 0;
		while cursor < contents.len() {
			let mut found_anything:bool = false;

			// Find start-identification.
			let allow_sub_parse:bool = depth.last().map(|last_identification| last_identification.allow_sub_parse()).unwrap_or(true);
			if allow_sub_parse {
				for identification_set in &self.identification {
					if !found_anything && Self::tag_matches_contents(&contents, cursor, identification_set.open(), identification_set.open_escape()) {
						if self.include_unmatched && depth.is_empty() && last_unmatched_cursor != cursor {
							results.push(NestedCode::new(&contents, last_unmatched_cursor..cursor, 0, UNMATCHED_SEGMENT_NAME));
							last_unmatched_cursor = cursor;
						}
						depth.push(identification_set);
						results.push(NestedCode::new(contents, cursor..cursor, depth.len(), &identification_set.name()));
						found_anything = true;
						continue;
					}
				}
			}
			
			// Find end-identification.
			if !found_anything {
				if let Some(identification_set) = depth.last() {
					if Self::tag_matches_contents(&contents, cursor, identification_set.close(), &identification_set.close_escape()) {
						if let Some(result) = results.iter_mut().rev().find(|result_set| result_set.type_name() == identification_set.name() && result_set.start() == result_set.end()) {
							result.set_end(cursor + identification_set.close().len());
						}
						found_anything = true;
					}
				}
				if found_anything {
					depth.remove(depth.len() - 1);
				}
			}

			// Increment cursor.
			cursor += 1;
		}

		// Add trail.
		if !depth.is_empty() {
			while !depth.is_empty() {
				let depth_node:&SegmentIdentification = depth.remove(depth.len() - 1);
				if let Some(result) = results.iter_mut().rev().find(|result_set| result_set.type_name() == depth_node.name()) {
					result.set_end(cursor);
				}
			}
		} else if self.include_unmatched && last_unmatched_cursor != cursor {
			results.push(NestedCode::new(&contents, last_unmatched_cursor..cursor, 0, UNMATCHED_SEGMENT_NAME));
		}

		// Return results.
		results
	}

	/// Parse a piece of code and create a debug document.
	pub fn parse_debug_file(&self, contents:&str, output_file_path:&str) -> Result<(), Box<dyn Error>> {
		let results:Vec<NestedCode<'_>> = self.parse(contents);

		// Prepare segment-names debug display.
		let mut names:Vec<String> = self.identification.iter().map(|ident| ident.name().to_string()).collect::<Vec<String>>();
		names.dedup();

		// Create debug display file.
		let mut debug_code:String = String::new();
		let mut injections:Vec<(usize, Option<String>)> = results.iter().map(|result| [(result.start(), Some(result.type_name().to_string())), (result.end(), None)]).flatten().collect::<Vec<(usize, Option<String>)>>();
		injections.sort_by(|a, b| a.0.cmp(&b.0));
		let mut last_injection_position:usize = 0;
		for (position, name) in injections {
			let mut contents_split:String = contents[last_injection_position..position].to_string();
			for (find, replace) in HTML_REPLACEMENTS {
				contents_split = contents_split.replace(find, &replace);
			}
			debug_code += &contents_split;
			debug_code += &match name { Some(name) => format!("<span class=\"identifier {name}\">"), None => "</span>".to_string() };
			last_injection_position = position;
		}
		debug_code += &contents[last_injection_position..];
		debug_code += "\n<style>\n";
		debug_code += "\t.spacer { margin-left: 20px; }";
		debug_code += "\t.identifier { color: #0FF000; background-color: #0000FF; border: solid black 1px; }";
		debug_code += &names.iter().enumerate().map(|(shift, name)| format!("\t.{name} {} filter: hue-rotate({}deg); {}", '{', (shift as f32 / names.len() as f32 * 360.0) as usize, '}')).collect::<Vec<String>>().join("\n");
		debug_code += "\n</style>";

		// Write debug code to file.
		FileRef::new(&output_file_path).write(&debug_code)
	}

	/// Check if a specific tag matches a specific place in contents.
	fn tag_matches_contents(contents:&str, cursor:usize, tag:&str, escape:&Option<String>) -> bool {
		if contents.len() >= cursor + tag.len() && &contents[cursor..cursor + tag.len()] == tag {
			if let Some(escape) = escape {
				cursor >= escape.len() && &contents[cursor - escape.len()..cursor] != escape
			} else {
				true
			}
		} else {
			false
		}
	}
}