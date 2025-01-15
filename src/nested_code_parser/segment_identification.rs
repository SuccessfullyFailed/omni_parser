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
	CharCompare(Vec<char>, Option<Vec<char>>)
}



pub trait MatchMethodSource {
	fn to_identification(&self) -> SegmentIdentification;
}
impl MatchMethodSource for SegmentIdentification {
	fn to_identification(&self) -> SegmentIdentification {
		self.clone()
	}
}
impl MatchMethodSource for (&str, bool, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		(self.0, self.1, self.2, None, self.3, None).to_identification()
	}
}
impl MatchMethodSource for (&str, bool, &str, &str, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		(self.0, self.1, self.2, Some(self.3), self.4, Some(self.5)).to_identification()
	}
}
impl MatchMethodSource for (&str, bool, &str, Option<&str>, &str, Option<&str>) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(
			self.0,
			self.1,
			MatchMethod::CharCompare(
				self.2.chars().collect(),
				self.3.map(|value| value.chars().collect())
			),
			MatchMethod::CharCompare(
				self.4.chars().collect(),
				self.5.map(|value| value.chars().collect())
			)
		)
	}
}