

use super::prelude::*;




pub mod exports {
	
	pub use super::Outcome;
	pub use super::Error;
	
	pub use super::error_generic;
	pub use super::error_unimplemented;
	pub use super::error_panic;
	
}




pub type Outcome<T> = Result<T, Error>;




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Error {
	pub code : u32,
}


impl Error {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	pub fn is_self (&self, other : &Error) -> (bool) {
		self.code == other.code
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn error_generic (code : u32) -> (Error) {
	Error {code : code}
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn error_unimplemented (code : u32) -> (Error) {
	Error {code : code}
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn error_panic (code : u32) -> (Error) {
	Error {code : code}
}




impl StdFrom<convert::Infallible> for Error {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	fn from (_error : convert::Infallible) -> (Error) {
		return error_panic (0xddde3965);
	}
}

