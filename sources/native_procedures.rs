

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;

use super::prelude::*;




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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (internals : ProcedureNativeInternals) -> (ProcedureNative) {
		return ProcedureNative (StdBox::new (internals));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ProcedureNativeInternals) {
		return &self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_into (self) -> (ProcedureNativeInternals) {
		return *self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle_value (&self) -> (u64) {
		let self_0 = self.internals_ref ();
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ProcedureNative) -> (bool) {
		return self.handle_value () == other.handle_value ();
	}
}

