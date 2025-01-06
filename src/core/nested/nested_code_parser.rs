use super::{ NestedCode, SegmentIdentification, SegmentIdentificationSource };
use std::{ error::Error, ops::Range };



pub const ROOT_NAME:&str = "ROOT";
pub const UNMATCHED_NAME:&str = "UNMATCHED";
pub const UNMATCHED_WHITESPACE_NAME:&str = "UNMATCHED_WHITESPACE";



#[derive(Clone)]
pub struct NestedCodeParser {
	identification:Vec<SegmentIdentification>,
	match_any_white_space:bool
}
impl NestedCodeParser {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new parser.
	pub fn new(identification:Vec<&dyn SegmentIdentificationSource>) -> NestedCodeParser {
		NestedCodeParser {
			identification: identification.iter().map(|id_source| id_source.to_identification()).collect::<Vec<SegmentIdentification>>(),
			match_any_white_space: false
		}
	}

	/// Return a version of self that, when matching strings, matches any white-spaces equally.
	pub fn match_any_white_space(mut self) -> Self {
		self.match_any_white_space = true;
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
				if let Some(match_length) = self.cursor_matches_tag(&target_identification.close, &target_identification.close_escape) {
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
					if let Some(match_length) = self.cursor_matches_tag(&identification_set.open, &identification_set.open_escape) {

						// Create snippet from unmatched code.
						if self.unmatched_cursor != self.cursor {
							let contents:&[char] = &self.contents[self.unmatched_cursor..self.cursor];
							let name:&str = if contents.iter().all(|character| character.is_whitespace()) { UNMATCHED_WHITESPACE_NAME } else { UNMATCHED_NAME };
							children.push(NestedCode::new(name, false, contents, &self.contents[self.cursor..self.cursor], Vec::new()));
						}

						// Create snippet from match.
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
			Ok(NestedCode::new(ROOT_NAME, false, &[], &[], children))
		}
	}

	/// Checks wether or not the contents at the cursor match the given tag. Returns the length of the match in contents or None.
	fn cursor_matches_tag(&self, tag:&[char], escape:&Option<Vec<char>>) -> Option<usize> {
		if self.origin.match_any_white_space {
			self.tag_matches_contents_match_any_whitespace(&self.contents, self.cursor, tag, escape)
		} else {
			self.cursor_matches_tag_literal(tag, escape)
		}
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn cursor_matches_tag_literal(&self, tag:&[char], escape:&Option<Vec<char>>) -> Option<usize> {
		let tag_end:usize = self.cursor + tag.len();
		if self.contents.len() >= tag_end && &self.contents[self.cursor..tag_end] == tag {
			if let Some(escape) = escape {
				if self.cursor >= escape.len() && &self.contents[self.cursor - escape.len()..self.cursor] != escape {
					return Some(tag.len());
				}
			} else {
				return Some(tag.len());
			}
		}
		None
	}

	/// Check if a specific tag matches a specific place in contents by simply checking if the strings are the same. Returns the length of the match.
	fn tag_matches_contents_match_any_whitespace(&self, contents:&[char], cursor:usize, tag:&[char], escape:&Option<Vec<char>>) -> Option<usize> {
		let sub_contents:&[char] = &contents[cursor..];

		// If tag is all white-space, simply check if the next white-space contains all tag chars.
		if tag.iter().all(|char| char.is_whitespace()) {
			let mut tag_char_index:usize = 0;
			for (character_index, character) in sub_contents.iter().enumerate() {
				if !character.is_whitespace() {
					break;
				}
				if character == &tag[tag_char_index] {
					tag_char_index += 1;
					if tag_char_index >= tag.len() {
						return Some(character_index + 1);
					}
				}
			}
			return None;
		}
		
		// Keep comparing contents to tag at indexes.
		let mut indexes:[usize; 2] = [0, 0];
		while indexes[0] < sub_contents.len() && indexes[1] < tag.len() {

			// Skip over white-space.
			if sub_contents[indexes[0]].is_whitespace() && tag[indexes[1]].is_whitespace() {
				while indexes[0] < sub_contents.len() && sub_contents[indexes[0]].is_whitespace() {
					indexes[0] += 1;
				}
				while indexes[1] < tag.len() && tag[indexes[1]].is_whitespace() {
					indexes[1] += 1;
				}
			}

			// If not both white-space, compare.
			else {
				if sub_contents[indexes[0]] != tag[indexes[1]] {
					return None;
				}
				indexes[0] += 1;
				indexes[1] += 1;
			}
		}

		// Do the same thing reversed for the escape tag.
		if let Some(escape) = escape {
			if self.tag_matches_contents_match_any_whitespace(&contents[..cursor].iter().rev().cloned().collect::<Vec<char>>(), 0, &escape.iter().rev().cloned().collect::<Vec<char>>(), &None).is_some() {
				return None;	
			}
		}

		// If tag indexer has reached end, that means it's a full match.
		if indexes[1] >= tag.len() {
			Some(indexes[0])
		} else {
			None
		}
	}
}