

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
	
	let templates = vec_map_into! (
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
			
			
			("locals", SyntaxPrimitiveV::Locals.into ()), // https://docs.racket-lang.org/reference/local.html
			("set!-values", SyntaxPrimitiveV::SetValues.into ()),
			
			("call", FunctionsPrimitiveV::Call.into ()),
			("call-with-values", FunctionsPrimitive2::CallWithValues.into ()),
			
			("and", BooleanPrimitiveV::And.into ()),
			("or", BooleanPrimitiveV::Or.into ()),
			("xor", BooleanPrimitiveV::Xor.into ()),
			("nand", BooleanPrimitiveV::Nand.into ()),
			("nor", BooleanPrimitiveV::Nor.into ()),
			("nxor", BooleanPrimitiveV::Nxor.into ()),
			
			("string-reverse", StringPrimitive1::StringReverse.into ()),
			
			("symbol-upcase", StringPrimitive1::SymbolToUpperCase.into ()),
			("symbol-downcase", StringPrimitive1::SymbolToLowerCase.into ()),
			("symbol-foldcase", StringPrimitive1::SymbolToFoldCase.into ()),
			
			("bytevector-reverse", BytesPrimitive1::BytesReverse.into ()),
			("bytevector-u8-fill", BytesPrimitiveV::BytesRangeFill.into ()),
			("bytevector-u8-map", FunctionsPrimitiveV::BytesMap.into ()),
			("bytevector-u8-for-each", FunctionsPrimitiveV::BytesIterate.into ()),
			("bytevector->vector", BytesPrimitiveV::BytesRangeToArray.into ()),
			("vector->bytevector", BytesPrimitiveV::ArrayRangeToBytes.into ()),
			("bytevector->list", BytesPrimitiveV::BytesRangeToList.into ()),
			("list->bytevector", BytesPrimitiveV::ListRangeToBytes.into ()),
			
			("vector-reverse", ArrayPrimitive1::ArrayReverse.into ()),
			
			("equivalent-by-identity?", ComparisonPrimitiveV::EquivalentByIdentity.into ()),
			("equivalent-by-value-strict?", ComparisonPrimitiveV::EquivalentByValueStrict.into ()),
			("equivalent-by-value-strict-recursive?", ComparisonPrimitiveV::EquivalentByValueStrictRecursive.into ()),
			("equivalent-by-value-coerced?", ComparisonPrimitiveV::EquivalentByValueCoerced.into ()),
			("equivalent-by-value-coerced-recursive?", ComparisonPrimitiveV::EquivalentByValueCoercedRecursive.into ()),
			
			("boolean<?", ComparisonPrimitiveV::BooleanLesser.into ()),
			("boolean>?", ComparisonPrimitiveV::BooleanGreater.into ()),
			("boolean<=?", ComparisonPrimitiveV::BooleanLesserOrEqual.into ()),
			("boolean>=?", ComparisonPrimitiveV::BooleanGreaterOrEqual.into ()),
			
			("symbol<?", ComparisonPrimitiveV::SymbolCaseSensitiveLesser.into ()),
			("symbol>?", ComparisonPrimitiveV::SymbolCaseSensitiveGreater.into ()),
			("symbol<=?", ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual.into ()),
			("symbol>=?", ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual.into ()),
			
			("symbol-ci=?", ComparisonPrimitiveV::SymbolCaseInsensitiveEqual.into ()),
			("symbol-ci<?", ComparisonPrimitiveV::SymbolCaseInsensitiveLesser.into ()),
			("symbol-ci>?", ComparisonPrimitiveV::SymbolCaseInsensitiveGreater.into ()),
			("symbol-ci<=?", ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual.into ()),
			("symbol-ci>=?", ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual.into ()),
			
			("bytevector=?", ComparisonPrimitiveV::BytesEqual.into ()),
			("bytevector<?", ComparisonPrimitiveV::BytesLesser.into ()),
			("bytevector>?", ComparisonPrimitiveV::BytesGreater.into ()),
			("bytevector<=?", ComparisonPrimitiveV::BytesLesserOrEqual.into ()),
			("bytevector>=?", ComparisonPrimitiveV::BytesGreaterOrEqual.into ()),
			
			("pair=?", ComparisonPrimitiveV::PairEqual.into ()),
			("pair<?", ComparisonPrimitiveV::PairLesser.into ()),
			("pair>?", ComparisonPrimitiveV::PairGreater.into ()),
			("pair<=?", ComparisonPrimitiveV::PairLesserOrEqual.into ()),
			("pair>=?", ComparisonPrimitiveV::PairGreaterOrEqual.into ()),
			
			("vector=?", ComparisonPrimitiveV::ArrayEqual.into ()),
			("vector<?", ComparisonPrimitiveV::ArrayLesser.into ()),
			("vector>?", ComparisonPrimitiveV::ArrayGreater.into ()),
			("vector<=?", ComparisonPrimitiveV::ArrayLesserOrEqual.into ()),
			("vector>=?", ComparisonPrimitiveV::ArrayGreaterOrEqual.into ()),
			
			("values=?", ComparisonPrimitiveV::ValuesEqual.into ()),
			("values<?", ComparisonPrimitiveV::ValuesLesser.into ()),
			("values>?", ComparisonPrimitiveV::ValuesGreater.into ()),
			("values<=?", ComparisonPrimitiveV::ValuesLesserOrEqual.into ()),
			("values>=?", ComparisonPrimitiveV::ValuesGreaterOrEqual.into ()),
			
			("generic=?", ComparisonPrimitiveV::GenericEqual.into ()),
			("generic<?", ComparisonPrimitiveV::GenericLesser.into ()),
			("generic>?", ComparisonPrimitiveV::GenericGreater.into ()),
			("generic<=?", ComparisonPrimitiveV::GenericLesserOrEqual.into ()),
			("generic>=?", ComparisonPrimitiveV::GenericGreaterOrEqual.into ()),
			
		];
	
	let definitions = vec_map_into! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

