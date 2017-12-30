

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_numbers::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Bytes, BytesRef, BytesImmutable, BytesMutable};
	pub use super::{bytes_immutable_new, bytes_immutable_clone_slice, bytes_immutable_clone_str, bytes_immutable_clone_characters};
	pub use super::{bytes_mutable_new, bytes_mutable_clone_slice, bytes_mutable_clone_str, bytes_mutable_clone_characters};
	pub use super::{bytes_new, bytes_clone_slice, bytes_clone_str, bytes_clone_characters};
	pub use super::{BytesIterator, BytesIterators};
}




pub trait Bytes {
	
	fn bytes_as_vec (&self) -> (&StdVec<u8>);
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_as_slice (&self) -> (&[u8]) {
		self.bytes_as_vec () .as_slice ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_iter (&self) -> (slice::Iter<u8>) {
		self.bytes_as_vec () .iter ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_clone (&self) -> (StdVec<u8>) {
		self.bytes_as_vec () .clone ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_is_empty (&self) -> (bool) {
		self.bytes_as_vec () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_is_not_empty (&self) -> (bool) {
		! self.bytes_as_vec () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_count (&self) -> (usize) {
		self.bytes_as_vec () .len ()
	}
}




#[ derive (Debug) ]
pub enum BytesRef <'a> {
	Immutable (&'a BytesImmutable, &'a StdVec<u8>),
	Mutable (&'a BytesMutable, StdRef<'a, StdVec<u8>>),
}


impl <'a> BytesRef<'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try (value : &'a Value) -> (Outcome<BytesRef<'a>>) {
		match *value {
			Value::BytesImmutable (_, ref value, _) =>
				succeed! (value.bytes_ref ()),
			Value::BytesMutable (_, ref value, _) =>
				succeed! (value.bytes_ref ()),
			_ =>
				fail! (0xb6042061),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			BytesRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			BytesRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &BytesRef) -> (bool) {
		match (self, other) {
			(&BytesRef::Immutable (self_0, _), &BytesRef::Immutable (other_0, _)) =>
				BytesImmutable::is_self (self_0, other_0),
			(&BytesRef::Mutable (self_0, _), &BytesRef::Mutable (other_0, _)) =>
				BytesMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> Bytes for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_as_vec (&self) -> (&StdVec<u8>) {
		match *self {
			BytesRef::Immutable (_, bytes) =>
				bytes,
			BytesRef::Mutable (_, ref bytes) =>
				bytes,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct BytesImmutable ( StdRc<StdVec<u8>> );


impl BytesImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &BytesImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bytes_ref (&self) -> (BytesRef) {
		BytesRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdVec<u8>>) {
		self.0.clone ()
	}
}


impl Bytes for BytesImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bytes_as_vec (&self) -> (&StdVec<u8>) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct BytesMutable ( StdRc<StdRefCell<StdVec<u8>>> );


impl BytesMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &BytesMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bytes_ref (&self) -> (BytesRef) {
		BytesRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdRefCell<StdVec<u8>>>) {
		self.0.clone ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_immutable_new (bytes : StdVec<u8>) -> (BytesImmutable) {
	BytesImmutable (StdRc::new (bytes))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_mutable_new (bytes : StdVec<u8>) -> (BytesMutable) {
	BytesMutable (StdRc::new (StdRefCell::new (bytes)))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_new (bytes : StdVec<u8>) -> (Value) {
	if true {
		bytes_immutable_new (bytes) .into ()
	} else {
		bytes_mutable_new (bytes) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_immutable_clone_slice (bytes : &[u8]) -> (BytesImmutable) {
	bytes_immutable_new (vec_clone_slice (bytes))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_mutable_clone_slice (bytes : &[u8]) -> (BytesMutable) {
	bytes_mutable_new (vec_clone_slice (bytes))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_clone_slice (bytes : &[u8]) -> (Value) {
	if true {
		bytes_immutable_clone_slice (bytes) .into ()
	} else {
		bytes_mutable_clone_slice (bytes) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_immutable_clone_str (string : &str) -> (BytesImmutable) {
	bytes_immutable_new (StdString::from (string) .into_bytes ())
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_mutable_clone_str (string : &str) -> (BytesMutable) {
	bytes_mutable_new (StdString::from (string) .into_bytes ())
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_clone_str (string : &str) -> (Value) {
	if true {
		bytes_immutable_clone_str (string) .into ()
	} else {
		bytes_mutable_clone_str (string) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_immutable_clone_characters (characters : &[char]) -> (BytesImmutable) {
	bytes_immutable_new (unicode_utf8_chars_clone (characters) .into_bytes ())
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_mutable_clone_characters (characters : &[char]) -> (BytesMutable) {
	bytes_mutable_new (unicode_utf8_chars_clone (characters) .into_bytes ())
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_clone_characters (characters : &[char]) -> (Value) {
	if true {
		bytes_immutable_clone_characters (characters) .into ()
	} else {
		bytes_mutable_clone_characters (characters) .into ()
	}
}




pub struct BytesIterator <'a> ( BytesRef<'a>, slice::Iter<'a, u8> );


impl <'a> BytesIterator <'a> {
	
	pub fn new (bytes : &'a Value) -> (Outcome<BytesIterator<'a>>) {
		let bytes = try_as_bytes_ref! (bytes);
		return BytesIterator::new_a (bytes);
	}
	
	pub fn new_a (bytes : BytesRef<'a>) -> (Outcome<BytesIterator<'a>>) {
		let iterator = unsafe { mem::transmute (bytes.bytes_iter ()) };
		succeed! (BytesIterator (bytes, iterator));
	}
}


impl <'a> iter::Iterator for BytesIterator <'a> {
	
	type Item = Outcome<Value>;
	
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		if let Some (value) = self.1.next () {
			return Some (succeeded! (number_i64 (*value as i64) .into ()));
		} else {
			return None;
		}
	}
}




pub struct BytesIterators <'a> ( StdVec<BytesIterator<'a>> );


impl <'a> BytesIterators <'a> {
	
	pub fn new (bytes : &'a [&'a Value]) -> (Outcome<BytesIterators<'a>>) {
		let iterators = try! (bytes.iter () .map (|bytes| BytesIterator::new (bytes)) .collect ());
		succeed! (BytesIterators (iterators));
	}
}


impl <'a> iter::Iterator for BytesIterators <'a> {
	
	type Item = Outcome<StdVec<Value>>;
	
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
