use super::{ NestedCode, SegmentIdentification, SegmentIdentificationSource };
use std::error::Error;
use file_ref::FileRef;



const HTML_REPLACEMENTS:&[(&str, &str)] = &[("<", "&lt;"), (">", "&gt;"), ("\"", "&quot;"), ("\r\n", "\n"), ("\r", "\n"), ("\n", "<br>"), ("\t", "<span class=\"spacer\"></span>")];
pub const UNMATCHED_SEGMENT_NAME:&str = "UNNAMED";
pub const UNMATCHED_WHITE_SPACE_NAME:&str = "WHITESPACE";



#[derive(Clone)]
pub struct NestedCodeParser {
	identification:Vec<SegmentIdentification>,
	include_unmatched:bool,
	include_unmatched_white_space:bool,
	match_any_white_space:bool
}
impl NestedCodeParser {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new parser.
	pub fn new(identification:Vec<&dyn SegmentIdentificationSource>) -> NestedCodeParser {
		NestedCodeParser {
			identification: identification.iter().map(|id_source| id_source.to_identification()).collect::<Vec<SegmentIdentification>>(),
			include_unmatched: false,
			include_unmatched_white_space: false,
			match_any_white_space: false
		}
	}

	/// Return a version of self that includes unmatched contents.
	pub fn include_unmatched(mut self, include_unmatched_white_space:bool) -> Self {
		self.include_unmatched = true;
		self.include_unmatched_white_space = include_unmatched_white_space;
		self
	}

	/// Return a version of self that, when matching strings, matches any white-spaces equally.
	pub fn match_any_white_space(mut self) -> Self {
		self.match_any_white_space = true;
		self
	}



	/* USAGE METHODS */

	/// Parse a piece of code.
	pub fn parse<'a>(&self, contents:&'a str) -> Vec<NestedCode<'a>> {
		let mut results:Vec<NestedCode> = Vec::new();

		// Loop through code.
		let mut depth:Vec<&SegmentIdentification> = Vec::new();
		let mut last_unmatched_cursor:usize = 0;
		let mut cursor:usize = 0;
		while cursor < contents.len() {
			let mut found_anything:bool = false;

			// Find start-identification.
			let allow_sub_parse:bool = depth.last().map(|last_identification| last_identification.allow_sub_parse()).unwrap_or(true);
			if allow_sub_parse {
				for identification_set in &self.identification {
					if !found_anything {
						if let Some(match_length) = self.tag_matches_contents(&contents, cursor, identification_set.open(), identification_set.open_escape()) {
							if self.include_unmatched && depth.is_empty() && last_unmatched_cursor != cursor {
								let all_white_space:bool = contents[last_unmatched_cursor..cursor].chars().all(|character| character.is_whitespace());
								if !all_white_space || self.include_unmatched_white_space {
									results.push(NestedCode::new(&contents, last_unmatched_cursor..cursor, 0, if all_white_space { UNMATCHED_WHITE_SPACE_NAME } else { UNMATCHED_SEGMENT_NAME }));
									last_unmatched_cursor = cursor;
								}
							}
							results.push(NestedCode::new(contents, cursor..cursor, depth.len(), &identification_set.name()));
							depth.push(identification_set);
							cursor += match_length - 1;
							found_anything = true;
						}
					}
				}
			}
			
			// Find end-identification.
			if !found_anything {
				if let Some(identification_set) = depth.last().cloned() {
					if let Some(match_length) = self.tag_matches_contents(&contents, cursor, identification_set.close(), &identification_set.close_escape()) {
						for result in results.iter_mut().rev() {
							if result.start() == result.end() {
								result.set_end(cursor + match_length);
							}
							if result.type_name() == identification_set.name() {
								break;
							}
						}
						depth.remove(depth.len() - 1);
						if depth.is_empty() {
							last_unmatched_cursor = cursor + match_length;
						}
						cursor += match_length - 1;
					}
				}
			}

			// Increment cursor.
			cursor += 1;
		}

		// Add trail.
		for result in &mut results {
			if result.start() == result.end() {
				result.set_end(cursor);
				last_unmatched_cursor = cursor;
			}
		}
		if self.match_any_white_space && last_unmatched_cursor != cursor {
			let all_white_space:bool = contents[last_unmatched_cursor..cursor].chars().all(|character| character.is_whitespace());
			if !all_white_space || self.include_unmatched_white_space {
				results.push(NestedCode::new(&contents, last_unmatched_cursor..cursor, 0, if all_white_space { UNMATCHED_WHITE_SPACE_NAME } else { UNMATCHED_SEGMENT_NAME }));
			}
		}

		// Return results.
		results
	}

	/// Parse a piece of code and create a debug document.
	pub fn parse_debug_file(&self, contents:&str, output_file_path:&str) -> Result<(), Box<dyn Error>> {
		let results:Vec<NestedCode<'_>> = self.clone().include_unmatched(true).parse(contents);

		// Prepare segment-names.
		let mut names:Vec<String> = self.identification.iter().map(|ident| ident.name().to_string()).collect::<Vec<String>>();
		names.dedup();

		// Create debug display file.
		let mut debug_code:String = String::new();
		if !results.is_empty() {
			let mut depth:usize = 0;
			for result in &results {
				depth += 1;

				// Encode result contents.
				let mut contents:String = result.contents().to_owned();
				for (find, replace) in HTML_REPLACEMENTS {
					contents = contents.replace(find, replace);
				}

				// Drop down to correct depth.
				while depth > result.depth() {
					debug_code += "</span>";
					depth -= 1;
				}

				// Create HTML tag for contents.
				debug_code += &format!("<span class=\"identifier {}\">", result.type_name());
				debug_code += &contents;
			}
			for _ in 0..depth {
				debug_code += "</span>";
			}
		}
		debug_code += "\n<style>\n";
		debug_code += "\t.spacer { margin-left: 20px; }";
		debug_code += "\t.identifier { color: #0FF000; background-color: #0000FF; border: solid black 1px; }";
		debug_code += &names.iter().enumerate().map(|(shift, name)| format!("\t.{name} {} filter: hue-rotate({}deg); {}", '{', (shift as f32 / names.len() as f32 * 360.0) as usize, '}')).collect::<Vec<String>>().join("\n");
		debug_code += "\n</style>";

		// Write debug code to file.
		FileRef::new(&output_file_path).write(&debug_code)
	}

	/// Check if a specific tag matches a specific place in contents. Returns the length of the match.
	pub(crate) fn tag_matches_contents(&self, contents:&str, cursor:usize, tag:&str, escape:&Option<String>) -> Option<usize> {
		if self.match_any_white_space {
			self.tag_matches_contents_whitespace_irrelevant(contents, cursor, tag, escape)
		} else {
			self.tag_matches_contents_simple(contents, cursor, tag, escape)
		}
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn tag_matches_contents_simple(&self, contents:&str, cursor:usize, tag:&str, escape:&Option<String>) -> Option<usize> {
		let tag_end:usize = cursor + tag.len();
		if contents.len() >= tag_end && &contents[cursor..tag_end] == tag {
			if let Some(escape) = escape {
				if cursor >= escape.len() && &contents[cursor - escape.len()..cursor] != escape {
					return Some(tag.len());
				}
			} else {
				return Some(tag.len());
			}
		}
		None
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn tag_matches_contents_whitespace_irrelevant(&self, contents:&str, cursor:usize, tag:&str, escape:&Option<String>) -> Option<usize> {
		let contents_chars:Vec<char> = contents[cursor..].chars().collect::<Vec<char>>();
		let tag_chars:Vec<char> = tag.chars().collect();

		// If tag is all white-space, simply check if the next white-space contains all tag chars.
		if tag_chars.iter().all(|char| char.is_whitespace()) {
			let mut tag_char_index:usize = 0;
			for (character_index, character) in contents_chars.iter().enumerate() {
				if !character.is_whitespace() {
					break;
				}
				if character == &tag_chars[tag_char_index] {
					tag_char_index += 1;
					if tag_char_index >= tag_chars.len() {
						return Some(character_index + 1);
					}
				}
			}
			return None;
		}
		
		// Keep comparing contents to tag at indexes.
		let mut indexes:[usize; 2] = [0, 0];
		while indexes[0] < contents_chars.len() && indexes[1] < tag_chars.len() {

			// Skip over white-space.
			if contents_chars[indexes[0]].is_whitespace() && tag_chars[indexes[1]].is_whitespace() {
				while indexes[0] < contents_chars.len() && contents_chars[indexes[0]].is_whitespace() {
					indexes[0] += 1;
				}
				while indexes[1] < tag_chars.len() && tag_chars[indexes[1]].is_whitespace() {
					indexes[1] += 1;
				}
			}

			// If not both white-space, compare.
			else {
				if contents_chars[indexes[0]] != tag_chars[indexes[1]] {
					return None;
				}
				indexes[0] += 1;
				indexes[1] += 1;
			}
		}

		// Do the same thing reversed for the escape tag.
		if let Some(escape) = escape {
			if self.tag_matches_contents(&contents[..cursor], cursor - 1, &escape.chars().rev().collect::<String>(), &None).is_some() {
				return None;	
			}
		}

		// If tag indexer has reached end, that means it's a full match.
		if indexes[1] >= tag_chars.len() {
			Some(indexes[0])
		} else {
			None
		}
	}
}