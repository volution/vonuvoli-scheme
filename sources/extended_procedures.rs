

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ProcedureExtended;
	pub use super::ProcedureExtendedInternals;
	
	pub use super::procedure_extended_evaluate_0;
	pub use super::procedure_extended_evaluate_1;
	pub use super::procedure_extended_evaluate_2;
	pub use super::procedure_extended_evaluate_3;
	pub use super::procedure_extended_evaluate_4;
	pub use super::procedure_extended_evaluate_5;
	pub use super::procedure_extended_evaluate_n;
	
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ProcedureExtended ( StdRc<ProcedureExtendedInternals> );


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureExtendedInternals {
	
	ComposedPrimitive1 (StdBox<[ProcedurePrimitive1]>),
	
}


impl ProcedureExtended {
	
	#[ inline (always) ]
	pub fn new (internals : ProcedureExtendedInternals) -> (ProcedureExtended) {
		return ProcedureExtended (StdRc::new (internals));
	}
	
	#[ inline (always) ]
	pub fn internals_ref (&self) -> (&ProcedureExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ProcedureExtended) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_0 (extended : &ProcedureExtended, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x9507fccd),
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_1 (extended : &ProcedureExtended, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (ref primitives) => {
			let primitives = primitives.as_ref ();
			if primitives.is_empty () {
				fail! (0x3ba06e9c);
			}
			let mut value = input_1.clone ();
			for primitive in primitives.iter () .rev () {
				value = try! (procedure_primitive_1_evaluate (*primitive, &value, evaluator));
			}
			succeed! (value);
		}
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_2 (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x786569ea),
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_3 (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x3a0174c2),
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_4 (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x25d23c58),
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_5 (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x80e07b4f),
		
	}
}




#[ inline (always) ]
pub fn procedure_extended_evaluate_n (extended : &ProcedureExtended, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			if inputs_count == 1 {
				return procedure_extended_evaluate_1 (extended, inputs[0], evaluator);
			} else {
				fail! (0x7b179cf1);
			}
		
	}
}

