

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::BytesPrimitive0;
	pub use super::BytesPrimitive1;
	pub use super::BytesPrimitive2;
	pub use super::BytesPrimitive3;
	pub use super::BytesPrimitive4;
	pub use super::BytesPrimitive5;
	pub use super::BytesPrimitiveN;
	pub use super::BytesPrimitiveV;
	
	pub use super::bytes_primitive_0_evaluate;
	pub use super::bytes_primitive_1_evaluate;
	pub use super::bytes_primitive_2_evaluate;
	pub use super::bytes_primitive_3_evaluate;
	pub use super::bytes_primitive_4_evaluate;
	pub use super::bytes_primitive_5_evaluate;
	pub use super::bytes_primitive_n_evaluate;
	
	pub use super::bytes_primitive_v_alternative_0;
	pub use super::bytes_primitive_v_alternative_1;
	pub use super::bytes_primitive_v_alternative_2;
	pub use super::bytes_primitive_v_alternative_3;
	pub use super::bytes_primitive_v_alternative_4;
	pub use super::bytes_primitive_v_alternative_5;
	pub use super::bytes_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bytes_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (BytesPrimitive0, (procedure, 0), {
	
	BytesBuild,
	BytesAppend,
	
});


def_primitives_enum! (BytesPrimitive1, (procedure, 1), {
	
	BytesLength,
	BytesClone,
	BytesCloneReverse,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReverse,
	
	BytesToList,
	ListToBytes,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	BytesToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayToBytes,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesToMutable,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexCompile,
	
});


def_primitives_enum! (BytesPrimitive2, (procedure, 2), {
	
	BytesAt,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesCopy,
	BytesRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeReverse,
	
	BytesRangeToList,
	ListRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	BytesRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToBytes,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatches,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchExtractFirst,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchExtractAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchExtractAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchPositionFirst,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchPositionAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchPositionAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractFirstAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractFirstAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractFirstAsArray,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractAllAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesExtractAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionFirstAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionFirstAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionFirstAsArray,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionAllAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	BytesRegexMatchCapturesPositionAllAsArray,
	
});


def_primitives_enum! (BytesPrimitive3, (procedure, 3), {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesAtSet,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeCopy,
	BytesRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeReverse,
	
	BytesRangeToList,
	ListRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	BytesRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToBytes,
	
});


def_primitives_enum! (BytesPrimitive4, (procedure, 4), {
	
	BytesBuild,
	BytesAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeCopy,
	
});


def_primitives_enum! (BytesPrimitive5, (procedure, 5), {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeCopy,
	
});


def_primitives_enum! (BytesPrimitiveN, (procedure, n), {
	
	BytesBuild,
	BytesAppend,
	
});


def_primitives_enum! (BytesPrimitiveV, (procedure, v), {
	
	BytesMake,
	BytesBuild,
	BytesAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeCopy,
	BytesRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesRangeReverse,
	
	BytesRangeToList,
	ListRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	BytesRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToBytes,
	
});




impl_procedure_primitive_enum_matrix! (
		(BytesPrimitive0, bytes_primitive_0_evaluate, bytes_primitive_0_attributes, bytes_primitive_v_alternative_0),
		(BytesPrimitive1, bytes_primitive_1_evaluate, bytes_primitive_1_attributes, bytes_primitive_v_alternative_1),
		(BytesPrimitive2, bytes_primitive_2_evaluate, bytes_primitive_2_attributes, bytes_primitive_v_alternative_2),
		(BytesPrimitive3, bytes_primitive_3_evaluate, bytes_primitive_3_attributes, bytes_primitive_v_alternative_3),
		(BytesPrimitive4, bytes_primitive_4_evaluate, bytes_primitive_4_attributes, bytes_primitive_v_alternative_4),
		(BytesPrimitive5, bytes_primitive_5_evaluate, bytes_primitive_5_attributes, bytes_primitive_v_alternative_5),
		(BytesPrimitiveN, bytes_primitive_n_evaluate, bytes_primitive_n_attributes, bytes_primitive_v_alternative_n),
		(BytesPrimitiveV, bytes_primitive_v_evaluate, bytes_primitive_v_attributes),
	);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_0_evaluate (primitive : BytesPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive0::BytesBuild =>
			return bytes_empty (None) .into_0 (),
		
		BytesPrimitive0::BytesAppend =>
			return bytes_empty (None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_1_evaluate (primitive : BytesPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive1::BytesLength =>
			return bytes_length (input_1) .into_0 (),
		
		BytesPrimitive1::BytesClone =>
			return bytes_clone (input_1, None),
		
		BytesPrimitive1::BytesCloneReverse =>
			return bytes_reverse (input_1, None),
		
		BytesPrimitive1::BytesMake =>
			return bytes_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None, None),
		
		BytesPrimitive1::BytesBuild =>
			return bytes_build_1 (input_1, None),
		
		BytesPrimitive1::BytesAppend =>
			return bytes_clone (input_1, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive1::BytesFill =>
			return bytes_fill_range (input_1, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive1::BytesReverse =>
			return bytes_reverse_range (input_1, None, None) .into_0 (),
		
		BytesPrimitive1::BytesToList =>
			return bytes_range_to_list (input_1, None, None, None),
		
		BytesPrimitive1::ListToBytes =>
			return list_range_to_bytes (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive1::BytesToArray =>
			return bytes_range_to_array (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive1::ArrayToBytes =>
			return array_range_to_bytes (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive1::BytesToImmutable =>
			return try_as_bytes_as_ref! (input_1) .to_immutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive1::BytesToMutable =>
			return try_as_bytes_as_ref! (input_1) .to_mutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive1::BytesRegexCompile =>
			return bytes_regex_compile (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_2_evaluate (primitive : BytesPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive2::BytesAt =>
			return bytes_at (input_1, r#try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		BytesPrimitive2::BytesMake =>
			return bytes_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2), None),
		
		BytesPrimitive2::BytesBuild =>
			return bytes_build_2 (input_1, input_2, None),
		
		BytesPrimitive2::BytesAppend =>
			return bytes_append_2 (input_1, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive2::BytesFill =>
			return bytes_fill_range (input_1, Some (input_2), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive2::BytesCopy =>
			return bytes_copy_range (input_1, None, input_2, None, None) .into_0 (),
		
		BytesPrimitive2::BytesRangeClone =>
			return bytes_clone_range (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive2::BytesRangeReverse =>
			return bytes_reverse_range (input_1, Some (input_2), None) .into_0 (),
		
		BytesPrimitive2::BytesRangeToList =>
			return bytes_range_to_list (input_1, Some (input_2), None, None),
		
		BytesPrimitive2::ListRangeToBytes =>
			return list_range_to_bytes (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive2::BytesRangeToArray =>
			return bytes_range_to_array (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive2::ArrayRangeToBytes =>
			return array_range_to_bytes (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatches =>
			return bytes_regex_matches (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchExtractFirst =>
			return bytes_regex_match_extract_first (input_1, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchExtractAllAsList =>
			return bytes_regex_match_extract_all (input_1, input_2, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchExtractAllAsArray =>
			return bytes_regex_match_extract_all (input_1, input_2, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchPositionFirst =>
			return bytes_regex_match_position_first (input_1, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchPositionAllAsList =>
			return bytes_regex_match_position_all (input_1, input_2, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchPositionAllAsArray =>
			return bytes_regex_match_position_all (input_1, input_2, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsList =>
			return bytes_regex_match_captures_extract_first (input_1, input_2, false, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsAssoc =>
			return bytes_regex_match_captures_extract_first (input_1, input_2, false, true, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsArray =>
			return bytes_regex_match_captures_extract_first (input_1, input_2, true, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsList =>
			return bytes_regex_match_captures_extract_all (input_1, input_2, false, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsAssoc =>
			return bytes_regex_match_captures_extract_all (input_1, input_2, false, true, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsArray =>
			return bytes_regex_match_captures_extract_all (input_1, input_2, true, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsList =>
			return bytes_regex_match_captures_position_first (input_1, input_2, false, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsAssoc =>
			return bytes_regex_match_captures_position_first (input_1, input_2, false, true, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsArray =>
			return bytes_regex_match_captures_position_first (input_1, input_2, true, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsList =>
			return bytes_regex_match_captures_position_all (input_1, input_2, false, false, false, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsAssoc =>
			return bytes_regex_match_captures_position_all (input_1, input_2, false, true, true, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsArray =>
			return bytes_regex_match_captures_position_all (input_1, input_2, true, false, false, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_3_evaluate (primitive : BytesPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive3::BytesMake =>
			return bytes_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive3::BytesAtSet =>
			return bytes_at_set (input_1, r#try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		BytesPrimitive3::BytesBuild =>
			return bytes_build_3 (input_1, input_2, input_3, None),
		
		BytesPrimitive3::BytesAppend =>
			return bytes_append_3 (input_1, input_2, input_3, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive3::BytesRangeFill =>
			return bytes_fill_range (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive3::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, None, None) .into_0 (),
		
		BytesPrimitive3::BytesRangeClone =>
			return bytes_clone_range (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive3::BytesRangeReverse =>
			return bytes_reverse_range (input_1, Some (input_2), Some (input_3)) .into_0 (),
		
		BytesPrimitive3::BytesRangeToList =>
			return bytes_range_to_list (input_1, Some (input_2), Some (input_3), None),
		
		BytesPrimitive3::ListRangeToBytes =>
			return list_range_to_bytes (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive3::BytesRangeToArray =>
			return bytes_range_to_array (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitive3::ArrayRangeToBytes =>
			return array_range_to_bytes (input_1, Some (input_2), Some (input_3), None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_4_evaluate (primitive : BytesPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive4::BytesBuild =>
			return bytes_build_4 (input_1, input_2, input_3, input_4, None),
		
		BytesPrimitive4::BytesAppend =>
			return bytes_append_4 (input_1, input_2, input_3, input_4, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive4::BytesRangeFill =>
			return bytes_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive4::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, Some (input_4), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_5_evaluate (primitive : BytesPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitive5::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_n_evaluate (primitive : BytesPrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitiveN::BytesBuild =>
			return bytes_build_n (inputs, None),
		
		BytesPrimitiveN::BytesAppend =>
			return bytes_append_n (inputs, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_0 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive0>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive0::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive0::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_1 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive1>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			Some (BytesPrimitive1::BytesMake),
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive1::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive1::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive1::BytesFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive1::BytesClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			Some (BytesPrimitive1::BytesReverse),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive1::BytesToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive1::ListToBytes),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive1::BytesToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive1::ArrayToBytes),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_2 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive2>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			Some (BytesPrimitive2::BytesMake),
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive2::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive2::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive2::BytesFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive2::BytesCopy),
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive2::BytesRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			Some (BytesPrimitive2::BytesRangeReverse),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive2::BytesRangeToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive2::ListRangeToBytes),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive2::BytesRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive2::ArrayRangeToBytes),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_3 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive3>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			Some (BytesPrimitive3::BytesMake),
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive3::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive3::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive3::BytesRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive3::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive3::BytesRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			Some (BytesPrimitive3::BytesRangeReverse),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive3::BytesRangeToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive3::ListRangeToBytes),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive3::BytesRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive3::ArrayRangeToBytes),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_4 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive4>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive4::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive4::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive4::BytesRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive4::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_5 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive5>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			None,
		BytesPrimitiveV::BytesAppend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive5::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_v_alternative_n (primitive : BytesPrimitiveV) -> (Option<BytesPrimitiveN>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitiveN::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitiveN::BytesAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesPrimitiveV::BytesRangeReverse =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_0_attributes (_primitive : BytesPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_1_attributes (_primitive : BytesPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_2_attributes (_primitive : BytesPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_3_attributes (_primitive : BytesPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_4_attributes (_primitive : BytesPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_5_attributes (_primitive : BytesPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_primitive_n_attributes (_primitive : BytesPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

