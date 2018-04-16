

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_numbers::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Bytes, BytesRef, BytesAsRef, BytesImmutable};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{BytesMutable, BytesMutableInternals};
	pub use super::{BytesMatchAsRef, BytesMatchInto, BytesMatchAsRef2};
	pub use super::{bytes_immutable_new, bytes_immutable_new_empty, bytes_immutable_clone_slice, bytes_immutable_clone_str, bytes_immutable_clone_characters};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{bytes_mutable_new, bytes_mutable_new_empty, bytes_mutable_clone_slice, bytes_mutable_clone_str, bytes_mutable_clone_characters};
	pub use super::{bytes_new, bytes_new_empty, bytes_clone_slice, bytes_clone_str, bytes_clone_characters};
	pub use super::{BytesIterator, BytesIterators};
}




pub enum BytesMatchAsRef <'a> {
	Immutable (&'a BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a BytesMutable),
}


pub enum BytesMatchInto {
	Immutable (BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (BytesMutable),
}


pub enum BytesMatchAsRef2 <'a> {
	ImmutableBoth (&'a BytesImmutable, &'a BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableBoth (&'a BytesMutable, &'a BytesMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ImmutableAndMutable (&'a BytesImmutable, &'a BytesMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableAndImmutable (&'a BytesMutable, &'a BytesImmutable),
}


impl <'a> BytesMatchAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref (&self) -> (Outcome<BytesRef<'a>>) {
		match *self {
			BytesMatchAsRef::Immutable (value) =>
				succeed! (value.bytes_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchAsRef::Mutable (value) =>
				return value.bytes_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_as_ref (self) -> (BytesAsRef<'a>) {
		match self {
			BytesMatchAsRef::Immutable (value) =>
				BytesAsRef::Immutable (value),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchAsRef::Mutable (value) =>
				BytesAsRef::Mutable (value),
		}
	}
}


impl <'a> BytesMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref (&self) -> (Outcome<(BytesRef<'a>, BytesRef<'a>)>) {
		match *self {
			BytesMatchAsRef2::ImmutableBoth (left, right) =>
				succeed! ((left.bytes_ref (), right.bytes_ref ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchAsRef2::MutableBoth (left, right) =>
				succeed! ((try! (left.bytes_ref ()), try! (right.bytes_ref ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchAsRef2::ImmutableAndMutable (left, right) =>
				succeed! ((left.bytes_ref (), try! (right.bytes_ref ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchAsRef2::MutableAndImmutable (left, right) =>
				succeed! ((try! (left.bytes_ref ()), right.bytes_ref ())),
		}
	}
}


impl BytesMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			BytesMatchInto::Immutable (value) =>
				value.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesMatchInto::Mutable (value) =>
				value.into (),
		}
	}
}




pub trait Bytes {
	
	fn bytes_as_slice (&self) -> (&[u8]);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_iter (&self) -> (slice::Iter<u8>) {
		self.bytes_as_slice () .iter ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_clone (&self) -> (StdVec<u8>) {
		self.bytes_as_slice () .to_vec ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_is_empty (&self) -> (bool) {
		self.bytes_as_slice () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_is_not_empty (&self) -> (bool) {
		! self.bytes_as_slice () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_count (&self) -> (usize) {
		self.bytes_as_slice () .len ()
	}
}




pub enum BytesRef <'a> {
	Immutable (&'a BytesImmutable, &'a [u8]),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a BytesMutable, StdRef<'a, [u8]>),
}


impl <'a> BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<BytesRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::BytesImmutable (value) =>
				succeed! (value.bytes_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::BytesMutable (value) =>
				return value.bytes_ref (),
			_ =>
				fail! (0xb6042061),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			BytesRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &BytesRef) -> (bool) {
		#[ allow (unreachable_patterns) ]
		match (self, other) {
			(&BytesRef::Immutable (self_0, _), &BytesRef::Immutable (other_0, _)) =>
				BytesImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&BytesRef::Mutable (self_0, _), &BytesRef::Mutable (other_0, _)) =>
				BytesMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> Bytes for BytesRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_as_slice (&self) -> (&[u8]) {
		match *self {
			BytesRef::Immutable (_, bytes) =>
				bytes,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesRef::Mutable (_, ref bytes) =>
				bytes,
		}
	}
}




pub enum BytesAsRef <'a> {
	Immutable (&'a BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a BytesMutable),
}


impl <'a> BytesAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<BytesAsRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::BytesImmutable (value) =>
				succeed! (BytesAsRef::Immutable (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::BytesMutable (value) =>
				succeed! (BytesAsRef::Mutable (value)),
			_ =>
				fail! (0xa0fe7201),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref (&self) -> (Outcome<BytesRef<'a>>) {
		match *self {
			BytesAsRef::Immutable (value) =>
				succeed! (value.bytes_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesAsRef::Mutable (value) =>
				return value.bytes_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			BytesAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesAsRef::Mutable (value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_rc_clone (&self) -> (Outcome<StdRc<StdBox<[u8]>>>) {
		match *self {
			BytesAsRef::Immutable (value) =>
				succeed! (value.bytes_rc_clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesAsRef::Mutable (value) =>
				succeed! (try_or_fail! ((value.0) .as_ref () .try_borrow_mut (), 0x42fd45a6) .to_cow ()),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<BytesImmutable>) {
		match *self {
			BytesAsRef::Immutable (value) =>
				succeed! ((*value) .clone ()),
			BytesAsRef::Mutable (value) =>
				return (*value) .to_immutable (),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (BytesMutable) {
		match *self {
			BytesAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			BytesAsRef::Mutable (value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &BytesAsRef) -> (bool) {
		#[ allow (unreachable_patterns) ]
		match (self, other) {
			(&BytesAsRef::Immutable (self_0), &BytesAsRef::Immutable (other_0)) =>
				BytesImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&BytesAsRef::Mutable (self_0), &BytesAsRef::Mutable (other_0)) =>
				BytesMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct BytesImmutable ( StdRc<StdBox<[u8]>> );


impl BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<[u8]>>) -> (BytesImmutable) {
		BytesImmutable (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<[u8]>>) -> (BytesImmutable) {
		BytesImmutable::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &BytesImmutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref (&self) -> (BytesRef) {
		BytesRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdBox<[u8]>>) {
		self.0.clone ()
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (BytesMutable) {
		BytesMutable::from_rc (self.bytes_rc_clone ())
	}
}


impl Bytes for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn bytes_as_slice (&self) -> (&[u8]) {
		self.0.as_ref ()
	}
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ derive (Clone, Debug) ]
pub struct BytesMutable ( StdRc<StdRefCell<BytesMutableInternals>> );


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ derive (Debug) ]
pub enum BytesMutableInternals {
	Owned (StdVec<u8>),
	Cow (StdRc<StdBox<[u8]>>),
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<[u8]>>) -> (BytesMutable) {
		let internals = BytesMutableInternals::Cow (rc);
		BytesMutable (StdRc::new (StdRefCell::new (internals)))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<[u8]>>) -> (BytesMutable) {
		BytesMutable::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &BytesMutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref (&self) -> (Outcome<BytesRef>) {
		let reference = try_or_fail! (self.0.as_ref () .try_borrow (), 0xd2c583c5);
		let reference = StdRef::map (reference, |reference| reference.as_ref ());
		succeed! (BytesRef::Mutable (self, reference));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdRefCell<BytesMutableInternals>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn bytes_ref_mut (&self) -> (Outcome<StdRefMut<StdVec<u8>>>) {
		let reference = try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0x0701b105);
		let reference = StdRefMut::map (reference, |reference| reference.as_mut ());
		succeed! (reference);
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<BytesImmutable>) {
		let mut reference = try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0x46cd7c85);
		let bytes = reference.to_cow ();
		succeed! (BytesImmutable::from_rc (bytes));
	}
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl BytesMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn to_cow (&mut self) -> (StdRc<StdBox<[u8]>>) {
		let bytes_cow = match *self {
			BytesMutableInternals::Owned (ref mut bytes_owned) => {
				let mut bytes_swap = StdVec::new ();
				mem::swap (&mut bytes_swap, bytes_owned);
				let bytes_swap = bytes_swap.into_boxed_slice ();
				bytes_swap
			},
			BytesMutableInternals::Cow (ref mut bytes) =>
				return bytes.clone (),
		};
		*self = BytesMutableInternals::Cow (StdRc::new (bytes_cow));
		return self.to_cow ();
	}
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl StdAsRef<[u8]> for BytesMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&[u8]) {
		match *self {
			BytesMutableInternals::Owned (ref bytes) =>
				bytes.deref (),
			BytesMutableInternals::Cow (ref bytes) =>
				bytes.deref (),
		}
	}
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl StdAsRefMut<StdVec<u8>> for BytesMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_mut (&mut self) -> (&mut StdVec<u8>) {
		let bytes_owned = match *self {
			BytesMutableInternals::Owned (ref mut bytes) =>
				return bytes,
			BytesMutableInternals::Cow (ref mut bytes_cow) => {
				let bytes_cow = StdRc::make_mut (bytes_cow);
				let mut bytes_swap = StdVec::new () .into_boxed_slice ();
				mem::swap (&mut bytes_swap, bytes_cow);
				let bytes_swap = StdVec::from (bytes_swap);
				bytes_swap
			},
		};
		*self = BytesMutableInternals::Owned (bytes_owned);
		return self.as_mut ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_new (bytes : StdVec<u8>) -> (BytesImmutable) {
	BytesImmutable (StdRc::new (bytes.into_boxed_slice ()))
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_new (bytes : StdVec<u8>) -> (BytesMutable) {
	let internals = BytesMutableInternals::Owned (bytes);
	BytesMutable (StdRc::new (StdRefCell::new (internals)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_new (bytes : StdVec<u8>) -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if BYTES_NEW_IMMUTABLE {
		bytes_immutable_new (bytes) .into ()
	} else {
		bytes_mutable_new (bytes) .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	bytes_immutable_new (bytes) .into ()
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_new_empty () -> (BytesImmutable) {
	bytes_immutable_new (StdVec::new ())
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_new_empty () -> (BytesMutable) {
	bytes_mutable_new (StdVec::new ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_new_empty () -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if BYTES_NEW_IMMUTABLE {
		bytes_immutable_new_empty () .into ()
	} else {
		bytes_mutable_new_empty () .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	bytes_immutable_new_empty () .into ()
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_clone_slice (bytes : &[u8]) -> (BytesImmutable) {
	bytes_immutable_new (vec_clone_slice (bytes))
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_clone_slice (bytes : &[u8]) -> (BytesMutable) {
	bytes_mutable_new (vec_clone_slice (bytes))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_clone_slice (bytes : &[u8]) -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if BYTES_NEW_IMMUTABLE {
		bytes_immutable_clone_slice (bytes) .into ()
	} else {
		bytes_mutable_clone_slice (bytes) .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	bytes_immutable_clone_slice (bytes) .into ()
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_clone_str (string : &str) -> (BytesImmutable) {
	bytes_immutable_new (StdString::from (string) .into_bytes ())
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_clone_str (string : &str) -> (BytesMutable) {
	bytes_mutable_new (StdString::from (string) .into_bytes ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_clone_str (string : &str) -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if BYTES_NEW_IMMUTABLE {
		bytes_immutable_clone_str (string) .into ()
	} else {
		bytes_mutable_clone_str (string) .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	bytes_immutable_clone_str (string) .into ()
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_immutable_clone_characters (characters : &[char]) -> (BytesImmutable) {
	bytes_immutable_new (unicode_utf8_chars_clone_string (characters) .into_bytes ())
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_mutable_clone_characters (characters : &[char]) -> (BytesMutable) {
	bytes_mutable_new (unicode_utf8_chars_clone_string (characters) .into_bytes ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_clone_characters (characters : &[char]) -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if BYTES_NEW_IMMUTABLE {
		bytes_immutable_clone_characters (characters) .into ()
	} else {
		bytes_mutable_clone_characters (characters) .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	bytes_immutable_clone_characters (characters) .into ()
}




pub struct BytesIterator <'a> ( BytesRef<'a>, slice::Iter<'a, u8> );


impl <'a> BytesIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (bytes : &'a Value) -> (Outcome<BytesIterator<'a>>) {
		let bytes = try_as_bytes_ref! (bytes);
		return BytesIterator::new_a (bytes);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_a (bytes : BytesRef<'a>) -> (Outcome<BytesIterator<'a>>) {
		let iterator = unsafe { mem::transmute (bytes.bytes_iter ()) };
		succeed! (BytesIterator (bytes, iterator));
	}
}


impl <'a> iter::Iterator for BytesIterator <'a> {
	
	type Item = Outcome<Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (bytes : &'a [&'a Value]) -> (Outcome<BytesIterators<'a>>) {
		let iterators = try! (bytes.iter () .map (|bytes| BytesIterator::new (bytes)) .collect ());
		succeed! (BytesIterators (iterators));
	}
}


impl <'a> iter::Iterator for BytesIterators <'a> {
	
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

