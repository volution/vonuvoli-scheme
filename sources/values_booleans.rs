

use super::prelude::*;




pub mod exports {
	pub use super::Boolean;
	pub use super::boolean;
}




#[ derive ( Clone, Eq, PartialEq, Ord, PartialOrd ) ] // OK !!
pub struct Boolean ( pub bool );


impl Boolean {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (&self) -> (bool) {
		self.0
	}
}


impl Boolean {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn not (&self) -> (Boolean) {
		(!self.0) .into ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn and (&self, other : &Boolean) -> (Boolean) {
		(self.0 && other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn or (&self, other : &Boolean) -> (Boolean) {
		(self.0 || other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn xor (&self, other : &Boolean) -> (Boolean) {
		(self.0 ^ other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn nand (&self, other : &Boolean) -> (Boolean) {
		self.and (other) .not ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn nor (&self, other : &Boolean) -> (Boolean) {
		self.or (other) .not ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn nxor (&self, other : &Boolean) -> (Boolean) {
		self.xor (other) .not ()
	}
}


impl ops::Not for Boolean {
	
	type Output = Boolean;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn not (self) -> (Boolean) {
		Boolean::not (&self)
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn boolean (value : bool) -> (Boolean) {
	Boolean (value)
}

