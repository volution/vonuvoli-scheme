

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_value::exports::*;

#[ cfg ( feature = "vonuvoli_values_array" ) ]
use super::values_arrays::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Pair, PairRef, PairAsRef, PairImmutable, PairImmutableInternals, PairMutable, PairMutableInternals};
	pub use super::{PairMatchAsRef, PairMatchInto, PairMatchAsRef2};
	pub use super::{pair_new, pair_immutable_new, pair_mutable_new};
	pub use super::{ListPairIterator, ListIterator, ListIterators};
}




pub enum PairMatchAsRef <'a> {
	Immutable (&'a PairImmutable),
	Mutable (&'a PairMutable),
}


pub enum PairMatchInto {
	Immutable (PairImmutable),
	Mutable (PairMutable),
}


pub enum PairMatchAsRef2 <'a> {
	ImmutableBoth (&'a PairImmutable, &'a PairImmutable),
	MutableBoth (&'a PairMutable, &'a PairMutable),
	ImmutableAndMutable (&'a PairImmutable, &'a PairMutable),
	MutableAndImmutable (&'a PairMutable, &'a PairImmutable),
}


impl <'a> PairMatchAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (Outcome<PairRef<'a>>) {
		match *self {
			PairMatchAsRef::Immutable (value) =>
				succeed! (value.pair_ref ()),
			PairMatchAsRef::Mutable (value) =>
				return value.pair_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_as_ref (self) -> (PairAsRef<'a>) {
		match self {
			PairMatchAsRef::Immutable (value) =>
				PairAsRef::Immutable (value),
			PairMatchAsRef::Mutable (value) =>
				PairAsRef::Mutable (value),
		}
	}
}


impl <'a> PairMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (Outcome<(PairRef<'a>, PairRef<'a>)>) {
		match *self {
			PairMatchAsRef2::ImmutableBoth (left, right) =>
				succeed! ((left.pair_ref (), right.pair_ref ())),
			PairMatchAsRef2::MutableBoth (left, right) =>
				succeed! ((try! (left.pair_ref ()), try! (right.pair_ref ()))),
			PairMatchAsRef2::ImmutableAndMutable (left, right) =>
				succeed! ((left.pair_ref (), try! (right.pair_ref ()))),
			PairMatchAsRef2::MutableAndImmutable (left, right) =>
				succeed! ((try! (left.pair_ref ()), right.pair_ref ())),
		}
	}
}


impl PairMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			PairMatchInto::Immutable (value) =>
				value.into (),
			PairMatchInto::Mutable (value) =>
				value.into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_immutable (self) -> (Outcome<PairImmutable>) {
		match self {
			PairMatchInto::Immutable (value) =>
				succeed! (value),
			PairMatchInto::Mutable (value) =>
				return value.into_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_mutable (self) -> (PairMutable) {
		match self {
			PairMatchInto::Immutable (value) =>
				value.into_mutable (),
			PairMatchInto::Mutable (value) =>
				value,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_and_right_into (self) -> (Outcome<(Value, Value)>) {
		match self {
			PairMatchInto::Immutable (value) =>
				succeed! (value.left_and_right_into ()),
			PairMatchInto::Mutable (value) =>
				return value.left_and_right_into (),
		}
	}
}




pub trait Pair {
	
	fn left (&self) -> (&Value);
	fn right (&self) -> (&Value);
	fn left_and_right (&self) -> ((&Value, &Value));
	fn left_and_right_as_slice (&self) -> (&[Value]);
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl <PairObject : Pair> Array for PairObject {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		self.left_and_right_as_slice ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_iter (&self) -> (slice::Iter<Value>) {
		self.left_and_right_as_slice () .iter ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		let (left, right) = self.left_and_right ();
		vec! [left.clone (), right.clone ()]
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_is_empty (&self) -> (bool) {
		false
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_is_not_empty (&self) -> (bool) {
		true
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_length (&self) -> (usize) {
		2
	}
}




pub enum PairRef <'a> {
	Immutable (&'a StdRc<PairImmutableInternals>, &'a PairImmutableInternals),
	ImmutableEmbedded (StdRc<PairImmutableInternals>, &'a PairImmutableInternals),
	Mutable (&'a StdRc<StdRefCell<PairMutableInternals>>, StdRef<'a, PairMutableInternals>),
	MutableEmbedded (StdRc<StdRefCell<PairMutableInternals>>, StdRef<'a, PairMutableInternals>),
}


impl <'a> PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_ref (value : &'a Value) -> (Outcome<PairRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::PairImmutable (value) =>
				succeed! (value.pair_ref ()),
			ValueKindMatchAsRef::PairMutable (value) =>
				return value.pair_ref (),
			_ =>
				fail! (0x0bb90a73),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable (value : PairImmutable) -> (PairRef<'a>) {
		let value = value.internals_rc_into ();
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairRef::ImmutableEmbedded (value, internals)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable (value : PairMutable) -> (Outcome<PairRef<'a>>) {
		let value = value.internals_rc_into ();
		let internals = unsafe { mem::transmute (try_or_fail! (value.as_ref () .try_borrow (), 0xc98353c8)) };
		succeed! (PairRef::MutableEmbedded (value, internals))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_ref (&self) -> (PairRef<'a>) {
		match *self {
			PairRef::Immutable (value, internals) =>
				PairRef::Immutable (value, internals),
			PairRef::ImmutableEmbedded (ref value, internals) =>
				PairRef::ImmutableEmbedded (StdRc::clone (value), internals),
			PairRef::Mutable (value, ref internals) =>
				PairRef::Mutable (value, StdRef::clone (internals)),
			PairRef::MutableEmbedded (ref value, ref internals) =>
				PairRef::MutableEmbedded (StdRc::clone (value), StdRef::clone (internals)),
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
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::ImmutableEmbedded (ref self_0, _), &PairRef::ImmutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::Immutable (self_0, _), &PairRef::ImmutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::ImmutableEmbedded (ref self_0, _), &PairRef::Immutable (other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			
			(&PairRef::Mutable (self_0, _), &PairRef::Mutable (other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::Mutable (self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::Mutable (other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			
			_ =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, internals) =>
				ValueRef::Immutable (internals.left ()),
			PairRef::ImmutableEmbedded (value, internals) =>
				ValueRef::ImmutableEmbedded (value, internals.left ()),
			PairRef::Mutable (_, internals) =>
				ValueRef::Mutable (StdRef::map (internals, |internals| internals.left ())),
			PairRef::MutableEmbedded (value, internals) =>
				ValueRef::MutableEmbedded (value, StdRef::map (internals, |internals| internals.left ())),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, internals) =>
				ValueRef::Immutable (internals.right ()),
			PairRef::ImmutableEmbedded (value, internals) =>
				ValueRef::ImmutableEmbedded (value, internals.right ()),
			PairRef::Mutable (_, internals) =>
				ValueRef::Mutable (StdRef::map (internals, |internals| internals.right ())),
			PairRef::MutableEmbedded (value, internals) =>
				ValueRef::MutableEmbedded (value, StdRef::map (internals, |internals| internals.right ())),
		}
	}
}


impl <'a> Pair for PairRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left (&self) -> (&Value) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left (),
			PairRef::Mutable (_, ref internals) =>
				internals.left (),
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn right (&self) -> (&Value) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.right (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.right (),
			PairRef::Mutable (_, ref internals) =>
				internals.right (),
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.right (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left_and_right (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left_and_right (),
			PairRef::Mutable (_, ref internals) =>
				internals.left_and_right (),
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left_and_right (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left_and_right_as_slice (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left_and_right_as_slice (),
			PairRef::Mutable (_, ref internals) =>
				internals.left_and_right_as_slice (),
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left_and_right_as_slice (),
		}
	}
}




pub enum PairAsRef <'a> {
	// TODO:  Use internals!
	Immutable (&'a PairImmutable),
	ImmutableEmbedded (StdRc<StdAny>, &'a PairImmutable),
	Mutable (&'a PairMutable),
	MutableEmbedded (StdRc<StdAny>, &'a PairMutable),
}


impl <'a> PairAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<PairAsRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::PairImmutable (value) =>
				succeed! (PairAsRef::Immutable (value)),
			ValueKindMatchAsRef::PairMutable (value) =>
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
	pub fn new_mutable_from_generic_ref (value : GenericRef<'a, PairMutable>) -> (PairAsRef<'a>) {
		match value {
			GenericRef::Immutable (value) =>
				PairAsRef::Mutable (value),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				PairAsRef::MutableEmbedded (embedded, value),
			GenericRef::Mutable (value) =>
				PairAsRef::new_embedded_mutable (value.as_ref () .clone ()),
			GenericRef::MutableEmbedded (_, value) =>
				PairAsRef::new_embedded_mutable (value.as_ref () .clone ()),
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
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::ImmutableEmbedded (value, internals)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable_from_rc (value : StdRc<PairMutable>) -> (PairAsRef<'a>) {
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::MutableEmbedded (value, internals)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (Outcome<PairRef<'a>>) {
		match *self {
			PairAsRef::Immutable (value) =>
				succeed! (value.pair_ref ()),
			PairAsRef::ImmutableEmbedded (_, value) =>
				succeed! (value.pair_ref ()),
			PairAsRef::Mutable (value) =>
				return value.pair_ref (),
			PairAsRef::MutableEmbedded (_, value) =>
				return value.pair_ref (),
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
	pub fn to_immutable (&self) -> (Outcome<PairImmutable>) {
		match *self {
			PairAsRef::Immutable (value) =>
				succeed! ((*value) .clone ()),
			PairAsRef::ImmutableEmbedded (_, value) =>
				succeed! ((*value) .clone ()),
			PairAsRef::Mutable (value) =>
				return (*value) .to_immutable (),
			PairAsRef::MutableEmbedded (_, value) =>
				return (*value) .to_immutable (),
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
	pub fn left_ref_into (self) -> (Outcome<ValueRef<'a>>) {
		match self {
			PairAsRef::Immutable (value) =>
				succeed! (ValueRef::Immutable (value.left ())),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				succeed! (ValueRef::ImmutableEmbedded (embedded, value.left ())),
			PairAsRef::Mutable (value) =>
				succeed! (ValueRef::Mutable (StdRef::map (try! (value.internals_rc_borrow ()), |value| value.left ()))),
			PairAsRef::MutableEmbedded (embedded, value) =>
				succeed! (ValueRef::MutableEmbedded (embedded, StdRef::map (try! (value.internals_rc_borrow ()), |value| value.left ()))),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn right_ref_into (self) -> (Outcome<ValueRef<'a>>) {
		match self {
			PairAsRef::Immutable (value) =>
				succeed! (ValueRef::Immutable (value.right ())),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				succeed! (ValueRef::ImmutableEmbedded (embedded, value.right ())),
			PairAsRef::Mutable (value) =>
				succeed! (ValueRef::Mutable (StdRef::map (try! (value.internals_rc_borrow ()), |value| value.right ()))),
			PairAsRef::MutableEmbedded (embedded, value) =>
				succeed! (ValueRef::MutableEmbedded (embedded, StdRef::map (try! (value.internals_rc_borrow ()), |value| value.right ()))),
		}
	}
}




#[ derive (Clone, Debug) ]
pub struct PairImmutable ( StdRc<PairImmutableInternals> );

#[ derive (Debug) ]
pub struct PairImmutableInternals {
	pub left : Value,
	pub right : Value,
}


impl PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairImmutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Immutable (&self.0, self.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<PairImmutableInternals>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<PairImmutableInternals>) {
		self.0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_and_right_into (self) -> ((Value, Value)) {
		match StdRc::try_unwrap (self.0) {
			Ok (internals) => {
				let PairImmutableInternals { left, right } = internals;
				(left, right)
			},
			Err (internals) => {
				let internals = internals.as_ref ();
				let left = internals.left.clone ();
				let right = internals.right.clone ();
				(left, right)
			},
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		let (left, right) = self.left_and_right ();
		let left = left.clone ();
		let right = right.clone ();
		let internals = PairMutableInternals { left, right };
		PairMutable (StdRc::new (StdRefCell::new (internals)))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_mutable (self) -> (PairMutable) {
		let (left, right) = self.left_and_right_into ();
		pair_mutable_new (left, right)
	}
}


impl Pair for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left (&self) -> (&Value) {
		self.0.as_ref () .left ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn right (&self) -> (&Value) {
		self.0.as_ref () .right ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		self.0.as_ref () .left_and_right ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		self.0.as_ref () .left_and_right_as_slice ()
	}
}


impl Pair for PairImmutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left (&self) -> (&Value) {
		&self.left
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn right (&self) -> (&Value) {
		&self.right
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		(&self.left, &self.right)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		let tuple : &[Value; 2] = unsafe { mem::transmute (self) };
		tuple
	}
}




#[ derive (Clone, Debug) ]
pub struct PairMutable ( StdRc<StdRefCell<PairMutableInternals>> );

#[ derive (Debug) ]
pub struct PairMutableInternals {
	pub left : Value,
	pub right : Value,
}


impl PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &PairMutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn pair_ref (&self) -> (Outcome<PairRef>) {
		succeed! (PairRef::Mutable (&self.0, try! (self.internals_rc_borrow ())));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_borrow (&self) -> (Outcome<StdRef<PairMutableInternals>>) {
		succeed! (try_or_fail! (self.0.as_ref () .try_borrow (), 0xf155eead));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<StdRefCell<PairMutableInternals>>) {
		self.0.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<StdRefCell<PairMutableInternals>>) {
		self.0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<PairMutableInternals>>) {
		succeed! (try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0x1bb74db3));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn left_and_right_into (self) -> (Outcome<(Value, Value)>) {
		match StdRc::try_unwrap (self.0) {
			Ok (internals) => {
				let internals = internals.into_inner ();
				let PairMutableInternals { left, right } = internals;
				succeed! ((left, right))
			},
			Err (internals) => {
				let internals = try_or_fail! (internals.as_ref () .try_borrow (), 0xe24ce55a);
				let left = internals.left.clone ();
				let right = internals.right.clone ();
				succeed! ((left, right))
			},
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<PairImmutable>) {
		let self_0 = try! (self.internals_rc_borrow ());
		let (left, right) = self_0.left_and_right ();
		let left = left.clone ();
		let right = right.clone ();
		let internals = PairImmutableInternals { left, right };
		succeed! (PairImmutable (StdRc::new (internals)))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_immutable (self) -> (Outcome<PairImmutable>) {
		let (left, right) = try! (self.left_and_right_into ());
		succeed! (pair_immutable_new (left, right));
	}
}


impl Pair for PairMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left (&self) -> (&Value) {
		&self.left
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn right (&self) -> (&Value) {
		&self.right
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		(&self.left, &self.right)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		let tuple : &[Value; 2] = unsafe { mem::transmute (self) };
		tuple
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_immutable_new (left : Value, right : Value) -> (PairImmutable) {
	let internals = PairImmutableInternals { left, right };
	PairImmutable (StdRc::new (internals))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_mutable_new (left : Value, right : Value) -> (PairMutable) {
	let internals = PairMutableInternals { left, right };
	PairMutable (StdRc::new (StdRefCell::new (internals)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_new (left : Value, right : Value, immutable : Option<bool>) -> (Value) {
	if immutable.unwrap_or (PAIR_NEW_IMMUTABLE) {
		pair_immutable_new (left, right) .into ()
	} else {
		pair_mutable_new (left, right) .into ()
	}
}




pub struct ListPairIterator <'a> ( Option<ValueRef<'a>>, bool );


impl <'a> ListPairIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value, dotted : bool) -> (Outcome<ListPairIterator<'a>>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListPairIterator (Some (value), dotted));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn dotted (self) -> (Option<ValueRef<'a>>) {
		return self.0;
	}
}


impl <'a> iter::Iterator for ListPairIterator <'a> {
	
	type Item = Outcome<PairAsRef<'a>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<PairAsRef<'a>>>) {
		let (pair, cursor) = {
			let previous = if let Some (ref previous) = self.0 {
				previous
			} else {
				return None
			};
			let (pair, cursor) = match previous.kind () {
				ValueKind::PairImmutable => {
					let pair = previous.clone_ref () .map_generic::<PairImmutable, _> (|value| value.expect_as_ref_0 ());
					let cursor = pair.clone_ref () .map_value (|pair| pair.right ());
					let pair = PairAsRef::new_immutable_from_generic_ref (pair);
					(Some (pair), Some (cursor))
				},
				ValueKind::PairMutable => {
					let pair = previous.clone_ref () .map_generic::<PairMutable, _> (|value| value.expect_as_ref_0 ());
					let cursor = match PairRef::new_embedded_mutable (pair.clone ()) {
						Ok (pair) =>
							pair.right_ref_into (),
						Err (error) =>
							return Some (Err (error)),
					};
					let pair = PairAsRef::new_mutable_from_generic_ref (pair);
					(Some (pair), Some (cursor))
				},
				ValueKind::Null =>
					(None, None),
				_ =>
					if self.1 {
						return None;
					} else {
						#[ allow (unreachable_code) ]
						return Some (failed! (0x1f8fea4c));
					},
			};
			if let Some (ref cursor) = cursor {
				if previous.value_is_self (cursor) {
					#[ allow (unreachable_code) ]
					return Some (failed! (0xa8ab23fb));
				}
			}
			(pair, cursor)
		};
		self.0 = cursor;
		if let Some (pair) = pair {
			return Some (succeeded! (pair));
		} else {
			return None;
		}
	}
}




pub struct ListIterator <'a> ( Option<ValueRef<'a>>, bool );


impl <'a> ListIterator <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (value : &'a Value, dotted : bool) -> (Outcome<ListIterator<'a>>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListIterator (Some (value), dotted));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn dotted (self) -> (Option<ValueRef<'a>>) {
		return self.0;
	}
}


impl <'a> iter::Iterator for ListIterator <'a> {
	
	type Item = Outcome<ValueRef<'a>>;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn next (&mut self) -> (Option<Outcome<ValueRef<'a>>>) {
		let (value, cursor) = {
			let previous = if let Some (ref previous) = self.0 {
				previous
			} else {
				return None
			};
			let (value, cursor) = match previous.kind () {
				ValueKind::PairImmutable => {
					let pair = previous.clone_ref () .map_generic::<PairImmutable, _> (|value| value.expect_as_ref_0 ());
					let cursor = pair.clone_ref () .map_value (|pair| pair.right ());
					let value = pair.map_value (|pair| pair.left ());
					(Some (value), Some (cursor))
				},
				ValueKind::PairMutable => {
					let pair = previous.clone_ref () .map_generic::<PairMutable, _> (|value| value.expect_as_ref_0 ());
					let pair = match PairRef::new_embedded_mutable (pair.clone ()) {
						Ok (pair) =>
							pair,
						Err (error) =>
							return Some (Err (error)),
					};
					let cursor = pair.clone_ref () .right_ref_into ();
					let value = pair.left_ref_into ();
					(Some (value), Some (cursor))
				},
				ValueKind::Null =>
					(None, None),
				_ =>
					if self.1 {
						return None;
					} else {
						#[ allow (unreachable_code) ]
						return Some (failed! (0xed511f9c));
					},
			};
			if let Some (ref cursor) = cursor {
				if previous.value_is_self (cursor) {
					#[ allow (unreachable_code) ]
					return Some (failed! (0x2f6495d9));
				}
			}
			(value, cursor)
		};
		self.0 = cursor;
		if let Some (value) = value {
			return Some (succeeded! (value));
		} else {
			return None;
		}
	}
}




pub struct ListIterators <'a> ( StdVec<ListIterator<'a>> );


impl <'a> ListIterators <'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lists : &'a [&Value], dotted : bool) -> (Outcome<ListIterators<'a>>) {
		let iterators = try! (lists.iter () .map (|list| ListIterator::new (list, dotted)) .collect ());
		succeed! (ListIterators (iterators));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn dotted (self) -> (StdVec<Option<ValueRef<'a>>>) {
		return vec_map_into! (self.0, iterator, iterator.dotted ());
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

