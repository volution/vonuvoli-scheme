

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::generate_binding_templates as language_builtins_generate_binding_templates;
	pub use super::generate_definitions as language_builtins_generate_definitions;
}




pub fn generate_binding_templates () -> (Outcome<StdVec<ContextBindingTemplate>>) {
	
	let definitions = try! (generate_definitions ());
	
	let templates = vec_map! (
			definitions,
			(identifier, value),
			ContextBindingTemplate {
					identifier : identifier,
					value : Some (value),
					immutable : true,
				}
		);
	
	succeed! (templates);
}




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			("locals", SyntaxPrimitiveN::Locals.into ()), // https://docs.racket-lang.org/reference/local.html
			("set-values!", SyntaxPrimitive2::SetValues.into ()),
			
			("call", FunctionsPrimitiveN::Call.into ()),
			("call-with-values", FunctionsPrimitive2::CallWithValues.into ()),
			
			("and", BooleanPrimitiveN::And.into ()),
			("or", BooleanPrimitiveN::Or.into ()),
			("xor", BooleanPrimitiveN::Xor.into ()),
			("nand", BooleanPrimitiveN::Nand.into ()),
			("nor", BooleanPrimitiveN::Nor.into ()),
			("nxor", BooleanPrimitiveN::Nxor.into ()),
			
			("string-reverse", StringPrimitive1::StringReverse.into ()),
			
			("symbol-upcase", StringPrimitive1::SymbolToUpperCase.into ()),
			("symbol-downcase", StringPrimitive1::SymbolToLowerCase.into ()),
			("symbol-foldcase", StringPrimitive1::SymbolToFoldCase.into ()),
			
			("bytevector-reverse", BytesPrimitive1::BytesReverse.into ()),
			("bytevector-u8-fill", BytesPrimitiveN::BytesRangeFill.into ()),
			("bytevector-u8-map", FunctionsPrimitiveN::BytesMap.into ()),
			("bytevector-u8-for-each", FunctionsPrimitiveN::BytesIterate.into ()),
			("bytevector->vector", BytesPrimitiveN::BytesRangeToArray.into ()),
			("vector->bytevector", BytesPrimitiveN::ArrayRangeToBytes.into ()),
			("bytevector->list", BytesPrimitiveN::BytesRangeToList.into ()),
			("list->bytevector", BytesPrimitiveN::ListRangeToBytes.into ()),
			
			("vector-reverse", ArrayPrimitive1::ArrayReverse.into ()),
			
			("equivalent-by-identity?", ComparisonPrimitiveN::EquivalentByIdentity.into ()),
			("equivalent-by-value-strict?", ComparisonPrimitiveN::EquivalentByValueStrict.into ()),
			("equivalent-by-value-strict-recursive?", ComparisonPrimitiveN::EquivalentByValueStrictRecursive.into ()),
			("equivalent-by-value-coerced?", ComparisonPrimitiveN::EquivalentByValueCoerced.into ()),
			("equivalent-by-value-coerced-recursive?", ComparisonPrimitiveN::EquivalentByValueCoercedRecursive.into ()),
			
			("boolean<?", ComparisonPrimitiveN::BooleanLesser.into ()),
			("boolean>?", ComparisonPrimitiveN::BooleanGreater.into ()),
			("boolean<=?", ComparisonPrimitiveN::BooleanLesserOrEqual.into ()),
			("boolean>=?", ComparisonPrimitiveN::BooleanGreaterOrEqual.into ()),
			
			("symbol<?", ComparisonPrimitiveN::SymbolCaseSensitiveLesser.into ()),
			("symbol>?", ComparisonPrimitiveN::SymbolCaseSensitiveGreater.into ()),
			("symbol<=?", ComparisonPrimitiveN::SymbolCaseSensitiveLesserOrEqual.into ()),
			("symbol>=?", ComparisonPrimitiveN::SymbolCaseSensitiveGreaterOrEqual.into ()),
			
			("symbol-ci=?", ComparisonPrimitiveN::SymbolCaseInsensitiveEqual.into ()),
			("symbol-ci<?", ComparisonPrimitiveN::SymbolCaseInsensitiveLesser.into ()),
			("symbol-ci>?", ComparisonPrimitiveN::SymbolCaseInsensitiveGreater.into ()),
			("symbol-ci<=?", ComparisonPrimitiveN::SymbolCaseInsensitiveLesserOrEqual.into ()),
			("symbol-ci>=?", ComparisonPrimitiveN::SymbolCaseInsensitiveGreaterOrEqual.into ()),
			
			("bytevector=?", ComparisonPrimitiveN::BytesEqual.into ()),
			("bytevector<?", ComparisonPrimitiveN::BytesLesser.into ()),
			("bytevector>?", ComparisonPrimitiveN::BytesGreater.into ()),
			("bytevector<=?", ComparisonPrimitiveN::BytesLesserOrEqual.into ()),
			("bytevector>=?", ComparisonPrimitiveN::BytesGreaterOrEqual.into ()),
			
			("pair=?", ComparisonPrimitiveN::PairEqual.into ()),
			("pair<?", ComparisonPrimitiveN::PairLesser.into ()),
			("pair>?", ComparisonPrimitiveN::PairGreater.into ()),
			("pair<=?", ComparisonPrimitiveN::PairLesserOrEqual.into ()),
			("pair>=?", ComparisonPrimitiveN::PairGreaterOrEqual.into ()),
			
			("vector=?", ComparisonPrimitiveN::ArrayEqual.into ()),
			("vector<?", ComparisonPrimitiveN::ArrayLesser.into ()),
			("vector>?", ComparisonPrimitiveN::ArrayGreater.into ()),
			("vector<=?", ComparisonPrimitiveN::ArrayLesserOrEqual.into ()),
			("vector>=?", ComparisonPrimitiveN::ArrayGreaterOrEqual.into ()),
			
			("values=?", ComparisonPrimitiveN::ValuesEqual.into ()),
			("values<?", ComparisonPrimitiveN::ValuesLesser.into ()),
			("values>?", ComparisonPrimitiveN::ValuesGreater.into ()),
			("values<=?", ComparisonPrimitiveN::ValuesLesserOrEqual.into ()),
			("values>=?", ComparisonPrimitiveN::ValuesGreaterOrEqual.into ()),
			
			("generic=?", ComparisonPrimitiveN::GenericEqual.into ()),
			("generic<?", ComparisonPrimitiveN::GenericLesser.into ()),
			("generic>?", ComparisonPrimitiveN::GenericGreater.into ()),
			("generic<=?", ComparisonPrimitiveN::GenericLesserOrEqual.into ()),
			("generic>=?", ComparisonPrimitiveN::GenericGreaterOrEqual.into ()),
			
		];
	
	let definitions = vec_map! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

