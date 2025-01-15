use std::{ fmt::{ self, Debug }, ops::{ Index, IndexMut, Range } };



pub struct NestedCodeSegment {
	type_name:String,
	matched:bool,
	outer_contents:String,
	open_tag_len:usize,
	close_tag_len:usize,
	sub_segments:Vec<NestedCodeSegment>
}
impl NestedCodeSegment {

	/* CONSTRUCTOR METHODS */

	/// Create a new NestedCode.
	pub fn new(type_name:&str, matched:bool, source_contents:&str, open_tag_position:Range<usize>, close_tag_position:Range<usize>, sub_segments:Vec<NestedCodeSegment>) -> NestedCodeSegment {
		NestedCodeSegment {
			type_name: type_name.to_string(),
			matched,
			outer_contents: source_contents[open_tag_position.start..close_tag_position.end].to_string(),
			open_tag_len: open_tag_position.end - open_tag_position.start,
			close_tag_len: close_tag_position.end - close_tag_position.start,
			sub_segments
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the type name.
	pub fn type_name(&self) -> &str {
		&self.type_name
	}

	/// Wether or not the code was matched.
	pub fn matched(&self) -> bool {
		self.matched
	}

	/// Get the open tag.
	pub fn open_tag(&self) -> &str {
		&self.outer_contents[..self.open_tag_len]
	}

	/// Get the closing tag.
	pub fn close_tag(&self) -> &str {
		&self.outer_contents[self.outer_contents.len() - self.close_tag_len..]
	}



	/* CONTENT GETTER METHODS */

	/// Get the outer contents of the full segment.
	pub fn outer_contents(&self) -> &str {
		&self.outer_contents
	}

	/// Get the inner contents of the full segment (without tags, only the code in between).
	pub fn inner_contents(&self) -> &str {
		&self.outer_contents[self.open_tag_len..self.outer_contents.len() - self.close_tag_len]
	}

	/// Get the sub-segments.
	pub fn sub_segments(&self) -> &Vec<NestedCodeSegment> {
		&self.sub_segments
	}

	/// Get all matched sub-segments.
	pub fn sub_segments_matched(&self) -> Vec<&NestedCodeSegment> {
		self.sub_segments.iter().filter(|segment| segment.matched).collect()
	}

	/// Get all unmatched sub-segments.
	pub fn sub_segments_unmatched(&self) -> Vec<&NestedCodeSegment> {
		self.sub_segments.iter().filter(|segment| !segment.matched).collect()
	}

	/// Get all code segments at a specific depth.
	pub fn sub_segments_at_depth(&self, depth:usize) -> Vec<&NestedCodeSegment> {
		self._sub_segments_at_depth(0, depth)
	}
	fn _sub_segments_at_depth(&self, current_depth:usize, target_depth:usize) -> Vec<&NestedCodeSegment> {
		if current_depth == target_depth {
			vec![self]
		} else if current_depth > target_depth {
			Vec::new()
		} else {
			let child_depth:usize = current_depth + 1;
			self.sub_segments.iter().map(|child| child._sub_segments_at_depth(child_depth, target_depth)).flatten().collect()
		}
	}

	/// Get a flat list of self and all children and their depth.
	pub fn flat(&self) -> Vec<(usize, &NestedCodeSegment)> {
		self._flat(0)
	}
	fn _flat(&self, current_depth:usize) -> Vec<(usize, &NestedCodeSegment)> {
		let mut flattened_with_depth:Vec<(usize, &NestedCodeSegment)> = vec![(current_depth, self)];
		let child_depth:usize = current_depth + 1;
		flattened_with_depth.extend(self.sub_segments.iter().map(|child| child._flat(child_depth)).flatten().collect::<Vec<(usize, &NestedCodeSegment)>>());
		flattened_with_depth
	}


	/// Get a flat list of all matched sub-segments.
	pub fn flat_segments_matched(&self) -> Vec<(usize, &NestedCodeSegment)> {
		let mut segments:Vec<(usize, &NestedCodeSegment)> = self.flat();
		segments.retain(|segment| segment.1.matched);
		segments
	}

	/// Get a flat list of all unmatched sub-segments.
	pub fn flat_sub_segments_unmatched(&self) -> Vec<(usize, &NestedCodeSegment)> {
		let mut segments:Vec<(usize, &NestedCodeSegment)> = self.flat();
		segments.retain(|segment| !segment.1.matched);
		segments
	}
}
impl Debug for NestedCodeSegment {
	fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
		const PADDING:&str = "\t";
		write!(
			f,
			"{} {}\n{}\n{}",
			self.type_name(),
			'{',
				self.sub_segments.iter().map(|code|
					format!("{:?}", code).split('\n').map(|line| PADDING.to_owned() + line).collect::<Vec<String>>().join("\n")
				).collect::<Vec<String>>().join("\n"),
			'}'
		)
	}
}
impl Index<usize> for NestedCodeSegment {
	type Output = NestedCodeSegment;
	fn index(&self, index:usize) -> &Self::Output {
		&self.sub_segments[index]
	}
}
impl IndexMut<usize> for NestedCodeSegment {
	fn index_mut(&mut self, index:usize) -> &mut Self::Output {
		&mut self.sub_segments[index]
	}
}
impl Index<&str> for NestedCodeSegment {
	type Output = NestedCodeSegment;
	fn index(&self, index:&str) -> &Self::Output {
		self.sub_segments.iter().find(|child| child.type_name() == index).unwrap()
	}
}
impl IndexMut<&str> for NestedCodeSegment {
	fn index_mut(&mut self, index:&str) -> &mut Self::Output {
		self.sub_segments.iter_mut().find(|child| child.type_name() == index).unwrap()
	}
}