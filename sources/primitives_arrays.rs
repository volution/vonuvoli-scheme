

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

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
	
	pub use super::array_primitive_0_attributes;
	pub use super::array_primitive_1_attributes;
	pub use super::array_primitive_2_attributes;
	pub use super::array_primitive_3_attributes;
	pub use super::array_primitive_4_attributes;
	pub use super::array_primitive_5_attributes;
	pub use super::array_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive0 {
	
	ArrayBuild,
	ArrayAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive1 {
	
	ArrayLength,
	ArrayClone,
	ArrayCloneReverse,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayFill,
	ArrayReverse,
	
	ArrayToList,
	ListToArray,
	
	ArrayToImmutable,
	ArrayToMutable,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive2 {
	
	ArrayAt,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayFill,
	ArrayCopy,
	ArrayRangeClone,
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive3 {
	
	ArrayAtSet,
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	ArrayRangeClone,
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive4 {
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitive5 {
	
	ArrayRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitiveN {
	
	ArrayBuild,
	ArrayAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArrayPrimitiveV {
	
	ArrayMake,
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	ArrayRangeClone,
	ArrayRangeReverse,
	
	ArrayRangeToList,
	ListRangeToArray,
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_0_evaluate (primitive : ArrayPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive0::ArrayBuild =>
			return array_empty () .into_0 (),
		
		ArrayPrimitive0::ArrayAppend =>
			return array_empty () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_1_evaluate (primitive : ArrayPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive1::ArrayLength =>
			return array_length (input_1) .into_0 (),
		
		ArrayPrimitive1::ArrayClone =>
			return array_clone (input_1),
		
		ArrayPrimitive1::ArrayCloneReverse =>
			return array_reverse (input_1),
		
		ArrayPrimitive1::ArrayMake =>
			return array_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		ArrayPrimitive1::ArrayBuild =>
			return array_build_1 (input_1) .into_0 (),
		
		ArrayPrimitive1::ArrayAppend =>
			return array_clone (input_1),
		
		ArrayPrimitive1::ArrayFill =>
			return array_fill_range (input_1, None, None, None) .into_0 (),
		
		ArrayPrimitive1::ArrayReverse =>
			return array_reverse_range (input_1, None, None) .into_0 (),
		
		ArrayPrimitive1::ArrayToList =>
			return array_range_to_list (input_1, None, None),
		
		ArrayPrimitive1::ListToArray =>
			return list_range_to_array (input_1, None, None),
		
		ArrayPrimitive1::ArrayToImmutable =>
			return try_as_array_as_ref! (input_1) .to_immutable () .into_0 (),
		
		ArrayPrimitive1::ArrayToMutable =>
			return try_as_array_as_ref! (input_1) .to_mutable () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_2_evaluate (primitive : ArrayPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive2::ArrayAt =>
			return array_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ArrayPrimitive2::ArrayMake =>
			return array_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2)),
		
		ArrayPrimitive2::ArrayBuild =>
			return array_build_2 (input_1, input_2) .into_0 (),
		
		ArrayPrimitive2::ArrayAppend =>
			return array_append_2 (input_1, input_2),
		
		ArrayPrimitive2::ArrayFill =>
			return array_fill_range (input_1, Some (input_2), None, None) .into_0 (),
		
		ArrayPrimitive2::ArrayCopy =>
			return array_copy_range (input_1, None, input_2, None, None) .into_0 (),
		
		ArrayPrimitive2::ArrayRangeClone =>
			return array_clone_range (input_1, Some (input_2), None),
		
		ArrayPrimitive2::ArrayRangeReverse =>
			return array_reverse_range (input_1, Some (input_2), None) .into_0 (),
		
		ArrayPrimitive2::ArrayRangeToList =>
			return array_range_to_list (input_1, Some (input_2), None),
		
		ArrayPrimitive2::ListRangeToArray =>
			return list_range_to_array (input_1, Some (input_2), None),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_3_evaluate (primitive : ArrayPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive3::ArrayAtSet =>
			return array_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ArrayPrimitive3::ArrayBuild =>
			return array_build_3 (input_1, input_2, input_3) .into_0 (),
		
		ArrayPrimitive3::ArrayAppend =>
			return array_append_3 (input_1, input_2, input_3),
		
		ArrayPrimitive3::ArrayRangeFill =>
			return array_fill_range (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		ArrayPrimitive3::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, None, None) .into_0 (),
		
		ArrayPrimitive3::ArrayRangeClone =>
			return array_clone_range (input_1, Some (input_2), Some (input_3)),
		
		ArrayPrimitive3::ArrayRangeReverse =>
			return array_reverse_range (input_1, Some (input_2), Some (input_3)) .into_0 (),
		
		ArrayPrimitive3::ArrayRangeToList =>
			return array_range_to_list (input_1, Some (input_2), Some (input_3)),
		
		ArrayPrimitive3::ListRangeToArray =>
			return list_range_to_array (input_1, Some (input_2), Some (input_3)),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_4_evaluate (primitive : ArrayPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive4::ArrayBuild =>
			return array_build_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		ArrayPrimitive4::ArrayAppend =>
			return array_append_4 (input_1, input_2, input_3, input_4),
		
		ArrayPrimitive4::ArrayRangeFill =>
			return array_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)) .into_0 (),
		
		ArrayPrimitive4::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, Some (input_4), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_5_evaluate (primitive : ArrayPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive5::ArrayRangeCopy =>
			return array_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_n_evaluate (primitive : ArrayPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitiveN::ArrayBuild =>
			return array_build_n (inputs) .into_0 (),
		
		ArrayPrimitiveN::ArrayAppend =>
			return array_append_n (inputs),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_0 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive0>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive0::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive0::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_1 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive1>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			Some (ArrayPrimitive1::ArrayMake),
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive1::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive1::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive1::ArrayFill),
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive1::ArrayClone),
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive1::ArrayReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive1::ArrayToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive1::ListToArray),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_2 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive2>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			Some (ArrayPrimitive2::ArrayMake),
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive2::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive2::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive2::ArrayFill),
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive2::ArrayCopy),
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive2::ArrayRangeClone),
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive2::ArrayRangeReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive2::ArrayRangeToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive2::ListRangeToArray),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_3 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive3>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive3::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive3::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive3::ArrayRangeFill),
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive3::ArrayRangeCopy),
		ArrayPrimitiveV::ArrayRangeClone =>
			Some (ArrayPrimitive3::ArrayRangeClone),
		ArrayPrimitiveV::ArrayRangeReverse =>
			Some (ArrayPrimitive3::ArrayRangeReverse),
		ArrayPrimitiveV::ArrayRangeToList =>
			Some (ArrayPrimitive3::ArrayRangeToList),
		ArrayPrimitiveV::ListRangeToArray =>
			Some (ArrayPrimitive3::ListRangeToArray),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_4 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive4>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitive4::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitive4::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			Some (ArrayPrimitive4::ArrayRangeFill),
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive4::ArrayRangeCopy),
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_5 (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitive5>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			None,
		ArrayPrimitiveV::ArrayAppend =>
			None,
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		ArrayPrimitiveV::ArrayRangeCopy =>
			Some (ArrayPrimitive5::ArrayRangeCopy),
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_v_alternative_n (primitive : ArrayPrimitiveV) -> (Option<ArrayPrimitiveN>) {
	match primitive {
		ArrayPrimitiveV::ArrayMake =>
			None,
		ArrayPrimitiveV::ArrayBuild =>
			Some (ArrayPrimitiveN::ArrayBuild),
		ArrayPrimitiveV::ArrayAppend =>
			Some (ArrayPrimitiveN::ArrayAppend),
		ArrayPrimitiveV::ArrayRangeFill =>
			None,
		ArrayPrimitiveV::ArrayRangeCopy =>
			None,
		ArrayPrimitiveV::ArrayRangeClone =>
			None,
		ArrayPrimitiveV::ArrayRangeReverse =>
			None,
		ArrayPrimitiveV::ArrayRangeToList =>
			None,
		ArrayPrimitiveV::ListRangeToArray =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_0_attributes (_primitive : ArrayPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_1_attributes (_primitive : ArrayPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_2_attributes (_primitive : ArrayPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_3_attributes (_primitive : ArrayPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_4_attributes (_primitive : ArrayPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_5_attributes (_primitive : ArrayPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn array_primitive_n_attributes (_primitive : ArrayPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

