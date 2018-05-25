

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
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
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
		
		filesystem_path_to_string,
		filesystem_string_to_path,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
		
		filesystem_path_to_bytes,
		filesystem_bytes_to_path,
		
	};
	
	pub use super::{
		
		filesystem_any_exists,
		
		filesystem_file_exists,
		filesystem_file_delete,
		
		filesystem_directory_exists,
		filesystem_directory_delete,
		
		filesystem_symlink_exists,
		
	};
	
	pub use super::{
		
		filesystem_symlink_resolve,
		
		filesystem_directory_list,
		filesystem_directory_fold,
		
		filesystem_directory_fold_recursive,
		
	};
	
	pub use super::{
		
		filesystem_mountpoint_is,
		
	};
	
	pub use super::{
		
		filesystem_metadata_resolve,
		filesystem_metadata_coerce,
		
		filesystem_metadata_is,
		filesystem_metadata_is_self,
		
		filesystem_metadata_get_kind,
		filesystem_metadata_get_kind_symbol,
		filesystem_metadata_has_kind,
		filesystem_metadata_has_kind_symbol,
		
		filesystem_metadata_file_get_size,
		filesystem_metadata_file_get_size_0,
		filesystem_metadata_file_is_empty,
		filesystem_metadata_file_is_not_empty,
		
		filesystem_metadata_is_readonly,
		filesystem_metadata_is_readable,
		filesystem_metadata_is_writeable,
		filesystem_metadata_file_is_executable,
		filesystem_metadata_directory_is_traversable,
		
		filesystem_metadata_unix_get_permissions_for_current_process,
		
		filesystem_metadata_unix_get_mode,
		filesystem_metadata_unix_get_type,
		filesystem_metadata_unix_get_permissions,
		filesystem_metadata_unix_get_user_identifier,
		filesystem_metadata_unix_get_group_identifier,
		filesystem_metadata_unix_get_data_accessed_at,
		filesystem_metadata_unix_get_data_modified_at,
		filesystem_metadata_unix_get_inode_changed_at,
		filesystem_metadata_unix_get_inode_device,
		filesystem_metadata_unix_get_inode_number,
		filesystem_metadata_unix_get_inode_links,
		
		FileSystemMetadataKind,
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
	pub use super::{
		
		filesystem_temporary_create_file,
		filesystem_temporary_create_directory,
		filesystem_temporary_release,
		
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::String (value) => {
			TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
			let value = value.string_as_ref ();
			let value = try! (value.string_rc_clone ());
			if value.is_empty () {
				fail! (0xb68cf8f4);
			}
			succeed! (Path::from_string_rc (value, normalize));
		},
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
			let value = value.bytes_as_ref ();
			let value = try! (value.bytes_rc_clone ());
			if value.is_empty () {
				fail! (0x5f54fd7b);
			}
			succeed! (Path::from_bytes_rc (value, normalize));
		},
		_ =>
			fail! (0x6b191dce),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_join (values : &[impl StdAsRef<Value>], normalize : bool) -> (Outcome<Path>) {
	if values.is_empty () {
		fail! (0xc5b87dea);
	}
	let mut buffer = fs_path::PathBuf::new ();
	let mut is_first = true;
	for value in values {
		TODO! ("refactor check for `is_absolute` and `is_first`", (github_issue, 82));
		let value = value.as_ref ();
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
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchAsRef::String (value) => {
				TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
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
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchAsRef::Bytes (value) => {
				TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
				let path = try! (value.bytes_ref ());
				let path = path.bytes_as_slice ();
				if path.is_empty () {
					fail! (0x0f0a6c54);
				}
				let path = fs_path::Path::new (ffi::OsStr::from_bytes (path));
				if path.is_absolute () {
					if is_first {
						buffer.push (path);
					} else {
						fail! (0x7ce6d2ed);
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
	return build_list_or_array (components, return_array);
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
			fail_panic! (0x97c85be5, github_issue_new);
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
			fail_panic! (0xc3e91299, github_issue_new);
		}
		fs_path::Path::new (name)
	} else {
		succeed! (FALSE_VALUE);
	};
	if let Some (stem) = name.file_stem () {
		if stem.is_empty () {
			fail_panic! (0x8ae49f96, github_issue_new);
		}
		if stem.as_bytes () .iter () .find (|byte| (**byte) != ('.' as u8)) .is_none () {
			// NOTE:  The name starts with multiple dots and it has no other extension.
			succeed! (Path::new_from_ref (fs_path::Path::new (name), false) .into ());
		} else {
			succeed! (Path::new_from_ref (fs_path::Path::new (stem), false) .into ());
		}
	} else {
		fail_unreachable! (0x4109a459, github_issue_new);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_only_extension (path : &Value) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let name = if let Some (name) = path.file_name () {
		if name.is_empty () {
			fail_panic! (0xfafb61b7, github_issue_new);
		}
		fs_path::Path::new (name)
	} else {
		succeed! (FALSE_VALUE);
	};
	if let Some (extension) = name.extension () {
		if let Some (stem) = name.file_stem () {
			if stem.is_empty () {
				fail_panic! (0x32e4db76, github_issue_new);
			}
			if stem.as_bytes () .iter () .find (|byte| (**byte) != ('.' as u8)) .is_none () {
				// NOTE:  The name starts with multiple dots and it has no other extension.
				succeed! (FALSE_VALUE);
			} else {
				succeed! (Path::new_from_ref (fs_path::Path::new (extension), false) .into ());
			}
		} else {
			fail_unreachable! (0xd9b8e113, github_issue_new);
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
			fail_panic! (0x6c0dff54, github_issue_new);
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
					fail_panic! (0x25fece1e, github_issue_new);
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
				fail_unreachable! (0x3dc45bfd, github_issue_new),
			(None, None) =>
				fail_unreachable! (0x19cbe613, github_issue_new),
		}
	}
	components.reverse ();
	return build_list_or_array (components, return_array);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_name_join (values : &[impl StdAsRef<Value>]) -> (Outcome<Path>) {
	if values.is_empty () {
		fail! (0x3e7ace18);
	}
	let mut buffer = ffi::OsString::new ();
	let mut is_first = true;
	for value in values {
		TODO! ("refactor the various checks and common code", (github_issue, 82));
		let value = value.as_ref ();
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
							fail_panic! (0x47deb6f8, github_issue_new);
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
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchAsRef::String (value) => {
				TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
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
							fail_panic! (0x0fd246e1, github_issue_new);
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
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchAsRef::Bytes (value) => {
				TODO! ("refactor string and bytes cases by extracting common code", (github_issue, 82));
				let path = try! (value.bytes_ref ());
				let path = path.bytes_as_slice ();
				if path.is_empty () {
					fail! (0x8d796f03);
				}
				let path = fs_path::Path::new (ffi::OsStr::from_bytes (path));
				if is_first {
					buffer.push (path);
				} else {
					if let Some (name) = path.file_name () {
						if name.is_empty () {
							fail_panic! (0x9d74bf3a, github_issue_new);
						}
						if name == path {
							buffer.push (".");
							buffer.push (path);
						} else {
							fail! (0x765b58ab);
						}
					} else {
						fail! (0x0d0902ba);
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
	TODO! ("refactor `path_name_is`, `path_name_has_prefix` and `path_name_has_suffix`");
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let expected = try! (path_slice_coerce (expected));
	let expected = expected.deref ();
	let path_name = if let Some (path_name) = path.file_name () {
		if path_name.is_empty () {
			fail_panic! (0x68aa0374, github_issue_new);
		}
		path_name
	} else {
		fail! (0x32e25892);
	};
	let expected_name = if let Some (expected_name) = expected.file_name () {
		if expected_name.is_empty () {
			fail_panic! (0x2988b840, github_issue_new);
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
			fail_panic! (0x01f73eaf, github_issue_new);
		}
		path_name
	} else {
		fail! (0x1e2367c6);
	};
	let path_name = path_name.as_bytes ();
	let prefix_name = if let Some (prefix_name) = prefix.file_name () {
		if prefix_name.is_empty () {
			fail_panic! (0x81b12ef4, github_issue_new);
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
			fail_panic! (0x2e54c28a, github_issue_new);
		}
		path_name
	} else {
		fail! (0xe4042ca5);
	};
	let path_name = path_name.as_bytes ();
	let suffix_name = if let Some (suffix_name) = suffix.file_name () {
		if suffix_name.is_empty () {
			fail_panic! (0xe5482553, github_issue_new);
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
pub fn filesystem_symlink_resolve (path : &Value, relativize : bool, normalize : bool) -> (Outcome<Value>) {
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
pub fn filesystem_directory_list (path : &Value, join_parent : bool, include_kind : bool, include_metadata : bool, follow : bool, sort : bool, return_array : bool) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let mut entries = StdVec::new ();
	for entry in try_or_fail! (fs::read_dir (path), 0xc28bc39c) {
		let entry = try_or_fail! (entry, 0xeea94f1d);
		let (entry_path, entry_kind, entry_metadata) = try! (filesystem_directory_entry_extract (&entry, join_parent, include_kind, include_metadata, follow));
		let entry = try! (filesystem_directory_entry_value (None, entry_path, entry_kind, entry_metadata));
		entries.push (entry);
	}
	if sort {
		#[ cfg ( feature = "vonuvoli_eqord" ) ]
		entries.sort ();
	}
	return build_list_or_array (entries, return_array);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_directory_fold (path : &Value, callable : &Value, accumulator : &Value, join_parent : bool, include_kind : bool, include_metadata : bool, follow : bool, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let mut accumulator = accumulator.clone ();
	for entry in try_or_fail! (fs::read_dir (path), 0xb051b19b) {
		let entry = try_or_fail! (entry, 0xe5411945);
		let (entry_path, entry_kind, entry_metadata) = try! (filesystem_directory_entry_extract (&entry, join_parent, include_kind, include_metadata, follow));
		accumulator = try! (filesystem_directory_entry_fold (None, entry_path, entry_kind, entry_metadata, callable, &accumulator, evaluator));
	}
	succeed! (accumulator);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_directory_fold_recursive (path : &Value, callable : &Value, recurse : &Value, accumulator : &Value, join_parent : bool, include_kind : bool, include_metadata : bool, follow : bool, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let mut accumulator = accumulator.clone ();
	let recurse = if is_false (recurse) { None } else { Some (recurse) };
	let mut stack = StdVec::new ();
	stack.push ((path.to_path_buf (), try_or_fail! (fs::read_dir (path), 0xa7282b4e)));
	loop {
		let (pop, push) = if let Some (&mut (ref path, ref mut entries)) = stack.last_mut () {
			if let Some (entry) = entries.next () {
				let entry = try_or_fail! (entry, 0x0a10646f);
				let parent_path = if join_parent { None } else { Some (Path::new_from_ref (path, false) .into ()) };
				let parent_path = parent_path.as_ref ();
				let (entry_path, entry_kind, entry_metadata) = try! (filesystem_directory_entry_extract (&entry, join_parent, true, include_metadata, follow));
				let entry_kind_for_evaluation = if include_kind { entry_kind } else { None };
				let entry_kind = try_some_or_panic! (entry_kind, 0xcd8addd3, github_issue_new);
				if entry_kind.is_dir () {
					if let Some (recurse) = recurse {
						accumulator = try! (filesystem_directory_entry_fold (parent_path, entry_path.clone (), entry_kind_for_evaluation, entry_metadata.clone (), callable, &accumulator, evaluator));
						let outcome = try! (filesystem_directory_entry_fold (parent_path, entry_path.clone (), entry_kind_for_evaluation, entry_metadata, recurse, &accumulator, evaluator));
						if is_false (&outcome) {
							(false, None)
						} else {
							(false, Some (entry.path ()))
						}
					} else {
						accumulator = try! (filesystem_directory_entry_fold (parent_path, entry_path, entry_kind_for_evaluation, entry_metadata, callable, &accumulator, evaluator));
						(false, Some (entry.path ()))
					}
				} else {
					accumulator = try! (filesystem_directory_entry_fold (parent_path, entry_path, entry_kind_for_evaluation, entry_metadata, callable, &accumulator, evaluator));
					(false, None)
				}
			} else {
				(true, None)
			}
		} else {
			break;
		};
		if pop {
			stack.pop ();
		}
		if let Some (path) = push {
			let entries = try_or_fail! (fs::read_dir (&path), 0x7e0522c8);
			stack.push ((path, entries));
		}
	}
	succeed! (accumulator);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn filesystem_directory_entry_extract (entry : &fs::DirEntry, join_parent : bool, include_kind : bool, include_metadata : bool, follow : bool) -> (Outcome<(fs_path::PathBuf, Option<fs::FileType>, Option<fs::Metadata>)>) {
	let entry_path = if join_parent {
		entry.path ()
	} else {
		entry.file_name () .into ()
	};
	
	let (entry_kind, entry_metadata) = if include_kind {
		let entry_kind = try_or_fail! (entry.file_type (), 0x0f94cd6c);
		if follow && entry_kind.is_symlink () {
			match fs::metadata (& entry.path ()) {
				Ok (entry_metadata) => {
					let entry_kind = entry_metadata.file_type ();
					(Some (entry_kind), Some (entry_metadata))
				},
				Err (error) =>
					match error.raw_os_error () {
						Some (ext::libc::ENOENT) =>
							(Some (entry_kind), None),
						_ =>
							fail! (0xcd07dbae),
					},
			}
		} else {
			(Some (entry_kind), None)
		}
	} else {
		(None, None)
	};
	let entry_metadata = if include_metadata {
		if let Some (entry_metadata) = entry_metadata {
			Some (entry_metadata)
		} else {
			let entry_metadata = if ! follow {
				try_or_fail! (entry.metadata (), 0xe4f53f27)
			} else {
				match fs::metadata (& entry.path ()) {
					Ok (entry_metadata) =>
						entry_metadata,
					Err (error) =>
						match error.raw_os_error () {
							Some (ext::libc::ENOENT) =>
								try_or_fail! (entry.metadata (), 0xa34903bc),
							_ =>
								fail! (0x23262f7f),
						},
				}
			};
			Some (entry_metadata)
		}
	} else {
		None
	};
	succeed! ((entry_path, entry_kind, entry_metadata));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn filesystem_directory_entry_value (parent : Option<&Value>, entry_path : fs_path::PathBuf, entry_kind : Option<fs::FileType>, entry_metadata : Option<fs::Metadata>) -> (Outcome<Value>) {
	let entry_path = Path::new_from_buffer (entry_path, false);
	let entry_path = if let Some (parent) = parent {
		pair_new (parent.clone (), entry_path.into (), None) .into ()
	} else {
		entry_path.into ()
	};
	let entry = match (entry_kind, entry_metadata) {
		(Some (entry_kind), Some (entry_metadata)) =>
			pair_new (
					entry_path,
					pair_new (
							Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
							opaque_new (entry_metadata) .into (),
							None,
						) .into (),
					None,
				) .into (),
		(Some (entry_kind), None) =>
			pair_new (
					entry_path,
					Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
					None,
				) .into (),
		(None, Some (entry_metadata)) =>
			pair_new (
					entry_path,
					opaque_new (entry_metadata) .into (),
					None,
				) .into (),
		(None, None) =>
			entry_path.into (),
	};
	succeed! (entry);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn filesystem_directory_entry_fold (parent : Option<&Value>, entry_path : fs_path::PathBuf, entry_kind : Option<fs::FileType>, entry_metadata : Option<fs::Metadata>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let entry_path = Path::new_from_buffer (entry_path, false);
	match (parent, entry_kind, entry_metadata) {
		(Some (parent), Some (entry_kind), Some (entry_metadata)) =>
			return evaluator.evaluate_procedure_call_5 (
					callable,
					accumulator,
					parent,
					& entry_path.into (),
					& Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
					& opaque_new (entry_metadata) .into (),
				),
		(None, Some (entry_kind), Some (entry_metadata)) =>
			return evaluator.evaluate_procedure_call_4 (
					callable,
					accumulator,
					& entry_path.into (),
					& Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
					& opaque_new (entry_metadata) .into (),
				),
		(Some (parent), Some (entry_kind), None) =>
			return evaluator.evaluate_procedure_call_4 (
					callable,
					accumulator,
					parent,
					& entry_path.into (),
					& Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
				),
		(None, Some (entry_kind), None) =>
			return evaluator.evaluate_procedure_call_3 (
					callable,
					accumulator,
					& entry_path.into (),
					& Symbol::from (& try! (FileSystemMetadataKind::try_from (&entry_kind))) .into (),
				),
		(Some (parent), None, Some (entry_metadata)) =>
			return evaluator.evaluate_procedure_call_4 (
					callable,
					accumulator,
					parent,
					& entry_path.into (),
					& opaque_new (entry_metadata) .into (),
				),
		(None, None, Some (entry_metadata)) =>
			return evaluator.evaluate_procedure_call_3 (
					callable,
					accumulator,
					& entry_path.into (),
					& opaque_new (entry_metadata) .into (),
				),
		(Some (parent), None, None) =>
			return evaluator.evaluate_procedure_call_3 (
					callable,
					accumulator,
					parent,
					& entry_path.into (),
				),
		(None, None, None) =>
			return evaluator.evaluate_procedure_call_2 (
					callable,
					accumulator,
					& entry_path.into (),
				),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_mountpoint_is (path : &Value, follow : bool) -> (Outcome<bool>) {
	//  NOTE:  Implementation inspired by Python's `os.path.Path.is_mount` at:
	//           https://github.com/python/cpython/blob/master/Lib/pathlib.py
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let path_metadata = try_or_fail! (if follow { fs::metadata (path) } else { fs::symlink_metadata (path) }, 0x849f27cb);
	if ! path_metadata.is_dir () {
		fail! (0xaa0728e0);
	}
	let parent = path.join (&fs_path::Component::ParentDir);
	let parent = &parent;
	let parent_metadata = try_or_fail! (if follow { fs::metadata (parent) } else { fs::symlink_metadata (parent) }, 0xa1d88e96);
	if ! parent_metadata.is_dir () {
		fail! (0xbe8ff62c);
	}
	if path_metadata.dev () != parent_metadata.dev () {
		succeed! (true);
	}
	if path_metadata.ino () == parent_metadata.ino () {
		succeed! (true);
	}
	succeed! (false);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
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

#[ cfg ( feature = "vonuvoli_values_string" ) ]
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

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_path_to_bytes (value : &Value) -> (Outcome<Value>) {
	let value = try_as_path_ref! (value);
	let value = value.internal_rc_clone ();
	let value = unsafe { mem::transmute (value) };
	let value = BytesImmutable::from_rc (value);
	succeed! (value.into ());
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
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
			fail_panic! (0x27130fff, github_issue_new);
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
pub fn filesystem_metadata_file_get_size (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let size = try! (filesystem_metadata_file_get_size_0 (metadata, follow));
	TODO! ("add support for `u64` numbers");
	let size = try! (NumberInteger::try_from (size));
	succeed! (size.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_file_get_size_0 (metadata : &Value, follow : bool) -> (Outcome<u64>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	if ! metadata.is_file () {
		fail! (0x128abc17);
	}
	let size = metadata.len ();
	succeed! (size);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_file_is_empty (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let size = try! (filesystem_metadata_file_get_size_0 (metadata, follow));
	succeed! (size == 0);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_file_is_not_empty (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let size = try! (filesystem_metadata_file_get_size_0 (metadata, follow));
	succeed! (size != 0);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_is_readonly (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let permissions = metadata.permissions ();
	let readonly = permissions.readonly ();
	succeed! (readonly);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_is_readable (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let permissions = try! (filesystem_metadata_unix_get_permissions_for_current_process (&metadata));
	if (permissions & 0b100) != 0 {
		succeed! (true);
	} else {
		succeed! (false);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_is_writeable (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let permissions = try! (filesystem_metadata_unix_get_permissions_for_current_process (&metadata));
	if (permissions & 0b010) != 0 {
		succeed! (true);
	} else {
		succeed! (false);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_file_is_executable (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	if ! metadata.is_file () {
		fail! (0x1f73f72c);
	}
	let permissions = try! (filesystem_metadata_unix_get_permissions_for_current_process (&metadata));
	if (permissions & 0b001) != 0 {
		succeed! (true);
	} else {
		succeed! (false);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_directory_is_traversable (metadata : &Value, follow : bool) -> (Outcome<bool>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	if ! metadata.is_dir () {
		fail! (0xd42d667f);
	}
	let permissions = try! (filesystem_metadata_unix_get_permissions_for_current_process (&metadata));
	if (permissions & 0b001) != 0 {
		succeed! (true);
	} else {
		succeed! (false);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_permissions_for_current_process (metadata : &fs::Metadata) -> (Outcome<u8>) {
	let permissions = metadata.mode () & 0b111_000_000;
	let process_uid = libc_geteuid ();
	let metadata_uid = metadata.uid ();
	if process_uid == metadata_uid {
		let permissions = (permissions & 0b111_000_000) >> 6;
		succeed! (permissions as u8);
	}
	let process_gid = libc_getegid ();
	let metadata_gid = metadata.gid ();
	if process_gid == metadata_gid {
		let permissions = (permissions & 0b000_111_000) >> 3;
		succeed! (permissions as u8);
	}
	for process_gid in libc_getgroups () .iter () {
		if *process_gid == metadata_gid {
			let permissions = (permissions & 0b000_111_000) >> 3;
			succeed! (permissions as u8);
		}
	}
	let permissions = (permissions & 0b000_000_111) >> 0;
	succeed! (permissions as u8);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_mode (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let mode = metadata.mode ();
	let mode = NumberInteger::from (mode);
	succeed! (mode.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_type (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let kind = metadata.mode () & ext::libc::S_IFMT;
	let kind = NumberInteger::from (kind);
	succeed! (kind.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_permissions (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let permissions = metadata.mode () & 0b111_111_111_111;
	let permissions = NumberInteger::from (permissions);
	succeed! (permissions.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_user_identifier (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let identifier = metadata.uid ();
	let identifier = NumberInteger::from (identifier);
	succeed! (identifier.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_group_identifier (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let identifier = metadata.gid ();
	let identifier = NumberInteger::from (identifier);
	succeed! (identifier.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_data_accessed_at (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let timestamp = metadata.atime ();
	if timestamp < 0 {
		fail_panic! (0x689c9eb8, github_issue_new);
	}
	let timestamp = NumberInteger::from (timestamp);
	succeed! (timestamp.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_data_modified_at (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let timestamp = metadata.mtime ();
	if timestamp < 0 {
		fail_panic! (0x01d8a0c5, github_issue_new);
	}
	let timestamp = NumberInteger::from (timestamp);
	succeed! (timestamp.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_inode_changed_at (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let timestamp = metadata.ctime ();
	if timestamp < 0 {
		fail_panic! (0x29b73822, github_issue_new);
	}
	let timestamp = NumberInteger::from (timestamp);
	succeed! (timestamp.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_inode_device (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let device = metadata.dev ();
	TODO! ("add support for `u64` numbers");
	let device = try! (NumberInteger::try_from (device));
	succeed! (device.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_inode_number (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let number = metadata.ino ();
	TODO! ("add support for `u64` numbers");
	let number = try! (NumberInteger::try_from (number));
	succeed! (number.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_unix_get_inode_links (metadata : &Value, follow : bool) -> (Outcome<Value>) {
	let metadata = try! (filesystem_metadata_coerce (metadata, follow));
	let metadata = metadata.deref ();
	let links = metadata.nlink ();
	TODO! ("add support for `u64` numbers");
	let links = try! (NumberInteger::try_from (links));
	succeed! (links.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_is (value : &Value) -> (bool) {
	return Opaque::value_is::<fs::Metadata> (value);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_metadata_is_self (left : &Value, right : &Value, follow : bool) -> (Outcome<bool>) {
	let left = try! (filesystem_metadata_coerce (left, follow));
	let right = try! (filesystem_metadata_coerce (right, follow));
	let left = left.deref ();
	let right = right.deref ();
	if left.dev () != right.dev () {
		succeed! (false);
	}
	if left.ino () != right.ino () {
		succeed! (false);
	}
	succeed! (true);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_any_exists (path : &Value, follow : bool) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	match
			if follow {
				fs::metadata (path)
			} else {
				fs::symlink_metadata (path)
			}
	{
		Ok (_) =>
			succeed! (true),
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (false),
				_ =>
					fail! (0xe00c6835),
			},
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_exists (path : &Value, follow : bool) -> (Outcome<bool>) {
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
			if metadata.is_file () {
				succeed! (true);
			} else {
				fail! (0xf2e70d12);
			},
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (false),
				_ =>
					fail! (0x7b7860f8),
			},
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_file_delete (path : &Value) -> (Outcome<()>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	succeed_or_fail! (fs::remove_file (path), 0xa1653696);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_directory_exists (path : &Value, follow : bool) -> (Outcome<bool>) {
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
			if metadata.is_dir () {
				succeed! (true);
			} else {
				fail! (0x0a0ddaa7);
			},
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (false),
				_ =>
					fail! (0x06e80918),
			},
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_directory_delete (path : &Value) -> (Outcome<()>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	succeed_or_fail! (fs::remove_dir (path), 0x9f527eb4);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_symlink_exists (path : &Value) -> (Outcome<bool>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	match fs::symlink_metadata (path) {
		Ok (metadata) =>
			if metadata.file_type () .is_symlink () {
				succeed! (true);
			} else {
				fail! (0x1028079e);
			},
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					succeed! (false),
				_ =>
					fail! (0xe277c8cb),
			},
	}
}




#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
enum TemporaryLock {
	File (StdRefCell<Option<ext::tempfile::TempPath>>),
	Directory (StdRefCell<Option<ext::tempfile::TempDir>>),
}


#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_temporary_create_file (parent : Option<&Value>, prefix : Option<&Value>, suffix : Option<&Value>) -> (Outcome<(Path, Opaque)>) {
	let wrapper = try! (filesystem_temporary_build (parent, prefix, suffix,
		|parent, builder| {
			let wrapper = if let Some (parent) = parent {
				try_or_fail! (builder.tempfile_in (parent), 0x7c8f4dc1)
			} else {
				try_or_fail! (builder.tempfile (), 0x2b197ad1)
			};
			let wrapper = wrapper.into_temp_path ();
			succeed! (wrapper);
		}));
	let path = Path::new_from_ref (wrapper.deref (), true);
	let lock = TemporaryLock::File (StdRefCell::new (Some (wrapper)));
	let lock = opaque_new (lock);
	succeed! ((path, lock));
}


#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_temporary_create_directory (parent : Option<&Value>, prefix : Option<&Value>, suffix : Option<&Value>) -> (Outcome<(Path, Opaque)>) {
	let wrapper = try! (filesystem_temporary_build (parent, prefix, suffix,
		|parent, builder| {
			let wrapper = if let Some (parent) = parent {
				try_or_fail! (builder.tempdir_in (parent), 0x06b4f0bb)
			} else {
				try_or_fail! (builder.tempdir (), 0xdb02e235)
			};
			succeed! (wrapper);
		}));
	let path = Path::new_from_ref (wrapper.path (), true);
	let lock = TemporaryLock::Directory (StdRefCell::new (Some (wrapper)));
	let lock = opaque_new (lock);
	succeed! ((path, lock));
}


#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn filesystem_temporary_build <Thunk, ThunkOutput> (parent : Option<&Value>, prefix : Option<&Value>, suffix : Option<&Value>, thunk : Thunk) -> (Outcome<ThunkOutput>)
		where Thunk : Fn (Option<&fs_path::Path>, &ext::tempfile::Builder) -> (Outcome<ThunkOutput>)
{
	let parent = try! (value_coerce_option_or_boolean (parent, None, Some (None)));
	let parent = try_option_map! (parent, path_slice_coerce (parent));
	let parent = option_ref_map! (parent, parent.deref ());
	let prefix = try! (value_coerce_option_or_boolean (prefix, None, Some (None)));
	let prefix = try_option_map! (prefix, path_name_slice_coerce (prefix));
	let prefix = option_ref_map! (prefix, prefix.deref ());
	let suffix = try! (value_coerce_option_or_boolean (suffix, None, Some (None)));
	let suffix = try_option_map! (suffix, path_name_slice_coerce (suffix));
	let suffix = option_ref_map! (suffix, suffix.deref ());
	let mut builder = ext::tempfile::Builder::new ();
	if let Some (prefix) = prefix {
		TODO! ("the `tempfile` crate requires for the moment an `str`");
		let prefix = try_some! (prefix.to_str (), 0x7ba86ec6);
		builder.prefix (prefix);
	}
	if let Some (suffix) = suffix {
		TODO! ("the `tempfile` crate requires for the moment an `str`");
		let suffix = try_some! (suffix.to_str (), 0x7eb9f789);
		builder.suffix (suffix);
	}
	builder.rand_bytes (8);
	return thunk (parent, &builder);
}


#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn filesystem_temporary_release (lock : &Value) -> (Outcome<()>) {
	let lock = try_as_opaque_ref! (lock);
	let lock : &TemporaryLock = try_some! (lock.downcast (), 0x34b45f6e);
	match lock {
		TemporaryLock::File (ref lock) => {
			let mut lock = try_or_fail! (StdRefCell::try_borrow_mut (lock), 0xa311e011);
			if let Some (lock) = lock.take () {
				try_or_fail! (lock.close (), 0x47bb78fb);
			}
			succeed! (());
		},
		TemporaryLock::Directory (ref lock) => {
			let mut lock = try_or_fail! (StdRefCell::try_borrow_mut (lock), 0x86ac7dee);
			if let Some (lock) = lock.take () {
				try_or_fail! (lock.close (), 0xdc88e3e5);
			}
			succeed! (());
		}
	}
}

