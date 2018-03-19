

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		Path,
	};
	
}




#[ derive (Clone, Debug) ]
pub struct Path ( StdRc<StdBox<fs_path::Path>> );




impl Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_raw (path : fs_path::PathBuf) -> (Path) {
		if path.as_os_str () .is_empty () {
			panic! ("ba1ee991");
		}
		Path (StdRc::new (path.into_boxed_path ()))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_buffer (path : fs_path::PathBuf) -> (Path) {
		Path::new_from_components (&mut path.components ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_ref (path : &fs_path::Path) -> (Path) {
		Path::new_from_components (&mut path.components ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_components (path : &mut iter::Iterator<Item = fs_path::Component>) -> (Path) {
		let mut buffer = fs_path::PathBuf::new ();
		for component in path {
			buffer.push (component.as_os_str ());
		}
		Path::new_from_raw (buffer)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_component (path : &fs_path::Component) -> (Path) {
		let mut buffer = fs_path::PathBuf::new ();
		buffer.push (path.as_os_str ());
		Path::new_from_raw (buffer)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_root () -> (Path) {
		Path::new_from_component (&fs_path::Component::RootDir)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_current () -> (Path) {
		Path::new_from_component (&fs_path::Component::CurDir)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_parent () -> (Path) {
		Path::new_from_component (&fs_path::Component::ParentDir)
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
	pub fn from_string_rc (rc : StdRc<StdBox<str>>) -> (Path) {
		if rc.is_empty () {
			panic! ("6f442154");
		}
		let rc = unsafe { mem::transmute (rc) };
		Path (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_string_rc (rc : &StdRc<StdBox<str>>) -> (Path) {
		Path::from_string_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<fs_path::Path>>) -> (Path) {
		if rc.as_os_str () .is_empty () {
			panic! ("e4a2aadd");
		}
		Path (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<fs_path::Path>>) -> (Path) {
		Path::from_rc (StdRc::clone (rc))
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

