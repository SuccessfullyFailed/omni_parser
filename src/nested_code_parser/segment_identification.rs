use regex::Regex;

#[derive(Clone)]
pub struct SegmentIdentification {
	pub(super) name:String,
	pub(super) allow_sub_parse:bool,
	pub(super) matching_method_open:MatchMethod,
	pub(super) matching_method_close:MatchMethod
}
impl SegmentIdentification {
	
	/* CONSTRUCTOR METHODS */
	
	/// Create a new depth modifier.
	pub fn new(name:&str, allow_sub_parse:bool, matching_method_open:MatchMethod, matching_method_close:MatchMethod) -> SegmentIdentification {
		SegmentIdentification {
			name: name.to_string(),
			allow_sub_parse,
			matching_method_open,
			matching_method_close
		}
	}
}



#[derive(Clone)]
pub enum MatchMethod {
	CharCompare(String, Option<String>),
	Method(&'static dyn Fn(&str) -> Option<usize>),
	Regex(Regex)
}



pub trait LazyMatchSource {
	fn to_identification(&self) -> SegmentIdentification;
}
impl LazyMatchSource for SegmentIdentification {
	fn to_identification(&self) -> SegmentIdentification {
		self.clone()
	}
}
impl LazyMatchSource for (&str, bool, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		(self.0, self.1, self.2, None, self.3, None).to_identification()
	}
}
impl LazyMatchSource for (&str, bool, &str, &str, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		(self.0, self.1, self.2, Some(self.3), self.4, Some(self.5)).to_identification()
	}
}
impl LazyMatchSource for (&str, bool, &str, Option<&str>, &str, Option<&str>) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(
			self.0,
			self.1,
			MatchMethod::CharCompare(
				self.2.to_string(),
				self.3.map(|value| value.to_string())
			),
			MatchMethod::CharCompare(
				self.4.to_string(),
				self.5.map(|value| value.to_string())
			)
		)
	}
}
impl LazyMatchSource for (&str, bool, &'static dyn Fn(&str) -> Option<usize>, &'static dyn Fn(&str) -> Option<usize>) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(
			self.0,
			self.1,
			MatchMethod::Method(self.2),
			MatchMethod::Method(self.3)
		)
	}
}
impl LazyMatchSource for (&str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		if !self.1.starts_with('^') {
			// Could be solved automatically, but this also warns the user that they are using regex.
			panic!("Any regex used in a NestedCodeParser should start with '^', making sure it only matches the contents at the current cursor.");
		}
		SegmentIdentification::new(
			self.0,
			false,
			MatchMethod::Regex(Regex::new(self.1).unwrap()),
			MatchMethod::Method(&|_| Some(0))
		)
	}
}