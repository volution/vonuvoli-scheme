

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_arithmetic::exports::*;
use super::primitives_arrays::exports::*;
use super::primitives_bitwise::exports::*;
use super::primitives_boolean::exports::*;
use super::primitives_bytes::exports::*;
use super::primitives_functions::exports::*;
use super::primitives_lists::exports::*;
use super::primitives_strings::exports::*;
use super::primitives_types::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ProcedurePrimitive0;
	pub use super::ProcedurePrimitive1;
	pub use super::ProcedurePrimitive2;
	pub use super::ProcedurePrimitive3;
	pub use super::ProcedurePrimitive4;
	pub use super::ProcedurePrimitive5;
	pub use super::ProcedurePrimitiveN;
	pub use super::ProcedurePrimitive;
	
	pub use super::procedure_primitive_0_evaluate;
	pub use super::procedure_primitive_1_evaluate;
	pub use super::procedure_primitive_2_evaluate;
	pub use super::procedure_primitive_3_evaluate;
	pub use super::procedure_primitive_4_evaluate;
	pub use super::procedure_primitive_5_evaluate;
	pub use super::procedure_primitive_n_evaluate;
	pub use super::procedure_primitive_n_evaluate_without_alternatives;
	pub use super::procedure_primitive_evaluate;
	
	pub use super::procedure_primitive_n_alternative_0;
	pub use super::procedure_primitive_n_alternative_1;
	pub use super::procedure_primitive_n_alternative_2;
	pub use super::procedure_primitive_n_alternative_3;
	pub use super::procedure_primitive_n_alternative_4;
	pub use super::procedure_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive {
	
	Primitive0 ( ProcedurePrimitive0 ),
	Primitive1 ( ProcedurePrimitive1 ),
	Primitive2 ( ProcedurePrimitive2 ),
	Primitive3 ( ProcedurePrimitive3 ),
	Primitive4 ( ProcedurePrimitive4 ),
	Primitive5 ( ProcedurePrimitive5 ),
	PrimitiveN ( ProcedurePrimitiveN ),
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive0 {
	
	Boolean ( BooleanPrimitive0 ),
	Arithmetic ( ArithmeticPrimitive0 ),
	Bitwise ( BitwisePrimitive0 ),
	
	List ( ListPrimitive0 ),
	Array ( ArrayPrimitive0 ),
	Bytes ( BytesPrimitive0 ),
	String ( StringPrimitive0 ),
	
	Functions ( FunctionsPrimitive0 ),
	
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
	
	Boolean ( BooleanPrimitive3 ),
	Arithmetic ( ArithmeticPrimitive3 ),
	Bitwise ( BitwisePrimitive3 ),
	
	List ( ListPrimitive3 ),
	Array ( ArrayPrimitive3 ),
	Bytes ( BytesPrimitive3 ),
	String ( StringPrimitive3 ),
	
	Functions ( FunctionsPrimitive3 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive4 {
	
	Boolean ( BooleanPrimitive4 ),
	Arithmetic ( ArithmeticPrimitive4 ),
	Bitwise ( BitwisePrimitive4 ),
	
	List ( ListPrimitive4 ),
	Array ( ArrayPrimitive4 ),
	Bytes ( BytesPrimitive4 ),
	String ( StringPrimitive4 ),
	
	Functions ( FunctionsPrimitive4 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedurePrimitive5 {
	
	Boolean ( BooleanPrimitive5 ),
	Arithmetic ( ArithmeticPrimitive5 ),
	Bitwise ( BitwisePrimitive5 ),
	
	List ( ListPrimitive5 ),
	Array ( ArrayPrimitive5 ),
	Bytes ( BytesPrimitive5 ),
	String ( StringPrimitive5 ),
	
	Functions ( FunctionsPrimitive5 ),
	
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




pub fn procedure_primitive_0_evaluate (primitive : ProcedurePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive0::Boolean (primitive) =>
			return boolean_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Arithmetic (primitive) =>
			return arithmetic_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Bitwise (primitive) =>
			return bitwise_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_evaluate (primitive),
		
		ProcedurePrimitive0::Functions (primitive) =>
			return functions_primitive_0_evaluate (primitive, evaluator),
		
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




pub fn procedure_primitive_3_evaluate (primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive3::Boolean (primitive) =>
			return boolean_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Arithmetic (primitive) =>
			return arithmetic_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Bitwise (primitive) =>
			return bitwise_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_evaluate (primitive, input_1, input_2, input_3),
		
		ProcedurePrimitive3::Functions (primitive) =>
			return functions_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
	}
}




pub fn procedure_primitive_4_evaluate (primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive4::Boolean (primitive) =>
			return boolean_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Arithmetic (primitive) =>
			return arithmetic_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Bitwise (primitive) =>
			return bitwise_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4),
		
		ProcedurePrimitive4::Functions (primitive) =>
			return functions_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
	}
}




pub fn procedure_primitive_5_evaluate (primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive5::Boolean (primitive) =>
			return boolean_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::Arithmetic (primitive) =>
			return arithmetic_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::Bitwise (primitive) =>
			return bitwise_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::List (primitive) =>
			return list_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::Array (primitive) =>
			return array_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::Bytes (primitive) =>
			return bytes_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::String (primitive) =>
			return string_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5),
		
		ProcedurePrimitive5::Functions (primitive) =>
			return functions_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
	}
}




pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	match inputs.len () {
		
		0 =>
			if let Some (primitive) = procedure_primitive_n_alternative_0 (primitive) {
				return procedure_primitive_0_evaluate (primitive, evaluator);
			},
		
		1 =>
			if let Some (primitive) = procedure_primitive_n_alternative_1 (primitive) {
				return procedure_primitive_1_evaluate (primitive, &inputs[0], evaluator);
			},
		
		2 =>
			if let Some (primitive) = procedure_primitive_n_alternative_2 (primitive) {
				return procedure_primitive_2_evaluate (primitive, &inputs[0], &inputs[1], evaluator);
			},
		
		3 =>
			if let Some (primitive) = procedure_primitive_n_alternative_3 (primitive) {
				return procedure_primitive_3_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], evaluator);
			},
		
		4 =>
			if let Some (primitive) = procedure_primitive_n_alternative_4 (primitive) {
				return procedure_primitive_4_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], evaluator);
			},
		
		5 =>
			if let Some (primitive) = procedure_primitive_n_alternative_5 (primitive) {
				return procedure_primitive_5_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4], evaluator);
			},
		
		_ =>
			(),
		
	}
	
	return procedure_primitive_n_evaluate_without_alternatives (primitive, inputs, evaluator);
}




pub fn procedure_primitive_n_evaluate_without_alternatives (primitive : ProcedurePrimitiveN, inputs : &[Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			if inputs_count == 5 {
				return procedure_primitive_5_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4], evaluator)
			} else {
				fail! (0x2e877045)
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




pub fn procedure_primitive_n_alternative_0 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive0>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Functions (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_n_alternative_1 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive1>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Functions (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_n_alternative_2 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive2>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Functions (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_n_alternative_3 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive3>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Functions (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_n_alternative_4 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive4>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Functions (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_n_alternative_5 (primitive : ProcedurePrimitiveN) -> (Option<ProcedurePrimitive5>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::List (primitive) =>
			if let Some (primitive) = list_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Array (primitive) =>
			if let Some (primitive) = array_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::String (primitive) =>
			if let Some (primitive) = string_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveN::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_n_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Functions (primitive))
			} else {
				None
			},
		
	}
}

