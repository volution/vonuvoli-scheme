

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ComparisonPrimitive0;
	pub use super::ComparisonPrimitive1;
	pub use super::ComparisonPrimitive2;
	pub use super::ComparisonPrimitive3;
	pub use super::ComparisonPrimitive4;
	pub use super::ComparisonPrimitive5;
	pub use super::ComparisonPrimitiveN;
	pub use super::ComparisonPrimitiveV;
	
	pub use super::comparison_primitive_0_evaluate;
	pub use super::comparison_primitive_1_evaluate;
	pub use super::comparison_primitive_2_evaluate;
	pub use super::comparison_primitive_3_evaluate;
	pub use super::comparison_primitive_4_evaluate;
	pub use super::comparison_primitive_5_evaluate;
	pub use super::comparison_primitive_n_evaluate;
	
	pub use super::comparison_primitive_v_alternative_0;
	pub use super::comparison_primitive_v_alternative_1;
	pub use super::comparison_primitive_v_alternative_2;
	pub use super::comparison_primitive_v_alternative_3;
	pub use super::comparison_primitive_v_alternative_4;
	pub use super::comparison_primitive_v_alternative_5;
	pub use super::comparison_primitive_v_alternative_n;
	
	pub use super::comparison_primitive_0_attributes;
	pub use super::comparison_primitive_1_attributes;
	pub use super::comparison_primitive_2_attributes;
	pub use super::comparison_primitive_3_attributes;
	pub use super::comparison_primitive_4_attributes;
	pub use super::comparison_primitive_5_attributes;
	pub use super::comparison_primitive_n_attributes;
	
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


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitiveV {
	
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_0_evaluate (primitive : ComparisonPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_0 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive0>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			None,
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			None,
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			None,
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			None,
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			None,
		ComparisonPrimitiveV::GenericLesser =>
			None,
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			None,
		ComparisonPrimitiveV::GenericEqual =>
			None,
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::GenericGreater =>
			None,
		ComparisonPrimitiveV::BooleanLesser =>
			None,
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			None,
		ComparisonPrimitiveV::BooleanEqual =>
			None,
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::BooleanGreater =>
			None,
		ComparisonPrimitiveV::NumberLesser =>
			None,
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			None,
		ComparisonPrimitiveV::NumberEqual =>
			None,
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::NumberGreater =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::BytesLesser =>
			None,
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			None,
		ComparisonPrimitiveV::BytesEqual =>
			None,
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::BytesGreater =>
			None,
		ComparisonPrimitiveV::PairLesser =>
			None,
		ComparisonPrimitiveV::PairLesserOrEqual =>
			None,
		ComparisonPrimitiveV::PairEqual =>
			None,
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::PairGreater =>
			None,
		ComparisonPrimitiveV::ArrayLesser =>
			None,
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			None,
		ComparisonPrimitiveV::ArrayEqual =>
			None,
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::ArrayGreater =>
			None,
		ComparisonPrimitiveV::ValuesLesser =>
			None,
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			None,
		ComparisonPrimitiveV::ValuesEqual =>
			None,
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::ValuesGreater =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_1 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive1>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			Some (ComparisonPrimitive1::EquivalentByIdentity),
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			Some (ComparisonPrimitive1::EquivalentByValueStrict),
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive1::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive1::EquivalentByValueCoerced),
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive1::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveV::GenericLesser =>
			Some (ComparisonPrimitive1::GenericLesser),
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			Some (ComparisonPrimitive1::GenericLesserOrEqual),
		ComparisonPrimitiveV::GenericEqual =>
			Some (ComparisonPrimitive1::GenericEqual),
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive1::GenericGreaterOrEqual),
		ComparisonPrimitiveV::GenericGreater =>
			Some (ComparisonPrimitive1::GenericGreater),
		ComparisonPrimitiveV::BooleanLesser =>
			Some (ComparisonPrimitive1::BooleanLesser),
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive1::BooleanLesserOrEqual),
		ComparisonPrimitiveV::BooleanEqual =>
			Some (ComparisonPrimitive1::BooleanEqual),
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive1::BooleanGreaterOrEqual),
		ComparisonPrimitiveV::BooleanGreater =>
			Some (ComparisonPrimitive1::BooleanGreater),
		ComparisonPrimitiveV::NumberLesser =>
			Some (ComparisonPrimitive1::NumberLesser),
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			Some (ComparisonPrimitive1::NumberLesserOrEqual),
		ComparisonPrimitiveV::NumberEqual =>
			Some (ComparisonPrimitive1::NumberEqual),
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive1::NumberGreaterOrEqual),
		ComparisonPrimitiveV::NumberGreater =>
			Some (ComparisonPrimitive1::NumberGreater),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::StringCaseSensitiveLesser),
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::StringCaseSensitiveGreater),
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveLesser),
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::StringCaseInsensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive1::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive1::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveV::BytesLesser =>
			Some (ComparisonPrimitive1::BytesLesser),
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			Some (ComparisonPrimitive1::BytesLesserOrEqual),
		ComparisonPrimitiveV::BytesEqual =>
			Some (ComparisonPrimitive1::BytesEqual),
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive1::BytesGreaterOrEqual),
		ComparisonPrimitiveV::BytesGreater =>
			Some (ComparisonPrimitive1::BytesGreater),
		ComparisonPrimitiveV::PairLesser =>
			Some (ComparisonPrimitive1::PairLesser),
		ComparisonPrimitiveV::PairLesserOrEqual =>
			Some (ComparisonPrimitive1::PairLesserOrEqual),
		ComparisonPrimitiveV::PairEqual =>
			Some (ComparisonPrimitive1::PairEqual),
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			Some (ComparisonPrimitive1::PairGreaterOrEqual),
		ComparisonPrimitiveV::PairGreater =>
			Some (ComparisonPrimitive1::PairGreater),
		ComparisonPrimitiveV::ArrayLesser =>
			Some (ComparisonPrimitive1::ArrayLesser),
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive1::ArrayLesserOrEqual),
		ComparisonPrimitiveV::ArrayEqual =>
			Some (ComparisonPrimitive1::ArrayEqual),
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive1::ArrayGreaterOrEqual),
		ComparisonPrimitiveV::ArrayGreater =>
			Some (ComparisonPrimitive1::ArrayGreater),
		ComparisonPrimitiveV::ValuesLesser =>
			Some (ComparisonPrimitive1::ValuesLesser),
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive1::ValuesLesserOrEqual),
		ComparisonPrimitiveV::ValuesEqual =>
			Some (ComparisonPrimitive1::ValuesEqual),
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive1::ValuesGreaterOrEqual),
		ComparisonPrimitiveV::ValuesGreater =>
			Some (ComparisonPrimitive1::ValuesGreater),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_2 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive2>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			Some (ComparisonPrimitive2::EquivalentByIdentity),
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			Some (ComparisonPrimitive2::EquivalentByValueStrict),
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive2::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive2::EquivalentByValueCoerced),
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive2::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveV::GenericLesser =>
			Some (ComparisonPrimitive2::GenericLesser),
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			Some (ComparisonPrimitive2::GenericLesserOrEqual),
		ComparisonPrimitiveV::GenericEqual =>
			Some (ComparisonPrimitive2::GenericEqual),
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive2::GenericGreaterOrEqual),
		ComparisonPrimitiveV::GenericGreater =>
			Some (ComparisonPrimitive2::GenericGreater),
		ComparisonPrimitiveV::BooleanLesser =>
			Some (ComparisonPrimitive2::BooleanLesser),
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive2::BooleanLesserOrEqual),
		ComparisonPrimitiveV::BooleanEqual =>
			Some (ComparisonPrimitive2::BooleanEqual),
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive2::BooleanGreaterOrEqual),
		ComparisonPrimitiveV::BooleanGreater =>
			Some (ComparisonPrimitive2::BooleanGreater),
		ComparisonPrimitiveV::NumberLesser =>
			Some (ComparisonPrimitive2::NumberLesser),
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			Some (ComparisonPrimitive2::NumberLesserOrEqual),
		ComparisonPrimitiveV::NumberEqual =>
			Some (ComparisonPrimitive2::NumberEqual),
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive2::NumberGreaterOrEqual),
		ComparisonPrimitiveV::NumberGreater =>
			Some (ComparisonPrimitive2::NumberGreater),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::StringCaseSensitiveLesser),
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::StringCaseSensitiveGreater),
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveLesser),
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::StringCaseInsensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive2::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive2::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveV::BytesLesser =>
			Some (ComparisonPrimitive2::BytesLesser),
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			Some (ComparisonPrimitive2::BytesLesserOrEqual),
		ComparisonPrimitiveV::BytesEqual =>
			Some (ComparisonPrimitive2::BytesEqual),
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive2::BytesGreaterOrEqual),
		ComparisonPrimitiveV::BytesGreater =>
			Some (ComparisonPrimitive2::BytesGreater),
		ComparisonPrimitiveV::PairLesser =>
			Some (ComparisonPrimitive2::PairLesser),
		ComparisonPrimitiveV::PairLesserOrEqual =>
			Some (ComparisonPrimitive2::PairLesserOrEqual),
		ComparisonPrimitiveV::PairEqual =>
			Some (ComparisonPrimitive2::PairEqual),
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			Some (ComparisonPrimitive2::PairGreaterOrEqual),
		ComparisonPrimitiveV::PairGreater =>
			Some (ComparisonPrimitive2::PairGreater),
		ComparisonPrimitiveV::ArrayLesser =>
			Some (ComparisonPrimitive2::ArrayLesser),
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive2::ArrayLesserOrEqual),
		ComparisonPrimitiveV::ArrayEqual =>
			Some (ComparisonPrimitive2::ArrayEqual),
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive2::ArrayGreaterOrEqual),
		ComparisonPrimitiveV::ArrayGreater =>
			Some (ComparisonPrimitive2::ArrayGreater),
		ComparisonPrimitiveV::ValuesLesser =>
			Some (ComparisonPrimitive2::ValuesLesser),
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive2::ValuesLesserOrEqual),
		ComparisonPrimitiveV::ValuesEqual =>
			Some (ComparisonPrimitive2::ValuesEqual),
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive2::ValuesGreaterOrEqual),
		ComparisonPrimitiveV::ValuesGreater =>
			Some (ComparisonPrimitive2::ValuesGreater),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_3 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive3>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			Some (ComparisonPrimitive3::EquivalentByIdentity),
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			Some (ComparisonPrimitive3::EquivalentByValueStrict),
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive3::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive3::EquivalentByValueCoerced),
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive3::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveV::GenericLesser =>
			Some (ComparisonPrimitive3::GenericLesser),
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			Some (ComparisonPrimitive3::GenericLesserOrEqual),
		ComparisonPrimitiveV::GenericEqual =>
			Some (ComparisonPrimitive3::GenericEqual),
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive3::GenericGreaterOrEqual),
		ComparisonPrimitiveV::GenericGreater =>
			Some (ComparisonPrimitive3::GenericGreater),
		ComparisonPrimitiveV::BooleanLesser =>
			Some (ComparisonPrimitive3::BooleanLesser),
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive3::BooleanLesserOrEqual),
		ComparisonPrimitiveV::BooleanEqual =>
			Some (ComparisonPrimitive3::BooleanEqual),
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive3::BooleanGreaterOrEqual),
		ComparisonPrimitiveV::BooleanGreater =>
			Some (ComparisonPrimitive3::BooleanGreater),
		ComparisonPrimitiveV::NumberLesser =>
			Some (ComparisonPrimitive3::NumberLesser),
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			Some (ComparisonPrimitive3::NumberLesserOrEqual),
		ComparisonPrimitiveV::NumberEqual =>
			Some (ComparisonPrimitive3::NumberEqual),
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive3::NumberGreaterOrEqual),
		ComparisonPrimitiveV::NumberGreater =>
			Some (ComparisonPrimitive3::NumberGreater),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::StringCaseSensitiveLesser),
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::StringCaseSensitiveGreater),
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveLesser),
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::StringCaseInsensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive3::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive3::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveV::BytesLesser =>
			Some (ComparisonPrimitive3::BytesLesser),
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			Some (ComparisonPrimitive3::BytesLesserOrEqual),
		ComparisonPrimitiveV::BytesEqual =>
			Some (ComparisonPrimitive3::BytesEqual),
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive3::BytesGreaterOrEqual),
		ComparisonPrimitiveV::BytesGreater =>
			Some (ComparisonPrimitive3::BytesGreater),
		ComparisonPrimitiveV::PairLesser =>
			Some (ComparisonPrimitive3::PairLesser),
		ComparisonPrimitiveV::PairLesserOrEqual =>
			Some (ComparisonPrimitive3::PairLesserOrEqual),
		ComparisonPrimitiveV::PairEqual =>
			Some (ComparisonPrimitive3::PairEqual),
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			Some (ComparisonPrimitive3::PairGreaterOrEqual),
		ComparisonPrimitiveV::PairGreater =>
			Some (ComparisonPrimitive3::PairGreater),
		ComparisonPrimitiveV::ArrayLesser =>
			Some (ComparisonPrimitive3::ArrayLesser),
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive3::ArrayLesserOrEqual),
		ComparisonPrimitiveV::ArrayEqual =>
			Some (ComparisonPrimitive3::ArrayEqual),
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive3::ArrayGreaterOrEqual),
		ComparisonPrimitiveV::ArrayGreater =>
			Some (ComparisonPrimitive3::ArrayGreater),
		ComparisonPrimitiveV::ValuesLesser =>
			Some (ComparisonPrimitive3::ValuesLesser),
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive3::ValuesLesserOrEqual),
		ComparisonPrimitiveV::ValuesEqual =>
			Some (ComparisonPrimitive3::ValuesEqual),
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive3::ValuesGreaterOrEqual),
		ComparisonPrimitiveV::ValuesGreater =>
			Some (ComparisonPrimitive3::ValuesGreater),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_4 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive4>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			Some (ComparisonPrimitive4::EquivalentByIdentity),
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			Some (ComparisonPrimitive4::EquivalentByValueStrict),
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitive4::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			Some (ComparisonPrimitive4::EquivalentByValueCoerced),
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitive4::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveV::GenericLesser =>
			Some (ComparisonPrimitive4::GenericLesser),
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			Some (ComparisonPrimitive4::GenericLesserOrEqual),
		ComparisonPrimitiveV::GenericEqual =>
			Some (ComparisonPrimitive4::GenericEqual),
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			Some (ComparisonPrimitive4::GenericGreaterOrEqual),
		ComparisonPrimitiveV::GenericGreater =>
			Some (ComparisonPrimitive4::GenericGreater),
		ComparisonPrimitiveV::BooleanLesser =>
			Some (ComparisonPrimitive4::BooleanLesser),
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			Some (ComparisonPrimitive4::BooleanLesserOrEqual),
		ComparisonPrimitiveV::BooleanEqual =>
			Some (ComparisonPrimitive4::BooleanEqual),
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitive4::BooleanGreaterOrEqual),
		ComparisonPrimitiveV::BooleanGreater =>
			Some (ComparisonPrimitive4::BooleanGreater),
		ComparisonPrimitiveV::NumberLesser =>
			Some (ComparisonPrimitive4::NumberLesser),
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			Some (ComparisonPrimitive4::NumberLesserOrEqual),
		ComparisonPrimitiveV::NumberEqual =>
			Some (ComparisonPrimitive4::NumberEqual),
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			Some (ComparisonPrimitive4::NumberGreaterOrEqual),
		ComparisonPrimitiveV::NumberGreater =>
			Some (ComparisonPrimitive4::NumberGreater),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::StringCaseSensitiveLesser),
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::StringCaseSensitiveGreater),
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveLesser),
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::StringCaseInsensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitive4::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitive4::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveV::BytesLesser =>
			Some (ComparisonPrimitive4::BytesLesser),
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			Some (ComparisonPrimitive4::BytesLesserOrEqual),
		ComparisonPrimitiveV::BytesEqual =>
			Some (ComparisonPrimitive4::BytesEqual),
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			Some (ComparisonPrimitive4::BytesGreaterOrEqual),
		ComparisonPrimitiveV::BytesGreater =>
			Some (ComparisonPrimitive4::BytesGreater),
		ComparisonPrimitiveV::PairLesser =>
			Some (ComparisonPrimitive4::PairLesser),
		ComparisonPrimitiveV::PairLesserOrEqual =>
			Some (ComparisonPrimitive4::PairLesserOrEqual),
		ComparisonPrimitiveV::PairEqual =>
			Some (ComparisonPrimitive4::PairEqual),
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			Some (ComparisonPrimitive4::PairGreaterOrEqual),
		ComparisonPrimitiveV::PairGreater =>
			Some (ComparisonPrimitive4::PairGreater),
		ComparisonPrimitiveV::ArrayLesser =>
			Some (ComparisonPrimitive4::ArrayLesser),
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			Some (ComparisonPrimitive4::ArrayLesserOrEqual),
		ComparisonPrimitiveV::ArrayEqual =>
			Some (ComparisonPrimitive4::ArrayEqual),
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitive4::ArrayGreaterOrEqual),
		ComparisonPrimitiveV::ArrayGreater =>
			Some (ComparisonPrimitive4::ArrayGreater),
		ComparisonPrimitiveV::ValuesLesser =>
			Some (ComparisonPrimitive4::ValuesLesser),
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			Some (ComparisonPrimitive4::ValuesLesserOrEqual),
		ComparisonPrimitiveV::ValuesEqual =>
			Some (ComparisonPrimitive4::ValuesEqual),
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitive4::ValuesGreaterOrEqual),
		ComparisonPrimitiveV::ValuesGreater =>
			Some (ComparisonPrimitive4::ValuesGreater),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_5 (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive5>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			None,
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			None,
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			None,
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			None,
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			None,
		ComparisonPrimitiveV::GenericLesser =>
			None,
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			None,
		ComparisonPrimitiveV::GenericEqual =>
			None,
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::GenericGreater =>
			None,
		ComparisonPrimitiveV::BooleanLesser =>
			None,
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			None,
		ComparisonPrimitiveV::BooleanEqual =>
			None,
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::BooleanGreater =>
			None,
		ComparisonPrimitiveV::NumberLesser =>
			None,
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			None,
		ComparisonPrimitiveV::NumberEqual =>
			None,
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::NumberGreater =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			None,
		ComparisonPrimitiveV::BytesLesser =>
			None,
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			None,
		ComparisonPrimitiveV::BytesEqual =>
			None,
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::BytesGreater =>
			None,
		ComparisonPrimitiveV::PairLesser =>
			None,
		ComparisonPrimitiveV::PairLesserOrEqual =>
			None,
		ComparisonPrimitiveV::PairEqual =>
			None,
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::PairGreater =>
			None,
		ComparisonPrimitiveV::ArrayLesser =>
			None,
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			None,
		ComparisonPrimitiveV::ArrayEqual =>
			None,
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::ArrayGreater =>
			None,
		ComparisonPrimitiveV::ValuesLesser =>
			None,
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			None,
		ComparisonPrimitiveV::ValuesEqual =>
			None,
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			None,
		ComparisonPrimitiveV::ValuesGreater =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_v_alternative_n (primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitiveN>) {
	match primitive {
		ComparisonPrimitiveV::EquivalentByIdentity =>
			Some (ComparisonPrimitiveN::EquivalentByIdentity),
		ComparisonPrimitiveV::EquivalentByValueStrict =>
			Some (ComparisonPrimitiveN::EquivalentByValueStrict),
		ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
			Some (ComparisonPrimitiveN::EquivalentByValueStrictRecursive),
		ComparisonPrimitiveV::EquivalentByValueCoerced =>
			Some (ComparisonPrimitiveN::EquivalentByValueCoerced),
		ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
			Some (ComparisonPrimitiveN::EquivalentByValueCoercedRecursive),
		ComparisonPrimitiveV::GenericLesser =>
			Some (ComparisonPrimitiveN::GenericLesser),
		ComparisonPrimitiveV::GenericLesserOrEqual =>
			Some (ComparisonPrimitiveN::GenericLesserOrEqual),
		ComparisonPrimitiveV::GenericEqual =>
			Some (ComparisonPrimitiveN::GenericEqual),
		ComparisonPrimitiveV::GenericGreaterOrEqual =>
			Some (ComparisonPrimitiveN::GenericGreaterOrEqual),
		ComparisonPrimitiveV::GenericGreater =>
			Some (ComparisonPrimitiveN::GenericGreater),
		ComparisonPrimitiveV::BooleanLesser =>
			Some (ComparisonPrimitiveN::BooleanLesser),
		ComparisonPrimitiveV::BooleanLesserOrEqual =>
			Some (ComparisonPrimitiveN::BooleanLesserOrEqual),
		ComparisonPrimitiveV::BooleanEqual =>
			Some (ComparisonPrimitiveN::BooleanEqual),
		ComparisonPrimitiveV::BooleanGreaterOrEqual =>
			Some (ComparisonPrimitiveN::BooleanGreaterOrEqual),
		ComparisonPrimitiveV::BooleanGreater =>
			Some (ComparisonPrimitiveN::BooleanGreater),
		ComparisonPrimitiveV::NumberLesser =>
			Some (ComparisonPrimitiveN::NumberLesser),
		ComparisonPrimitiveV::NumberLesserOrEqual =>
			Some (ComparisonPrimitiveN::NumberLesserOrEqual),
		ComparisonPrimitiveV::NumberEqual =>
			Some (ComparisonPrimitiveN::NumberEqual),
		ComparisonPrimitiveV::NumberGreaterOrEqual =>
			Some (ComparisonPrimitiveN::NumberGreaterOrEqual),
		ComparisonPrimitiveV::NumberGreater =>
			Some (ComparisonPrimitiveN::NumberGreater),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
			Some (ComparisonPrimitiveN::CharacterCaseSensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseSensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
			Some (ComparisonPrimitiveN::CharacterCaseSensitiveGreater),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
			Some (ComparisonPrimitiveN::CharacterCaseInsensitiveLesser),
		ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseInsensitiveEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
			Some (ComparisonPrimitiveN::CharacterCaseInsensitiveGreater),
		ComparisonPrimitiveV::StringCaseSensitiveLesser =>
			Some (ComparisonPrimitiveN::StringCaseSensitiveLesser),
		ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveEqual =>
			Some (ComparisonPrimitiveN::StringCaseSensitiveEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseSensitiveGreater =>
			Some (ComparisonPrimitiveN::StringCaseSensitiveGreater),
		ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
			Some (ComparisonPrimitiveN::StringCaseInsensitiveLesser),
		ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
			Some (ComparisonPrimitiveN::StringCaseInsensitiveEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
			Some (ComparisonPrimitiveN::StringCaseInsensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
			Some (ComparisonPrimitiveN::SymbolCaseSensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseSensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
			Some (ComparisonPrimitiveN::SymbolCaseSensitiveGreater),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
			Some (ComparisonPrimitiveN::SymbolCaseInsensitiveLesser),
		ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseInsensitiveEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
			Some (ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual),
		ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
			Some (ComparisonPrimitiveN::SymbolCaseInsensitiveGreater),
		ComparisonPrimitiveV::BytesLesser =>
			Some (ComparisonPrimitiveN::BytesLesser),
		ComparisonPrimitiveV::BytesLesserOrEqual =>
			Some (ComparisonPrimitiveN::BytesLesserOrEqual),
		ComparisonPrimitiveV::BytesEqual =>
			Some (ComparisonPrimitiveN::BytesEqual),
		ComparisonPrimitiveV::BytesGreaterOrEqual =>
			Some (ComparisonPrimitiveN::BytesGreaterOrEqual),
		ComparisonPrimitiveV::BytesGreater =>
			Some (ComparisonPrimitiveN::BytesGreater),
		ComparisonPrimitiveV::PairLesser =>
			Some (ComparisonPrimitiveN::PairLesser),
		ComparisonPrimitiveV::PairLesserOrEqual =>
			Some (ComparisonPrimitiveN::PairLesserOrEqual),
		ComparisonPrimitiveV::PairEqual =>
			Some (ComparisonPrimitiveN::PairEqual),
		ComparisonPrimitiveV::PairGreaterOrEqual =>
			Some (ComparisonPrimitiveN::PairGreaterOrEqual),
		ComparisonPrimitiveV::PairGreater =>
			Some (ComparisonPrimitiveN::PairGreater),
		ComparisonPrimitiveV::ArrayLesser =>
			Some (ComparisonPrimitiveN::ArrayLesser),
		ComparisonPrimitiveV::ArrayLesserOrEqual =>
			Some (ComparisonPrimitiveN::ArrayLesserOrEqual),
		ComparisonPrimitiveV::ArrayEqual =>
			Some (ComparisonPrimitiveN::ArrayEqual),
		ComparisonPrimitiveV::ArrayGreaterOrEqual =>
			Some (ComparisonPrimitiveN::ArrayGreaterOrEqual),
		ComparisonPrimitiveV::ArrayGreater =>
			Some (ComparisonPrimitiveN::ArrayGreater),
		ComparisonPrimitiveV::ValuesLesser =>
			Some (ComparisonPrimitiveN::ValuesLesser),
		ComparisonPrimitiveV::ValuesLesserOrEqual =>
			Some (ComparisonPrimitiveN::ValuesLesserOrEqual),
		ComparisonPrimitiveV::ValuesEqual =>
			Some (ComparisonPrimitiveN::ValuesEqual),
		ComparisonPrimitiveV::ValuesGreaterOrEqual =>
			Some (ComparisonPrimitiveN::ValuesGreaterOrEqual),
		ComparisonPrimitiveV::ValuesGreater =>
			Some (ComparisonPrimitiveN::ValuesGreater),
	}
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_0_attributes (_primitive : ComparisonPrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_1_attributes (_primitive : ComparisonPrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_2_attributes (_primitive : ComparisonPrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_3_attributes (_primitive : ComparisonPrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_3);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_4_attributes (_primitive : ComparisonPrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_5_attributes (_primitive : ComparisonPrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn comparison_primitive_n_attributes (_primitive : ComparisonPrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

