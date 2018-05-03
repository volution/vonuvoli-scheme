

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxExtended;
	pub use super::SyntaxExtendedInternals;
	
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd) ]
pub struct SyntaxExtended ( StdRc<SyntaxExtendedInternals> );


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub enum SyntaxExtendedInternals {}


impl SyntaxExtended {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (internals : SyntaxExtendedInternals) -> (SyntaxExtended) {
		return SyntaxExtended (StdRc::new (internals));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&SyntaxExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &SyntaxExtended) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}

