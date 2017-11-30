

use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::BytesPrimitive0;
	pub use super::BytesPrimitive1;
	pub use super::BytesPrimitive2;
	pub use super::BytesPrimitive3;
	pub use super::BytesPrimitive4;
	pub use super::BytesPrimitive5;
	pub use super::BytesPrimitiveN;
	
	pub use super::bytes_primitive_0_evaluate;
	pub use super::bytes_primitive_1_evaluate;
	pub use super::bytes_primitive_2_evaluate;
	pub use super::bytes_primitive_3_evaluate;
	pub use super::bytes_primitive_4_evaluate;
	pub use super::bytes_primitive_5_evaluate;
	pub use super::bytes_primitive_n_evaluate;
	
	pub use super::bytes_primitive_n_alternative_0;
	pub use super::bytes_primitive_n_alternative_1;
	pub use super::bytes_primitive_n_alternative_2;
	pub use super::bytes_primitive_n_alternative_3;
	pub use super::bytes_primitive_n_alternative_4;
	pub use super::bytes_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive0 {
	
	BytesBuild,
	BytesAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive4 {
	
	BytesBuild,
	BytesAppend,
	
	BytesRangeFill,
	BytesRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive5 {
	
	BytesRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitiveN {
	
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




pub fn bytes_primitive_0_evaluate (primitive : BytesPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive0::BytesBuild =>
			succeed! (bytes_empty ()),
		
		BytesPrimitive0::BytesAppend =>
			succeed! (bytes_empty ()),
		
	}
}




pub fn bytes_primitive_1_evaluate (primitive : BytesPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
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
			fail_unimplemented! (0x80c25ce5),
		
		BytesPrimitive1::ListToBytes =>
			fail_unimplemented! (0xa6a0bffe),
		
		BytesPrimitive1::BytesToArray =>
			fail_unimplemented! (0x5f03ef78),
		
		BytesPrimitive1::ArrayToBytes =>
			fail_unimplemented! (0xa554ede3),
		
	}
}




pub fn bytes_primitive_2_evaluate (primitive : BytesPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
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
			fail_unimplemented! (0x9dd4c575),
		
		BytesPrimitive2::ListRangeToBytes =>
			fail_unimplemented! (0xb698c5e8),
		
		BytesPrimitive2::BytesRangeToArray =>
			fail_unimplemented! (0x30910c70),
		
		BytesPrimitive2::ArrayRangeToBytes =>
			fail_unimplemented! (0xac2e5c1c),
		
	}
}




pub fn bytes_primitive_3_evaluate (primitive : BytesPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
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
			fail_unimplemented! (0xd155ed3b),
		
		BytesPrimitive3::ListRangeToBytes =>
			fail_unimplemented! (0x7b99c44f),
		
		BytesPrimitive3::BytesRangeToArray =>
			fail_unimplemented! (0x9c84f53a),
		
		BytesPrimitive3::ArrayRangeToBytes =>
			fail_unimplemented! (0x4bcf5b5c),
		
	}
}




pub fn bytes_primitive_4_evaluate (primitive : BytesPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
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




pub fn bytes_primitive_5_evaluate (primitive : BytesPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive5::BytesRangeCopy =>
			return bytes_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




pub fn bytes_primitive_n_evaluate (primitive : BytesPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		BytesPrimitiveN::BytesMake =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesMake, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x670a044a),
			},
		
		BytesPrimitiveN::BytesBuild =>
			match inputs_count {
				0 =>
					return bytes_primitive_0_evaluate (BytesPrimitive0::BytesBuild),
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesBuild, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesBuild, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return bytes_build_n (inputs),
			},
		
		BytesPrimitiveN::BytesAppend =>
			match inputs_count {
				0 =>
					return bytes_primitive_0_evaluate (BytesPrimitive0::BytesAppend),
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesAppend, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesAppend, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return bytes_append_n (inputs),
			},
		
		BytesPrimitiveN::BytesRangeFill =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesFill, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesFill, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesRangeFill, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesRangeFill, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					fail! (0x2837269d),
			},
		
		BytesPrimitiveN::BytesRangeCopy =>
			match inputs_count {
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesCopy, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesRangeCopy, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return bytes_primitive_4_evaluate (BytesPrimitive4::BytesRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				5 =>
					return bytes_primitive_5_evaluate (BytesPrimitive5::BytesRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]),
				_ =>
					fail! (0xc1e611dd),
			},
		
		BytesPrimitiveN::BytesRangeClone =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesClone, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesRangeClone, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesRangeClone, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xf54b2943),
			},
		
		BytesPrimitiveN::BytesRangeToList =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesToList, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesRangeToList, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesRangeToList, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xc1b7658d),
			},
		
		BytesPrimitiveN::ListRangeToBytes =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::ListToBytes, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::ListRangeToBytes, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::ListRangeToBytes, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x6de74659),
			},
		
		BytesPrimitiveN::BytesRangeToArray =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::BytesToArray, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::BytesRangeToArray, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::BytesRangeToArray, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xb3fee627),
			},
		
		BytesPrimitiveN::ArrayRangeToBytes =>
			match inputs_count {
				1 =>
					return bytes_primitive_1_evaluate (BytesPrimitive1::ArrayToBytes, &inputs[0]),
				2 =>
					return bytes_primitive_2_evaluate (BytesPrimitive2::ArrayRangeToBytes, &inputs[0], &inputs[1]),
				3 =>
					return bytes_primitive_3_evaluate (BytesPrimitive3::ArrayRangeToBytes, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xf8126771),
			},
		
	}
}




pub fn bytes_primitive_n_alternative_0 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive0>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			None,
		BytesPrimitiveN::BytesBuild =>
			Some (BytesPrimitive0::BytesBuild),
		BytesPrimitiveN::BytesAppend =>
			Some (BytesPrimitive0::BytesAppend),
		BytesPrimitiveN::BytesRangeFill =>
			None,
		BytesPrimitiveN::BytesRangeCopy =>
			None,
		BytesPrimitiveN::BytesRangeClone =>
			None,
		BytesPrimitiveN::BytesRangeToList =>
			None,
		BytesPrimitiveN::ListRangeToBytes =>
			None,
		BytesPrimitiveN::BytesRangeToArray =>
			None,
		BytesPrimitiveN::ArrayRangeToBytes =>
			None,
	}
}


pub fn bytes_primitive_n_alternative_1 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive1>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			Some (BytesPrimitive1::BytesMake),
		BytesPrimitiveN::BytesBuild =>
			Some (BytesPrimitive1::BytesBuild),
		BytesPrimitiveN::BytesAppend =>
			Some (BytesPrimitive1::BytesAppend),
		BytesPrimitiveN::BytesRangeFill =>
			Some (BytesPrimitive1::BytesFill),
		BytesPrimitiveN::BytesRangeCopy =>
			None,
		BytesPrimitiveN::BytesRangeClone =>
			Some (BytesPrimitive1::BytesClone),
		BytesPrimitiveN::BytesRangeToList =>
			Some (BytesPrimitive1::BytesToList),
		BytesPrimitiveN::ListRangeToBytes =>
			Some (BytesPrimitive1::ListToBytes),
		BytesPrimitiveN::BytesRangeToArray =>
			Some (BytesPrimitive1::BytesToArray),
		BytesPrimitiveN::ArrayRangeToBytes =>
			Some (BytesPrimitive1::ArrayToBytes),
	}
}


pub fn bytes_primitive_n_alternative_2 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive2>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			Some (BytesPrimitive2::BytesMake),
		BytesPrimitiveN::BytesBuild =>
			Some (BytesPrimitive2::BytesBuild),
		BytesPrimitiveN::BytesAppend =>
			Some (BytesPrimitive2::BytesAppend),
		BytesPrimitiveN::BytesRangeFill =>
			Some (BytesPrimitive2::BytesFill),
		BytesPrimitiveN::BytesRangeCopy =>
			Some (BytesPrimitive2::BytesCopy),
		BytesPrimitiveN::BytesRangeClone =>
			Some (BytesPrimitive2::BytesRangeClone),
		BytesPrimitiveN::BytesRangeToList =>
			Some (BytesPrimitive2::BytesRangeToList),
		BytesPrimitiveN::ListRangeToBytes =>
			Some (BytesPrimitive2::ListRangeToBytes),
		BytesPrimitiveN::BytesRangeToArray =>
			Some (BytesPrimitive2::BytesRangeToArray),
		BytesPrimitiveN::ArrayRangeToBytes =>
			Some (BytesPrimitive2::ArrayRangeToBytes),
	}
}


pub fn bytes_primitive_n_alternative_3 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive3>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			None,
		BytesPrimitiveN::BytesBuild =>
			Some (BytesPrimitive3::BytesBuild),
		BytesPrimitiveN::BytesAppend =>
			Some (BytesPrimitive3::BytesAppend),
		BytesPrimitiveN::BytesRangeFill =>
			Some (BytesPrimitive3::BytesRangeFill),
		BytesPrimitiveN::BytesRangeCopy =>
			Some (BytesPrimitive3::BytesRangeCopy),
		BytesPrimitiveN::BytesRangeClone =>
			Some (BytesPrimitive3::BytesRangeClone),
		BytesPrimitiveN::BytesRangeToList =>
			Some (BytesPrimitive3::BytesRangeToList),
		BytesPrimitiveN::ListRangeToBytes =>
			Some (BytesPrimitive3::ListRangeToBytes),
		BytesPrimitiveN::BytesRangeToArray =>
			Some (BytesPrimitive3::BytesRangeToArray),
		BytesPrimitiveN::ArrayRangeToBytes =>
			Some (BytesPrimitive3::ArrayRangeToBytes),
	}
}


pub fn bytes_primitive_n_alternative_4 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive4>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			None,
		BytesPrimitiveN::BytesBuild =>
			Some (BytesPrimitive4::BytesBuild),
		BytesPrimitiveN::BytesAppend =>
			Some (BytesPrimitive4::BytesAppend),
		BytesPrimitiveN::BytesRangeFill =>
			Some (BytesPrimitive4::BytesRangeFill),
		BytesPrimitiveN::BytesRangeCopy =>
			Some (BytesPrimitive4::BytesRangeCopy),
		BytesPrimitiveN::BytesRangeClone =>
			None,
		BytesPrimitiveN::BytesRangeToList =>
			None,
		BytesPrimitiveN::ListRangeToBytes =>
			None,
		BytesPrimitiveN::BytesRangeToArray =>
			None,
		BytesPrimitiveN::ArrayRangeToBytes =>
			None,
	}
}


pub fn bytes_primitive_n_alternative_5 (primitive : BytesPrimitiveN) -> (Option<BytesPrimitive5>) {
	match primitive {
		BytesPrimitiveN::BytesMake =>
			None,
		BytesPrimitiveN::BytesBuild =>
			None,
		BytesPrimitiveN::BytesAppend =>
			None,
		BytesPrimitiveN::BytesRangeFill =>
			None,
		BytesPrimitiveN::BytesRangeCopy =>
			Some (BytesPrimitive5::BytesRangeCopy),
		BytesPrimitiveN::BytesRangeClone =>
			None,
		BytesPrimitiveN::BytesRangeToList =>
			None,
		BytesPrimitiveN::ListRangeToBytes =>
			None,
		BytesPrimitiveN::BytesRangeToArray =>
			None,
		BytesPrimitiveN::ArrayRangeToBytes =>
			None,
	}
}

