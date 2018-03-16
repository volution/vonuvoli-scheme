

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_characters::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{String, StringRef, StringAsRef, StringImmutable, StringMutable, StringMutableInternals};
	pub use super::{StringMatchAsRef, StringMatchInto, StringMatchAsRef2};
	pub use super::{string_immutable_new, string_immutable_new_empty, string_immutable_clone_str, string_immutable_clone_characters};
	pub use super::{string_mutable_new, string_mutable_new_empty, string_mutable_clone_str, string_mutable_clone_characters};
	pub use super::{string_new, string_new_empty, string_clone_str, string_clone_characters};
	pub use super::{StringIterator, StringIterators};
}




pub enum StringMatchAsRef <'a> {
	Immutable (&'a StringImmutable),
	Mutable (&'a StringMutable),
}


pub enum StringMatchInto {
	Immutable (StringImmutable),
	Mutable (StringMutable),
}


pub enum StringMatchAsRef2 <'a> {
	ImmutableBoth (&'a StringImmutable, &'a StringImmutable),
	MutableBoth (&'a StringMutable, &'a StringMutable),
	ImmutableAndMutable (&'a StringImmutable, &'a StringMutable),
	MutableAndImmutable (&'a StringMutable, &'a StringImmutable),
}


impl <'a> StringMatchAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref (&self) -> (Outcome<StringRef<'a>>) {
		match *self {
			StringMatchAsRef::Immutable (value) =>
				succeed! (value.string_ref ()),
			StringMatchAsRef::Mutable (value) =>
				return value.string_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_as_ref (self) -> (StringAsRef<'a>) {
		match self {
			StringMatchAsRef::Immutable (value) =>
				StringAsRef::Immutable (value),
			StringMatchAsRef::Mutable (value) =>
				StringAsRef::Mutable (value),
		}
	}
}


impl <'a> StringMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref (&self) -> (Outcome<(StringRef<'a>, StringRef<'a>)>) {
		match *self {
			StringMatchAsRef2::ImmutableBoth (left, right) =>
				succeed! ((left.string_ref (), right.string_ref ())),
			StringMatchAsRef2::MutableBoth (left, right) =>
				succeed! ((try! (left.string_ref ()), try! (right.string_ref ()))),
			StringMatchAsRef2::ImmutableAndMutable (left, right) =>
				succeed! ((left.string_ref (), try! (right.string_ref ()))),
			StringMatchAsRef2::MutableAndImmutable (left, right) =>
				succeed! ((try! (left.string_ref ()), right.string_ref ())),
		}
	}
}


impl StringMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			StringMatchInto::Immutable (value) =>
				value.into (),
			StringMatchInto::Mutable (value) =>
				value.into (),
		}
	}
}




pub trait String {
	
	fn string_as_str (&self) -> (&str);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_as_bytes (&self) -> (&[u8]) {
		self.string_as_str () .as_bytes ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_chars (&self) -> (str::Chars) {
		self.string_as_str () .chars ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_clone (&self) -> (StdString) {
		self.string_as_str () .to_string ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_is_empty (&self) -> (bool) {
		self.string_as_str () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_is_not_empty (&self) -> (bool) {
		! self.string_as_str () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_eq (&self, other : &str) -> (bool) {
		self.string_as_str () .eq (other)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_utf8_bytes_count (&self) -> (usize) {
		self.string_as_str () .len ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_chars_count_compute (&self) -> (usize) {
		self.string_chars () .count ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_char_at_compute (&self, index : usize) -> (Option<char>) {
		self.string_chars () .nth (index)
	}
}




pub enum StringRef <'a> {
	Immutable (&'a StringImmutable, &'a str),
	Mutable (&'a StringMutable, StdRef<'a, str>),
}


impl <'a> StringRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<StringRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::StringImmutable (value) =>
				succeed! (value.string_ref ()),
			ValueKindMatchAsRef::StringMutable (value) =>
				return value.string_ref (),
			_ =>
				fail! (0x20d78ff4),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			StringRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			StringRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_as_str (&self) -> (&str) {
		match *self {
			StringRef::Immutable (_, string) =>
				string,
			StringRef::Mutable (_, ref string) =>
				string,
		}
	}
}




pub enum StringAsRef <'a> {
	Immutable (&'a StringImmutable),
	Mutable (&'a StringMutable),
}


impl <'a> StringAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<StringAsRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::StringImmutable (value) =>
				succeed! (StringAsRef::Immutable (value)),
			ValueKindMatchAsRef::StringMutable (value) =>
				succeed! (StringAsRef::Mutable (value)),
			_ =>
				fail! (0x2aacd51d),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref (&self) -> (Outcome<StringRef<'a>>) {
		match *self {
			StringAsRef::Immutable (value) =>
				succeed! (value.string_ref ()),
			StringAsRef::Mutable (value) =>
				return value.string_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			StringAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			StringAsRef::Mutable (value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_rc_clone (&self) -> (Outcome<StdRc<StdBox<str>>>) {
		match *self {
			StringAsRef::Immutable (value) =>
				succeed! (value.string_rc_clone ()),
			StringAsRef::Mutable (value) =>
				succeed! (try_or_fail! ((value.0) .as_ref () .try_borrow_mut (), 0xf34ad2b8) .to_cow ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<StringImmutable>) {
		match *self {
			StringAsRef::Immutable (value) =>
				succeed! ((*value) .clone ()),
			StringAsRef::Mutable (value) =>
				return (*value) .to_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (StringMutable) {
		match *self {
			StringAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			StringAsRef::Mutable (value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &StringAsRef) -> (bool) {
		match (self, other) {
			(&StringAsRef::Immutable (self_0), &StringAsRef::Immutable (other_0)) =>
				StringImmutable::is_self (self_0, other_0),
			(&StringAsRef::Mutable (self_0), &StringAsRef::Mutable (other_0)) =>
				StringMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct StringImmutable ( StdRc<StdBox<str>> );


impl StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<str>>) -> (StringImmutable) {
		StringImmutable (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<str>>) -> (StringImmutable) {
		StringImmutable::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &StringImmutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref (&self) -> (StringRef) {
		StringRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (StringMutable) {
		StringMutable::from_rc (self.string_rc_clone ())
	}
}


impl String for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct StringMutable ( StdRc<StdRefCell<StringMutableInternals>> );


#[ derive (Debug) ]
pub enum StringMutableInternals {
	Owned (StdString),
	Cow (StdRc<StdBox<str>>),
}


impl StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<str>>) -> (StringMutable) {
		let internals = StringMutableInternals::Cow (rc);
		StringMutable (StdRc::new (StdRefCell::new (internals)))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<str>>) -> (StringMutable) {
		StringMutable::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &StringMutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref (&self) -> (Outcome<StringRef>) {
		let reference = try_or_fail! (self.0.as_ref () .try_borrow (), 0xb5044e71);
		let reference = StdRef::map (reference, |reference| reference.as_ref ());
		succeed! (StringRef::Mutable (self, reference));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdRefCell<StringMutableInternals>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_ref_mut (&self) -> (Outcome<StdRefMut<StdString>>) {
		let reference = try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0x050c6ef5);
		let reference = StdRefMut::map (reference, |reference| reference.as_mut ());
		succeed! (reference);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<StringImmutable>) {
		let mut reference = try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0xfb81994f);
		let string = reference.to_cow ();
		succeed! (StringImmutable::from_rc (string));
	}
}


impl StringMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn to_cow (&mut self) -> (StdRc<StdBox<str>>) {
		let string_cow = match *self {
			StringMutableInternals::Owned (ref mut string_owned) => {
				let mut string_swap = StdString::new ();
				mem::swap (&mut string_swap, string_owned);
				let string_swap = string_swap.into_boxed_str ();
				string_swap
			},
			StringMutableInternals::Cow (ref mut string) =>
				return string.clone (),
		};
		*self = StringMutableInternals::Cow (StdRc::new (string_cow));
		return self.to_cow ();
	}
}


impl StdAsRef<str> for StringMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&str) {
		match *self {
			StringMutableInternals::Owned (ref string) =>
				string.deref (),
			StringMutableInternals::Cow (ref string) =>
				string.deref (),
		}
	}
}


impl StdAsRefMut<StdString> for StringMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_mut (&mut self) -> (&mut StdString) {
		let string_owned = match *self {
			StringMutableInternals::Owned (ref mut string) =>
				return string,
			StringMutableInternals::Cow (ref mut string_cow) => {
				let string_cow = StdRc::make_mut (string_cow);
				let mut string_swap = StdString::new () .into_boxed_str ();
				mem::swap (&mut string_swap, string_cow);
				let string_swap = StdString::from (string_swap);
				string_swap
			},
		};
		*self = StringMutableInternals::Owned (string_owned);
		return self.as_mut ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_new (string : StdString) -> (StringImmutable) {
	StringImmutable (StdRc::new (string.into_boxed_str ()))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_new (string : StdString) -> (StringMutable) {
	let internals = StringMutableInternals::Owned (string);
	StringMutable (StdRc::new (StdRefCell::new (internals)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_new (string : StdString) -> (Value) {
	if STRING_NEW_IMMUTABLE {
		string_immutable_new (string) .into ()
	} else {
		string_mutable_new (string) .into ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_new_empty () -> (StringImmutable) {
	string_immutable_new (StdString::new ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_new_empty () -> (StringMutable) {
	string_mutable_new (StdString::new ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_new_empty () -> (Value) {
	if STRING_NEW_IMMUTABLE {
		string_immutable_new_empty () .into ()
	} else {
		string_mutable_new_empty () .into ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_clone_str (string : &str) -> (StringImmutable) {
	string_immutable_new (StdString::from (string))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_clone_str (string : &str) -> (StringMutable) {
	string_mutable_new (StdString::from (string))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_clone_str (string : &str) -> (Value) {
	if STRING_NEW_IMMUTABLE {
		string_immutable_clone_str (string) .into ()
	} else {
		string_mutable_clone_str (string) .into ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_immutable_clone_characters (characters : &[char]) -> (StringImmutable) {
	string_immutable_new (unicode_utf8_chars_clone_string (characters))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_mutable_clone_characters (characters : &[char]) -> (StringMutable) {
	string_mutable_new (unicode_utf8_chars_clone_string (characters))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_clone_characters (characters : &[char]) -> (Value) {
	if STRING_NEW_IMMUTABLE {
		string_immutable_clone_characters (characters) .into ()
	} else {
		string_mutable_clone_characters (characters) .into ()
	}
}




pub struct StringIterator <'a> ( StringRef<'a>, str::Chars<'a> );


impl <'a> StringIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (string : &'a Value) -> (Outcome<StringIterator<'a>>) {
		let string = try_as_string_ref! (string);
		return StringIterator::new_a (string);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_a (string : StringRef<'a>) -> (Outcome<StringIterator<'a>>) {
		let iterator = unsafe { mem::transmute (string.string_chars ()) };
		succeed! (StringIterator (string, iterator));
	}
}


impl <'a> iter::Iterator for StringIterator <'a> {
	
	type Item = Outcome<Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (strings : &'a [&'a Value]) -> (Outcome<StringIterators<'a>>) {
		let iterators = try! (strings.iter () .map (|string| StringIterator::new (string)) .collect ());
		succeed! (StringIterators (iterators));
	}
}


impl <'a> iter::Iterator for StringIterators <'a> {
	
	type Item = Outcome<StdVec<Value>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

