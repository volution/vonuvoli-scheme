

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
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new (internals : SyntaxNativeInternals) -> (SyntaxNative) {
		return SyntaxNative (StdBox::new (internals));
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn internals_ref (&self) -> (&SyntaxNativeInternals) {
		return &self.0;
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn internals_into (self) -> (SyntaxNativeInternals) {
		return *self.0;
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn handle_value (&self) -> (u64) {
		let self_0 = self.internals_ref ();
		match *self_0 {
			SyntaxNativeInternals::NativeG (ref native) =>
				unsafe { mem::transmute_copy (native) },
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_self (&self, other : &SyntaxNative) -> (bool) {
		return self.handle_value () == other.handle_value ();
	}
}

