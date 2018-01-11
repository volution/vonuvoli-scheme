

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
	let output = try! (type_primitive_1_evaluate_0 (primitive, input_1));
	succeed! (output.into ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn type_primitive_1_evaluate_0 (primitive : TypePrimitive1, input_1 : &Value) -> (Outcome<bool>) {
	let output = match primitive {
		
		TypePrimitive1::IsNull =>
			is_null (input_1),
		
		TypePrimitive1::IsNullNot =>
			is_not_null (input_1),
		
		TypePrimitive1::IsVoid =>
			is_void (input_1),
		
		TypePrimitive1::IsVoidNot =>
			is_not_void (input_1),
		
		TypePrimitive1::IsUndefined =>
			is_undefined (input_1),
		
		TypePrimitive1::IsUndefinedNot =>
			is_not_undefined (input_1),
		
		TypePrimitive1::IsBoolean =>
			is_boolean (input_1),
		
		TypePrimitive1::IsTrue =>
			is_true (input_1),
		
		TypePrimitive1::IsTrueNot =>
			is_not_true (input_1),
		
		TypePrimitive1::IsFalse =>
			is_false (input_1),
		
		TypePrimitive1::IsFalseNot =>
			is_not_false (input_1),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			is_true_or_equivalent (input_1),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			is_false_or_equivalent (input_1),
		
		TypePrimitive1::IsNumber =>
			is_number (input_1),
		
		TypePrimitive1::IsNumberInteger =>
			is_number_integer (input_1),
		
		TypePrimitive1::IsNumberRational =>
			is_number_rational (input_1),
		
		TypePrimitive1::IsNumberReal =>
			is_number_real (input_1),
		
		TypePrimitive1::IsNumberComplex =>
			is_number_complex (input_1),
		
		TypePrimitive1::IsNumberExact =>
			try! (is_number_exact (input_1)),
		
		TypePrimitive1::IsNumberExactInteger =>
			try! (is_number_exact_integer (input_1)),
		
		TypePrimitive1::IsNumberInexact =>
			try! (is_number_inexact (input_1)),
		
		TypePrimitive1::IsCharacter =>
			is_character (input_1),
		
		TypePrimitive1::IsSymbol =>
			is_symbol (input_1),
		
		TypePrimitive1::IsString =>
			is_string (input_1),
		
		TypePrimitive1::IsStringImmutable =>
			is_string_immutable (input_1),
		
		TypePrimitive1::IsStringMutable =>
			is_string_mutable (input_1),
		
		TypePrimitive1::IsBytes =>
			is_bytes (input_1),
		
		TypePrimitive1::IsBytesImmutable =>
			is_bytes_immutable (input_1),
		
		TypePrimitive1::IsBytesMutable =>
			is_bytes_mutable (input_1),
		
		TypePrimitive1::IsPair =>
			is_pair (input_1),
		
		TypePrimitive1::IsPairImmutable =>
			is_pair_immutable (input_1),
		
		TypePrimitive1::IsPairMutable =>
			is_pair_mutable (input_1),
		
		TypePrimitive1::IsArray =>
			is_array (input_1),
		
		TypePrimitive1::IsArrayImmutable =>
			is_array_immutable (input_1),
		
		TypePrimitive1::IsArrayMutable =>
			is_array_mutable (input_1),
		
		TypePrimitive1::IsValues =>
			is_values (input_1),
		
		TypePrimitive1::IsError =>
			is_error (input_1),
		
		TypePrimitive1::IsErrorSyntax =>
			try! (is_error_syntax (input_1)),
		
		TypePrimitive1::IsErrorFile =>
			try! (is_error_file (input_1)),
		
		TypePrimitive1::IsErrorPort =>
			try! (is_error_port (input_1)),
		
		TypePrimitive1::IsErrorPortInput =>
			try! (is_error_port_input (input_1)),
		
		TypePrimitive1::IsErrorPortOutput =>
			try! (is_error_port_output (input_1)),
		
		TypePrimitive1::IsList =>
			is_list (input_1),
		
		TypePrimitive1::IsListProper =>
			is_list_proper (input_1),
		
		TypePrimitive1::IsListProperOrEmpty =>
			is_list_proper_or_empty (input_1),
		
		TypePrimitive1::IsListDotted =>
			is_list_dotted (input_1),
		
		TypePrimitive1::IsListDottedOrEmpty =>
			is_list_dotted_or_empty (input_1),
		
		TypePrimitive1::IsListCyclic =>
			is_list_cyclic (input_1),
		
		TypePrimitive1::IsListCyclicOrEmpty =>
			is_list_cyclic_or_empty (input_1),
		
		TypePrimitive1::IsProcedure =>
			is_procedure (input_1),
		
		TypePrimitive1::IsSyntax =>
			is_syntax (input_1),
		
		TypePrimitive1::IsPort =>
			is_port (input_1),
		
		TypePrimitive1::IsPortInput =>
			try! (is_port_input (input_1)),
		
		TypePrimitive1::IsPortOutput =>
			try! (is_port_output (input_1)),
		
		TypePrimitive1::IsPortBinary =>
			try! (is_port_binary (input_1)),
		
		TypePrimitive1::IsPortTextual =>
			try! (is_port_textual (input_1)),
		
		TypePrimitive1::IsPortEof =>
			is_port_eof (input_1),
		
		TypePrimitive1::IsProcess =>
			is_process (input_1),
		
		TypePrimitive1::IsResource =>
			is_resource (input_1),
		
		TypePrimitive1::IsOpaque =>
			is_opaque (input_1),
		
		TypePrimitive1::IsNumberZero =>
			try! (is_number_zero (input_1)),
		
		TypePrimitive1::IsNumberPositive =>
			try! (is_number_positive (input_1)),
		
		TypePrimitive1::IsNumberNegative =>
			try! (is_number_negative (input_1)),
		
		TypePrimitive1::IsNumberFinite =>
			try! (is_number_finite (input_1)),
		
		TypePrimitive1::IsNumberInfinite =>
			try! (is_number_infinite (input_1)),
		
		TypePrimitive1::IsNumberNan =>
			try! (is_number_nan (input_1)),
		
		TypePrimitive1::IsNumberEven =>
			try! (is_number_even (input_1)),
		
		TypePrimitive1::IsNumberOdd =>
			try! (is_number_odd (input_1)),
		
		TypePrimitive1::IsCharacterNumeric =>
			try! (character_is_numeric (input_1)),
		
		TypePrimitive1::IsCharacterAlphabetic =>
			try! (character_is_alphabetic (input_1)),
		
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			try! (character_is_alphabetic_upper_case (input_1)),
		
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			try! (character_is_alphabetic_lower_case (input_1)),
		
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			try! (character_is_alphabetic_or_numeric (input_1)),
		
		TypePrimitive1::IsCharacterWhitespace =>
			try! (character_is_whitespace (input_1)),
		
		TypePrimitive1::IsCharacterControl =>
			try! (character_is_control (input_1)),
		
		TypePrimitive1::IsCharacterAscii =>
			try! (character_is_ascii (input_1)),
		
		TypePrimitive1::IsCharacterAsciiNumeric =>
			try! (character_is_ascii_numeric (input_1)),
		
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			try! (character_is_ascii_numeric_base_8 (input_1)),
		
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			try! (character_is_ascii_numeric_base_16 (input_1)),
		
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			try! (character_is_ascii_alphabetic (input_1)),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			try! (character_is_ascii_alphabetic_upper_case (input_1)),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			try! (character_is_ascii_alphabetic_lower_case (input_1)),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			try! (character_is_ascii_alphabetic_or_numeric (input_1)),
		
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			try! (character_is_ascii_whitespace (input_1)),
		
		TypePrimitive1::IsCharacterAsciiControl =>
			try! (character_is_ascii_control (input_1)),
		
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			try! (character_is_ascii_punctuation (input_1)),
		
		TypePrimitive1::IsCharacterAsciiGraphic =>
			try! (character_is_ascii_graphic (input_1)),
		
	};
	succeed! (output);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn type_primitive_1_attributes (_primitive : TypePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

