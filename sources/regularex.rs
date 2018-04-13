

#[ allow (unused_imports) ]
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
		
		
		#[ derive (Clone, Debug) ]
		pub struct $type ( StdRc<$wrapped> );
		
		
		impl $type {
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			pub fn new (regex : $wrapped) -> ($type) {
				$type (StdRc::new (regex))
			}
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			pub fn regex_ref (&self) -> (&$wrapped) {
				&self.0
			}
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			pub fn from_rc (rc : StdRc<$wrapped>) -> ($type) {
				$type (rc)
			}
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			pub fn clone_rc (rc : &StdRc<$wrapped>) -> ($type) {
				$type::from_rc (StdRc::clone (rc))
			}
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			pub fn is_self (&self, other : &$type) -> (bool) {
				StdRc::ptr_eq (&self.0, &other.0)
			}
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

