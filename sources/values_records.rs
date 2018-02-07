

use super::errors::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{Record, RecordRef, RecordAsRef, RecordKind, RecordKindInternals, RecordImmutable, RecordMutable, RecordMutableInternals};
	pub use super::{RecordMatchAsRef, RecordMatchInto, RecordMatchAsRef2};
	pub use super::{record_immutable_new, record_immutable_clone_slice, record_immutable_clone_slice_ref};
	pub use super::{record_mutable_new, record_mutable_clone_slice, record_mutable_clone_slice_ref};
	pub use super::{record_new, record_clone_slice, record_clone_slice_ref};
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RecordMatchAsRef <'a> {
	Immutable (&'a RecordImmutable),
	Mutable (&'a RecordMutable),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RecordMatchInto {
	Immutable (RecordImmutable),
	Mutable (RecordMutable),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RecordMatchAsRef2 <'a> {
	ImmutableBoth (&'a RecordImmutable, &'a RecordImmutable),
	MutableBoth (&'a RecordMutable, &'a RecordMutable),
	ImmutableAndMutable (&'a RecordImmutable, &'a RecordMutable),
	MutableAndImmutable (&'a RecordMutable, &'a RecordImmutable),
}


impl <'a> RecordMatchAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn record_ref (&self) -> (Outcome<RecordRef<'a>>) {
		match *self {
			RecordMatchAsRef::Immutable (value) =>
				succeed! (value.record_ref ()),
			RecordMatchAsRef::Mutable (value) =>
				return value.record_ref (),
		}
	}
}


impl <'a> RecordMatchAsRef2<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn record_ref (&self) -> (Outcome<(RecordRef<'a>, RecordRef<'a>)>) {
		match *self {
			RecordMatchAsRef2::ImmutableBoth (left, right) =>
				succeed! ((left.record_ref (), right.record_ref ())),
			RecordMatchAsRef2::MutableBoth (left, right) =>
				succeed! ((try! (left.record_ref ()), try! (right.record_ref ()))),
			RecordMatchAsRef2::ImmutableAndMutable (left, right) =>
				succeed! ((left.record_ref (), try! (right.record_ref ()))),
			RecordMatchAsRef2::MutableAndImmutable (left, right) =>
				succeed! ((try! (left.record_ref ()), right.record_ref ())),
		}
	}
}


impl RecordMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			RecordMatchInto::Immutable (value) =>
				value.into (),
			RecordMatchInto::Mutable (value) =>
				value.into (),
		}
	}
}




pub trait Record {
	
	fn kind (&self) -> (&RecordKind);
	
	fn values_as_slice (&self) -> (&[Value]);
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_kind (&self, kind : &RecordKind) -> (bool) {
		RecordKind::is_self (self.kind (), kind)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		self.values_as_slice () .to_vec ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_count (&self) -> (usize) {
		self.kind () .values_count ()
	}
}




#[ derive (Debug) ]
pub enum RecordRef <'a> {
	Immutable (&'a RecordImmutable, &'a RecordKind, &'a [Value]),
	Mutable (&'a RecordMutable, &'a RecordKind, StdRef<'a, [Value]>),
}


impl <'a> RecordRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<RecordRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::RecordImmutable (value) =>
				succeed! (value.record_ref ()),
			ValueKindMatchAsRef::RecordMutable (value) =>
				return value.record_ref (),
			_ =>
				fail! (0xc4f99def),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			RecordRef::Immutable (value, _, _) =>
				(*value) .clone () .into (),
			RecordRef::Mutable (value, _, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &RecordRef) -> (bool) {
		match (self, other) {
			(&RecordRef::Immutable (self_0, _, _), &RecordRef::Immutable (other_0, _, _)) =>
				RecordImmutable::is_self (self_0, other_0),
			(&RecordRef::Mutable (self_0, _, _), &RecordRef::Mutable (other_0, _, _)) =>
				RecordMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> Record for RecordRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn kind (&self) -> (&RecordKind) {
		match *self {
			RecordRef::Immutable (_, kind, _) =>
				kind,
			RecordRef::Mutable (_, kind, _) =>
				kind,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		match *self {
			RecordRef::Immutable (_, _, values) =>
				values,
			RecordRef::Mutable (_, _, ref values) =>
				values,
		}
	}
}




#[ derive (Debug) ]
pub enum RecordAsRef <'a> {
	Immutable (&'a RecordImmutable),
	Mutable (&'a RecordMutable),
}


impl <'a> RecordAsRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try (value : &'a Value) -> (Outcome<RecordAsRef<'a>>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::RecordImmutable (value) =>
				succeed! (RecordAsRef::Immutable (value)),
			ValueKindMatchAsRef::RecordMutable (value) =>
				succeed! (RecordAsRef::Mutable (value)),
			_ =>
				fail! (0x9f91cb83),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn record_ref (&self) -> (Outcome<RecordRef<'a>>) {
		match *self {
			RecordAsRef::Immutable (value) =>
				succeed! (value.record_ref ()),
			RecordAsRef::Mutable (value) =>
				return value.record_ref (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			RecordAsRef::Immutable (value) =>
				(*value) .clone () .into (),
			RecordAsRef::Mutable (value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (Outcome<StdRc<StdBox<[Value]>>>) {
		match *self {
			RecordAsRef::Immutable (value) =>
				succeed! (value.values_rc_clone ()),
			RecordAsRef::Mutable (value) =>
				succeed! (try_or_fail! (((value.0).1) .as_ref () .try_borrow_mut (), 0x2b6ee4f1) .to_cow ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<RecordImmutable>) {
		match *self {
			RecordAsRef::Immutable (value) =>
				succeed! ((*value) .clone ()),
			RecordAsRef::Mutable (value) =>
				return (*value) .to_immutable (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (RecordMutable) {
		match *self {
			RecordAsRef::Immutable (value) =>
				(*value) .to_mutable (),
			RecordAsRef::Mutable (value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &RecordAsRef) -> (bool) {
		match (self, other) {
			(&RecordAsRef::Immutable (self_0), &RecordAsRef::Immutable (other_0)) =>
				RecordImmutable::is_self (self_0, other_0),
			(&RecordAsRef::Mutable (self_0), &RecordAsRef::Mutable (other_0)) =>
				RecordMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}




#[ derive (Clone) ]
pub struct RecordKind ( StdRc<RecordKindInternals> );

#[ derive (Debug) ]
pub struct RecordKindInternals {
	pub identifier : Option<StdRc<StdBox<str>>>,
	pub fields : Option<StdBox<[(Option<StdRc<StdBox<str>>>, bool)]>>,
	pub handle : Handle,
	pub size : usize,
}


impl RecordKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_a (identifier : Option<&str>, fields : Option<&[(Option<&str>, bool)]>, size : usize) -> (Outcome<RecordKind>) {
		let identifier = option_map! (identifier, StdRc::new (StdString::from (identifier) .into_boxed_str ()));
		if let Some (fields) = fields {
			if fields.len () != size {
				fail! (0x51b48e0a);
			}
		}
		let fields = option_map! (fields,
				vec_map! (fields.iter (), &(field, immutable),
						(option_map! (field, StdRc::new (StdString::from (field) .into_boxed_str ())), immutable)
					) .into_boxed_slice ());
		return RecordKind::new (identifier, fields, size);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (identifier : Option<StdRc<StdBox<str>>>, fields : Option<StdBox<[(Option<StdRc<StdBox<str>>>, bool)]>>, size : usize) -> (Outcome<RecordKind>) {
		if let Some (ref fields) = fields {
			// TODO:  Check if a named field appeares twice!
			for &(ref field, _immutable) in fields.iter () {
				if let Some (ref field) = *field {
					if field.is_empty () {
						fail! (0x6880aadb);
					}
				}
			}
		}
		let internals = RecordKindInternals {
				identifier : identifier,
				fields : fields,
				handle : record_handles_next (),
				size : size,
			};
		succeed! (RecordKind (StdRc::new (internals)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn identifier_as_str (&self) -> (Option<&str>) {
		let self_0 = self.internals_ref ();
		let identifier = self_0.identifier.as_ref ();
		let identifier = option_map! (identifier, identifier.as_ref () .as_ref ());
		return identifier;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn identifier_rc_clone (&self) -> (Option<StdRc<StdBox<str>>>) {
		let self_0 = self.internals_ref ();
		let identifier = self_0.identifier.as_ref ();
		let identifier = option_map! (identifier, identifier.clone ());
		return identifier;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_count (&self) -> (usize) {
		let self_0 = self.internals_ref ();
		return self_0.size;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&RecordKindInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<RecordKindInternals>) {
		return self.0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<RecordKindInternals>) {
		return self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &RecordKind) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}




#[ derive (Clone, Debug) ]
pub struct RecordImmutable ( StdRc<(RecordKind, StdRc<StdBox<[Value]>>)> );


impl RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (kind : &RecordKind, rc : StdRc<StdBox<[Value]>>) -> (RecordImmutable) {
		let kind = kind.clone ();
		RecordImmutable (StdRc::new ((kind, rc)))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (kind : &RecordKind, rc : &StdRc<StdBox<[Value]>>) -> (RecordImmutable) {
		RecordImmutable::from_rc (kind, StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &RecordImmutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn record_ref (&self) -> (RecordRef) {
		RecordRef::Immutable (self, &(self.0).0, (self.0).1.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdBox<[Value]>>) {
		(self.0).1.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (RecordMutable) {
		RecordMutable::from_rc (&(self.0).0, self.values_rc_clone ())
	}
}


impl Record for RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn kind (&self) -> (&RecordKind) {
		&(self.0).0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn values_as_slice (&self) -> (&[Value]) {
		(self.0).1.as_ref ()
	}
}




#[ derive (Clone, Debug) ]
pub struct RecordMutable ( StdRc<(RecordKind, StdRc<StdRefCell<RecordMutableInternals>>)> );


#[ derive (Debug) ]
pub enum RecordMutableInternals {
	Owned (StdVec<Value>),
	Cow (StdRc<StdBox<[Value]>>),
}


impl RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (kind : &RecordKind, rc : StdRc<StdBox<[Value]>>) -> (RecordMutable) {
		let kind = kind.clone ();
		let internals = RecordMutableInternals::Cow (rc);
		RecordMutable (StdRc::new ((kind, StdRc::new (StdRefCell::new (internals)))))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (kind : &RecordKind, rc : &StdRc<StdBox<[Value]>>) -> (RecordMutable) {
		RecordMutable::from_rc (kind, StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &RecordMutable) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn record_ref (&self) -> (Outcome<RecordRef>) {
		let reference = try_or_fail! ((self.0).1.as_ref () .try_borrow (), 0xc63c4285);
		let reference = StdRef::map (reference, |reference| reference.as_ref ());
		succeed! (RecordRef::Mutable (self, &(self.0).0, reference));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<RecordMutableInternals>>) {
		(self.0).1.clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_ref_mut (&self) -> (Outcome<StdRefMut<StdVec<Value>>>) {
		let reference = try_or_fail! ((self.0).1.as_ref () .try_borrow_mut (), 0xb93a7d1d);
		let reference = StdRefMut::map (reference, |reference| reference.as_mut ());
		succeed! (reference);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<RecordImmutable>) {
		let mut reference = try_or_fail! ((self.0).1.as_ref () .try_borrow_mut (), 0xe88d42b2);
		let values = reference.to_cow ();
		succeed! (RecordImmutable::from_rc (&(self.0).0, values));
	}
}


impl RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (&RecordKind) {
		&(self.0).0
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_kind (&self, kind : &RecordKind) -> (bool) {
		RecordKind::is_self (self.kind (), kind)
	}
}


impl RecordMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn to_cow (&mut self) -> (StdRc<StdBox<[Value]>>) {
		let values_cow = match *self {
			RecordMutableInternals::Owned (ref mut values_owned) => {
				let mut values_swap = StdVec::new ();
				mem::swap (&mut values_swap, values_owned);
				let values_swap = values_swap.into_boxed_slice ();
				values_swap
			},
			RecordMutableInternals::Cow (ref mut values) =>
				return values.clone (),
		};
		*self = RecordMutableInternals::Cow (StdRc::new (values_cow));
		return self.to_cow ();
	}
}


impl StdAsRef<[Value]> for RecordMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&[Value]) {
		match *self {
			RecordMutableInternals::Owned (ref values) =>
				values.deref (),
			RecordMutableInternals::Cow (ref values) =>
				values.deref (),
		}
	}
}


impl StdAsRefMut<StdVec<Value>> for RecordMutableInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_mut (&mut self) -> (&mut StdVec<Value>) {
		let values_owned = match *self {
			RecordMutableInternals::Owned (ref mut values) =>
				return values,
			RecordMutableInternals::Cow (ref mut values_cow) => {
				let values_cow = StdRc::make_mut (values_cow);
				let mut values_swap = StdVec::new () .into_boxed_slice ();
				mem::swap (&mut values_swap, values_cow);
				let values_swap = StdVec::from (values_swap);
				values_swap
			},
		};
		*self = RecordMutableInternals::Owned (values_owned);
		return self.as_mut ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_immutable_new (kind : &RecordKind, values : StdVec<Value>) -> (Outcome<RecordImmutable>) {
	if kind.values_count () != values.len () {
		fail! (0x186e1398);
	}
	let kind = kind.clone ();
	let record = RecordImmutable (StdRc::new ((kind, StdRc::new (values.into_boxed_slice ()))));
	succeed! (record);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_mutable_new (kind : &RecordKind, values : StdVec<Value>) -> (Outcome<RecordMutable>) {
	if kind.values_count () != values.len () {
		fail! (0x2d222092);
	}
	let kind = kind.clone ();
	let internals = RecordMutableInternals::Owned (values);
	let record = RecordMutable (StdRc::new ((kind, StdRc::new (StdRefCell::new (internals)))));
	succeed! (record);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_new (kind : &RecordKind, values : StdVec<Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	if immutable.unwrap_or (RECORD_NEW_IMMUTABLE) {
		succeed! (try! (record_immutable_new (kind, values)) .into ());
	} else {
		succeed! (try! (record_mutable_new (kind, values)) .into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_immutable_clone_slice (kind : &RecordKind, values : &[Value]) -> (Outcome<RecordImmutable>) {
	record_immutable_new (kind, vec_clone_slice (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_mutable_clone_slice (kind : &RecordKind, values : &[Value]) -> (Outcome<RecordMutable>) {
	record_mutable_new (kind, vec_clone_slice (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_clone_slice (kind : &RecordKind, values : &[Value], immutable : Option<bool>) -> (Outcome<Value>) {
	if immutable.unwrap_or (RECORD_NEW_IMMUTABLE) {
		succeed! (try! (record_immutable_clone_slice (kind, values)) .into ());
	} else {
		succeed! (try! (record_mutable_clone_slice (kind, values)) .into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_immutable_clone_slice_ref (kind : &RecordKind, values : &[&Value]) -> (Outcome<RecordImmutable>) {
	record_immutable_new (kind, vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_mutable_clone_slice_ref (kind : &RecordKind, values : &[&Value]) -> (Outcome<RecordMutable>) {
	record_mutable_new (kind, vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_clone_slice_ref (kind : &RecordKind, values : &[&Value], immutable : Option<bool>) -> (Outcome<Value>) {
	if immutable.unwrap_or (RECORD_NEW_IMMUTABLE) {
		succeed! (try! (record_immutable_clone_slice_ref (kind, values)) .into ());
	} else {
		succeed! (try! (record_mutable_clone_slice_ref (kind, values)) .into ());
	}
}

