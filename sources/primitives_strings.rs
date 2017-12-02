

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




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive0 {
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive1 {
	
	StringLength,
	StringClone,
	StringReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	
	StringToList,
	ListToString,
	StringToArray,
	ArrayToString,
	StringToBytes,
	BytesToString,
	
	StringToSymbol,
	SymbolToString,
	StringToNumber,
	NumberToString,
	CharacterToNumber,
	NumberToCharacter,
	
	StringToUpperCase,
	StringToLowerCase,
	StringToFoldCase,
	CharacterToUpperCase,
	CharacterToLowerCase,
	CharacterToFoldCase,
	CharacterToDigitNumber,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive2 {
	
	StringAt,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	StringCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive3 {
	
	StringAtSet,
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	NumberToString,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive4 {
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive5 {
	
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitiveN {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
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
		
		StringPrimitive1::StringToList =>
			return string_range_to_list (input_1, None, None),
		
		StringPrimitive1::ListToString =>
			return list_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToArray =>
			return string_range_to_array (input_1, None, None),
		
		StringPrimitive1::ArrayToString =>
			return array_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToBytes =>
			return string_range_to_bytes (input_1, None, None),
		
		StringPrimitive1::BytesToString =>
			return bytes_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToSymbol =>
			return string_to_symbol (input_1),
		
		StringPrimitive1::SymbolToString =>
			return symbol_to_string (input_1),
		
		StringPrimitive1::StringToNumber =>
			return string_to_number (input_1, None),
		
		StringPrimitive1::NumberToString =>
			return number_to_string (input_1, None, None),
		
		StringPrimitive1::CharacterToNumber =>
			return character_to_number (input_1),
		
		StringPrimitive1::NumberToCharacter =>
			return number_to_character (input_1),
		
		StringPrimitive1::StringToUpperCase =>
			return string_to_upper_case (input_1),
		
		StringPrimitive1::StringToLowerCase =>
			return string_to_lower_case (input_1),
		
		StringPrimitive1::StringToFoldCase =>
			return string_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToUpperCase =>
			return character_to_upper_case (input_1),
		
		StringPrimitive1::CharacterToLowerCase =>
			return character_to_lower_case (input_1),
		
		StringPrimitive1::CharacterToFoldCase =>
			return character_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, None),
		
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
		
		StringPrimitive2::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), None),
		
		StringPrimitive2::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), None),
		
		StringPrimitive2::ArrayRangeToString =>
			return array_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), None),
		
		StringPrimitive2::BytesRangeToString =>
			return bytes_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringToNumber =>
			return string_to_number (input_1, Some (input_2)),
		
		StringPrimitive2::NumberToString =>
			return number_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, Some (input_2)),
		
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
		
		StringPrimitive3::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::ArrayRangeToString =>
			return array_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::BytesRangeToString =>
			return bytes_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::NumberToString =>
			return number_to_string (input_1, Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())),
		
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
		
		StringPrimitiveN::StringRangeToList =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringToList, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringRangeToList, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeToList, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x273584ff),
			},
		
		StringPrimitiveN::ListRangeToString =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::ListToString, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::ListRangeToString, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::ListRangeToString, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xd0a9123a),
			},
		
		StringPrimitiveN::StringRangeToArray =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringToArray, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringRangeToArray, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeToArray, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x091d3683),
			},
		
		StringPrimitiveN::ArrayRangeToString =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::ArrayToString, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::ArrayRangeToString, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::ArrayRangeToString, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x5d6d69b0),
			},
		
		StringPrimitiveN::StringRangeToBytes =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringToBytes, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringRangeToBytes, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::StringRangeToBytes, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xe7f5f988),
			},
		
		StringPrimitiveN::BytesRangeToString =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::BytesToString, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::BytesRangeToString, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::BytesRangeToString, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0xa4900e52),
			},
		
		StringPrimitiveN::StringToNumber =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::StringToNumber, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::StringToNumber, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x3fefec61),
			},
		
		StringPrimitiveN::NumberToString =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::NumberToString, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::NumberToString, &inputs[0], &inputs[1]),
				3 =>
					return string_primitive_3_evaluate (StringPrimitive3::NumberToString, &inputs[0], &inputs[1], &inputs[2]),
				_ =>
					fail! (0x8c1917bf),
			},
		
		StringPrimitiveN::CharacterToDigitNumber =>
			match inputs_count {
				1 =>
					return string_primitive_1_evaluate (StringPrimitive1::CharacterToDigitNumber, &inputs[0]),
				2 =>
					return string_primitive_2_evaluate (StringPrimitive2::CharacterToDigitNumber, &inputs[0], &inputs[1]),
				_ =>
					fail! (0x49dc7b05),
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
		StringPrimitiveN::StringRangeToList =>
			None,
		StringPrimitiveN::ListRangeToString =>
			None,
		StringPrimitiveN::StringRangeToArray =>
			None,
		StringPrimitiveN::ArrayRangeToString =>
			None,
		StringPrimitiveN::StringRangeToBytes =>
			None,
		StringPrimitiveN::BytesRangeToString =>
			None,
		StringPrimitiveN::StringToNumber =>
			None,
		StringPrimitiveN::NumberToString =>
			None,
		StringPrimitiveN::CharacterToDigitNumber =>
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
		StringPrimitiveN::StringRangeToList =>
			Some (StringPrimitive1::StringToList),
		StringPrimitiveN::ListRangeToString =>
			Some (StringPrimitive1::ListToString),
		StringPrimitiveN::StringRangeToArray =>
			Some (StringPrimitive1::StringToArray),
		StringPrimitiveN::ArrayRangeToString =>
			Some (StringPrimitive1::ArrayToString),
		StringPrimitiveN::StringRangeToBytes =>
			Some (StringPrimitive1::StringToBytes),
		StringPrimitiveN::BytesRangeToString =>
			Some (StringPrimitive1::BytesToString),
		StringPrimitiveN::StringToNumber =>
			Some (StringPrimitive1::StringToNumber),
		StringPrimitiveN::NumberToString =>
			Some (StringPrimitive1::NumberToString),
		StringPrimitiveN::CharacterToDigitNumber =>
			Some (StringPrimitive1::CharacterToDigitNumber),
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
		StringPrimitiveN::StringRangeToList =>
			Some (StringPrimitive2::StringRangeToList),
		StringPrimitiveN::ListRangeToString =>
			Some (StringPrimitive2::ListRangeToString),
		StringPrimitiveN::StringRangeToArray =>
			Some (StringPrimitive2::StringRangeToArray),
		StringPrimitiveN::ArrayRangeToString =>
			Some (StringPrimitive2::ArrayRangeToString),
		StringPrimitiveN::StringRangeToBytes =>
			Some (StringPrimitive2::StringRangeToBytes),
		StringPrimitiveN::BytesRangeToString =>
			Some (StringPrimitive2::BytesRangeToString),
		StringPrimitiveN::StringToNumber =>
			Some (StringPrimitive2::StringToNumber),
		StringPrimitiveN::NumberToString =>
			Some (StringPrimitive2::NumberToString),
		StringPrimitiveN::CharacterToDigitNumber =>
			Some (StringPrimitive2::CharacterToDigitNumber),
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
		StringPrimitiveN::StringRangeToList =>
			Some (StringPrimitive3::StringRangeToList),
		StringPrimitiveN::ListRangeToString =>
			Some (StringPrimitive3::ListRangeToString),
		StringPrimitiveN::StringRangeToArray =>
			Some (StringPrimitive3::StringRangeToArray),
		StringPrimitiveN::ArrayRangeToString =>
			Some (StringPrimitive3::ArrayRangeToString),
		StringPrimitiveN::StringRangeToBytes =>
			Some (StringPrimitive3::StringRangeToBytes),
		StringPrimitiveN::BytesRangeToString =>
			Some (StringPrimitive3::BytesRangeToString),
		StringPrimitiveN::StringToNumber =>
			None,
		StringPrimitiveN::NumberToString =>
			Some (StringPrimitive3::NumberToString),
		StringPrimitiveN::CharacterToDigitNumber =>
			None,
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
		StringPrimitiveN::StringRangeToList =>
			None,
		StringPrimitiveN::ListRangeToString =>
			None,
		StringPrimitiveN::StringRangeToArray =>
			None,
		StringPrimitiveN::ArrayRangeToString =>
			None,
		StringPrimitiveN::StringRangeToBytes =>
			None,
		StringPrimitiveN::BytesRangeToString =>
			None,
		StringPrimitiveN::StringToNumber =>
			None,
		StringPrimitiveN::NumberToString =>
			None,
		StringPrimitiveN::CharacterToDigitNumber =>
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
		StringPrimitiveN::StringRangeToList =>
			None,
		StringPrimitiveN::ListRangeToString =>
			None,
		StringPrimitiveN::StringRangeToArray =>
			None,
		StringPrimitiveN::ArrayRangeToString =>
			None,
		StringPrimitiveN::StringRangeToBytes =>
			None,
		StringPrimitiveN::BytesRangeToString =>
			None,
		StringPrimitiveN::StringToNumber =>
			None,
		StringPrimitiveN::NumberToString =>
			None,
		StringPrimitiveN::CharacterToDigitNumber =>
			None,
	}
}

