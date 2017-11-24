

use super::builtins::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::TypePrimitive1;
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




pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input : &Value) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		TypePrimitive1::IsNull =>
			input.is (ValueClass::Null),
		
		TypePrimitive1::IsVoid =>
			input.is (ValueClass::Void),
		
		TypePrimitive1::IsUndefined =>
			input.is (ValueClass::Undefined),
		
		TypePrimitive1::IsBoolean =>
			input.is (ValueClass::Boolean),
		
		TypePrimitive1::IsNumberInteger =>
			input.is (ValueClass::NumberInteger),
		
		TypePrimitive1::IsNumberReal =>
			input.is (ValueClass::NumberReal),
		
		TypePrimitive1::IsCharacter =>
			input.is (ValueClass::Character),
		
		TypePrimitive1::IsSymbol =>
			input.is (ValueClass::Symbol),
		
		TypePrimitive1::IsString =>
			input.is (ValueClass::String),
		
		TypePrimitive1::IsBytes =>
			input.is (ValueClass::Bytes),
		
		TypePrimitive1::IsPair =>
			input.is (ValueClass::Pair),
		
		TypePrimitive1::IsArray =>
			input.is (ValueClass::Array),
		
		TypePrimitive1::IsError =>
			input.is (ValueClass::Error),
		
		TypePrimitive1::IsNumber =>
			is_number (input),
		
		TypePrimitive1::IsList =>
			is_list (input),
		
		TypePrimitive1::IsListProper =>
			is_list_proper (input),
		
		TypePrimitive1::IsListProperOrEmpty =>
			is_list_proper_or_empty (input),
		
		TypePrimitive1::IsListDotted =>
			is_list_dotted (input),
		
		TypePrimitive1::IsListDottedOrEmpty =>
			is_list_dotted_or_empty (input),
		
		TypePrimitive1::IsListCyclic =>
			is_list_cyclic (input),
		
		TypePrimitive1::IsListCyclicOrEmpty =>
			is_list_cyclic_or_empty (input),
		
		TypePrimitive1::IsProcedure =>
			is_procedure (input),
		
		TypePrimitive1::IsSyntax =>
			is_syntax (input),
		
		TypePrimitive1::IsTrue =>
			is_true (input),
		
		TypePrimitive1::IsFalse =>
			is_false (input),
		
		TypePrimitive1::IsNotFalse =>
			is_not_false (input),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			is_true_or_equivalent (input),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			is_false_or_equivalent (input),
		
	};
	
	succeed! (output.into ());
}

