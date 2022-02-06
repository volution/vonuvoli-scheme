

use super::prelude::*;




pub mod exports {
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
		StringRegex,
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
		BytesRegex,
	};
	
}




macro_rules! define_regex {
	( $type : ident, $wrapped : ty ) => {
		
		
		#[ derive ( Clone ) ] // OK
		#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
		pub struct $type ( StdRc<$wrapped> );
		
		
		impl $type {
			
			pub fn new (regex : $wrapped) -> ($type) {
				$type (StdRc::new (regex))
			}
			
			pub fn regex_ref (&self) -> (&$wrapped) {
				&self.0
			}
			
			pub fn from_rc (rc : StdRc<$wrapped>) -> ($type) {
				$type (rc)
			}
			
			pub fn clone_rc (rc : &StdRc<$wrapped>) -> ($type) {
				$type::from_rc (StdRc::clone (rc))
			}
			
			pub fn is_self (&self, other : &$type) -> (bool) {
				StdRc::ptr_eq (&self.0, &other.0)
			}
			
			pub fn internal_rc_clone (&self) -> (StdRc<$wrapped>) {
				StdRc::clone (&self.0)
			}
		}
		
		
	};
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
define_regex! (StringRegex, regex::Regex);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
define_regex! (BytesRegex, regex::bytes::Regex);

