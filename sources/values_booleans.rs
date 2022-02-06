

use super::prelude::*;




pub mod exports {
	pub use super::Boolean;
	pub use super::boolean;
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Boolean ( pub bool );


impl Boolean {
	
	pub fn value (&self) -> (bool) {
		self.0
	}
	
	pub fn is_true (&self) -> (bool) {
		self.0
	}
	
	pub fn is_false (&self) -> (bool) {
		! self.0
	}
	
	pub fn is_self (&self, other : &Boolean) -> (bool) {
		bool::eq (&self.0, &other.0)
	}
}


impl Boolean {
	
	pub fn not (&self) -> (Boolean) {
		(!self.0) .into ()
	}
	
	pub fn and (&self, other : &Boolean) -> (Boolean) {
		(self.0 && other.0) .into ()
	}
	
	pub fn or (&self, other : &Boolean) -> (Boolean) {
		(self.0 || other.0) .into ()
	}
	
	pub fn xor (&self, other : &Boolean) -> (Boolean) {
		(self.0 ^ other.0) .into ()
	}
	
	pub fn nand (&self, other : &Boolean) -> (Boolean) {
		self.and (other) .not ()
	}
	
	pub fn nor (&self, other : &Boolean) -> (Boolean) {
		self.or (other) .not ()
	}
	
	pub fn nxor (&self, other : &Boolean) -> (Boolean) {
		self.xor (other) .not ()
	}
}


impl ops::Not for Boolean {
	
	type Output = Boolean;
	
	fn not (self) -> (Boolean) {
		Boolean::not (&self)
	}
}




pub fn boolean (value : bool) -> (Boolean) {
	Boolean (value)
}

