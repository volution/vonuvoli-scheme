

use super::builtins::exports::*;
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




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive1 {
	
	EquivalentByIdentity,
	EquivalentByValueStrict,
	EquivalentByValueStrictRecursive,
	EquivalentByValueCoerced,
	EquivalentByValueCoercedRecursive,
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive2 {
	
	EquivalentByIdentity,
	EquivalentByValueStrict,
	EquivalentByValueStrictRecursive,
	EquivalentByValueCoerced,
	EquivalentByValueCoercedRecursive,
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive3 {
	
	EquivalentByIdentity,
	EquivalentByValueStrict,
	EquivalentByValueStrictRecursive,
	EquivalentByValueCoerced,
	EquivalentByValueCoercedRecursive,
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive4 {
	
	EquivalentByIdentity,
	EquivalentByValueStrict,
	EquivalentByValueStrictRecursive,
	EquivalentByValueCoerced,
	EquivalentByValueCoercedRecursive,
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitiveN {
	
	EquivalentByIdentity,
	EquivalentByValueStrict,
	EquivalentByValueStrictRecursive,
	EquivalentByValueCoerced,
	EquivalentByValueCoercedRecursive,
	
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




pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	let output = match primitive {
		
		ComparisonPrimitive1::EquivalentByIdentity =>
			try! (compare_1 (input_1, Comparison::Equivalence (Equivalence::ByIdentity, None, None))),
		
		ComparisonPrimitive1::EquivalentByValueStrict =>
			try! (compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)))),
		
		ComparisonPrimitive1::EquivalentByValueStrictRecursive =>
			try! (compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)))),
		
		ComparisonPrimitive1::EquivalentByValueCoerced =>
			try! (compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)))),
		
		ComparisonPrimitive1::EquivalentByValueCoercedRecursive =>
			try! (compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)))),
		
		ComparisonPrimitive1::GenericLesser =>
			try! (compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::GenericLesserOrEqual =>
			try! (compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::GenericEqual =>
			try! (compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::GenericGreaterOrEqual =>
			try! (compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::GenericGreater =>
			try! (compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::BooleanLesser =>
			try! (boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::BooleanLesserOrEqual =>
			try! (boolean_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::BooleanEqual =>
			try! (boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::BooleanGreaterOrEqual =>
			try! (boolean_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::BooleanGreater =>
			try! (boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::NumberLesser =>
			try! (number_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::NumberLesserOrEqual =>
			try! (number_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::NumberEqual =>
			try! (number_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::NumberGreaterOrEqual =>
			try! (number_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::NumberGreater =>
			try! (number_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::CharacterCaseSensitiveLesser =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::CharacterCaseSensitiveEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::CharacterCaseSensitiveGreater =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveLesser =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveGreater =>
			try! (character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive1::StringCaseSensitiveLesser =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::StringCaseSensitiveEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::StringCaseSensitiveGreater =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive1::StringCaseInsensitiveLesser =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::StringCaseInsensitiveEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::StringCaseInsensitiveGreater =>
			try! (string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive1::BytesLesser =>
			try! (bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::BytesLesserOrEqual =>
			try! (bytes_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::BytesEqual =>
			try! (bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::BytesGreaterOrEqual =>
			try! (bytes_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::BytesGreater =>
			try! (bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::PairLesser =>
			try! (pair_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::PairLesserOrEqual =>
			try! (pair_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::PairEqual =>
			try! (pair_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::PairGreaterOrEqual =>
			try! (pair_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::PairGreater =>
			try! (pair_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::ArrayLesser =>
			try! (array_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::ArrayLesserOrEqual =>
			try! (array_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::ArrayEqual =>
			try! (array_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::ArrayGreaterOrEqual =>
			try! (array_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::ArrayGreater =>
			try! (array_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive1::ValuesLesser =>
			try! (values_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive1::ValuesLesserOrEqual =>
			try! (values_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive1::ValuesEqual =>
			try! (values_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive1::ValuesGreaterOrEqual =>
			try! (values_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive1::ValuesGreater =>
			try! (values_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
}




pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	let output = match primitive {
		
		ComparisonPrimitive2::EquivalentByIdentity =>
			try! (compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByIdentity, None, None))),
		
		ComparisonPrimitive2::EquivalentByValueStrict =>
			try! (compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)))),
		
		ComparisonPrimitive2::EquivalentByValueStrictRecursive =>
			try! (compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)))),
		
		ComparisonPrimitive2::EquivalentByValueCoerced =>
			try! (compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)))),
		
		ComparisonPrimitive2::EquivalentByValueCoercedRecursive =>
			try! (compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)))),
		
		ComparisonPrimitive2::GenericLesser =>
			try! (compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::GenericLesserOrEqual =>
			try! (compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::GenericEqual =>
			try! (compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::GenericGreaterOrEqual =>
			try! (compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::GenericGreater =>
			try! (compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::BooleanLesser =>
			try! (boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::BooleanLesserOrEqual =>
			try! (boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::BooleanEqual =>
			try! (boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::BooleanGreaterOrEqual =>
			try! (boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::BooleanGreater =>
			try! (boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::NumberLesser =>
			try! (number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::NumberLesserOrEqual =>
			try! (number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::NumberEqual =>
			try! (number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::NumberGreaterOrEqual =>
			try! (number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::NumberGreater =>
			try! (number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveLesser =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreater =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesser =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreater =>
			try! (character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseSensitiveLesser =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveGreater =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesser =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreater =>
			try! (string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::BytesLesser =>
			try! (bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::BytesLesserOrEqual =>
			try! (bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::BytesEqual =>
			try! (bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::BytesGreaterOrEqual =>
			try! (bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::BytesGreater =>
			try! (bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::PairLesser =>
			try! (pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::PairLesserOrEqual =>
			try! (pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::PairEqual =>
			try! (pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::PairGreaterOrEqual =>
			try! (pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::PairGreater =>
			try! (pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::ArrayLesser =>
			try! (array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::ArrayLesserOrEqual =>
			try! (array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::ArrayEqual =>
			try! (array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::ArrayGreaterOrEqual =>
			try! (array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::ArrayGreater =>
			try! (array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::ValuesLesser =>
			try! (values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::ValuesLesserOrEqual =>
			try! (values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::ValuesEqual =>
			try! (values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::ValuesGreaterOrEqual =>
			try! (values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::ValuesGreater =>
			try! (values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
}




pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	let output = match primitive {
		
		ComparisonPrimitive3::EquivalentByIdentity =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByIdentity, None, None))),
		
		ComparisonPrimitive3::EquivalentByValueStrict =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)))),
		
		ComparisonPrimitive3::EquivalentByValueStrictRecursive =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)))),
		
		ComparisonPrimitive3::EquivalentByValueCoerced =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)))),
		
		ComparisonPrimitive3::EquivalentByValueCoercedRecursive =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)))),
		
		ComparisonPrimitive3::GenericLesser =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::GenericLesserOrEqual =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::GenericEqual =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::GenericGreaterOrEqual =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::GenericGreater =>
			try! (compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::BooleanLesser =>
			try! (boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::BooleanLesserOrEqual =>
			try! (boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::BooleanEqual =>
			try! (boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::BooleanGreaterOrEqual =>
			try! (boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::BooleanGreater =>
			try! (boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::NumberLesser =>
			try! (number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::NumberLesserOrEqual =>
			try! (number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::NumberEqual =>
			try! (number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::NumberGreaterOrEqual =>
			try! (number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::NumberGreater =>
			try! (number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::CharacterCaseSensitiveLesser =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive3::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::CharacterCaseSensitiveEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive3::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::CharacterCaseSensitiveGreater =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveLesser =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveGreater =>
			try! (character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive3::StringCaseSensitiveLesser =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive3::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::StringCaseSensitiveEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive3::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::StringCaseSensitiveGreater =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive3::StringCaseInsensitiveLesser =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive3::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::StringCaseInsensitiveEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive3::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::StringCaseInsensitiveGreater =>
			try! (string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive3::BytesLesser =>
			try! (bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::BytesLesserOrEqual =>
			try! (bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::BytesEqual =>
			try! (bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::BytesGreaterOrEqual =>
			try! (bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::BytesGreater =>
			try! (bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::PairLesser =>
			try! (pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::PairLesserOrEqual =>
			try! (pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::PairEqual =>
			try! (pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::PairGreaterOrEqual =>
			try! (pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::PairGreater =>
			try! (pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::ArrayLesser =>
			try! (array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::ArrayLesserOrEqual =>
			try! (array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::ArrayEqual =>
			try! (array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::ArrayGreaterOrEqual =>
			try! (array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::ArrayGreater =>
			try! (array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive3::ValuesLesser =>
			try! (values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive3::ValuesLesserOrEqual =>
			try! (values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive3::ValuesEqual =>
			try! (values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive3::ValuesGreaterOrEqual =>
			try! (values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive3::ValuesGreater =>
			try! (values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
}




pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	let output = match primitive {
		
		ComparisonPrimitive4::EquivalentByIdentity =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByIdentity, None, None))),
		
		ComparisonPrimitive4::EquivalentByValueStrict =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)))),
		
		ComparisonPrimitive4::EquivalentByValueStrictRecursive =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)))),
		
		ComparisonPrimitive4::EquivalentByValueCoerced =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)))),
		
		ComparisonPrimitive4::EquivalentByValueCoercedRecursive =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)))),
		
		ComparisonPrimitive4::GenericLesser =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::GenericLesserOrEqual =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::GenericEqual =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::GenericGreaterOrEqual =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::GenericGreater =>
			try! (compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::BooleanLesser =>
			try! (boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::BooleanLesserOrEqual =>
			try! (boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::BooleanEqual =>
			try! (boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::BooleanGreaterOrEqual =>
			try! (boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::BooleanGreater =>
			try! (boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::NumberLesser =>
			try! (number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::NumberLesserOrEqual =>
			try! (number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::NumberEqual =>
			try! (number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::NumberGreaterOrEqual =>
			try! (number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::NumberGreater =>
			try! (number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::CharacterCaseSensitiveLesser =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive4::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::CharacterCaseSensitiveEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive4::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::CharacterCaseSensitiveGreater =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveLesser =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveGreater =>
			try! (character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive4::StringCaseSensitiveLesser =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive4::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::StringCaseSensitiveEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive4::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::StringCaseSensitiveGreater =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive4::StringCaseInsensitiveLesser =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive4::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::StringCaseInsensitiveEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive4::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::StringCaseInsensitiveGreater =>
			try! (string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive4::BytesLesser =>
			try! (bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::BytesLesserOrEqual =>
			try! (bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::BytesEqual =>
			try! (bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::BytesGreaterOrEqual =>
			try! (bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::BytesGreater =>
			try! (bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::PairLesser =>
			try! (pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::PairLesserOrEqual =>
			try! (pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::PairEqual =>
			try! (pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::PairGreaterOrEqual =>
			try! (pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::PairGreater =>
			try! (pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::ArrayLesser =>
			try! (array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::ArrayLesserOrEqual =>
			try! (array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::ArrayEqual =>
			try! (array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::ArrayGreaterOrEqual =>
			try! (array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::ArrayGreater =>
			try! (array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive4::ValuesLesser =>
			try! (values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive4::ValuesLesserOrEqual =>
			try! (values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive4::ValuesEqual =>
			try! (values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive4::ValuesGreaterOrEqual =>
			try! (values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive4::ValuesGreater =>
			try! (values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
}




pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let output = match primitive {
		
		ComparisonPrimitiveN::EquivalentByIdentity =>
			try! (compare_n (inputs, Comparison::Equivalence (Equivalence::ByIdentity, None, None))),
		
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			try! (compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false)))),
		
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			try! (compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true)))),
		
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			try! (compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false)))),
		
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			try! (compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true)))),
		
		ComparisonPrimitiveN::GenericLesser =>
			try! (compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			try! (compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::GenericEqual =>
			try! (compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			try! (compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::GenericGreater =>
			try! (compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::BooleanLesser =>
			try! (boolean_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			try! (boolean_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::BooleanEqual =>
			try! (boolean_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			try! (boolean_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::BooleanGreater =>
			try! (boolean_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::NumberLesser =>
			try! (number_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			try! (number_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::NumberEqual =>
			try! (number_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			try! (number_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::NumberGreater =>
			try! (number_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			try! (character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			try! (string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitiveN::BytesLesser =>
			try! (bytes_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			try! (bytes_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::BytesEqual =>
			try! (bytes_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			try! (bytes_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::BytesGreater =>
			try! (bytes_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::PairLesser =>
			try! (pair_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::PairLesserOrEqual =>
			try! (pair_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::PairEqual =>
			try! (pair_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			try! (pair_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::PairGreater =>
			try! (pair_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::ArrayLesser =>
			try! (array_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			try! (array_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::ArrayEqual =>
			try! (array_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			try! (array_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::ArrayGreater =>
			try! (array_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitiveN::ValuesLesser =>
			try! (values_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			try! (values_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitiveN::ValuesEqual =>
			try! (values_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			try! (values_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitiveN::ValuesGreater =>
			try! (values_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
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
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			Some (ComparisonPrimitive1::EquivalentByValueStrict),
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive1::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive1::EquivalentByValueCoerced),
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive1::EquivalentByValueCoercedRecursive),
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
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			Some (ComparisonPrimitive2::EquivalentByValueStrict),
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive2::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive2::EquivalentByValueCoerced),
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive2::EquivalentByValueCoercedRecursive),
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
		ComparisonPrimitiveN::EquivalentByIdentity =>
			Some (ComparisonPrimitive3::EquivalentByIdentity),
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			Some (ComparisonPrimitive3::EquivalentByValueStrict),
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive3::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive3::EquivalentByValueCoerced),
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive3::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveN::GenericLesser =>
			Some (ComparisonPrimitive3::GenericLesser),
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			Some (ComparisonPrimitive3::GenericLesserOrEqual),
		ComparisonPrimitiveN::GenericEqual =>
			Some (ComparisonPrimitive3::GenericEqual),
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive3::GenericGreaterOrEqual),
		ComparisonPrimitiveN::GenericGreater =>
			Some (ComparisonPrimitive3::GenericGreater),
		ComparisonPrimitiveN::BooleanLesser =>
			Some (ComparisonPrimitive3::BooleanLesser),
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive3::BooleanLesserOrEqual),
		ComparisonPrimitiveN::BooleanEqual =>
			Some (ComparisonPrimitive3::BooleanEqual),
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive3::BooleanGreaterOrEqual),
		ComparisonPrimitiveN::BooleanGreater =>
			Some (ComparisonPrimitive3::BooleanGreater),
		ComparisonPrimitiveN::NumberLesser =>
			Some (ComparisonPrimitive3::NumberLesser),
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			Some (ComparisonPrimitive3::NumberLesserOrEqual),
		ComparisonPrimitiveN::NumberEqual =>
			Some (ComparisonPrimitive3::NumberEqual),
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive3::NumberGreaterOrEqual),
		ComparisonPrimitiveN::NumberGreater =>
			Some (ComparisonPrimitive3::NumberGreater),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::StringCaseSensitiveLesser),
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::StringCaseSensitiveGreater),
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveLesser),
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveN::BytesLesser =>
			Some (ComparisonPrimitive3::BytesLesser),
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			Some (ComparisonPrimitive3::BytesLesserOrEqual),
		ComparisonPrimitiveN::BytesEqual =>
			Some (ComparisonPrimitive3::BytesEqual),
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive3::BytesGreaterOrEqual),
		ComparisonPrimitiveN::BytesGreater =>
			Some (ComparisonPrimitive3::BytesGreater),
		ComparisonPrimitiveN::PairLesser =>
			Some (ComparisonPrimitive3::PairLesser),
		ComparisonPrimitiveN::PairLesserOrEqual =>
			Some (ComparisonPrimitive3::PairLesserOrEqual),
		ComparisonPrimitiveN::PairEqual =>
			Some (ComparisonPrimitive3::PairEqual),
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			Some (ComparisonPrimitive3::PairGreaterOrEqual),
		ComparisonPrimitiveN::PairGreater =>
			Some (ComparisonPrimitive3::PairGreater),
		ComparisonPrimitiveN::ArrayLesser =>
			Some (ComparisonPrimitive3::ArrayLesser),
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive3::ArrayLesserOrEqual),
		ComparisonPrimitiveN::ArrayEqual =>
			Some (ComparisonPrimitive3::ArrayEqual),
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive3::ArrayGreaterOrEqual),
		ComparisonPrimitiveN::ArrayGreater =>
			Some (ComparisonPrimitive3::ArrayGreater),
		ComparisonPrimitiveN::ValuesLesser =>
			Some (ComparisonPrimitive3::ValuesLesser),
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive3::ValuesLesserOrEqual),
		ComparisonPrimitiveN::ValuesEqual =>
			Some (ComparisonPrimitive3::ValuesEqual),
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive3::ValuesGreaterOrEqual),
		ComparisonPrimitiveN::ValuesGreater =>
			Some (ComparisonPrimitive3::ValuesGreater),
	}
}


pub fn comparison_primitive_n_alternative_4 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive4>) {
	match primitive {
		ComparisonPrimitiveN::EquivalentByIdentity =>
			Some (ComparisonPrimitive4::EquivalentByIdentity),
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			Some (ComparisonPrimitive4::EquivalentByValueStrict),
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive4::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive4::EquivalentByValueCoerced),
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive4::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveN::GenericLesser =>
			Some (ComparisonPrimitive4::GenericLesser),
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			Some (ComparisonPrimitive4::GenericLesserOrEqual),
		ComparisonPrimitiveN::GenericEqual =>
			Some (ComparisonPrimitive4::GenericEqual),
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive4::GenericGreaterOrEqual),
		ComparisonPrimitiveN::GenericGreater =>
			Some (ComparisonPrimitive4::GenericGreater),
		ComparisonPrimitiveN::BooleanLesser =>
			Some (ComparisonPrimitive4::BooleanLesser),
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive4::BooleanLesserOrEqual),
		ComparisonPrimitiveN::BooleanEqual =>
			Some (ComparisonPrimitive4::BooleanEqual),
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive4::BooleanGreaterOrEqual),
		ComparisonPrimitiveN::BooleanGreater =>
			Some (ComparisonPrimitive4::BooleanGreater),
		ComparisonPrimitiveN::NumberLesser =>
			Some (ComparisonPrimitive4::NumberLesser),
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			Some (ComparisonPrimitive4::NumberLesserOrEqual),
		ComparisonPrimitiveN::NumberEqual =>
			Some (ComparisonPrimitive4::NumberEqual),
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive4::NumberGreaterOrEqual),
		ComparisonPrimitiveN::NumberGreater =>
			Some (ComparisonPrimitive4::NumberGreater),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::StringCaseSensitiveLesser),
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::StringCaseSensitiveGreater),
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveLesser),
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveN::BytesLesser =>
			Some (ComparisonPrimitive4::BytesLesser),
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			Some (ComparisonPrimitive4::BytesLesserOrEqual),
		ComparisonPrimitiveN::BytesEqual =>
			Some (ComparisonPrimitive4::BytesEqual),
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive4::BytesGreaterOrEqual),
		ComparisonPrimitiveN::BytesGreater =>
			Some (ComparisonPrimitive4::BytesGreater),
		ComparisonPrimitiveN::PairLesser =>
			Some (ComparisonPrimitive4::PairLesser),
		ComparisonPrimitiveN::PairLesserOrEqual =>
			Some (ComparisonPrimitive4::PairLesserOrEqual),
		ComparisonPrimitiveN::PairEqual =>
			Some (ComparisonPrimitive4::PairEqual),
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			Some (ComparisonPrimitive4::PairGreaterOrEqual),
		ComparisonPrimitiveN::PairGreater =>
			Some (ComparisonPrimitive4::PairGreater),
		ComparisonPrimitiveN::ArrayLesser =>
			Some (ComparisonPrimitive4::ArrayLesser),
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive4::ArrayLesserOrEqual),
		ComparisonPrimitiveN::ArrayEqual =>
			Some (ComparisonPrimitive4::ArrayEqual),
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive4::ArrayGreaterOrEqual),
		ComparisonPrimitiveN::ArrayGreater =>
			Some (ComparisonPrimitive4::ArrayGreater),
		ComparisonPrimitiveN::ValuesLesser =>
			Some (ComparisonPrimitive4::ValuesLesser),
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive4::ValuesLesserOrEqual),
		ComparisonPrimitiveN::ValuesEqual =>
			Some (ComparisonPrimitive4::ValuesEqual),
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive4::ValuesGreaterOrEqual),
		ComparisonPrimitiveN::ValuesGreater =>
			Some (ComparisonPrimitive4::ValuesGreater),
	}
}


pub fn comparison_primitive_n_alternative_5 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive5>) {
	match primitive {
		_ =>
			None,
	}
}

