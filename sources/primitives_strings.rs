

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

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
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::string_primitive_n_attributes;
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive0 {
	
	StringBuild,
	StringAppend,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive1 {
	
	StringLength,
	StringClone,
	StringCloneReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReverse,
	
	StringToList,
	ListToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayToString,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	StringToBytes,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringToImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringToMutable,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexCompile,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive2 {
	
	StringAt,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringCopy,
	StringRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToString,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	StringRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatches,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchExtractFirst,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchExtractAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchExtractAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchPositionFirst,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchPositionAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchPositionAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractFirstAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractFirstAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractFirstAsArray,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractAllAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesExtractAllAsArray,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionFirstAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionFirstAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionFirstAsArray,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionAllAsList,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionAllAsAssoc,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	StringRegexMatchCapturesPositionAllAsArray,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive3 {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringAtSet,
	
	StringBuild,
	StringAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeCopy,
	StringRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToString,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	StringRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRangeToString,
	
	NumberToString,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive4 {
	
	StringBuild,
	StringAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeCopy,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitive5 {
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeCopy,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitiveN {
	
	StringBuild,
	StringAppend,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum StringPrimitiveV {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeFill,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeCopy,
	StringRangeClone,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringRangeReverse,
	
	StringRangeToList,
	ListRangeToString,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	StringRangeToArray,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayRangeToString,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	StringRangeToBytes,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive1::StringFill =>
			return string_fill_range (input_1, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitive1::StringToBytes =>
			return string_range_to_bytes (input_1, None, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive1::StringToImmutable =>
			return try_as_string_as_ref! (input_1) .to_immutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive1::StringToMutable =>
			return try_as_string_as_ref! (input_1) .to_mutable () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive1::StringRegexCompile =>
			return string_regex_compile (input_1) .into_0 (),
		
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
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive2::StringFill =>
			return string_fill_range (input_1, Some (input_2), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive2::StringCopy =>
			return string_copy_range (input_1, None, input_2, None, None) .into_0 (),
		
		StringPrimitive2::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), None),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitive2::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitive2::BytesRangeToString =>
			return bytes_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringToNumber =>
			return string_to_number (input_1, Some (input_2)),
		
		StringPrimitive2::NumberToString =>
			return number_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, Some (input_2)),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatches =>
			return string_regex_matches (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchExtractFirst =>
			return string_regex_match_extract_first (input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchExtractAllAsList =>
			return string_regex_match_extract_all (input_1, input_2, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchExtractAllAsArray =>
			return string_regex_match_extract_all (input_1, input_2, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchPositionFirst =>
			return string_regex_match_position_first (input_1, input_2),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchPositionAllAsList =>
			return string_regex_match_position_all (input_1, input_2, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchPositionAllAsArray =>
			return string_regex_match_position_all (input_1, input_2, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractFirstAsList =>
			return string_regex_match_captures_extract_first (input_1, input_2, false, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractFirstAsAssoc =>
			return string_regex_match_captures_extract_first (input_1, input_2, false, true, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractFirstAsArray =>
			return string_regex_match_captures_extract_first (input_1, input_2, true, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractAllAsList =>
			return string_regex_match_captures_extract_all (input_1, input_2, false, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractAllAsAssoc =>
			return string_regex_match_captures_extract_all (input_1, input_2, false, true, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesExtractAllAsArray =>
			return string_regex_match_captures_extract_all (input_1, input_2, true, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionFirstAsList =>
			return string_regex_match_captures_position_first (input_1, input_2, false, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionFirstAsAssoc =>
			return string_regex_match_captures_position_first (input_1, input_2, false, true, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionFirstAsArray =>
			return string_regex_match_captures_position_first (input_1, input_2, true, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionAllAsList =>
			return string_regex_match_captures_position_all (input_1, input_2, false, false, false),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionAllAsAssoc =>
			return string_regex_match_captures_position_all (input_1, input_2, false, true, true),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		StringPrimitive2::StringRegexMatchCapturesPositionAllAsArray =>
			return string_regex_match_captures_position_all (input_1, input_2, true, false, false),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_3_evaluate (primitive : StringPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive3::StringAtSet =>
			return string_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		StringPrimitive3::StringBuild =>
			return string_build_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringAppend =>
			return string_append_3 (input_1, input_2, input_3),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive3::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive3::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, None, None) .into_0 (),
		
		StringPrimitive3::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), Some (input_3)),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitive3::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), Some (input_3)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive4::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive4::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_5_evaluate (primitive : StringPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitive5::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_n_evaluate (primitive : StringPrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive1::StringFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive1::StringClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive1::StringToBytes),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive2::StringFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive2::StringCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive2::StringRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive2::StringRangeToBytes),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive3::StringRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive3::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive3::StringRangeClone),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive3::StringRangeToBytes),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive4::StringRangeFill),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive4::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive5::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeFill =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			None,
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		StringPrimitiveV::StringRangeToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_0_attributes (_primitive : StringPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_1_attributes (_primitive : StringPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_2_attributes (_primitive : StringPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_3_attributes (_primitive : StringPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_4_attributes (_primitive : StringPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_5_attributes (_primitive : StringPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_primitive_n_attributes (_primitive : StringPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

