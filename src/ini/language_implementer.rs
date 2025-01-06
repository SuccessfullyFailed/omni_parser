#[macro_export]
macro_rules! create_ini_flavor {
	($type_name:ident, $encoder:expr, $decoder:expr) => {
		pub struct $type_name(pub crate::IniCore);
		impl $type_name {
			
			/// Create a new IniCore from a file.
			pub fn from_file(path:&str) -> Result<$type_name, Box<dyn std::error::Error>> {
				Ok($type_name(crate::IniCore::from_file(path, $encoder, $decoder)?))
			}

			/// Create a new IniCore from contents.
			pub fn from_contents(contents:&str) -> Result<$type_name, Box<dyn std::error::Error>> {
				Ok($type_name(crate::IniCore::from_contents(contents, $encoder, $decoder)?))
			}

			/// Save the changes made to the original file if there is one.
			pub fn save_changes(&self) -> Result<(), Box<dyn std::error::Error>> {
				self.0.save_changes()
			}

			/// Save the ini to the specified file.
			pub fn save_to_file(&self, file_path:&str) -> Result<(), Box<dyn std::error::Error>> {
				self.0.save_to_file(file_path)
			}
		}
		impl std::ops::Index<&str> for $type_name {
			type Output = crate::IniCategory;
			fn index(&self, target_name:&str) -> &Self::Output {
				&self.0[target_name]
			}
		}
		impl std::ops::IndexMut<&str> for $type_name {
			fn index_mut(&mut self, target_name:&str) -> &mut Self::Output {
				&mut self.0[target_name]
			}
		}
	};
}