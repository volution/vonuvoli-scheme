

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;

use super::primitives_arithmetic::*;
use super::primitives_bitwise::*;
use super::primitives_boolean::*;
use super::primitives_types::*;




pub mod exports {
	
	pub use super::ProcedurePrimitive0;
	pub use super::ProcedurePrimitive1;
	pub use super::ProcedurePrimitive2;
	pub use super::ProcedurePrimitiveN;
	pub use super::ProcedurePrimitive;
	
	pub use super::procedure_primitive_0_evaluate;
	pub use super::procedure_primitive_1_evaluate;
	pub use super::procedure_primitive_2_evaluate;
	pub use super::procedure_primitive_n_evaluate;
	pub use super::procedure_primitive_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive {
	Unimplemented,
	Primitive0 ( ProcedurePrimitive0 ),
	Primitive1 ( ProcedurePrimitive1 ),
	Primitive2 ( ProcedurePrimitive2 ),
	PrimitiveN ( ProcedurePrimitiveN ),
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive0 {
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive1 {
	Type ( TypePrimitive1 ),
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive2 {
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitiveN {
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
}




#[ inline (always) ]
pub fn procedure_primitive_0_evaluate (_primitive : ProcedurePrimitive0, _context : &mut EvaluationContext) -> (Outcome<Value>) {
	failed_unimplemented! (0xf5c28466)
}


#[ inline (always) ]
pub fn procedure_primitive_1_evaluate (primitive : ProcedurePrimitive1, input : &Value, _context : &mut EvaluationContext) -> (Outcome<Value>) {
	match primitive {
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input),
		_ =>
			return failed_unimplemented! (0x85e495c7)
	}
}


#[ inline (always) ]
pub fn procedure_primitive_2_evaluate (_primitive : ProcedurePrimitive2, _input_1 : &Value, _input_2 : &Value, _context : &mut EvaluationContext) -> (Outcome<Value>) {
	failed_unimplemented! (0x9ed223e5)
}


#[ inline (always) ]
pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[Value], _context : &mut EvaluationContext) -> (Outcome<Value>) {
	match primitive {
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs),
		_ =>
			return failed_unimplemented! (0xa1bd3a06)
	}
}




#[ inline (always) ]
pub fn procedure_primitive_evaluate (primitive : ProcedurePrimitive, inputs : &[Value], context : &mut EvaluationContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		ProcedurePrimitive::Primitive0 (primitive) =>
			if inputs_count != 0 {
				return failed! (0xabfe1f25);
			} else {
				return procedure_primitive_0_evaluate (primitive, context);
			},
		ProcedurePrimitive::Primitive1 (primitive) =>
			if inputs_count != 1 {
				return failed! (0x5bc94cf2);
			} else {
				return procedure_primitive_1_evaluate (primitive, &inputs[0], context);
			},
		ProcedurePrimitive::Primitive2 (primitive) =>
			if inputs_count != 2 {
				return failed! (0xb1c56ed3);
			} else {
				return procedure_primitive_2_evaluate (primitive, &inputs[0], &inputs[1], context);
			},
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, inputs, context),
		ProcedurePrimitive::Unimplemented =>
			return failed_unimplemented! (0x10d3710f),
	}
}

