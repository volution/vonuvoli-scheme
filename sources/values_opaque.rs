

use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Opaque;
	pub use super::{opaque_new};
}




#[ derive (Clone) ]
pub struct Opaque ( StdRc<StdBox<StdAny>> );


impl Opaque {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<StdAny>>) -> (Opaque) {
		Opaque (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<StdAny>>) -> (Opaque) {
		Opaque::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Opaque) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn any_rc_clone (&self) -> (StdRc<StdBox<StdAny>>) {
		StdRc::clone (&self.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn any_as_ref (&self) -> (&StdAny) {
		StdBox::deref (StdRc::deref (&self.0))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let pointer = self.any_as_ref ();
		let pointer : *const StdAny = pointer;
		let pointer : *const () = pointer as *const ();
		let pointer : usize = pointer as usize;
		let pointer : u64 = pointer as u64;
		Handle::new (pointer)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is <Value : StdAny> (&self) -> (bool) {
		self.any_as_ref () .is::<Value> ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn downcast <Value : StdAny> (&self) -> (Option<&Value>) {
		self.any_as_ref () .downcast_ref::<Value> ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn opaque_new <Value : StdAny> (value : Value) -> (Opaque) {
	Opaque (StdRc::new (StdBox::new (value)))
}

