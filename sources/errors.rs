

use super::runtime::exports::*;

use std::fmt;




pub mod exports {
	
	pub use super::Outcome;
	pub use super::Error;
	
	pub use super::error_generic;
	pub use super::error_unimplemented;
	
}




pub type Outcome<T> = StdResult<T, Error>;




#[ derive (Clone, Eq, PartialEq, Hash) ]
pub struct Error {
	pub code : u32,
}


impl fmt::Display for Error {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<error:{:08x}>", self.code)
	}
}


impl fmt::Debug for Error {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "#<error:{:08x}>", self.code)
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

