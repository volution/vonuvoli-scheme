

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_arithmetic::exports::*;
use super::primitives_arrays::exports::*;
use super::primitives_bitwise::exports::*;
use super::primitives_boolean::exports::*;
use super::primitives_bytes::exports::*;
use super::primitives_comparisons::exports::*;
use super::primitives_functions::exports::*;
use super::primitives_lists::exports::*;
use super::primitives_ports::exports::*;
use super::primitives_runtime::exports::*;
use super::primitives_strings::exports::*;
use super::primitives_types::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ProcedurePrimitive0;
	pub use super::ProcedurePrimitive1;
	pub use super::ProcedurePrimitive2;
	pub use super::ProcedurePrimitive3;
	pub use super::ProcedurePrimitive4;
	pub use super::ProcedurePrimitive5;
	pub use super::ProcedurePrimitiveN;
	pub use super::ProcedurePrimitiveV;
	pub use super::ProcedurePrimitive;
	
	pub use super::ProcedureArity;
	pub use super::ProcedureAttributes;
	pub use super::ProcedureOutputAttributes;
	
	pub use super::procedure_primitive_0_evaluate;
	pub use super::procedure_primitive_1_evaluate;
	pub use super::procedure_primitive_2_evaluate;
	pub use super::procedure_primitive_3_evaluate;
	pub use super::procedure_primitive_4_evaluate;
	pub use super::procedure_primitive_5_evaluate;
	pub use super::procedure_primitive_n_evaluate;
	
	pub use super::procedure_primitive_v_evaluate_0;
	pub use super::procedure_primitive_v_evaluate_1;
	pub use super::procedure_primitive_v_evaluate_2;
	pub use super::procedure_primitive_v_evaluate_3;
	pub use super::procedure_primitive_v_evaluate_4;
	pub use super::procedure_primitive_v_evaluate_5;
	pub use super::procedure_primitive_v_evaluate_n;
	
	pub use super::procedure_primitive_g_evaluate_0;
	pub use super::procedure_primitive_g_evaluate_1;
	pub use super::procedure_primitive_g_evaluate_2;
	pub use super::procedure_primitive_g_evaluate_3;
	pub use super::procedure_primitive_g_evaluate_4;
	pub use super::procedure_primitive_g_evaluate_5;
	pub use super::procedure_primitive_g_evaluate_n;
	
	pub use super::procedure_primitive_v_alternative_0;
	pub use super::procedure_primitive_v_alternative_1;
	pub use super::procedure_primitive_v_alternative_2;
	pub use super::procedure_primitive_v_alternative_3;
	pub use super::procedure_primitive_v_alternative_4;
	pub use super::procedure_primitive_v_alternative_5;
	pub use super::procedure_primitive_v_alternative_n;
	
	pub use super::procedure_primitive_0_attributes;
	pub use super::procedure_primitive_1_attributes;
	pub use super::procedure_primitive_2_attributes;
	pub use super::procedure_primitive_3_attributes;
	pub use super::procedure_primitive_4_attributes;
	pub use super::procedure_primitive_5_attributes;
	pub use super::procedure_primitive_n_attributes;
	pub use super::procedure_primitive_g_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive {
	
	Primitive0 ( ProcedurePrimitive0 ),
	Primitive1 ( ProcedurePrimitive1 ),
	Primitive2 ( ProcedurePrimitive2 ),
	Primitive3 ( ProcedurePrimitive3 ),
	Primitive4 ( ProcedurePrimitive4 ),
	Primitive5 ( ProcedurePrimitive5 ),
	PrimitiveN ( ProcedurePrimitiveN ),
	PrimitiveV ( ProcedurePrimitiveV ),
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive0 {
	
	Boolean ( BooleanPrimitive0 ),
	Arithmetic ( ArithmeticPrimitive0 ),
	Bitwise ( BitwisePrimitive0 ),
	Comparison ( ComparisonPrimitive0 ),
	
	List ( ListPrimitive0 ),
	Array ( ArrayPrimitive0 ),
	Bytes ( BytesPrimitive0 ),
	String ( StringPrimitive0 ),
	
	Functions ( FunctionsPrimitive0 ),
	Runtime ( RuntimePrimitive0 ),
	Port ( PortPrimitive0 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive1 {
	
	Type ( TypePrimitive1 ),
	
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
	Comparison ( ComparisonPrimitive1 ),
	
	List ( ListPrimitive1 ),
	Array ( ArrayPrimitive1 ),
	Bytes ( BytesPrimitive1 ),
	String ( StringPrimitive1 ),
	
	Functions ( FunctionsPrimitive1 ),
	Runtime ( RuntimePrimitive1 ),
	Port ( PortPrimitive1 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive2 {
	
	Boolean ( BooleanPrimitive2 ),
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
	Comparison ( ComparisonPrimitive2 ),
	
	List ( ListPrimitive2 ),
	Array ( ArrayPrimitive2 ),
	Bytes ( BytesPrimitive2 ),
	String ( StringPrimitive2 ),
	
	Functions ( FunctionsPrimitive2 ),
	Runtime ( RuntimePrimitive2 ),
	Port ( PortPrimitive2 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive3 {
	
	Boolean ( BooleanPrimitive3 ),
	Arithmetic ( ArithmeticPrimitive3 ),
	Bitwise ( BitwisePrimitive3 ),
	Comparison ( ComparisonPrimitive3 ),
	
	List ( ListPrimitive3 ),
	Array ( ArrayPrimitive3 ),
	Bytes ( BytesPrimitive3 ),
	String ( StringPrimitive3 ),
	
	Functions ( FunctionsPrimitive3 ),
	Runtime ( RuntimePrimitive3 ),
	Port ( PortPrimitive3 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive4 {
	
	Boolean ( BooleanPrimitive4 ),
	Arithmetic ( ArithmeticPrimitive4 ),
	Bitwise ( BitwisePrimitive4 ),
	Comparison ( ComparisonPrimitive4 ),
	
	List ( ListPrimitive4 ),
	Array ( ArrayPrimitive4 ),
	Bytes ( BytesPrimitive4 ),
	String ( StringPrimitive4 ),
	
	Functions ( FunctionsPrimitive4 ),
	Runtime ( RuntimePrimitive4 ),
	Port ( PortPrimitive4 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitive5 {
	
	Boolean ( BooleanPrimitive5 ),
	Arithmetic ( ArithmeticPrimitive5 ),
	Bitwise ( BitwisePrimitive5 ),
	Comparison ( ComparisonPrimitive5 ),
	
	List ( ListPrimitive5 ),
	Array ( ArrayPrimitive5 ),
	Bytes ( BytesPrimitive5 ),
	String ( StringPrimitive5 ),
	
	Functions ( FunctionsPrimitive5 ),
	Runtime ( RuntimePrimitive5 ),
	Port ( PortPrimitive5 ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitiveN {
	
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
	Comparison ( ComparisonPrimitiveN ),
	
	List ( ListPrimitiveN ),
	Array ( ArrayPrimitiveN ),
	Bytes ( BytesPrimitiveN ),
	String ( StringPrimitiveN ),
	
	Functions ( FunctionsPrimitiveN ),
	Runtime ( RuntimePrimitiveN ),
	Port ( PortPrimitiveN ),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedurePrimitiveV {
	
	Boolean ( BooleanPrimitiveV ),
	Arithmetic ( ArithmeticPrimitiveV ),
	Bitwise ( BitwisePrimitiveV ),
	Comparison ( ComparisonPrimitiveV ),
	
	List ( ListPrimitiveV ),
	Array ( ArrayPrimitiveV ),
	Bytes ( BytesPrimitiveV ),
	String ( StringPrimitiveV ),
	
	Functions ( FunctionsPrimitiveV ),
	Runtime ( RuntimePrimitiveV ),
	Port ( PortPrimitiveV ),
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ProcedureAttributes {
	
	pub deterministic : bool,
	pub arity : ProcedureArity,
	pub output : ProcedureOutputAttributes,
	
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureArity {
	Undefined,
	Exact ( usize ),
	Bounded ( Option<usize>, Option<usize> ),
	Unbounded,
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureOutputAttributes {
	Undefined,
	Constant,
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_0_evaluate (primitive : ProcedurePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive0::Boolean (primitive) =>
			return boolean_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Arithmetic (primitive) =>
			return arithmetic_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Bitwise (primitive) =>
			return bitwise_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Comparison (primitive) =>
			return comparison_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Functions (primitive) =>
			return functions_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Runtime (primitive) =>
			return runtime_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Port (primitive) =>
			return port_primitive_0_evaluate (primitive, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_1_evaluate (primitive : ProcedurePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Comparison (primitive) =>
			return comparison_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Array (primitive) =>
			return array_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Bytes (primitive) =>
			return bytes_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::String (primitive) =>
			return string_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Functions (primitive) =>
			return functions_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Runtime (primitive) =>
			return runtime_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Port (primitive) =>
			return port_primitive_1_evaluate (primitive, input_1, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_2_evaluate (primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Comparison (primitive) =>
			return comparison_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Array (primitive) =>
			return array_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Bytes (primitive) =>
			return bytes_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::String (primitive) =>
			return string_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Functions (primitive) =>
			return functions_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Runtime (primitive) =>
			return runtime_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Port (primitive) =>
			return port_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_3_evaluate (primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive3::Boolean (primitive) =>
			return boolean_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Arithmetic (primitive) =>
			return arithmetic_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Bitwise (primitive) =>
			return bitwise_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Comparison (primitive) =>
			return comparison_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Functions (primitive) =>
			return functions_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Runtime (primitive) =>
			return runtime_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Port (primitive) =>
			return port_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_4_evaluate (primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive4::Boolean (primitive) =>
			return boolean_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Arithmetic (primitive) =>
			return arithmetic_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Bitwise (primitive) =>
			return bitwise_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Comparison (primitive) =>
			return comparison_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Functions (primitive) =>
			return functions_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Runtime (primitive) =>
			return runtime_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Port (primitive) =>
			return port_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_5_evaluate (primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive5::Boolean (primitive) =>
			return boolean_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Arithmetic (primitive) =>
			return arithmetic_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Bitwise (primitive) =>
			return bitwise_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Comparison (primitive) =>
			return comparison_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::List (primitive) =>
			return list_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Array (primitive) =>
			return array_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Bytes (primitive) =>
			return bytes_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::String (primitive) =>
			return string_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Functions (primitive) =>
			return functions_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Runtime (primitive) =>
			return runtime_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Port (primitive) =>
			return port_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Comparison (primitive) =>
			return comparison_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Array (primitive) =>
			return array_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			return bytes_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::String (primitive) =>
			return string_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Functions (primitive) =>
			return functions_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Runtime (primitive) =>
			return runtime_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Port (primitive) =>
			return port_primitive_n_evaluate (primitive, inputs, evaluator),
		
	}
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_0 (primitive : ProcedurePrimitiveV, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
		return procedure_primitive_0_evaluate (primitive, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[], evaluator);
	}
	
	fail! (0x45c55a28);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_1 (primitive : ProcedurePrimitiveV, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1], evaluator);
	}
	
	fail! (0xef55f95e);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_2 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2], evaluator);
	}
	
	fail! (0x3f354a4d);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_3 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3], evaluator);
	}
	
	fail! (0x315dd3f2);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_4 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4], evaluator);
	}
	
	fail! (0x7ff0968c);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_5 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4, input_5], evaluator);
	}
	
	fail! (0x93a6cbdc);
}




#[ inline (never) ]
pub fn procedure_primitive_v_evaluate_n (primitive : ProcedurePrimitiveV, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	
	match inputs_count {
		
		0 =>
			if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
				return procedure_primitive_0_evaluate (primitive, evaluator);
			},
		
		1 =>
			if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
				return procedure_primitive_1_evaluate (primitive, inputs[0], evaluator);
			},
		
		2 =>
			if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
				return procedure_primitive_2_evaluate (primitive, inputs[0], inputs[1], evaluator);
			},
		
		3 =>
			if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
				return procedure_primitive_3_evaluate (primitive, inputs[0], inputs[1], inputs[2], evaluator);
			},
		
		4 =>
			if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
				return procedure_primitive_4_evaluate (primitive, inputs[0], inputs[1], inputs[2], inputs[3], evaluator);
			},
		
		5 =>
			if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
				return procedure_primitive_5_evaluate (primitive, inputs[0], inputs[1], inputs[2], inputs[3], inputs[4], evaluator);
			},
		
		_ =>
			(),
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluator);
	}
	
	fail! (0x270e1433);
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_0 (primitive : ProcedurePrimitive, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive0 (primitive) =>
			return procedure_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_0 (primitive, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x635e3f45), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x588a2e8f), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0x649839df),
		
		_ =>
			fail! (0x38623a76),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_1 (primitive : ProcedurePrimitive, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive1 (primitive) =>
			return procedure_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_1 (primitive, input_1, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x6c063079), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xc689378a), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0xfed99283),
		
		_ =>
			fail! (0x92f0f0ce),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_2 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_2 (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0xd79d710f), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xfae8451f), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0x0b04b93e),
		
		_ =>
			fail! (0xdaa4c8f5),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_3 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_3 (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x299b0807), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x0d3c6416), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0x310b03d4),
		
		_ =>
			fail! (0x99473830),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_4 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_4 (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x41ea7797), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xbcbf1bcd), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0x37319aac),
		
		_ =>
			fail! (0x723b728c),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_5 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4, input_5], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_5 (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x97c9abd0), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x2e569e14), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0xc8a6c70d),
		
		_ =>
			fail! (0xa4f4a6c6),
	}
}




#[ inline (never) ]
pub fn procedure_primitive_g_evaluate_n (primitive : ProcedurePrimitive, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	
	match primitive {
		
		ProcedurePrimitive::Primitive0 (primitive) =>
			if inputs_count == 0 {
				return procedure_primitive_0_evaluate (primitive, evaluator);
			} else {
				fail! (0xabfe1f25)
			},
		
		ProcedurePrimitive::Primitive1 (primitive) =>
			if inputs_count == 1 {
				return procedure_primitive_1_evaluate (primitive, inputs[0], evaluator);
			} else {
				fail! (0x5bc94cf2)
			},
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			if inputs_count == 2 {
				return procedure_primitive_2_evaluate (primitive, inputs[0], inputs[1], evaluator);
			} else {
				fail! (0xb1c56ed3)
			},
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			if inputs_count == 3 {
				return procedure_primitive_3_evaluate (primitive, inputs[0], inputs[1], inputs[2], evaluator);
			} else {
				fail! (0x990f006e)
			},
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			if inputs_count == 4 {
				return procedure_primitive_4_evaluate (primitive, inputs[0], inputs[1], inputs[2], inputs[3], evaluator);
			} else {
				fail! (0x62f33d3e)
			},
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			if inputs_count == 5 {
				return procedure_primitive_5_evaluate (primitive, inputs[0], inputs[1], inputs[2], inputs[3], inputs[4], evaluator);
			} else {
				fail! (0x2e877045)
			},
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_n (primitive, inputs, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x10d3710f), // OK
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x8baac30b), // OK
		
		ProcedurePrimitive::Reserved =>
			fail! (0xb687a39c),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_0 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive0>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_1 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive1>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_2 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive2>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_3 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive3>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_4 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive4>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_5 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive5>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_v_alternative_n (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitiveN>) {
	match primitive {
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			if let Some (primitive) = boolean_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Boolean (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			if let Some (primitive) = arithmetic_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Arithmetic (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			if let Some (primitive) = bitwise_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Bitwise (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Comparison (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::List (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Array (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Bytes (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::String (primitive) =>
			if let Some (primitive) = string_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::String (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Functions (primitive) =>
			if let Some (primitive) = functions_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Functions (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Runtime (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Port (primitive))
			} else {
				None
			},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_0_attributes (primitive : ProcedurePrimitive0) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive0::Boolean (primitive) =>
			return boolean_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Arithmetic (primitive) =>
			return arithmetic_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Bitwise (primitive) =>
			return bitwise_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Comparison (primitive) =>
			return comparison_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Functions (primitive) =>
			return functions_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Runtime (primitive) =>
			return runtime_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Port (primitive) =>
			return port_primitive_0_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_1_attributes (primitive : ProcedurePrimitive1) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Comparison (primitive) =>
			return comparison_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Array (primitive) =>
			return array_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Bytes (primitive) =>
			return bytes_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::String (primitive) =>
			return string_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Functions (primitive) =>
			return functions_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Runtime (primitive) =>
			return runtime_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Port (primitive) =>
			return port_primitive_1_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_2_attributes (primitive : ProcedurePrimitive2) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Comparison (primitive) =>
			return comparison_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Array (primitive) =>
			return array_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Bytes (primitive) =>
			return bytes_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::String (primitive) =>
			return string_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Functions (primitive) =>
			return functions_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Runtime (primitive) =>
			return runtime_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Port (primitive) =>
			return port_primitive_2_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_3_attributes (primitive : ProcedurePrimitive3) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive3::Boolean (primitive) =>
			return boolean_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Arithmetic (primitive) =>
			return arithmetic_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Bitwise (primitive) =>
			return bitwise_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Comparison (primitive) =>
			return comparison_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Functions (primitive) =>
			return functions_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Runtime (primitive) =>
			return runtime_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Port (primitive) =>
			return port_primitive_3_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_4_attributes (primitive : ProcedurePrimitive4) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive4::Boolean (primitive) =>
			return boolean_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Arithmetic (primitive) =>
			return arithmetic_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Bitwise (primitive) =>
			return bitwise_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Comparison (primitive) =>
			return comparison_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Functions (primitive) =>
			return functions_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Runtime (primitive) =>
			return runtime_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Port (primitive) =>
			return port_primitive_4_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_5_attributes (primitive : ProcedurePrimitive5) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive5::Boolean (primitive) =>
			return boolean_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Arithmetic (primitive) =>
			return arithmetic_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Bitwise (primitive) =>
			return bitwise_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Comparison (primitive) =>
			return comparison_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::List (primitive) =>
			return list_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Array (primitive) =>
			return array_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Bytes (primitive) =>
			return bytes_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::String (primitive) =>
			return string_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Functions (primitive) =>
			return functions_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Runtime (primitive) =>
			return runtime_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Port (primitive) =>
			return port_primitive_5_attributes (primitive),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_n_attributes (primitive : ProcedurePrimitiveN) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Comparison (primitive) =>
			return comparison_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Array (primitive) =>
			return array_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Bytes (primitive) =>
			return bytes_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::String (primitive) =>
			return string_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Functions (primitive) =>
			return functions_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Runtime (primitive) =>
			return runtime_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Port (primitive) =>
			return port_primitive_n_attributes (primitive),
		
	}
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn procedure_primitive_g_attributes (primitive : ProcedurePrimitive) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive::Primitive0 (primitive) =>
			return procedure_primitive_0_attributes (primitive),
		
		ProcedurePrimitive::Primitive1 (primitive) =>
			return procedure_primitive_1_attributes (primitive),
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			return procedure_primitive_2_attributes (primitive),
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			return procedure_primitive_3_attributes (primitive),
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			return procedure_primitive_4_attributes (primitive),
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			return procedure_primitive_5_attributes (primitive),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_attributes (primitive),
		
		ProcedurePrimitive::PrimitiveV (_) =>
			return None,
		
		ProcedurePrimitive::Unimplemented =>
			return None,
		
		ProcedurePrimitive::Unsupported =>
			return None,
		
		ProcedurePrimitive::Reserved =>
			return None,
		
	}
}

