

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

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
	
	pub use super::bytes_primitive_0_attributes;
	pub use super::bytes_primitive_1_attributes;
	pub use super::bytes_primitive_2_attributes;
	pub use super::bytes_primitive_3_attributes;
	pub use super::bytes_primitive_4_attributes;
	pub use super::bytes_primitive_5_attributes;
	pub use super::bytes_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive0 {
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive1 {
	
	BytesLength,
	BytesClone,
	BytesReverse,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	BytesFill,
	
	BytesToList,
	ListToBytes,
	BytesToArray,
	ArrayToBytes,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive2 {
	
	BytesAt,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	BytesFill,
	BytesCopy,
	BytesRangeClone,
	
	BytesRangeToList,
	ListRangeToBytes,
	BytesRangeToArray,
	ArrayRangeToBytes,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive3 {
	
	BytesAtSet,
	
	BytesBuild,
	BytesAppend,
	
	BytesRangeFill,
	BytesRangeCopy,
	BytesRangeClone,
	
	BytesRangeToList,
	ListRangeToBytes,
	BytesRangeToArray,
	ArrayRangeToBytes,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive4 {
	
	BytesBuild,
	BytesAppend,
	
	BytesRangeFill,
	BytesRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitive5 {
	
	BytesRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitiveN {
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum BytesPrimitiveV {
	
	BytesMake,
	BytesBuild,
	BytesAppend,
	
	BytesRangeFill,
	BytesRangeCopy,
	BytesRangeClone,
	
	BytesRangeToList,
	ListRangeToBytes,
	BytesRangeToArray,
	ArrayRangeToBytes,
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_0_evaluate (primitive : BytesPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive0::BytesBuild =>
			succeed! (bytes_empty ()),
		
		BytesPrimitive0::BytesAppend =>
			succeed! (bytes_empty ()),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_1_evaluate (primitive : BytesPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive1::BytesLength => {
			let length = try! (bytes_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		BytesPrimitive1::BytesClone =>
			return bytes_clone (input_1),
		
		BytesPrimitive1::BytesReverse =>
			return bytes_reverse (input_1),
		
		BytesPrimitive1::BytesMake =>
			return bytes_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		BytesPrimitive1::BytesBuild =>
			return bytes_build_1 (input_1),
		
		BytesPrimitive1::BytesAppend =>
			return bytes_clone (input_1),
		
		BytesPrimitive1::BytesFill =>
			return bytes_fill_range (input_1, None, None, None),
		
		BytesPrimitive1::BytesToList =>
			return bytes_range_to_list (input_1, None, None),
		
		BytesPrimitive1::ListToBytes =>
			return list_range_to_bytes (input_1, None, None),
		
		BytesPrimitive1::BytesToArray =>
			return bytes_range_to_array (input_1, None, None),
		
		BytesPrimitive1::ArrayToBytes =>
			return array_range_to_bytes (input_1, None, None),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_2_evaluate (primitive : BytesPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive2::BytesAt =>
			return bytes_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		BytesPrimitive2::BytesMake =>
			return bytes_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2)),
		
		BytesPrimitive2::BytesBuild =>
			return bytes_build_2 (input_1, input_2),
		
		BytesPrimitive2::BytesAppend =>
			return bytes_append_2 (input_1, input_2),
		
		BytesPrimitive2::BytesFill =>
			return bytes_fill_range (input_1, Some (input_2), None, None),
		
		BytesPrimitive2::BytesCopy =>
			return bytes_copy_range (input_1, None, input_2, None, None),
		
		BytesPrimitive2::BytesRangeClone =>
			return bytes_clone_range (input_1, Some (input_2), None),
		
		BytesPrimitive2::BytesRangeToList =>
			return bytes_range_to_list (input_1, Some (input_2), None),
		
		BytesPrimitive2::ListRangeToBytes =>
			return list_range_to_bytes (input_1, Some (input_2), None),
		
		BytesPrimitive2::BytesRangeToArray =>
			return bytes_range_to_array (input_1, Some (input_2), None),
		
		BytesPrimitive2::ArrayRangeToBytes =>
			return array_range_to_bytes (input_1, Some (input_2), None),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_3_evaluate (primitive : BytesPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive3::BytesAtSet =>
			return bytes_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		BytesPrimitive3::BytesBuild =>
			return bytes_build_3 (input_1, input_2, input_3),
		
		BytesPrimitive3::BytesAppend =>
			return bytes_append_3 (input_1, input_2, input_3),
		
		BytesPrimitive3::BytesRangeFill =>
			return bytes_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		BytesPrimitive3::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, None, None),
		
		BytesPrimitive3::BytesRangeClone =>
			return bytes_clone_range (input_1, Some (input_2), Some (input_3)),
		
		BytesPrimitive3::BytesRangeToList =>
			return bytes_range_to_list (input_1, Some (input_2), Some (input_3)),
		
		BytesPrimitive3::ListRangeToBytes =>
			return list_range_to_bytes (input_1, Some (input_2), Some (input_3)),
		
		BytesPrimitive3::BytesRangeToArray =>
			return bytes_range_to_array (input_1, Some (input_2), Some (input_3)),
		
		BytesPrimitive3::ArrayRangeToBytes =>
			return array_range_to_bytes (input_1, Some (input_2), Some (input_3)),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_4_evaluate (primitive : BytesPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive4::BytesBuild =>
			return bytes_build_4 (input_1, input_2, input_3, input_4),
		
		BytesPrimitive4::BytesAppend =>
			return bytes_append_4 (input_1, input_2, input_3, input_4),
		
		BytesPrimitive4::BytesRangeFill =>
			return bytes_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		BytesPrimitive4::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_5_evaluate (primitive : BytesPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive5::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_n_evaluate (primitive : BytesPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitiveN::BytesBuild =>
			return bytes_build_n (inputs),
		
		BytesPrimitiveN::BytesAppend =>
			return bytes_append_n (inputs),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_0 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive0>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive0::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive0::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			None,
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_1 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive1>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			Some (BytesPrimitive1::BytesMake),
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive1::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive1::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive1::BytesFill),
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive1::BytesClone),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive1::BytesToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive1::ListToBytes),
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive1::BytesToArray),
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive1::ArrayToBytes),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_2 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive2>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			Some (BytesPrimitive2::BytesMake),
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive2::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive2::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive2::BytesFill),
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive2::BytesCopy),
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive2::BytesRangeClone),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive2::BytesRangeToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive2::ListRangeToBytes),
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive2::BytesRangeToArray),
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive2::ArrayRangeToBytes),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_3 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive3>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive3::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive3::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive3::BytesRangeFill),
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive3::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			Some (BytesPrimitive3::BytesRangeClone),
		BytesPrimitiveV::BytesRangeToList =>
			Some (BytesPrimitive3::BytesRangeToList),
		BytesPrimitiveV::ListRangeToBytes =>
			Some (BytesPrimitive3::ListRangeToBytes),
		BytesPrimitiveV::BytesRangeToArray =>
			Some (BytesPrimitive3::BytesRangeToArray),
		BytesPrimitiveV::ArrayRangeToBytes =>
			Some (BytesPrimitive3::ArrayRangeToBytes),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_4 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive4>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitive4::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitive4::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			Some (BytesPrimitive4::BytesRangeFill),
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive4::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_5 (primitive : BytesPrimitiveV) -> (Option<BytesPrimitive5>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			None,
		BytesPrimitiveV::BytesAppend =>
			None,
		BytesPrimitiveV::BytesRangeFill =>
			None,
		BytesPrimitiveV::BytesRangeCopy =>
			Some (BytesPrimitive5::BytesRangeCopy),
		BytesPrimitiveV::BytesRangeClone =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_v_alternative_n (primitive : BytesPrimitiveV) -> (Option<BytesPrimitiveN>) {
	match primitive {
		BytesPrimitiveV::BytesMake =>
			None,
		BytesPrimitiveV::BytesBuild =>
			Some (BytesPrimitiveN::BytesBuild),
		BytesPrimitiveV::BytesAppend =>
			Some (BytesPrimitiveN::BytesAppend),
		BytesPrimitiveV::BytesRangeFill =>
			None,
		BytesPrimitiveV::BytesRangeCopy =>
			None,
		BytesPrimitiveV::BytesRangeClone =>
			None,
		BytesPrimitiveV::BytesRangeToList =>
			None,
		BytesPrimitiveV::ListRangeToBytes =>
			None,
		BytesPrimitiveV::BytesRangeToArray =>
			None,
		BytesPrimitiveV::ArrayRangeToBytes =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_0_attributes (_primitive : BytesPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_1_attributes (_primitive : BytesPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_2_attributes (_primitive : BytesPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_3_attributes (_primitive : BytesPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_4_attributes (_primitive : BytesPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_5_attributes (_primitive : BytesPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_primitive_n_attributes (_primitive : BytesPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

