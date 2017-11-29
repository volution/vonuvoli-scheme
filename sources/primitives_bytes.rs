

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
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive2 {
	
	BytesAt,
	
	BytesMake,
	
	BytesBuild,
	BytesAppend,
	
	BytesFill,
	BytesCopy,
	BytesSliceClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive3 {
	
	BytesAtSet,
	
	BytesBuild,
	BytesAppend,
	
	BytesSliceFill,
	BytesSliceCopy,
	BytesSliceClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive4 {
	
	BytesBuild,
	BytesAppend,
	
	BytesSliceFill,
	BytesSliceCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitive5 {
	
	BytesSliceCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BytesPrimitiveN {
	
	BytesMake,
	BytesBuild,
	BytesAppend,
	
	BytesSliceFill,
	BytesSliceCopy,
	BytesSliceClone,
	
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
			fail_unimplemented! (0x4528afb1),
		
		BytesPrimitive2::BytesCopy =>
			fail_unimplemented! (0xac59b811),
		
		BytesPrimitive2::BytesSliceClone =>
			fail_unimplemented! (0xcd4a7888),
		
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
		
		BytesPrimitive3::BytesSliceFill =>
			fail_unimplemented! (0x20bbd19a),
		
		BytesPrimitive3::BytesSliceCopy =>
			fail_unimplemented! (0x105cc749),
		
		BytesPrimitive3::BytesSliceClone =>
			fail_unimplemented! (0x8b767508),
		
	}
}




pub fn bytes_primitive_4_evaluate (primitive : BytesPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive4::BytesBuild =>
			return bytes_build_4 (input_1, input_2, input_3, input_4),
		
		BytesPrimitive4::BytesAppend =>
			return bytes_append_4 (input_1, input_2, input_3, input_4),
		
		BytesPrimitive4::BytesSliceFill =>
			fail_unimplemented! (0xb1cb6d46),
		
		BytesPrimitive4::BytesSliceCopy =>
			fail_unimplemented! (0x27459191),
		
	}
}




pub fn bytes_primitive_5_evaluate (primitive : BytesPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		BytesPrimitive5::BytesSliceCopy =>
			fail_unimplemented! (0xc92234a6),
		
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
		
		BytesPrimitiveN::BytesSliceFill =>
			fail_unimplemented! (0xdb534eb6),
		
		BytesPrimitiveN::BytesSliceCopy =>
			fail_unimplemented! (0xe7a2e534),
		
		BytesPrimitiveN::BytesSliceClone =>
			if inputs_count == 1 {
				return bytes_primitive_1_evaluate (BytesPrimitive1::BytesClone, &inputs[0]);
			} else {
				fail_unimplemented! (0x2e876257);
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
		BytesPrimitiveN::BytesSliceFill =>
			None,
		BytesPrimitiveN::BytesSliceCopy =>
			None,
		BytesPrimitiveN::BytesSliceClone =>
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
		BytesPrimitiveN::BytesSliceFill =>
			None,
		BytesPrimitiveN::BytesSliceCopy =>
			None,
		BytesPrimitiveN::BytesSliceClone =>
			Some (BytesPrimitive1::BytesClone),
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
		BytesPrimitiveN::BytesSliceFill =>
			Some (BytesPrimitive2::BytesFill),
		BytesPrimitiveN::BytesSliceCopy =>
			Some (BytesPrimitive2::BytesCopy),
		BytesPrimitiveN::BytesSliceClone =>
			Some (BytesPrimitive2::BytesSliceClone),
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
		BytesPrimitiveN::BytesSliceFill =>
			Some (BytesPrimitive3::BytesSliceFill),
		BytesPrimitiveN::BytesSliceCopy =>
			Some (BytesPrimitive3::BytesSliceCopy),
		BytesPrimitiveN::BytesSliceClone =>
			Some (BytesPrimitive3::BytesSliceClone),
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
		BytesPrimitiveN::BytesSliceFill =>
			Some (BytesPrimitive4::BytesSliceFill),
		BytesPrimitiveN::BytesSliceCopy =>
			Some (BytesPrimitive4::BytesSliceCopy),
		BytesPrimitiveN::BytesSliceClone =>
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
		BytesPrimitiveN::BytesSliceFill =>
			None,
		BytesPrimitiveN::BytesSliceCopy =>
			Some (BytesPrimitive5::BytesSliceCopy),
		BytesPrimitiveN::BytesSliceClone =>
			None,
	}
}

