

use super::runtime::exports::*;
use super::values_value::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Values;
	pub use super::{values_new, values_new_from_vec};
	pub use super::{values_clone_slice, values_clone_slice_ref};
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
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
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdBox<[Value]>>) {
		self.0.clone ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_new (values : StdBox<[Value]>) -> (Values) {
	Values (StdRc::new (values))
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
pub fn values_clone_slice_ref (values : &[&Value]) -> (Values) {
	values_new_from_vec (vec_clone_slice_ref (values))
}

