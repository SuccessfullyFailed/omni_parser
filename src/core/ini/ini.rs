use std::ops::{Index, IndexMut};



const UNAVAILABLE_NAME:&str = "UNAVAILABLE_PROPERTY_ERROR";



pub struct Ini {
	data:Vec<IniCategory>
}
impl Index<&str> for Ini {
	type Output = IniCategory;
	fn index(&self, target_name:&str) -> &Self::Output {
		match self.data.iter().position(|variable| &variable.name == target_name) {
			Some(index) => &self.data[index],
			None => IniCategory::error_instance()
		}
	}
}
impl IndexMut<&str> for Ini {
	fn index_mut(&mut self, target_name:&str) -> &mut Self::Output {
		match self.data.iter().position(|category| &category.name == target_name) {
			Some(index) => &mut self.data[index],
			None => {
				self.data.push(IniCategory { name: target_name.to_string(), data: Vec::new() });
				self.data.iter_mut().last().unwrap()
			}
		}
	}
}



pub struct IniCategory {
	pub name:String,
	pub data:Vec<IniVariable>
}
impl IniCategory {

	/// Create a new IniCategory.
	pub fn new(name:&str) -> IniCategory {
		if name == UNAVAILABLE_NAME {
			panic!("'{UNAVAILABLE_NAME}' should not be used for an ini category name. This string is reserved for incorrect property fetching.");
		}
		IniCategory {
			name: name.to_string(),
			data: Vec::new()
		}
	}

	/// Get the error IniCategory. Used for when an ini category is expected, but not available.
	pub fn error_instance() -> &'static IniCategory {
		unsafe {
			static mut INVALID_RETURN:Option<IniCategory> = None;
			match INVALID_RETURN.as_ref() {
				Some(ini_var) => ini_var,
				None => {
					INVALID_RETURN = Some(IniCategory { name: String::from(UNAVAILABLE_NAME), data: Vec::new()});
					Self::error_instance()
				}
			}
		}
	}

	/// Check if the gotten variable is ok, not the error category.
	pub fn is_ok(&self) -> bool {
		self.name == UNAVAILABLE_NAME
	}
}
impl Index<&str> for IniCategory {
	type Output = IniVariable;
	fn index(&self, target_name:&str) -> &Self::Output {
		match self.data.iter().position(|variable| &variable.name == target_name) {
			Some(index) => &self.data[index],
			None => IniVariable::error_instance()
		}
	}
}
impl IndexMut<&str> for IniCategory {
	fn index_mut(&mut self, target_name:&str) -> &mut Self::Output {
		match self.data.iter().position(|variable| &variable.name == target_name) {
			Some(index) => &mut self.data[index],
			None => {
				self.data.push(IniVariable::new(target_name, ""));
				self.data.iter_mut().last().unwrap()
			}
		}
	}
}



pub struct IniVariable {
	pub name:String,
	pub value:String
}
impl IniVariable {

	/// Create a new IniVariable.
	pub fn new(name:&str, value:&str) -> IniVariable {
		if name == UNAVAILABLE_NAME {
			panic!("'{UNAVAILABLE_NAME}' should not be used for an ini variable name. This string is reserved for incorrect property fetching.");
		}
		IniVariable {
			name: name.to_string(),
			value: value.to_string()
		}
	}

	/// Get the error IniVariable. Used for when an ini variable is expected, but not available.
	pub fn error_instance() -> &'static IniVariable {
		unsafe {
			static mut INVALID_RETURN:Option<IniVariable> = None;
			match INVALID_RETURN.as_ref() {
				Some(ini_var) => ini_var,
				None => {
					INVALID_RETURN = Some(IniVariable { name: String::from(UNAVAILABLE_NAME), value: String::from(UNAVAILABLE_NAME) });
					Self::error_instance()
				}
			}
		}
	}

	/// Check if the gotten variable is ok, not the error variable.
	pub fn is_ok(&self) -> bool {
		self.name == UNAVAILABLE_NAME
	}
}