pub trait SegmentIdentificationSource {
	fn to_identification(&self) -> SegmentIdentification;
}
impl SegmentIdentificationSource for SegmentIdentification {
	fn to_identification(&self) -> SegmentIdentification {
		self.clone()
	}
}
impl SegmentIdentificationSource for (&str, bool, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(self.0, self.1, self.2, None, self.3, None)
	}
}
impl SegmentIdentificationSource for (&str, bool, &str, Option<&str>, &str, Option<&str>) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(self.0, self.1, self.2, self.3, self.4, self.5)
	}
}
impl SegmentIdentificationSource for (&str, bool, &str, &str, &str, &str) {
	fn to_identification(&self) -> SegmentIdentification {
		SegmentIdentification::new(self.0, self.1, self.2, if self.3.is_empty() { None } else { Some(self.3) }, self.4, if self.5.is_empty() { None } else { Some(self.5) })
	}
}



#[derive(Clone)]
pub struct SegmentIdentification {
	pub(super) name:String,
	pub(super) allow_sub_parse:bool,
	pub(super) open:Vec<char>,
	pub(super) open_escape:Option<Vec<char>>,
	pub(super) close:Vec<char>,
	pub(super) close_escape:Option<Vec<char>>
}
impl SegmentIdentification {
	
	/* CONSTRUCTOR METHODS */
	
	/// Create a new depth modifier.
	pub fn new(name:&str, allow_sub_parse:bool, open:&str, open_escape:Option<&str>, close:&str, close_escape:Option<&str>) -> SegmentIdentification {
		SegmentIdentification {
			name: name.to_string(),
			allow_sub_parse,
			open: open.chars().collect(),
			open_escape: open_escape.map(|increase_escape| increase_escape.chars().collect()),
			close: close.chars().collect(),
			close_escape: close_escape.map(|decrease_escape| decrease_escape.chars().collect())
		}
	}
}