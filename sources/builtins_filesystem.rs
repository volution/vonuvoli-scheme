

use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		filesystem_path_coerce,
		
	};
	
	pub use super::{
		
		filesystem_file_exists,
		filesystem_file_delete,
		
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_coerce (value : &Value) -> (Outcome<Path>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Path (value) =>
			succeed! (value.clone () .into ()),
		ValueClassMatchAsRef::Symbol (value) =>
			match value.string_as_str () {
				"/" =>
					succeed! (Path::new_root ()),
				"home" =>
					succeed! (try_some! (env::home_dir (), 0xf9959c59) .into ()),
				"temporary" | "tmp" =>
					succeed! (env::temp_dir () .into ()),
				"current" | "cwd" =>
					succeed! (try_or_fail! (env::current_dir (), 0x1ad5c430) .into ()),
				_ =>
					fail! (0x1912686e),
			},
		ValueClassMatchAsRef::String (value) =>
			succeed! (Path::from_string_rc (try! (value.string_as_ref () .string_rc_clone ()))),
		_ =>
			fail! (0x6b191dce),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_exists (path : &Value) -> (Outcome<bool>) {
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	succeed! (path.exists ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_delete (path : &Value) -> (Outcome<()>) {
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	succeed_or_fail! (fs::remove_file (path), 0xa1653696);
}

