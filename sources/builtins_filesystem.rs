

use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		filesystem_file_exists,
		filesystem_file_delete,
		
	};
	
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

