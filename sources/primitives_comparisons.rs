

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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_0_evaluate (primitive : ComparisonPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive1::EquivalentByIdentity =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByIdentity, None, None)) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueStrict =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueStrictRecursive =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueCoerced =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false))) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueCoercedRecursive =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true))) .into_0 (),
		
		ComparisonPrimitive1::GenericLesser =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::GenericLesserOrEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::GenericEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::GenericGreaterOrEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::GenericGreater =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BooleanLesser =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BooleanLesserOrEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BooleanEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BooleanGreaterOrEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BooleanGreater =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::NumberLesser =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::NumberLesserOrEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::NumberEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::NumberGreaterOrEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::NumberGreater =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseSensitiveLesser =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseSensitiveEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseSensitiveGreater =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveLesser =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::CharacterCaseInsensitiveGreater =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseSensitiveLesser =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseSensitiveEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseSensitiveGreater =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseInsensitiveLesser =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseInsensitiveEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::StringCaseInsensitiveGreater =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesser =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreater =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesser =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreater =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::BytesLesser =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BytesLesserOrEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BytesEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BytesGreaterOrEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::BytesGreater =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::PairLesser =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::PairLesserOrEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::PairEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::PairGreaterOrEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::PairGreater =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ArrayLesser =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ArrayLesserOrEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ArrayEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ArrayGreaterOrEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ArrayGreater =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ValuesLesser =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ValuesLesserOrEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ValuesEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ValuesGreaterOrEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::ValuesGreater =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive2::EquivalentByIdentity =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByIdentity, None, None)) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueStrict =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueStrictRecursive =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueCoerced =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false))) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueCoercedRecursive =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true))) .into_0 (),
		
		ComparisonPrimitive2::GenericLesser =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::GenericLesserOrEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::GenericEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::GenericGreaterOrEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::GenericGreater =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BooleanLesser =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BooleanLesserOrEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BooleanEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BooleanGreaterOrEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BooleanGreater =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::NumberLesser =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::NumberLesserOrEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::NumberEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::NumberGreaterOrEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::NumberGreater =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseSensitiveLesser =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseSensitiveEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreater =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesser =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreater =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseSensitiveLesser =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseSensitiveEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseSensitiveGreater =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesser =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseInsensitiveEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreater =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesser =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreater =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesser =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreater =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::BytesLesser =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BytesLesserOrEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BytesEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BytesGreaterOrEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::BytesGreater =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::PairLesser =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::PairLesserOrEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::PairEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::PairGreaterOrEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::PairGreater =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ArrayLesser =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ArrayLesserOrEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ArrayEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ArrayGreaterOrEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ArrayGreater =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ValuesLesser =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ValuesLesserOrEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ValuesEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ValuesGreaterOrEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::ValuesGreater =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive3::EquivalentByIdentity =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByIdentity, None, None)) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueStrict =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueStrictRecursive =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueCoerced =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false))) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueCoercedRecursive =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true))) .into_0 (),
		
		ComparisonPrimitive3::GenericLesser =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::GenericLesserOrEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::GenericEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::GenericGreaterOrEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::GenericGreater =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BooleanLesser =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BooleanLesserOrEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BooleanEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BooleanGreaterOrEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BooleanGreater =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::NumberLesser =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::NumberLesserOrEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::NumberEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::NumberGreaterOrEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::NumberGreater =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseSensitiveLesser =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseSensitiveEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseSensitiveGreater =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveLesser =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::CharacterCaseInsensitiveGreater =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseSensitiveLesser =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseSensitiveLesserOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseSensitiveEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseSensitiveGreater =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseInsensitiveLesser =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseInsensitiveEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::StringCaseInsensitiveGreater =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesser =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreater =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesser =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreater =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::BytesLesser =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BytesLesserOrEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BytesEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BytesGreaterOrEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::BytesGreater =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::PairLesser =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::PairLesserOrEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::PairEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::PairGreaterOrEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::PairGreater =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ArrayLesser =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ArrayLesserOrEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ArrayEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ArrayGreaterOrEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ArrayGreater =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ValuesLesser =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ValuesLesserOrEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ValuesEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ValuesGreaterOrEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::ValuesGreater =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive4::EquivalentByIdentity =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByIdentity, None, None)) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueStrict =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueStrictRecursive =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueCoerced =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false))) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueCoercedRecursive =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true))) .into_0 (),
		
		ComparisonPrimitive4::GenericLesser =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::GenericLesserOrEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::GenericEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::GenericGreaterOrEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::GenericGreater =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BooleanLesser =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BooleanLesserOrEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BooleanEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BooleanGreaterOrEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BooleanGreater =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::NumberLesser =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::NumberLesserOrEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::NumberEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::NumberGreaterOrEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::NumberGreater =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseSensitiveLesser =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseSensitiveEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseSensitiveGreater =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveLesser =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::CharacterCaseInsensitiveGreater =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseSensitiveLesser =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseSensitiveLesserOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseSensitiveEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseSensitiveGreater =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseInsensitiveLesser =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseInsensitiveEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::StringCaseInsensitiveGreater =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesser =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreater =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesser =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreater =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::BytesLesser =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BytesLesserOrEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BytesEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BytesGreaterOrEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::BytesGreater =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::PairLesser =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::PairLesserOrEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::PairEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::PairGreaterOrEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::PairGreater =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ArrayLesser =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ArrayLesserOrEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ArrayEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ArrayGreaterOrEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ArrayGreater =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ValuesLesser =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ValuesLesserOrEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ValuesEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ValuesGreaterOrEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::ValuesGreater =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitiveN::EquivalentByIdentity =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByIdentity, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::GenericLesser =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::GenericEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::GenericGreater =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanLesser =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanGreater =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::NumberLesser =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::NumberEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::NumberGreater =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::BytesLesser =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BytesEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::BytesGreater =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::PairLesser =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::PairLesserOrEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::PairEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::PairGreater =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ArrayLesser =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ArrayEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ArrayGreater =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ValuesLesser =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ValuesEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::ValuesGreater =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_0_attributes (_primitive : ComparisonPrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_1_attributes (_primitive : ComparisonPrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_2_attributes (_primitive : ComparisonPrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_3_attributes (_primitive : ComparisonPrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_3);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_4_attributes (_primitive : ComparisonPrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_5_attributes (_primitive : ComparisonPrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_n_attributes (_primitive : ComparisonPrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

