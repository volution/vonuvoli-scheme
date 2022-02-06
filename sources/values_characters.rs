

use super::prelude::*;




pub mod exports {
	pub use super::Character;
	pub use super::character;
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Character ( pub char );


impl Character {
	
	pub fn value (&self) -> (char) {
		self.0
	}
	
	pub fn is_self (&self, other : &Character) -> (bool) {
		char::eq (&self.0, &other.0)
	}
}




pub fn character (value : char) -> (Character) {
	Character (value)
}

