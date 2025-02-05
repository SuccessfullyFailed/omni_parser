use super::{ MatchMethod, LazyMatchSource, NestedSegment, SegmentIdentification };
use std::ops::Range;



pub const ROOT_NAME:&str = "ROOT";



pub struct NestedCodeParser {
	identification:Vec<SegmentIdentification>,
	ignore_white_space_segments:bool
}
impl NestedCodeParser {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new parser.
	pub fn new(identification:Vec<&dyn LazyMatchSource>) -> NestedCodeParser {
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
	pub fn parse<'b>(&self, contents:&'b str) -> NestedSegment {
		let mut parser:InnerNestedCodeParser<'_, 'b> = InnerNestedCodeParser::new(self, contents);
		parser.parse(None)
	}
}



pub struct InnerNestedCodeParser<'a, 'b> {
	origin:&'a NestedCodeParser,
	contents:&'b str,
	cursor:usize,
	unmatched_cursor:usize
}
impl<'a, 'b> InnerNestedCodeParser<'a, 'b> {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new inner code parser.
	pub fn new(origin:&'a NestedCodeParser, contents:&'b str) -> InnerNestedCodeParser<'a, 'b> {
		InnerNestedCodeParser {
			origin,
			contents,
			cursor: 0,
			unmatched_cursor: 0
		}
	}



	/* USAGE METHODS */

	/// Parse one single code snippet.
	fn parse(&mut self, scope_terminator:Option<(Range<usize>, &SegmentIdentification)>) -> NestedSegment {
		let mut children:Vec<NestedSegment> = Vec::new();
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
					return NestedSegment::new_code(&target_identification.name, &self.contents[open_tag_location.clone()], children, &self.contents[start..self.cursor]);
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
						children.push(self.parse(Some((start..self.cursor, &identification_set))));
						self.cursor -= 1; // The cursor loop is not broken, so the cursor will be incremented in the end of the loop.
						break;
					}
				}
			}

			self.cursor += 1;
		}

		// If target end not found, consider end of string the end of the tag.
		if let Some((open_tag_location, target_identification)) = scope_terminator {
			return NestedSegment::new_code(&target_identification.name, &self.contents[open_tag_location.clone()], children, &self.contents[self.unmatched_cursor..self.cursor]);
		}
		
		// No active expected end meant this is the root element.
		if let Some(from_unmatched) = self.code_from_unmatched() {
			children.push(from_unmatched);
		}
		NestedSegment::new_code(ROOT_NAME, &self.contents[0..0], children, &self.contents[0..0])
	}

	/// Create a snippet from unmatched code at the cursor.
	fn code_from_unmatched(&self) -> Option<NestedSegment> {
		if self.unmatched_cursor != self.cursor {
			let contents:&str = &self.contents[self.unmatched_cursor..self.cursor];
			let is_whitespace:bool = contents.chars().all(|char| char.is_whitespace());
			if !is_whitespace || !self.origin.ignore_white_space_segments {
				return Some(NestedSegment::new_contents(contents));
			}
		}
		None
	}

	/// Checks wether or not the contents at the cursor match the given tag. Returns the length of the match in contents or None.
	fn cursor_matches_tag(&self, matching_method:&MatchMethod) -> Option<usize> {
		match matching_method {
			MatchMethod::CharCompare(tag, escape) => self.cursor_matches_str_literal(tag, escape),
			MatchMethod::Method(method)  => method(&self.contents[self.cursor..]),
			MatchMethod::Regex(regex) => regex.find(&self.contents[self.cursor..]).map(|regex_match| regex_match.len())
		}
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn cursor_matches_str_literal(&self, tag:&str, escape:&Option<String>) -> Option<usize> {
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