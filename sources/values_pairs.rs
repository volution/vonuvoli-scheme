

use super::errors::exports::*;
use super::runtime::exports::*;
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
	Immutable (&'a StdRc<(Value, Value)>, &'a (Value, Value)),
	ImmutableEmbedded (StdRc<(Value, Value)>, &'a (Value, Value)),
	Mutable (&'a StdRc<StdRefCell<(Value, Value)>>, StdRef<'a, (Value, Value)>),
	MutableEmbedded (StdRc<StdRefCell<(Value, Value)>>, StdRef<'a, (Value, Value)>),
}


impl <'a> PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_ref (value : &'a Value) -> (Outcome<PairRef<'a>>) {
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
	pub fn new_embedded_immutable (value : PairImmutable) -> (PairRef<'a>) {
		let value = value.values_rc_into ();
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		PairRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable (value : PairMutable) -> (PairRef<'a>) {
		let value = value.values_rc_into ();
		let value_ref = unsafe { mem::transmute (value.as_ref () .borrow ()) };
		PairRef::MutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_ref (&self) -> (PairRef<'a>) {
		match *self {
			PairRef::Immutable (value, reference) =>
				PairRef::Immutable (value, reference),
			PairRef::ImmutableEmbedded (ref value, reference) =>
				PairRef::ImmutableEmbedded (StdRc::clone (value), reference),
			PairRef::Mutable (value, ref reference) =>
				PairRef::Mutable (value, StdRef::clone (reference)),
			PairRef::MutableEmbedded (ref value, ref reference) =>
				PairRef::MutableEmbedded (StdRc::clone (value), StdRef::clone (reference)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_clone (&self) -> (Value) {
		match *self {
			PairRef::Immutable (value, _) =>
				PairImmutable (StdRc::clone (value)) .into (),
			PairRef::ImmutableEmbedded (ref value, _) =>
				PairImmutable (StdRc::clone (value)) .into (),
			PairRef::Mutable (value, _) =>
				PairMutable (StdRc::clone (value)) .into (),
			PairRef::MutableEmbedded (ref value, _) =>
				PairMutable (StdRc::clone (value)) .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_is_self (&self, other : &PairRef) -> (bool) {
		match (self, other) {
			
			(&PairRef::Immutable (self_0, _), &PairRef::Immutable (other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::ImmutableEmbedded (ref self_0, _), &PairRef::ImmutableEmbedded (ref other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::Immutable (self_0, _), &PairRef::ImmutableEmbedded (ref other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::ImmutableEmbedded (ref self_0, _), &PairRef::Immutable (other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			
			(&PairRef::Mutable (self_0, _), &PairRef::Mutable (other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::Mutable (self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::Mutable (other_0, _)) =>
				ptr::eq (self_0.as_ref (), other_0.as_ref ()),
			
			_ =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.0),
			PairRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, &value.0),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.0)),
			PairRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, |value| &value.0)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.1),
			PairRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, &value.1),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.1)),
			PairRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, |value| &value.1)),
		}
	}
}


impl <'a> Pair for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_tuple (&self) -> (&(Value, Value)) {
		match *self {
			PairRef::Immutable (_, values) =>
				values,
			PairRef::ImmutableEmbedded (_, values) =>
				values,
			PairRef::Mutable (_, ref values) =>
				values,
			PairRef::MutableEmbedded (_, ref values) =>
				values,
		}
	}
}




#[ derive (Clone, Debug) ]
pub enum PairAsRef <'a> {
	Immutable (&'a PairImmutable),
	ImmutableEmbedded (StdRc<StdAny>, &'a PairImmutable),
	Mutable (&'a PairMutable),
	MutableEmbedded (StdRc<StdAny>, &'a PairMutable),
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
	pub fn new_immutable_from_generic_ref (value : GenericRef<'a, PairImmutable>) -> (PairAsRef<'a>) {
		match value {
			GenericRef::Immutable (value) =>
				PairAsRef::Immutable (value),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				PairAsRef::ImmutableEmbedded (embedded, value),
			GenericRef::Mutable (value) =>
				PairAsRef::new_embedded_immutable (value.as_ref () .clone ()),
			GenericRef::MutableEmbedded (_, value) =>
				PairAsRef::new_embedded_immutable (value.as_ref () .clone ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable (value : PairImmutable) -> (PairAsRef<'a>) {
		let value = StdRc::new (value);
		PairAsRef::new_embedded_immutable_from_rc (value)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable (value : PairMutable) -> (PairAsRef<'a>) {
		let value = StdRc::new (value);
		PairAsRef::new_embedded_mutable_from_rc (value)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable_from_rc (value : StdRc<PairImmutable>) -> (PairAsRef<'a>) {
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable_from_rc (value : StdRc<PairMutable>) -> (PairAsRef<'a>) {
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::MutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (PairRef<'a>) {
		match *self {
			PairAsRef::Immutable (value) =>
				value.pair_ref (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				value.pair_ref (),
			PairAsRef::Mutable (value) =>
				value.pair_ref (),
			PairAsRef::MutableEmbedded (_, value) =>
				value.pair_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				(*value) .clone () .into (),
			PairAsRef::Mutable (value) =>
				(*value) .clone () .into (),
			PairAsRef::MutableEmbedded (_, value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (PairImmutable) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .clone (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				(*value) .clone (),
			PairAsRef::Mutable (value) =>
				(*value) .to_immutable (),
			PairAsRef::MutableEmbedded (_, value) =>
				(*value) .to_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				(*value) .to_mutable (),
			PairAsRef::Mutable (value) =>
				(*value) .clone (),
			PairAsRef::MutableEmbedded (_, value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairAsRef) -> (bool) {
		match (self, other) {
			
			(&PairAsRef::Immutable (self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::ImmutableEmbedded (_, self_0), &PairAsRef::ImmutableEmbedded (_, other_0)) =>
				PairImmutable::is_self (self_0.as_ref (), other_0.as_ref ()),
			(&PairAsRef::Immutable (self_0), &PairAsRef::ImmutableEmbedded (_, other_0)) =>
				PairImmutable::is_self (self_0, other_0.as_ref ()),
			(&PairAsRef::ImmutableEmbedded (_, self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0.as_ref (), other_0),
			
			(&PairAsRef::Mutable (self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0, other_0),
			(&PairAsRef::MutableEmbedded (_, self_0), &PairAsRef::MutableEmbedded (_, other_0)) =>
				PairMutable::is_self (self_0.as_ref (), other_0.as_ref ()),
			(&PairAsRef::Mutable (self_0), &PairAsRef::MutableEmbedded (_, other_0)) =>
				PairMutable::is_self (self_0, other_0.as_ref ()),
			(&PairAsRef::MutableEmbedded (_, self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0.as_ref (), other_0),
			
			_ =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairAsRef::Immutable (value) =>
				ValueRef::Immutable (value.left ()),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, value.left ()),
			PairAsRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value.values_rc_borrow (), |value| &value.0)),
			PairAsRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value.values_rc_borrow (), |value| &value.0)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairAsRef::Immutable (value) =>
				ValueRef::Immutable (value.right ()),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, value.right ()),
			PairAsRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value.values_rc_borrow (), |value| &value.1)),
			PairAsRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value.values_rc_borrow (), |value| &value.1)),
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
		PairRef::Immutable (&self.0, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<(Value, Value)>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_into (self) -> (StdRc<(Value, Value)>) {
		self.0
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
		PairRef::Mutable (&self.0, self.values_rc_borrow ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_borrow (&self) -> (StdRef<(Value, Value)>) {
		self.0.as_ref () .borrow ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<(Value, Value)>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_into (self) -> (StdRc<StdRefCell<(Value, Value)>>) {
		self.0
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
pub fn pair_new (left : Value, right : Value, immutable : Option<bool>) -> (Value) {
	if immutable.unwrap_or (PAIR_NEW_IMMUTABLE) {
		pair_immutable_new (left, right) .into ()
	} else {
		pair_mutable_new (left, right) .into ()
	}
}




pub struct ListPairIterator <'a> ( ValueRef<'a> );


impl <'a> ListPairIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value) -> (Outcome<ListPairIterator<'a>>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListPairIterator (value));
	}
}


impl <'a> iter::Iterator for ListPairIterator <'a> {
	
	type Item = Outcome<PairAsRef<'a>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<PairAsRef<'a>>>) {
		let (pair, cursor) = match self.0.kind () {
			ValueKind::PairImmutable => {
				let pair = self.0.clone_ref () .map_generic::<PairImmutable, _> (|value| value.expect_as_ref_0 ());
				let cursor = pair.clone_ref () .map_value (|pair| pair.right ());
				let pair = PairAsRef::new_immutable_from_generic_ref (pair);
				(pair, cursor)
			},
			ValueKind::Null =>
				return None,
			_ =>
				return Some (failed! (0x1f8fea4c)),
		};
		if self.0.value_is_self (&cursor) {
			return Some (failed! (0xa8ab23fb));
		}
		self.0 = cursor;
		return Some (succeeded! (pair));
	}
}




pub struct ListIterator <'a> ( ValueRef<'a> );


impl <'a> ListIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value) -> (Outcome<ListIterator<'a>>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListIterator (value));
	}
}


impl <'a> iter::Iterator for ListIterator <'a> {
	
	type Item = Outcome<ValueRef<'a>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<ValueRef<'a>>>) {
		let (value, cursor) = match self.0.kind () {
			ValueKind::PairImmutable => {
				let pair = self.0.clone_ref () .map_generic::<PairImmutable, _> (|value| value.expect_as_ref_0 ());
				let cursor = pair.clone_ref () .map_value (|pair| pair.right ());
				let value = pair.map_value (|pair| pair.left ());
				(value, cursor)
			},
			ValueKind::Null =>
				return None,
			_ =>
				return Some (failed! (0xed511f9c)),
		};
		if self.0.value_is_self (&cursor) {
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
	
	type Item = Outcome<StdVec<ValueRef<'a>>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<StdVec<ValueRef<'a>>>>) {
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

