

use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ArrayPrimitive0;
	pub use super::ArrayPrimitive1;
	pub use super::ArrayPrimitive2;
	pub use super::ArrayPrimitive3;
	pub use super::ArrayPrimitive4;
	pub use super::ArrayPrimitive5;
	pub use super::ArrayPrimitiveN;
	
	pub use super::array_primitive_0_evaluate;
	pub use super::array_primitive_1_evaluate;
	pub use super::array_primitive_2_evaluate;
	pub use super::array_primitive_3_evaluate;
	pub use super::array_primitive_4_evaluate;
	pub use super::array_primitive_5_evaluate;
	pub use super::array_primitive_n_evaluate;
	
	pub use super::array_primitive_n_alternative_0;
	pub use super::array_primitive_n_alternative_1;
	pub use super::array_primitive_n_alternative_2;
	pub use super::array_primitive_n_alternative_3;
	pub use super::array_primitive_n_alternative_4;
	pub use super::array_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive0 {
	
	ArrayBuild,
	ArrayAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive1 {
	
	ArrayLength,
	ArrayClone,
	ArrayReverse,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive2 {
	
	ArrayAt,
	
	ArrayMake,
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayFill,
	ArrayCopy,
	ArrayRangeClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive3 {
	
	ArrayAtSet,
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	ArrayRangeClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive4 {
	
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitive5 {
	
	ArrayRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ArrayPrimitiveN {
	
	ArrayMake,
	ArrayBuild,
	ArrayAppend,
	
	ArrayRangeFill,
	ArrayRangeCopy,
	ArrayRangeClone,
	
}




pub fn array_primitive_0_evaluate (primitive : ArrayPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive0::ArrayBuild =>
			succeed! (array_empty ()),
		
		ArrayPrimitive0::ArrayAppend =>
			succeed! (array_empty ()),
		
	}
}




pub fn array_primitive_1_evaluate (primitive : ArrayPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive1::ArrayLength => {
			let length = try! (array_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		ArrayPrimitive1::ArrayClone =>
			return array_clone (input_1),
		
		ArrayPrimitive1::ArrayReverse =>
			return array_reverse (input_1),
		
		ArrayPrimitive1::ArrayMake =>
			return array_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		ArrayPrimitive1::ArrayBuild =>
			succeed! (array_build_1 (input_1)),
		
		ArrayPrimitive1::ArrayAppend =>
			return array_clone (input_1),
		
	}
}




pub fn array_primitive_2_evaluate (primitive : ArrayPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive2::ArrayAt =>
			return array_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ArrayPrimitive2::ArrayMake =>
			return array_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2)),
		
		ArrayPrimitive2::ArrayBuild =>
			succeed! (array_build_2 (input_1, input_2)),
		
		ArrayPrimitive2::ArrayAppend =>
			return array_append_2 (input_1, input_2),
		
		ArrayPrimitive2::ArrayFill =>
			fail_unimplemented! (0xda8e037a),
		
		ArrayPrimitive2::ArrayCopy =>
			fail_unimplemented! (0x8ac08433),
		
		ArrayPrimitive2::ArrayRangeClone =>
			fail_unimplemented! (0x091fd07f),
		
	}
}




pub fn array_primitive_3_evaluate (primitive : ArrayPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive3::ArrayAtSet =>
			return array_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ArrayPrimitive3::ArrayBuild =>
			succeed! (array_build_3 (input_1, input_2, input_3)),
		
		ArrayPrimitive3::ArrayAppend =>
			return array_append_3 (input_1, input_2, input_3),
		
		ArrayPrimitive3::ArrayRangeFill =>
			fail_unimplemented! (0xb2562068),
		
		ArrayPrimitive3::ArrayRangeCopy =>
			fail_unimplemented! (0x956afb85),
		
		ArrayPrimitive3::ArrayRangeClone =>
			fail_unimplemented! (0x543abc34),
		
	}
}




pub fn array_primitive_4_evaluate (primitive : ArrayPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive4::ArrayBuild =>
			succeed! (array_build_4 (input_1, input_2, input_3, input_4)),
		
		ArrayPrimitive4::ArrayAppend =>
			return array_append_4 (input_1, input_2, input_3, input_4),
		
		ArrayPrimitive4::ArrayRangeFill =>
			fail_unimplemented! (0x0ee45a9c),
		
		ArrayPrimitive4::ArrayRangeCopy =>
			fail_unimplemented! (0x6c892fe2),
		
	}
}




pub fn array_primitive_5_evaluate (primitive : ArrayPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ArrayPrimitive5::ArrayRangeCopy =>
			fail_unimplemented! (0x0cf03a9c),
		
	}
}




pub fn array_primitive_n_evaluate (primitive : ArrayPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		ArrayPrimitiveN::ArrayMake =>
			match inputs_count {
				1 =>
					return array_primitive_1_evaluate (ArrayPrimitive1::ArrayMake, &inputs[0]),
				2 =>
					return array_primitive_2_evaluate (ArrayPrimitive2::ArrayMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0xdd5940d5),
			},
		
		ArrayPrimitiveN::ArrayBuild =>
			match inputs_count {
				0 =>
					return array_primitive_0_evaluate (ArrayPrimitive0::ArrayBuild),
				1 =>
					return array_primitive_1_evaluate (ArrayPrimitive1::ArrayBuild, &inputs[0]),
				2 =>
					return array_primitive_2_evaluate (ArrayPrimitive2::ArrayBuild, &inputs[0], &inputs[1]),
				3 =>
					return array_primitive_3_evaluate (ArrayPrimitive3::ArrayBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return array_primitive_4_evaluate (ArrayPrimitive4::ArrayBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					succeed! (array_build_n (inputs)),
			},
		
		ArrayPrimitiveN::ArrayAppend =>
			match inputs_count {
				0 =>
					return array_primitive_0_evaluate (ArrayPrimitive0::ArrayAppend),
				1 =>
					return array_primitive_1_evaluate (ArrayPrimitive1::ArrayAppend, &inputs[0]),
				2 =>
					return array_primitive_2_evaluate (ArrayPrimitive2::ArrayAppend, &inputs[0], &inputs[1]),
				3 =>
					return array_primitive_3_evaluate (ArrayPrimitive3::ArrayAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return array_primitive_4_evaluate (ArrayPrimitive4::ArrayAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return array_append_n (inputs),
			},
		
		ArrayPrimitiveN::ArrayRangeFill =>
			fail_unimplemented! (0xe9fd172d),
		
		ArrayPrimitiveN::ArrayRangeCopy =>
			fail_unimplemented! (0xa591cae9),
		
		ArrayPrimitiveN::ArrayRangeClone =>
			if inputs_count == 1 {
				return array_primitive_1_evaluate (ArrayPrimitive1::ArrayClone, &inputs[0]);
			} else {
				fail_unimplemented! (0x4fbc2e34);
			},
		
	}
}




pub fn array_primitive_n_alternative_0 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive0>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			None,
		ArrayPrimitiveN::ArrayBuild =>
			Some (ArrayPrimitive0::ArrayBuild),
		ArrayPrimitiveN::ArrayAppend =>
			Some (ArrayPrimitive0::ArrayAppend),
		ArrayPrimitiveN::ArrayRangeFill =>
			None,
		ArrayPrimitiveN::ArrayRangeCopy =>
			None,
		ArrayPrimitiveN::ArrayRangeClone =>
			None,
	}
}


pub fn array_primitive_n_alternative_1 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive1>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			Some (ArrayPrimitive1::ArrayMake),
		ArrayPrimitiveN::ArrayBuild =>
			Some (ArrayPrimitive1::ArrayBuild),
		ArrayPrimitiveN::ArrayAppend =>
			Some (ArrayPrimitive1::ArrayAppend),
		ArrayPrimitiveN::ArrayRangeFill =>
			None,
		ArrayPrimitiveN::ArrayRangeCopy =>
			None,
		ArrayPrimitiveN::ArrayRangeClone =>
			Some (ArrayPrimitive1::ArrayClone),
	}
}


pub fn array_primitive_n_alternative_2 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive2>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			Some (ArrayPrimitive2::ArrayMake),
		ArrayPrimitiveN::ArrayBuild =>
			Some (ArrayPrimitive2::ArrayBuild),
		ArrayPrimitiveN::ArrayAppend =>
			Some (ArrayPrimitive2::ArrayAppend),
		ArrayPrimitiveN::ArrayRangeFill =>
			Some (ArrayPrimitive2::ArrayFill),
		ArrayPrimitiveN::ArrayRangeCopy =>
			Some (ArrayPrimitive2::ArrayCopy),
		ArrayPrimitiveN::ArrayRangeClone =>
			Some (ArrayPrimitive2::ArrayRangeClone),
	}
}


pub fn array_primitive_n_alternative_3 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive3>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			None,
		ArrayPrimitiveN::ArrayBuild =>
			Some (ArrayPrimitive3::ArrayBuild),
		ArrayPrimitiveN::ArrayAppend =>
			Some (ArrayPrimitive3::ArrayAppend),
		ArrayPrimitiveN::ArrayRangeFill =>
			Some (ArrayPrimitive3::ArrayRangeFill),
		ArrayPrimitiveN::ArrayRangeCopy =>
			Some (ArrayPrimitive3::ArrayRangeCopy),
		ArrayPrimitiveN::ArrayRangeClone =>
			Some (ArrayPrimitive3::ArrayRangeClone),
	}
}


pub fn array_primitive_n_alternative_4 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive4>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			None,
		ArrayPrimitiveN::ArrayBuild =>
			Some (ArrayPrimitive4::ArrayBuild),
		ArrayPrimitiveN::ArrayAppend =>
			Some (ArrayPrimitive4::ArrayAppend),
		ArrayPrimitiveN::ArrayRangeFill =>
			Some (ArrayPrimitive4::ArrayRangeFill),
		ArrayPrimitiveN::ArrayRangeCopy =>
			Some (ArrayPrimitive4::ArrayRangeCopy),
		ArrayPrimitiveN::ArrayRangeClone =>
			None,
	}
}


pub fn array_primitive_n_alternative_5 (primitive : ArrayPrimitiveN) -> (Option<ArrayPrimitive5>) {
	match primitive {
		ArrayPrimitiveN::ArrayMake =>
			None,
		ArrayPrimitiveN::ArrayBuild =>
			None,
		ArrayPrimitiveN::ArrayAppend =>
			None,
		ArrayPrimitiveN::ArrayRangeFill =>
			None,
		ArrayPrimitiveN::ArrayRangeCopy =>
			Some (ArrayPrimitive5::ArrayRangeCopy),
		ArrayPrimitiveN::ArrayRangeClone =>
			None,
	}
}

