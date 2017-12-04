

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::fmt;
use std::ptr;




pub mod exports {
	
	pub use super::ProcedureExtended;
	pub use super::ProcedureExtendedInternals;
	
	pub use super::procedure_extended_0_evaluate;
	pub use super::procedure_extended_1_evaluate;
	pub use super::procedure_extended_2_evaluate;
	pub use super::procedure_extended_3_evaluate;
	pub use super::procedure_extended_4_evaluate;
	pub use super::procedure_extended_5_evaluate;
	pub use super::procedure_extended_n_evaluate;
	pub use super::procedure_extended_evaluate;
	
	pub use super::procedure_extended_accepts_arity;
	
}




#[ derive (Clone, Hash) ]
pub struct ProcedureExtended ( StdRc<ProcedureExtendedInternals> );


#[ derive (Debug, Hash) ]
pub enum ProcedureExtendedInternals {
	
	ComposedPrimitive1 (StdBox<[ProcedurePrimitive1]>),
	
}


impl ProcedureExtended {
	
	pub fn new (internals : ProcedureExtendedInternals) -> (ProcedureExtended) {
		return ProcedureExtended (StdRc::new (internals));
	}
	
	pub fn internals (&self) -> (&ProcedureExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn is_self (&self, other : &ProcedureExtended) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for ProcedureExtended {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<procedure-extended>")
	}
}


impl fmt::Debug for ProcedureExtended {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}




pub fn procedure_extended_0_evaluate (extended : &ProcedureExtended, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x9507fccd),
		
	}
}




pub fn procedure_extended_1_evaluate (extended : &ProcedureExtended, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
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




pub fn procedure_extended_2_evaluate (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x786569ea),
		
	}
}




pub fn procedure_extended_3_evaluate (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x3a0174c2),
		
	}
}




pub fn procedure_extended_4_evaluate (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x25d23c58),
		
	}
}




pub fn procedure_extended_5_evaluate (extended : &ProcedureExtended, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			fail! (0x80e07b4f),
		
	}
}




pub fn procedure_extended_n_evaluate (extended : &ProcedureExtended, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			if inputs_count == 1 {
				return procedure_extended_1_evaluate (extended, &inputs[0], evaluator);
			} else {
				fail! (0x7b179cf1);
			}
		
	}
}




pub fn procedure_extended_evaluate (extended : &ProcedureExtended, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	return procedure_extended_n_evaluate (extended, inputs, evaluator);
}




pub fn procedure_extended_accepts_arity (extended : &ProcedureExtended, arity : usize) -> (bool) {
	match *extended.internals () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (_) =>
			return arity == 1,
		
	}
}

