use std::{ fmt::{ self, Debug }, ops::{ Index, IndexMut } };

use super::ROOT_NAME;



pub(super) const CONTENTS_NAME:&str = "contents";
pub(super) const WHITESPACE_NAME:&str = "whitespace";
#[derive(Clone, PartialEq, Eq)]
pub struct NestedSegmentCode { pub type_name:String, pub open_tag:String, pub sub_segments:Vec<NestedSegment>, pub close_tag:String }
#[derive(Clone, PartialEq, Eq)]
pub enum NestedSegment { Code(NestedSegmentCode), Contents(String), WhiteSpace(String) }
impl NestedSegment {

	/* CONSTRUCTOR METHODS */

	/// Create a new code segment.
	pub fn new_code(type_name:&str, open_tag:&str, sub_segments:Vec<NestedSegment>, close_tag:&str) -> NestedSegment {
		NestedSegment::Code(
			NestedSegmentCode {
				type_name: type_name.to_string(),
				open_tag: open_tag.to_string(),
				sub_segments: sub_segments,
				close_tag: close_tag.to_string()
			}
		)
	}

	/// Create a new contents segment.
	pub fn new_contents(contents:&str) -> NestedSegment {
		if contents.chars().all(|char| char.is_whitespace()) {
			NestedSegment::WhiteSpace(contents.to_string())
		} else {
			NestedSegment::Contents(contents.to_string())
		}
	}

	/// Return self without whitespace.
	pub fn without_whitespace(mut self) -> Self {
		self.remove_whitespace();
		self
	}

	/// Build a code segment from a flat list.
	pub fn from_flat(mut segments:Vec<(usize, NestedSegment)>) -> Option<NestedSegment> {
		if segments.is_empty() {
			return None;
		}
		let mut cursor:usize = segments.len() - 1;
		Self::process_flat_list_to_tree(&mut segments, &mut cursor, 0);
		match segments.len() {
			0 => None,
			1 => Some(segments.remove(0).1),
			_ => Some(NestedSegment::new_code(ROOT_NAME, "", segments.iter().map(|(_, segment)| segment.clone()).collect::<Vec<NestedSegment>>(), ""))
		}
	}
	fn process_flat_list_to_tree(segments:&mut Vec<(usize, NestedSegment)>, cursor:&mut usize, parse_until_depth:usize) {

		// From last to first, try combine children.
		while !segments.is_empty() && *cursor > 0 && segments[*cursor].0 >= parse_until_depth {
			let current_depth:usize = segments[*cursor].0;
			let previous_depth:usize = segments[*cursor - 1].0;

			// If depth increased, make current segment and siblings children of previous segment.
			if current_depth > previous_depth {
				while *cursor < segments.len() && segments[*cursor].0 > previous_depth {
					let child:NestedSegment = segments.remove(*cursor).1;
					segments[*cursor - 1].1.sub_segments_mut().push(child);
				}
			}
			
			// If depth decreased, parse deeper level first.
			else if current_depth < previous_depth {
				*cursor -= 1;
				Self::process_flat_list_to_tree(segments, cursor, previous_depth);
				*cursor += 1;
				continue;
			}

			*cursor -= 1;
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the type-name of the segment.
	pub fn type_name(&self) -> &str {
		match self {
			NestedSegment::Code(code) => &code.type_name,
			NestedSegment::Contents(_) => CONTENTS_NAME,
			NestedSegment::WhiteSpace(_) => WHITESPACE_NAME,
		}
	}

	/// Wether or not the type is code.
	pub fn is_code(&self) -> bool {
		matches!(self, NestedSegment::Code(_))
	}

	/// Wether or not the type is contents.
	pub fn is_contents(&self) -> bool {
		matches!(self, NestedSegment::Contents(_))
	}

	/// Wether or not the type is whitespace.
	pub fn is_whitespace(&self) -> bool {
		matches!(self, NestedSegment::WhiteSpace(_))
	}

	/// Wether or not the struct has no contents.
	pub fn is_empty(&self) -> bool {
		match self {
		    NestedSegment::Code(code) => code.open_tag.is_empty() && code.sub_segments.is_empty() && code.close_tag.is_empty(),
		    NestedSegment::Contents(contents) => contents.is_empty(),
		    NestedSegment::WhiteSpace(contents) => contents.is_empty(),
		}
	}

	/// Get the segments' sub-segments.
	pub fn sub_segments(&self) -> &[NestedSegment] {
		match self {
			NestedSegment::Code(code) => &code.sub_segments,
			_ => &[]
		}
	}

	/// Get the segments' sub-segments mutable.
	pub fn sub_segments_mut(&mut self) -> &mut Vec<NestedSegment> {
		match self {
			NestedSegment::Code(code) => &mut code.sub_segments,
			_ => {
				static mut FAKE_LIST:Vec<NestedSegment> = Vec::new();
				unsafe { FAKE_LIST.as_mut() }
			}
		}
	}



	/* FLATTENING METHODS */

	/// Turn self into a flat owned list.
	pub fn to_flat(self) -> Vec<(usize, NestedSegment)> {
		let mut segments:Vec<(usize, NestedSegment)> = Vec::new();
		self._to_flat(&mut segments, 0);
		segments
	}
	fn _to_flat(mut self, segments:&mut Vec<(usize, NestedSegment)>, depth:usize) {
		let children:Vec<NestedSegment> = match &mut self { NestedSegment::Code(code) => code.sub_segments.drain(..).collect(), _ => Vec::new() };
		segments.push((depth, self));
		for child in children {
			child._to_flat(segments, depth + 1);
		}
	}

	/// Recursively get filtered segments and sub-segments flattened with their depth.
	pub fn flat_filtered<T>(&self, filter:T) -> Vec<(usize, &NestedSegment)> where T:Fn(usize, &NestedSegment) -> bool {
		let mut segments:Vec<(usize, &NestedSegment)> = Vec::new();
		self._flat_filtered(&mut segments, 0, &filter);
		segments
	}
	fn _flat_filtered<'a>(&'a self, result_list:&mut Vec<(usize, &'a NestedSegment)>, depth:usize, filter:&dyn Fn(usize, &NestedSegment) -> bool) {
		if filter(depth, self) {
			result_list.push((depth, self));
		}
		for sub_segment in self.sub_segments() {
			sub_segment._flat_filtered(result_list, depth + 1, filter);
		}
	}

	/// Recursively get filtered segments and sub-segments mutable, flattened with their depth.
	pub fn flat_filtered_mut<T>(&mut self, filter:T) -> Vec<(usize, &mut NestedSegment)> where T:Fn(usize, &NestedSegment) -> bool {
		let mut segments:Vec<(usize, &mut NestedSegment)> = Vec::new();
		self._flat_filtered_mut(&mut segments, 0, &filter);
		segments
	}
	fn _flat_filtered_mut<'a>(&'a mut self, result_list:&mut Vec<(usize, &'a mut NestedSegment)>, depth:usize, filter:&dyn Fn(usize, &NestedSegment) -> bool) {
		if filter(depth, self) {
			let self_pointer:*mut NestedSegment = self as *mut NestedSegment;
			result_list.push((depth, unsafe { &mut *self_pointer }));
		}
		for sub_segment in self.sub_segments_mut() {
			sub_segment._flat_filtered_mut(result_list, depth + 1, filter);
		}
	}

	/// Recursively get the segments and sub-segments flattened with their depth.
	pub fn flat(&self) -> Vec<(usize, &NestedSegment)> {
		self.flat_filtered(|_, _| true)
	}

	/// Recursively get the segments and sub-segments mutable, flattened with their depth.
	pub fn flat_mut(&mut self) -> Vec<(usize, &mut NestedSegment)> {
		self.flat_filtered_mut(|_, _| true)
	}

	
	/// Recursively get code segments and sub-segments mutable, flattened with their depth.
	pub fn flat_code_filtered<T>(&self, filter:T) -> Vec<(usize, &NestedSegmentCode)> where T:Fn(usize, &NestedSegmentCode) -> bool {
		let mut results:Vec<(usize, &NestedSegmentCode)> = Vec::new();
		for (depth, segment) in self.flat_filtered(|_, segment| segment.is_code()) {
			if let NestedSegment::Code(code) = segment {
				if filter(depth, &code) {
					results.push((depth, code));
				}
			}
		}
		results
	}

	/// Recursively get code segments and sub-segments mutable, flattened with their depth.
	pub fn flat_code_filtered_mut<T>(&mut self, filter:T) -> Vec<(usize, &mut NestedSegmentCode)> where T:Fn(usize, &NestedSegmentCode) -> bool {
		let mut results:Vec<(usize, &mut NestedSegmentCode)> = Vec::new();
		for (depth, segment) in self.flat_filtered_mut(|_, segment| segment.is_code()) {
			if let NestedSegment::Code(code) = segment {
				if filter(depth, &code) {
					results.push((depth, code));
				}
			}
		}
		results
	}

	/// Recursively get code segments and sub-segments flattened with their depth.
	pub fn flat_code(&self) -> Vec<(usize, &NestedSegmentCode)> {
		self.flat_code_filtered(|_, _| true)
	}

	/// Recursively get code segments and sub-segments mutable, flattened with their depth.
	pub fn flat_code_mut(&mut self) -> Vec<(usize, &mut NestedSegmentCode)> {
		self.flat_code_filtered_mut(|_, _| true)
	}
	


	/* ACTION METHODS */

	/// Remove all white-space from the tree.
	pub fn remove_whitespace(&mut self) {
		self.retain_child_segments(|_, segment| !segment.is_whitespace());
	}

	/// Retain all child segments in the recursive tree from a filter based on depth and the segment.
	pub fn retain_child_segments<T>(&mut self, filter:T) where T:Fn(usize, &NestedSegment) -> bool {
		self._retain_child_segments(0, &filter);
	}
	fn _retain_child_segments<T>(&mut self, depth:usize, filter:&T) where T:Fn(usize, &NestedSegment) -> bool {
		match self {
			NestedSegment::Code(code) => {
				let mut index:usize = 0;
				while index < code.sub_segments.len() {
					let segment:&mut NestedSegment = &mut code.sub_segments[index];
					if filter(depth, segment) {
						segment._retain_child_segments(depth + 1, filter);
						index += 1;
					} else {
						code.sub_segments.remove(index);
					}
				}
			},
			_ => {}
		}
	}

	/// Create a string from sub-contents only.
	pub fn sub_contents_to_string(&self) -> String {
		self.sub_segments().iter().map(|sub_segment| sub_segment.to_string()).collect::<Vec<String>>().join("")
	}
}
impl ToString for NestedSegment {
	fn to_string(&self) -> String {
		match self {
			NestedSegment::Code(code) => format!("{}{}{}", code.open_tag, code.sub_segments.iter().map(|segment| segment.to_string()).collect::<Vec<String>>().join(""), code.close_tag),
			NestedSegment::Contents(contents) => contents.clone(),
			NestedSegment::WhiteSpace(whitespace) => whitespace.clone()
		}
	}
}
impl Debug for NestedSegment {
	fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
		const PADDING:&str = "\t";
		write!(
			f,
			"{} {}\n{}\n{}",
			self.type_name(),
			'{',
				self.sub_segments().iter().map(|code|
					format!("{:?}", code).split('\n').map(|line| PADDING.to_owned() + line).collect::<Vec<String>>().join("\n")
				).collect::<Vec<String>>().join("\n"),
			'}'
		)
	}
}
impl Index<usize> for NestedSegment {
	type Output = NestedSegment;
	fn index(&self, index:usize) -> &Self::Output {
		&self.sub_segments()[index]
	}
}
impl IndexMut<usize> for NestedSegment {
	fn index_mut(&mut self, index:usize) -> &mut Self::Output {
		&mut self.sub_segments_mut()[index]
	}
}