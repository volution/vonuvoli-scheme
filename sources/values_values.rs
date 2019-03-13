

use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Values;
	pub use super::{values_new, values_new_empty, values_new_from_vec, values_from_rc};
	pub use super::{values_clone_slice, values_clone_slice_ref};
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Values ( StdRc<StdBox<[Value]>> );


impl Values {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_as_slice (&self) -> (&[Value]) {
		self.values_ref () .as_ref ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_iter (&self) -> (slice::Iter<Value>) {
		self.values_ref () .iter ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::borrowed_box) ) ]
	pub fn values_ref (&self) -> (&StdBox<[Value]>) {
		self.0.as_ref ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_clone (&self) -> (StdBox<[Value]>) {
		self.values_ref () .clone ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_is_not_empty (&self) -> (bool) {
		! self.values_ref () .is_empty ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<[Value]>>) -> (Values) {
		Values (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<[Value]>>) -> (Values) {
		Values::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Values) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdBox<[Value]>>) {
		StdRc::clone (&self.0)
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_new (values : StdBox<[Value]>) -> (Values) {
	Values (StdRc::new (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_new_empty () -> (Values) {
	Values (StdRc::new (StdBox::new ([])))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_new_from_vec (values : StdVec<Value>) -> (Values) {
	values_new (values.into_boxed_slice ())
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_clone_slice (values : &[Value]) -> (Values) {
	values_new_from_vec (vec_clone_slice (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_clone_slice_ref (values : &[impl StdAsRef<Value>]) -> (Values) {
	values_new_from_vec (vec_clone_slice_ref (values))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_from_rc (values : StdRc<StdBox<[Value]>>) -> (Values) {
	Values::from_rc (values)
}

