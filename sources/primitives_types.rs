

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
	pub use super::type_primitive_2_evaluate;
	pub use super::type_primitive_3_evaluate;
	pub use super::type_primitive_4_evaluate;
	pub use super::type_primitive_5_evaluate;
	pub use super::type_primitive_n_evaluate;
	
	pub use super::type_primitive_1_evaluate_0;
	pub use super::type_primitive_2_evaluate_0;
	pub use super::type_primitive_3_evaluate_0;
	pub use super::type_primitive_4_evaluate_0;
	pub use super::type_primitive_n_evaluate_0;
	
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacter,
			
			IsSymbol,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			IsKeyword,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			IsUnique,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsString,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringImmutable,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringMutable,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringEmpty,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringImmutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringMutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringImmutableEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsStringMutableEmptyNot,
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytes,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesImmutable,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesMutable,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesEmpty,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesImmutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesMutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesImmutableEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			IsBytesMutableEmptyNot,
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			IsStringRegex,
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			IsBytesRegex,
			
			IsPair,
			IsPairMutable,
			IsPairImmutable,
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArray,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayMutable,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayImmutable,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayEmpty,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayMutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayImmutableEmpty,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayMutableEmptyNot,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			IsArrayImmutableEmptyNot,
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			IsValues,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			IsValuesEmpty,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			IsValuesEmptyNot,
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			IsRecordKind,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			IsRecord,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			IsRecordImmutable,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			IsRecordMutable,
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			IsError,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			IsErrorSyntax,
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsErrorFile,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsErrorPort,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsErrorPortInput,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPort,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortInput,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortOutput,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortBinary,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortTextual,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortInputBinary,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortInputTextual,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortOutputBinary,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortOutputTextual,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			IsPortEof,
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			IsPath,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			IsPathAbsolute,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			IsPathRelative,
			
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			IsProcess,
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			IsContext,
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			IsBinding,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			IsParameters,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			IsParameter,
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			IsPromise,
			
			IsResource,
			IsInternal,
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			IsOpaque,
			
			IsNumberZero,
			IsNumberPositive,
			IsNumberNegative,
			IsNumberFinite,
			IsNumberInfinite,
			IsNumberNan,
			IsNumberEven,
			IsNumberOdd,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterNumeric,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAlphabetic,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAlphabeticUpperCase,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAlphabeticLowerCase,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAlphabeticOrNumeric,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterWhitespace,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterControl,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAscii,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiNumeric,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiNumericBase8,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiNumericBase16,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiAlphabetic,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiAlphabeticUpperCase,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiAlphabeticLowerCase,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiAlphabeticOrNumeric,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiWhitespace,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiControl,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			IsCharacterAsciiPunctuation,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
pub fn type_primitive_0_evaluate (primitive : TypePrimitive0, _negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input_1 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let outcome = try! (type_primitive_1_evaluate_0 (primitive, input_1));
	let outcome = outcome ^ negated;
	succeed! (outcome.into ());
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacter =>
			return is_character (input_1) .into_0 (),
		
		TypePrimitive1::IsSymbol =>
			return is_symbol (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		TypePrimitive1::IsKeyword =>
			return is_keyword (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		TypePrimitive1::IsUnique =>
			return is_unique (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsString =>
			return is_string (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringImmutable =>
			return is_string_immutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringMutable =>
			return is_string_mutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringEmpty =>
			return is_string_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringImmutableEmpty =>
			return is_string_immutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringMutableEmpty =>
			return is_string_mutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringEmptyNot =>
			return is_string_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytes =>
			return is_bytes (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesImmutable =>
			return is_bytes_immutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesMutable =>
			return is_bytes_mutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesEmpty =>
			return is_bytes_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesEmptyNot =>
			return is_bytes_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive1::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive1::IsStringRegex =>
			return is_string_regex (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive1::IsBytesRegex =>
			return is_bytes_regex (input_1) .into_0 (),
		
		TypePrimitive1::IsPair =>
			return is_pair (input_1) .into_0 (),
		
		TypePrimitive1::IsPairImmutable =>
			return is_pair_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsPairMutable =>
			return is_pair_mutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArray =>
			return is_array (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayImmutable =>
			return is_array_immutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayMutable =>
			return is_array_mutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayEmpty =>
			return is_array_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayImmutableEmpty =>
			return is_array_immutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayMutableEmpty =>
			return is_array_mutable_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayEmptyNot =>
			return is_array_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive1::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive1::IsValues =>
			return is_values (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive1::IsValuesEmpty =>
			return is_values_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive1::IsValuesEmptyNot =>
			return is_values_not_empty (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive1::IsRecordKind =>
			return is_record_kind (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive1::IsRecord =>
			return is_record (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive1::IsRecordImmutable =>
			return is_record_immutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive1::IsRecordMutable =>
			return is_record_mutable (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive1::IsError =>
			return is_error (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive1::IsErrorSyntax =>
			return is_error_syntax (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsErrorFile =>
			return is_error_file (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsErrorPort =>
			return is_error_port (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsErrorPortInput =>
			return is_error_port_input (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPort =>
			return is_port (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortInput =>
			return is_port_input (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortOutput =>
			return is_port_output (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortBinary =>
			return is_port_binary (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortTextual =>
			return is_port_textual (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortInputBinary =>
			return is_port_input_binary (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortInputTextual =>
			return is_port_input_textual (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortOutputBinary =>
			return is_port_output_binary (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortOutputTextual =>
			return is_port_output_textual (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive1::IsPortEof =>
			return is_port_eof (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive1::IsPath =>
			return is_path (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive1::IsPathAbsolute =>
			return is_path_absolute (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive1::IsPathRelative =>
			return is_path_relative (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		TypePrimitive1::IsProcess =>
			return is_process (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive1::IsContext =>
			return is_context (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive1::IsBinding =>
			return is_binding (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive1::IsParameters =>
			return is_parameters (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive1::IsParameter =>
			return is_parameter (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		TypePrimitive1::IsPromise =>
			return is_promise (input_1) .into_0 (),
		
		TypePrimitive1::IsResource =>
			return is_resource (input_1) .into_0 (),
		
		TypePrimitive1::IsInternal =>
			return is_internal (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterNumeric =>
			return is_character_numeric (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAlphabetic =>
			return is_character_alphabetic (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterWhitespace =>
			return is_character_whitespace (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterControl =>
			return is_character_control (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAscii =>
			return is_character_ascii (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8 (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16 (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiControl =>
			return is_character_ascii_control (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive1::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_evaluate (primitive : TypePrimitive2, input_1 : &Value, input_2 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let outcome = try! (type_primitive_2_evaluate_0 (primitive, input_1, input_2));
	let outcome = outcome ^ negated;
	succeed! (outcome.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_evaluate_0 (primitive : TypePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<bool>) {
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacter =>
			return is_character_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsSymbol =>
			return is_symbol_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		TypePrimitive2::IsKeyword =>
			return is_keyword_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		TypePrimitive2::IsUnique =>
			return is_unique_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsString =>
			return is_string_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringImmutable =>
			return is_string_immutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringMutable =>
			return is_string_mutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringEmpty =>
			return is_string_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringEmptyNot =>
			return is_string_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytes =>
			return is_bytes_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesImmutable =>
			return is_bytes_immutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesMutable =>
			return is_bytes_mutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesEmpty =>
			return is_bytes_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive2::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive2::IsStringRegex =>
			return is_string_regex_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive2::IsBytesRegex =>
			return is_bytes_regex_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPair =>
			return is_pair_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPairImmutable =>
			return is_pair_immutable_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsPairMutable =>
			return is_pair_mutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArray =>
			return is_array_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayImmutable =>
			return is_array_immutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayMutable =>
			return is_array_mutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayEmpty =>
			return is_array_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayEmptyNot =>
			return is_array_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive2::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive2::IsValues =>
			return is_values_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive2::IsValuesEmpty =>
			return is_values_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive2::IsValuesEmptyNot =>
			return is_values_not_empty_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive2::IsRecordKind =>
			return is_record_kind_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive2::IsRecord =>
			return is_record_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive2::IsRecordImmutable =>
			return is_record_immutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive2::IsRecordMutable =>
			return is_record_mutable_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive2::IsError =>
			return is_error_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive2::IsErrorSyntax =>
			return is_error_syntax_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsErrorFile =>
			return is_error_file_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsErrorPort =>
			return is_error_port_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsErrorPortInput =>
			return is_error_port_input_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPort =>
			return is_port_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortInput =>
			return is_port_input_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortOutput =>
			return is_port_output_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortBinary =>
			return is_port_binary_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortTextual =>
			return is_port_textual_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortInputBinary =>
			return is_port_input_binary_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortInputTextual =>
			return is_port_input_textual_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortOutputBinary =>
			return is_port_output_binary_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortOutputTextual =>
			return is_port_output_textual_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive2::IsPortEof =>
			return is_port_eof_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive2::IsPath =>
			return is_path_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive2::IsPathAbsolute =>
			return is_path_absolute_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive2::IsPathRelative =>
			return is_path_relative_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		TypePrimitive2::IsProcess =>
			return is_process_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive2::IsContext =>
			return is_context_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive2::IsBinding =>
			return is_binding_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive2::IsParameters =>
			return is_parameters_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive2::IsParameter =>
			return is_parameter_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		TypePrimitive2::IsPromise =>
			return is_promise_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsResource =>
			return is_resource_all_2 (input_1, input_2) .into_0 (),
		
		TypePrimitive2::IsInternal =>
			return is_internal_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterNumeric =>
			return is_character_numeric_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterWhitespace =>
			return is_character_whitespace_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterControl =>
			return is_character_control_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAscii =>
			return is_character_ascii_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_2 (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive2::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_2 (input_1, input_2) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_evaluate (primitive : TypePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let outcome = try! (type_primitive_3_evaluate_0 (primitive, input_1, input_2, input_3));
	let outcome = outcome ^ negated;
	succeed! (outcome.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_evaluate_0 (primitive : TypePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<bool>) {
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacter =>
			return is_character_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsSymbol =>
			return is_symbol_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		TypePrimitive3::IsKeyword =>
			return is_keyword_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		TypePrimitive3::IsUnique =>
			return is_unique_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsString =>
			return is_string_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringImmutable =>
			return is_string_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringMutable =>
			return is_string_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringEmpty =>
			return is_string_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringEmptyNot =>
			return is_string_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytes =>
			return is_bytes_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesImmutable =>
			return is_bytes_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesMutable =>
			return is_bytes_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesEmpty =>
			return is_bytes_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive3::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive3::IsStringRegex =>
			return is_string_regex_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive3::IsBytesRegex =>
			return is_bytes_regex_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPair =>
			return is_pair_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPairImmutable =>
			return is_pair_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsPairMutable =>
			return is_pair_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArray =>
			return is_array_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayImmutable =>
			return is_array_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayMutable =>
			return is_array_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayEmpty =>
			return is_array_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayEmptyNot =>
			return is_array_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive3::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive3::IsValues =>
			return is_values_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive3::IsValuesEmpty =>
			return is_values_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive3::IsValuesEmptyNot =>
			return is_values_not_empty_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive3::IsRecordKind =>
			return is_record_kind_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive3::IsRecord =>
			return is_record_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive3::IsRecordImmutable =>
			return is_record_immutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive3::IsRecordMutable =>
			return is_record_mutable_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive3::IsError =>
			return is_error_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive3::IsErrorSyntax =>
			return is_error_syntax_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsErrorFile =>
			return is_error_file_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsErrorPort =>
			return is_error_port_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsErrorPortInput =>
			return is_error_port_input_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPort =>
			return is_port_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortInput =>
			return is_port_input_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortOutput =>
			return is_port_output_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortBinary =>
			return is_port_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortTextual =>
			return is_port_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortInputBinary =>
			return is_port_input_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortInputTextual =>
			return is_port_input_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortOutputBinary =>
			return is_port_output_binary_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortOutputTextual =>
			return is_port_output_textual_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive3::IsPortEof =>
			return is_port_eof_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive3::IsPath =>
			return is_path_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive3::IsPathAbsolute =>
			return is_path_absolute_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive3::IsPathRelative =>
			return is_path_relative_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		TypePrimitive3::IsProcess =>
			return is_process_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive3::IsContext =>
			return is_context_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive3::IsBinding =>
			return is_binding_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive3::IsParameters =>
			return is_parameters_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive3::IsParameter =>
			return is_parameter_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		TypePrimitive3::IsPromise =>
			return is_promise_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsResource =>
			return is_resource_all_3 (input_1, input_2, input_3) .into_0 (),
		
		TypePrimitive3::IsInternal =>
			return is_internal_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterNumeric =>
			return is_character_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterWhitespace =>
			return is_character_whitespace_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterControl =>
			return is_character_control_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAscii =>
			return is_character_ascii_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_3 (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive3::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_3 (input_1, input_2, input_3) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_evaluate (primitive : TypePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let outcome = try! (type_primitive_4_evaluate_0 (primitive, input_1, input_2, input_3, input_4));
	let outcome = outcome ^ negated;
	succeed! (outcome.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_evaluate_0 (primitive : TypePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<bool>) {
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacter =>
			return is_character_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsSymbol =>
			return is_symbol_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		TypePrimitive4::IsKeyword =>
			return is_keyword_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		TypePrimitive4::IsUnique =>
			return is_unique_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsString =>
			return is_string_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringImmutable =>
			return is_string_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringMutable =>
			return is_string_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringEmpty =>
			return is_string_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringEmptyNot =>
			return is_string_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytes =>
			return is_bytes_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesImmutable =>
			return is_bytes_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesMutable =>
			return is_bytes_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesEmpty =>
			return is_bytes_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitive4::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive4::IsStringRegex =>
			return is_string_regex_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitive4::IsBytesRegex =>
			return is_bytes_regex_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPair =>
			return is_pair_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPairImmutable =>
			return is_pair_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsPairMutable =>
			return is_pair_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArray =>
			return is_array_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayImmutable =>
			return is_array_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayMutable =>
			return is_array_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayEmpty =>
			return is_array_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayEmptyNot =>
			return is_array_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitive4::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive4::IsValues =>
			return is_values_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive4::IsValuesEmpty =>
			return is_values_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitive4::IsValuesEmptyNot =>
			return is_values_not_empty_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive4::IsRecordKind =>
			return is_record_kind_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive4::IsRecord =>
			return is_record_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive4::IsRecordImmutable =>
			return is_record_immutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitive4::IsRecordMutable =>
			return is_record_mutable_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive4::IsError =>
			return is_error_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitive4::IsErrorSyntax =>
			return is_error_syntax_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsErrorFile =>
			return is_error_file_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsErrorPort =>
			return is_error_port_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsErrorPortInput =>
			return is_error_port_input_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPort =>
			return is_port_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortInput =>
			return is_port_input_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortOutput =>
			return is_port_output_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortBinary =>
			return is_port_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortTextual =>
			return is_port_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortInputBinary =>
			return is_port_input_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortInputTextual =>
			return is_port_input_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortOutputBinary =>
			return is_port_output_binary_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortOutputTextual =>
			return is_port_output_textual_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitive4::IsPortEof =>
			return is_port_eof_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive4::IsPath =>
			return is_path_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive4::IsPathAbsolute =>
			return is_path_absolute_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitive4::IsPathRelative =>
			return is_path_relative_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		TypePrimitive4::IsProcess =>
			return is_process_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive4::IsContext =>
			return is_context_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitive4::IsBinding =>
			return is_binding_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive4::IsParameters =>
			return is_parameters_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitive4::IsParameter =>
			return is_parameter_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		TypePrimitive4::IsPromise =>
			return is_promise_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsResource =>
			return is_resource_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		TypePrimitive4::IsInternal =>
			return is_internal_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterNumeric =>
			return is_character_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterWhitespace =>
			return is_character_whitespace_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterControl =>
			return is_character_control_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAscii =>
			return is_character_ascii_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitive4::IsCharacterAsciiGraphic =>
			return is_character_ascii_graphic_all_4 (input_1, input_2, input_3, input_4) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_5_evaluate (primitive : TypePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_evaluate (primitive : TypePrimitiveN, inputs : &[&Value], negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let outcome = try! (type_primitive_n_evaluate_0 (primitive, inputs));
	let outcome = outcome ^ negated;
	succeed! (outcome.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_evaluate_0 (primitive : TypePrimitiveN, inputs : &[&Value]) -> (Outcome<bool>) {
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacter =>
			return is_character_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsSymbol =>
			return is_symbol_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		TypePrimitiveN::IsKeyword =>
			return is_keyword_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		TypePrimitiveN::IsUnique =>
			return is_unique_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsString =>
			return is_string_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringImmutable =>
			return is_string_immutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringMutable =>
			return is_string_mutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringEmpty =>
			return is_string_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringImmutableEmpty =>
			return is_string_immutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringMutableEmpty =>
			return is_string_mutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringEmptyNot =>
			return is_string_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytes =>
			return is_bytes_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesImmutable =>
			return is_bytes_immutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesMutable =>
			return is_bytes_mutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesEmpty =>
			return is_bytes_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesEmptyNot =>
			return is_bytes_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		TypePrimitiveN::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitiveN::IsStringRegex =>
			return is_string_regex_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		TypePrimitiveN::IsBytesRegex =>
			return is_bytes_regex_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPair =>
			return is_pair_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPairImmutable =>
			return is_pair_immutable_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsPairMutable =>
			return is_pair_mutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArray =>
			return is_array_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayImmutable =>
			return is_array_immutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayMutable =>
			return is_array_mutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayEmpty =>
			return is_array_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayImmutableEmpty =>
			return is_array_immutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayMutableEmpty =>
			return is_array_mutable_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayEmptyNot =>
			return is_array_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		TypePrimitiveN::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitiveN::IsValues =>
			return is_values_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitiveN::IsValuesEmpty =>
			return is_values_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		TypePrimitiveN::IsValuesEmptyNot =>
			return is_values_not_empty_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitiveN::IsRecordKind =>
			return is_record_kind_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitiveN::IsRecord =>
			return is_record_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitiveN::IsRecordImmutable =>
			return is_record_immutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		TypePrimitiveN::IsRecordMutable =>
			return is_record_mutable_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitiveN::IsError =>
			return is_error_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		TypePrimitiveN::IsErrorSyntax =>
			return is_error_syntax_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsErrorFile =>
			return is_error_file_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsErrorPort =>
			return is_error_port_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsErrorPortInput =>
			return is_error_port_input_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPort =>
			return is_port_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortInput =>
			return is_port_input_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortOutput =>
			return is_port_output_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortBinary =>
			return is_port_binary_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortTextual =>
			return is_port_textual_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortInputBinary =>
			return is_port_input_binary_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortInputTextual =>
			return is_port_input_textual_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortOutputBinary =>
			return is_port_output_binary_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortOutputTextual =>
			return is_port_output_textual_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		TypePrimitiveN::IsPortEof =>
			return is_port_eof_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitiveN::IsPath =>
			return is_path_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitiveN::IsPathAbsolute =>
			return is_path_absolute_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		TypePrimitiveN::IsPathRelative =>
			return is_path_relative_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		TypePrimitiveN::IsProcess =>
			return is_process_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitiveN::IsContext =>
			return is_context_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
		TypePrimitiveN::IsBinding =>
			return is_binding_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitiveN::IsParameters =>
			return is_parameters_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		TypePrimitiveN::IsParameter =>
			return is_parameter_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
		TypePrimitiveN::IsPromise =>
			return is_promise_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsResource =>
			return is_resource_all_n (inputs) .into_0 (),
		
		TypePrimitiveN::IsInternal =>
			return is_internal_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterNumeric =>
			return is_character_numeric_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAlphabetic =>
			return is_character_alphabetic_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAlphabeticUpperCase =>
			return is_character_alphabetic_upper_case_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAlphabeticLowerCase =>
			return is_character_alphabetic_lower_case_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAlphabeticOrNumeric =>
			return is_character_alphabetic_or_numeric_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterWhitespace =>
			return is_character_whitespace_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterControl =>
			return is_character_control_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAscii =>
			return is_character_ascii_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiNumeric =>
			return is_character_ascii_numeric_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiNumericBase8 =>
			return is_character_ascii_numeric_base_8_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiNumericBase16 =>
			return is_character_ascii_numeric_base_16_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiAlphabetic =>
			return is_character_ascii_alphabetic_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiAlphabeticUpperCase =>
			return is_character_ascii_alphabetic_upper_case_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiAlphabeticLowerCase =>
			return is_character_ascii_alphabetic_lower_case_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiAlphabeticOrNumeric =>
			return is_character_ascii_alphabetic_or_numeric_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiWhitespace =>
			return is_character_ascii_whitespace_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiControl =>
			return is_character_ascii_control_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		TypePrimitiveN::IsCharacterAsciiPunctuation =>
			return is_character_ascii_punctuation_all_n (inputs) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacter =>
					Some ($alternative::IsCharacter),
				TypePrimitiveV::IsSymbol =>
					Some ($alternative::IsSymbol),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				TypePrimitiveV::IsKeyword =>
					Some ($alternative::IsKeyword),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				TypePrimitiveV::IsUnique =>
					Some ($alternative::IsUnique),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsString =>
					Some ($alternative::IsString),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringImmutable =>
					Some ($alternative::IsStringImmutable),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringMutable =>
					Some ($alternative::IsStringMutable),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringEmpty =>
					Some ($alternative::IsStringEmpty),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringImmutableEmpty =>
					Some ($alternative::IsStringImmutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringMutableEmpty =>
					Some ($alternative::IsStringMutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringEmptyNot =>
					Some ($alternative::IsStringEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringImmutableEmptyNot =>
					Some ($alternative::IsStringImmutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsStringMutableEmptyNot =>
					Some ($alternative::IsStringMutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytes =>
					Some ($alternative::IsBytes),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesImmutable =>
					Some ($alternative::IsBytesImmutable),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesMutable =>
					Some ($alternative::IsBytesMutable),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesEmpty =>
					Some ($alternative::IsBytesEmpty),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesImmutableEmpty =>
					Some ($alternative::IsBytesImmutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesMutableEmpty =>
					Some ($alternative::IsBytesMutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesEmptyNot =>
					Some ($alternative::IsBytesEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesImmutableEmptyNot =>
					Some ($alternative::IsBytesImmutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				TypePrimitiveV::IsBytesMutableEmptyNot =>
					Some ($alternative::IsBytesMutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
				TypePrimitiveV::IsStringRegex =>
					Some ($alternative::IsStringRegex),
				#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
				TypePrimitiveV::IsBytesRegex =>
					Some ($alternative::IsBytesRegex),
				TypePrimitiveV::IsPair =>
					Some ($alternative::IsPair),
				TypePrimitiveV::IsPairMutable =>
					Some ($alternative::IsPairMutable),
				TypePrimitiveV::IsPairImmutable =>
					Some ($alternative::IsPairImmutable),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArray =>
					Some ($alternative::IsArray),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayMutable =>
					Some ($alternative::IsArrayMutable),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayImmutable =>
					Some ($alternative::IsArrayImmutable),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayEmpty =>
					Some ($alternative::IsArrayEmpty),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayMutableEmpty =>
					Some ($alternative::IsArrayMutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayImmutableEmpty =>
					Some ($alternative::IsArrayImmutableEmpty),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayEmptyNot =>
					Some ($alternative::IsArrayEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayMutableEmptyNot =>
					Some ($alternative::IsArrayMutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				TypePrimitiveV::IsArrayImmutableEmptyNot =>
					Some ($alternative::IsArrayImmutableEmptyNot),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				TypePrimitiveV::IsValues =>
					Some ($alternative::IsValues),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				TypePrimitiveV::IsValuesEmpty =>
					Some ($alternative::IsValuesEmpty),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				TypePrimitiveV::IsValuesEmptyNot =>
					Some ($alternative::IsValuesEmptyNot),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				TypePrimitiveV::IsRecordKind =>
					Some ($alternative::IsRecordKind),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				TypePrimitiveV::IsRecord =>
					Some ($alternative::IsRecord),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				TypePrimitiveV::IsRecordImmutable =>
					Some ($alternative::IsRecordImmutable),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				TypePrimitiveV::IsRecordMutable =>
					Some ($alternative::IsRecordMutable),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				TypePrimitiveV::IsError =>
					Some ($alternative::IsError),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				TypePrimitiveV::IsErrorSyntax =>
					Some ($alternative::IsErrorSyntax),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsErrorFile =>
					Some ($alternative::IsErrorFile),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsErrorPort =>
					Some ($alternative::IsErrorPort),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsErrorPortInput =>
					Some ($alternative::IsErrorPortInput),
				#[ cfg ( feature = "vonuvoli_values_error" ) ]
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
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
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPort =>
					Some ($alternative::IsPort),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortInput =>
					Some ($alternative::IsPortInput),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortOutput =>
					Some ($alternative::IsPortOutput),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortBinary =>
					Some ($alternative::IsPortBinary),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortTextual =>
					Some ($alternative::IsPortTextual),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortInputBinary =>
					Some ($alternative::IsPortInputBinary),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortInputTextual =>
					Some ($alternative::IsPortInputTextual),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortOutputBinary =>
					Some ($alternative::IsPortOutputBinary),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortOutputTextual =>
					Some ($alternative::IsPortOutputTextual),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				TypePrimitiveV::IsPortEof =>
					Some ($alternative::IsPortEof),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				TypePrimitiveV::IsPath =>
					Some ($alternative::IsPath),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				TypePrimitiveV::IsPathAbsolute =>
					Some ($alternative::IsPathAbsolute),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				TypePrimitiveV::IsPathRelative =>
					Some ($alternative::IsPathRelative),
				#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
				TypePrimitiveV::IsProcess =>
					Some ($alternative::IsProcess),
				#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
				TypePrimitiveV::IsContext =>
					Some ($alternative::IsContext),
				#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
				TypePrimitiveV::IsBinding =>
					Some ($alternative::IsBinding),
				#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
				TypePrimitiveV::IsParameters =>
					Some ($alternative::IsParameters),
				#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
				TypePrimitiveV::IsParameter =>
					Some ($alternative::IsParameter),
				#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
				TypePrimitiveV::IsPromise =>
					Some ($alternative::IsPromise),
				TypePrimitiveV::IsResource =>
					Some ($alternative::IsResource),
				TypePrimitiveV::IsInternal =>
					Some ($alternative::IsInternal),
				#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
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
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterNumeric =>
					Some ($alternative::IsCharacterNumeric),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAlphabetic =>
					Some ($alternative::IsCharacterAlphabetic),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAlphabeticUpperCase =>
					Some ($alternative::IsCharacterAlphabeticUpperCase),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAlphabeticLowerCase =>
					Some ($alternative::IsCharacterAlphabeticLowerCase),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAlphabeticOrNumeric =>
					Some ($alternative::IsCharacterAlphabeticOrNumeric),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterWhitespace =>
					Some ($alternative::IsCharacterWhitespace),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterControl =>
					Some ($alternative::IsCharacterControl),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAscii =>
					Some ($alternative::IsCharacterAscii),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiNumeric =>
					Some ($alternative::IsCharacterAsciiNumeric),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiNumericBase8 =>
					Some ($alternative::IsCharacterAsciiNumericBase8),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiNumericBase16 =>
					Some ($alternative::IsCharacterAsciiNumericBase16),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiAlphabetic =>
					Some ($alternative::IsCharacterAsciiAlphabetic),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiAlphabeticUpperCase =>
					Some ($alternative::IsCharacterAsciiAlphabeticUpperCase),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiAlphabeticLowerCase =>
					Some ($alternative::IsCharacterAsciiAlphabeticLowerCase),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiAlphabeticOrNumeric =>
					Some ($alternative::IsCharacterAsciiAlphabeticOrNumeric),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiWhitespace =>
					Some ($alternative::IsCharacterAsciiWhitespace),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiControl =>
					Some ($alternative::IsCharacterAsciiControl),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				TypePrimitiveV::IsCharacterAsciiPunctuation =>
					Some ($alternative::IsCharacterAsciiPunctuation),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
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

