

use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_strings::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Symbol;
	pub use super::{symbol_new, symbol_clone_str, symbol_clone_characters};
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Symbol ( StdRc<StdBox<str>> );


impl Symbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<str>>) -> (Symbol) {
		Symbol (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<str>>) -> (Symbol) {
		Symbol::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Symbol) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || StdRc::eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		self.0.clone ()
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl String for Symbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_new (string : StdString) -> (Symbol) {
	Symbol (StdRc::new (string.into_boxed_str ()))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_clone_str (string : &str) -> (Symbol) {
	symbol_new (StdString::from (string))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_clone_characters (characters : &[char]) -> (Symbol) {
	symbol_new (unicode_utf8_chars_clone_string (characters))
}

