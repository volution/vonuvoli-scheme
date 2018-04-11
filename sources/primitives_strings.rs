

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::StringPrimitive0;
	pub use super::StringPrimitive1;
	pub use super::StringPrimitive2;
	pub use super::StringPrimitive3;
	pub use super::StringPrimitive4;
	pub use super::StringPrimitive5;
	pub use super::StringPrimitiveN;
	pub use super::StringPrimitiveV;
	
	pub use super::string_primitive_0_evaluate;
	pub use super::string_primitive_1_evaluate;
	pub use super::string_primitive_2_evaluate;
	pub use super::string_primitive_3_evaluate;
	pub use super::string_primitive_4_evaluate;
	pub use super::string_primitive_5_evaluate;
	pub use super::string_primitive_n_evaluate;
	
	pub use super::string_primitive_v_alternative_0;
	pub use super::string_primitive_v_alternative_1;
	pub use super::string_primitive_v_alternative_2;
	pub use super::string_primitive_v_alternative_3;
	pub use super::string_primitive_v_alternative_4;
	pub use super::string_primitive_v_alternative_5;
	pub use super::string_primitive_v_alternative_n;
	
	pub use super::string_primitive_0_attributes;
	pub use super::string_primitive_1_attributes;
	pub use super::string_primitive_2_attributes;
	pub use super::string_primitive_3_attributes;
	pub use super::string_primitive_4_attributes;
	pub use super::string_primitive_5_attributes;
	pub use super::string_primitive_n_attributes;
	
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
	StringCloneReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	StringReverse,
	
	StringToList,
	ListToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayToString,
	StringToBytes,
	BytesToString,
	
	StringToSymbol,
	SymbolToString,
	StringToNumber,
	NumberToString,
	CharacterToNumber,
	NumberToCharacter,
	
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	StringToKeyword,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	KeywordToString,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	SymbolToKeyword,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	KeywordToSymbol,
	
	StringToUpperCase,
	StringToLowerCase,
	StringToFoldCase,
	SymbolToUpperCase,
	SymbolToLowerCase,
	SymbolToFoldCase,
	CharacterToUpperCase,
	CharacterToLowerCase,
	CharacterToFoldCase,
	CharacterToDigitNumber,
	
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	KeywordToUpperCase,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	KeywordToLowerCase,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	KeywordToFoldCase,
	
	StringToImmutable,
	StringToMutable,
	
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
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
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
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
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
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitiveV {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_0_evaluate (primitive : StringPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive0::StringBuild =>
			return string_empty () .into_0 (),
		
		StringPrimitive0::StringAppend =>
			return string_empty () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_1_evaluate (primitive : StringPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive1::StringLength =>
			return string_length (input_1) .into_0 (),
		
		StringPrimitive1::StringClone =>
			return string_clone (input_1),
		
		StringPrimitive1::StringCloneReverse =>
			return string_reverse (input_1),
		
		StringPrimitive1::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		StringPrimitive1::StringBuild =>
			return string_build_1 (input_1),
		
		StringPrimitive1::StringAppend =>
			return string_clone (input_1),
		
		StringPrimitive1::StringFill =>
			return string_fill_range (input_1, None, None, None) .into_0 (),
		
		StringPrimitive1::StringReverse =>
			return string_reverse_range (input_1, None, None) .into_0 (),
		
		StringPrimitive1::StringToList =>
			return string_range_to_list (input_1, None, None, None),
		
		StringPrimitive1::ListToString =>
			return list_range_to_string (input_1, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitive1::StringToArray =>
			return string_range_to_array (input_1, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::StringToKeyword =>
			return string_to_keyword (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::KeywordToString =>
			return keyword_to_string (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::SymbolToKeyword =>
			return symbol_to_keyword (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::KeywordToSymbol =>
			return keyword_to_symbol (input_1),
		
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
		
		StringPrimitive1::SymbolToUpperCase =>
			return symbol_to_upper_case (input_1),
		
		StringPrimitive1::SymbolToLowerCase =>
			return symbol_to_lower_case (input_1),
		
		StringPrimitive1::SymbolToFoldCase =>
			return symbol_to_fold_case (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::KeywordToUpperCase =>
			return keyword_to_upper_case (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::KeywordToLowerCase =>
			return keyword_to_lower_case (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		StringPrimitive1::KeywordToFoldCase =>
			return keyword_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToUpperCase =>
			return character_to_upper_case (input_1),
		
		StringPrimitive1::CharacterToLowerCase =>
			return character_to_lower_case (input_1),
		
		StringPrimitive1::CharacterToFoldCase =>
			return character_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, None),
		
		StringPrimitive1::StringToImmutable =>
			return try_as_string_as_ref! (input_1) .to_immutable () .into_0 (),
		
		StringPrimitive1::StringToMutable =>
			return try_as_string_as_ref! (input_1) .to_mutable () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_2_evaluate (primitive : StringPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
			return string_fill_range (input_1, Some (input_2), None, None) .into_0 (),
		
		StringPrimitive2::StringCopy =>
			return string_copy_range (input_1, None, input_2, None, None) .into_0 (),
		
		StringPrimitive2::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeReverse =>
			return string_reverse_range (input_1, Some (input_2), None) .into_0 (),
		
		StringPrimitive2::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), None, None),
		
		StringPrimitive2::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitive2::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_3_evaluate (primitive : StringPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive3::StringAtSet =>
			return string_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		StringPrimitive3::StringBuild =>
			return string_build_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringAppend =>
			return string_append_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		StringPrimitive3::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, None, None) .into_0 (),
		
		StringPrimitive3::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeReverse =>
			return string_reverse_range (input_1, Some (input_2), Some (input_3)) .into_0 (),
		
		StringPrimitive3::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), Some (input_3), None),
		
		StringPrimitive3::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitive3::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), Some (input_3)),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_4_evaluate (primitive : StringPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive4::StringBuild =>
			return string_build_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringAppend =>
			return string_append_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)) .into_0 (),
		
		StringPrimitive4::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_5_evaluate (primitive : StringPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive5::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_n_evaluate (primitive : StringPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitiveN::StringBuild =>
			return string_build_n (inputs),
		
		StringPrimitiveN::StringAppend =>
			return string_append_n (inputs),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_0 (primitive : StringPrimitiveV) -> (Option<StringPrimitive0>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive0::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive0::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			None,
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeReverse =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_1 (primitive : StringPrimitiveV) -> (Option<StringPrimitive1>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			Some (StringPrimitive1::StringMake),
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive1::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive1::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive1::StringFill),
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive1::StringClone),
		StringPrimitiveV::StringRangeReverse =>
			Some (StringPrimitive1::StringReverse),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive1::StringToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive1::ListToString),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive1::StringToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive1::ArrayToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive1::StringToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive1::BytesToString),
		StringPrimitiveV::StringToNumber =>
			Some (StringPrimitive1::StringToNumber),
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive1::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			Some (StringPrimitive1::CharacterToDigitNumber),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_2 (primitive : StringPrimitiveV) -> (Option<StringPrimitive2>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			Some (StringPrimitive2::StringMake),
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive2::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive2::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive2::StringFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive2::StringCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive2::StringRangeClone),
		StringPrimitiveV::StringRangeReverse =>
			Some (StringPrimitive2::StringRangeReverse),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive2::StringRangeToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive2::ListRangeToString),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive2::StringRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive2::ArrayRangeToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive2::StringRangeToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive2::BytesRangeToString),
		StringPrimitiveV::StringToNumber =>
			Some (StringPrimitive2::StringToNumber),
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive2::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			Some (StringPrimitive2::CharacterToDigitNumber),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_3 (primitive : StringPrimitiveV) -> (Option<StringPrimitive3>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive3::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive3::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive3::StringRangeFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive3::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive3::StringRangeClone),
		StringPrimitiveV::StringRangeReverse =>
			Some (StringPrimitive3::StringRangeReverse),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive3::StringRangeToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive3::ListRangeToString),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive3::StringRangeToArray),
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive3::ArrayRangeToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive3::StringRangeToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive3::BytesRangeToString),
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive3::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_4 (primitive : StringPrimitiveV) -> (Option<StringPrimitive4>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive4::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive4::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive4::StringRangeFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive4::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeReverse =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_5 (primitive : StringPrimitiveV) -> (Option<StringPrimitive5>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			None,
		StringPrimitiveV::StringAppend =>
			None,
		StringPrimitiveV::StringRangeFill =>
			None,
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive5::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeReverse =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_v_alternative_n (primitive : StringPrimitiveV) -> (Option<StringPrimitiveN>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitiveN::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitiveN::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			None,
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeReverse =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::StringRangeToArray =>
			None,
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_0_attributes (_primitive : StringPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_1_attributes (_primitive : StringPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_2_attributes (_primitive : StringPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_3_attributes (_primitive : StringPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_4_attributes (_primitive : StringPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_5_attributes (_primitive : StringPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_n_attributes (_primitive : StringPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

