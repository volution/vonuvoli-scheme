

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::generate_binding_templates as language_builtins_generate_binding_templates;
	pub use super::generate_definitions as language_builtins_generate_definitions;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			("locals", SyntaxPrimitiveV::Locals.into ()),
			("set!-values", SyntaxPrimitiveV::SetValues.into ()),
			
			("guard*", SyntaxPrimitiveV::Guard.into ()),
			
			("call", FunctionsPrimitiveV::Call.into ()),
			("call-with-values*", FunctionsPrimitive2::CallWithValues.into ()),
			
			("make-error", RuntimePrimitiveV::ErrorBuild.into ()),
			("error-object-irritants->vector", RuntimePrimitive1::ErrorArgumentsAsArray.into ()),
			("error-object-irritants->values", RuntimePrimitive1::ErrorArgumentsAsValues.into ()),
			
			("not-null?", TypePrimitiveV::IsNullNot.into ()),
			
			("void?", TypePrimitiveV::IsVoid.into ()),
			("not-void?", TypePrimitiveV::IsVoidNot.into ()),
			("undefined?", TypePrimitiveV::IsUndefined.into ()),
			("not-undefined?", TypePrimitiveV::IsUndefinedNot.into ()),
			
			("true?", TypePrimitiveV::IsTrue.into ()),
			("not-true?", TypePrimitiveV::IsTrueNot.into ()),
			("true-or-equivalent?", TypePrimitiveV::IsTrueOrEquivalent.into ()),
			("false?", TypePrimitiveV::IsFalse.into ()),
			("not-false?", TypePrimitiveV::IsFalseNot.into ()),
			("false-or-equivalent?", TypePrimitiveV::IsFalseOrEquivalent.into ()),
			
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
			("string-immutable?", TypePrimitiveV::IsStringImmutable.into ()),
			("string-mutable?", TypePrimitiveV::IsStringMutable.into ()),
			
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
			("bytevector-immutable?", TypePrimitiveV::IsBytesImmutable.into ()),
			("bytevector-mutable?", TypePrimitiveV::IsBytesMutable.into ()),
			
			("pair->immutable", ListPrimitive1::PairToImmutable.into ()),
			("pair->mutable", ListPrimitive1::PairToMutable.into ()),
			("pair-immutable?", TypePrimitiveV::IsPairImmutable.into ()),
			("pair-mutable?", TypePrimitiveV::IsPairMutable.into ()),
			
			("list->immutable", ListPrimitive1::ListToImmutable.into ()),
			("list->mutable", ListPrimitive1::ListToMutable.into ()),
			
			("vector-reverse", ArrayPrimitive1::ArrayCloneReverse.into ()),
			("vector-reverse!", ArrayPrimitiveV::ArrayRangeReverse.into ()),
			("vector->immutable", ArrayPrimitive1::ArrayToImmutable.into ()),
			("vector->mutable", ArrayPrimitive1::ArrayToMutable.into ()),
			("vector-immutable?", TypePrimitiveV::IsArrayImmutable.into ()),
			("vector-mutable?", TypePrimitiveV::IsArrayMutable.into ()),
			
			("values?", TypePrimitiveV::IsValues.into ()),
			
			("record-type?", TypePrimitiveV::IsRecordKind.into ()),
			("record-type", RecordPrimitive1::RecordKindGet.into ()),
			("record-type-predicate", RecordPrimitiveV::RecordKindIsFn.into ()),
			("record-type-constructor", RecordPrimitiveV::RecordBuildFn.into ()),
			("record-type-accessor", RecordPrimitiveV::RecordGetFn.into ()),
			("record-type-mutator", RecordPrimitiveV::RecordSetFn.into ()),
			("record-type-identifier", RecordPrimitive1::RecordKindIdentifier.into ()),
			("record-type-size", RecordPrimitive1::RecordKindSize.into ()),
			("make-record-type", RecordPrimitiveV::RecordKindBuild.into ()),
			
			("record?", RecordPrimitiveV::RecordKindIs.into ()),
			("record-immutable?", TypePrimitiveV::IsRecordImmutable.into ()),
			("record-mutable?", TypePrimitiveV::IsRecordMutable.into ()),
			("make-record", RecordPrimitiveV::RecordBuild.into ()),
			("record-ref", RecordPrimitiveV::RecordGet.into ()),
			("record-set!", RecordPrimitiveV::RecordSet.into ()),
			("record->immutable", RecordPrimitive1::RecordToImmutable.into ()),
			("record->mutable", RecordPrimitive1::RecordToMutable.into ()),
			("record->array", RecordPrimitiveV::RecordToArray.into ()),
			("array->record", RecordPrimitiveV::RecordFromArray.into ()),
			("record->values", RecordPrimitiveV::RecordToValues.into ()),
			("values->record", RecordPrimitiveV::RecordFromValues.into ()),
			("record->list", RecordPrimitiveV::RecordToList.into ()),
			("list->record", RecordPrimitiveV::RecordFromList.into ()),
			
			("record=?", ComparisonPrimitiveV::RecordEqual.into ()),
			("record<?", ComparisonPrimitiveV::RecordLesser.into ()),
			("record>?", ComparisonPrimitiveV::RecordGreater.into ()),
			("record<=?", ComparisonPrimitiveV::RecordLesserOrEqual.into ()),
			("record>=?", ComparisonPrimitiveV::RecordGreaterOrEqual.into ()),
			
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
			
			("keyword=?", ComparisonPrimitiveV::KeywordCaseSensitiveEqual.into ()),
			("keyword<?", ComparisonPrimitiveV::KeywordCaseSensitiveLesser.into ()),
			("keyword>?", ComparisonPrimitiveV::KeywordCaseSensitiveGreater.into ()),
			("keyword<=?", ComparisonPrimitiveV::KeywordCaseSensitiveLesserOrEqual.into ()),
			("keyword>=?", ComparisonPrimitiveV::KeywordCaseSensitiveGreaterOrEqual.into ()),
			
			("keyword-ci=?", ComparisonPrimitiveV::KeywordCaseInsensitiveEqual.into ()),
			("keyword-ci<?", ComparisonPrimitiveV::KeywordCaseInsensitiveLesser.into ()),
			("keyword-ci>?", ComparisonPrimitiveV::KeywordCaseInsensitiveGreater.into ()),
			("keyword-ci<=?", ComparisonPrimitiveV::KeywordCaseInsensitiveLesserOrEqual.into ()),
			("keyword-ci>=?", ComparisonPrimitiveV::KeywordCaseInsensitiveGreaterOrEqual.into ()),
			
			("unique=?", ComparisonPrimitiveV::UniqueEqual.into ()),
			("unique<?", ComparisonPrimitiveV::UniqueLesser.into ()),
			("unique>?", ComparisonPrimitiveV::UniqueGreater.into ()),
			("unique<=?", ComparisonPrimitiveV::UniqueLesserOrEqual.into ()),
			("unique>=?", ComparisonPrimitiveV::UniqueGreaterOrEqual.into ()),
			
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
			
			
			("read-bytevector-chunk", PortPrimitiveV::BytesReadChunk.into ()),
			("read-bytevector-line", PortPrimitiveV::BytesReadLine.into ()),
			("read-bytevector-append!", PortPrimitiveV::BytesReadExtend.into ()),
			
			("read-string-chunk", PortPrimitiveV::StringReadChunk.into ()),
			("read-string-line", PortPrimitiveV::StringReadLine.into ()),
			("read-string-append!", PortPrimitiveV::StringReadExtend.into ()),
			
			
			("parameter?", TypePrimitiveV::IsParameter.into ()),
			("parameter-ref", RuntimePrimitiveV::ParameterResolve.into ()),
			("parameter-set!", RuntimePrimitiveV::ParameterConfigure.into ()),
			
			
			("process-spawn", RuntimePrimitiveN::ProcessSpawn.into ()),
			("process-spawn*", RuntimePrimitiveV::ProcessSpawnExtended.into ()),
			("process-wait-poll", RuntimePrimitive1::ProcessWaitPoll.into ()),
			("process-wait", RuntimePrimitive1::ProcessWaitCheck.into ()),
			("process-wait-try", RuntimePrimitive1::ProcessWaitTry.into ()),
			("process-run", RuntimePrimitiveN::ProcessRunCheck.into ()),
			("process-run-try", RuntimePrimitiveN::ProcessRunTry.into ()),
			
			("process-stdin", RuntimePrimitive1::ProcessStdinGet.into ()),
			("process-stdout", RuntimePrimitive1::ProcessStdoutGet.into ()),
			("process-stderr", RuntimePrimitive1::ProcessStderrGet.into ()),
			
			("process?", TypePrimitiveV::IsProcess.into ()),
			("resource?", TypePrimitiveV::IsResource.into ()),
			("opaque?", TypePrimitiveV::IsOpaque.into ()),
			
		];
	
	let definitions = vec_map_into! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

