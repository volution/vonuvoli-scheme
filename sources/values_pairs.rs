

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values_value::exports::*;

#[ cfg ( feature = "vonuvoli_values_array" ) ]
use super::values_arrays::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Pair, PairRef, PairAsRef, PairImmutable, PairImmutableInternals};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{PairMutable, PairMutableInternals};
	pub use super::{PairMatchAsRef, PairMatchInto, PairMatchAsRef2};
	pub use super::{pair_new, pair_immutable_new};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{pair_mutable_new};
	pub use super::{ListPairIterator, ListIterator, ListIterators};
}




pub enum PairMatchAsRef <'a> {
	Immutable (&'a PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a PairMutable),
}


pub enum PairMatchInto {
	Immutable (PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (PairMutable),
}


pub enum PairMatchAsRef2 <'a> {
	ImmutableBoth (&'a PairImmutable, &'a PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableBoth (&'a PairMutable, &'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ImmutableAndMutable (&'a PairImmutable, &'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableAndImmutable (&'a PairMutable, &'a PairImmutable),
}


impl <'a> PairMatchAsRef<'a> {
	
	pub fn pair_ref (&self) -> (Outcome<PairRef<'a>>) {
		match *self {
			PairMatchAsRef::Immutable (value) =>
				succeed! (value.pair_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchAsRef::Mutable (value) =>
				return value.pair_ref (),
		}
	}
	
	pub fn pair_as_ref (self) -> (PairAsRef<'a>) {
		match self {
			PairMatchAsRef::Immutable (value) =>
				PairAsRef::Immutable (value),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchAsRef::Mutable (value) =>
				PairAsRef::Mutable (value),
		}
	}
}


impl <'a> PairMatchAsRef2<'a> {
	
	pub fn pair_ref (&self) -> (Outcome<(PairRef<'a>, PairRef<'a>)>) {
		match *self {
			PairMatchAsRef2::ImmutableBoth (left, right) =>
				succeed! ((left.pair_ref (), right.pair_ref ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchAsRef2::MutableBoth (left, right) =>
				succeed! ((r#try! (left.pair_ref ()), r#try! (right.pair_ref ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchAsRef2::ImmutableAndMutable (left, right) =>
				succeed! ((left.pair_ref (), r#try! (right.pair_ref ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchAsRef2::MutableAndImmutable (left, right) =>
				succeed! ((r#try! (left.pair_ref ()), right.pair_ref ())),
		}
	}
}


impl PairMatchInto {
	
	pub fn pair_ref (&self) -> (Outcome<PairRef>) {
		match *self {
			PairMatchInto::Immutable (ref value) =>
				succeed! (value.pair_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchInto::Mutable (ref value) =>
				return value.pair_ref (),
		}
	}
	
	pub fn pair_as_ref (&self) -> (PairAsRef) {
		match self {
			PairMatchInto::Immutable (value) =>
				PairAsRef::Immutable (value),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchInto::Mutable (value) =>
				PairAsRef::Mutable (value),
		}
	}
	
	pub fn value (self) -> (Value) {
		match self {
			PairMatchInto::Immutable (value) =>
				value.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchInto::Mutable (value) =>
				value.into (),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn into_immutable (self) -> (Outcome<PairImmutable>) {
		match self {
			PairMatchInto::Immutable (value) =>
				succeed! (value),
			PairMatchInto::Mutable (value) =>
				return value.into_immutable (),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn into_mutable (self) -> (PairMutable) {
		match self {
			PairMatchInto::Immutable (value) =>
				value.into_mutable (),
			PairMatchInto::Mutable (value) =>
				value,
		}
	}
	
	pub fn left_and_right_into (self) -> (Outcome<(Value, Value)>) {
		match self {
			PairMatchInto::Immutable (value) =>
				succeed! (value.left_and_right_into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairMatchInto::Mutable (value) =>
				return value.left_and_right_into (),
		}
	}
	
	pub fn left_into (self) -> (Outcome<Value>) {
		let (left, _right) = r#try! (self.left_and_right_into ());
		succeed! (left);
	}
	
	pub fn right_into (self) -> (Outcome<Value>) {
		let (_left, right) = r#try! (self.left_and_right_into ());
		succeed! (right);
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
	
	fn values_as_slice (&self) -> (&[Value]) {
		self.left_and_right_as_slice ()
	}
	
	fn values_iter (&self) -> (slice::Iter<Value>) {
		self.left_and_right_as_slice () .iter ()
	}
	
	fn values_clone (&self) -> (StdVec<Value>) {
		let (left, right) = self.left_and_right ();
		vec! [left.clone (), right.clone ()]
	}
	
	fn values_is_empty (&self) -> (bool) {
		false
	}
	
	fn values_is_not_empty (&self) -> (bool) {
		true
	}
	
	fn values_length (&self) -> (usize) {
		2
	}
}




pub enum PairRef <'a> {
	Immutable (&'a StdRc<PairImmutableInternals>, &'a PairImmutableInternals),
	ImmutableEmbedded (StdRc<PairImmutableInternals>, &'a PairImmutableInternals),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a StdRc<StdRefCell<PairMutableInternals>>, StdRef<'a, PairMutableInternals>),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableEmbedded (StdRc<StdRefCell<PairMutableInternals>>, StdRef<'a, PairMutableInternals>),
}


impl <'a> PairRef<'a> {
	
	pub fn try_ref (value : &Value) -> (Outcome<PairRef>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::PairImmutable (value) =>
				succeed! (value.pair_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::PairMutable (value) =>
				return value.pair_ref (),
			_ =>
				fail! (0x0bb90a73),
		}
	}
	
	pub fn new_embedded_immutable (value : PairImmutable) -> (PairRef<'static>) {
		let value = value.internals_rc_into ();
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairRef::ImmutableEmbedded (value, internals)
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn new_embedded_mutable (value : PairMutable) -> (Outcome<PairRef<'static>>) {
		let value = value.internals_rc_into ();
		let internals = unsafe { mem::transmute (try_or_fail! (value.as_ref () .try_borrow (), 0xc98353c8)) };
		succeed! (PairRef::MutableEmbedded (value, internals))
	}
	
	pub fn clone_ref (&self) -> (PairRef<'a>) {
		match *self {
			PairRef::Immutable (value, internals) =>
				PairRef::Immutable (value, internals),
			PairRef::ImmutableEmbedded (ref value, internals) =>
				PairRef::ImmutableEmbedded (StdRc::clone (value), internals),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (value, ref internals) =>
				PairRef::Mutable (value, StdRef::clone (internals)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (ref value, ref internals) =>
				PairRef::MutableEmbedded (StdRc::clone (value), StdRef::clone (internals)),
		}
	}
	
	pub fn value_clone (&self) -> (Value) {
		match *self {
			PairRef::Immutable (value, _) =>
				PairImmutable (StdRc::clone (value)) .into (),
			PairRef::ImmutableEmbedded (ref value, _) =>
				PairImmutable (StdRc::clone (value)) .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (value, _) =>
				PairMutable (StdRc::clone (value)) .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (ref value, _) =>
				PairMutable (StdRc::clone (value)) .into (),
		}
	}
	
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
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairRef::Mutable (self_0, _), &PairRef::Mutable (other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairRef::Mutable (self_0, _), &PairRef::MutableEmbedded (ref other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairRef::MutableEmbedded (ref self_0, _), &PairRef::Mutable (other_0, _)) =>
				StdRc::ptr_eq (self_0, other_0),
			
			_ =>
				false,
		}
	}
	
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, internals) =>
				ValueRef::Immutable (internals.left ()),
			PairRef::ImmutableEmbedded (value, internals) =>
				ValueRef::ImmutableEmbedded (value, internals.left ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, internals) =>
				ValueRef::Mutable (StdRef::map (internals, |internals| internals.left ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (value, internals) =>
				ValueRef::MutableEmbedded (value, StdRef::map (internals, |internals| internals.left ())),
		}
	}
	
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, internals) =>
				ValueRef::Immutable (internals.right ()),
			PairRef::ImmutableEmbedded (value, internals) =>
				ValueRef::ImmutableEmbedded (value, internals.right ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, internals) =>
				ValueRef::Mutable (StdRef::map (internals, |internals| internals.right ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (value, internals) =>
				ValueRef::MutableEmbedded (value, StdRef::map (internals, |internals| internals.right ())),
		}
	}
}


impl <'a> Pair for PairRef<'a> {
	
	fn left (&self) -> (&Value) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, ref internals) =>
				internals.left (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left (),
		}
	}
	
	fn right (&self) -> (&Value) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.right (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.right (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, ref internals) =>
				internals.right (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.right (),
		}
	}
	
	fn left_and_right (&self) -> ((&Value, &Value)) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left_and_right (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left_and_right (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, ref internals) =>
				internals.left_and_right (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left_and_right (),
		}
	}
	
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		match *self {
			PairRef::Immutable (_, internals) =>
				internals.left_and_right_as_slice (),
			PairRef::ImmutableEmbedded (_, internals) =>
				internals.left_and_right_as_slice (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::Mutable (_, ref internals) =>
				internals.left_and_right_as_slice (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairRef::MutableEmbedded (_, ref internals) =>
				internals.left_and_right_as_slice (),
		}
	}
}




TODO! ("use internals");
pub enum PairAsRef <'a> {
	Immutable (&'a PairImmutable),
	ImmutableEmbedded (StdRc<dyn StdAny>, &'a PairImmutable),
	ImmutableOwned (PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (&'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableEmbedded (StdRc<dyn StdAny>, &'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableOwned (PairMutable),
}


impl <'a> PairAsRef<'a> {
	
	pub fn r#try (value : &Value) -> (Outcome<PairAsRef>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::PairImmutable (value) =>
				succeed! (PairAsRef::Immutable (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::PairMutable (value) =>
				succeed! (PairAsRef::Mutable (value)),
			_ =>
				fail! (0x1cb1913b),
		}
	}
	
	pub fn new_immutable_from_generic_ref (value : GenericRef<PairImmutable>) -> (PairAsRef) {
		match value {
			GenericRef::Immutable (value) =>
				PairAsRef::Immutable (value),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				PairAsRef::ImmutableEmbedded (embedded, value),
			GenericRef::ImmutableOwned (_, value) =>
				PairAsRef::ImmutableOwned (value.clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::Mutable (value) =>
				PairAsRef::new_embedded_immutable (value.as_ref () .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableEmbedded (_, value) =>
				PairAsRef::new_embedded_immutable (value.as_ref () .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableOwned (_, value) =>
				PairAsRef::ImmutableOwned (value.clone ()),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn new_mutable_from_generic_ref (value : GenericRef<PairMutable>) -> (PairAsRef) {
		match value {
			GenericRef::Immutable (value) =>
				PairAsRef::Mutable (value),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				PairAsRef::MutableEmbedded (embedded, value),
			GenericRef::ImmutableOwned (_, value) =>
				PairAsRef::MutableOwned (value.clone ()),
			GenericRef::Mutable (value) =>
				PairAsRef::new_embedded_mutable (value.as_ref () .clone ()),
			GenericRef::MutableEmbedded (_, value) =>
				PairAsRef::new_embedded_mutable (value.as_ref () .clone ()),
			GenericRef::MutableOwned (_, value) =>
				PairAsRef::MutableOwned (value.clone ()),
		}
	}
	
	pub fn new_embedded_immutable (value : PairImmutable) -> (PairAsRef<'static>) {
		let value = StdRc::new (value);
		PairAsRef::new_embedded_immutable_from_rc (value)
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn new_embedded_mutable (value : PairMutable) -> (PairAsRef<'static>) {
		let value = StdRc::new (value);
		PairAsRef::new_embedded_mutable_from_rc (value)
	}
	
	pub fn new_embedded_immutable_from_rc (value : StdRc<PairImmutable>) -> (PairAsRef<'static>) {
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::ImmutableEmbedded (value, internals)
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn new_embedded_mutable_from_rc (value : StdRc<PairMutable>) -> (PairAsRef<'static>) {
		let internals = unsafe { mem::transmute (value.as_ref ()) };
		PairAsRef::MutableEmbedded (value, internals)
	}
	
	pub fn pair_ref (&'a self) -> (Outcome<PairRef<'a>>) where {
		match *self {
			PairAsRef::Immutable (value) =>
				succeed! (value.pair_ref ()),
			PairAsRef::ImmutableEmbedded (_, value) =>
				succeed! (value.pair_ref ()),
			PairAsRef::ImmutableOwned (ref value) =>
				succeed! (value.pair_ref ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::Mutable (value) =>
				return value.pair_ref (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableEmbedded (_, value) =>
				return value.pair_ref (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableOwned (ref value) =>
				return value.pair_ref (),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<PairMutableInternals>>) {
		match *self {
			PairAsRef::Mutable (value) =>
				return value.internals_ref_mut (),
			PairAsRef::MutableEmbedded (_, value) =>
				return value.internals_ref_mut (),
			PairAsRef::MutableOwned (ref value) =>
				return value.internals_ref_mut (),
			PairAsRef::Immutable (_) | PairAsRef::ImmutableEmbedded (_, _) | PairAsRef::ImmutableOwned (_) =>
				fail! (0x3175a4eb),
		}
	}
	
	pub fn clone (&self) -> (Value) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				(*value) .clone () .into (),
			PairAsRef::ImmutableOwned (ref value) =>
				(*value) .clone () .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::Mutable (value) =>
				(*value) .clone () .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableEmbedded (_, value) =>
				(*value) .clone () .into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableOwned (ref value) =>
				(*value) .clone () .into (),
		}
	}
	
	pub fn to_owned (&self) -> (PairAsRef<'static>) {
		match *self {
			PairAsRef::Immutable (value) =>
				PairAsRef::ImmutableOwned ((*value) .clone ()),
			PairAsRef::ImmutableEmbedded (_, value) =>
				PairAsRef::ImmutableOwned ((*value) .clone ()),
			PairAsRef::ImmutableOwned (ref value) =>
				PairAsRef::ImmutableOwned ((*value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::Mutable (value) =>
				PairAsRef::MutableOwned ((*value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableEmbedded (_, value) =>
				PairAsRef::MutableOwned ((*value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableOwned (ref value) =>
				PairAsRef::MutableOwned ((*value) .clone ()),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn to_immutable (&self) -> (Outcome<PairImmutable>) {
		match *self {
			PairAsRef::Immutable (value) =>
				succeed! ((*value) .clone ()),
			PairAsRef::ImmutableEmbedded (_, value) =>
				succeed! ((*value) .clone ()),
			PairAsRef::ImmutableOwned (ref value) =>
				succeed! ((*value) .clone ()),
			PairAsRef::Mutable (value) =>
				return (*value) .to_immutable (),
			PairAsRef::MutableEmbedded (_, value) =>
				return (*value) .to_immutable (),
			PairAsRef::MutableOwned (ref value) =>
				return (*value) .to_immutable (),
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		match *self {
			PairAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			PairAsRef::ImmutableEmbedded (_, value) =>
				(*value) .to_mutable (),
			PairAsRef::ImmutableOwned (ref value) =>
				(*value) .to_mutable (),
			PairAsRef::Mutable (value) =>
				(*value) .clone (),
			PairAsRef::MutableEmbedded (_, value) =>
				(*value) .clone (),
			PairAsRef::MutableOwned (ref value) =>
				(*value) .clone (),
		}
	}
	
	pub fn is_self (&self, other : &PairAsRef) -> (bool) {
		match (self, other) {
			
			(&PairAsRef::Immutable (self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::Immutable (self_0), &PairAsRef::ImmutableEmbedded (_, other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::Immutable (self_0), &PairAsRef::ImmutableOwned (ref other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			
			(&PairAsRef::ImmutableEmbedded (_, self_0), &PairAsRef::ImmutableEmbedded (_, other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::ImmutableEmbedded (_, self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::ImmutableEmbedded (_, self_0), &PairAsRef::ImmutableOwned (ref other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			
			(&PairAsRef::ImmutableOwned (ref self_0), &PairAsRef::ImmutableOwned (ref other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::ImmutableOwned (ref self_0), &PairAsRef::Immutable (other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairAsRef::ImmutableOwned (ref self_0), &PairAsRef::ImmutableEmbedded (_, other_0)) =>
				PairImmutable::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::Mutable (self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::Mutable (self_0), &PairAsRef::MutableEmbedded (_, other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::Mutable (self_0), &PairAsRef::MutableOwned (ref other_0)) =>
				PairMutable::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableEmbedded (_, self_0), &PairAsRef::MutableEmbedded (_, other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableEmbedded (_, self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableEmbedded (_, self_0), &PairAsRef::MutableOwned (ref other_0)) =>
				PairMutable::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableOwned (ref self_0), &PairAsRef::MutableOwned (ref other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableOwned (ref self_0), &PairAsRef::Mutable (other_0)) =>
				PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&PairAsRef::MutableOwned (ref self_0), &PairAsRef::MutableEmbedded (_, other_0)) =>
				PairMutable::is_self (self_0, other_0),
			
			_ =>
				false,
		}
	}
	
	pub fn left_ref_into (self) -> (Outcome<ValueRef<'a>>) {
		match self {
			PairAsRef::Immutable (value) =>
				succeed! (ValueRef::Immutable (value.left ())),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				succeed! (ValueRef::ImmutableEmbedded (embedded, value.left ())),
			PairAsRef::ImmutableOwned (value) =>
				succeed! (ValueRef::Owned (value.left_into ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::Mutable (value) =>
				succeed! (ValueRef::Mutable (StdRef::map (r#try! (value.internals_rc_borrow ()), |value| value.left ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableEmbedded (embedded, value) =>
				succeed! (ValueRef::MutableEmbedded (embedded, StdRef::map (r#try! (value.internals_rc_borrow ()), |value| value.left ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableOwned (value) =>
				succeed! (ValueRef::Owned (r#try! (value.left_into ()))),
		}
	}
	
	pub fn right_ref_into (self) -> (Outcome<ValueRef<'a>>) {
		match self {
			PairAsRef::Immutable (value) =>
				succeed! (ValueRef::Immutable (value.right ())),
			PairAsRef::ImmutableEmbedded (embedded, value) =>
				succeed! (ValueRef::ImmutableEmbedded (embedded, value.right ())),
			PairAsRef::ImmutableOwned (value) =>
				succeed! (ValueRef::Owned (value.right_into ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::Mutable (value) =>
				succeed! (ValueRef::Mutable (StdRef::map (r#try! (value.internals_rc_borrow ()), |value| value.right ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableEmbedded (embedded, value) =>
				succeed! (ValueRef::MutableEmbedded (embedded, StdRef::map (r#try! (value.internals_rc_borrow ()), |value| value.right ()))),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PairAsRef::MutableOwned (value) =>
				succeed! (ValueRef::Owned (r#try! (value.right_into ()))),
		}
	}
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct PairImmutable ( StdRc<PairImmutableInternals> );

#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct PairImmutableInternals {
	pub left : Value,
	pub right : Value,
}


impl PairImmutable {
	
	pub fn is_self (&self, other : &PairImmutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Immutable (&self.0, self.0.as_ref ())
	}
	
	pub fn internals_rc_clone (&self) -> (StdRc<PairImmutableInternals>) {
		StdRc::clone (&self.0)
	}
	
	pub fn internals_rc_into (self) -> (StdRc<PairImmutableInternals>) {
		unsafe { mem::transmute (self) }
	}
	
	pub fn left_and_right_into (self) -> ((Value, Value)) {
		match StdRc::try_unwrap (self.internals_rc_into ()) {
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
	
	pub fn left_into (self) -> (Value) {
		let (left, _right) = self.left_and_right_into ();
		left
	}
	
	pub fn right_into (self) -> (Value) {
		let (_left, right) = self.left_and_right_into ();
		right
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn to_mutable (&self) -> (PairMutable) {
		let (left, right) = self.left_and_right ();
		let left = left.clone ();
		let right = right.clone ();
		let internals = PairMutableInternals { left, right };
		PairMutable (StdRc::new (StdRefCell::new (internals)))
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn into_mutable (self) -> (PairMutable) {
		let (left, right) = self.left_and_right_into ();
		pair_mutable_new (left, right)
	}
}


#[ cfg ( feature = "vonuvoli_values_pair_drop_iterative" ) ]
impl ops::Drop for PairImmutable {
	
	fn drop (&mut self) -> () {
		loop {
			let internals = if let Some (internals) = StdRc::get_mut (&mut self.0) {
				let mut swap = PairImmutableInternals {
						left : ValueSingleton::Undefined.into (),
						right : ValueSingleton::Undefined.into (),
					};
				mem::swap (&mut swap, internals);
				swap
			} else {
				break;
			};
			let PairImmutableInternals {left, right} = internals;
			match right.try_into () {
				Ok (mut pair) => {
					mem::drop (left);
					mem::swap (self, &mut pair);
					mem::drop (pair);
					continue;
				},
				Err (_) =>
					(),
			}
			match left.try_into () {
				Ok (mut pair) => {
					mem::swap (self, &mut pair);
					mem::drop (pair);
					continue;
				},
				Err (_) =>
					(),
			}
			break;
		}
	}
}


impl Pair for PairImmutable {
	
	fn left (&self) -> (&Value) {
		self.0.as_ref () .left ()
	}
	
	fn right (&self) -> (&Value) {
		self.0.as_ref () .right ()
	}
	
	fn left_and_right (&self) -> ((&Value, &Value)) {
		self.0.as_ref () .left_and_right ()
	}
	
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		self.0.as_ref () .left_and_right_as_slice ()
	}
}


impl Pair for PairImmutableInternals {
	
	fn left (&self) -> (&Value) {
		&self.left
	}
	
	fn right (&self) -> (&Value) {
		&self.right
	}
	
	fn left_and_right (&self) -> ((&Value, &Value)) {
		(&self.left, &self.right)
	}
	
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		let tuple : &[Value; 2] = unsafe { mem::transmute (self) };
		tuple
	}
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // FIXME
pub struct PairMutable ( StdRc<StdRefCell<PairMutableInternals>> );

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct PairMutableInternals {
	pub left : Value,
	pub right : Value,
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl PairMutable {
	
	pub fn is_self (&self, other : &PairMutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	pub fn pair_ref (&self) -> (Outcome<PairRef>) {
		succeed! (PairRef::Mutable (&self.0, r#try! (self.internals_rc_borrow ())));
	}
	
	pub fn internals_rc_borrow (&self) -> (Outcome<StdRef<PairMutableInternals>>) {
		succeed! (try_or_fail! (self.0.as_ref () .try_borrow (), 0xf155eead));
	}
	
	pub fn internals_rc_clone (&self) -> (StdRc<StdRefCell<PairMutableInternals>>) {
		StdRc::clone (&self.0)
	}
	
	pub fn internals_rc_into (self) -> (StdRc<StdRefCell<PairMutableInternals>>) {
		unsafe { mem::transmute (self) }
	}
	
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<PairMutableInternals>>) {
		succeed! (try_or_fail! (self.0.as_ref () .try_borrow_mut (), 0x1bb74db3));
	}
	
	pub fn left_and_right_into (self) -> (Outcome<(Value, Value)>) {
		match StdRc::try_unwrap (self.internals_rc_into ()) {
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
	
	pub fn left_into (self) -> (Outcome<Value>) {
		let (left, _right) = r#try! (self.left_and_right_into ());
		succeed! (left);
	}
	
	pub fn right_into (self) -> (Outcome<Value>) {
		let (_left, right) = r#try! (self.left_and_right_into ());
		succeed! (right);
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn to_immutable (&self) -> (Outcome<PairImmutable>) {
		let self_0 = r#try! (self.internals_rc_borrow ());
		let (left, right) = self_0.left_and_right ();
		let left = left.clone ();
		let right = right.clone ();
		let internals = PairImmutableInternals { left, right };
		succeed! (PairImmutable (StdRc::new (internals)))
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub fn into_immutable (self) -> (Outcome<PairImmutable>) {
		let (left, right) = r#try! (self.left_and_right_into ());
		succeed! (pair_immutable_new (left, right));
	}
}


#[ cfg ( feature = "vonuvoli_values_pair_drop_iterative" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl ops::Drop for PairMutable {
	
	fn drop (&mut self) -> () {
		loop {
			let internals = if let Some (internals) = StdRc::get_mut (&mut self.0) {
				let internals = StdRefCell::get_mut (internals);
				let mut swap = PairMutableInternals {
						left : ValueSingleton::Undefined.into (),
						right : ValueSingleton::Undefined.into (),
					};
				mem::swap (&mut swap, internals);
				swap
			} else {
				break;
			};
			let PairMutableInternals {left, right} = internals;
			match right.try_into () {
				Ok (mut pair) => {
					mem::drop (left);
					mem::swap (self, &mut pair);
					mem::drop (pair);
					continue;
				},
				Err (_) =>
					(),
			}
			match left.try_into () {
				Ok (mut pair) => {
					mem::swap (self, &mut pair);
					mem::drop (pair);
					continue;
				},
				Err (_) =>
					(),
			}
			break;
		}
	}
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl Pair for PairMutableInternals {
	
	fn left (&self) -> (&Value) {
		&self.left
	}
	
	fn right (&self) -> (&Value) {
		&self.right
	}
	
	fn left_and_right (&self) -> ((&Value, &Value)) {
		(&self.left, &self.right)
	}
	
	fn left_and_right_as_slice (&self) -> (&[Value]) {
		let tuple : &[Value; 2] = unsafe { mem::transmute (self) };
		tuple
	}
}




pub fn pair_immutable_new (left : Value, right : Value) -> (PairImmutable) {
	let internals = PairImmutableInternals { left, right };
	PairImmutable (StdRc::new (internals))
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn pair_mutable_new (left : Value, right : Value) -> (PairMutable) {
	let internals = PairMutableInternals { left, right };
	PairMutable (StdRc::new (StdRefCell::new (internals)))
}

pub fn pair_new (left : Value, right : Value, immutable : Option<bool>) -> (Value) {
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	{ if immutable.unwrap_or (PAIR_NEW_IMMUTABLE) {
		pair_immutable_new (left, right) .into ()
	} else {
		pair_mutable_new (left, right) .into ()
	} }
	#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
	pair_immutable_new (left, right) .into ()
}




pub struct ListPairIterator <'a> ( Option<ValueRef<'a>>, bool, bool );


impl <'a> ListPairIterator <'a> {
	
	pub fn new (value : &Value, dotted : bool) -> (Outcome<ListPairIterator>) {
		return ListPairIterator::new_extended (value, dotted, false);
	}
	
	pub fn new_extended (value : &Value, dotted : bool, cloned : bool) -> (Outcome<ListPairIterator>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListPairIterator (Some (value), dotted, cloned));
	}
	
	pub fn dotted (self) -> (Option<ValueRef<'a>>) {
		return self.0;
	}
}


impl <'a> iter::Iterator for ListPairIterator <'a> {
	
	type Item = Outcome<PairAsRef<'a>>;
	
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
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
						return Some (failed! (0x1f8fea4c));
					},
			};
			if let Some (ref cursor) = cursor {
				if previous.value_is_self (cursor) {
					return Some (failed! (0xa8ab23fb));
				}
			}
			(pair, cursor)
		};
		let (pair, cursor) = if self.2 {
			let pair = option_map! (pair, pair.to_owned ());
			let cursor = option_map! (cursor, cursor.value_owned ());
			(pair, cursor)
		} else {
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
	
	pub fn new (value : &Value, dotted : bool) -> (Outcome<ListIterator>) {
		let value = ValueRef::Immutable (value);
		succeed! (ListIterator (Some (value), dotted));
	}
	
	pub fn dotted (self) -> (Option<ValueRef<'a>>) {
		return self.0;
	}
}


impl <'a> iter::Iterator for ListIterator <'a> {
	
	type Item = Outcome<ValueRef<'a>>;
	
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
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
						return Some (failed! (0xed511f9c));
					},
			};
			if let Some (ref cursor) = cursor {
				if previous.value_is_self (cursor) {
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
	
	pub fn new (lists : &[impl StdAsRef<Value>], dotted : bool) -> (Outcome<ListIterators>) {
		let iterators = r#try! (lists.iter () .map (|list| ListIterator::new (list.as_ref (), dotted)) .collect ());
		succeed! (ListIterators (iterators));
	}
	
	pub fn dotted (self) -> (StdVec<Option<ValueRef<'a>>>) {
		return vec_map_into! (self.0, iterator, iterator.dotted ());
	}
}


impl <'a> iter::Iterator for ListIterators <'a> {
	
	type Item = Outcome<StdVec<ValueRef<'a>>>;
	
	fn next (&mut self) -> (Option<Outcome<StdVec<ValueRef<'a>>>>) {
		let mut outcomes = StdVec::with_capacity (self.0.len ());
		for mut iterator in &mut self.0 {
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

