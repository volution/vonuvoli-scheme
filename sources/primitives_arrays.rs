

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ArrayPrimitive0;
	pub use super::ArrayPrimitive1;
	pub use super::ArrayPrimitive2;
	pub use super::ArrayPrimitive3;
	pub use super::ArrayPrimitive4;
	pub use super::ArrayPrimitive5;
	pub use super::ArrayPrimitiveN;
	pub use super::ArrayPrimitiveV;
	
	pub use super::array_primitive_0_evaluate;
	pub use super::array_primitive_1_evaluate;
	pub use super::array_primitive_2_evaluate;
	pub use super::array_primitive_3_evaluate;
	pub use super::array_primitive_4_evaluate;
	pub use super::array_primitive_5_evaluate;
	pub use super::array_primitive_n_evaluate;
	
	pub use super::array_primitive_v_alternative_0;
	pub use super::array_primitive_v_alternative_1;
	pub use super::array_primitive_v_alternative_2;
	pub use super::array_primitive_v_alternative_3;
	pub use super::array_primitive_v_alternative_4;
	pub use super::array_primitive_v_alternative_5;
	pub use super::array_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::array_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (ArrayPrimitive0, (procedure, 0), {
	
	ArrayBuild,
	ArrayAppend,
	
});


def_primitives_enum! (ArrayPrimitive1, (procedure, 1), {
	
	ArrayLength,
	ArrayClone,
	ArrayCloneReverse,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayReverse,
	
	ArrayToList,
	ListToArray,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayToMutable,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPop,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayClear,
	
});


def_primitives_enum! (ArrayPrimitive2, (procedure, 2), {
	
	ArrayAt,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayCopy,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayExtend,
	ArrayRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPushFrom,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	ArrayPop,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRemove,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResize,
	
	ArrayFind,
	
});


def_primitives_enum! (ArrayPrimitive3, (procedure, 3), {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayAtSet,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeCopy,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeExtend,
	ArrayRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsert,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsertFrom,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	ArrayRemove,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArraySwap,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResize,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResizeAt,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayClearAt,
	
});


def_primitives_enum! (ArrayPrimitive4, (procedure, 4), {
	
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeCopy,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeExtend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsert,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResizeAt,
	
});


def_primitives_enum! (ArrayPrimitive5, (procedure, 5), {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeCopy,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeExtend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsert,
	
});


def_primitives_enum! (ArrayPrimitiveN, (procedure, n), {
	
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsert,
	
});


def_primitives_enum! (ArrayPrimitiveV, (procedure, v), {
	
	ArrayMake,
	ArrayBuild,
	ArrayAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeCopy,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeExtend,
	ArrayRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPush,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayPop,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayInsert,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayRemove,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResize,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayResizeAt,
	
});




impl_procedure_primitive_enum_matrix! (
		(ArrayPrimitive0, array_primitive_0_evaluate, array_primitive_0_attributes, array_primitive_v_alternative_0),
		(ArrayPrimitive1, array_primitive_1_evaluate, array_primitive_1_attributes, array_primitive_v_alternative_1),
		(ArrayPrimitive2, array_primitive_2_evaluate, array_primitive_2_attributes, array_primitive_v_alternative_2),
		(ArrayPrimitive3, array_primitive_3_evaluate, array_primitive_3_attributes, array_primitive_v_alternative_3),
		(ArrayPrimitive4, array_primitive_4_evaluate, array_primitive_4_attributes, array_primitive_v_alternative_4),
		(ArrayPrimitive5, array_primitive_5_evaluate, array_primitive_5_attributes, array_primitive_v_alternative_5),
		(ArrayPrimitiveN, array_primitive_n_evaluate, array_primitive_n_attributes, array_primitive_v_alternative_n),
		(ArrayPrimitiveV, array_primitive_v_evaluate, array_primitive_v_attributes),
	);




pub fn array_primitive_0_evaluate (primitive : ArrayPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive0::ArrayBuild =>
			return array_empty (None) .into_0 (),
		
		ArrayPrimitive0::ArrayAppend =>
			return array_empty (None) .into_0 (),
		
	}
}




pub fn array_primitive_1_evaluate (primitive : ArrayPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive1::ArrayLength =>
			return array_length (input_1) .into_0 (),
		
		ArrayPrimitive1::ArrayClone =>
			return array_clone (input_1, None),
		
		ArrayPrimitive1::ArrayCloneReverse =>
			return array_reverse (input_1, None),
		
		ArrayPrimitive1::ArrayMake =>
			return array_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None, None),
		
		ArrayPrimitive1::ArrayBuild =>
			return array_build_1 (input_1, None) .into_0 (),
		
		ArrayPrimitive1::ArrayAppend =>
			return array_clone (input_1, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayFill =>
			return array_fill_range (input_1, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayReverse =>
			return array_reverse_range (input_1, None, None) .into_0 (),
		
		ArrayPrimitive1::ArrayToList =>
			return array_range_to_list (input_1, None, None, None),
		
		ArrayPrimitive1::ListToArray =>
			return list_range_to_array (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayToImmutable =>
			return try_as_array_as_ref! (input_1) .to_immutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayToMutable =>
			return try_as_array_as_ref! (input_1) .to_mutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayPop =>
			return array_pop (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive1::ArrayClear =>
			return array_clear (input_1) .into_0 (),
		
	}
}




pub fn array_primitive_2_evaluate (primitive : ArrayPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive2::ArrayAt =>
			return array_at (input_1, r#try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ArrayPrimitive2::ArrayMake =>
			return array_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2), None),
		
		ArrayPrimitive2::ArrayBuild =>
			return array_build_2 (input_1, input_2, None) .into_0 (),
		
		ArrayPrimitive2::ArrayAppend =>
			return array_append_2 (input_1, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayFill =>
			return array_fill_range (input_1, Some (input_2), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayCopy =>
			return array_copy_range (input_1, None, input_2, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayExtend =>
			return array_extend_range (input_1, None, input_2, None, None) .into_0 (),
		
		ArrayPrimitive2::ArrayRangeClone =>
			return array_clone_range (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayRangeReverse =>
			return array_reverse_range (input_1, Some (input_2), None) .into_0 (),
		
		ArrayPrimitive2::ArrayRangeToList =>
			return array_range_to_list (input_1, Some (input_2), None, None),
		
		ArrayPrimitive2::ListRangeToArray =>
			return list_range_to_array (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayPush =>
			return array_push (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayPushFrom =>
			return array_push_from (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ArrayPrimitive2::ArrayPop =>
			return array_pop_n (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayRemove =>
			return array_remove (input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive2::ArrayResize =>
			return array_resize (input_1, input_2, None) .into_0 (),
		
		ArrayPrimitive2::ArrayFind =>
			return array_find (input_2, input_1, evaluator),
		
	}
}




pub fn array_primitive_3_evaluate (primitive : ArrayPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayAtSet =>
			return array_at_set (input_1, r#try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ArrayPrimitive3::ArrayMake =>
			return array_make (r#try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())),
		
		ArrayPrimitive3::ArrayBuild =>
			return array_build_3 (input_1, input_2, input_3, None) .into_0 (),
		
		ArrayPrimitive3::ArrayAppend =>
			return array_append_3 (input_1, input_2, input_3, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayRangeFill =>
			return array_fill_range (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayRangeExtend =>
			return array_extend_range (input_1, None, input_2, Some (input_3), None) .into_0 (),
		
		ArrayPrimitive3::ArrayRangeClone =>
			return array_clone_range (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayRangeReverse =>
			return array_reverse_range (input_1, Some (input_2), Some (input_3)) .into_0 (),
		
		ArrayPrimitive3::ArrayRangeToList =>
			return array_range_to_list (input_1, Some (input_2), Some (input_3), None),
		
		ArrayPrimitive3::ListRangeToArray =>
			return list_range_to_array (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayPush =>
			return array_push_n (input_1, &[input_2, input_3]) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayInsert =>
			return array_insert (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayInsertFrom =>
			return array_insert_from (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ArrayPrimitive3::ArrayRemove =>
			return array_remove_n (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArraySwap =>
			return array_swap (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayResize =>
			return array_resize (input_1, input_2, Some (input_3)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayResizeAt =>
			return array_resize_at (input_1, input_2, input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive3::ArrayClearAt =>
			return array_clear_at (input_1, input_2, input_3) .into_0 (),
		
	}
}




pub fn array_primitive_4_evaluate (primitive : ArrayPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive4::ArrayBuild =>
			return array_build_4 (input_1, input_2, input_3, input_4, None) .into_0 (),
		
		ArrayPrimitive4::ArrayAppend =>
			return array_append_4 (input_1, input_2, input_3, input_4, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayRangeFill =>
			return array_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, Some (input_4), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayRangeExtend =>
			return array_extend_range (input_1, None, input_2, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayPush =>
			return array_push_n (input_1, &[input_2, input_3, input_4]) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayInsert =>
			return array_insert_n (input_1, input_2, &[input_3, input_4]) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive4::ArrayResizeAt =>
			return array_resize_at (input_1, input_2, input_3, Some (input_4)) .into_0 (),
		
	}
}




pub fn array_primitive_5_evaluate (primitive : ArrayPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive5::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive5::ArrayRangeExtend =>
			return array_extend_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive5::ArrayPush =>
			return array_push_n (input_1, &[input_2, input_3, input_4, input_5]) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitive5::ArrayInsert =>
			return array_insert_n (input_1, input_2, &[input_3, input_4, input_5]) .into_0 (),
		
	}
}




pub fn array_primitive_n_evaluate (primitive : ArrayPrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitiveN::ArrayBuild =>
			return array_build_n (inputs, None) .into_0 (),
		
		ArrayPrimitiveN::ArrayAppend =>
			return array_append_n (inputs, None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveN::ArrayPush =>
			if inputs.len () >= 2 {
				return array_push_n (inputs[0].as_ref (), &inputs[1..]) .into_0 ();
			} else {
				fail! (0xe5e9121d);
			},
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveN::ArrayInsert =>
			if inputs.len () >= 3 {
				return array_insert_n (inputs[0].as_ref (), inputs[1].as_ref (), &inputs[2..]) .into_0 ();
			} else {
				fail! (0x68d3fe45);
			},
		
	}
}




pub fn array_primitive_v_alternative_0 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive0>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive0::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive0::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			None,
	}
}




pub fn array_primitive_v_alternative_1 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive1>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			Some (ArrayPrimitive1::ArrayMake),
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive1::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive1::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive1::ArrayFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive1::ArrayClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive1::ArrayReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive1::ArrayToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive1::ListToArray),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			Some (ArrayPrimitive1::ArrayPop),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			None,
	}
}




pub fn array_primitive_v_alternative_2 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive2>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			Some (ArrayPrimitive2::ArrayMake),
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive2::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive2::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive2::ArrayFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive2::ArrayCopy),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			Some (ArrayPrimitive2::ArrayExtend),
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive2::ArrayRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive2::ArrayRangeReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive2::ArrayRangeToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive2::ListRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			Some (ArrayPrimitive2::ArrayPush),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ArrayPrimitiveV::ArrayPop =>
			Some (ArrayPrimitive2::ArrayPop),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( not ( feature = "vonuvoli_values_values" ) ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			Some (ArrayPrimitive2::ArrayRemove),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			Some (ArrayPrimitive2::ArrayResize),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			None,
	}
}




pub fn array_primitive_v_alternative_3 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive3>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			Some (ArrayPrimitive3::ArrayMake),
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive3::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive3::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive3::ArrayRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive3::ArrayRangeCopy),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			Some (ArrayPrimitive3::ArrayRangeExtend),
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive3::ArrayRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive3::ArrayRangeReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive3::ArrayRangeToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive3::ListRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			Some (ArrayPrimitive3::ArrayPush),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			Some (ArrayPrimitive3::ArrayInsert),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			Some (ArrayPrimitive3::ArrayRemove),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		#[ cfg ( not ( feature = "vonuvoli_values_values" ) ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			Some (ArrayPrimitive3::ArrayResize),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			Some (ArrayPrimitive3::ArrayResizeAt),
	}
}




pub fn array_primitive_v_alternative_4 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive4>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive4::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive4::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive4::ArrayRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive4::ArrayRangeCopy),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			Some (ArrayPrimitive4::ArrayRangeExtend),
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			Some (ArrayPrimitive4::ArrayPush),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			Some (ArrayPrimitive4::ArrayInsert),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			Some (ArrayPrimitive4::ArrayResizeAt),
	}
}




pub fn array_primitive_v_alternative_5 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive5>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			None,
		ArrayPrimitiveV::ArrayAppend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive5::ArrayRangeCopy),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			Some (ArrayPrimitive5::ArrayRangeExtend),
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			Some (ArrayPrimitive5::ArrayPush),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			Some (ArrayPrimitive5::ArrayInsert),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			None,
	}
}




pub fn array_primitive_v_alternative_n (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitiveN>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitiveN::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitiveN::ArrayAppend),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeExtend =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPush =>
			Some (ArrayPrimitiveN::ArrayPush),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayPop =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayInsert =>
			Some (ArrayPrimitiveN::ArrayInsert),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayRemove =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResize =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ArrayPrimitiveV::ArrayResizeAt =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_0_attributes (_primitive : ArrayPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_1_attributes (_primitive : ArrayPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_2_attributes (_primitive : ArrayPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_3_attributes (_primitive : ArrayPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_4_attributes (_primitive : ArrayPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_5_attributes (_primitive : ArrayPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub fn array_primitive_n_attributes (_primitive : ArrayPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

