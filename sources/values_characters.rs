

use super::prelude::*;




pub mod exports {
	pub use super::Character;
	pub use super::character;
}




#[ derive ( Clone, Eq, PartialEq, Ord, PartialOrd ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Character ( pub char );


impl Character {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (&self) -> (char) {
		self.0
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character (value : char) -> (Character) {
	Character (value)
}

