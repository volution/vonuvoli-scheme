

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::generate_binding_templates as language_builtins_generate_binding_templates;
	pub use super::generate_definitions as language_builtins_generate_definitions;
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn generate_binding_templates () -> (Outcome<StdVec<BindingTemplate>>) {
	
	let definitions = try! (generate_definitions ());
	
	let templates = vec_map_into! (
			definitions,
			(identifier, value),
			BindingTemplate {
					identifier : Some (identifier),
					value : Some (value),
					immutable : true,
				}
		);
	
	succeed! (templates);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			("locals", SyntaxPrimitiveV::Locals.into ()), // https://docs.racket-lang.org/reference/local.html
			("set!-values", SyntaxPrimitiveV::SetValues.into ()),
			
			("call", FunctionsPrimitiveV::Call.into ()),
			("call-with-values*", FunctionsPrimitive2::CallWithValues.into ()),
			
			("not-null?", TypePrimitive1::IsNullNot.into ()),
			
			("void?", TypePrimitive1::IsVoid.into ()),
			("not-void?", TypePrimitive1::IsVoidNot.into ()),
			("undefined?", TypePrimitive1::IsUndefined.into ()),
			("not-undefined?", TypePrimitive1::IsUndefinedNot.into ()),
			
			("true?", TypePrimitive1::IsTrue.into ()),
			("not-true?", TypePrimitive1::IsTrueNot.into ()),
			("true-or-equivalent?", TypePrimitive1::IsTrueOrEquivalent.into ()),
			("false?", TypePrimitive1::IsFalse.into ()),
			("not-false?", TypePrimitive1::IsFalseNot.into ()),
			("false-or-equivalent?", TypePrimitive1::IsFalseOrEquivalent.into ()),
			
			("and*", BooleanPrimitiveV::And.into ()),
			("or*", BooleanPrimitiveV::Or.into ()),
			("xor*", BooleanPrimitiveV::Xor.into ()),
			("nand*", BooleanPrimitiveV::Nand.into ()),
			("nor*", BooleanPrimitiveV::Nor.into ()),
			("nxor*", BooleanPrimitiveV::Nxor.into ()),
			
			("string-reverse", StringPrimitive1::StringCloneReverse.into ()),
			("string-reverse!", StringPrimitiveV::StringRangeReverse.into ()),
			("string->immutable", StringPrimitive1::StringToImmutable.into ()),
			("string->mutable", StringPrimitive1::StringToMutable.into ()),
			("string-immutable?", TypePrimitive1::IsStringImmutable.into ()),
			("string-mutable?", TypePrimitive1::IsStringMutable.into ()),
			
			("symbol-upcase", StringPrimitive1::SymbolToUpperCase.into ()),
			("symbol-downcase", StringPrimitive1::SymbolToLowerCase.into ()),
			("symbol-foldcase", StringPrimitive1::SymbolToFoldCase.into ()),
			
			("bytevector-reverse", BytesPrimitive1::BytesCloneReverse.into ()),
			("bytevector-reverse!", BytesPrimitiveV::BytesRangeReverse.into ()),
			("bytevector-u8-fill!", BytesPrimitiveV::BytesRangeFill.into ()),
			("bytevector-u8-map", FunctionsPrimitiveV::BytesMap.into ()),
			("bytevector-u8-for-each", FunctionsPrimitiveV::BytesIterate.into ()),
			("bytevector->vector", BytesPrimitiveV::BytesRangeToArray.into ()),
			("vector->bytevector", BytesPrimitiveV::ArrayRangeToBytes.into ()),
			("bytevector->list", BytesPrimitiveV::BytesRangeToList.into ()),
			("list->bytevector", BytesPrimitiveV::ListRangeToBytes.into ()),
			("bytevector->immutable", BytesPrimitive1::BytesToImmutable.into ()),
			("bytevector->mutable", BytesPrimitive1::BytesToMutable.into ()),
			("bytevector-immutable?", TypePrimitive1::IsBytesImmutable.into ()),
			("bytevector-mutable?", TypePrimitive1::IsBytesMutable.into ()),
			
			("pair->immutable", ListPrimitive1::PairToImmutable.into ()),
			("pair->mutable", ListPrimitive1::PairToMutable.into ()),
			("pair-immutable?", TypePrimitive1::IsPairImmutable.into ()),
			("pair-mutable?", TypePrimitive1::IsPairMutable.into ()),
			
			("list->immutable", ListPrimitive1::ListToImmutable.into ()),
			("list->mutable", ListPrimitive1::ListToMutable.into ()),
			
			("vector-reverse", ArrayPrimitive1::ArrayCloneReverse.into ()),
			("vector-reverse!", ArrayPrimitiveV::ArrayRangeReverse.into ()),
			("vector->immutable", ArrayPrimitive1::ArrayToImmutable.into ()),
			("vector->mutable", ArrayPrimitive1::ArrayToMutable.into ()),
			("vector-immutable?", TypePrimitive1::IsArrayImmutable.into ()),
			("vector-mutable?", TypePrimitive1::IsArrayMutable.into ()),
			
			("values?", TypePrimitive1::IsValues.into ()),
			
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
			
			
			("process-spawn", RuntimePrimitiveN::ProcessSpawn.into ()),
			("process-wait-poll", RuntimePrimitive1::ProcessWaitPoll.into ()),
			("process-wait", RuntimePrimitive1::ProcessWaitCheck.into ()),
			("process-wait-try", RuntimePrimitive1::ProcessWaitTry.into ()),
			("process-run", RuntimePrimitiveN::ProcessRunCheck.into ()),
			("process-run-try", RuntimePrimitiveN::ProcessRunTry.into ()),
			
			("process-stdin", RuntimePrimitive1::ProcessStdinGet.into ()),
			("process-stdout", RuntimePrimitive1::ProcessStdoutGet.into ()),
			("process-stderr", RuntimePrimitive1::ProcessStderrGet.into ()),
			
			("process?", TypePrimitive1::IsProcess.into ()),
			("resource?", TypePrimitive1::IsResource.into ()),
			("opaque?", TypePrimitive1::IsOpaque.into ()),
			
		];
	
	let definitions = vec_map_into! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

