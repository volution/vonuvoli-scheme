

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Array, ArrayRef, ArrayAsRef, ArrayImmutable, ArrayMutable, ArrayMutableInternals};
	pub use super::{ArrayMatchAsRef, ArrayMatchInto, ArrayMatchAsRef2};
	pub use super::{array_immutable_new, array_immutable_clone_slice, array_immutable_clone_slice_ref};
	pub use super::{array_mutable_new, array_mutable_clone_slice, array_mutable_clone_slice_ref};
	pub use super::{array_new, array_clone_slice, array_clone_slice_ref};
	pub use super::{ArrayIterator, ArrayIterators};
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayMatchAsRef <'a> {
	Immutable (&'a ArrayImmutable),
	Mutable (&'a ArrayMutable),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayMatchInto {
	Immutable (ArrayImmutable),
	Mutable (ArrayMutable),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayMatchAsRef2 <'a> {
	ImmutableBoth (&'a ArrayImmutable, &'a ArrayImmutable),
	MutableBoth (&'a ArrayMutable, &'a ArrayMutable),
	ImmutableAndMutable (&'a ArrayImmutable, &'a ArrayMutable),
	MutableAndImmutable (&'a ArrayMutable, &'a ArrayImmutable),
}


impl <'a> ArrayMatchAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn array_ref (&self) -> (ArrayRef<'a>) {
		match *self {
			ArrayMatchAsRef::Immutable (value) =>
				value.array_ref (),
			ArrayMatchAsRef::Mutable (value) =>
				value.array_ref (),
		}
	}
}


impl <'a> ArrayMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn array_ref (&self) -> ((ArrayRef<'a>, ArrayRef<'a>)) {
		match *self {
			ArrayMatchAsRef2::ImmutableBoth (left, right) =>
				(left.array_ref (), right.array_ref ()),
			ArrayMatchAsRef2::MutableBoth (left, right) =>
				(left.array_ref (), right.array_ref ()),
			ArrayMatchAsRef2::ImmutableAndMutable (left, right) =>
				(left.array_ref (), right.array_ref ()),
			ArrayMatchAsRef2::MutableAndImmutable (left, right) =>
				(left.array_ref (), right.array_ref ()),
		}
	}
}


impl ArrayMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ArrayMatchInto::Immutable (value) =>
				value.into (),
			ArrayMatchInto::Mutable (value) =>
				value.into (),
		}
	}
}




pub trait Array {
	
	fn values_as_slice (&self) -> (&[Value]);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_iter (&self) -> (slice::Iter<Value>) {
		self.values_as_slice () .iter ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		self.values_as_slice () .to_vec ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_is_empty (&self) -> (bool) {
		self.values_as_slice () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_is_not_empty (&self) -> (bool) {
		! self.values_as_slice () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_length (&self) -> (usize) {
		self.values_as_slice () .len ()
	}
}




#[ derive (Debug) ]
pub enum ArrayRef <'a> {
	Immutable (&'a ArrayImmutable, &'a [Value]),
	Mutable (&'a ArrayMutable, StdRef<'a, [Value]>),
}


impl <'a> ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<ArrayRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::ArrayImmutable (value) =>
				succeed! (value.array_ref ()),
			ValueKindMatchAsRef::ArrayMutable (value) =>
				succeed! (value.array_ref ()),
			_ =>
				fail! (0x4e577110),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			ArrayRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			ArrayRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ArrayRef) -> (bool) {
		match (self, other) {
			(&ArrayRef::Immutable (self_0, _), &ArrayRef::Immutable (other_0, _)) =>
				ArrayImmutable::is_self (self_0, other_0),
			(&ArrayRef::Mutable (self_0, _), &ArrayRef::Mutable (other_0, _)) =>
				ArrayMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> Array for ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		match *self {
			ArrayRef::Immutable (_, values) =>
				values,
			ArrayRef::Mutable (_, ref values) =>
				values,
		}
	}
}




#[ derive (Debug) ]
pub enum ArrayAsRef <'a> {
	Immutable (&'a ArrayImmutable),
	Mutable (&'a ArrayMutable),
}


impl <'a> ArrayAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<ArrayAsRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::ArrayImmutable (value) =>
				succeed! (ArrayAsRef::Immutable (value)),
			ValueKindMatchAsRef::ArrayMutable (value) =>
				succeed! (ArrayAsRef::Mutable (value)),
			_ =>
				fail! (0xde9b3abe),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn array_ref (&self) -> (ArrayRef<'a>) {
		match *self {
			ArrayAsRef::Immutable (value) =>
				value.array_ref (),
			ArrayAsRef::Mutable (value) =>
				value.array_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			ArrayAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			ArrayAsRef::Mutable (value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (ArrayImmutable) {
		match *self {
			ArrayAsRef::Immutable (value) =>
				(*value) .clone (),
			ArrayAsRef::Mutable (value) =>
				(*value) .to_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (ArrayMutable) {
		match *self {
			ArrayAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			ArrayAsRef::Mutable (value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ArrayAsRef) -> (bool) {
		match (self, other) {
			(&ArrayAsRef::Immutable (self_0), &ArrayAsRef::Immutable (other_0)) =>
				ArrayImmutable::is_self (self_0, other_0),
			(&ArrayAsRef::Mutable (self_0), &ArrayAsRef::Mutable (other_0)) =>
				ArrayMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct ArrayImmutable ( StdRc<StdBox<[Value]>> );


impl ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ArrayImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn array_ref (&self) -> (ArrayRef) {
		ArrayRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdBox<[Value]>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (ArrayMutable) {
		let internals = ArrayMutableInternals::Cow (self.values_rc_clone ());
		ArrayMutable (StdRc::new (StdRefCell::new (internals)))
	}
}


impl Array for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct ArrayMutable ( StdRc<StdRefCell<ArrayMutableInternals>> );


#[ derive (Debug) ]
pub enum ArrayMutableInternals {
	Owned (StdVec<Value>),
	Cow (StdRc<StdBox<[Value]>>),
}


impl ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ArrayMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn array_ref (&self) -> (ArrayRef) {
		let reference = self.0.as_ref () .borrow ();
		let reference = StdRef::map (reference, |reference| reference.as_ref ());
		ArrayRef::Mutable (self, reference)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<ArrayMutableInternals>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_ref_mut (&self) -> (StdRefMut<StdVec<Value>>) {
		let reference = self.0.as_ref () .borrow_mut ();
		let reference = StdRefMut::map (reference, |reference| reference.as_mut ());
		reference
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (ArrayImmutable) {
		let mut reference = self.0.as_ref () .borrow_mut ();
		let values = reference.to_cow ();
		ArrayImmutable (values)
	}
}


impl ArrayMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn to_cow (&mut self) -> (StdRc<StdBox<[Value]>>) {
		let values_cow = match *self {
			ArrayMutableInternals::Owned (ref mut values_owned) => {
				let mut values_swap = StdVec::new ();
				mem::swap (&mut values_swap, values_owned);
				let values_swap = values_swap.into_boxed_slice ();
				values_swap
			},
			ArrayMutableInternals::Cow (ref mut values) =>
				return values.clone (),
		};
		*self = ArrayMutableInternals::Cow (StdRc::new (values_cow));
		return self.to_cow ();
	}
}


impl StdAsRef<[Value]> for ArrayMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&[Value]) {
		match *self {
			ArrayMutableInternals::Owned (ref values) =>
				values.deref (),
			ArrayMutableInternals::Cow (ref values) =>
				values.deref (),
		}
	}
}


impl StdAsRefMut<StdVec<Value>> for ArrayMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_mut (&mut self) -> (&mut StdVec<Value>) {
		let values_owned = match *self {
			ArrayMutableInternals::Owned (ref mut values) =>
				return values,
			ArrayMutableInternals::Cow (ref mut values_cow) => {
				let values_cow = StdRc::make_mut (values_cow);
				let mut values_swap = StdVec::new () .into_boxed_slice ();
				mem::swap (&mut values_swap, values_cow);
				let values_swap = StdVec::from (values_swap);
				values_swap
			},
		};
		*self = ArrayMutableInternals::Owned (values_owned);
		return self.as_mut ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_new (values : StdVec<Value>) -> (ArrayImmutable) {
	ArrayImmutable (StdRc::new (values.into_boxed_slice ()))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_new (values : StdVec<Value>) -> (ArrayMutable) {
	let internals = ArrayMutableInternals::Owned (values);
	ArrayMutable (StdRc::new (StdRefCell::new (internals)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_new (values : StdVec<Value>) -> (Value) {
	if ARRAY_NEW_IMMUTABLE {
		array_immutable_new (values) .into ()
	} else {
		array_mutable_new (values) .into ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_clone_slice (values : &[Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_clone_slice (values : &[Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clone_slice (values : &[Value]) -> (Value) {
	if ARRAY_NEW_IMMUTABLE {
		array_immutable_clone_slice (values) .into ()
	} else {
		array_mutable_clone_slice (values) .into ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_immutable_clone_slice_ref (values : &[&Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_mutable_clone_slice_ref (values : &[&Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clone_slice_ref (values : &[&Value]) -> (Value) {
	if ARRAY_NEW_IMMUTABLE {
		array_immutable_clone_slice_ref (values) .into ()
	} else {
		array_mutable_clone_slice_ref (values) .into ()
	}
}




pub struct ArrayIterator <'a> ( ArrayRef<'a>, slice::Iter<'a, Value> );


impl <'a> ArrayIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (array : &'a Value) -> (Outcome<ArrayIterator<'a>>) {
		let array = try_as_array_ref! (array);
		return ArrayIterator::new_a (array);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_a (array : ArrayRef<'a>) -> (Outcome<ArrayIterator<'a>>) {
		let iterator = unsafe { mem::transmute (array.values_iter ()) };
		succeed! (ArrayIterator (array, iterator));
	}
}


impl <'a> iter::Iterator for ArrayIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		if let Some (value) = self.1.next () {
			return Some (succeeded! (value));
		} else {
			return None;
		}
	}
}




pub struct ArrayIterators <'a> ( StdVec<ArrayIterator<'a>> );


impl <'a> ArrayIterators <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (arrays : &'a [&'a Value]) -> (Outcome<ArrayIterators<'a>>) {
		let iterators = try! (arrays.iter () .map (|array| ArrayIterator::new (array)) .collect ());
		succeed! (ArrayIterators (iterators));
	}
}


impl <'a> iter::Iterator for ArrayIterators <'a> {
	
	type Item = Outcome<StdVec<&'a Value>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<StdVec<&'a Value>>>) {
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

