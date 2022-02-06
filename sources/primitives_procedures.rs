

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_arithmetic::exports::*;
use super::primitives_bitwise::exports::*;
use super::primitives_boolean::exports::*;
use super::primitives_functions::exports::*;
use super::primitives_lists::exports::*;
use super::primitives_runtime::exports::*;
use super::primitives_types::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
use super::primitives_comparisons::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::primitives_strings::exports::*;

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
use super::primitives_bytes::exports::*;

#[ cfg ( feature = "vonuvoli_values_array" ) ]
use super::primitives_arrays::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
use super::primitives_records::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
use super::primitives_ports::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
use super::primitives_filesystem::exports::*;

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
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::ProcedureArity;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::ProcedureAttributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
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
	pub use super::procedure_primitive_v_alternatives;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_n_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::procedure_primitive_g_attributes;
	
	pub use super::procedure_primitive_0_variants;
	pub use super::procedure_primitive_1_variants;
	pub use super::procedure_primitive_2_variants;
	pub use super::procedure_primitive_3_variants;
	pub use super::procedure_primitive_4_variants;
	pub use super::procedure_primitive_5_variants;
	pub use super::procedure_primitive_n_variants;
	pub use super::procedure_primitive_v_variants;
	pub use super::procedure_primitive_variants;
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
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


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitiveClass {
	
	Type,
	
	Boolean,
	Arithmetic,
	Bitwise,
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison,
	
	List,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String,
	
	Functions,
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record,
	Runtime,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port,
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem,
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive0 {
	
	Type ( TypePrimitive0 ),
	TypeNegated ( TypePrimitive0 ),
	
	Boolean ( BooleanPrimitive0 ),
	Arithmetic ( ArithmeticPrimitive0 ),
	Bitwise ( BitwisePrimitive0 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive0 ),
	
	List ( ListPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive0 ),
	
	Functions ( FunctionsPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive0 ),
	Runtime ( RuntimePrimitive0 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive0 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive0 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive1 {
	
	Type ( TypePrimitive1 ),
	TypeNegated ( TypePrimitive1 ),
	
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive1 ),
	
	List ( ListPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive1 ),
	
	Functions ( FunctionsPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive1 ),
	Runtime ( RuntimePrimitive1 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive1 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive1 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive2 {
	
	Type ( TypePrimitive2 ),
	TypeNegated ( TypePrimitive2 ),
	
	Boolean ( BooleanPrimitive2 ),
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive2 ),
	
	List ( ListPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive2 ),
	
	Functions ( FunctionsPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive2 ),
	Runtime ( RuntimePrimitive2 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive2 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive2 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive3 {
	
	Type ( TypePrimitive3 ),
	TypeNegated ( TypePrimitive3 ),
	
	Boolean ( BooleanPrimitive3 ),
	Arithmetic ( ArithmeticPrimitive3 ),
	Bitwise ( BitwisePrimitive3 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive3 ),
	
	List ( ListPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive3 ),
	
	Functions ( FunctionsPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive3 ),
	Runtime ( RuntimePrimitive3 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive3 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive3 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive4 {
	
	Type ( TypePrimitive4 ),
	TypeNegated ( TypePrimitive4 ),
	
	Boolean ( BooleanPrimitive4 ),
	Arithmetic ( ArithmeticPrimitive4 ),
	Bitwise ( BitwisePrimitive4 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive4 ),
	
	List ( ListPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive4 ),
	
	Functions ( FunctionsPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive4 ),
	Runtime ( RuntimePrimitive4 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive4 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive4 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitive5 {
	
	Type ( TypePrimitive5 ),
	TypeNegated ( TypePrimitive5 ),
	
	Boolean ( BooleanPrimitive5 ),
	Arithmetic ( ArithmeticPrimitive5 ),
	Bitwise ( BitwisePrimitive5 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitive5 ),
	
	List ( ListPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitive5 ),
	
	Functions ( FunctionsPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitive5 ),
	Runtime ( RuntimePrimitive5 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitive5 ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitive5 ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitiveN {
	
	Type ( TypePrimitiveN ),
	TypeNegated ( TypePrimitiveN ),
	
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitiveN ),
	
	List ( ListPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitiveN ),
	
	Functions ( FunctionsPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitiveN ),
	Runtime ( RuntimePrimitiveN ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitiveN ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitiveN ),
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedurePrimitiveV {
	
	Type ( TypePrimitiveV ),
	TypeNegated ( TypePrimitiveV ),
	
	Boolean ( BooleanPrimitiveV ),
	Arithmetic ( ArithmeticPrimitiveV ),
	Bitwise ( BitwisePrimitiveV ),
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Comparison ( ComparisonPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	ComparisonNegated ( ComparisonPrimitiveV ),
	
	List ( ListPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array ( ArrayPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes ( BytesPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String ( StringPrimitiveV ),
	
	Functions ( FunctionsPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record ( RecordPrimitiveV ),
	Runtime ( RuntimePrimitiveV ),
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( PortPrimitiveV ),
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	FileSystem ( FileSystemPrimitiveV ),
	
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct ProcedureAttributes {
	
	pub deterministic : bool,
	pub arity : ProcedureArity,
	pub output : ProcedureOutputAttributes,
	
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
#[ allow (variant_size_differences) ] // OK
pub enum ProcedureArity {
	Undefined,
	Exact ( usize ),
	Bounded ( Option<usize>, Option<usize> ),
	Unbounded,
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ProcedureOutputAttributes {
	Undefined,
	Constant,
}




pub fn procedure_primitive_0_evaluate (primitive : ProcedurePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive0::Type (primitive) =>
			return type_primitive_0_evaluate (primitive, false, evaluator),
		
		ProcedurePrimitive0::TypeNegated (primitive) =>
			return type_primitive_0_evaluate (primitive, true, evaluator),
		
		ProcedurePrimitive0::Boolean (primitive) =>
			return boolean_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Arithmetic (primitive) =>
			return arithmetic_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Bitwise (primitive) =>
			return bitwise_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive0::Comparison (primitive) =>
			return comparison_primitive_0_evaluate (primitive, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive0::ComparisonNegated (primitive) =>
			return comparison_primitive_0_evaluate (primitive, true, evaluator),
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Functions (primitive) =>
			return functions_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive0::Record (primitive) =>
			return record_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive0::Runtime (primitive) =>
			return runtime_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive0::Port (primitive) =>
			return port_primitive_0_evaluate (primitive, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive0::FileSystem (primitive) =>
			return filesystem_primitive_0_evaluate (primitive, evaluator),
		
	}
}




pub fn procedure_primitive_1_evaluate (primitive : ProcedurePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_evaluate (primitive, input_1, false, evaluator),
		
		ProcedurePrimitive1::TypeNegated (primitive) =>
			return type_primitive_1_evaluate (primitive, input_1, true, evaluator),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive1::Comparison (primitive) =>
			return comparison_primitive_1_evaluate (primitive, input_1, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive1::ComparisonNegated (primitive) =>
			return comparison_primitive_1_evaluate (primitive, input_1, true, evaluator),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive1::Array (primitive) =>
			return array_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive1::Bytes (primitive) =>
			return bytes_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive1::String (primitive) =>
			return string_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Functions (primitive) =>
			return functions_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive1::Record (primitive) =>
			return record_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive1::Runtime (primitive) =>
			return runtime_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive1::Port (primitive) =>
			return port_primitive_1_evaluate (primitive, input_1, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive1::FileSystem (primitive) =>
			return filesystem_primitive_1_evaluate (primitive, input_1, evaluator),
		
	}
}




pub fn procedure_primitive_2_evaluate (primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive2::Type (primitive) =>
			return type_primitive_2_evaluate (primitive, input_1, input_2, false, evaluator),
		
		ProcedurePrimitive2::TypeNegated (primitive) =>
			return type_primitive_2_evaluate (primitive, input_1, input_2, true, evaluator),
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive2::Comparison (primitive) =>
			return comparison_primitive_2_evaluate (primitive, input_1, input_2, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive2::ComparisonNegated (primitive) =>
			return comparison_primitive_2_evaluate (primitive, input_1, input_2, true, evaluator),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive2::Array (primitive) =>
			return array_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive2::Bytes (primitive) =>
			return bytes_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive2::String (primitive) =>
			return string_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Functions (primitive) =>
			return functions_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive2::Record (primitive) =>
			return record_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive2::Runtime (primitive) =>
			return runtime_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive2::Port (primitive) =>
			return port_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive2::FileSystem (primitive) =>
			return filesystem_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
	}
}




pub fn procedure_primitive_3_evaluate (primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive3::Type (primitive) =>
			return type_primitive_3_evaluate (primitive, input_1, input_2, input_3, false, evaluator),
		
		ProcedurePrimitive3::TypeNegated (primitive) =>
			return type_primitive_3_evaluate (primitive, input_1, input_2, input_3, true, evaluator),
		
		ProcedurePrimitive3::Boolean (primitive) =>
			return boolean_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Arithmetic (primitive) =>
			return arithmetic_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Bitwise (primitive) =>
			return bitwise_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive3::Comparison (primitive) =>
			return comparison_primitive_3_evaluate (primitive, input_1, input_2, input_3, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive3::ComparisonNegated (primitive) =>
			return comparison_primitive_3_evaluate (primitive, input_1, input_2, input_3, true, evaluator),
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Functions (primitive) =>
			return functions_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive3::Record (primitive) =>
			return record_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive3::Runtime (primitive) =>
			return runtime_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive3::Port (primitive) =>
			return port_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive3::FileSystem (primitive) =>
			return filesystem_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
	}
}




pub fn procedure_primitive_4_evaluate (primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive4::Type (primitive) =>
			return type_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, false, evaluator),
		
		ProcedurePrimitive4::TypeNegated (primitive) =>
			return type_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, true, evaluator),
		
		ProcedurePrimitive4::Boolean (primitive) =>
			return boolean_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Arithmetic (primitive) =>
			return arithmetic_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Bitwise (primitive) =>
			return bitwise_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive4::Comparison (primitive) =>
			return comparison_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive4::ComparisonNegated (primitive) =>
			return comparison_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, true, evaluator),
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Functions (primitive) =>
			return functions_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive4::Record (primitive) =>
			return record_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive4::Runtime (primitive) =>
			return runtime_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive4::Port (primitive) =>
			return port_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive4::FileSystem (primitive) =>
			return filesystem_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
	}
}




pub fn procedure_primitive_5_evaluate (primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive5::Type (primitive) =>
			return type_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, false, evaluator),
		
		ProcedurePrimitive5::TypeNegated (primitive) =>
			return type_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, true, evaluator),
		
		ProcedurePrimitive5::Boolean (primitive) =>
			return boolean_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Arithmetic (primitive) =>
			return arithmetic_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Bitwise (primitive) =>
			return bitwise_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive5::Comparison (primitive) =>
			return comparison_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive5::ComparisonNegated (primitive) =>
			return comparison_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, true, evaluator),
		
		ProcedurePrimitive5::List (primitive) =>
			return list_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive5::Array (primitive) =>
			return array_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive5::Bytes (primitive) =>
			return bytes_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive5::String (primitive) =>
			return string_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Functions (primitive) =>
			return functions_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive5::Record (primitive) =>
			return record_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive5::Runtime (primitive) =>
			return runtime_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive5::Port (primitive) =>
			return port_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive5::FileSystem (primitive) =>
			return filesystem_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
	}
}




pub fn procedure_primitive_n_evaluate (primitive : ProcedurePrimitiveN, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitiveN::Type (primitive) =>
			return type_primitive_n_evaluate (primitive, inputs, false, evaluator),
		
		ProcedurePrimitiveN::TypeNegated (primitive) =>
			return type_primitive_n_evaluate (primitive, inputs, true, evaluator),
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveN::Comparison (primitive) =>
			return comparison_primitive_n_evaluate (primitive, inputs, false, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveN::ComparisonNegated (primitive) =>
			return comparison_primitive_n_evaluate (primitive, inputs, true, evaluator),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveN::Array (primitive) =>
			return array_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveN::Bytes (primitive) =>
			return bytes_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitiveN::String (primitive) =>
			return string_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Functions (primitive) =>
			return functions_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveN::Record (primitive) =>
			return record_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitiveN::Runtime (primitive) =>
			return runtime_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveN::Port (primitive) =>
			return port_primitive_n_evaluate (primitive, inputs, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveN::FileSystem (primitive) =>
			return filesystem_primitive_n_evaluate (primitive, inputs, evaluator),
		
	}
}




pub fn procedure_primitive_v_evaluate_0 (primitive : ProcedurePrimitiveV, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
		return procedure_primitive_0_evaluate (primitive, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		const INPUTS_EMPTY : &[&Value] = &[];
		return procedure_primitive_n_evaluate (primitive, INPUTS_EMPTY, evaluator);
	}
	
	fail! (0x45c55a28);
}




pub fn procedure_primitive_v_evaluate_1 (primitive : ProcedurePrimitiveV, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1], evaluator);
	}
	
	fail! (0xef55f95e);
}




pub fn procedure_primitive_v_evaluate_2 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2], evaluator);
	}
	
	fail! (0x3f354a4d);
}




pub fn procedure_primitive_v_evaluate_3 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3], evaluator);
	}
	
	fail! (0x315dd3f2);
}




pub fn procedure_primitive_v_evaluate_4 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4], evaluator);
	}
	
	fail! (0x7ff0968c);
}




pub fn procedure_primitive_v_evaluate_5 (primitive : ProcedurePrimitiveV, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator);
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4, input_5], evaluator);
	}
	
	fail! (0x93a6cbdc);
}




pub fn procedure_primitive_v_evaluate_n (primitive : ProcedurePrimitiveV, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	
	match inputs_count {
		
		0 =>
			if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
				return procedure_primitive_0_evaluate (primitive, evaluator);
			},
		
		1 =>
			if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
				return procedure_primitive_1_evaluate (primitive, inputs[0].as_ref (), evaluator);
			},
		
		2 =>
			if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
				return procedure_primitive_2_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), evaluator);
			},
		
		3 =>
			if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
				return procedure_primitive_3_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), evaluator);
			},
		
		4 =>
			if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
				return procedure_primitive_4_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), evaluator);
			},
		
		5 =>
			if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
				return procedure_primitive_5_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), inputs[4].as_ref (), evaluator);
			},
		
		_ =>
			(),
	}
	
	if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluator);
	}
	
	fail! (0x270e1433);
}




pub fn procedure_primitive_g_evaluate_0 (primitive : ProcedurePrimitive, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	const INPUTS_EMPTY : &[&Value] = &[];
	match primitive {
		
		ProcedurePrimitive::Primitive0 (primitive) =>
			return procedure_primitive_0_evaluate (primitive, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, INPUTS_EMPTY, evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_0 (primitive, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x635e3f45, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x588a2e8f, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0x649839df, OK),
		
		_ =>
			fail! (0x38623a76),
	}
}




pub fn procedure_primitive_g_evaluate_1 (primitive : ProcedurePrimitive, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive1 (primitive) =>
			return procedure_primitive_1_evaluate (primitive, input_1, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_1 (primitive, input_1, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x6c063079, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xc689378a, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0xfed99283, OK),
		
		_ =>
			fail! (0x92f0f0ce),
	}
}




pub fn procedure_primitive_g_evaluate_2 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_2 (primitive, input_1, input_2, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0xd79d710f, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xfae8451f, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0x0b04b93e, OK),
		
		_ =>
			fail! (0xdaa4c8f5),
	}
}




pub fn procedure_primitive_g_evaluate_3 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_3 (primitive, input_1, input_2, input_3, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x299b0807, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x0d3c6416, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0x310b03d4, OK),
		
		_ =>
			fail! (0x99473830),
	}
}




pub fn procedure_primitive_g_evaluate_4 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_4 (primitive, input_1, input_2, input_3, input_4, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x41ea7797, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0xbcbf1bcd, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0x37319aac, OK),
		
		_ =>
			fail! (0x723b728c),
	}
}




pub fn procedure_primitive_g_evaluate_5 (primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, &[input_1, input_2, input_3, input_4, input_5], evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_5 (primitive, input_1, input_2, input_3, input_4, input_5, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x97c9abd0, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x2e569e14, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0xc8a6c70d, OK),
		
		_ =>
			fail! (0xa4f4a6c6),
	}
}




pub fn procedure_primitive_g_evaluate_n (primitive : ProcedurePrimitive, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
				return procedure_primitive_1_evaluate (primitive, inputs[0].as_ref (), evaluator);
			} else {
				fail! (0x5bc94cf2)
			},
		
		ProcedurePrimitive::Primitive2 (primitive) =>
			if inputs_count == 2 {
				return procedure_primitive_2_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), evaluator);
			} else {
				fail! (0xb1c56ed3)
			},
		
		ProcedurePrimitive::Primitive3 (primitive) =>
			if inputs_count == 3 {
				return procedure_primitive_3_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), evaluator);
			} else {
				fail! (0x990f006e)
			},
		
		ProcedurePrimitive::Primitive4 (primitive) =>
			if inputs_count == 4 {
				return procedure_primitive_4_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), evaluator);
			} else {
				fail! (0x62f33d3e)
			},
		
		ProcedurePrimitive::Primitive5 (primitive) =>
			if inputs_count == 5 {
				return procedure_primitive_5_evaluate (primitive, inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), inputs[4].as_ref (), evaluator);
			} else {
				fail! (0x2e877045)
			},
		
		ProcedurePrimitive::PrimitiveN (primitive) =>
			return procedure_primitive_n_evaluate (primitive, inputs, evaluator),
		
		ProcedurePrimitive::PrimitiveV (primitive) =>
			return procedure_primitive_v_evaluate_n (primitive, inputs, evaluator),
		
		ProcedurePrimitive::Unimplemented =>
			fail_unimplemented! (0x10d3710f, OK),
		
		ProcedurePrimitive::Unsupported =>
			fail_unimplemented! (0x8baac30b, OK),
		
		ProcedurePrimitive::Reserved =>
			fail_unimplemented! (0xb687a39c, OK),
		
	}
}




pub fn procedure_primitive_v_alternative_0 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive0>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_0 (primitive) {
				Some (ProcedurePrimitive0::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_1 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive1>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_1 (primitive) {
				Some (ProcedurePrimitive1::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_2 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive2>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_2 (primitive) {
				Some (ProcedurePrimitive2::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_3 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive3>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_3 (primitive) {
				Some (ProcedurePrimitive3::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_4 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive4>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_4 (primitive) {
				Some (ProcedurePrimitive4::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_5 (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitive5>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_5 (primitive) {
				Some (ProcedurePrimitive5::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternative_n (primitive : ProcedurePrimitiveV) -> (Option<ProcedurePrimitiveN>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Type (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			if let Some (primitive) = type_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::TypeNegated (primitive))
			} else {
				None
			},
		
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
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Comparison (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			if let Some (primitive) = comparison_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::ComparisonNegated (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::List (primitive) =>
			if let Some (primitive) = list_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::List (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			if let Some (primitive) = array_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Array (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			if let Some (primitive) = bytes_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Bytes (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			if let Some (primitive) = record_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Record (primitive))
			} else {
				None
			},
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			if let Some (primitive) = runtime_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Runtime (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			if let Some (primitive) = port_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::Port (primitive))
			} else {
				None
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			if let Some (primitive) = filesystem_primitive_v_alternative_n (primitive) {
				Some (ProcedurePrimitiveN::FileSystem (primitive))
			} else {
				None
			},
		
	}
}




pub fn procedure_primitive_v_alternatives (primitive : ProcedurePrimitiveV) -> (StdBox<[ProcedurePrimitive]>) {
	match primitive {
		
		ProcedurePrimitiveV::Type (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::TypeNegated (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::Boolean (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::Arithmetic (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::Bitwise (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::Comparison (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveV::ComparisonNegated (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::List (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveV::Array (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveV::Bytes (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitiveV::String (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::Functions (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveV::Record (primitive) =>
			primitive.alternatives_all_into (),
		
		ProcedurePrimitiveV::Runtime (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveV::Port (primitive) =>
			primitive.alternatives_all_into (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveV::FileSystem (primitive) =>
			primitive.alternatives_all_into (),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_0_attributes (primitive : ProcedurePrimitive0) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive0::Type (primitive) =>
			return type_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::TypeNegated (primitive) =>
			return type_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Boolean (primitive) =>
			return boolean_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Arithmetic (primitive) =>
			return arithmetic_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Bitwise (primitive) =>
			return bitwise_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive0::Comparison (primitive) =>
			return comparison_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive0::ComparisonNegated (primitive) =>
			return comparison_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::List (primitive) =>
			return list_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive0::Array (primitive) =>
			return array_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive0::Bytes (primitive) =>
			return bytes_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive0::String (primitive) =>
			return string_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Functions (primitive) =>
			return functions_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive0::Record (primitive) =>
			return record_primitive_0_attributes (primitive),
		
		ProcedurePrimitive0::Runtime (primitive) =>
			return runtime_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive0::Port (primitive) =>
			return port_primitive_0_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive0::FileSystem (primitive) =>
			return filesystem_primitive_0_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_1_attributes (primitive : ProcedurePrimitive1) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive1::Type (primitive) =>
			return type_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::TypeNegated (primitive) =>
			return type_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Boolean (primitive) =>
			return boolean_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Arithmetic (primitive) =>
			return arithmetic_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Bitwise (primitive) =>
			return bitwise_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive1::Comparison (primitive) =>
			return comparison_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive1::ComparisonNegated (primitive) =>
			return comparison_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::List (primitive) =>
			return list_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive1::Array (primitive) =>
			return array_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive1::Bytes (primitive) =>
			return bytes_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive1::String (primitive) =>
			return string_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Functions (primitive) =>
			return functions_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive1::Record (primitive) =>
			return record_primitive_1_attributes (primitive),
		
		ProcedurePrimitive1::Runtime (primitive) =>
			return runtime_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive1::Port (primitive) =>
			return port_primitive_1_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive1::FileSystem (primitive) =>
			return filesystem_primitive_1_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_2_attributes (primitive : ProcedurePrimitive2) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive2::Type (primitive) =>
			return type_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::TypeNegated (primitive) =>
			return type_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Boolean (primitive) =>
			return boolean_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Arithmetic (primitive) =>
			return arithmetic_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Bitwise (primitive) =>
			return bitwise_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive2::Comparison (primitive) =>
			return comparison_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive2::ComparisonNegated (primitive) =>
			return comparison_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::List (primitive) =>
			return list_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive2::Array (primitive) =>
			return array_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive2::Bytes (primitive) =>
			return bytes_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive2::String (primitive) =>
			return string_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Functions (primitive) =>
			return functions_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive2::Record (primitive) =>
			return record_primitive_2_attributes (primitive),
		
		ProcedurePrimitive2::Runtime (primitive) =>
			return runtime_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive2::Port (primitive) =>
			return port_primitive_2_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive2::FileSystem (primitive) =>
			return filesystem_primitive_2_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_3_attributes (primitive : ProcedurePrimitive3) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive3::Type (primitive) =>
			return type_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::TypeNegated (primitive) =>
			return type_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Boolean (primitive) =>
			return boolean_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Arithmetic (primitive) =>
			return arithmetic_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Bitwise (primitive) =>
			return bitwise_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive3::Comparison (primitive) =>
			return comparison_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive3::ComparisonNegated (primitive) =>
			return comparison_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::List (primitive) =>
			return list_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive3::Array (primitive) =>
			return array_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive3::Bytes (primitive) =>
			return bytes_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive3::String (primitive) =>
			return string_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Functions (primitive) =>
			return functions_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive3::Record (primitive) =>
			return record_primitive_3_attributes (primitive),
		
		ProcedurePrimitive3::Runtime (primitive) =>
			return runtime_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive3::Port (primitive) =>
			return port_primitive_3_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive3::FileSystem (primitive) =>
			return filesystem_primitive_3_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_4_attributes (primitive : ProcedurePrimitive4) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive4::Type (primitive) =>
			return type_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::TypeNegated (primitive) =>
			return type_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Boolean (primitive) =>
			return boolean_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Arithmetic (primitive) =>
			return arithmetic_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Bitwise (primitive) =>
			return bitwise_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive4::Comparison (primitive) =>
			return comparison_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive4::ComparisonNegated (primitive) =>
			return comparison_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::List (primitive) =>
			return list_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive4::Array (primitive) =>
			return array_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive4::Bytes (primitive) =>
			return bytes_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive4::String (primitive) =>
			return string_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Functions (primitive) =>
			return functions_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive4::Record (primitive) =>
			return record_primitive_4_attributes (primitive),
		
		ProcedurePrimitive4::Runtime (primitive) =>
			return runtime_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive4::Port (primitive) =>
			return port_primitive_4_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive4::FileSystem (primitive) =>
			return filesystem_primitive_4_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_5_attributes (primitive : ProcedurePrimitive5) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitive5::Type (primitive) =>
			return type_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::TypeNegated (primitive) =>
			return type_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Boolean (primitive) =>
			return boolean_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Arithmetic (primitive) =>
			return arithmetic_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Bitwise (primitive) =>
			return bitwise_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive5::Comparison (primitive) =>
			return comparison_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitive5::ComparisonNegated (primitive) =>
			return comparison_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::List (primitive) =>
			return list_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitive5::Array (primitive) =>
			return array_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitive5::Bytes (primitive) =>
			return bytes_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitive5::String (primitive) =>
			return string_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Functions (primitive) =>
			return functions_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitive5::Record (primitive) =>
			return record_primitive_5_attributes (primitive),
		
		ProcedurePrimitive5::Runtime (primitive) =>
			return runtime_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitive5::Port (primitive) =>
			return port_primitive_5_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitive5::FileSystem (primitive) =>
			return filesystem_primitive_5_attributes (primitive),
		
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn procedure_primitive_n_attributes (primitive : ProcedurePrimitiveN) -> (Option<ProcedureAttributes>) {
	match primitive {
		
		ProcedurePrimitiveN::Type (primitive) =>
			return type_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::TypeNegated (primitive) =>
			return type_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Arithmetic (primitive) =>
			return arithmetic_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Bitwise (primitive) =>
			return bitwise_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveN::Comparison (primitive) =>
			return comparison_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
		ProcedurePrimitiveN::ComparisonNegated (primitive) =>
			return comparison_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::List (primitive) =>
			return list_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ProcedurePrimitiveN::Array (primitive) =>
			return array_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ProcedurePrimitiveN::Bytes (primitive) =>
			return bytes_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ProcedurePrimitiveN::String (primitive) =>
			return string_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Functions (primitive) =>
			return functions_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ProcedurePrimitiveN::Record (primitive) =>
			return record_primitive_n_attributes (primitive),
		
		ProcedurePrimitiveN::Runtime (primitive) =>
			return runtime_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ProcedurePrimitiveN::Port (primitive) =>
			return port_primitive_n_attributes (primitive),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ProcedurePrimitiveN::FileSystem (primitive) =>
			return filesystem_primitive_n_attributes (primitive),
		
	}
}


#[ cfg ( feature = "vonuvoli_optimizer" ) ]
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




pub fn procedure_primitive_0_variants <T : StdFrom<ProcedurePrimitive0>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Type (*variant) .into ());
	}
	for variant in TypePrimitive0::variants () {
		variants.push (ProcedurePrimitive0::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive0::variants () {
		variants.push (ProcedurePrimitive0::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_1_variants <T : StdFrom<ProcedurePrimitive1>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Type (*variant) .into ());
	}
	for variant in TypePrimitive1::variants () {
		variants.push (ProcedurePrimitive1::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive1::variants () {
		variants.push (ProcedurePrimitive1::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_2_variants <T : StdFrom<ProcedurePrimitive2>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Type (*variant) .into ());
	}
	for variant in TypePrimitive2::variants () {
		variants.push (ProcedurePrimitive2::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive2::variants () {
		variants.push (ProcedurePrimitive2::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_3_variants <T : StdFrom<ProcedurePrimitive3>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Type (*variant) .into ());
	}
	for variant in TypePrimitive3::variants () {
		variants.push (ProcedurePrimitive3::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive3::variants () {
		variants.push (ProcedurePrimitive3::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_4_variants <T : StdFrom<ProcedurePrimitive4>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Type (*variant) .into ());
	}
	for variant in TypePrimitive4::variants () {
		variants.push (ProcedurePrimitive4::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive4::variants () {
		variants.push (ProcedurePrimitive4::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_5_variants <T : StdFrom<ProcedurePrimitive5>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Type (*variant) .into ());
	}
	for variant in TypePrimitive5::variants () {
		variants.push (ProcedurePrimitive5::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Record (*variant) .into ());
	}
	for variant in RuntimePrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitive5::variants () {
		variants.push (ProcedurePrimitive5::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_n_variants <T : StdFrom<ProcedurePrimitiveN>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Type (*variant) .into ());
	}
	for variant in TypePrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Record (*variant) .into ());
	}
	for variant in RuntimePrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitiveN::variants () {
		variants.push (ProcedurePrimitiveN::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_v_variants <T : StdFrom<ProcedurePrimitiveV>> () -> (StdBox<[T]>) {
	let mut variants = StdVec::new ();
	
	for variant in TypePrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Type (*variant) .into ());
	}
	for variant in TypePrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::TypeNegated (*variant) .into ());
	}
	
	for variant in BooleanPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Boolean (*variant) .into ());
	}
	for variant in ArithmeticPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Arithmetic (*variant) .into ());
	}
	for variant in BitwisePrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Bitwise (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Comparison (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	for variant in ComparisonPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::ComparisonNegated (*variant) .into ());
	}
	
	for variant in ListPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::List (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	for variant in ArrayPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Array (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	for variant in BytesPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Bytes (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	for variant in StringPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::String (*variant) .into ());
	}
	
	for variant in FunctionsPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Functions (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	for variant in RecordPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Record (*variant) .into ());
	}
	for variant in RuntimePrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Runtime (*variant) .into ());
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	for variant in PortPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::Port (*variant) .into ());
	}
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	for variant in FileSystemPrimitiveV::variants () {
		variants.push (ProcedurePrimitiveV::FileSystem (*variant) .into ());
	}
	
	variants.into_boxed_slice ()
}


pub fn procedure_primitive_variants <T> () -> (StdBox<[T]>)
		where T : StdFrom<ProcedurePrimitive0> + StdFrom<ProcedurePrimitive1> + StdFrom<ProcedurePrimitive2> + StdFrom<ProcedurePrimitive3> + StdFrom<ProcedurePrimitive4> + StdFrom<ProcedurePrimitive5> + StdFrom<ProcedurePrimitiveN> + StdFrom<ProcedurePrimitiveV>
{
	let mut variants = StdVec::new ();
	
	for variant in StdVec::from (procedure_primitive_0_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_1_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_2_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_3_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_4_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_5_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_n_variants ()) {
		variants.push (variant);
	}
	for variant in StdVec::from (procedure_primitive_v_variants ()) {
		variants.push (variant);
	}
	
	variants.into_boxed_slice ()
}




impl ProcedurePrimitive {
	
	pub fn is_self (&self, other : &ProcedurePrimitive) -> (bool) {
		*self == *other
	}
	
	pub fn class (self) -> (ProcedurePrimitiveClass) {
		match self {
			
			ProcedurePrimitive::Primitive0 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::Primitive1 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::Primitive2 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::Primitive3 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::Primitive4 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::Primitive5 (primitive) =>
				primitive.class (),
			ProcedurePrimitive::PrimitiveN (primitive) =>
				primitive.class (),
			ProcedurePrimitive::PrimitiveV (primitive) =>
				primitive.class (),
			
			ProcedurePrimitive::Unimplemented =>
				ProcedurePrimitiveClass::Unimplemented,
			ProcedurePrimitive::Unsupported =>
				ProcedurePrimitiveClass::Unsupported,
			ProcedurePrimitive::Reserved =>
				ProcedurePrimitiveClass::Reserved,
			
		}
	}
	
	pub fn identifier (self) -> (&'static str) {
		match self {
			
			ProcedurePrimitive::Primitive0 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::Primitive1 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::Primitive2 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::Primitive3 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::Primitive4 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::Primitive5 (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::PrimitiveN (primitive) =>
				primitive.identifier (),
			ProcedurePrimitive::PrimitiveV (primitive) =>
				primitive.identifier (),
			
			ProcedurePrimitive::Unimplemented =>
				"ProcedurePrimitive::Unimplemented",
			ProcedurePrimitive::Unsupported =>
				"ProcedurePrimitive::Unsupported",
			ProcedurePrimitive::Reserved =>
				"ProcedurePrimitive::Reserved",
			
		}
	}
	
	pub fn is_negated (self) -> (Option<bool>) {
		match self {
			
			ProcedurePrimitive::Primitive0 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::Primitive1 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::Primitive2 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::Primitive3 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::Primitive4 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::Primitive5 (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::PrimitiveN (primitive) =>
				primitive.is_negated (),
			ProcedurePrimitive::PrimitiveV (primitive) =>
				primitive.is_negated (),
			
			ProcedurePrimitive::Unimplemented =>
				None,
			ProcedurePrimitive::Unsupported =>
				None,
			ProcedurePrimitive::Reserved =>
				None,
			
		}
	}
}




macro_rules! impl_procedure_primitive_x {
	
	( $enum : ident ) => (
		
		impl $enum {
			
			pub fn is_self (&self, other : &$enum) -> (bool) {
				*self == *other
			}
			
			pub fn class (&self) -> (ProcedurePrimitiveClass) {
				match *self {
					
					$enum::Type (_primitive) =>
						ProcedurePrimitiveClass::Type,
					
					$enum::TypeNegated (_primitive) =>
						ProcedurePrimitiveClass::Type,
					
					$enum::Boolean (_primitive) =>
						ProcedurePrimitiveClass::Boolean,
					
					$enum::Arithmetic (_primitive) =>
						ProcedurePrimitiveClass::Arithmetic,
					
					$enum::Bitwise (_primitive) =>
						ProcedurePrimitiveClass::Bitwise,
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::Comparison (_primitive) =>
						ProcedurePrimitiveClass::Comparison,
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::ComparisonNegated (_primitive) =>
						ProcedurePrimitiveClass::Comparison,
					
					$enum::List (_primitive) =>
						ProcedurePrimitiveClass::List,
					
					#[ cfg ( feature = "vonuvoli_values_array" ) ]
					$enum::Array (_primitive) =>
						ProcedurePrimitiveClass::Array,
					
					#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
					$enum::Bytes (_primitive) =>
						ProcedurePrimitiveClass::Bytes,
					
					#[ cfg ( feature = "vonuvoli_values_string" ) ]
					$enum::String (_primitive) =>
						ProcedurePrimitiveClass::String,
					
					$enum::Functions (_primitive) =>
						ProcedurePrimitiveClass::Functions,
					
					#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
					$enum::Record (_primitive) =>
						ProcedurePrimitiveClass::Record,
					
					$enum::Runtime (_primitive) =>
						ProcedurePrimitiveClass::Runtime,
					
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					$enum::Port (_primitive) =>
						ProcedurePrimitiveClass::Port,
					
					#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
					$enum::FileSystem (_primitive) =>
						ProcedurePrimitiveClass::FileSystem,
					
				}
			}
			
			pub fn identifier (&self) -> (&'static str) {
				match *self {
					
					$enum::Type (primitive) =>
						primitive.identifier (),
					
					$enum::TypeNegated (primitive) =>
						primitive.identifier (),
					
					$enum::Boolean (primitive) =>
						primitive.identifier (),
					
					$enum::Arithmetic (primitive) =>
						primitive.identifier (),
					
					$enum::Bitwise (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::Comparison (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::ComparisonNegated (primitive) =>
						primitive.identifier (),
					
					$enum::List (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_values_array" ) ]
					$enum::Array (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
					$enum::Bytes (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_values_string" ) ]
					$enum::String (primitive) =>
						primitive.identifier (),
					
					$enum::Functions (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
					$enum::Record (primitive) =>
						primitive.identifier (),
					
					$enum::Runtime (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					$enum::Port (primitive) =>
						primitive.identifier (),
					
					#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
					$enum::FileSystem (primitive) =>
						primitive.identifier (),
					
				}
			}
			
			pub fn is_negated (&self) -> (Option<bool>) {
				match *self {
					
					$enum::Type (_primitive) =>
						Some (false),
					
					$enum::TypeNegated (_primitive) =>
						Some (true),
					
					$enum::Boolean (_primitive) =>
						None,
					
					$enum::Arithmetic (_primitive) =>
						None,
					
					$enum::Bitwise (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::Comparison (_primitive) =>
						Some (false),
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					$enum::ComparisonNegated (_primitive) =>
						Some (true),
					
					$enum::List (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_values_array" ) ]
					$enum::Array (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
					$enum::Bytes (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_values_string" ) ]
					$enum::String (_primitive) =>
						None,
					
					$enum::Functions (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
					$enum::Record (_primitive) =>
						None,
					
					$enum::Runtime (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					$enum::Port (_primitive) =>
						None,
					
					#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
					$enum::FileSystem (_primitive) =>
						None,
					
				}
			}
			
		}
		
	);
	
}


impl_procedure_primitive_x! (ProcedurePrimitive0);
impl_procedure_primitive_x! (ProcedurePrimitive1);
impl_procedure_primitive_x! (ProcedurePrimitive2);
impl_procedure_primitive_x! (ProcedurePrimitive3);
impl_procedure_primitive_x! (ProcedurePrimitive4);
impl_procedure_primitive_x! (ProcedurePrimitive5);
impl_procedure_primitive_x! (ProcedurePrimitiveN);
impl_procedure_primitive_x! (ProcedurePrimitiveV);




impl ProcedurePrimitiveV {
	
	pub fn alternative_0 <T : StdFrom<ProcedurePrimitive0>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_0 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_1 <T : StdFrom<ProcedurePrimitive1>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_1 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_2 <T : StdFrom<ProcedurePrimitive2>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_2 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_3 <T : StdFrom<ProcedurePrimitive3>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_3 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_4 <T : StdFrom<ProcedurePrimitive4>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_4 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_5 <T : StdFrom<ProcedurePrimitive5>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_5 (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternative_n <T : StdFrom<ProcedurePrimitiveN>> (self) -> (Option<T>) {
		if let Some (variant) = procedure_primitive_v_alternative_n (self) {
			Some (variant.into ())
		} else {
			None
		}
	}
	
	pub fn alternatives_all_into <T> (self) -> (StdBox<[T]>)
			where T : StdFrom<ProcedurePrimitive0> + StdFrom<ProcedurePrimitive1> + StdFrom<ProcedurePrimitive2> + StdFrom<ProcedurePrimitive3> + StdFrom<ProcedurePrimitive4> + StdFrom<ProcedurePrimitive5> + StdFrom<ProcedurePrimitiveN>
	{
		let mut variants = StdVec::new ();
		if let Some (variant) = self.alternative_0 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_1 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_2 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_3 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_4 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_5 () {
			variants.push (variant);
		}
		if let Some (variant) = self.alternative_n () {
			variants.push (variant);
		}
		variants.into_boxed_slice ()
	}
}

