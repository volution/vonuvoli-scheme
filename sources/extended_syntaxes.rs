

use super::runtime::exports::*;

use std::fmt;
use std::ptr;




pub mod exports {
	
	pub use super::SyntaxExtended;
	pub use super::SyntaxExtendedInternals;
	
}




#[ derive (Clone, Hash) ]
pub struct SyntaxExtended ( StdRc<SyntaxExtendedInternals> );


#[ derive (Debug, Hash) ]
pub enum SyntaxExtendedInternals {}


impl SyntaxExtended {
	
	pub fn new (internals : SyntaxExtendedInternals) -> (SyntaxExtended) {
		return SyntaxExtended (StdRc::new (internals));
	}
	
	pub fn internals (&self) -> (&SyntaxExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn is_self (&self, other : &SyntaxExtended) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for SyntaxExtended {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<syntax-extended>")
	}
}


impl fmt::Debug for SyntaxExtended {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}

