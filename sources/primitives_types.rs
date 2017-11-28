

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
	IsError,
	
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
		
		TypePrimitive1::IsError =>
			input_1.is (ValueClass::Error),
		
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
		
	};
	
	succeed! (output.into ());
}

