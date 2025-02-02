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
impl LazyMatchSource for (&str, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		(self.0, false, self.1, None, self.2, None).to_identification()
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
			if self.2.starts_with('^') && self.3.is_none() {
				MatchMethod::Regex(Regex::new(&self.2).expect("Could not parse regex"))
			} else {
				MatchMethod::CharCompare(
					self.2.to_string(),
					self.3.map(|value| value.to_string())
				)
			},
			if self.4.starts_with('^') && self.5.is_none() {
				MatchMethod::Regex(Regex::new(&self.4).expect("Could not parse regex"))
			} else {
				MatchMethod::CharCompare(
					self.4.to_string(),
					self.5.map(|value| value.to_string())
				)
			}
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
		let regex:String = if self.1.starts_with('^') { self.1.to_string() } else { "^".to_string() + self.1 };
		SegmentIdentification::new(
			self.0,
			false,
			MatchMethod::Regex(Regex::new(&regex).expect("Could not parse regex")),
			MatchMethod::Method(AUTO_CLOSE)
		)
	}
}
pub const AUTO_CLOSE:&'static dyn Fn(&str) -> Option<usize> = &|_| Some(0);