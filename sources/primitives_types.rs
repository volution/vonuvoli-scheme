

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
	IsVoid,
	IsUndefined,
	
	IsBoolean,
	IsNumberInteger,
	IsNumberReal,
	IsCharacter,
	IsSymbol,
	IsString,
	IsBytes,
	IsPair,
	IsArray,
	IsValues,
	
	IsError,
	IsErrorSyntax,
	IsErrorFile,
	IsErrorPort,
	IsErrorPortInput,
	IsErrorPortOutput,
	
	IsNumber,
	
	IsList,
	IsListProper,
	IsListProperOrEmpty,
	IsListDotted,
	IsListDottedOrEmpty,
	IsListCyclic,
	IsListCyclicOrEmpty,
	
	IsProcedure,
	IsSyntax,
	
	IsTrue,
	IsFalse,
	IsNotTrue,
	IsNotFalse,
	IsTrueOrEquivalent,
	IsFalseOrEquivalent,
	
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
	
	IsPort,
	IsPortInput,
	IsPortOutput,
	IsPortBinary,
	IsPortTextual,
	IsPortEof,
	
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
			input_1.is (ValueClass::Null),
		
		TypePrimitive1::IsVoid =>
			input_1.is (ValueClass::Void),
		
		TypePrimitive1::IsUndefined =>
			input_1.is (ValueClass::Undefined),
		
		TypePrimitive1::IsBoolean =>
			input_1.is (ValueClass::Boolean),
		
		TypePrimitive1::IsNumberInteger =>
			input_1.is (ValueClass::NumberInteger),
		
		TypePrimitive1::IsNumberReal =>
			input_1.is (ValueClass::NumberReal),
		
		TypePrimitive1::IsCharacter =>
			input_1.is (ValueClass::Character),
		
		TypePrimitive1::IsSymbol =>
			input_1.is (ValueClass::Symbol),
		
		TypePrimitive1::IsString =>
			input_1.is (ValueClass::String),
		
		TypePrimitive1::IsBytes =>
			input_1.is (ValueClass::Bytes),
		
		TypePrimitive1::IsPair =>
			input_1.is (ValueClass::Pair),
		
		TypePrimitive1::IsArray =>
			input_1.is (ValueClass::Array),
		
		TypePrimitive1::IsValues =>
			input_1.is (ValueClass::Values),
		
		TypePrimitive1::IsError =>
			input_1.is (ValueClass::Error),
		
		TypePrimitive1::IsErrorSyntax =>
			fail_unimplemented! (0x6519f984),
		
		TypePrimitive1::IsErrorFile =>
			fail_unimplemented! (0xa2f85ee0),
		
		TypePrimitive1::IsErrorPort =>
			fail_unimplemented! (0x758847be),
		
		TypePrimitive1::IsErrorPortInput =>
			fail_unimplemented! (0x5da25665),
		
		TypePrimitive1::IsErrorPortOutput =>
			fail_unimplemented! (0x03b1e64b),
		
		TypePrimitive1::IsNumber =>
			is_number (input_1),
		
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
		
		TypePrimitive1::IsTrue =>
			is_true (input_1),
		
		TypePrimitive1::IsFalse =>
			is_false (input_1),
		
		TypePrimitive1::IsNotTrue =>
			is_not_true (input_1),
		
		TypePrimitive1::IsNotFalse =>
			is_not_false (input_1),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			is_true_or_equivalent (input_1),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			is_false_or_equivalent (input_1),
		
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
		
	};
	succeed! (output);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn type_primitive_1_attributes (_primitive : TypePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

