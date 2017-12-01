

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
pub enum ComparisonPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ComparisonPrimitive4 {}


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
	let input_class = input_1.class ();
	let output = input_class != ValueClass::Undefined;
	let output = output && match primitive {
		
		ComparisonPrimitive1::EquivalentByIdentity |
		ComparisonPrimitive1::EquivalentByValueStrict |
		ComparisonPrimitive1::EquivalentByValueStrictRecursive |
		ComparisonPrimitive1::EquivalentByValueCoerced |
		ComparisonPrimitive1::EquivalentByValueCoercedRecursive =>
			true,
		
		ComparisonPrimitive1::GenericLesser |
		ComparisonPrimitive1::GenericLesserOrEqual |
		ComparisonPrimitive1::GenericEqual |
		ComparisonPrimitive1::GenericGreaterOrEqual |
		ComparisonPrimitive1::GenericGreater =>
			true,
		
		ComparisonPrimitive1::BooleanLesser |
		ComparisonPrimitive1::BooleanLesserOrEqual |
		ComparisonPrimitive1::BooleanEqual |
		ComparisonPrimitive1::BooleanGreaterOrEqual |
		ComparisonPrimitive1::BooleanGreater =>
			if input_class == ValueClass::Boolean {
				true
			} else {
				fail! (0xe88d22bf);
			},
		
		ComparisonPrimitive1::NumberLesser |
		ComparisonPrimitive1::NumberLesserOrEqual |
		ComparisonPrimitive1::NumberEqual |
		ComparisonPrimitive1::NumberGreaterOrEqual |
		ComparisonPrimitive1::NumberGreater =>
			if input_class == ValueClass::NumberInteger {
				true
			} else if input_class == ValueClass::NumberReal {
				if NumberReal::as_ref (input_1) .is_nan () {
					false
				} else {
					true
				}
			} else {
				fail! (0x0e6bfc4b);
			},
		
		ComparisonPrimitive1::CharacterCaseSensitiveLesser |
		ComparisonPrimitive1::CharacterCaseSensitiveLesserOrEqual |
		ComparisonPrimitive1::CharacterCaseSensitiveEqual |
		ComparisonPrimitive1::CharacterCaseSensitiveGreaterOrEqual |
		ComparisonPrimitive1::CharacterCaseSensitiveGreater |
		ComparisonPrimitive1::CharacterCaseInsensitiveLesser |
		ComparisonPrimitive1::CharacterCaseInsensitiveLesserOrEqual |
		ComparisonPrimitive1::CharacterCaseInsensitiveEqual |
		ComparisonPrimitive1::CharacterCaseInsensitiveGreaterOrEqual |
		ComparisonPrimitive1::CharacterCaseInsensitiveGreater =>
			if input_class == ValueClass::Character {
				true
			} else {
				fail! (0x8534e7de);
			},
		
		ComparisonPrimitive1::StringCaseSensitiveLesser |
		ComparisonPrimitive1::StringCaseSensitiveLesserOrEqual |
		ComparisonPrimitive1::StringCaseSensitiveEqual |
		ComparisonPrimitive1::StringCaseSensitiveGreaterOrEqual |
		ComparisonPrimitive1::StringCaseSensitiveGreater |
		ComparisonPrimitive1::StringCaseInsensitiveLesser |
		ComparisonPrimitive1::StringCaseInsensitiveLesserOrEqual |
		ComparisonPrimitive1::StringCaseInsensitiveEqual |
		ComparisonPrimitive1::StringCaseInsensitiveGreaterOrEqual |
		ComparisonPrimitive1::StringCaseInsensitiveGreater =>
			if input_class == ValueClass::String {
				true
			} else {
				fail! (0xfa796630);
			},
		
		ComparisonPrimitive1::SymbolCaseSensitiveLesser |
		ComparisonPrimitive1::SymbolCaseSensitiveLesserOrEqual |
		ComparisonPrimitive1::SymbolCaseSensitiveEqual |
		ComparisonPrimitive1::SymbolCaseSensitiveGreaterOrEqual |
		ComparisonPrimitive1::SymbolCaseSensitiveGreater |
		ComparisonPrimitive1::SymbolCaseInsensitiveLesser |
		ComparisonPrimitive1::SymbolCaseInsensitiveLesserOrEqual |
		ComparisonPrimitive1::SymbolCaseInsensitiveEqual |
		ComparisonPrimitive1::SymbolCaseInsensitiveGreaterOrEqual |
		ComparisonPrimitive1::SymbolCaseInsensitiveGreater =>
			if input_class == ValueClass::Symbol {
				true
			} else {
				fail! (0x04423388);
			},
		
		ComparisonPrimitive1::BytesLesser |
		ComparisonPrimitive1::BytesLesserOrEqual |
		ComparisonPrimitive1::BytesEqual |
		ComparisonPrimitive1::BytesGreaterOrEqual |
		ComparisonPrimitive1::BytesGreater =>
			if input_class == ValueClass::Bytes {
				true
			} else {
				fail! (0xdbb68433);
			},
		
		ComparisonPrimitive1::PairLesser |
		ComparisonPrimitive1::PairLesserOrEqual |
		ComparisonPrimitive1::PairEqual |
		ComparisonPrimitive1::PairGreaterOrEqual |
		ComparisonPrimitive1::PairGreater =>
			if input_class == ValueClass::Pair {
				true
			} else {
				fail! (0xb578af9b);
			},
		
		ComparisonPrimitive1::ArrayLesser |
		ComparisonPrimitive1::ArrayLesserOrEqual |
		ComparisonPrimitive1::ArrayEqual |
		ComparisonPrimitive1::ArrayGreaterOrEqual |
		ComparisonPrimitive1::ArrayGreater =>
			if input_class == ValueClass::Array {
				true
			} else {
				fail! (0x729535e2);
			},
		
		ComparisonPrimitive1::ValuesLesser |
		ComparisonPrimitive1::ValuesLesserOrEqual |
		ComparisonPrimitive1::ValuesEqual |
		ComparisonPrimitive1::ValuesGreaterOrEqual |
		ComparisonPrimitive1::ValuesGreater =>
			if input_class == ValueClass::Values {
				true
			} else {
				fail! (0x3bb51ff6);
			},
		
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
			try! (boolean_compare_2 (try_as_boolean_ref! (input_1), try_as_boolean_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::BooleanLesserOrEqual =>
			try! (boolean_compare_2 (try_as_boolean_ref! (input_1), try_as_boolean_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::BooleanEqual =>
			try! (boolean_compare_2 (try_as_boolean_ref! (input_1), try_as_boolean_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::BooleanGreaterOrEqual =>
			try! (boolean_compare_2 (try_as_boolean_ref! (input_1), try_as_boolean_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::BooleanGreater =>
			try! (boolean_compare_2 (try_as_boolean_ref! (input_1), try_as_boolean_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, None))),
		
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
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveLesserOrEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreaterOrEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseSensitiveGreater =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesser =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveLesserOrEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreaterOrEqual =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::CharacterCaseInsensitiveGreater =>
			try! (character_compare_2 (try_as_character_ref! (input_1), try_as_character_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseSensitiveLesser =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveLesserOrEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveGreaterOrEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseSensitiveGreater =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesser =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveLesserOrEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreaterOrEqual =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::StringCaseInsensitiveGreater =>
			try! (string_compare_2 (try_as_string_ref! (input_1), try_as_string_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesser =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveLesserOrEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreaterOrEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseSensitiveGreater =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (true)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesser =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveLesserOrEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreaterOrEqual =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, Some (false)))),
		
		ComparisonPrimitive2::SymbolCaseInsensitiveGreater =>
			try! (symbol_compare_2 (try_as_symbol_ref! (input_1), try_as_symbol_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, Some (false)))),
		
		ComparisonPrimitive2::BytesLesser =>
			try! (bytes_compare_2 (try_as_bytes_ref! (input_1), try_as_bytes_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::BytesLesserOrEqual =>
			try! (bytes_compare_2 (try_as_bytes_ref! (input_1), try_as_bytes_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::BytesEqual =>
			try! (bytes_compare_2 (try_as_bytes_ref! (input_1), try_as_bytes_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::BytesGreaterOrEqual =>
			try! (bytes_compare_2 (try_as_bytes_ref! (input_1), try_as_bytes_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::BytesGreater =>
			try! (bytes_compare_2 (try_as_bytes_ref! (input_1), try_as_bytes_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::PairLesser =>
			try! (pair_compare_2 (try_as_pair_ref! (input_1), try_as_pair_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::PairLesserOrEqual =>
			try! (pair_compare_2 (try_as_pair_ref! (input_1), try_as_pair_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::PairEqual =>
			try! (pair_compare_2 (try_as_pair_ref! (input_1), try_as_pair_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::PairGreaterOrEqual =>
			try! (pair_compare_2 (try_as_pair_ref! (input_1), try_as_pair_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::PairGreater =>
			try! (pair_compare_2 (try_as_pair_ref! (input_1), try_as_pair_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::ArrayLesser =>
			try! (array_compare_2 (try_as_array_ref! (input_1), try_as_array_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::ArrayLesserOrEqual =>
			try! (array_compare_2 (try_as_array_ref! (input_1), try_as_array_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::ArrayEqual =>
			try! (array_compare_2 (try_as_array_ref! (input_1), try_as_array_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::ArrayGreaterOrEqual =>
			try! (array_compare_2 (try_as_array_ref! (input_1), try_as_array_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::ArrayGreater =>
			try! (array_compare_2 (try_as_array_ref! (input_1), try_as_array_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, None))),
		
		ComparisonPrimitive2::ValuesLesser =>
			try! (values_compare_2 (try_as_values_ref! (input_1), try_as_values_ref! (input_2), Comparison::Ordering (Ordering::Lesser, None, None))),
		
		ComparisonPrimitive2::ValuesLesserOrEqual =>
			try! (values_compare_2 (try_as_values_ref! (input_1), try_as_values_ref! (input_2), Comparison::Ordering (Ordering::LesserOrEqual, None, None))),
		
		ComparisonPrimitive2::ValuesEqual =>
			try! (values_compare_2 (try_as_values_ref! (input_1), try_as_values_ref! (input_2), Comparison::Ordering (Ordering::Equal, None, None))),
		
		ComparisonPrimitive2::ValuesGreaterOrEqual =>
			try! (values_compare_2 (try_as_values_ref! (input_1), try_as_values_ref! (input_2), Comparison::Ordering (Ordering::GreaterOrEqual, None, None))),
		
		ComparisonPrimitive2::ValuesGreater =>
			try! (values_compare_2 (try_as_values_ref! (input_1), try_as_values_ref! (input_2), Comparison::Ordering (Ordering::Greater, None, None))),
		
	};
	
	succeed! (output.into ());
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

