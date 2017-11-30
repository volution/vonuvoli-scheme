

use super::builtins::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::TypePrimitive1;
	
	pub use super::type_primitive_1_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
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
	
}




pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	
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
		
		TypePrimitive1::IsNotFalse =>
			is_not_false (input_1),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			is_true_or_equivalent (input_1),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			is_false_or_equivalent (input_1),
		
		TypePrimitive1::IsCharacterNumeric =>
			fail_unimplemented! (0x369a2267),
		
		TypePrimitive1::IsCharacterAlphabetic =>
			fail_unimplemented! (0xa74af316),
		
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			fail_unimplemented! (0x2e44dce2),
		
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			fail_unimplemented! (0x18a55f25),
		
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			fail_unimplemented! (0x592dcd5d),
		
		TypePrimitive1::IsCharacterWhitespace =>
			fail_unimplemented! (0xa66e8d9c),
		
		TypePrimitive1::IsCharacterControl =>
			fail_unimplemented! (0x502c6e07),
		
		TypePrimitive1::IsCharacterAscii =>
			fail_unimplemented! (0xf18721e8),
		
		TypePrimitive1::IsCharacterAsciiNumeric =>
			fail_unimplemented! (0x975a21a6),
		
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			fail_unimplemented! (0xefddca04),
		
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			fail_unimplemented! (0x9849c1c5),
		
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			fail_unimplemented! (0xe5e4cd71),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			fail_unimplemented! (0x9229646e),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			fail_unimplemented! (0xf552b50f),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			fail_unimplemented! (0x34a71556),
		
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			fail_unimplemented! (0x18cdd61f),
		
		TypePrimitive1::IsCharacterAsciiControl =>
			fail_unimplemented! (0x5572e51b),
		
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			fail_unimplemented! (0x576a2df0),
		
		TypePrimitive1::IsCharacterAsciiGraphic =>
			fail_unimplemented! (0x70b94759),
		
	};
	
	succeed! (output.into ());
}

