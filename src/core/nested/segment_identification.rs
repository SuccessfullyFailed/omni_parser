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
	name:String,
	allow_sub_parse:bool,
	open:String,
	open_escape:Option<String>,
	close:String,
	close_escape:Option<String>
}
impl SegmentIdentification {
	
	/* CONSTRUCTOR METHODS */
	
	/// Create a new depth modifier.
	pub fn new(name:&str, allow_sub_parse:bool, open:&str, open_escape:Option<&str>, close:&str, close_escape:Option<&str>) -> SegmentIdentification {
		SegmentIdentification {
			name: name.to_string(),
			allow_sub_parse,
			open: open.to_string(),
			open_escape: open_escape.map(|increase_escape| increase_escape.to_string()),
			close: close.to_string(),
			close_escape: close_escape.map(|decrease_escape| decrease_escape.to_string())
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Return the SegmentIdentifications name.
	pub fn name(&self) -> &str {
		&self.name
	}
	
	/// Return the SegmentIdentifications allow_sub_parse.
	pub fn allow_sub_parse(&self) -> bool {
		self.allow_sub_parse
	}
	
	/// Return the SegmentIdentifications open.
	pub fn open(&self) -> &str {
		&self.open
	}
	
	/// Return the SegmentIdentifications open_escape.
	pub fn open_escape(&self) -> &Option<String> {
		&self.open_escape
	}
	
	/// Return the SegmentIdentifications close.
	pub fn close(&self) -> &str {
		&self.close
	}
	
	/// Return the SegmentIdentifications close_escape.
	pub fn close_escape(&self) -> &Option<String> {
		&self.close_escape
	}
}