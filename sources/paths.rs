

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
	pub fn new (path : fs_path::PathBuf) -> (Path) {
		Path (StdRc::new (path.into_boxed_path ()))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_root () -> (Path) {
		let mut path = fs_path::PathBuf::new ();
		path.push (fs_path::Component::RootDir);
		Path::new (path)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn path_ref (&self) -> (&fs_path::Path) {
		&self.0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_string_rc (rc : StdRc<StdBox<str>>) -> (Path) {
		let rc = unsafe { mem::transmute (rc) };
		Path (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_string_rc (rc : &StdRc<StdBox<str>>) -> (Path) {
		Path::from_string_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<fs_path::Path>>) -> (Path) {
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

