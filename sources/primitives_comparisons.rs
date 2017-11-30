

use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ComparisonPrimitive0;
	pub use super::ComparisonPrimitive1;
	pub use super::ComparisonPrimitive2;
	pub use super::ComparisonPrimitive3;
	pub use super::ComparisonPrimitive4;
	pub use super::ComparisonPrimitive5;
	pub use super::ComparisonPrimitiveN;
	
	pub use super::comparison_primitive_0_evaluate;
	pub use super::comparison_primitive_1_evaluate;
	pub use super::comparison_primitive_2_evaluate;
	pub use super::comparison_primitive_3_evaluate;
	pub use super::comparison_primitive_4_evaluate;
	pub use super::comparison_primitive_5_evaluate;
	pub use super::comparison_primitive_n_evaluate;
	
	pub use super::comparison_primitive_n_alternative_0;
	pub use super::comparison_primitive_n_alternative_1;
	pub use super::comparison_primitive_n_alternative_2;
	pub use super::comparison_primitive_n_alternative_3;
	pub use super::comparison_primitive_n_alternative_4;
	pub use super::comparison_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive1 {
	
	EquivalentByIdentity,
	EquivalentByValue,
	EquivalentByValueRecursive,
	
	GenericLesser,
	GenericLesserOrEqual,
	GenericEqual,
	GenericGreaterOrEqual,
	GenericGreater,
	
	BooleanLesser,
	BooleanLesserOrEqual,
	BooleanEqual,
	BooleanGreaterOrEqual,
	BooleanGreater,
	
	NumberLesser,
	NumberLesserOrEqual,
	NumberEqual,
	NumberGreaterOrEqual,
	NumberGreater,
	
	CharacterCaseSensitiveLesser,
	CharacterCaseSensitiveLesserOrEqual,
	CharacterCaseSensitiveEqual,
	CharacterCaseSensitiveGreaterOrEqual,
	CharacterCaseSensitiveGreater,
	
	CharacterCaseInsensitiveLesser,
	CharacterCaseInsensitiveLesserOrEqual,
	CharacterCaseInsensitiveEqual,
	CharacterCaseInsensitiveGreaterOrEqual,
	CharacterCaseInsensitiveGreater,
	
	StringCaseSensitiveLesser,
	StringCaseSensitiveLesserOrEqual,
	StringCaseSensitiveEqual,
	StringCaseSensitiveGreaterOrEqual,
	StringCaseSensitiveGreater,
	
	StringCaseInsensitiveLesser,
	StringCaseInsensitiveLesserOrEqual,
	StringCaseInsensitiveEqual,
	StringCaseInsensitiveGreaterOrEqual,
	StringCaseInsensitiveGreater,
	
	SymbolCaseSensitiveLesser,
	SymbolCaseSensitiveLesserOrEqual,
	SymbolCaseSensitiveEqual,
	SymbolCaseSensitiveGreaterOrEqual,
	SymbolCaseSensitiveGreater,
	
	SymbolCaseInsensitiveLesser,
	SymbolCaseInsensitiveLesserOrEqual,
	SymbolCaseInsensitiveEqual,
	SymbolCaseInsensitiveGreaterOrEqual,
	SymbolCaseInsensitiveGreater,
	
	BytesLesser,
	BytesLesserOrEqual,
	BytesEqual,
	BytesGreaterOrEqual,
	BytesGreater,
	
	PairLesser,
	PairLesserOrEqual,
	PairEqual,
	PairGreaterOrEqual,
	PairGreater,
	
	ArrayLesser,
	ArrayLesserOrEqual,
	ArrayEqual,
	ArrayGreaterOrEqual,
	ArrayGreater,
	
	ValuesLesser,
	ValuesLesserOrEqual,
	ValuesEqual,
	ValuesGreaterOrEqual,
	ValuesGreater,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive2 {
	
	EquivalentByIdentity,
	EquivalentByValue,
	EquivalentByValueRecursive,
	
	GenericLesser,
	GenericLesserOrEqual,
	GenericEqual,
	GenericGreaterOrEqual,
	GenericGreater,
	
	BooleanLesser,
	BooleanLesserOrEqual,
	BooleanEqual,
	BooleanGreaterOrEqual,
	BooleanGreater,
	
	NumberLesser,
	NumberLesserOrEqual,
	NumberEqual,
	NumberGreaterOrEqual,
	NumberGreater,
	
	CharacterCaseSensitiveLesser,
	CharacterCaseSensitiveLesserOrEqual,
	CharacterCaseSensitiveEqual,
	CharacterCaseSensitiveGreaterOrEqual,
	CharacterCaseSensitiveGreater,
	
	CharacterCaseInsensitiveLesser,
	CharacterCaseInsensitiveLesserOrEqual,
	CharacterCaseInsensitiveEqual,
	CharacterCaseInsensitiveGreaterOrEqual,
	CharacterCaseInsensitiveGreater,
	
	StringCaseSensitiveLesser,
	StringCaseSensitiveLesserOrEqual,
	StringCaseSensitiveEqual,
	StringCaseSensitiveGreaterOrEqual,
	StringCaseSensitiveGreater,
	
	StringCaseInsensitiveLesser,
	StringCaseInsensitiveLesserOrEqual,
	StringCaseInsensitiveEqual,
	StringCaseInsensitiveGreaterOrEqual,
	StringCaseInsensitiveGreater,
	
	SymbolCaseSensitiveLesser,
	SymbolCaseSensitiveLesserOrEqual,
	SymbolCaseSensitiveEqual,
	SymbolCaseSensitiveGreaterOrEqual,
	SymbolCaseSensitiveGreater,
	
	SymbolCaseInsensitiveLesser,
	SymbolCaseInsensitiveLesserOrEqual,
	SymbolCaseInsensitiveEqual,
	SymbolCaseInsensitiveGreaterOrEqual,
	SymbolCaseInsensitiveGreater,
	
	BytesLesser,
	BytesLesserOrEqual,
	BytesEqual,
	BytesGreaterOrEqual,
	BytesGreater,
	
	PairLesser,
	PairLesserOrEqual,
	PairEqual,
	PairGreaterOrEqual,
	PairGreater,
	
	ArrayLesser,
	ArrayLesserOrEqual,
	ArrayEqual,
	ArrayGreaterOrEqual,
	ArrayGreater,
	
	ValuesLesser,
	ValuesLesserOrEqual,
	ValuesEqual,
	ValuesGreaterOrEqual,
	ValuesGreater,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitiveN {
	
	EquivalentByIdentity,
	EquivalentByValue,
	EquivalentByValueRecursive,
	
	GenericLesser,
	GenericLesserOrEqual,
	GenericEqual,
	GenericGreaterOrEqual,
	GenericGreater,
	
	BooleanLesser,
	BooleanLesserOrEqual,
	BooleanEqual,
	BooleanGreaterOrEqual,
	BooleanGreater,
	
	NumberLesser,
	NumberLesserOrEqual,
	NumberEqual,
	NumberGreaterOrEqual,
	NumberGreater,
	
	CharacterCaseSensitiveLesser,
	CharacterCaseSensitiveLesserOrEqual,
	CharacterCaseSensitiveEqual,
	CharacterCaseSensitiveGreaterOrEqual,
	CharacterCaseSensitiveGreater,
	
	CharacterCaseInsensitiveLesser,
	CharacterCaseInsensitiveLesserOrEqual,
	CharacterCaseInsensitiveEqual,
	CharacterCaseInsensitiveGreaterOrEqual,
	CharacterCaseInsensitiveGreater,
	
	StringCaseSensitiveLesser,
	StringCaseSensitiveLesserOrEqual,
	StringCaseSensitiveEqual,
	StringCaseSensitiveGreaterOrEqual,
	StringCaseSensitiveGreater,
	
	StringCaseInsensitiveLesser,
	StringCaseInsensitiveLesserOrEqual,
	StringCaseInsensitiveEqual,
	StringCaseInsensitiveGreaterOrEqual,
	StringCaseInsensitiveGreater,
	
	SymbolCaseSensitiveLesser,
	SymbolCaseSensitiveLesserOrEqual,
	SymbolCaseSensitiveEqual,
	SymbolCaseSensitiveGreaterOrEqual,
	SymbolCaseSensitiveGreater,
	
	SymbolCaseInsensitiveLesser,
	SymbolCaseInsensitiveLesserOrEqual,
	SymbolCaseInsensitiveEqual,
	SymbolCaseInsensitiveGreaterOrEqual,
	SymbolCaseInsensitiveGreater,
	
	BytesLesser,
	BytesLesserOrEqual,
	BytesEqual,
	BytesGreaterOrEqual,
	BytesGreater,
	
	PairLesser,
	PairLesserOrEqual,
	PairEqual,
	PairGreaterOrEqual,
	PairGreater,
	
	ArrayLesser,
	ArrayLesserOrEqual,
	ArrayEqual,
	ArrayGreaterOrEqual,
	ArrayGreater,
	
	ValuesLesser,
	ValuesLesserOrEqual,
	ValuesEqual,
	ValuesGreaterOrEqual,
	ValuesGreater,
	
}




pub fn comparison_primitive_0_evaluate (primitive : ComparisonPrimitive0) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, _input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		_ =>
			fail_unimplemented! (0xc4e740a4),
	}
}




pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, _input_1 : &Value, _input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		_ =>
			fail_unimplemented! (0x72639c4a),
	}
}




pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, _inputs : &[Value]) -> (Outcome<Value>) {
	match primitive {
		_ =>
			fail_unimplemented! (0x48241946),
	}
}




pub fn comparison_primitive_n_alternative_0 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive0>) {
	match primitive {
		_ =>
			None,
	}
}


pub fn comparison_primitive_n_alternative_1 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive1>) {
	match primitive {
		ComparisonPrimitiveN::EquivalentByIdentity =>
			Some (ComparisonPrimitive1::EquivalentByIdentity),
		ComparisonPrimitiveN::EquivalentByValue =>
			Some (ComparisonPrimitive1::EquivalentByValue),
		ComparisonPrimitiveN::EquivalentByValueRecursive =>
			Some (ComparisonPrimitive1::EquivalentByValueRecursive),
		ComparisonPrimitiveN::GenericLesser =>
			Some (ComparisonPrimitive1::GenericLesser),
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			Some (ComparisonPrimitive1::GenericLesserOrEqual),
		ComparisonPrimitiveN::GenericEqual =>
			Some (ComparisonPrimitive1::GenericEqual),
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive1::GenericGreaterOrEqual),
		ComparisonPrimitiveN::GenericGreater =>
			Some (ComparisonPrimitive1::GenericGreater),
		ComparisonPrimitiveN::BooleanLesser =>
			Some (ComparisonPrimitive1::BooleanLesser),
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive1::BooleanLesserOrEqual),
		ComparisonPrimitiveN::BooleanEqual =>
			Some (ComparisonPrimitive1::BooleanEqual),
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive1::BooleanGreaterOrEqual),
		ComparisonPrimitiveN::BooleanGreater =>
			Some (ComparisonPrimitive1::BooleanGreater),
		ComparisonPrimitiveN::NumberLesser =>
			Some (ComparisonPrimitive1::NumberLesser),
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			Some (ComparisonPrimitive1::NumberLesserOrEqual),
		ComparisonPrimitiveN::NumberEqual =>
			Some (ComparisonPrimitive1::NumberEqual),
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive1::NumberGreaterOrEqual),
		ComparisonPrimitiveN::NumberGreater =>
			Some (ComparisonPrimitive1::NumberGreater),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::StringCaseSensitiveLesser),
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::StringCaseSensitiveGreater),
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveLesser),
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveN::BytesLesser =>
			Some (ComparisonPrimitive1::BytesLesser),
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			Some (ComparisonPrimitive1::BytesLesserOrEqual),
		ComparisonPrimitiveN::BytesEqual =>
			Some (ComparisonPrimitive1::BytesEqual),
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive1::BytesGreaterOrEqual),
		ComparisonPrimitiveN::BytesGreater =>
			Some (ComparisonPrimitive1::BytesGreater),
		ComparisonPrimitiveN::PairLesser =>
			Some (ComparisonPrimitive1::PairLesser),
		ComparisonPrimitiveN::PairLesserOrEqual =>
			Some (ComparisonPrimitive1::PairLesserOrEqual),
		ComparisonPrimitiveN::PairEqual =>
			Some (ComparisonPrimitive1::PairEqual),
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			Some (ComparisonPrimitive1::PairGreaterOrEqual),
		ComparisonPrimitiveN::PairGreater =>
			Some (ComparisonPrimitive1::PairGreater),
		ComparisonPrimitiveN::ArrayLesser =>
			Some (ComparisonPrimitive1::ArrayLesser),
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive1::ArrayLesserOrEqual),
		ComparisonPrimitiveN::ArrayEqual =>
			Some (ComparisonPrimitive1::ArrayEqual),
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive1::ArrayGreaterOrEqual),
		ComparisonPrimitiveN::ArrayGreater =>
			Some (ComparisonPrimitive1::ArrayGreater),
		ComparisonPrimitiveN::ValuesLesser =>
			Some (ComparisonPrimitive1::ValuesLesser),
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive1::ValuesLesserOrEqual),
		ComparisonPrimitiveN::ValuesEqual =>
			Some (ComparisonPrimitive1::ValuesEqual),
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive1::ValuesGreaterOrEqual),
		ComparisonPrimitiveN::ValuesGreater =>
			Some (ComparisonPrimitive1::ValuesGreater),
	}
}


pub fn comparison_primitive_n_alternative_2 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive2>) {
	match primitive {
		ComparisonPrimitiveN::EquivalentByIdentity =>
			Some (ComparisonPrimitive2::EquivalentByIdentity),
		ComparisonPrimitiveN::EquivalentByValue =>
			Some (ComparisonPrimitive2::EquivalentByValue),
		ComparisonPrimitiveN::EquivalentByValueRecursive =>
			Some (ComparisonPrimitive2::EquivalentByValueRecursive),
		ComparisonPrimitiveN::GenericLesser =>
			Some (ComparisonPrimitive2::GenericLesser),
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			Some (ComparisonPrimitive2::GenericLesserOrEqual),
		ComparisonPrimitiveN::GenericEqual =>
			Some (ComparisonPrimitive2::GenericEqual),
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive2::GenericGreaterOrEqual),
		ComparisonPrimitiveN::GenericGreater =>
			Some (ComparisonPrimitive2::GenericGreater),
		ComparisonPrimitiveN::BooleanLesser =>
			Some (ComparisonPrimitive2::BooleanLesser),
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive2::BooleanLesserOrEqual),
		ComparisonPrimitiveN::BooleanEqual =>
			Some (ComparisonPrimitive2::BooleanEqual),
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive2::BooleanGreaterOrEqual),
		ComparisonPrimitiveN::BooleanGreater =>
			Some (ComparisonPrimitive2::BooleanGreater),
		ComparisonPrimitiveN::NumberLesser =>
			Some (ComparisonPrimitive2::NumberLesser),
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			Some (ComparisonPrimitive2::NumberLesserOrEqual),
		ComparisonPrimitiveN::NumberEqual =>
			Some (ComparisonPrimitive2::NumberEqual),
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive2::NumberGreaterOrEqual),
		ComparisonPrimitiveN::NumberGreater =>
			Some (ComparisonPrimitive2::NumberGreater),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::StringCaseSensitiveLesser),
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::StringCaseSensitiveGreater),
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveLesser),
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveN::BytesLesser =>
			Some (ComparisonPrimitive2::BytesLesser),
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			Some (ComparisonPrimitive2::BytesLesserOrEqual),
		ComparisonPrimitiveN::BytesEqual =>
			Some (ComparisonPrimitive2::BytesEqual),
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive2::BytesGreaterOrEqual),
		ComparisonPrimitiveN::BytesGreater =>
			Some (ComparisonPrimitive2::BytesGreater),
		ComparisonPrimitiveN::PairLesser =>
			Some (ComparisonPrimitive2::PairLesser),
		ComparisonPrimitiveN::PairLesserOrEqual =>
			Some (ComparisonPrimitive2::PairLesserOrEqual),
		ComparisonPrimitiveN::PairEqual =>
			Some (ComparisonPrimitive2::PairEqual),
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			Some (ComparisonPrimitive2::PairGreaterOrEqual),
		ComparisonPrimitiveN::PairGreater =>
			Some (ComparisonPrimitive2::PairGreater),
		ComparisonPrimitiveN::ArrayLesser =>
			Some (ComparisonPrimitive2::ArrayLesser),
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive2::ArrayLesserOrEqual),
		ComparisonPrimitiveN::ArrayEqual =>
			Some (ComparisonPrimitive2::ArrayEqual),
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive2::ArrayGreaterOrEqual),
		ComparisonPrimitiveN::ArrayGreater =>
			Some (ComparisonPrimitive2::ArrayGreater),
		ComparisonPrimitiveN::ValuesLesser =>
			Some (ComparisonPrimitive2::ValuesLesser),
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive2::ValuesLesserOrEqual),
		ComparisonPrimitiveN::ValuesEqual =>
			Some (ComparisonPrimitive2::ValuesEqual),
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive2::ValuesGreaterOrEqual),
		ComparisonPrimitiveN::ValuesGreater =>
			Some (ComparisonPrimitive2::ValuesGreater),
	}
}


pub fn comparison_primitive_n_alternative_3 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive3>) {
	match primitive {
		_ =>
			None,
	}
}


pub fn comparison_primitive_n_alternative_4 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive4>) {
	match primitive {
		_ =>
			None,
	}
}


pub fn comparison_primitive_n_alternative_5 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive5>) {
	match primitive {
		_ =>
			None,
	}
}

