use std::{ fmt::{ self, Debug }, ops::{ Index, IndexMut } };



pub struct NestedCode {
	type_name:String,
	matched:bool,
	open_tag:String,
	close_tag:String,
	contents:Vec<NestedCode>
}
impl NestedCode {

	/* CONSTRUCTOR METHODS */

	/// Create a new NestedCode<'a>.
	pub fn new(type_name:&str, matched:bool, open_tag:&[char], close_tag:&[char], contents:Vec<NestedCode>) -> NestedCode {
		NestedCode {
			type_name: type_name.to_string(),
			matched,
			open_tag: open_tag.iter().collect::<String>(),
			close_tag: close_tag.iter().collect::<String>(),
			contents
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
		&self.open_tag
	}

	/// Get the closing tag.
	pub fn close_tag(&self) -> &str {
		&self.close_tag
	}

	/// Get the contents.
	pub fn contents(&self) -> &Vec<NestedCode> {
		&self.contents
	}

	/// Get the contents.
	pub fn contents_joined(&self) -> String {
		[
			self.open_tag.clone(),
			self.contents.iter().map(|child| child.contents_joined()).collect::<Vec<String>>().join(""),
			self.close_tag.clone()
		].join("")
	}

	/// Get a flat list of self and all children and their depth.
	pub fn flatten(&self) -> Vec<(usize, &NestedCode)> {
		let mut entries:Vec<(usize, &NestedCode)> = vec![(0, &self)];
		let mut children:Vec<(usize, &NestedCode)> = self.contents.iter().map(|child| child.flatten()).flatten().collect::<Vec<(usize, &NestedCode)>>();
		children.iter_mut().for_each(|(depth, _)| *depth += 1);
		entries.extend(children);
		entries
	}
}
impl Debug for NestedCode {
	fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
		const PADDING:&str = "\t";
		write!(
			f,
			"{} {}\n{}\n{}",
			self.type_name(),
			'{',
				self.contents.iter().map(|code|
					format!("{:?}", code).split('\n').map(|line| PADDING.to_owned() + line).collect::<Vec<String>>().join("\n")
				).collect::<Vec<String>>().join("\n"),
			'}'
		)
	}
}
impl Index<usize> for NestedCode {
	type Output = NestedCode;
	fn index(&self, index:usize) -> &Self::Output {
		&self.contents[index]
	}
}
impl IndexMut<usize> for NestedCode {
	fn index_mut(&mut self, index:usize) -> &mut Self::Output {
		&mut self.contents[index]
	}
}
impl Index<&str> for NestedCode {
	type Output = NestedCode;
	fn index(&self, index:&str) -> &Self::Output {
		self.contents.iter().find(|child| child.type_name() == index).unwrap()
	}
}
impl IndexMut<&str> for NestedCode {
	fn index_mut(&mut self, index:&str) -> &mut Self::Output {
		self.contents.iter_mut().find(|child| child.type_name() == index).unwrap()
	}
}