

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
		
		#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
		#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseSensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseSensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseSensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseSensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseSensitiveGreater,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseInsensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseInsensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseInsensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseInsensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			CharacterCaseInsensitiveGreater,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseSensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseSensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseSensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseSensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseSensitiveGreater,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseInsensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseInsensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseInsensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			StringCaseInsensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseSensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseSensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseSensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseSensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseSensitiveGreater,
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseInsensitiveLesser,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseInsensitiveLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseInsensitiveEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseInsensitiveGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			KeywordCaseInsensitiveGreater,
			
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			UniqueLesser,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			UniqueLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			UniqueEqual,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			UniqueGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			UniqueGreater,
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			BytesLesser,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			BytesLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			BytesEqual,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			BytesGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			BytesGreater,
			
			PairLesser,
			PairLesserOrEqual,
			PairEqual,
			PairGreaterOrEqual,
			PairGreater,
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ArrayLesser,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ArrayLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ArrayEqual,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ArrayGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ArrayGreater,
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValuesLesser,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValuesLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValuesEqual,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValuesGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValuesGreater,
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			RecordLesser,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			RecordLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			RecordEqual,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			RecordGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			RecordGreater,
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			PathLesser,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			PathLesserOrEqual,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			PathEqual,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			PathGreaterOrEqual,
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			PathGreater,
			
		}
		
	);
}




def_comparison_primitive_enum! (ComparisonPrimitive1);
def_comparison_primitive_enum! (ComparisonPrimitive2);
def_comparison_primitive_enum! (ComparisonPrimitive3);
def_comparison_primitive_enum! (ComparisonPrimitive4);
def_comparison_primitive_enum! (ComparisonPrimitiveN);
def_comparison_primitive_enum! (ComparisonPrimitiveV);


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ComparisonPrimitive0 {}

#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ComparisonPrimitive5 {}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_0_evaluate (primitive : ComparisonPrimitive0, _negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, input_1 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive1::EquivalentByIdentity =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueStrict =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueStrictRecursive =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueCoerced =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::EquivalentByValueCoercedRecursive =>
			return compare_1 (input_1, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::GenericLesser =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::GenericLesserOrEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::GenericEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::GenericGreaterOrEqual =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::GenericGreater =>
			return compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::BooleanLesser =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::BooleanLesserOrEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::BooleanEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::BooleanGreaterOrEqual =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::BooleanGreater =>
			return boolean_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::NumberLesser =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::NumberLesserOrEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::NumberEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::NumberGreaterOrEqual =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::NumberGreater =>
			return number_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseSensitiveLesser =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseSensitiveEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseSensitiveGreater =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseInsensitiveLesser =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseInsensitiveEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::CharacterCaseInsensitiveGreater =>
			return character_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseSensitiveLesser =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseSensitiveEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseSensitiveGreater =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseInsensitiveLesser =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseInsensitiveEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive1::StringCaseInsensitiveGreater =>
			return string_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesser =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseSensitiveGreater =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesser =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive1::SymbolCaseInsensitiveGreater =>
			return symbol_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseSensitiveLesser =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseSensitiveEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseSensitiveGreater =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseInsensitiveLesser =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseInsensitiveEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive1::KeywordCaseInsensitiveGreater =>
			return keyword_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive1::UniqueLesser =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive1::UniqueLesserOrEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive1::UniqueEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive1::UniqueGreaterOrEqual =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive1::UniqueGreater =>
			return unique_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive1::BytesLesser =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive1::BytesLesserOrEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive1::BytesEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive1::BytesGreaterOrEqual =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive1::BytesGreater =>
			return bytes_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::PairLesser =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::PairLesserOrEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::PairEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::PairGreaterOrEqual =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive1::PairGreater =>
			return pair_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive1::ArrayLesser =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive1::ArrayLesserOrEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive1::ArrayEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive1::ArrayGreaterOrEqual =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive1::ArrayGreater =>
			return array_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive1::ValuesLesser =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive1::ValuesLesserOrEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive1::ValuesEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive1::ValuesGreaterOrEqual =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive1::ValuesGreater =>
			return values_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive1::RecordLesser =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive1::RecordLesserOrEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive1::RecordEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive1::RecordGreaterOrEqual =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive1::RecordGreater =>
			return record_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive1::PathLesser =>
			return path_compare_1 (input_1, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive1::PathLesserOrEqual =>
			return path_compare_1 (input_1, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive1::PathEqual =>
			return path_compare_1 (input_1, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive1::PathGreaterOrEqual =>
			return path_compare_1 (input_1, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive1::PathGreater =>
			return path_compare_1 (input_1, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, input_1 : &Value, input_2 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive2::EquivalentByIdentity =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueStrict =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueStrictRecursive =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueCoerced =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::EquivalentByValueCoercedRecursive =>
			return compare_2 (input_1, input_2, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::GenericLesser =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::GenericLesserOrEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::GenericEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::GenericGreaterOrEqual =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::GenericGreater =>
			return compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::BooleanLesser =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::BooleanLesserOrEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::BooleanEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::BooleanGreaterOrEqual =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::BooleanGreater =>
			return boolean_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::NumberLesser =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::NumberLesserOrEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::NumberEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::NumberGreaterOrEqual =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::NumberGreater =>
			return number_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseSensitiveLesser =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseSensitiveEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseSensitiveGreater =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseInsensitiveLesser =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseInsensitiveEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::CharacterCaseInsensitiveGreater =>
			return character_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseSensitiveLesser =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseSensitiveEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseSensitiveGreater =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseInsensitiveLesser =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseInsensitiveEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive2::StringCaseInsensitiveGreater =>
			return string_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesser =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreater =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesser =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreater =>
			return symbol_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseSensitiveLesser =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseSensitiveEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseSensitiveGreater =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseInsensitiveLesser =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseInsensitiveEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive2::KeywordCaseInsensitiveGreater =>
			return keyword_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive2::UniqueLesser =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive2::UniqueLesserOrEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive2::UniqueEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive2::UniqueGreaterOrEqual =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive2::UniqueGreater =>
			return unique_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive2::BytesLesser =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive2::BytesLesserOrEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive2::BytesEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive2::BytesGreaterOrEqual =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive2::BytesGreater =>
			return bytes_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::PairLesser =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::PairLesserOrEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::PairEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::PairGreaterOrEqual =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive2::PairGreater =>
			return pair_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive2::ArrayLesser =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive2::ArrayLesserOrEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive2::ArrayEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive2::ArrayGreaterOrEqual =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive2::ArrayGreater =>
			return array_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive2::ValuesLesser =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive2::ValuesLesserOrEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive2::ValuesEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive2::ValuesGreaterOrEqual =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive2::ValuesGreater =>
			return values_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive2::RecordLesser =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive2::RecordLesserOrEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive2::RecordEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive2::RecordGreaterOrEqual =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive2::RecordGreater =>
			return record_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive2::PathLesser =>
			return path_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive2::PathLesserOrEqual =>
			return path_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive2::PathEqual =>
			return path_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive2::PathGreaterOrEqual =>
			return path_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive2::PathGreater =>
			return path_compare_2 (input_1, input_2, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive3::EquivalentByIdentity =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueStrict =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueStrictRecursive =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueCoerced =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::EquivalentByValueCoercedRecursive =>
			return compare_3 (input_1, input_2, input_3, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::GenericLesser =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::GenericLesserOrEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::GenericEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::GenericGreaterOrEqual =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::GenericGreater =>
			return compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::BooleanLesser =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::BooleanLesserOrEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::BooleanEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::BooleanGreaterOrEqual =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::BooleanGreater =>
			return boolean_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::NumberLesser =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::NumberLesserOrEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::NumberEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::NumberGreaterOrEqual =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::NumberGreater =>
			return number_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseSensitiveLesser =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseSensitiveEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseSensitiveGreater =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseInsensitiveLesser =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseInsensitiveEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::CharacterCaseInsensitiveGreater =>
			return character_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseSensitiveLesser =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseSensitiveLesserOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseSensitiveEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseSensitiveGreater =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseInsensitiveLesser =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseInsensitiveEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive3::StringCaseInsensitiveGreater =>
			return string_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesser =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseSensitiveGreater =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesser =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive3::SymbolCaseInsensitiveGreater =>
			return symbol_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseSensitiveLesser =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseSensitiveEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseSensitiveGreater =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseInsensitiveLesser =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseInsensitiveEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive3::KeywordCaseInsensitiveGreater =>
			return keyword_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive3::UniqueLesser =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive3::UniqueLesserOrEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive3::UniqueEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive3::UniqueGreaterOrEqual =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive3::UniqueGreater =>
			return unique_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive3::BytesLesser =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive3::BytesLesserOrEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive3::BytesEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive3::BytesGreaterOrEqual =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive3::BytesGreater =>
			return bytes_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::PairLesser =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::PairLesserOrEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::PairEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::PairGreaterOrEqual =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive3::PairGreater =>
			return pair_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive3::ArrayLesser =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive3::ArrayLesserOrEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive3::ArrayEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive3::ArrayGreaterOrEqual =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive3::ArrayGreater =>
			return array_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive3::ValuesLesser =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive3::ValuesLesserOrEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive3::ValuesEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive3::ValuesGreaterOrEqual =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive3::ValuesGreater =>
			return values_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive3::RecordLesser =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive3::RecordLesserOrEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive3::RecordEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive3::RecordGreaterOrEqual =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive3::RecordGreater =>
			return record_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive3::PathLesser =>
			return path_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive3::PathLesserOrEqual =>
			return path_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive3::PathEqual =>
			return path_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive3::PathGreaterOrEqual =>
			return path_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive3::PathGreater =>
			return path_compare_3 (input_1, input_2, input_3, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitive4::EquivalentByIdentity =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueStrict =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueStrictRecursive =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueCoerced =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::EquivalentByValueCoercedRecursive =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::GenericLesser =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::GenericLesserOrEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::GenericEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::GenericGreaterOrEqual =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::GenericGreater =>
			return compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::BooleanLesser =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::BooleanLesserOrEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::BooleanEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::BooleanGreaterOrEqual =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::BooleanGreater =>
			return boolean_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::NumberLesser =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::NumberLesserOrEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::NumberEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::NumberGreaterOrEqual =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::NumberGreater =>
			return number_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseSensitiveLesser =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseSensitiveEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseSensitiveGreater =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseInsensitiveLesser =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseInsensitiveEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::CharacterCaseInsensitiveGreater =>
			return character_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseSensitiveLesser =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseSensitiveLesserOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseSensitiveEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseSensitiveGreater =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseInsensitiveLesser =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseInsensitiveEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitive4::StringCaseInsensitiveGreater =>
			return string_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesser =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseSensitiveGreater =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesser =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitive4::SymbolCaseInsensitiveGreater =>
			return symbol_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseSensitiveLesser =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseSensitiveEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseSensitiveGreater =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseInsensitiveLesser =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseInsensitiveEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitive4::KeywordCaseInsensitiveGreater =>
			return keyword_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive4::UniqueLesser =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive4::UniqueLesserOrEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive4::UniqueEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive4::UniqueGreaterOrEqual =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitive4::UniqueGreater =>
			return unique_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive4::BytesLesser =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive4::BytesLesserOrEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive4::BytesEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive4::BytesGreaterOrEqual =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitive4::BytesGreater =>
			return bytes_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::PairLesser =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::PairLesserOrEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::PairEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::PairGreaterOrEqual =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitive4::PairGreater =>
			return pair_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive4::ArrayLesser =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive4::ArrayLesserOrEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive4::ArrayEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive4::ArrayGreaterOrEqual =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitive4::ArrayGreater =>
			return array_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive4::ValuesLesser =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive4::ValuesLesserOrEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive4::ValuesEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive4::ValuesGreaterOrEqual =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitive4::ValuesGreater =>
			return values_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive4::RecordLesser =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive4::RecordLesserOrEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive4::RecordEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive4::RecordGreaterOrEqual =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitive4::RecordGreater =>
			return record_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive4::PathLesser =>
			return path_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive4::PathLesserOrEqual =>
			return path_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive4::PathEqual =>
			return path_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive4::PathGreaterOrEqual =>
			return path_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitive4::PathGreater =>
			return path_compare_4 (input_1, input_2, input_3, input_4, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, inputs : &[&Value], negated : bool, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ComparisonPrimitiveN::EquivalentByIdentity =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByIdentity, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueStrict =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueStrictRecursive =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueCoerced =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::EquivalentByValueCoercedRecursive =>
			return compare_n (inputs, Comparison::Equivalence (Equivalence::ByValue, Some (true), Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::GenericLesser =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::GenericLesserOrEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::GenericEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::GenericGreaterOrEqual =>
			return compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::GenericGreater =>
			return compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanLesser =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanLesserOrEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanGreaterOrEqual =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::BooleanGreater =>
			return boolean_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::NumberLesser =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::NumberLesserOrEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::NumberEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::NumberGreaterOrEqual =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::NumberGreater =>
			return number_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseSensitiveLesser =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseSensitiveLesserOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseSensitiveEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseSensitiveGreaterOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseSensitiveGreater =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesser =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseInsensitiveLesserOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseInsensitiveEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreaterOrEqual =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::CharacterCaseInsensitiveGreater =>
			return character_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseSensitiveLesser =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseSensitiveLesserOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseSensitiveEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseSensitiveGreaterOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseSensitiveGreater =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseInsensitiveLesser =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseInsensitiveLesserOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseInsensitiveEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseInsensitiveGreaterOrEqual =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ComparisonPrimitiveN::StringCaseInsensitiveGreater =>
			return string_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesser =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseSensitiveGreater =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesser =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		ComparisonPrimitiveN::SymbolCaseInsensitiveGreater =>
			return symbol_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseSensitiveLesser =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseSensitiveLesserOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseSensitiveEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseSensitiveGreaterOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseSensitiveGreater =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (true), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseInsensitiveLesser =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseInsensitiveLesserOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseInsensitiveEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseInsensitiveGreaterOrEqual =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ComparisonPrimitiveN::KeywordCaseInsensitiveGreater =>
			return keyword_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, Some (false), negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitiveN::UniqueLesser =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitiveN::UniqueLesserOrEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitiveN::UniqueEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitiveN::UniqueGreaterOrEqual =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ComparisonPrimitiveN::UniqueGreater =>
			return unique_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitiveN::BytesLesser =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitiveN::BytesLesserOrEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitiveN::BytesEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitiveN::BytesGreaterOrEqual =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ComparisonPrimitiveN::BytesGreater =>
			return bytes_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::PairLesser =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::PairLesserOrEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::PairEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::PairGreaterOrEqual =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		ComparisonPrimitiveN::PairGreater =>
			return pair_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitiveN::ArrayLesser =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitiveN::ArrayLesserOrEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitiveN::ArrayEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitiveN::ArrayGreaterOrEqual =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ComparisonPrimitiveN::ArrayGreater =>
			return array_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitiveN::ValuesLesser =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitiveN::ValuesLesserOrEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitiveN::ValuesEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitiveN::ValuesGreaterOrEqual =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ComparisonPrimitiveN::ValuesGreater =>
			return values_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitiveN::RecordLesser =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitiveN::RecordLesserOrEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitiveN::RecordEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitiveN::RecordGreaterOrEqual =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ComparisonPrimitiveN::RecordGreater =>
			return record_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitiveN::PathLesser =>
			return path_compare_n (inputs, Comparison::Ordering (Ordering::Lesser, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitiveN::PathLesserOrEqual =>
			return path_compare_n (inputs, Comparison::Ordering (Ordering::LesserOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitiveN::PathEqual =>
			return path_compare_n (inputs, Comparison::Ordering (Ordering::Equal, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitiveN::PathGreaterOrEqual =>
			return path_compare_n (inputs, Comparison::Ordering (Ordering::GreaterOrEqual, None, None, negated)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ComparisonPrimitiveN::PathGreater =>
			return path_compare_n (inputs, Comparison::Ordering (Ordering::Greater, None, None, negated)) .into_0 (),
		
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
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseSensitiveLesser =>
					Some ($alternative::CharacterCaseSensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual =>
					Some ($alternative::CharacterCaseSensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseSensitiveEqual =>
					Some ($alternative::CharacterCaseSensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual =>
					Some ($alternative::CharacterCaseSensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseSensitiveGreater =>
					Some ($alternative::CharacterCaseSensitiveGreater),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseInsensitiveLesser =>
					Some ($alternative::CharacterCaseInsensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual =>
					Some ($alternative::CharacterCaseInsensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseInsensitiveEqual =>
					Some ($alternative::CharacterCaseInsensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::CharacterCaseInsensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::CharacterCaseInsensitiveGreater =>
					Some ($alternative::CharacterCaseInsensitiveGreater),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseSensitiveLesser =>
					Some ($alternative::StringCaseSensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual =>
					Some ($alternative::StringCaseSensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseSensitiveEqual =>
					Some ($alternative::StringCaseSensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual =>
					Some ($alternative::StringCaseSensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseSensitiveGreater =>
					Some ($alternative::StringCaseSensitiveGreater),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseInsensitiveLesser =>
					Some ($alternative::StringCaseInsensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual =>
					Some ($alternative::StringCaseInsensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseInsensitiveEqual =>
					Some ($alternative::StringCaseInsensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
				ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::StringCaseInsensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseSensitiveLesser =>
					Some ($alternative::KeywordCaseSensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseSensitiveLesserOrEqual =>
					Some ($alternative::KeywordCaseSensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseSensitiveEqual =>
					Some ($alternative::KeywordCaseSensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseSensitiveGreaterOrEqual =>
					Some ($alternative::KeywordCaseSensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseSensitiveGreater =>
					Some ($alternative::KeywordCaseSensitiveGreater),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseInsensitiveLesser =>
					Some ($alternative::KeywordCaseInsensitiveLesser),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseInsensitiveLesserOrEqual =>
					Some ($alternative::KeywordCaseInsensitiveLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseInsensitiveEqual =>
					Some ($alternative::KeywordCaseInsensitiveEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseInsensitiveGreaterOrEqual =>
					Some ($alternative::KeywordCaseInsensitiveGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
				ComparisonPrimitiveV::KeywordCaseInsensitiveGreater =>
					Some ($alternative::KeywordCaseInsensitiveGreater),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ComparisonPrimitiveV::UniqueLesser =>
					Some ($alternative::UniqueLesser),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ComparisonPrimitiveV::UniqueLesserOrEqual =>
					Some ($alternative::UniqueLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ComparisonPrimitiveV::UniqueEqual =>
					Some ($alternative::UniqueEqual),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ComparisonPrimitiveV::UniqueGreaterOrEqual =>
					Some ($alternative::UniqueGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_unique" ) ]
				ComparisonPrimitiveV::UniqueGreater =>
					Some ($alternative::UniqueGreater),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ComparisonPrimitiveV::BytesLesser =>
					Some ($alternative::BytesLesser),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ComparisonPrimitiveV::BytesLesserOrEqual =>
					Some ($alternative::BytesLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ComparisonPrimitiveV::BytesEqual =>
					Some ($alternative::BytesEqual),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
				ComparisonPrimitiveV::BytesGreaterOrEqual =>
					Some ($alternative::BytesGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
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
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ComparisonPrimitiveV::ArrayLesser =>
					Some ($alternative::ArrayLesser),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ComparisonPrimitiveV::ArrayLesserOrEqual =>
					Some ($alternative::ArrayLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ComparisonPrimitiveV::ArrayEqual =>
					Some ($alternative::ArrayEqual),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ComparisonPrimitiveV::ArrayGreaterOrEqual =>
					Some ($alternative::ArrayGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				ComparisonPrimitiveV::ArrayGreater =>
					Some ($alternative::ArrayGreater),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ComparisonPrimitiveV::ValuesLesser =>
					Some ($alternative::ValuesLesser),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ComparisonPrimitiveV::ValuesLesserOrEqual =>
					Some ($alternative::ValuesLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ComparisonPrimitiveV::ValuesEqual =>
					Some ($alternative::ValuesEqual),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ComparisonPrimitiveV::ValuesGreaterOrEqual =>
					Some ($alternative::ValuesGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_values_values" ) ]
				ComparisonPrimitiveV::ValuesGreater =>
					Some ($alternative::ValuesGreater),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ComparisonPrimitiveV::RecordLesser =>
					Some ($alternative::RecordLesser),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ComparisonPrimitiveV::RecordLesserOrEqual =>
					Some ($alternative::RecordLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ComparisonPrimitiveV::RecordEqual =>
					Some ($alternative::RecordEqual),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ComparisonPrimitiveV::RecordGreaterOrEqual =>
					Some ($alternative::RecordGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
				ComparisonPrimitiveV::RecordGreater =>
					Some ($alternative::RecordGreater),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ComparisonPrimitiveV::PathLesser =>
					Some ($alternative::PathLesser),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ComparisonPrimitiveV::PathLesserOrEqual =>
					Some ($alternative::PathLesserOrEqual),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ComparisonPrimitiveV::PathEqual =>
					Some ($alternative::PathEqual),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ComparisonPrimitiveV::PathGreaterOrEqual =>
					Some ($alternative::PathGreaterOrEqual),
				#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
				ComparisonPrimitiveV::PathGreater =>
					Some ($alternative::PathGreater),
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

