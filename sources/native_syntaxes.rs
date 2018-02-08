

use super::errors::exports::*;
use super::expressions::exports::*;
use super::compiler::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxNative;
	pub use super::SyntaxNativeInternals;
	
	pub use super::SyntaxNativeG;
	
}




pub type SyntaxNativeG = fn (CompilerContext, Value) -> (Outcome<(CompilerContext, Expression)>);




#[ derive (Clone) ]
pub struct SyntaxNative ( StdRc<SyntaxNativeInternals> );


#[ derive (Clone) ]
pub enum SyntaxNativeInternals {
	
	NativeG (SyntaxNativeG),
	
}


impl SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (internals : SyntaxNativeInternals) -> (SyntaxNative) {
		return SyntaxNative (StdRc::new (internals));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&SyntaxNativeInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_into (self) -> (SyntaxNativeInternals) {
		let self_0 = self.internals_ref ();
		return self_0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		let value = match *self_0 {
			SyntaxNativeInternals::NativeG (ref native) =>
				unsafe { mem::transmute_copy (native) },
		};
		return Handle::new (value);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &SyntaxNative) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || Handle::eq (&self.handle (), &other.handle ())
	}
}

