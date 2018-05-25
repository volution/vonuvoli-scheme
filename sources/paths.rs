

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		Path,
	};
	
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Path ( StdRc<StdBox<fs_path::Path>> );




impl Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_raw (path : fs_path::PathBuf, normalize : bool) -> (Path) {
		if normalize && path.as_os_str () .is_empty () {
			panic_0! (0xba1ee991, (github_issue, 49));
		}
		Path (StdRc::new (path.into_boxed_path ()))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_buffer (path : fs_path::PathBuf, normalize : bool) -> (Path) {
		Path::new_from_components (&mut path.components (), normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_ref (path : &fs_path::Path, normalize : bool) -> (Path) {
		Path::new_from_components (&mut path.components (), normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_components (path : &mut iter::Iterator<Item = fs_path::Component>, normalize : bool) -> (Path) {
		let mut buffer = fs_path::PathBuf::new ();
		for component in path {
			buffer.push (component.as_os_str ());
		}
		Path::new_from_raw (buffer, normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_component (path : &fs_path::Component, normalize : bool) -> (Path) {
		let mut buffer = fs_path::PathBuf::new ();
		buffer.push (path.as_os_str ());
		Path::new_from_raw (buffer, normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_root () -> (Path) {
		Path::new_from_component (&fs_path::Component::RootDir, false)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_current () -> (Path) {
		Path::new_from_component (&fs_path::Component::CurDir, false)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_parent () -> (Path) {
		Path::new_from_component (&fs_path::Component::ParentDir, false)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_absolute (&self) -> (bool) {
		self.path_ref () .is_absolute ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_relative (&self) -> (bool) {
		self.path_ref () .is_relative ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn path_ref (&self) -> (&fs_path::Path) {
		&self.0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_string_rc (rc : StdRc<StdBox<str>>, normalize : bool) -> (Path) {
		if normalize && rc.is_empty () {
			panic_0! (0x6f442154, (github_issue, 49));
		}
		if normalize {
			FIXME! ("check if the normalized path is the same and if so keep the current `rc`");
			let path = rc.deref () .deref ();
			let path = fs_path::Path::new (path);
			Path::new_from_ref (path, normalize)
		} else {
			let rc = unsafe { mem::transmute (rc) };
			Path (rc)
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_string_rc (rc : &StdRc<StdBox<str>>, normalize : bool) -> (Path) {
		Path::from_string_rc (StdRc::clone (rc), normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_bytes_rc (rc : StdRc<StdBox<[u8]>>, normalize : bool) -> (Path) {
		if normalize && rc.is_empty () {
			panic_0! (0xbdff9c23, (github_issue, 49));
		}
		if normalize {
			FIXME! ("check if the normalized path is the same and if so keep the current `rc`");
			let path = rc.deref () .deref ();
			let path = fs_path::Path::new (ffi::OsStr::from_bytes (path));
			Path::new_from_ref (path, normalize)
		} else {
			let rc = unsafe { mem::transmute (rc) };
			Path (rc)
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_bytes_rc (rc : &StdRc<StdBox<[u8]>>, normalize : bool) -> (Path) {
		Path::from_bytes_rc (StdRc::clone (rc), normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<fs_path::Path>>, normalize : bool) -> (Path) {
		if normalize && rc.as_os_str () .is_empty () {
			panic_0! (0xe4a2aadd, (github_issue, 49));
		}
		if normalize {
			FIXME! ("check if the normalized path is the same and if so keep the current `rc`");
			let path = &rc;
			Path::new_from_ref (path, normalize)
		} else {
			Path (rc)
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<fs_path::Path>>, normalize : bool) -> (Path) {
		Path::from_rc (StdRc::clone (rc), normalize)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Path) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internal_rc_clone (&self) -> (StdRc<StdBox<fs_path::Path>>) {
		StdRc::clone (&self.0)
	}
}

