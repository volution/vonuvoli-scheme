

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
	IsListDotted,
	
	IsProcedure,
	IsSyntax,
	
	IsTrue,
	IsFalse,
	
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
			input.is (ValueClass::Number),
		
		TypePrimitive1::IsList =>
			input.is (ValueClass::List),
		
		TypePrimitive1::IsListProper =>
			input.is (ValueClass::ListProper),
		
		TypePrimitive1::IsListDotted =>
			input.is (ValueClass::ListDotted),
		
		TypePrimitive1::IsProcedure =>
			input.is (ValueClass::Procedure),
		
		TypePrimitive1::IsSyntax =>
			input.is (ValueClass::Syntax),
		
		TypePrimitive1::IsTrue =>
			input.is (ValueClass::True),
		
		TypePrimitive1::IsFalse =>
			input.is (ValueClass::False),
		
	};
	
	succeed! (output.into ());
}

