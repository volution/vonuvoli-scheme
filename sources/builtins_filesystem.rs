

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		filesystem_path_coerce,
		filesystem_path_join,
		filesystem_path_split,
		filesystem_path_parent,
		
		filesystem_path_name,
		filesystem_path_name_without_extension,
		filesystem_path_name_only_extension,
		filesystem_path_name_join,
		filesystem_path_name_split,
		
		filesystem_path_has_prefix,
		filesystem_path_has_suffix,
		
		filesystem_path_name_is,
		filesystem_path_name_has_prefix,
		filesystem_path_name_has_suffix,
		
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
	
	
	pub use super::{
		
		filesystem_metadata_resolve,
		filesystem_metadata_coerce,
		
		filesystem_metadata_get_kind,
		filesystem_metadata_get_kind_symbol,
		filesystem_metadata_has_kind,
		filesystem_metadata_has_kind_symbol,
		
		FileSystemMetadataKind,
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_coerce (value : &Value, normalize : bool) -> (Outcome<Path>) {
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
					succeed! (Path::new_from_buffer (try_some! (env::home_dir (), 0xab8aa16c), normalize)),
				"temporary" | "tmp" =>
					succeed! (Path::new_from_buffer (env::temp_dir (), normalize)),
				"working-directory" | "current-working-directory" | "wd" | "cwd" =>
					succeed! (Path::new_from_buffer (try_or_fail! (env::current_dir (), 0x1ad5c430), normalize)),
				_ =>
					fail! (0xa2667867),
			},
		ValueClassMatchAsRef::String (value) => {
			let value = value.string_as_ref ();
			let value = try! (value.string_rc_clone ());
			if value.is_empty () {
				fail! (0xb68cf8f4);
			}
			succeed! (Path::from_string_rc (value, normalize));
		},
		_ =>
			fail! (0x6b191dce),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_join (values : &[&Value], normalize : bool) -> (Outcome<Path>) {
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
				if path.is_empty () {
					fail! (0x76377671);
				}
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
			},
		_ =>
			fail! (0x20b5d1a2),
		}
		is_first = false;
	}
	succeed! (Path::new_from_buffer (buffer, normalize));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_split (path : &Value, return_array : bool) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let mut components = StdVec::new ();
	for component in path.components () {
		let component = Path::new_from_component (&component, false);
		components.push (component.into ());
	}
	if return_array {
		succeed! (array_new (components) .into ());
	} else {
		succeed! (list_collect (components, None));
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_parent (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	if let Some (parent) = path.parent () {
		if ! parent.as_os_str () .is_empty () {
			succeed! (Path::new_from_ref (parent, false) .into ());
		} else {
			// NOTE:  The path has just a component which is not the root.
			succeed! (FALSE_VALUE);
		}
	} else {
		succeed! (FALSE_VALUE);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	if let Some (name) = path.file_name () {
		if name.is_empty () {
			fail_panic! (0x97c85be5);
		}
		succeed! (Path::new_from_ref (fs_path::Path::new (name), false) .into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_without_extension (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let name = if let Some (name) = path.file_name () {
		if name.is_empty () {
			fail_panic! (0xc3e91299);
		}
		fs_path::Path::new (name)
	} else {
		succeed! (FALSE_VALUE);
	};
	if let Some (stem) = name.file_stem () {
		if stem.is_empty () {
			fail_panic! (0x8ae49f96);
		}
		if stem.as_bytes () .iter () .find (|byte| (**byte) != ('.' as u8)) .is_none () {
			// NOTE:  The name starts with multiple dots and it has no other extension.
			succeed! (Path::new_from_ref (fs_path::Path::new (name), false) .into ());
		} else {
			succeed! (Path::new_from_ref (fs_path::Path::new (stem), false) .into ());
		}
	} else {
		fail_unreachable! (0x4109a459);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_only_extension (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let name = if let Some (name) = path.file_name () {
		if name.is_empty () {
			fail_panic! (0xfafb61b7);
		}
		fs_path::Path::new (name)
	} else {
		succeed! (FALSE_VALUE);
	};
	if let Some (extension) = name.extension () {
		if let Some (stem) = name.file_stem () {
			if stem.is_empty () {
				fail_panic! (0x32e4db76);
			}
			if stem.as_bytes () .iter () .find (|byte| (**byte) != ('.' as u8)) .is_none () {
				// NOTE:  The name starts with multiple dots and it has no other extension.
				succeed! (FALSE_VALUE);
			} else {
				succeed! (Path::new_from_ref (fs_path::Path::new (extension), false) .into ());
			}
		} else {
			fail_unreachable! (0xd9b8e113);
		}
	} else {
		succeed! (FALSE_VALUE);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_split (path : &Value, return_array : bool) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let mut name = if let Some (name) = path.file_name () {
		if name.is_empty () {
			fail_panic! (0x6c0dff54);
		}
		fs_path::Path::new (name)
	} else {
		succeed! (FALSE_VALUE);
	};
	let mut components = StdVec::new ();
	loop {
		match (name.file_stem (), name.extension ()) {
			(Some (stem), Some (extension)) => {
				if stem.is_empty () {
					fail_panic! (0x25fece1e);
				}
				if stem.as_bytes () .iter () .find (|byte| (**byte) != ('.' as u8)) .is_none () {
					// NOTE:  The name starts with multiple dots and it has no other extension.
					components.push (Path::new_from_ref (name, false) .into ());
					break;
				} else {
					components.push (Path::new_from_ref (fs_path::Path::new (extension), false) .into ());
					name = fs_path::Path::new (stem);
				}
			},
			(Some (stem), None) => {
				components.push (Path::new_from_ref (fs_path::Path::new (stem), false) .into ());
				break;
			},
			(None, Some (_)) =>
				fail_unreachable! (0x3dc45bfd),
			(None, None) =>
				fail_unreachable! (0x19cbe613),
		}
	}
	components.reverse ();
	if return_array {
		succeed! (array_new (components) .into ());
	} else {
		succeed! (list_collect (components, None));
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_join (values : &[&Value]) -> (Outcome<Path>) {
	if values.is_empty () {
		fail! (0x3e7ace18);
	}
	let mut buffer = ffi::OsString::new ();
	let mut is_first = true;
	for value in values {
		match value.class_match_as_ref () {
			ValueClassMatchAsRef::Path (value) => {
				let path = value.path_ref ();
				if path.as_os_str () .is_empty () {
					fail! (0xfd4b3414);
				}
				if is_first {
					buffer.push (path);
				} else {
					if let Some (name) = path.file_name () {
						if name.is_empty () {
							fail_panic! (0x47deb6f8);
						}
						if name == path {
							buffer.push (".");
							buffer.push (path);
						} else {
							fail! (0xa935f83e);
						}
					} else {
						fail! (0x1d35bddc);
					}
				}
			},
			ValueClassMatchAsRef::String (value) => {
				let path = try! (value.string_ref ());
				let path = path.string_as_str ();
				if path.is_empty () {
					fail! (0xc95e1bb7);
				}
				let path = fs_path::Path::new (path);
				if is_first {
					buffer.push (path);
				} else {
					if let Some (name) = path.file_name () {
						if name.is_empty () {
							fail_panic! (0x0fd246e1);
						}
						if name == path {
							buffer.push (".");
							buffer.push (path);
						} else {
							fail! (0x1d50e6f1);
						}
					} else {
						fail! (0xa74d064e);
					}
				}
			},
		_ =>
			fail! (0x9c6ca0a8),
		}
		is_first = false;
	}
	succeed! (Path::new_from_buffer (fs_path::PathBuf::from (buffer), false));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_has_prefix (path : &Value, prefix : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let prefix = try! (path_slice_coerce (prefix));
	let prefix = prefix.deref ();
	succeed! (path.starts_with (prefix));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_has_suffix (path : &Value, suffix : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let suffix = try! (path_slice_coerce (suffix));
	let suffix = suffix.deref ();
	succeed! (path.ends_with (suffix));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_is (path : &Value, expected : &Value) -> (Outcome<bool>) {
	// TODO:  Refactor `path_name_is`, `path_name_has_prefix` and `path_name_has_suffix`!
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let expected = try! (path_slice_coerce (expected));
	let expected = expected.deref ();
	let path_name = if let Some (path_name) = path.file_name () {
		if path_name.is_empty () {
			fail_panic! (0x68aa0374);
		}
		path_name
	} else {
		fail! (0x32e25892);
	};
	let expected_name = if let Some (expected_name) = expected.file_name () {
		if expected_name.is_empty () {
			fail_panic! (0x2988b840);
		}
		if expected_name != expected {
			fail! (0x4d5c2779);
		}
		expected_name
	} else {
		fail! (0xd514d0bf);
	};
	succeed! (ffi::OsStr::eq (path_name, expected_name));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_has_prefix (path : &Value, prefix : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let prefix = try! (path_slice_coerce (prefix));
	let prefix = prefix.deref ();
	let path_name = if let Some (path_name) = path.file_name () {
		if path_name.is_empty () {
			fail_panic! (0x01f73eaf);
		}
		path_name
	} else {
		fail! (0x1e2367c6);
	};
	let path_name = path_name.as_bytes ();
	let prefix_name = if let Some (prefix_name) = prefix.file_name () {
		if prefix_name.is_empty () {
			fail_panic! (0x81b12ef4);
		}
		if prefix_name != prefix {
			fail! (0x4ec40ed6);
		}
		prefix_name.as_bytes ()
	} else {
		const DOT : u8 = '.' as u8;
		match prefix.as_os_str () .as_bytes () {
			// NOTE:  This is a corner case in which we check for `` or `.` or `..`!
			prefix_name @ &[] |
			prefix_name @ &[DOT] |
			prefix_name @ &[DOT, DOT] =>
				prefix_name,
			_ =>
				fail! (0xe958ac00),
		}
	};
	succeed! (path_name.starts_with (prefix_name));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_has_suffix (path : &Value, suffix : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let suffix = try! (path_slice_coerce (suffix));
	let suffix = suffix.deref ();
	let path_name = if let Some (path_name) = path.file_name () {
		if path_name.is_empty () {
			fail_panic! (0x2e54c28a);
		}
		path_name
	} else {
		fail! (0xe4042ca5);
	};
	let path_name = path_name.as_bytes ();
	let suffix_name = if let Some (suffix_name) = suffix.file_name () {
		if suffix_name.is_empty () {
			fail_panic! (0xe5482553);
		}
		if suffix_name != suffix {
			fail! (0x9a1ae18e);
		}
		suffix_name.as_bytes ()
	} else {
		const DOT : u8 = '.' as u8;
		match suffix.as_os_str () .as_bytes () {
			// NOTE:  This is a corner case in which we check for `` or `.` or `..`!
			suffix_name @ &[] |
			suffix_name @ &[DOT] |
			suffix_name @ &[DOT, DOT] =>
				suffix_name,
			_ =>
				fail! (0xe1929d8d),
		}
	};
	succeed! (path_name.ends_with (suffix_name));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_canonicalize (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	match fs::canonicalize (path) {
		Ok (path) =>
			succeed! (Path::new_from_raw (path, false) .into ()),
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
pub fn filesystem_link_resolve (path : &Value, relativize : bool, normalize : bool) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	match fs::read_link (path) {
		Ok (resolved) => {
			if normalize && resolved.as_os_str () .is_empty () {
				fail! (0xc814dbda);
			}
			if ! relativize {
				succeed! (Path::new_from_raw (resolved, normalize) .into ());
			} else {
				if let Some (parent) = path.parent () {
					if ! parent.as_os_str () .is_empty () {
						succeed! (Path::new_from_buffer (parent.join (resolved), normalize) .into ());
					} else {
						// NOTE:  The path has just a component which is not the root.
						succeed! (Path::new_from_buffer (resolved, normalize) .into ());
					}
				} else {
					succeed! (Path::new_from_buffer (resolved, normalize) .into ());
				}
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
	let value = Path::from_string_rc (value, false);
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
		fail! (0x3dd3eb28);
	}
	let value = unsafe { mem::transmute (value) };
	let value = Path::from_rc (value, false);
	succeed! (value.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_resolve (path : &Value, follow : bool) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	match
			if follow {
				fs::metadata (path)
			} else {
				fs::symlink_metadata (path)
			}
	{
		Ok (metadata) =>
			succeed! (opaque_new (metadata) .into ()),
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (FALSE_VALUE),
				_ =>
					fail! (0x3ff7b86e),
			},
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_coerce (value : &Value, follow : bool) -> (Outcome<CoercedRef<fs::Metadata>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Opaque (value) =>
			succeed! (CoercedRef::new_reference (try_some! (value.downcast (), 0x102f7d24))),
		_ => {
			let path = try! (path_slice_coerce (value));
			let path = path.deref ();
			let metadata = if follow {
				try_or_fail! (fs::metadata (path), 0x3b6116ee)
			} else {
				try_or_fail! (fs::symlink_metadata (path), 0xa73b2fd0)
			};
			succeed! (CoercedRef::new_value (metadata));
		},
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum FileSystemMetadataKind {
	File,
	Directory,
	SymLink,
	Fifo,
	Socket,
	BlockDevice,
	CharacterDevice,
}


impl <'a> StdTryFrom<&'a fs::FileType> for FileSystemMetadataKind {
	
	type Error = Error;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_from (kind : &'a fs::FileType) -> (Outcome<FileSystemMetadataKind>) {
		let kind = if kind.is_file () {
			FileSystemMetadataKind::File
		} else if kind.is_dir () {
			FileSystemMetadataKind::Directory
		} else if kind.is_symlink () {
			FileSystemMetadataKind::SymLink
		} else if kind.is_fifo () {
			FileSystemMetadataKind::Fifo
		} else if kind.is_socket () {
			FileSystemMetadataKind::Socket
		} else if kind.is_block_device () {
			FileSystemMetadataKind::BlockDevice
		} else if kind.is_char_device () {
			FileSystemMetadataKind::CharacterDevice
		} else {
			fail_panic! (0x27130fff);
		};
		succeed! (kind);
	}
}


impl <'a> StdTryFrom<&'a Value> for FileSystemMetadataKind {
	
	type Error = Error;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_from (value : &'a Value) -> (Outcome<FileSystemMetadataKind>) {
		let kind = try_as_symbol_ref! (value);
		let kind = kind.string_as_str ();
		let kind = match kind {
			"file" =>
				FileSystemMetadataKind::File,
			"directory" =>
				FileSystemMetadataKind::Directory,
			"symlink" =>
				FileSystemMetadataKind::SymLink,
			"fifo" =>
				FileSystemMetadataKind::Fifo,
			"socket" =>
				FileSystemMetadataKind::Socket,
			"block-device" =>
				FileSystemMetadataKind::BlockDevice,
			"character-device" | "char-device" =>
				FileSystemMetadataKind::CharacterDevice,
			_ =>
				fail! (0x8bf649ca),
		};
		succeed! (kind);
	}
}


impl <'a> StdFrom<&'a FileSystemMetadataKind> for Symbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (kind : &'a FileSystemMetadataKind) -> (Symbol) {
		let kind = match kind {
			FileSystemMetadataKind::File =>
				"file",
			FileSystemMetadataKind::Directory =>
				"directory",
			FileSystemMetadataKind::SymLink =>
				"symlink",
			FileSystemMetadataKind::Fifo =>
				"fifo",
			FileSystemMetadataKind::Socket =>
				"socket",
			FileSystemMetadataKind::BlockDevice =>
				"block-device",
			FileSystemMetadataKind::CharacterDevice =>
				"character-device",
		};
		let kind = symbol_clone_str (kind);
		return kind;
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_get_kind (metadata : &Value, follow : bool) -> (Outcome<FileSystemMetadataKind>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let kind = metadata.file_type ();
	let kind = try! (FileSystemMetadataKind::try_from (&kind));
	succeed! (kind);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_get_kind_symbol (metadata : &Value, follow : bool) -> (Outcome<Symbol>) {
	let kind = try! (filesystem_metadata_get_kind (metadata, follow));
	let kind = Symbol::from (&kind);
	succeed! (kind);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_has_kind (metadata : &Value, kind : FileSystemMetadataKind, follow : bool) -> (Outcome<bool>) {
	let follow = follow && (kind != FileSystemMetadataKind::SymLink);
	let actual = try! (filesystem_metadata_get_kind (metadata, follow));
	succeed! (actual == kind);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_has_kind_symbol (metadata : &Value, kind : &Value, follow : bool) -> (Outcome<bool>) {
	let kind = try! (FileSystemMetadataKind::try_from (kind));
	return filesystem_metadata_has_kind (metadata, kind, follow);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_exists (path : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	succeed! (path.exists ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_delete (path : &Value) -> (Outcome<()>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	succeed_or_fail! (fs::remove_file (path), 0xa1653696);
}

