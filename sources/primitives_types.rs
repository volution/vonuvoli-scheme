

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::TypePrimitive1;
	
	pub use super::type_primitive_1_evaluate;
	pub use super::type_primitive_1_evaluate_0;
	
	pub use super::type_primitive_1_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum TypePrimitive1 {
	
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
	
	IsString,
	IsStringImmutable,
	IsStringMutable,
	IsBytes,
	IsBytesImmutable,
	IsBytesMutable,
	IsPair,
	IsPairMutable,
	IsPairImmutable,
	IsArray,
	IsArrayMutable,
	IsArrayImmutable,
	
	IsValues,
	
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
	IsPortEof,
	
	IsProcess,
	IsResource,
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	return type_primitive_1_evaluate_0 (primitive, input_1) .into_0 ();
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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
		
		TypePrimitive1::IsString =>
			return is_string (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutable =>
			return is_string_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutable =>
			return is_string_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytes =>
			return is_bytes (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutable =>
			return is_bytes_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutable =>
			return is_bytes_mutable (input_1) .into_0 (),
		
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
		
		TypePrimitive1::IsValues =>
			return is_values (input_1) .into_0 (),
		
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
		
		TypePrimitive1::IsPortEof =>
			return is_port_eof (input_1) .into_0 (),
		
		TypePrimitive1::IsProcess =>
			return is_process (input_1) .into_0 (),
		
		TypePrimitive1::IsResource =>
			return is_resource (input_1) .into_0 (),
		
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
			return character_is_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabetic =>
			return character_is_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			return character_is_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			return character_is_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			return character_is_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterWhitespace =>
			return character_is_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterControl =>
			return character_is_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAscii =>
			return character_is_ascii (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumeric =>
			return character_is_ascii_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			return character_is_ascii_numeric_base_8 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			return character_is_ascii_numeric_base_16 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			return character_is_ascii_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			return character_is_ascii_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			return character_is_ascii_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			return character_is_ascii_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			return character_is_ascii_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiControl =>
			return character_is_ascii_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			return character_is_ascii_punctuation (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiGraphic =>
			return character_is_ascii_graphic (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn type_primitive_1_attributes (_primitive : TypePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

