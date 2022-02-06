

use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Opaque;
	pub use super::{opaque_new, opaque_from_rc};
}




#[ derive ( Clone ) ] // OK
pub struct Opaque ( StdRc<StdBox<dyn StdAny>> );


impl Opaque {
	
	pub fn from_rc (rc : StdRc<StdBox<dyn StdAny>>) -> (Opaque) {
		Opaque (rc)
	}
	
	pub fn clone_rc (rc : &StdRc<StdBox<dyn StdAny>>) -> (Opaque) {
		Opaque::from_rc (StdRc::clone (rc))
	}
	
	pub fn is_self (&self, other : &Opaque) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	pub fn any_rc_clone (&self) -> (StdRc<StdBox<dyn StdAny>>) {
		StdRc::clone (&self.0)
	}
	
	pub fn any_as_ref (&self) -> (&dyn StdAny) {
		StdBox::deref (StdRc::deref (&self.0))
	}
	
	pub fn handle (&self) -> (Handle) {
		let pointer = self.any_as_ref ();
		let pointer : *const dyn StdAny = pointer;
		let pointer : *const () = pointer as *const ();
		let pointer : usize = pointer as usize;
		let pointer : u64 = pointer as u64;
		Handle::new (pointer)
	}
	
	pub fn is <OpaqueValue : StdAny> (&self) -> (bool) {
		self.any_as_ref () .is::<OpaqueValue> ()
	}
	
	pub fn downcast <OpaqueValue : StdAny> (&self) -> (Option<&OpaqueValue>) {
		self.any_as_ref () .downcast_ref::<OpaqueValue> ()
	}
	
	pub fn value_is <OpaqueValue : StdAny> (value : &Value) -> (bool) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::Opaque (value) =>
				return value.is::<OpaqueValue> (),
			_ =>
				return false,
		}
	}
	
	pub fn value_downcast <OpaqueValue : StdAny> (value : &Value) -> (Option<&OpaqueValue>) {
		match value.kind_match_as_ref () {
			ValueKindMatchAsRef::Opaque (value) =>
				return value.downcast::<OpaqueValue> (),
			_ =>
				return None,
		}
	}
}




pub fn opaque_new <Value : StdAny> (value : Value) -> (Opaque) {
	Opaque (StdRc::new (StdBox::new (value)))
}

pub fn opaque_from_rc (value : StdRc<StdBox<dyn StdAny>>) -> (Opaque) {
	Opaque::from_rc (value)
}

