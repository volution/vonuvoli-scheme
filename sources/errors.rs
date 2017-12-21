

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
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Error) -> (bool) {
		self.code == other.code
	}
}




#[ inline (always) ]
pub fn error_generic (code : u32) -> (Error) {
	Error {code : code}
}

#[ inline (always) ]
pub fn error_unimplemented (code : u32) -> (Error) {
	Error {code : code}
}

#[ inline (always) ]
pub fn error_panic (code : u32) -> (Error) {
	Error {code : code}
}

