

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_arithmetic::*;
use super::primitives_arrays::*;
use super::primitives_bitwise::*;
use super::primitives_boolean::*;
use super::primitives_bytes::*;
use super::primitives_functions::*;
use super::primitives_lists::*;
use super::primitives_strings::*;
use super::primitives_types::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ProcedurePrimitive0;
	pub use super::ProcedurePrimitive1;
	pub use super::ProcedurePrimitive2;
	pub use super::ProcedurePrimitive3;
	pub use super::ProcedurePrimitive4;
	pub use super::ProcedurePrimitiveN;
	pub use super::ProcedurePrimitive;
	
	pub use super::procedure_primitive_0_evaluate;
	pub use super::procedure_primitive_1_evaluate;
	pub use super::procedure_primitive_2_evaluate;
	pub use super::procedure_primitive_3_evaluate;
	pub use super::procedure_primitive_4_evaluate;
	pub use super::procedure_primitive_n_evaluate;
	pub use super::procedure_primitive_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive {
	
	Primitive0 ( ProcedurePrimitive0 ),
	Primitive1 ( ProcedurePrimitive1 ),
	Primitive2 ( ProcedurePrimitive2 ),
	Primitive3 ( ProcedurePrimitive3 ),
	Primitive4 ( ProcedurePrimitive4 ),
	PrimitiveN ( ProcedurePrimitiveN ),
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive0 {
	
	List ( ListPrimitive0 ),
	Array ( ArrayPrimitive0 ),
	Bytes ( BytesPrimitive0 ),
	String ( StringPrimitive0 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive1 {
	
	Type ( TypePrimitive1 ),
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
	List ( ListPrimitive1 ),
	Array ( ArrayPrimitive1 ),
	Bytes ( BytesPrimitive1 ),
	String ( StringPrimitive1 ),
	Functions ( FunctionsPrimitive1 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive2 {
	
	Boolean ( BooleanPrimitive2 ),
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
	List ( ListPrimitive2 ),
	Array ( ArrayPrimitive2 ),
	Bytes ( BytesPrimitive2 ),
	String ( StringPrimitive2 ),
	Functions ( FunctionsPrimitive2 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive3 {
	
	List ( ListPrimitive3 ),
	Array ( ArrayPrimitive3 ),
	Bytes ( BytesPrimitive3 ),
	String ( StringPrimitive3 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive4 {
	
	List ( ListPrimitive4 ),
	Array ( ArrayPrimitive4 ),
	Bytes ( BytesPrimitive4 ),
	String ( StringPrimitive4 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitiveN {
	
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
	List ( ListPrimitiveN ),
	Array ( ArrayPrimitiveN ),
	Bytes ( BytesPrimitiveN ),
	String ( StringPrimitiveN ),
	Functions ( FunctionsPrimitiveN ),
	
}




pub fn procedure_primitive_0_evaluate (primitive : ProcedurePrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_evaluate (primitive),
		
	}
}




pub fn procedure_primitive_1_evaluate (primitive : ProcedurePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Array (primitive) =>
			return array_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Bytes (primitive) =>
			return bytes_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::String (primitive) =>
			return string_primitive_1_evaluate (primitive, input_1),
		
		ProcedurePrimitive1::Functions (primitive) =>
			return functions_primitive_1_evaluate (primitive, input_1, evaluator),
		
	}
}




pub fn procedure_primitive_2_evaluate (primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Array (primitive) =>
			return array_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Bytes (primitive) =>
			return bytes_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::String (primitive) =>
			return string_primitive_2_evaluate (primitive, input_1, input_2),
		
		ProcedurePrimitive2::Functions (primitive) =>
			return functions_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
	}
}




pub fn procedure_primitive_3_evaluate (primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
	}
}




pub fn procedure_primitive_4_evaluate (primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
	}
}




pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Array (primitive) =>
			return array_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			return bytes_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::String (primitive) =>
			return string_primitive_n_evaluate (primitive, inputs),
		
		ProcedurePrimitiveN::Functions (primitive) =>
			return functions_primitive_n_evaluate (primitive, inputs, evaluator),
		
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
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			if inputs_count == 3 {
				return procedure_primitive_3_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], evaluator)
			} else {
				fail! (0x990f006e)
			},
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			if inputs_count == 4 {
				return procedure_primitive_4_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], evaluator)
			} else {
				fail! (0x62f33d3e)
			},
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x10d3710f),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x8baac30b),
		
		ProcedurePrimitive::Reserved =>
			fail! (0xb687a39c),
		
	}
}

