

use super::errors::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Pair, PairRef, PairAsRef, PairImmutable, PairMutable};
	pub use super::{pair_new, pair_immutable_new, pair_mutable_new};
	pub use super::{ListPairIterator, ListIterator, ListIterators};
}




pub trait Pair {
	
	fn values_as_tuple (&self) -> (&(Value, Value));
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left (&self) -> (&Value) {
		let values = self.values_as_tuple ();
		&values.0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn right (&self) -> (&Value) {
		let values = self.values_as_tuple ();
		&values.1
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		let values = self.values_as_tuple ();
		(&values.0, &values.1)
	}
}




#[ derive (Debug) ]
pub enum PairRef <'a> {
	Immutable (&'a PairImmutable, &'a (Value, Value)),
	Mutable (&'a PairMutable, StdRef<'a, (Value, Value)>),
}


impl <'a> PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<PairRef<'a>>) {
		match *value {
			Value::PairImmutable (_, ref value, _) =>
				succeed! (value.pair_ref ()),
			Value::PairMutable (_, ref value, _) =>
				succeed! (value.pair_ref ()),
			_ =>
				fail! (0x0bb90a73),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			PairRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			PairRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairRef) -> (bool) {
		match (self, other) {
			(&PairRef::Immutable (self_0, _), &PairRef::Immutable (other_0, _)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairRef::Mutable (self_0, _), &PairRef::Mutable (other_0, _)) =>
				PairMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.0),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.0)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.1),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.1)),
		}
	}
}


impl <'a> Pair for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_tuple (&self) -> (&(Value, Value)) {
		match *self {
			PairRef::Immutable (_, values) =>
				values,
			PairRef::Mutable (_, ref values) =>
				values,
		}
	}
}




#[ derive (Debug) ]
pub enum PairAsRef <'a> {
	Immutable (&'a PairImmutable),
	Mutable (&'a PairMutable),
}


impl <'a> PairAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<PairAsRef<'a>>) {
		match *value {
			Value::PairImmutable (_, ref value, _) =>
				succeed! (PairAsRef::Immutable (value)),
			Value::PairMutable (_, ref value, _) =>
				succeed! (PairAsRef::Mutable (value)),
			_ =>
				fail! (0x1cb1913b),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (PairRef<'a>) {
		match *self {
			PairAsRef::Immutable (value) =>
				value.pair_ref (),
			PairAsRef::Mutable (value) =>
				value.pair_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			PairAsRef::Mutable (value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (PairImmutable) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .clone (),
			PairAsRef::Mutable (value) =>
				(*value) .to_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			PairAsRef::Mutable (value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairAsRef) -> (bool) {
		match (self, other) {
			(&PairAsRef::Immutable (self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::Mutable (self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct PairImmutable ( StdRc<(Value, Value)> );


impl PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<(Value, Value)>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		PairMutable (StdRc::new (StdRefCell::new ((*self.0) .clone ())))
	}
}


impl Pair for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_tuple (&self) -> (&(Value, Value)) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct PairMutable ( StdRc<StdRefCell<(Value, Value)>> );


impl PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<(Value, Value)>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_ref_mut (&self) -> (StdRefMut<(Value, Value)>) {
		self.0.as_ref () .borrow_mut ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (PairImmutable) {
		let reference = self.0.as_ref () .borrow ();
		PairImmutable (StdRc::new ((*reference) .clone ()))
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_new (left : Value, right : Value) -> (PairImmutable) {
	PairImmutable (StdRc::new ((left, right)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_new (left : Value, right : Value) -> (PairMutable) {
	PairMutable (StdRc::new (StdRefCell::new ((left, right))))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_new (left : Value, right : Value) -> (Value) {
	if true {
		pair_immutable_new (left, right) .into ()
	} else {
		pair_mutable_new (left, right) .into ()
	}
}




pub struct ListPairIterator <'a> ( &'a Value );


impl <'a> ListPairIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value) -> (Outcome<ListPairIterator<'a>>) {
		succeed! (ListPairIterator (value));
	}
}


impl <'a> iter::Iterator for ListPairIterator <'a> {
	
	type Item = Outcome<&'a PairImmutable>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<&'a PairImmutable>>) {
		
		let cursor = self.0;
		let (pair, cursor) = match cursor.class () {
			ValueClass::Pair =>
				// FIXME:  Add support for mutable pairs!
				if let Ok (pair) = StdTryAsRef0::<PairImmutable>::try_as_ref_0 (cursor) {
					pair.left_and_right ();
					let cursor = pair.right ();
					(pair, cursor)
				} else {
					return Some (failed! (0x14fb94f0));
				},
			ValueClass::Null =>
				return None,
			_ =>
				return Some (failed! (0x1f8fea4c)),
		};
		if self.0.is_self (cursor) {
			return Some (failed! (0xa8ab23fb));
		}
		self.0 = cursor;
		return Some (succeeded! (pair));
	}
}




pub struct ListIterator <'a> ( &'a Value );


impl <'a> ListIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value) -> (Outcome<ListIterator<'a>>) {
		succeed! (ListIterator (value));
	}
}


impl <'a> iter::Iterator for ListIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		let cursor = self.0;
		let (value, cursor) = match cursor.class () {
			ValueClass::Pair =>
				match *cursor {
					Value::PairImmutable (_, ref pair, _) =>
						pair.left_and_right (),
					// FIXME:  Add support for mutable pairs!
					//Value::PairMutable (_, ref pair, _) =>
					//	pair.pair_ref () .left_and_right (),
					_ =>
						return Some (failed! (0xff1a6f2d)),
				},
			ValueClass::Null =>
				return None,
			_ =>
				return Some (failed! (0xed511f9c)),
		};
		if self.0.is_self (cursor) {
			return Some (failed! (0x2f6495d9));
		}
		self.0 = cursor;
		return Some (succeeded! (value));
	}
}




pub struct ListIterators <'a> ( StdVec<ListIterator<'a>> );


impl <'a> ListIterators <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lists : &'a [&Value]) -> (Outcome<ListIterators<'a>>) {
		let iterators = try! (lists.iter () .map (|list| ListIterator::new (list)) .collect ());
		succeed! (ListIterators (iterators));
	}
}


impl <'a> iter::Iterator for ListIterators <'a> {
	
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

