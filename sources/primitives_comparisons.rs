

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




macro_rules! def_comparison_primitive_enum {
	( $identifier : ident ) => (
		
		#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
		pub enum $identifier {
			
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
			
			KeywordCaseSensitiveLesser,
			KeywordCaseSensitiveLesserOrEqual,
			KeywordCaseSensitiveEqual,
			KeywordCaseSensitiveGreaterOrEqual,
			KeywordCaseSensitiveGreater,
			
			KeywordCaseInsensitiveLesser,
			KeywordCaseInsensitiveLesserOrEqual,
			KeywordCaseInsensitiveEqual,
			KeywordCaseInsensitiveGreaterOrEqual,
			KeywordCaseInsensitiveGreater,
			
			UniqueLesser,
			UniqueLesserOrEqual,
			UniqueEqual,
			UniqueGreaterOrEqual,
			UniqueGreater,
			
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
			
			RecordLesser,
			RecordLesserOrEqual,
			RecordEqual,
			RecordGreaterOrEqual,
			RecordGreater,
			
		}
		
	);
}




def_comparison_primitive_enum! (ComparisonPrimitive1);
def_comparison_primitive_enum! (ComparisonPrimitive2);
def_comparison_primitive_enum! (ComparisonPrimitive3);
def_comparison_primitive_enum! (ComparisonPrimitive4);
def_comparison_primitive_enum! (ComparisonPrimitiveN);
def_comparison_primitive_enum! (ComparisonPrimitiveV);


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive0 {}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive5 {}




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
		
		ComparisonPrimitive1::KeywordCaseSensitiveLesser =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseSensitiveEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseSensitiveGreater =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseInsensitiveLesser =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseInsensitiveEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::KeywordCaseInsensitiveGreater =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive1::UniqueLesser =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::UniqueLesserOrEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::UniqueEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::UniqueGreaterOrEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::UniqueGreater =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive1::RecordLesser =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive1::RecordLesserOrEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::RecordEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive1::RecordGreaterOrEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive1::RecordGreater =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive2::KeywordCaseSensitiveLesser =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseSensitiveEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseSensitiveGreater =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseInsensitiveLesser =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseInsensitiveEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::KeywordCaseInsensitiveGreater =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive2::UniqueLesser =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::UniqueLesserOrEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::UniqueEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::UniqueGreaterOrEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::UniqueGreater =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive2::RecordLesser =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive2::RecordLesserOrEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::RecordEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive2::RecordGreaterOrEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive2::RecordGreater =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive3::KeywordCaseSensitiveLesser =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseSensitiveEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseSensitiveGreater =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseInsensitiveLesser =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseInsensitiveEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::KeywordCaseInsensitiveGreater =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive3::UniqueLesser =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::UniqueLesserOrEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::UniqueEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::UniqueGreaterOrEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::UniqueGreater =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive3::RecordLesser =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive3::RecordLesserOrEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::RecordEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive3::RecordGreaterOrEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive3::RecordGreater =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive4::KeywordCaseSensitiveLesser =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseSensitiveEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseSensitiveGreater =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseInsensitiveLesser =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseInsensitiveEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::KeywordCaseInsensitiveGreater =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitive4::UniqueLesser =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::UniqueLesserOrEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::UniqueEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::UniqueGreaterOrEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::UniqueGreater =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitive4::RecordLesser =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitive4::RecordLesserOrEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::RecordEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitive4::RecordGreaterOrEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitive4::RecordGreater =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitiveN::KeywordCaseSensitiveLesser =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseSensitiveEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseSensitiveGreater =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseInsensitiveLesser =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseInsensitiveEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::KeywordCaseInsensitiveGreater =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false))) .into_0 (),
		
		ComparisonPrimitiveN::UniqueLesser =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::UniqueLesserOrEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::UniqueEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::UniqueGreaterOrEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::UniqueGreater =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
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
		
		ComparisonPrimitiveN::RecordLesser =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::RecordLesserOrEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::RecordEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::RecordGreaterOrEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None)) .into_0 (),
		
		ComparisonPrimitiveN::RecordGreater =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None)) .into_0 (),
		
	}
}




macro_rules! def_comparison_primitive_v_alternative_fn {
	( $identifier : ident, $alternative : ident ) => (
		
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $identifier (primitive : ComparisonPrimitiveV) -> (Option<$alternative>) {
			match primitive {
				ComparisonPrimitiveV::EquivalentByIdentity =>
					Some ($alternative::EquivalentByIdentity),
				ComparisonPrimitiveV::EquivalentByValueStrict =>
					Some ($alternative::EquivalentByValueStrict),
				ComparisonPrimitiveV::EquivalentByValueStrictRecursive =>
					Some ($alternative::EquivalentByValueStrictRecursive),
				ComparisonPrimitiveV::EquivalentByValueCoerced =>
					Some ($alternative::EquivalentByValueCoerced),
				ComparisonPrimitiveV::EquivalentByValueCoercedRecursive =>
					Some ($alternative::EquivalentByValueCoercedRecursive),
				ComparisonPrimitiveV::GenericLesser =>
					Some ($alternative::GenericLesser),
				ComparisonPrimitiveV::GenericLesserOrEqual =>
					Some ($alternative::GenericLesserOrEqual),
				ComparisonPrimitiveV::GenericEqual =>
					Some ($alternative::GenericEqual),
				ComparisonPrimitiveV::GenericGreaterOrEqual =>
					Some ($alternative::GenericGreaterOrEqual),
				ComparisonPrimitiveV::GenericGreater =>
					Some ($alternative::GenericGreater),
				ComparisonPrimitiveV::BooleanLesser =>
					Some ($alternative::BooleanLesser),
				ComparisonPrimitiveV::BooleanLesserOrEqual =>
					Some ($alternative::BooleanLesserOrEqual),
				ComparisonPrimitiveV::BooleanEqual =>
					Some ($alternative::BooleanEqual),
				ComparisonPrimitiveV::BooleanGreaterOrEqual =>
					Some ($alternative::BooleanGreaterOrEqual),
				ComparisonPrimitiveV::BooleanGreater =>
					Some ($alternative::BooleanGreater),
				ComparisonPrimitiveV::NumberLesser =>
					Some ($alternative::NumberLesser),
				ComparisonPrimitiveV::NumberLesserOrEqual =>
					Some ($alternative::NumberLesserOrEqual),
				ComparisonPrimitiveV::NumberEqual =>
					Some ($alternative::NumberEqual),
				ComparisonPrimitiveV::NumberGreaterOrEqual =>
					Some ($alternative::NumberGreaterOrEqual),
				ComparisonPrimitiveV::NumberGreater =>
					Some ($alternative::NumberGreater),
				ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
					Some ($alternative::CharacterCaseSensitiveLesser),
				ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
					Some ($alternative::CharacterCaseSensitiveLesserOrEqual),
				ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
					Some ($alternative::CharacterCaseSensitiveEqual),
				ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
					Some ($alternative::CharacterCaseSensitiveGreaterOrEqual),
				ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
					Some ($alternative::CharacterCaseSensitiveGreater),
				ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
					Some ($alternative::CharacterCaseInsensitiveLesser),
				ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
					Some ($alternative::CharacterCaseInsensitiveLesserOrEqual),
				ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
					Some ($alternative::CharacterCaseInsensitiveEqual),
				ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::CharacterCaseInsensitiveGreaterOrEqual),
				ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
					Some ($alternative::CharacterCaseInsensitiveGreater),
				ComparisonPrimitiveV::StringCaseSensitiveLesser =>
					Some ($alternative::StringCaseSensitiveLesser),
				ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
					Some ($alternative::StringCaseSensitiveLesserOrEqual),
				ComparisonPrimitiveV::StringCaseSensitiveEqual =>
					Some ($alternative::StringCaseSensitiveEqual),
				ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
					Some ($alternative::StringCaseSensitiveGreaterOrEqual),
				ComparisonPrimitiveV::StringCaseSensitiveGreater =>
					Some ($alternative::StringCaseSensitiveGreater),
				ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
					Some ($alternative::StringCaseInsensitiveLesser),
				ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
					Some ($alternative::StringCaseInsensitiveLesserOrEqual),
				ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
					Some ($alternative::StringCaseInsensitiveEqual),
				ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::StringCaseInsensitiveGreaterOrEqual),
				ComparisonPrimitiveV::StringCaseInsensitiveGreater =>
					Some ($alternative::StringCaseInsensitiveGreater),
				ComparisonPrimitiveV::SymbolCaseSensitiveLesser =>
					Some ($alternative::SymbolCaseSensitiveLesser),
				ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual =>
					Some ($alternative::SymbolCaseSensitiveLesserOrEqual),
				ComparisonPrimitiveV::SymbolCaseSensitiveEqual =>
					Some ($alternative::SymbolCaseSensitiveEqual),
				ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual =>
					Some ($alternative::SymbolCaseSensitiveGreaterOrEqual),
				ComparisonPrimitiveV::SymbolCaseSensitiveGreater =>
					Some ($alternative::SymbolCaseSensitiveGreater),
				ComparisonPrimitiveV::SymbolCaseInsensitiveLesser =>
					Some ($alternative::SymbolCaseInsensitiveLesser),
				ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual =>
					Some ($alternative::SymbolCaseInsensitiveLesserOrEqual),
				ComparisonPrimitiveV::SymbolCaseInsensitiveEqual =>
					Some ($alternative::SymbolCaseInsensitiveEqual),
				ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::SymbolCaseInsensitiveGreaterOrEqual),
				ComparisonPrimitiveV::SymbolCaseInsensitiveGreater =>
					Some ($alternative::SymbolCaseInsensitiveGreater),
				ComparisonPrimitiveV::KeywordCaseSensitiveLesser =>
					Some ($alternative::KeywordCaseSensitiveLesser),
				ComparisonPrimitiveV::KeywordCaseSensitiveLesserOrEqual =>
					Some ($alternative::KeywordCaseSensitiveLesserOrEqual),
				ComparisonPrimitiveV::KeywordCaseSensitiveEqual =>
					Some ($alternative::KeywordCaseSensitiveEqual),
				ComparisonPrimitiveV::KeywordCaseSensitiveGreaterOrEqual =>
					Some ($alternative::KeywordCaseSensitiveGreaterOrEqual),
				ComparisonPrimitiveV::KeywordCaseSensitiveGreater =>
					Some ($alternative::KeywordCaseSensitiveGreater),
				ComparisonPrimitiveV::KeywordCaseInsensitiveLesser =>
					Some ($alternative::KeywordCaseInsensitiveLesser),
				ComparisonPrimitiveV::KeywordCaseInsensitiveLesserOrEqual =>
					Some ($alternative::KeywordCaseInsensitiveLesserOrEqual),
				ComparisonPrimitiveV::KeywordCaseInsensitiveEqual =>
					Some ($alternative::KeywordCaseInsensitiveEqual),
				ComparisonPrimitiveV::KeywordCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::KeywordCaseInsensitiveGreaterOrEqual),
				ComparisonPrimitiveV::KeywordCaseInsensitiveGreater =>
					Some ($alternative::KeywordCaseInsensitiveGreater),
				ComparisonPrimitiveV::UniqueLesser =>
					Some ($alternative::UniqueLesser),
				ComparisonPrimitiveV::UniqueLesserOrEqual =>
					Some ($alternative::UniqueLesserOrEqual),
				ComparisonPrimitiveV::UniqueEqual =>
					Some ($alternative::UniqueEqual),
				ComparisonPrimitiveV::UniqueGreaterOrEqual =>
					Some ($alternative::UniqueGreaterOrEqual),
				ComparisonPrimitiveV::UniqueGreater =>
					Some ($alternative::UniqueGreater),
				ComparisonPrimitiveV::BytesLesser =>
					Some ($alternative::BytesLesser),
				ComparisonPrimitiveV::BytesLesserOrEqual =>
					Some ($alternative::BytesLesserOrEqual),
				ComparisonPrimitiveV::BytesEqual =>
					Some ($alternative::BytesEqual),
				ComparisonPrimitiveV::BytesGreaterOrEqual =>
					Some ($alternative::BytesGreaterOrEqual),
				ComparisonPrimitiveV::BytesGreater =>
					Some ($alternative::BytesGreater),
				ComparisonPrimitiveV::PairLesser =>
					Some ($alternative::PairLesser),
				ComparisonPrimitiveV::PairLesserOrEqual =>
					Some ($alternative::PairLesserOrEqual),
				ComparisonPrimitiveV::PairEqual =>
					Some ($alternative::PairEqual),
				ComparisonPrimitiveV::PairGreaterOrEqual =>
					Some ($alternative::PairGreaterOrEqual),
				ComparisonPrimitiveV::PairGreater =>
					Some ($alternative::PairGreater),
				ComparisonPrimitiveV::ArrayLesser =>
					Some ($alternative::ArrayLesser),
				ComparisonPrimitiveV::ArrayLesserOrEqual =>
					Some ($alternative::ArrayLesserOrEqual),
				ComparisonPrimitiveV::ArrayEqual =>
					Some ($alternative::ArrayEqual),
				ComparisonPrimitiveV::ArrayGreaterOrEqual =>
					Some ($alternative::ArrayGreaterOrEqual),
				ComparisonPrimitiveV::ArrayGreater =>
					Some ($alternative::ArrayGreater),
				ComparisonPrimitiveV::ValuesLesser =>
					Some ($alternative::ValuesLesser),
				ComparisonPrimitiveV::ValuesLesserOrEqual =>
					Some ($alternative::ValuesLesserOrEqual),
				ComparisonPrimitiveV::ValuesEqual =>
					Some ($alternative::ValuesEqual),
				ComparisonPrimitiveV::ValuesGreaterOrEqual =>
					Some ($alternative::ValuesGreaterOrEqual),
				ComparisonPrimitiveV::ValuesGreater =>
					Some ($alternative::ValuesGreater),
				ComparisonPrimitiveV::RecordLesser =>
					Some ($alternative::RecordLesser),
				ComparisonPrimitiveV::RecordLesserOrEqual =>
					Some ($alternative::RecordLesserOrEqual),
				ComparisonPrimitiveV::RecordEqual =>
					Some ($alternative::RecordEqual),
				ComparisonPrimitiveV::RecordGreaterOrEqual =>
					Some ($alternative::RecordGreaterOrEqual),
				ComparisonPrimitiveV::RecordGreater =>
					Some ($alternative::RecordGreater),
			}
		}
		
	);
}


def_comparison_primitive_v_alternative_fn! (comparison_primitive_v_alternative_1, ComparisonPrimitive1);
def_comparison_primitive_v_alternative_fn! (comparison_primitive_v_alternative_2, ComparisonPrimitive2);
def_comparison_primitive_v_alternative_fn! (comparison_primitive_v_alternative_3, ComparisonPrimitive3);
def_comparison_primitive_v_alternative_fn! (comparison_primitive_v_alternative_4, ComparisonPrimitive4);
def_comparison_primitive_v_alternative_fn! (comparison_primitive_v_alternative_n, ComparisonPrimitiveN);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_v_alternative_0 (_primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive0>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_v_alternative_5 (_primitive : ComparisonPrimitiveV) -> (Option<ComparisonPrimitive5>) {
	return None;
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

