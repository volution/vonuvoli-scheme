

use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_strings::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Symbol;
	pub use super::{symbol_new, symbol_clone_str, symbol_clone_characters, symbol_from_rc};
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
// NOTE:  These traits are essential for core parts of the "engine"!  Perhaps it should be refactored...
#[ cfg_attr ( not ( feature = "vonuvoli_eqord" ), derive ( Eq, PartialEq ) ) ] // OK
#[ cfg_attr ( not ( feature = "vonuvoli_hash" ), derive ( Hash ) ) ] // OK
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

#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
impl Symbol {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_eq (&self, other : &str) -> (bool) {
		self.string_as_str () .eq (other)
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_from_rc (string : StdRc<StdBox<str>>) -> (Symbol) {
	Symbol::from_rc (string)
}

