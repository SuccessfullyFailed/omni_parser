use super::{ MatchMethod, MatchMethodSource, NestedCode, SegmentIdentification };
use std::{ error::Error, ops::Range };



pub const ROOT_NAME:&str = "ROOT";
pub const UNMATCHED_NAME:&str = "UNMATCHED";
pub const UNMATCHED_WHITESPACE_NAME:&str = "UNMATCHED_WHITESPACE";



#[derive(Clone)]
pub struct NestedCodeParser {
	identification:Vec<SegmentIdentification>,
	ignore_white_space_segments:bool
}
impl NestedCodeParser {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new parser.
	pub fn new(identification:Vec<&dyn MatchMethodSource>) -> NestedCodeParser {
		NestedCodeParser {
			identification: identification.iter().map(|id_source| id_source.to_identification()).collect::<Vec<SegmentIdentification>>(),
			ignore_white_space_segments: false
		}
	}

	/// Return a version of self that will not add segments that are only white-space.
	pub fn ignore_white_space_segments(mut self) -> Self {
		self.ignore_white_space_segments = true;
		self
	}
	


	/* USAGE METHODS */

	/// Parse some code.
	pub fn parse(&self, contents:&str) -> Result<NestedCode, Box<dyn Error>> {
		InnerNestedCodeParser::new(self, contents).parse(None)
	}
}



pub struct InnerNestedCodeParser<'a> {
	origin:&'a NestedCodeParser,
	contents:Vec<char>,
	cursor:usize,
	unmatched_cursor:usize
}
impl<'a, 'b> InnerNestedCodeParser<'a> {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new inner code parser.
	pub fn new(origin:&'a NestedCodeParser, contents:&str) -> InnerNestedCodeParser<'a> {
		InnerNestedCodeParser {
			origin,
			contents: contents.chars().collect(),
			cursor: 0,
			unmatched_cursor: 0
		}
	}



	/* USAGE METHODS */

	/// Parse one single code snippet.
	fn parse(&mut self, scope_terminator:Option<(Range<usize>, &SegmentIdentification)>) -> Result<NestedCode, Box<dyn Error>> {
		let mut children:Vec<NestedCode> = Vec::new();
		while self.cursor < self.contents.len() {
			
			// Try to match closing tag.
			if let Some((open_tag_location, target_identification)) = &scope_terminator {
				if let Some(match_length) = self.cursor_matches_tag(&target_identification.matching_method_close) {
					if let Some(from_unmatched) = self.code_from_unmatched() {
						children.push(from_unmatched);
					}
					let start:usize = self.cursor;
					self.cursor += match_length;
					self.unmatched_cursor = self.cursor;
					return Ok(NestedCode::new(&target_identification.name, true, &self.contents[open_tag_location.clone()], &self.contents[start..self.cursor], children));
				}
			}

			// Try to match opening tag.
			let allow_recurse:bool = scope_terminator.as_ref().map(|(_, identification)| identification.allow_sub_parse).unwrap_or(true);
			if allow_recurse {
				for identification_set in &self.origin.identification {
					if let Some(match_length) = self.cursor_matches_tag(&identification_set.matching_method_open) {
						if let Some(from_unmatched) = self.code_from_unmatched() {
							children.push(from_unmatched);
						}
						let start:usize = self.cursor;
						self.cursor += match_length;
						self.unmatched_cursor = self.cursor;
						children.push(self.parse(Some((start..self.cursor, &identification_set)))?);
						self.cursor -= 1; // The cursor loop is not broken, so the cursor will be incremented in the end of the loop.
						break;
					}
				}
			}

			self.cursor += 1;
		}

		// Target end not found.
		if let Some((open_tag_location, target_identification)) = scope_terminator {
			let line_break_locations:Vec<usize> = self.contents[..open_tag_location.start].iter().enumerate().filter(|(_, character)| **character == '\n' || **character == '\r').map(|(index, _)| index).collect::<Vec<usize>>();
			Err(format!("Could not find end of {} starting at {}:{}", &target_identification.name, line_break_locations.len(), open_tag_location.start - line_break_locations.last().unwrap_or(&0)).into())
		} else {
			if let Some(from_unmatched) = self.code_from_unmatched() {
				children.push(from_unmatched);
			}
			Ok(NestedCode::new(ROOT_NAME, false, &[], &[], children))
		}
	}

	/// Create a snippet from unmatched code at the cursor.
	fn code_from_unmatched(&self) -> Option<NestedCode> {
		if self.unmatched_cursor != self.cursor {
			let contents:&[char] = &self.contents[self.unmatched_cursor..self.cursor];
			let is_white_space:bool = contents.iter().all(|character| character.is_whitespace());
			if is_white_space && self.origin.ignore_white_space_segments {
				return None;
			}
			let name:&str = if is_white_space { UNMATCHED_WHITESPACE_NAME } else { UNMATCHED_NAME };
			Some(NestedCode::new(name, false, contents, &[], Vec::new()))
		} else {
			None
		}
	}

	/// Checks wether or not the contents at the cursor match the given tag. Returns the length of the match in contents or None.
	fn cursor_matches_tag(&self, matching_method:&MatchMethod) -> Option<usize> {
		match matching_method {
			MatchMethod::CharCompare(tag, escape) => self.cursor_matches_tag_literal(tag, escape)
		}
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn cursor_matches_tag_literal(&self, tag:&[char], escape:&Option<Vec<char>>) -> Option<usize> {
		let tag_end:usize = self.cursor + tag.len();
		if self.contents.len() >= tag_end && &self.contents[self.cursor..tag_end] == tag {
			if let Some(escape) = escape {
				let mut escaped:bool = false;
				let mut cursor:usize = self.cursor;
				while cursor >= escape.len() && &self.contents[cursor - escape.len()..cursor] == escape {
					escaped = !escaped;
					cursor -= escape.len();
				}
				if !escaped {
					return Some(tag.len());
				}
			} else {
				return Some(tag.len());
			}
		}
		None
	}
}