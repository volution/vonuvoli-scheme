

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::mem;




pub mod exports {
	
	pub use super::ProcedureNative;
	pub use super::ProcedureNativeInternals;
	
	pub use super::ProcedureNative0;
	pub use super::ProcedureNative1;
	pub use super::ProcedureNative2;
	pub use super::ProcedureNative3;
	pub use super::ProcedureNative4;
	pub use super::ProcedureNative5;
	pub use super::ProcedureNativeN;
	
}




pub type ProcedureNative0 = fn (&mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNative1 = fn (&Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNative2 = fn (&Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNative3 = fn (&Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNative4 = fn (&Value, &Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNative5 = fn (&Value, &Value, &Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeN = fn (&[&Value], &mut EvaluatorContext) -> (Outcome<Value>);




#[ derive (Clone) ]
pub struct ProcedureNative ( StdBox<ProcedureNativeInternals> );


#[ derive (Clone) ]
pub enum ProcedureNativeInternals {
	
	Native0 (ProcedureNative0),
	Native1 (ProcedureNative1),
	Native2 (ProcedureNative2),
	Native3 (ProcedureNative3),
	Native4 (ProcedureNative4),
	Native5 (ProcedureNative5),
	NativeN (ProcedureNativeN),
	
}


impl ProcedureNative {
	
	#[ inline (always) ]
	pub fn new (internals : ProcedureNativeInternals) -> (ProcedureNative) {
		return ProcedureNative (StdBox::new (internals));
	}
	
	#[ inline (always) ]
	pub fn internals (&self) -> (&ProcedureNativeInternals) {
		return &self.0;
	}
	
	#[ inline (always) ]
	pub fn internals_into (self) -> (ProcedureNativeInternals) {
		return *self.0;
	}
	
	#[ inline (always) ]
	pub fn handle_value (&self) -> (u64) {
		let self_0 = self.internals ();
		match *self_0 {
			ProcedureNativeInternals::Native0 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::Native1 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::Native2 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::Native3 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::Native4 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::Native5 (ref native) =>
				unsafe { mem::transmute_copy (native) },
			ProcedureNativeInternals::NativeN (ref native) =>
				unsafe { mem::transmute_copy (native) },
		}
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ProcedureNative) -> (bool) {
		return self.handle_value () == other.handle_value ();
	}
}


impl cmp::Eq for ProcedureNative {}

impl cmp::PartialEq for ProcedureNative {
	fn eq (&self, other : &ProcedureNative) -> (bool) {
		u64::eq (&self.handle_value (), &other.handle_value ())
	}
}


impl cmp::Ord for ProcedureNative {
	fn cmp (&self, other : &ProcedureNative) -> (cmp::Ordering) {
		u64::cmp (&self.handle_value (), &other.handle_value ())
	}
}

impl cmp::PartialOrd for ProcedureNative {
	fn partial_cmp (&self, other : &ProcedureNative) -> (Option<cmp::Ordering>) {
		u64::partial_cmp (&self.handle_value (), &other.handle_value ())
	}
}


impl hash::Hash for ProcedureNative {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_value () .hash (hasher);
	}
}


impl fmt::Display for ProcedureNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "#<procedure-native:{:016x}>", self.handle_value ());
	}
}


impl fmt::Debug for ProcedureNative {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.debug_tuple ("ProcedureNative") .field (&self.handle_value ()) .finish ()
	}
}
