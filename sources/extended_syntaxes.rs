

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxExtended;
	pub use super::SyntaxExtendedInternals;
	
}




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
pub struct SyntaxExtended ( StdRc<SyntaxExtendedInternals> );


#[ cfg_attr ( feature = "vonuvoli_eqord", derive ( Eq, PartialEq, Ord, PartialOrd ) ) ] // OK !!
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::empty_enum) ) ]
pub enum SyntaxExtendedInternals {
	// FIXME: ...
}


impl SyntaxExtended {
	
	pub fn new (internals : SyntaxExtendedInternals) -> (SyntaxExtended) {
		return SyntaxExtended (StdRc::new (internals));
	}
	
	pub fn internals_ref (&self) -> (&SyntaxExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn is_self (&self, other : &SyntaxExtended) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}

