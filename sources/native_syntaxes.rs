

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
	
	pub use super::SyntaxNativeFnG;
	
	pub use super::super::conversions::syntax_native_g;
	
}




#[ derive ( Clone ) ] // OK ~~
pub struct SyntaxNativeG (pub SyntaxNativeFnG);


pub type SyntaxNativeFnG = fn (CompilerContext, Value) -> (Outcome<(CompilerContext, Expression)>);




#[ derive ( Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct SyntaxNative ( StdRc<SyntaxNativeInternals> );


#[ derive ( Clone ) ] // OK ~~
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum SyntaxNativeInternals {
	
	NativeG (SyntaxNativeG),
	
}


impl SyntaxNative {
	
	pub fn new (internals : SyntaxNativeInternals) -> (SyntaxNative) {
		return SyntaxNative (StdRc::new (internals));
	}
	
	pub fn internals_ref (&self) -> (&SyntaxNativeInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn internals_into (self) -> (SyntaxNativeInternals) {
		let self_0 = self.internals_ref ();
		return self_0.clone ();
	}
	
	pub fn symbol (&self) -> (BacktraceSymbol) {
		let self_0 = self.internals_ref ();
		return self_0.symbol ();
	}
	
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle ();
	}
	
	pub fn is_self (&self, other : &SyntaxNative) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || Handle::eq (&self.handle (), &other.handle ())
	}
}




impl SyntaxNativeInternals {
	
	pub fn symbol (&self) -> (BacktraceSymbol) {
		match *self {
			SyntaxNativeInternals::NativeG (ref native) =>
				return native.symbol (),
		}
	}
	
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}




impl SyntaxNativeG {
	
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

