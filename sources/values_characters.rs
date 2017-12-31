

use super::prelude::*;




pub mod exports {
	pub use super::Character;
	pub use super::character;
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Character ( pub char );


impl Character {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	pub fn value (&self) -> (char) {
		self.0
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn character (value : char) -> (Character) {
	Character (value)
}

