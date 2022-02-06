

use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_strings::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Keyword;
	pub use super::{keyword_new, keyword_clone_str, keyword_clone_characters, keyword_from_rc};
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Keyword ( StdRc<StdBox<str>> );


impl Keyword {
	
	pub fn from_rc (rc : StdRc<StdBox<str>>) -> (Keyword) {
		Keyword (rc)
	}
	
	pub fn clone_rc (rc : &StdRc<StdBox<str>>) -> (Keyword) {
		Keyword::from_rc (StdRc::clone (rc))
	}
	
	pub fn is_self (&self, other : &Keyword) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || StdRc::eq (&self.0, &other.0)
	}
	
	pub fn string_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		StdRc::clone (&self.0)
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl String for Keyword {
	
	fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
}

#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
impl Keyword {
	
	pub fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
	
	pub fn string_eq (&self, other : &str) -> (bool) {
		self.string_as_str () .eq (other)
	}
}




pub fn keyword_new (string : StdString) -> (Keyword) {
	Keyword (StdRc::new (string.into_boxed_str ()))
}

pub fn keyword_clone_str (string : &str) -> (Keyword) {
	keyword_new (StdString::from (string))
}

pub fn keyword_clone_characters (characters : &[char]) -> (Keyword) {
	keyword_new (unicode_utf8_chars_clone_string (characters))
}

pub fn keyword_from_rc (string : StdRc<StdBox<str>>) -> (Keyword) {
	Keyword::from_rc (string)
}

