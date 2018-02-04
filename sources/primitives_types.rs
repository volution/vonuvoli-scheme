

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::TypePrimitive0;
	pub use super::TypePrimitive1;
	pub use super::TypePrimitive2;
	pub use super::TypePrimitive3;
	pub use super::TypePrimitive4;
	pub use super::TypePrimitive5;
	pub use super::TypePrimitiveN;
	pub use super::TypePrimitiveV;
	
	pub use super::type_primitive_0_evaluate;
	pub use super::type_primitive_1_evaluate;
	pub use super::type_primitive_1_evaluate_0;
	pub use super::type_primitive_2_evaluate;
	pub use super::type_primitive_3_evaluate;
	pub use super::type_primitive_4_evaluate;
	pub use super::type_primitive_5_evaluate;
	pub use super::type_primitive_n_evaluate;
	
	pub use super::type_primitive_v_alternative_0;
	pub use super::type_primitive_v_alternative_1;
	pub use super::type_primitive_v_alternative_2;
	pub use super::type_primitive_v_alternative_3;
	pub use super::type_primitive_v_alternative_4;
	pub use super::type_primitive_v_alternative_5;
	pub use super::type_primitive_v_alternative_n;
	
	pub use super::type_primitive_0_attributes;
	pub use super::type_primitive_1_attributes;
	pub use super::type_primitive_2_attributes;
	pub use super::type_primitive_3_attributes;
	pub use super::type_primitive_4_attributes;
	pub use super::type_primitive_5_attributes;
	pub use super::type_primitive_n_attributes;
	
}




macro_rules! def_type_primitive_enum {
	( $identifier : ident ) => (
		
		#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
		pub enum $identifier {
			
			IsNull,
			IsNullNot,
			IsVoid,
			IsVoidNot,
			IsUndefined,
			IsUndefinedNot,
			
			IsBoolean,
			IsTrue,
			IsTrueNot,
			IsFalse,
			IsFalseNot,
			IsTrueOrEquivalent,
			IsFalseOrEquivalent,
			
			IsNumber,
			IsNumberInteger,
			IsNumberRational,
			IsNumberReal,
			IsNumberComplex,
			IsNumberExact,
			IsNumberExactInteger,
			IsNumberInexact,
			
			IsCharacter,
			
			IsSymbol,
			IsKeyword,
			IsUnique,
			
			IsStringRegex,
			
			IsString,
			IsStringImmutable,
			IsStringMutable,
			IsStringEmpty,
			IsStringImmutableEmpty,
			IsStringMutableEmpty,
			IsStringEmptyNot,
			IsStringImmutableEmptyNot,
			IsStringMutableEmptyNot,
			
			IsBytes,
			IsBytesImmutable,
			IsBytesMutable,
			IsBytesEmpty,
			IsBytesImmutableEmpty,
			IsBytesMutableEmpty,
			IsBytesEmptyNot,
			IsBytesImmutableEmptyNot,
			IsBytesMutableEmptyNot,
			
			IsPair,
			IsPairMutable,
			IsPairImmutable,
			
			IsArray,
			IsArrayMutable,
			IsArrayImmutable,
			IsArrayEmpty,
			IsArrayMutableEmpty,
			IsArrayImmutableEmpty,
			IsArrayEmptyNot,
			IsArrayMutableEmptyNot,
			IsArrayImmutableEmptyNot,
			
			IsValues,
			IsValuesEmpty,
			IsValuesEmptyNot,
			
			IsRecordKind,
			IsRecord,
			IsRecordImmutable,
			IsRecordMutable,
			
			IsError,
			IsErrorSyntax,
			IsErrorFile,
			IsErrorPort,
			IsErrorPortInput,
			IsErrorPortOutput,
			
			IsList,
			IsListProper,
			IsListProperOrEmpty,
			IsListDotted,
			IsListDottedOrEmpty,
			IsListCyclic,
			IsListCyclicOrEmpty,
			
			IsProcedure,
			IsSyntax,
			
			IsPort,
			IsPortInput,
			IsPortOutput,
			IsPortBinary,
			IsPortTextual,
			IsPortInputBinary,
			IsPortInputTextual,
			IsPortOutputBinary,
			IsPortOutputTextual,
			IsPortEof,
			
			IsProcess,
			IsContext,
			IsBinding,
			IsParameters,
			IsParameter,
			IsPromise,
			
			IsResource,
			IsInternal,
			IsOpaque,
			
			IsNumberZero,
			IsNumberPositive,
			IsNumberNegative,
			IsNumberFinite,
			IsNumberInfinite,
			IsNumberNan,
			IsNumberEven,
			IsNumberOdd,
			
			IsCharacterNumeric,
			IsCharacterAlphabetic,
			IsCharacterAlphabeticUpperCase,
			IsCharacterAlphabeticLowerCase,
			IsCharacterAlphabeticOrNumeric,
			IsCharacterWhitespace,
			IsCharacterControl,
			IsCharacterAscii,
			IsCharacterAsciiNumeric,
			IsCharacterAsciiNumericBase8,
			IsCharacterAsciiNumericBase16,
			IsCharacterAsciiAlphabetic,
			IsCharacterAsciiAlphabeticUpperCase,
			IsCharacterAsciiAlphabeticLowerCase,
			IsCharacterAsciiAlphabeticOrNumeric,
			IsCharacterAsciiWhitespace,
			IsCharacterAsciiControl,
			IsCharacterAsciiPunctuation,
			IsCharacterAsciiGraphic,
			
		}
		
	);
}




def_type_primitive_enum! (TypePrimitive1);
def_type_primitive_enum! (TypePrimitive2);
def_type_primitive_enum! (TypePrimitive3);
def_type_primitive_enum! (TypePrimitive4);
def_type_primitive_enum! (TypePrimitiveN);
def_type_primitive_enum! (TypePrimitiveV);


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum TypePrimitive0 {}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum TypePrimitive5 {}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_0_evaluate (primitive : TypePrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	return type_primitive_1_evaluate_0 (primitive, input_1) .into_0 ();
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_evaluate_0 (primitive : TypePrimitive1, input_1 : &Value) -> (Outcome<bool>) {
	match primitive {
		
		TypePrimitive1::IsNull =>
			return is_null (input_1) .into_0 (),
		
		TypePrimitive1::IsNullNot =>
			return is_not_null (input_1) .into_0 (),
		
		TypePrimitive1::IsVoid =>
			return is_void (input_1) .into_0 (),
		
		TypePrimitive1::IsVoidNot =>
			return is_not_void (input_1) .into_0 (),
		
		TypePrimitive1::IsUndefined =>
			return is_undefined (input_1) .into_0 (),
		
		TypePrimitive1::IsUndefinedNot =>
			return is_not_undefined (input_1) .into_0 (),
		
		TypePrimitive1::IsBoolean =>
			return is_boolean (input_1) .into_0 (),
		
		TypePrimitive1::IsTrue =>
			return is_true (input_1) .into_0 (),
		
		TypePrimitive1::IsTrueNot =>
			return is_not_true (input_1) .into_0 (),
		
		TypePrimitive1::IsFalse =>
			return is_false (input_1) .into_0 (),
		
		TypePrimitive1::IsFalseNot =>
			return is_not_false (input_1) .into_0 (),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			return is_true_or_equivalent (input_1) .into_0 (),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			return is_false_or_equivalent (input_1) .into_0 (),
		
		TypePrimitive1::IsNumber =>
			return is_number (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInteger =>
			return is_number_integer (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberRational =>
			return is_number_rational (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberReal =>
			return is_number_real (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberComplex =>
			return is_number_complex (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberExact =>
			return is_number_exact (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberExactInteger =>
			return is_number_exact_integer (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInexact =>
			return is_number_inexact (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacter =>
			return is_character (input_1) .into_0 (),
		
		TypePrimitive1::IsSymbol =>
			return is_symbol (input_1) .into_0 (),
		
		TypePrimitive1::IsKeyword =>
			return is_keyword (input_1) .into_0 (),
		
		TypePrimitive1::IsUnique =>
			return is_unique (input_1) .into_0 (),
		
		TypePrimitive1::IsStringRegex =>
			return is_string_regex (input_1) .into_0 (),
		
		TypePrimitive1::IsString =>
			return is_string (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutable =>
			return is_string_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutable =>
			return is_string_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsStringEmpty =>
			return is_string_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutableEmpty =>
			return is_string_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutableEmpty =>
			return is_string_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringEmptyNot =>
			return is_string_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytes =>
			return is_bytes (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutable =>
			return is_bytes_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutable =>
			return is_bytes_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesEmpty =>
			return is_bytes_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesEmptyNot =>
			return is_bytes_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsPair =>
			return is_pair (input_1) .into_0 (),
		
		TypePrimitive1::IsPairImmutable =>
			return is_pair_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsPairMutable =>
			return is_pair_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArray =>
			return is_array (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutable =>
			return is_array_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutable =>
			return is_array_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayEmpty =>
			return is_array_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutableEmpty =>
			return is_array_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutableEmpty =>
			return is_array_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayEmptyNot =>
			return is_array_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsValues =>
			return is_values (input_1) .into_0 (),
		
		TypePrimitive1::IsValuesEmpty =>
			return is_values_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsValuesEmptyNot =>
			return is_values_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordKind =>
			return is_record_kind (input_1) .into_0 (),
		
		TypePrimitive1::IsRecord =>
			return is_record (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordImmutable =>
			return is_record_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordMutable =>
			return is_record_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsError =>
			return is_error (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorSyntax =>
			return is_error_syntax (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorFile =>
			return is_error_file (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPort =>
			return is_error_port (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPortInput =>
			return is_error_port_input (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPortOutput =>
			return is_error_port_output (input_1) .into_0 (),
		
		TypePrimitive1::IsList =>
			return is_list (input_1) .into_0 (),
		
		TypePrimitive1::IsListProper =>
			return is_list_proper (input_1) .into_0 (),
		
		TypePrimitive1::IsListProperOrEmpty =>
			return is_list_proper_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsListDotted =>
			return is_list_dotted (input_1) .into_0 (),
		
		TypePrimitive1::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsListCyclic =>
			return is_list_cyclic (input_1) .into_0 (),
		
		TypePrimitive1::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsProcedure =>
			return is_procedure (input_1) .into_0 (),
		
		TypePrimitive1::IsSyntax =>
			return is_syntax (input_1) .into_0 (),
		
		TypePrimitive1::IsPort =>
			return is_port (input_1) .into_0 (),
		
		TypePrimitive1::IsPortInput =>
			return is_port_input (input_1) .into_0 (),
		
		TypePrimitive1::IsPortOutput =>
			return is_port_output (input_1) .into_0 (),
		
		TypePrimitive1::IsPortBinary =>
			return is_port_binary (input_1) .into_0 (),
		
		TypePrimitive1::IsPortTextual =>
			return is_port_textual (input_1) .into_0 (),
		
		TypePrimitive1::IsPortInputBinary =>
			return is_port_input_binary (input_1) .into_0 (),
		
		TypePrimitive1::IsPortInputTextual =>
			return is_port_input_textual (input_1) .into_0 (),
		
		TypePrimitive1::IsPortOutputBinary =>
			return is_port_output_binary (input_1) .into_0 (),
		
		TypePrimitive1::IsPortOutputTextual =>
			return is_port_output_textual (input_1) .into_0 (),
		
		TypePrimitive1::IsPortEof =>
			return is_port_eof (input_1) .into_0 (),
		
		TypePrimitive1::IsProcess =>
			return is_process (input_1) .into_0 (),
		
		TypePrimitive1::IsContext =>
			return is_context (input_1) .into_0 (),
		
		TypePrimitive1::IsBinding =>
			return is_binding (input_1) .into_0 (),
		
		TypePrimitive1::IsParameters =>
			return is_parameters (input_1) .into_0 (),
		
		TypePrimitive1::IsParameter =>
			return is_parameter (input_1) .into_0 (),
		
		TypePrimitive1::IsPromise =>
			return is_promise (input_1) .into_0 (),
		
		TypePrimitive1::IsResource =>
			return is_resource (input_1) .into_0 (),
		
		TypePrimitive1::IsInternal =>
			return is_internal (input_1) .into_0 (),
		
		TypePrimitive1::IsOpaque =>
			return is_opaque (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberZero =>
			return is_number_zero (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberPositive =>
			return is_number_positive (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberNegative =>
			return is_number_negative (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberFinite =>
			return is_number_finite (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInfinite =>
			return is_number_infinite (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberNan =>
			return is_number_nan (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberEven =>
			return is_number_even (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberOdd =>
			return is_number_odd (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterNumeric =>
			return is_character_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabetic =>
			return is_character_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterWhitespace =>
			return is_character_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterControl =>
			return is_character_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAscii =>
			return is_character_ascii (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiControl =>
			return is_character_ascii_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_evaluate (primitive : TypePrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		TypePrimitive2::IsNull =>
			return is_null_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNullNot =>
			return is_not_null_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsVoid =>
			return is_void_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsVoidNot =>
			return is_not_void_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsUndefined =>
			return is_undefined_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsUndefinedNot =>
			return is_not_undefined_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBoolean =>
			return is_boolean_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsTrue =>
			return is_true_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsTrueNot =>
			return is_not_true_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsFalse =>
			return is_false_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsFalseNot =>
			return is_not_false_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsTrueOrEquivalent =>
			return is_true_or_equivalent_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsFalseOrEquivalent =>
			return is_false_or_equivalent_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumber =>
			return is_number_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberInteger =>
			return is_number_integer_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberRational =>
			return is_number_rational_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberReal =>
			return is_number_real_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberComplex =>
			return is_number_complex_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberExact =>
			return is_number_exact_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberExactInteger =>
			return is_number_exact_integer_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberInexact =>
			return is_number_inexact_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacter =>
			return is_character_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsSymbol =>
			return is_symbol_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsKeyword =>
			return is_keyword_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsUnique =>
			return is_unique_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringRegex =>
			return is_string_regex_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsString =>
			return is_string_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringImmutable =>
			return is_string_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringMutable =>
			return is_string_mutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringEmpty =>
			return is_string_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringEmptyNot =>
			return is_string_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytes =>
			return is_bytes_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesImmutable =>
			return is_bytes_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesMutable =>
			return is_bytes_mutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesEmpty =>
			return is_bytes_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPair =>
			return is_pair_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPairImmutable =>
			return is_pair_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPairMutable =>
			return is_pair_mutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArray =>
			return is_array_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayImmutable =>
			return is_array_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayMutable =>
			return is_array_mutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayEmpty =>
			return is_array_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayEmptyNot =>
			return is_array_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsValues =>
			return is_values_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsValuesEmpty =>
			return is_values_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsValuesEmptyNot =>
			return is_values_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsRecordKind =>
			return is_record_kind_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsRecord =>
			return is_record_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsRecordImmutable =>
			return is_record_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsRecordMutable =>
			return is_record_mutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsError =>
			return is_error_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsErrorSyntax =>
			return is_error_syntax_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsErrorFile =>
			return is_error_file_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsErrorPort =>
			return is_error_port_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsErrorPortInput =>
			return is_error_port_input_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsErrorPortOutput =>
			return is_error_port_output_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsList =>
			return is_list_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListProper =>
			return is_list_proper_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListProperOrEmpty =>
			return is_list_proper_or_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListDotted =>
			return is_list_dotted_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListCyclic =>
			return is_list_cyclic_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsProcedure =>
			return is_procedure_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsSyntax =>
			return is_syntax_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPort =>
			return is_port_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortInput =>
			return is_port_input_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortOutput =>
			return is_port_output_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortBinary =>
			return is_port_binary_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortTextual =>
			return is_port_textual_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortInputBinary =>
			return is_port_input_binary_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortInputTextual =>
			return is_port_input_textual_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortOutputBinary =>
			return is_port_output_binary_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortOutputTextual =>
			return is_port_output_textual_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPortEof =>
			return is_port_eof_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsProcess =>
			return is_process_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsContext =>
			return is_context_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsBinding =>
			return is_binding_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsParameters =>
			return is_parameters_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsParameter =>
			return is_parameter_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPromise =>
			return is_promise_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsResource =>
			return is_resource_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsInternal =>
			return is_internal_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsOpaque =>
			return is_opaque_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberZero =>
			return is_number_zero_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberPositive =>
			return is_number_positive_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberNegative =>
			return is_number_negative_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberFinite =>
			return is_number_finite_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberInfinite =>
			return is_number_infinite_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberNan =>
			return is_number_nan_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberEven =>
			return is_number_even_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsNumberOdd =>
			return is_number_odd_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterNumeric =>
			return is_character_numeric_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterWhitespace =>
			return is_character_whitespace_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterControl =>
			return is_character_control_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAscii =>
			return is_character_ascii_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_2 (input_1, input_2) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_evaluate (primitive : TypePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		TypePrimitive3::IsNull =>
			return is_null_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNullNot =>
			return is_not_null_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsVoid =>
			return is_void_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsVoidNot =>
			return is_not_void_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsUndefined =>
			return is_undefined_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsUndefinedNot =>
			return is_not_undefined_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBoolean =>
			return is_boolean_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsTrue =>
			return is_true_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsTrueNot =>
			return is_not_true_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsFalse =>
			return is_false_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsFalseNot =>
			return is_not_false_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsTrueOrEquivalent =>
			return is_true_or_equivalent_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsFalseOrEquivalent =>
			return is_false_or_equivalent_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumber =>
			return is_number_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberInteger =>
			return is_number_integer_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberRational =>
			return is_number_rational_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberReal =>
			return is_number_real_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberComplex =>
			return is_number_complex_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberExact =>
			return is_number_exact_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberExactInteger =>
			return is_number_exact_integer_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberInexact =>
			return is_number_inexact_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacter =>
			return is_character_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsSymbol =>
			return is_symbol_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsKeyword =>
			return is_keyword_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsUnique =>
			return is_unique_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringRegex =>
			return is_string_regex_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsString =>
			return is_string_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringImmutable =>
			return is_string_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringMutable =>
			return is_string_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringEmpty =>
			return is_string_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringEmptyNot =>
			return is_string_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytes =>
			return is_bytes_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesImmutable =>
			return is_bytes_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesMutable =>
			return is_bytes_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesEmpty =>
			return is_bytes_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPair =>
			return is_pair_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPairImmutable =>
			return is_pair_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPairMutable =>
			return is_pair_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArray =>
			return is_array_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayImmutable =>
			return is_array_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayMutable =>
			return is_array_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayEmpty =>
			return is_array_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayEmptyNot =>
			return is_array_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsValues =>
			return is_values_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsValuesEmpty =>
			return is_values_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsValuesEmptyNot =>
			return is_values_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsRecordKind =>
			return is_record_kind_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsRecord =>
			return is_record_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsRecordImmutable =>
			return is_record_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsRecordMutable =>
			return is_record_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsError =>
			return is_error_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsErrorSyntax =>
			return is_error_syntax_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsErrorFile =>
			return is_error_file_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsErrorPort =>
			return is_error_port_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsErrorPortInput =>
			return is_error_port_input_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsErrorPortOutput =>
			return is_error_port_output_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsList =>
			return is_list_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListProper =>
			return is_list_proper_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListProperOrEmpty =>
			return is_list_proper_or_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListDotted =>
			return is_list_dotted_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListCyclic =>
			return is_list_cyclic_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsProcedure =>
			return is_procedure_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsSyntax =>
			return is_syntax_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPort =>
			return is_port_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortInput =>
			return is_port_input_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortOutput =>
			return is_port_output_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortBinary =>
			return is_port_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortTextual =>
			return is_port_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortInputBinary =>
			return is_port_input_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortInputTextual =>
			return is_port_input_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortOutputBinary =>
			return is_port_output_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortOutputTextual =>
			return is_port_output_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPortEof =>
			return is_port_eof_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsProcess =>
			return is_process_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsContext =>
			return is_context_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsBinding =>
			return is_binding_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsParameters =>
			return is_parameters_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsParameter =>
			return is_parameter_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPromise =>
			return is_promise_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsResource =>
			return is_resource_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsInternal =>
			return is_internal_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsOpaque =>
			return is_opaque_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberZero =>
			return is_number_zero_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberPositive =>
			return is_number_positive_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberNegative =>
			return is_number_negative_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberFinite =>
			return is_number_finite_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberInfinite =>
			return is_number_infinite_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberNan =>
			return is_number_nan_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberEven =>
			return is_number_even_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsNumberOdd =>
			return is_number_odd_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterNumeric =>
			return is_character_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterWhitespace =>
			return is_character_whitespace_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterControl =>
			return is_character_control_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAscii =>
			return is_character_ascii_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_3 (input_1, input_2, input_3) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_evaluate (primitive : TypePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		TypePrimitive4::IsNull =>
			return is_null_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNullNot =>
			return is_not_null_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsVoid =>
			return is_void_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsVoidNot =>
			return is_not_void_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsUndefined =>
			return is_undefined_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsUndefinedNot =>
			return is_not_undefined_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBoolean =>
			return is_boolean_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsTrue =>
			return is_true_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsTrueNot =>
			return is_not_true_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsFalse =>
			return is_false_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsFalseNot =>
			return is_not_false_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsTrueOrEquivalent =>
			return is_true_or_equivalent_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsFalseOrEquivalent =>
			return is_false_or_equivalent_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumber =>
			return is_number_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberInteger =>
			return is_number_integer_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberRational =>
			return is_number_rational_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberReal =>
			return is_number_real_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberComplex =>
			return is_number_complex_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberExact =>
			return is_number_exact_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberExactInteger =>
			return is_number_exact_integer_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberInexact =>
			return is_number_inexact_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacter =>
			return is_character_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsSymbol =>
			return is_symbol_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsKeyword =>
			return is_keyword_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsUnique =>
			return is_unique_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringRegex =>
			return is_string_regex_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsString =>
			return is_string_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringImmutable =>
			return is_string_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringMutable =>
			return is_string_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringEmpty =>
			return is_string_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringEmptyNot =>
			return is_string_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytes =>
			return is_bytes_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesImmutable =>
			return is_bytes_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesMutable =>
			return is_bytes_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesEmpty =>
			return is_bytes_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPair =>
			return is_pair_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPairImmutable =>
			return is_pair_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPairMutable =>
			return is_pair_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArray =>
			return is_array_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayImmutable =>
			return is_array_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayMutable =>
			return is_array_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayEmpty =>
			return is_array_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayEmptyNot =>
			return is_array_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsValues =>
			return is_values_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsValuesEmpty =>
			return is_values_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsValuesEmptyNot =>
			return is_values_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsRecordKind =>
			return is_record_kind_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsRecord =>
			return is_record_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsRecordImmutable =>
			return is_record_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsRecordMutable =>
			return is_record_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsError =>
			return is_error_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsErrorSyntax =>
			return is_error_syntax_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsErrorFile =>
			return is_error_file_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsErrorPort =>
			return is_error_port_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsErrorPortInput =>
			return is_error_port_input_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsErrorPortOutput =>
			return is_error_port_output_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsList =>
			return is_list_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListProper =>
			return is_list_proper_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListProperOrEmpty =>
			return is_list_proper_or_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListDotted =>
			return is_list_dotted_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListCyclic =>
			return is_list_cyclic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsProcedure =>
			return is_procedure_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsSyntax =>
			return is_syntax_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPort =>
			return is_port_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortInput =>
			return is_port_input_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortOutput =>
			return is_port_output_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortBinary =>
			return is_port_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortTextual =>
			return is_port_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortInputBinary =>
			return is_port_input_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortInputTextual =>
			return is_port_input_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortOutputBinary =>
			return is_port_output_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortOutputTextual =>
			return is_port_output_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPortEof =>
			return is_port_eof_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsProcess =>
			return is_process_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsContext =>
			return is_context_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsBinding =>
			return is_binding_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsParameters =>
			return is_parameters_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsParameter =>
			return is_parameter_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPromise =>
			return is_promise_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsResource =>
			return is_resource_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsInternal =>
			return is_internal_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsOpaque =>
			return is_opaque_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberZero =>
			return is_number_zero_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberPositive =>
			return is_number_positive_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberNegative =>
			return is_number_negative_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberFinite =>
			return is_number_finite_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberInfinite =>
			return is_number_infinite_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberNan =>
			return is_number_nan_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberEven =>
			return is_number_even_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsNumberOdd =>
			return is_number_odd_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterNumeric =>
			return is_character_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterWhitespace =>
			return is_character_whitespace_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterControl =>
			return is_character_control_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAscii =>
			return is_character_ascii_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_5_evaluate (primitive : TypePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_evaluate (primitive : TypePrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		TypePrimitiveN::IsNull =>
			return is_null_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNullNot =>
			return is_not_null_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsVoid =>
			return is_void_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsVoidNot =>
			return is_not_void_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsUndefined =>
			return is_undefined_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsUndefinedNot =>
			return is_not_undefined_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBoolean =>
			return is_boolean_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsTrue =>
			return is_true_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsTrueNot =>
			return is_not_true_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsFalse =>
			return is_false_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsFalseNot =>
			return is_not_false_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsTrueOrEquivalent =>
			return is_true_or_equivalent_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsFalseOrEquivalent =>
			return is_false_or_equivalent_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumber =>
			return is_number_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberInteger =>
			return is_number_integer_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberRational =>
			return is_number_rational_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberReal =>
			return is_number_real_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberComplex =>
			return is_number_complex_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberExact =>
			return is_number_exact_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberExactInteger =>
			return is_number_exact_integer_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberInexact =>
			return is_number_inexact_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacter =>
			return is_character_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsSymbol =>
			return is_symbol_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsKeyword =>
			return is_keyword_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsUnique =>
			return is_unique_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringRegex =>
			return is_string_regex_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsString =>
			return is_string_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringImmutable =>
			return is_string_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringMutable =>
			return is_string_mutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringEmpty =>
			return is_string_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringEmptyNot =>
			return is_string_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytes =>
			return is_bytes_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesImmutable =>
			return is_bytes_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesMutable =>
			return is_bytes_mutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesEmpty =>
			return is_bytes_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPair =>
			return is_pair_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPairImmutable =>
			return is_pair_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPairMutable =>
			return is_pair_mutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArray =>
			return is_array_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayImmutable =>
			return is_array_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayMutable =>
			return is_array_mutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayEmpty =>
			return is_array_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayEmptyNot =>
			return is_array_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsValues =>
			return is_values_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsValuesEmpty =>
			return is_values_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsValuesEmptyNot =>
			return is_values_not_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsRecordKind =>
			return is_record_kind_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsRecord =>
			return is_record_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsRecordImmutable =>
			return is_record_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsRecordMutable =>
			return is_record_mutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsError =>
			return is_error_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsErrorSyntax =>
			return is_error_syntax_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsErrorFile =>
			return is_error_file_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsErrorPort =>
			return is_error_port_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsErrorPortInput =>
			return is_error_port_input_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsErrorPortOutput =>
			return is_error_port_output_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsList =>
			return is_list_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListProper =>
			return is_list_proper_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListProperOrEmpty =>
			return is_list_proper_or_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListDotted =>
			return is_list_dotted_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListCyclic =>
			return is_list_cyclic_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsProcedure =>
			return is_procedure_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsSyntax =>
			return is_syntax_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPort =>
			return is_port_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortInput =>
			return is_port_input_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortOutput =>
			return is_port_output_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortBinary =>
			return is_port_binary_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortTextual =>
			return is_port_textual_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortInputBinary =>
			return is_port_input_binary_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortInputTextual =>
			return is_port_input_textual_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortOutputBinary =>
			return is_port_output_binary_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortOutputTextual =>
			return is_port_output_textual_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPortEof =>
			return is_port_eof_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsProcess =>
			return is_process_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsContext =>
			return is_context_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsBinding =>
			return is_binding_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsParameters =>
			return is_parameters_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsParameter =>
			return is_parameter_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPromise =>
			return is_promise_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsResource =>
			return is_resource_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsInternal =>
			return is_internal_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsOpaque =>
			return is_opaque_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberZero =>
			return is_number_zero_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberPositive =>
			return is_number_positive_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberNegative =>
			return is_number_negative_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberFinite =>
			return is_number_finite_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberInfinite =>
			return is_number_infinite_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberNan =>
			return is_number_nan_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberEven =>
			return is_number_even_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsNumberOdd =>
			return is_number_odd_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterNumeric =>
			return is_character_numeric_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterWhitespace =>
			return is_character_whitespace_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterControl =>
			return is_character_control_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAscii =>
			return is_character_ascii_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_n (inputs) .into_0 (),
		
	}
}




macro_rules! def_type_primitive_v_alternative_fn {
	( $identifier : ident, $alternative : ident ) => (
		
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $identifier (primitive : TypePrimitiveV) -> (Option<$alternative>) {
			match primitive {
				TypePrimitiveV::IsNull =>
					Some ($alternative::IsNull),
				TypePrimitiveV::IsNullNot =>
					Some ($alternative::IsNullNot),
				TypePrimitiveV::IsVoid =>
					Some ($alternative::IsVoid),
				TypePrimitiveV::IsVoidNot =>
					Some ($alternative::IsVoidNot),
				TypePrimitiveV::IsUndefined =>
					Some ($alternative::IsUndefined),
				TypePrimitiveV::IsUndefinedNot =>
					Some ($alternative::IsUndefinedNot),
				TypePrimitiveV::IsBoolean =>
					Some ($alternative::IsBoolean),
				TypePrimitiveV::IsTrue =>
					Some ($alternative::IsTrue),
				TypePrimitiveV::IsTrueNot =>
					Some ($alternative::IsTrueNot),
				TypePrimitiveV::IsFalse =>
					Some ($alternative::IsFalse),
				TypePrimitiveV::IsFalseNot =>
					Some ($alternative::IsFalseNot),
				TypePrimitiveV::IsTrueOrEquivalent =>
					Some ($alternative::IsTrueOrEquivalent),
				TypePrimitiveV::IsFalseOrEquivalent =>
					Some ($alternative::IsFalseOrEquivalent),
				TypePrimitiveV::IsNumber =>
					Some ($alternative::IsNumber),
				TypePrimitiveV::IsNumberInteger =>
					Some ($alternative::IsNumberInteger),
				TypePrimitiveV::IsNumberRational =>
					Some ($alternative::IsNumberRational),
				TypePrimitiveV::IsNumberReal =>
					Some ($alternative::IsNumberReal),
				TypePrimitiveV::IsNumberComplex =>
					Some ($alternative::IsNumberComplex),
				TypePrimitiveV::IsNumberExact =>
					Some ($alternative::IsNumberExact),
				TypePrimitiveV::IsNumberExactInteger =>
					Some ($alternative::IsNumberExactInteger),
				TypePrimitiveV::IsNumberInexact =>
					Some ($alternative::IsNumberInexact),
				TypePrimitiveV::IsCharacter =>
					Some ($alternative::IsCharacter),
				TypePrimitiveV::IsSymbol =>
					Some ($alternative::IsSymbol),
				TypePrimitiveV::IsKeyword =>
					Some ($alternative::IsKeyword),
				TypePrimitiveV::IsUnique =>
					Some ($alternative::IsUnique),
				TypePrimitiveV::IsStringRegex =>
					Some ($alternative::IsStringRegex),
				TypePrimitiveV::IsString =>
					Some ($alternative::IsString),
				TypePrimitiveV::IsStringImmutable =>
					Some ($alternative::IsStringImmutable),
				TypePrimitiveV::IsStringMutable =>
					Some ($alternative::IsStringMutable),
				TypePrimitiveV::IsStringEmpty =>
					Some ($alternative::IsStringEmpty),
				TypePrimitiveV::IsStringImmutableEmpty =>
					Some ($alternative::IsStringImmutableEmpty),
				TypePrimitiveV::IsStringMutableEmpty =>
					Some ($alternative::IsStringMutableEmpty),
				TypePrimitiveV::IsStringEmptyNot =>
					Some ($alternative::IsStringEmptyNot),
				TypePrimitiveV::IsStringImmutableEmptyNot =>
					Some ($alternative::IsStringImmutableEmptyNot),
				TypePrimitiveV::IsStringMutableEmptyNot =>
					Some ($alternative::IsStringMutableEmptyNot),
				TypePrimitiveV::IsBytes =>
					Some ($alternative::IsBytes),
				TypePrimitiveV::IsBytesImmutable =>
					Some ($alternative::IsBytesImmutable),
				TypePrimitiveV::IsBytesMutable =>
					Some ($alternative::IsBytesMutable),
				TypePrimitiveV::IsBytesEmpty =>
					Some ($alternative::IsBytesEmpty),
				TypePrimitiveV::IsBytesImmutableEmpty =>
					Some ($alternative::IsBytesImmutableEmpty),
				TypePrimitiveV::IsBytesMutableEmpty =>
					Some ($alternative::IsBytesMutableEmpty),
				TypePrimitiveV::IsBytesEmptyNot =>
					Some ($alternative::IsBytesEmptyNot),
				TypePrimitiveV::IsBytesImmutableEmptyNot =>
					Some ($alternative::IsBytesImmutableEmptyNot),
				TypePrimitiveV::IsBytesMutableEmptyNot =>
					Some ($alternative::IsBytesMutableEmptyNot),
				TypePrimitiveV::IsPair =>
					Some ($alternative::IsPair),
				TypePrimitiveV::IsPairMutable =>
					Some ($alternative::IsPairMutable),
				TypePrimitiveV::IsPairImmutable =>
					Some ($alternative::IsPairImmutable),
				TypePrimitiveV::IsArray =>
					Some ($alternative::IsArray),
				TypePrimitiveV::IsArrayMutable =>
					Some ($alternative::IsArrayMutable),
				TypePrimitiveV::IsArrayImmutable =>
					Some ($alternative::IsArrayImmutable),
				TypePrimitiveV::IsArrayEmpty =>
					Some ($alternative::IsArrayEmpty),
				TypePrimitiveV::IsArrayMutableEmpty =>
					Some ($alternative::IsArrayMutableEmpty),
				TypePrimitiveV::IsArrayImmutableEmpty =>
					Some ($alternative::IsArrayImmutableEmpty),
				TypePrimitiveV::IsArrayEmptyNot =>
					Some ($alternative::IsArrayEmptyNot),
				TypePrimitiveV::IsArrayMutableEmptyNot =>
					Some ($alternative::IsArrayMutableEmptyNot),
				TypePrimitiveV::IsArrayImmutableEmptyNot =>
					Some ($alternative::IsArrayImmutableEmptyNot),
				TypePrimitiveV::IsValues =>
					Some ($alternative::IsValues),
				TypePrimitiveV::IsValuesEmpty =>
					Some ($alternative::IsValuesEmpty),
				TypePrimitiveV::IsValuesEmptyNot =>
					Some ($alternative::IsValuesEmptyNot),
				TypePrimitiveV::IsRecordKind =>
					Some ($alternative::IsRecordKind),
				TypePrimitiveV::IsRecord =>
					Some ($alternative::IsRecord),
				TypePrimitiveV::IsRecordImmutable =>
					Some ($alternative::IsRecordImmutable),
				TypePrimitiveV::IsRecordMutable =>
					Some ($alternative::IsRecordMutable),
				TypePrimitiveV::IsError =>
					Some ($alternative::IsError),
				TypePrimitiveV::IsErrorSyntax =>
					Some ($alternative::IsErrorSyntax),
				TypePrimitiveV::IsErrorFile =>
					Some ($alternative::IsErrorFile),
				TypePrimitiveV::IsErrorPort =>
					Some ($alternative::IsErrorPort),
				TypePrimitiveV::IsErrorPortInput =>
					Some ($alternative::IsErrorPortInput),
				TypePrimitiveV::IsErrorPortOutput =>
					Some ($alternative::IsErrorPortOutput),
				TypePrimitiveV::IsList =>
					Some ($alternative::IsList),
				TypePrimitiveV::IsListProper =>
					Some ($alternative::IsListProper),
				TypePrimitiveV::IsListProperOrEmpty =>
					Some ($alternative::IsListProperOrEmpty),
				TypePrimitiveV::IsListDotted =>
					Some ($alternative::IsListDotted),
				TypePrimitiveV::IsListDottedOrEmpty =>
					Some ($alternative::IsListDottedOrEmpty),
				TypePrimitiveV::IsListCyclic =>
					Some ($alternative::IsListCyclic),
				TypePrimitiveV::IsListCyclicOrEmpty =>
					Some ($alternative::IsListCyclicOrEmpty),
				TypePrimitiveV::IsProcedure =>
					Some ($alternative::IsProcedure),
				TypePrimitiveV::IsSyntax =>
					Some ($alternative::IsSyntax),
				TypePrimitiveV::IsPort =>
					Some ($alternative::IsPort),
				TypePrimitiveV::IsPortInput =>
					Some ($alternative::IsPortInput),
				TypePrimitiveV::IsPortOutput =>
					Some ($alternative::IsPortOutput),
				TypePrimitiveV::IsPortBinary =>
					Some ($alternative::IsPortBinary),
				TypePrimitiveV::IsPortTextual =>
					Some ($alternative::IsPortTextual),
				TypePrimitiveV::IsPortInputBinary =>
					Some ($alternative::IsPortInputBinary),
				TypePrimitiveV::IsPortInputTextual =>
					Some ($alternative::IsPortInputTextual),
				TypePrimitiveV::IsPortOutputBinary =>
					Some ($alternative::IsPortOutputBinary),
				TypePrimitiveV::IsPortOutputTextual =>
					Some ($alternative::IsPortOutputTextual),
				TypePrimitiveV::IsPortEof =>
					Some ($alternative::IsPortEof),
				TypePrimitiveV::IsProcess =>
					Some ($alternative::IsProcess),
				TypePrimitiveV::IsContext =>
					Some ($alternative::IsContext),
				TypePrimitiveV::IsBinding =>
					Some ($alternative::IsBinding),
				TypePrimitiveV::IsParameters =>
					Some ($alternative::IsParameters),
				TypePrimitiveV::IsParameter =>
					Some ($alternative::IsParameter),
				TypePrimitiveV::IsPromise =>
					Some ($alternative::IsPromise),
				TypePrimitiveV::IsResource =>
					Some ($alternative::IsResource),
				TypePrimitiveV::IsInternal =>
					Some ($alternative::IsInternal),
				TypePrimitiveV::IsOpaque =>
					Some ($alternative::IsOpaque),
				TypePrimitiveV::IsNumberZero =>
					Some ($alternative::IsNumberZero),
				TypePrimitiveV::IsNumberPositive =>
					Some ($alternative::IsNumberPositive),
				TypePrimitiveV::IsNumberNegative =>
					Some ($alternative::IsNumberNegative),
				TypePrimitiveV::IsNumberFinite =>
					Some ($alternative::IsNumberFinite),
				TypePrimitiveV::IsNumberInfinite =>
					Some ($alternative::IsNumberInfinite),
				TypePrimitiveV::IsNumberNan =>
					Some ($alternative::IsNumberNan),
				TypePrimitiveV::IsNumberEven =>
					Some ($alternative::IsNumberEven),
				TypePrimitiveV::IsNumberOdd =>
					Some ($alternative::IsNumberOdd),
				TypePrimitiveV::IsCharacterNumeric =>
					Some ($alternative::IsCharacterNumeric),
				TypePrimitiveV::IsCharacterAlphabetic =>
					Some ($alternative::IsCharacterAlphabetic),
				TypePrimitiveV::IsCharacterAlphabeticUpperCase =>
					Some ($alternative::IsCharacterAlphabeticUpperCase),
				TypePrimitiveV::IsCharacterAlphabeticLowerCase =>
					Some ($alternative::IsCharacterAlphabeticLowerCase),
				TypePrimitiveV::IsCharacterAlphabeticOrNumeric =>
					Some ($alternative::IsCharacterAlphabeticOrNumeric),
				TypePrimitiveV::IsCharacterWhitespace =>
					Some ($alternative::IsCharacterWhitespace),
				TypePrimitiveV::IsCharacterControl =>
					Some ($alternative::IsCharacterControl),
				TypePrimitiveV::IsCharacterAscii =>
					Some ($alternative::IsCharacterAscii),
				TypePrimitiveV::IsCharacterAsciiNumeric =>
					Some ($alternative::IsCharacterAsciiNumeric),
				TypePrimitiveV::IsCharacterAsciiNumericBase8 =>
					Some ($alternative::IsCharacterAsciiNumericBase8),
				TypePrimitiveV::IsCharacterAsciiNumericBase16 =>
					Some ($alternative::IsCharacterAsciiNumericBase16),
				TypePrimitiveV::IsCharacterAsciiAlphabetic =>
					Some ($alternative::IsCharacterAsciiAlphabetic),
				TypePrimitiveV::IsCharacterAsciiAlphabeticUpperCase =>
					Some ($alternative::IsCharacterAsciiAlphabeticUpperCase),
				TypePrimitiveV::IsCharacterAsciiAlphabeticLowerCase =>
					Some ($alternative::IsCharacterAsciiAlphabeticLowerCase),
				TypePrimitiveV::IsCharacterAsciiAlphabeticOrNumeric =>
					Some ($alternative::IsCharacterAsciiAlphabeticOrNumeric),
				TypePrimitiveV::IsCharacterAsciiWhitespace =>
					Some ($alternative::IsCharacterAsciiWhitespace),
				TypePrimitiveV::IsCharacterAsciiControl =>
					Some ($alternative::IsCharacterAsciiControl),
				TypePrimitiveV::IsCharacterAsciiPunctuation =>
					Some ($alternative::IsCharacterAsciiPunctuation),
				TypePrimitiveV::IsCharacterAsciiGraphic =>
					Some ($alternative::IsCharacterAsciiGraphic),
			}
		}
		
	);
}


def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_1, TypePrimitive1);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_2, TypePrimitive2);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_3, TypePrimitive3);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_4, TypePrimitive4);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_n, TypePrimitiveN);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_v_alternative_0 (_primitive : TypePrimitiveV) -> (Option<TypePrimitive0>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_v_alternative_5 (_primitive : TypePrimitiveV) -> (Option<TypePrimitive5>) {
	return None;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_0_attributes (_primitive : TypePrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_attributes (_primitive : TypePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_attributes (_primitive : TypePrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_attributes (_primitive : TypePrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_3);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_attributes (_primitive : TypePrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_5_attributes (_primitive : TypePrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_attributes (_primitive : TypePrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

