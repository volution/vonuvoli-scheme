

use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::StringPrimitive0;
	pub use super::StringPrimitive1;
	pub use super::StringPrimitive2;
	pub use super::StringPrimitive3;
	pub use super::StringPrimitive4;
	pub use super::StringPrimitive5;
	pub use super::StringPrimitiveN;
	
	pub use super::string_primitive_0_evaluate;
	pub use super::string_primitive_1_evaluate;
	pub use super::string_primitive_2_evaluate;
	pub use super::string_primitive_3_evaluate;
	pub use super::string_primitive_4_evaluate;
	pub use super::string_primitive_5_evaluate;
	pub use super::string_primitive_n_evaluate;
	
	pub use super::string_primitive_n_alternative_0;
	pub use super::string_primitive_n_alternative_1;
	pub use super::string_primitive_n_alternative_2;
	pub use super::string_primitive_n_alternative_3;
	pub use super::string_primitive_n_alternative_4;
	pub use super::string_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive0 {
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive1 {
	
	StringLength,
	StringClone,
	StringReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive2 {
	
	StringAt,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	StringCopy,
	StringRangeClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive3 {
	
	StringAtSet,
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive4 {
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitive5 {
	
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum StringPrimitiveN {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
}




pub fn string_primitive_0_evaluate (primitive : StringPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive0::StringBuild =>
			succeed! (string_empty ()),
		
		StringPrimitive0::StringAppend =>
			succeed! (string_empty ()),
		
	}
}




pub fn string_primitive_1_evaluate (primitive : StringPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive1::StringLength => {
			let length = try! (string_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		StringPrimitive1::StringClone =>
			return string_clone (input_1),
		
		StringPrimitive1::StringReverse =>
			return string_reverse (input_1),
		
		StringPrimitive1::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		StringPrimitive1::StringBuild =>
			return string_build_1 (input_1),
		
		StringPrimitive1::StringAppend =>
			return string_clone (input_1),
		
		StringPrimitive1::StringFill =>
			return string_fill_range (input_1, None, None, None),
		
	}
}




pub fn string_primitive_2_evaluate (primitive : StringPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive2::StringAt =>
			return string_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		StringPrimitive2::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2)),
		
		StringPrimitive2::StringBuild =>
			return string_build_2 (input_1, input_2),
		
		StringPrimitive2::StringAppend =>
			return string_append_2 (input_1, input_2),
		
		StringPrimitive2::StringFill =>
			return string_fill_range (input_1, Some (input_2), None, None),
		
		StringPrimitive2::StringCopy =>
			return string_copy_range (input_1, None, input_2, None, None),
		
		StringPrimitive2::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), None),
		
	}
}




pub fn string_primitive_3_evaluate (primitive : StringPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive3::StringAtSet =>
			return string_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		StringPrimitive3::StringBuild =>
			return string_build_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringAppend =>
			return string_append_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		StringPrimitive3::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, None, None),
		
		StringPrimitive3::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), Some (input_3)),
		
	}
}




pub fn string_primitive_4_evaluate (primitive : StringPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive4::StringBuild =>
			return string_build_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringAppend =>
			return string_append_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		StringPrimitive4::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




pub fn string_primitive_5_evaluate (primitive : StringPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive5::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




pub fn string_primitive_n_evaluate (primitive : StringPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		StringPrimitiveN::StringMake =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringMake, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x81290cad),
			},
		
		StringPrimitiveN::StringBuild =>
			match inputs_count {
				0 =>
					return string_primitive_0_evaluate (StringPrimitive0::StringBuild),
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringBuild, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringBuild, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return string_build_n (inputs),
			},
		
		StringPrimitiveN::StringAppend =>
			match inputs_count {
				0 =>
					return string_primitive_0_evaluate (StringPrimitive0::StringAppend),
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringAppend, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringAppend, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return string_append_n (inputs),
			},
		
		StringPrimitiveN::StringRangeFill =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringFill, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringFill, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeFill, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringRangeFill, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					fail! (0x04d2afc0),
			},
		
		StringPrimitiveN::StringRangeCopy =>
			match inputs_count {
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringCopy, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeCopy, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return string_primitive_4_evaluate (StringPrimitive4::StringRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				5 =>
					return string_primitive_5_evaluate (StringPrimitive5::StringRangeCopy, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4]),
				_ =>
					fail! (0x8c5e5181),
			},
		
		StringPrimitiveN::StringRangeClone =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringClone, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringRangeClone, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeClone, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x0d49ddab),
			},
		
	}
}




pub fn string_primitive_n_alternative_0 (primitive : StringPrimitiveN) -> (Option<StringPrimitive0>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			None,
		StringPrimitiveN::StringBuild =>
			Some (StringPrimitive0::StringBuild),
		StringPrimitiveN::StringAppend =>
			Some (StringPrimitive0::StringAppend),
		StringPrimitiveN::StringRangeFill =>
			None,
		StringPrimitiveN::StringRangeCopy =>
			None,
		StringPrimitiveN::StringRangeClone =>
			None,
	}
}


pub fn string_primitive_n_alternative_1 (primitive : StringPrimitiveN) -> (Option<StringPrimitive1>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			Some (StringPrimitive1::StringMake),
		StringPrimitiveN::StringBuild =>
			Some (StringPrimitive1::StringBuild),
		StringPrimitiveN::StringAppend =>
			Some (StringPrimitive1::StringAppend),
		StringPrimitiveN::StringRangeFill =>
			Some (StringPrimitive1::StringFill),
		StringPrimitiveN::StringRangeCopy =>
			None,
		StringPrimitiveN::StringRangeClone =>
			Some (StringPrimitive1::StringClone),
	}
}


pub fn string_primitive_n_alternative_2 (primitive : StringPrimitiveN) -> (Option<StringPrimitive2>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			Some (StringPrimitive2::StringMake),
		StringPrimitiveN::StringBuild =>
			Some (StringPrimitive2::StringBuild),
		StringPrimitiveN::StringAppend =>
			Some (StringPrimitive2::StringAppend),
		StringPrimitiveN::StringRangeFill =>
			Some (StringPrimitive2::StringFill),
		StringPrimitiveN::StringRangeCopy =>
			Some (StringPrimitive2::StringCopy),
		StringPrimitiveN::StringRangeClone =>
			Some (StringPrimitive2::StringRangeClone),
	}
}


pub fn string_primitive_n_alternative_3 (primitive : StringPrimitiveN) -> (Option<StringPrimitive3>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			None,
		StringPrimitiveN::StringBuild =>
			Some (StringPrimitive3::StringBuild),
		StringPrimitiveN::StringAppend =>
			Some (StringPrimitive3::StringAppend),
		StringPrimitiveN::StringRangeFill =>
			Some (StringPrimitive3::StringRangeFill),
		StringPrimitiveN::StringRangeCopy =>
			Some (StringPrimitive3::StringRangeCopy),
		StringPrimitiveN::StringRangeClone =>
			Some (StringPrimitive3::StringRangeClone),
	}
}


pub fn string_primitive_n_alternative_4 (primitive : StringPrimitiveN) -> (Option<StringPrimitive4>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			None,
		StringPrimitiveN::StringBuild =>
			Some (StringPrimitive4::StringBuild),
		StringPrimitiveN::StringAppend =>
			Some (StringPrimitive4::StringAppend),
		StringPrimitiveN::StringRangeFill =>
			Some (StringPrimitive4::StringRangeFill),
		StringPrimitiveN::StringRangeCopy =>
			Some (StringPrimitive4::StringRangeCopy),
		StringPrimitiveN::StringRangeClone =>
			None,
	}
}


pub fn string_primitive_n_alternative_5 (primitive : StringPrimitiveN) -> (Option<StringPrimitive5>) {
	match primitive {
		StringPrimitiveN::StringMake =>
			None,
		StringPrimitiveN::StringBuild =>
			None,
		StringPrimitiveN::StringAppend =>
			None,
		StringPrimitiveN::StringRangeFill =>
			None,
		StringPrimitiveN::StringRangeCopy =>
			Some (StringPrimitive5::StringRangeCopy),
		StringPrimitiveN::StringRangeClone =>
			None,
	}
}

