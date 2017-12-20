

use super::errors::exports::*;
use super::expressions::exports::*;
use super::compiler::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxNative;
	pub use super::SyntaxNativeInternals;
	
	pub use super::SyntaxNativeG;
	
}




pub type SyntaxNativeG = fn (CompilerContext, Value) -> (Outcome<(CompilerContext, Expression)>);




#[ derive (Clone) ]
pub struct SyntaxNative ( StdBox<SyntaxNativeInternals> );


#[ derive (Clone) ]
pub enum SyntaxNativeInternals {
	
	NativeG (SyntaxNativeG),
	
}


impl SyntaxNative {
	
	#[ inline (always) ]
	pub fn new (internals : SyntaxNativeInternals) -> (SyntaxNative) {
		return SyntaxNative (StdBox::new (internals));
	}
	
	#[ inline (always) ]
	pub fn internals (&self) -> (&SyntaxNativeInternals) {
		return &self.0;
	}
	
	#[ inline (always) ]
	pub fn internals_into (self) -> (SyntaxNativeInternals) {
		return *self.0;
	}
	
	#[ inline (always) ]
	pub fn handle_value (&self) -> (u64) {
		let self_0 = self.internals ();
		match *self_0 {
			SyntaxNativeInternals::NativeG (ref native) =>
				unsafe { mem::transmute_copy (native) },
		}
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &SyntaxNative) -> (bool) {
		return self.handle_value () == other.handle_value ();
	}
}


impl cmp::Eq for SyntaxNative {}

impl cmp::PartialEq for SyntaxNative {
	fn eq (&self, other : &SyntaxNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}


impl cmp::Ord for SyntaxNative {
	fn cmp (&self, other : &SyntaxNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for SyntaxNative {
	fn partial_cmp (&self, other : &SyntaxNative) -> (Option<cmp::Ordering>) {
		u64::partial_cmp (&self.handle_value (), &other.handle_value ())
	}
}


impl hash::Hash for SyntaxNative {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_value () .hash (hasher);
	}
}


impl fmt::Display for SyntaxNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "#<syntax-native:{:016x}>", self.handle_value ());
	}
}


impl fmt::Debug for SyntaxNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("SyntaxNative") .field (&self.handle_value ()) .finish ()
	}
}

