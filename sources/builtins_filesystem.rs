

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		filesystem_path_coerce,
		filesystem_path_join_n,
		filesystem_path_parent,
		filesystem_path_canonicalize,
		
		filesystem_path_to_string,
		filesystem_string_to_path,
		filesystem_path_to_bytes,
		filesystem_bytes_to_path,
		
	};
	
	pub use super::{
		
		filesystem_file_exists,
		filesystem_file_delete,
		
	};
	
	pub use super::{
		
		filesystem_link_resolve,
		
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_coerce (value : &Value) -> (Outcome<Path>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Path (value) =>
			succeed! (value.clone () .into ()),
		ValueClassMatchAsRef::Symbol (value) =>
			match value.string_as_str () {
				"root" | "/" =>
					succeed! (Path::new_root ()),
				"current" | "." =>
					succeed! (Path::new_current ()),
				"parent" | ".." =>
					succeed! (Path::new_parent ()),
				"home" | "~" =>
					succeed! (try_some! (env::home_dir (), 0xf9959c59) .into ()),
				"temporary" | "tmp" =>
					succeed! (env::temp_dir () .into ()),
				"working-directory" | "current-working-directory" | "wd" | "cwd" =>
					succeed! (try_or_fail! (env::current_dir (), 0x1ad5c430) .into ()),
				_ =>
					fail! (0x1912686e),
			},
		ValueClassMatchAsRef::String (value) => {
			let path = try! (value.string_ref ());
			let path = path.string_as_str ();
			if ! path.is_empty () {
				let path = fs_path::Path::new (path);
				succeed! (Path::new_from_ref (path));
			} else {
				fail! (0xb68cf8f4);
			}
		},
		_ =>
			fail! (0x6b191dce),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_join_n (values : &[&Value]) -> (Outcome<Path>) {
	if values.is_empty () {
		fail! (0xc5b87dea);
	}
	let mut buffer = fs_path::PathBuf::new ();
	let mut is_first = true;
	for value in values {
		match value.class_match_as_ref () {
			ValueClassMatchAsRef::Path (value) => {
				let path = value.path_ref ();
				if path.is_absolute () {
					if is_first {
						buffer.push (path);
					} else {
						fail! (0xd12579e0);
					}
				} else {
					buffer.push (path);
				}
			},
			ValueClassMatchAsRef::Symbol (value) =>
				match value.string_as_str () {
					"root" | "/" =>
						if is_first {
							buffer.push (fs_path::Component::RootDir.as_os_str ());
						} else {
							fail! (0x11a0d385);
						},
					"current" | "." =>
						buffer.push (fs_path::Component::CurDir.as_os_str ()),
					"parent" | ".." =>
						buffer.push (fs_path::Component::ParentDir.as_os_str ()),
					"home" | "~" =>
						if is_first {
							buffer.push (try_some! (env::home_dir (), 0xf9959c59));
						} else {
							fail! (0x05969271);
						},
					"temporary" | "tmp" =>
						if is_first {
							buffer.push (env::temp_dir ());
						} else {
							fail! (0x887ed229);
						},
					"working-directory" | "current-working-directory" | "wd" | "cwd" =>
						if is_first {
							buffer.push (try_or_fail! (env::current_dir (), 0x62fa7232));
						} else {
							fail! (0xc1419284);
						},
					_ =>
						fail! (0x1912686e),
				},
			ValueClassMatchAsRef::String (value) => {
				let path = try! (value.string_ref ());
				let path = path.string_as_str ();
				if ! path.is_empty () {
					let path = fs_path::Path::new (path);
					if path.is_absolute () {
						if is_first {
							buffer.push (path);
						} else {
							fail! (0x6e7ff09e);
						}
					} else {
						buffer.push (path);
					}
				} else {
					fail! (0x76377671);
				}
			},
		_ =>
			fail! (0x20b5d1a2),
		}
		is_first = false;
	}
	succeed! (Path::new_from_buffer (buffer));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_parent (path : &Value) -> (Outcome<Value>) {
	// TODO:  Add support for string objects!
	let path = try_as_path_ref! (path);
	let path = path.path_ref ();
	if let Some (parent) = path.parent () {
		if ! parent.as_os_str () .is_empty () {
			succeed! (Path::new_from_ref (parent) .into ());
		} else {
			succeed! (FALSE_VALUE);
		}
	} else {
		succeed! (FALSE_VALUE);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_canonicalize (path : &Value) -> (Outcome<Value>) {
	// TODO:  Add support for string objects!
	let path = try_as_path_ref! (path);
	let path = path.path_ref ();
	match fs::canonicalize (path) {
		Ok (path) =>
			succeed! (Path::new_from_raw (path) .into ()),
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (FALSE_VALUE),
				_ =>
					fail! (0xc7060401),
			},
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_link_resolve (path : &Value, relativize : bool) -> (Outcome<Value>) {
	// TODO:  Add support for string objects!
	let path = try_as_path_ref! (path);
	let path = path.path_ref ();
	match fs::read_link (path) {
		Ok (resolved) =>
			if ! relativize {
				succeed! (Path::new_from_raw (resolved) .into ());
			} else {
				if let Some (parent) = path.parent () {
					succeed! (Path::new_from_buffer (parent.join (resolved)) .into ());
				} else {
					succeed! (Path::new_from_buffer (resolved) .into ());
				}
			},
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::EINVAL) =>
					succeed! (FALSE_VALUE),
				_ =>
					fail! (0x65a36326),
			},
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_to_string (value : &Value, lossy : bool) -> (Outcome<Value>) {
	let value = try_as_path_ref! (value);
	let value = value.internal_rc_clone ();
	let value = match value.to_string_lossy () {
		borrow::Cow::Borrowed (_) =>
			unsafe { mem::transmute (value) },
		borrow::Cow::Owned (value) =>
			if lossy {
				StdRc::new (value.into_boxed_str ())
			} else {
				fail! (0x9c1f18e8);
			},
	};
	let value = StringImmutable::from_rc (value);
	succeed! (value.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_string_to_path (value : &Value) -> (Outcome<Value>) {
	let value = try_as_string_as_ref! (value);
	let value = try! (value.string_rc_clone ());
	if value.is_empty () {
		fail! (0x853e68e9);
	}
	let value = Path::from_string_rc (value);
	succeed! (value.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_to_bytes (value : &Value) -> (Outcome<Value>) {
	let value = try_as_path_ref! (value);
	let value = value.internal_rc_clone ();
	let value = unsafe { mem::transmute (value) };
	let value = BytesImmutable::from_rc (value);
	succeed! (value.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_bytes_to_path (value : &Value) -> (Outcome<Value>) {
	let value = try_as_bytes_as_ref! (value);
	let value = try! (value.bytes_rc_clone ());
	if value.is_empty () {
		fail! (0x853e68e9);
	}
	let value = unsafe { mem::transmute (value) };
	let value = Path::from_rc (value);
	succeed! (value.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_exists (path : &Value) -> (Outcome<bool>) {
	// TODO:  Add support for path objects!
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	succeed! (path.exists ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_delete (path : &Value) -> (Outcome<()>) {
	// TODO:  Add support for path objects!
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	succeed_or_fail! (fs::remove_file (path), 0xa1653696);
}

