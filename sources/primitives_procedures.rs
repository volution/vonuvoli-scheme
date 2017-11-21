

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;

use super::primitives_arithmetic::*;
use super::primitives_bitwise::*;
use super::primitives_boolean::*;
use super::primitives_lists::*;
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
pub enum ProcedurePrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive1 {
	
	Type ( TypePrimitive1 ),
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
	List ( ListPrimitive1 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive2 {
	
	Boolean ( BooleanPrimitive2 ),
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
	List ( ListPrimitive2 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitiveN {
	
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
	List ( ListPrimitiveN ),
	
}




pub fn procedure_primitive_0_evaluate (_primitive : ProcedurePrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail! (0xf5c28466)
}




pub fn procedure_primitive_1_evaluate (primitive : ProcedurePrimitive1, input : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_evaluate (primitive, input),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_evaluate (primitive, input),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_evaluate (primitive, input),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_evaluate (primitive, input),
		
	}
}




pub fn procedure_primitive_2_evaluate (primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_evaluate (primitive, input_1, input_2),
	}
}




pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_evaluate (primitive, inputs),
		
	}
}




pub fn procedure_primitive_evaluate (primitive : ProcedurePrimitive, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		ProcedurePrimitive::Primitive0 (primitive) =>
			if inputs_count == 0 {
				return procedure_primitive_0_evaluate (primitive, evaluator)
			} else {
				fail! (0xabfe1f25)
			},
		
		ProcedurePrimitive::Primitive1 (primitive) =>
			if inputs_count == 1 {
				return procedure_primitive_1_evaluate (primitive, &inputs[0], evaluator)
			} else {
				fail! (0x5bc94cf2)
			},
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			if inputs_count == 2 {
				return procedure_primitive_2_evaluate (primitive, &inputs[0], &inputs[1], evaluator)
			} else {
				fail! (0xb1c56ed3)
			},
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x10d3710f),
		
	}
}

