use std::{fmt::Debug, ops::{Index, IndexMut, Range}};



pub struct NestedCode {
	type_name:String,
	open_tag:String,
	close_tag:String,
	contents:String,
	children:Vec<NestedCode>
}
impl NestedCode {

	/* CONSTRUCTOR METHODS */

	/// Create a new NestedCode<'a>.
	pub fn new(type_name:&str, open_tag_location:Range<usize>, close_tag_location:Range<usize>, contents:&[char], children:Vec<NestedCode>) -> NestedCode {
		NestedCode {
			type_name: type_name.to_string(),
			open_tag: contents[open_tag_location.clone()].iter().collect::<String>(),
			close_tag: contents[close_tag_location.clone()].iter().collect::<String>(),
			contents: contents[open_tag_location.end..close_tag_location.start].iter().collect::<String>(),
			children
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the type name.
	pub fn type_name(&self) -> &str {
		&self.type_name
	}

	/// Get the open tag.
	pub fn open_tag(&self) -> &str {
		&self.open_tag
	}

	/// Get the closing tag.
	pub fn close_tag(&self) -> &str {
		&self.close_tag
	}

	/// Get the contents.
	pub fn contents(&self) -> &str {
		&self.contents
	}

	/// Get the sub-code.
	pub fn children(&self) -> &Vec<NestedCode> {
		&self.children
	}

	/// Get a flat list of self and all children and their depth.
	pub fn flatten(&self) -> Vec<(usize, &NestedCode)> {
		let mut entries:Vec<(usize, &NestedCode)> = vec![(0, &self)];
		let mut children:Vec<(usize, &NestedCode)> = self.children.iter().map(|child| child.flatten()).flatten().collect::<Vec<(usize, &NestedCode)>>();
		children.iter_mut().for_each(|(depth, _)| *depth += 1);
		entries.extend(children);
		entries
	}
}
impl Debug for NestedCode {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		const PADDING:&str = "\t";
		write!(
			f,
			"{} {}\n{}\n{}",
			self.type_name(),
			'{',
				self.children.iter().map(|code|
					format!("{:?}", code).split('\n').map(|line| PADDING.to_owned() + line).collect::<Vec<String>>().join("\n")
				).collect::<Vec<String>>().join("\n"),
			'}'
		)
	}
}
impl Index<usize> for NestedCode {
	type Output = NestedCode;
	fn index(&self, index:usize) -> &Self::Output {
		&self.children[index]
	}
}
impl IndexMut<usize> for NestedCode {
	fn index_mut(&mut self, index:usize) -> &mut Self::Output {
		&mut self.children[index]
	}
}
impl Index<&str> for NestedCode {
	type Output = NestedCode;
	fn index(&self, index:&str) -> &Self::Output {
		self.children.iter().find(|child| child.type_name() == index).unwrap()
	}
}
impl IndexMut<&str> for NestedCode {
	fn index_mut(&mut self, index:&str) -> &mut Self::Output {
		self.children.iter_mut().find(|child| child.type_name() == index).unwrap()
	}
}