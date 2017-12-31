

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_characters::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{String, StringRef, StringImmutable, StringMutable};
	pub use super::{string_immutable_new, string_immutable_clone_str, string_immutable_clone_characters};
	pub use super::{string_mutable_new, string_mutable_clone_str, string_mutable_clone_characters};
	pub use super::{string_new, string_clone_str, string_clone_characters};
	pub use super::{StringIterator, StringIterators};
}




pub trait String {
	
	fn string_as_string (&self) -> (&StdString);
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_as_str (&self) -> (&str) {
		self.string_as_string () .as_str ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_as_bytes (&self) -> (&[u8]) {
		self.string_as_string () .as_bytes ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_chars (&self) -> (str::Chars) {
		self.string_as_string () .chars ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_clone (&self) -> (StdString) {
		self.string_as_string () .clone ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_is_empty (&self) -> (bool) {
		self.string_as_string () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_is_not_empty (&self) -> (bool) {
		! self.string_as_string () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_eq (&self, other : &str) -> (bool) {
		self.string_as_string () .eq (other)
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_utf8_bytes_count (&self) -> (usize) {
		self.string_as_string () .len ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_chars_count_compute (&self) -> (usize) {
		self.string_chars () .count ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_char_at_compute (&self, index : usize) -> (Option<char>) {
		self.string_chars () .nth (index)
	}
}




#[ derive (Debug) ]
pub enum StringRef <'a> {
	Immutable (&'a StringImmutable, &'a StdString),
	Mutable (&'a StringMutable, StdRef<'a, StdString>),
}


impl <'a> StringRef<'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try (value : &'a Value) -> (Outcome<StringRef<'a>>) {
		match *value {
			Value::StringImmutable (_, ref value, _) =>
				succeed! (value.string_ref ()),
			Value::StringMutable (_, ref value, _) =>
				succeed! (value.string_ref ()),
			_ =>
				fail! (0x20d78ff4),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			StringRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			StringRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &StringRef) -> (bool) {
		match (self, other) {
			(&StringRef::Immutable (self_0, _), &StringRef::Immutable (other_0, _)) =>
				StringImmutable::is_self (self_0, other_0),
			(&StringRef::Mutable (self_0, _), &StringRef::Mutable (other_0, _)) =>
				StringMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> String for StringRef<'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_as_string (&self) -> (&StdString) {
		match *self {
			StringRef::Immutable (_, string) =>
				string,
			StringRef::Mutable (_, ref string) =>
				string,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct StringImmutable ( StdRc<StdString> );


impl StringImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &StringImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn string_ref (&self) -> (StringRef) {
		StringRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdString>) {
		self.0.clone ()
	}
}


impl String for StringImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn string_as_string (&self) -> (&StdString) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct StringMutable ( StdRc<StdRefCell<StdString>> );


impl StringMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &StringMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn string_ref (&self) -> (StringRef) {
		StringRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdRefCell<StdString>>) {
		self.0.clone ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_immutable_new (string : StdString) -> (StringImmutable) {
	StringImmutable (StdRc::new (string))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_mutable_new (string : StdString) -> (StringMutable) {
	StringMutable (StdRc::new (StdRefCell::new (string)))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_new (string : StdString) -> (Value) {
	if true {
		string_immutable_new (string) .into ()
	} else {
		string_mutable_new (string) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_immutable_clone_str (string : &str) -> (StringImmutable) {
	string_immutable_new (StdString::from (string))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_mutable_clone_str (string : &str) -> (StringMutable) {
	string_mutable_new (StdString::from (string))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_clone_str (string : &str) -> (Value) {
	if true {
		string_immutable_clone_str (string) .into ()
	} else {
		string_mutable_clone_str (string) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_immutable_clone_characters (characters : &[char]) -> (StringImmutable) {
	string_immutable_new (unicode_utf8_chars_clone (characters))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_mutable_clone_characters (characters : &[char]) -> (StringMutable) {
	string_mutable_new (unicode_utf8_chars_clone (characters))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn string_clone_characters (characters : &[char]) -> (Value) {
	if true {
		string_immutable_clone_characters (characters) .into ()
	} else {
		string_mutable_clone_characters (characters) .into ()
	}
}




pub struct StringIterator <'a> ( StringRef<'a>, str::Chars<'a> );


impl <'a> StringIterator <'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new (string : &'a Value) -> (Outcome<StringIterator<'a>>) {
		let string = try_as_string_ref! (string);
		return StringIterator::new_a (string);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new_a (string : StringRef<'a>) -> (Outcome<StringIterator<'a>>) {
		let iterator = unsafe { mem::transmute (string.string_chars ()) };
		succeed! (StringIterator (string, iterator));
	}
}


impl <'a> iter::Iterator for StringIterator <'a> {
	
	type Item = Outcome<Value>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		if let Some (value) = self.1.next () {
			return Some (succeeded! (character (value) .into ()));
		} else {
			return None;
		}
	}
}




pub struct StringIterators <'a> ( StdVec<StringIterator<'a>> );


impl <'a> StringIterators <'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new (strings : &'a [&'a Value]) -> (Outcome<StringIterators<'a>>) {
		let iterators = try! (strings.iter () .map (|string| StringIterator::new (string)) .collect ());
		succeed! (StringIterators (iterators));
	}
}


impl <'a> iter::Iterator for StringIterators <'a> {
	
	type Item = Outcome<StdVec<Value>>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn next (&mut self) -> (Option<Outcome<StdVec<Value>>>) {
		let mut outcomes = StdVec::with_capacity (self.0.len ());
		for mut iterator in self.0.iter_mut () {
			match iterator.next () {
				Some (Ok (outcome)) =>
					outcomes.push (outcome),
				Some (Err (error)) =>
					return Some (Err (error)),
				None =>
					return None,
			}
		}
		return Some (succeeded! (outcomes));
	}
}

