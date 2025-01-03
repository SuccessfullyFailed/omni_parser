use std::ops::Range;



pub struct NestedCode<'a> {
	start:usize,
	end:usize,
	depth:usize,
	contents:&'a str,
	identifier_name:String
}
impl<'a> NestedCode<'a> {

	/* CONSTRUCTOR METHODS */

	/// Create a new code segment.
	pub fn new(contents:&'a str, range:Range<usize>, depth:usize, identifier_name:&str) -> NestedCode<'a> {
		NestedCode {
			start: range.start,
			end: range.end,
			depth,
			contents: &contents[range],
			identifier_name: identifier_name.to_string()
		}
	}



	/* PROPERTY SETTER METHODS */

	/// Set the NestedCode's end.
	pub(super) fn set_end(&mut self, end:usize) {
		self.end = end;
	}


	
	/* PROPERTY GETTER METHODS */

	/// Return the NestedCode's start.
	pub fn start(&self) -> usize {
		self.start
	}
	
	/// Return the NestedCode's end.
	pub fn end(&self) -> usize {
		self.end
	}
	
	/// Return the NestedCode's depth.
	pub fn depth(&self) -> usize {
		self.depth
	}
	
	/// Return the NestedCode's contents.
	pub fn contents(&self) -> &str {
		&self.contents
	}
	
	/// Return the NestedCode's identifier_name.
	pub fn type_name(&self) -> &str {
		&self.identifier_name
	}
}