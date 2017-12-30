

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Array, ArrayRef, ArrayImmutable, ArrayMutable};
	pub use super::{array_immutable_new, array_immutable_clone_slice, array_immutable_clone_slice_ref};
	pub use super::{array_mutable_new, array_mutable_clone_slice, array_mutable_clone_slice_ref};
	pub use super::{array_new, array_clone_slice, array_clone_slice_ref};
	pub use super::{ArrayIterator, ArrayIterators};
}




pub trait Array {
	
	fn values_as_vec (&self) -> (&StdVec<Value>);
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		self.values_as_vec () .as_slice ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_iter (&self) -> (slice::Iter<Value>) {
		self.values_as_vec () .iter ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		self.values_as_vec () .clone ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_is_empty (&self) -> (bool) {
		self.values_as_vec () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_is_not_empty (&self) -> (bool) {
		! self.values_as_vec () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_length (&self) -> (usize) {
		self.values_as_vec () .len ()
	}
}




#[ derive (Debug) ]
pub enum ArrayRef <'a> {
	Immutable (&'a ArrayImmutable, &'a StdVec<Value>),
	Mutable (&'a ArrayMutable, StdRef<'a, StdVec<Value>>),
}


impl <'a> ArrayRef<'a> {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try (value : &'a Value) -> (Outcome<ArrayRef<'a>>) {
		match *value {
			Value::ArrayImmutable (_, ref value, _) =>
				succeed! (value.values_ref ()),
			Value::ArrayMutable (_, ref value, _) =>
				succeed! (value.values_ref ()),
			_ =>
				fail! (0x4e577110),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			ArrayRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			ArrayRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
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
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_as_vec (&self) -> (&StdVec<Value>) {
		match *self {
			ArrayRef::Immutable (_, values) =>
				values,
			ArrayRef::Mutable (_, ref values) =>
				values,
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct ArrayImmutable ( StdRc<StdVec<Value>> );


impl ArrayImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &ArrayImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn values_ref (&self) -> (ArrayRef) {
		ArrayRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdVec<Value>>) {
		self.0.clone ()
	}
}


impl Array for ArrayImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn values_as_vec (&self) -> (&StdVec<Value>) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct ArrayMutable ( StdRc<StdRefCell<StdVec<Value>>> );


impl ArrayMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &ArrayMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn values_ref (&self) -> (ArrayRef) {
		ArrayRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<StdVec<Value>>>) {
		self.0.clone ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_immutable_new (values : StdVec<Value>) -> (ArrayImmutable) {
	ArrayImmutable (StdRc::new (values))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_mutable_new (values : StdVec<Value>) -> (ArrayMutable) {
	ArrayMutable (StdRc::new (StdRefCell::new (values)))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_new (values : StdVec<Value>) -> (Value) {
	if true {
		array_immutable_new (values) .into ()
	} else {
		array_mutable_new (values) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_immutable_clone_slice (values : &[Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice (values))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_mutable_clone_slice (values : &[Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice (values))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_clone_slice (values : &[Value]) -> (Value) {
	if true {
		array_immutable_clone_slice (values) .into ()
	} else {
		array_mutable_clone_slice (values) .into ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_immutable_clone_slice_ref (values : &[&Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_mutable_clone_slice_ref (values : &[&Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_clone_slice_ref (values : &[&Value]) -> (Value) {
	if true {
		array_immutable_clone_slice_ref (values) .into ()
	} else {
		array_mutable_clone_slice_ref (values) .into ()
	}
}




pub struct ArrayIterator <'a> ( ArrayRef<'a>, slice::Iter<'a, Value> );


impl <'a> ArrayIterator <'a> {
	
	pub fn new (array : &'a Value) -> (Outcome<ArrayIterator<'a>>) {
		let array = try_as_array_ref! (array);
		return ArrayIterator::new_a (array);
	}
	
	pub fn new_a (array : ArrayRef<'a>) -> (Outcome<ArrayIterator<'a>>) {
		let iterator = unsafe { mem::transmute (array.values_iter ()) };
		succeed! (ArrayIterator (array, iterator));
	}
}


impl <'a> iter::Iterator for ArrayIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
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
	
	pub fn new (arrays : &'a [&'a Value]) -> (Outcome<ArrayIterators<'a>>) {
		let iterators = try! (arrays.iter () .map (|array| ArrayIterator::new (array)) .collect ());
		succeed! (ArrayIterators (iterators));
	}
}


impl <'a> iter::Iterator for ArrayIterators <'a> {
	
	type Item = Outcome<StdVec<&'a Value>>;
	
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

