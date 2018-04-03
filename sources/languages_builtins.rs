

use super::builtins::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::parameters::exports::*;
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
	
	let mut definitions = StdVec::new ();
	
	// NOTE:  R7RS extensions (natural extensions to existing constructs)
	definitions.extend_from_slice (&[
			
			("locals", SyntaxPrimitiveV::Locals.into ()),
			("set!-values", SyntaxPrimitiveV::SetValues.into ()),
			
			("loop", SyntaxPrimitiveV::Loop.into ()),
			
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
			
			("write-line", PortPrimitiveV::ValueWriteAndNewLine.into ()),
			("display-line", PortPrimitiveV::ValueDisplayAndNewLine.into ()),
			
			("parameter?", TypePrimitiveV::IsParameter.into ()),
			("parameter-ref", RuntimePrimitiveV::ParameterResolve.into ()),
			("parameter-set!", RuntimePrimitiveV::ParameterConfigure.into ()),
			
			("command-line-ref", RuntimePrimitive1::ProcessArgument.into ()),
			("command-line->vector", RuntimePrimitive0::ProcessArgumentsAsArray.into ()),
			("get-environment-variables->vector", RuntimePrimitive0::ProcessEnvironmentVariablesAsArray.into ()),
			
		]);
	
	// NOTE:  value extensions
	definitions.extend_from_slice (&[
			
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
			
			("resource?", TypePrimitiveV::IsResource.into ()),
			("opaque?", TypePrimitiveV::IsOpaque.into ()),
			
		]);
	
	// NOTE:  file-system operations
	definitions.extend_from_slice (&[
			
			("fs-canonicalize", FileSystemPrimitive1::PathCanonicalize.into ()),
			("fs-link-resolve", FileSystemPrimitiveV::SymLinkResolve.into ()),
			
			("fs-directory-list", FileSystemPrimitiveV::DirectoryListAsList.into ()),
			("fs-directory-list->vector", FileSystemPrimitiveV::DirectoryListAsArray.into ()),
			
			("fs-metadata", FileSystemPrimitiveV::MetadataResolve.into ()),
			("fs-metadata-eq?", FileSystemPrimitiveV::MetadataIsSelf.into ()),
			
			("fs-metadata-kind", FileSystemPrimitiveV::MetadataKindGet.into ()),
			("fs-metadata-kind?", FileSystemPrimitive2::MetadataKindHas.into ()),
			("fs-metadata-file?", FileSystemPrimitive1::MetadataKindIsFile.into ()),
			("fs-metadata-directory?", FileSystemPrimitive1::MetadataKindIsDirectory.into ()),
			("fs-metadata-symlink?", FileSystemPrimitive1::MetadataKindIsSymLink.into ()),
			("fs-metadata-fifo?", FileSystemPrimitive1::MetadataKindIsFifo.into ()),
			("fs-metadata-socket?", FileSystemPrimitive1::MetadataKindIsSocket.into ()),
			("fs-metadata-block-device?", FileSystemPrimitive1::MetadataKindIsBlockDevice.into ()),
			("fs-metadata-character-device?", FileSystemPrimitive1::MetadataKindIsCharacterDevice.into ()),
			
			("fs-metadata-file-size", FileSystemPrimitiveV::MetadataFileGetSize.into ()),
			("fs-metadata-file-empty?", FileSystemPrimitiveV::MetadataFileIsEmpty.into ()),
			("fs-metadata-file-not-empty?", FileSystemPrimitiveV::MetadataFileIsEmptyNot.into ()),
			
			("fs-metadata-readonly?", FileSystemPrimitiveV::MetadataIsReadonly.into ()),
			("fs-metadata-readable?", FileSystemPrimitiveV::MetadataIsReadable.into ()),
			("fs-metadata-writeable?", FileSystemPrimitiveV::MetadataIsWriteable.into ()),
			("fs-metadata-file-executable?", FileSystemPrimitiveV::MetadataFileIsExecutable.into ()),
			("fs-metadata-directory-traversable?", FileSystemPrimitiveV::MetadataDirectoryIsTraversable.into ()),
			
			("fs-metadata-unix-mode", FileSystemPrimitiveV::MetadataUnixGetMode.into ()),
			("fs-metadata-unix-type", FileSystemPrimitiveV::MetadataUnixGetType.into ()),
			("fs-metadata-unix-permissions", FileSystemPrimitiveV::MetadataUnixGetPermissions.into ()),
			("fs-metadata-unix-user-identifier", FileSystemPrimitiveV::MetadataUnixGetUserIdentifier.into ()),
			("fs-metadata-unix-group-identifier", FileSystemPrimitiveV::MetadataUnixGetGroupIdentifier.into ()),
			("fs-metadata-unix-data-accessed-at", FileSystemPrimitiveV::MetadataUnixGetDataAccessedAt.into ()),
			("fs-metadata-unix-data-modified-at", FileSystemPrimitiveV::MetadataUnixGetDataModifiedAt.into ()),
			("fs-metadata-unix-inode-changed-at", FileSystemPrimitiveV::MetadataUnixGetInodeChangedAt.into ()),
			("fs-metadata-unix-inode-device", FileSystemPrimitiveV::MetadataUnixGetInodeDevice.into ()),
			("fs-metadata-unix-inode-number", FileSystemPrimitiveV::MetadataUnixGetInodeNumber.into ()),
			("fs-metadata-unix-inode-links", FileSystemPrimitiveV::MetadataUnixGetInodeLinks.into ()),
			
			("fs-file-exists?", FileSystemPrimitiveV::FileExists.into ()),
			("fs-file-delete", FileSystemPrimitive1::FileDelete.into ()),
			("fs-directory-exists?", FileSystemPrimitiveV::DirectoryExists.into ()),
			("fs-directory-delete", FileSystemPrimitive1::DirectoryDelete.into ()),
			("fs-symlink-exists?", FileSystemPrimitive1::SymLinkExists.into ()),
			("fs-mount?", FileSystemPrimitiveV::MountPointIs.into ()),
			
		]);
	
	// NOTE:  file-system paths
	definitions.extend_from_slice (&[
			
			("path?", TypePrimitiveV::IsPath.into ()),
			
			("path", FileSystemPrimitive1::PathCoerce.into ()),
			
			("path-absolute?", TypePrimitiveV::IsPathAbsolute.into ()),
			("path-relative?", TypePrimitiveV::IsPathRelative.into ()),
			
			("path-parent", FileSystemPrimitive1::PathParent.into ()),
			("path-join", FileSystemPrimitiveV::PathJoin.into ()),
			("path-split", FileSystemPrimitiveV::PathSplit.into ()),
			("path-prefix?", FileSystemPrimitive2::PathHasPrefix.into ()),
			("path-suffix?", FileSystemPrimitive2::PathHasSuffix.into ()),
			
			("path-name", FileSystemPrimitive1::PathName.into ()),
			("path-name-without-extension", FileSystemPrimitive1::PathNameWithoutExtension.into ()),
			("path-name-only-extension", FileSystemPrimitive1::PathNameOnlyExtension.into ()),
			("path-name-join", FileSystemPrimitiveV::PathNameJoin.into ()),
			("path-name-split", FileSystemPrimitiveV::PathNameSplit.into ()),
			("path-name?", FileSystemPrimitive2::PathNameIs.into ()),
			("path-name-prefix?", FileSystemPrimitive2::PathNameHasPrefix.into ()),
			("path-name-suffix?", FileSystemPrimitive2::PathNameHasSuffix.into ()),
			
			("path->string", FileSystemPrimitive1::PathToString.into ()),
			("string->path", FileSystemPrimitive1::StringToPath.into ()),
			("path->bytevector", FileSystemPrimitive1::PathToBytes.into ()),
			("bytevector->path", FileSystemPrimitive1::BytesToPath.into ()),
			
			("path=?", ComparisonPrimitiveV::PathEqual.into ()),
			("path<?", ComparisonPrimitiveV::PathLesser.into ()),
			("path>?", ComparisonPrimitiveV::PathGreater.into ()),
			("path<=?", ComparisonPrimitiveV::PathLesserOrEqual.into ()),
			("path>=?", ComparisonPrimitiveV::PathGreaterOrEqual.into ()),
			
		]);
	
	// NOTE:  sub-processes
	definitions.extend_from_slice (&[
			
			("process?", TypePrimitiveV::IsProcess.into ()),
			
			("process-spawn", RuntimePrimitiveN::ProcessSpawn.into ()),
			("process-spawn*", RuntimePrimitiveV::ProcessSpawnExtended.into ()),
			("process-wait-poll", RuntimePrimitive1::ProcessWaitPoll.into ()),
			("process-wait", RuntimePrimitive1::ProcessWaitCheck.into ()),
			("process-wait-try", RuntimePrimitive1::ProcessWaitTry.into ()),
			("process-run", RuntimePrimitiveN::ProcessRunCheck.into ()),
			("process-run-try", RuntimePrimitiveN::ProcessRunTry.into ()),
			
			("process-spawn:env-empty", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-empty"), PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE, false) .into ()),
			("process-spawn:env-include", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-include"), PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE, false) .into ()),
			("process-spawn:env-exclude", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-exclude"), PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE, false) .into ()),
			("process-spawn:directory", Parameter::for_builtin (symbol_clone_str ("process-spawn:working-directory"), PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE, false) .into ()),
			("process-spawn:stdin", Parameter::for_builtin (symbol_clone_str ("process-spawn:stdin"), PROCESS_PARAMETER_STDIN_HANDLE_VALUE, false) .into ()),
			("process-spawn:stdout", Parameter::for_builtin (symbol_clone_str ("process-spawn:stdout"), PROCESS_PARAMETER_STDOUT_HANDLE_VALUE, false) .into ()),
			("process-spawn:stderr", Parameter::for_builtin (symbol_clone_str ("process-spawn:stderr"), PROCESS_PARAMETER_STDERR_HANDLE_VALUE, false) .into ()),
			
			("process-stdin", RuntimePrimitive1::ProcessStdinGet.into ()),
			("process-stdout", RuntimePrimitive1::ProcessStdoutGet.into ()),
			("process-stderr", RuntimePrimitive1::ProcessStderrGet.into ()),
			
		]);
	
	// NOTE:  transcripts
	definitions.extend_from_slice (&[
			
			("trace-critical", RuntimePrimitiveV::TranscriptTraceCritical.into ()),
			("trace-error", RuntimePrimitiveV::TranscriptTraceError.into ()),
			("trace-warning", RuntimePrimitiveV::TranscriptTraceWarning.into ()),
			("trace-notice", RuntimePrimitiveV::TranscriptTraceNotice.into ()),
			("trace-information", RuntimePrimitiveV::TranscriptTraceInformation.into ()),
			("trace-internal", RuntimePrimitiveV::TranscriptTraceInternal.into ()),
			("trace-debugging", RuntimePrimitiveV::TranscriptTraceDebugging.into ()),
			
		]);
	
	// NOTE:  SRFI-1
	definitions.extend_from_slice (&[
			
			("first", ListPrimitive1::ListFirstAt1.into ()),
			("first-tail", ListPrimitive1::ListRestAt1.into ()),
			("first-pair", ListPrimitive1::ListPairAt1.into ()),
			("second", ListPrimitive1::ListFirstAt2.into ()),
			("second-tail", ListPrimitive1::ListRestAt2.into ()),
			("second-pair", ListPrimitive1::ListPairAt2.into ()),
			("third", ListPrimitive1::ListFirstAt3.into ()),
			("third-tail", ListPrimitive1::ListRestAt3.into ()),
			("third-pair", ListPrimitive1::ListPairAt3.into ()),
			("fourth", ListPrimitive1::ListFirstAt4.into ()),
			("fourth-tail", ListPrimitive1::ListRestAt4.into ()),
			("fourth-pair", ListPrimitive1::ListPairAt4.into ()),
			("fifth", ListPrimitive1::ListFirstAt5.into ()),
			("fifth-tail", ListPrimitive1::ListRestAt5.into ()),
			("fifth-pair", ListPrimitive1::ListPairAt5.into ()),
			("sixth", ListPrimitive1::ListFirstAt6.into ()),
			("sixth-tail", ListPrimitive1::ListRestAt6.into ()),
			("sixth-pair", ListPrimitive1::ListPairAt6.into ()),
			("seventh", ListPrimitive1::ListFirstAt7.into ()),
			("seventh-tail", ListPrimitive1::ListRestAt7.into ()),
			("seventh-pair", ListPrimitive1::ListPairAt7.into ()),
			("eighth", ListPrimitive1::ListFirstAt8.into ()),
			("eighth-tail", ListPrimitive1::ListRestAt8.into ()),
			("eighth-pair", ListPrimitive1::ListPairAt8.into ()),
			("ninth", ListPrimitive1::ListFirstAt9.into ()),
			("ninth-tail", ListPrimitive1::ListRestAt9.into ()),
			("ninth-pair", ListPrimitive1::ListPairAt9.into ()),
			("tenth", ListPrimitive1::ListFirstAt10.into ()),
			("tenth-tail", ListPrimitive1::ListRestAt10.into ()),
			("tenth-pair", ListPrimitive1::ListPairAt10.into ()),
			
			("find", ListPrimitive2::ListFind.into ()),
			("xcons", ListPrimitive2::PairExchanged.into ()),
			("map-in-order", FunctionsPrimitive2::ListsMap.into ()),
			
			// NOTE:  SRFI-1 unimplemented procedures!
			("cons*", ProcedurePrimitive::Unimplemented.into ()),
			("list-tabulate", ProcedurePrimitive::Unimplemented.into ()),
			("circular-list", ProcedurePrimitive::Unimplemented.into ()),
			("iota", ProcedurePrimitive::Unimplemented.into ()),
			("proper-list?", ProcedurePrimitive::Unimplemented.into ()),
			("circular-list?", ProcedurePrimitive::Unimplemented.into ()),
			("dotted-list?", ProcedurePrimitive::Unimplemented.into ()),
			("null-list?", ProcedurePrimitive::Unimplemented.into ()),
			("not-pair?", ProcedurePrimitive::Unimplemented.into ()),
			("list<=", ProcedurePrimitive::Unimplemented.into ()),
			("list<", ProcedurePrimitive::Unimplemented.into ()),
			("list=", ProcedurePrimitive::Unimplemented.into ()),
			("list>", ProcedurePrimitive::Unimplemented.into ()),
			("list>=", ProcedurePrimitive::Unimplemented.into ()),
			("take", ProcedurePrimitive::Unimplemented.into ()),
			("drop", ProcedurePrimitive::Unimplemented.into ()),
			("take-right", ProcedurePrimitive::Unimplemented.into ()),
			("drop-right", ProcedurePrimitive::Unimplemented.into ()),
			("split-at", ProcedurePrimitive::Unimplemented.into ()),
			("last", ProcedurePrimitive::Unimplemented.into ()),
			("last-pair", ProcedurePrimitive::Unimplemented.into ()),
			("concatenate", ProcedurePrimitive::Unimplemented.into ()),
			("append-reverse", ProcedurePrimitive::Unimplemented.into ()),
			("zip", ProcedurePrimitive::Unimplemented.into ()),
			("unzip1", ProcedurePrimitive::Unimplemented.into ()),
			("unzip2", ProcedurePrimitive::Unimplemented.into ()),
			("unzip3", ProcedurePrimitive::Unimplemented.into ()),
			("unzip4", ProcedurePrimitive::Unimplemented.into ()),
			("unzip5", ProcedurePrimitive::Unimplemented.into ()),
			("unzip*", ProcedurePrimitive::Unimplemented.into ()),
			("count", ProcedurePrimitive::Unimplemented.into ()),
			("fold", ProcedurePrimitive::Unimplemented.into ()),
			("fold-right", ProcedurePrimitive::Unimplemented.into ()),
			("pair-fold", ProcedurePrimitive::Unimplemented.into ()),
			("pair-fold-right", ProcedurePrimitive::Unimplemented.into ()),
			("reduce", ProcedurePrimitive::Unimplemented.into ()),
			("reduce-right", ProcedurePrimitive::Unimplemented.into ()),
			("unfold", ProcedurePrimitive::Unimplemented.into ()),
			("unfold-right", ProcedurePrimitive::Unimplemented.into ()),
			("append-map", ProcedurePrimitive::Unimplemented.into ()),
			("pair-for-each", ProcedurePrimitive::Unimplemented.into ()),
			("filter-map", ProcedurePrimitive::Unimplemented.into ()),
			("partition", ProcedurePrimitive::Unimplemented.into ()),
			("remove", ProcedurePrimitive::Unimplemented.into ()),
			("find-tail", ProcedurePrimitive::Unimplemented.into ()),
			("find-pair", ProcedurePrimitive::Unimplemented.into ()),
			("take-while", ProcedurePrimitive::Unimplemented.into ()),
			("drop-while", ProcedurePrimitive::Unimplemented.into ()),
			("span", ProcedurePrimitive::Unimplemented.into ()),
			("break", ProcedurePrimitive::Unimplemented.into ()),
			("any", ProcedurePrimitive::Unimplemented.into ()),
			("every", ProcedurePrimitive::Unimplemented.into ()),
			("list-index", ProcedurePrimitive::Unimplemented.into ()),
			("delete", ProcedurePrimitive::Unimplemented.into ()),
			("delete-duplicates", ProcedurePrimitive::Unimplemented.into ()),
			("alist-cons", ProcedurePrimitive::Unimplemented.into ()),
			("alist-copy", ProcedurePrimitive::Unimplemented.into ()),
			("alist-delete", ProcedurePrimitive::Unimplemented.into ()),
			
			// NOTE:  SRFI-1 unsupported miscellaneous procedures!
			("car+cdr", ProcedurePrimitive::Unsupported.into ()),
			("length+", ProcedurePrimitive::Unsupported.into ()),
			
			// NOTE:  SRFI-1 unsupported linear-update procedures!
			("take!", ProcedurePrimitive::Unsupported.into ()),
			("take-right!", ProcedurePrimitive::Unsupported.into ()),
			("drop-right!", ProcedurePrimitive::Unsupported.into ()),
			("split-at!", ProcedurePrimitive::Unsupported.into ()),
			("append!", ProcedurePrimitive::Unsupported.into ()),
			("concatenate!", ProcedurePrimitive::Unsupported.into ()),
			("reverse!", ProcedurePrimitive::Unsupported.into ()),
			("append-reverse!", ProcedurePrimitive::Unsupported.into ()),
			("map!", ProcedurePrimitive::Unsupported.into ()),
			("append-map!", ProcedurePrimitive::Unsupported.into ()),
			("filter!", ProcedurePrimitive::Unsupported.into ()),
			("partition!", ProcedurePrimitive::Unsupported.into ()),
			("remove!", ProcedurePrimitive::Unsupported.into ()),
			("take-while!", ProcedurePrimitive::Unsupported.into ()),
			("drop-while!", ProcedurePrimitive::Unsupported.into ()),
			("span!", ProcedurePrimitive::Unsupported.into ()),
			("break!", ProcedurePrimitive::Unsupported.into ()),
			("delete!", ProcedurePrimitive::Unsupported.into ()),
			("delete-duplicates!", ProcedurePrimitive::Unsupported.into ()),
			("alist-delete!", ProcedurePrimitive::Unsupported.into ()),
			
			// NOTE:  SRFI-1 unsupported list-set procedures!
			("lset<=", ProcedurePrimitive::Unsupported.into ()),
			("lset<", ProcedurePrimitive::Unsupported.into ()),
			("lset=", ProcedurePrimitive::Unsupported.into ()),
			("lset>", ProcedurePrimitive::Unsupported.into ()),
			("lset>=", ProcedurePrimitive::Unsupported.into ()),
			("lset-adjoin", ProcedurePrimitive::Unsupported.into ()),
			("lset-union", ProcedurePrimitive::Unsupported.into ()),
			("lset-intersection", ProcedurePrimitive::Unsupported.into ()),
			("lset-difference", ProcedurePrimitive::Unsupported.into ()),
			("lset-xor", ProcedurePrimitive::Unsupported.into ()),
			("lset-diff+intersection", ProcedurePrimitive::Unsupported.into ()),
			
			// NOTE:  SRFI-1 unsupported list-set linear-update procedures!
			("lset-union!", ProcedurePrimitive::Unsupported.into ()),
			("lset-intersection!", ProcedurePrimitive::Unsupported.into ()),
			("lset-difference!", ProcedurePrimitive::Unsupported.into ()),
			("lset-xor!", ProcedurePrimitive::Unsupported.into ()),
			("lset-diff+intersection!", ProcedurePrimitive::Unsupported.into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
	definitions.extend_from_slice (&[
			
			("crypto-bytevector", procedure_native_1 (crypto_generate_bytes_build) .into ()),
			("crypto-bytevector-append!", procedure_native_2 (crypto_generate_bytes_extend) .into ()),
			("crypto-bytevector-fill!", procedure_native_v (crypto_generate_bytes_fill_v) .into ()),
			
			("crypto-md5", procedure_native_1 (crypto_hash_md5) .into ()),
			("crypto-sha1", procedure_native_1 (crypto_hash_sha1) .into ()),
			("crypto-sha2-224", procedure_native_1 (crypto_hash_sha2_256_224) .into ()),
			("crypto-sha2-256", procedure_native_1 (crypto_hash_sha2_256) .into ()),
			("crypto-sha2-256-224", procedure_native_1 (crypto_hash_sha2_256_224) .into ()),
			("crypto-sha2-384", procedure_native_1 (crypto_hash_sha2_512_384) .into ()),
			("crypto-sha2-512", procedure_native_1 (crypto_hash_sha2_512) .into ()),
			("crypto-sha2-512-224", procedure_native_1 (crypto_hash_sha2_512_224) .into ()),
			("crypto-sha2-512-256", procedure_native_1 (crypto_hash_sha2_512_256) .into ()),
			("crypto-sha2-512-384", procedure_native_1 (crypto_hash_sha2_512_384) .into ()),
			("crypto-sha3-224", procedure_native_1 (crypto_hash_sha3_224) .into ()),
			("crypto-sha3-256", procedure_native_1 (crypto_hash_sha3_256) .into ()),
			("crypto-sha3-384", procedure_native_1 (crypto_hash_sha3_384) .into ()),
			("crypto-sha3-512", procedure_native_1 (crypto_hash_sha3_512) .into ()),
			("crypto-blake2b-64", procedure_native_1 (crypto_hash_blake2b_64) .into ()),
			("crypto-blake2b-128", procedure_native_1 (crypto_hash_blake2b_128) .into ()),
			("crypto-blake2b-192", procedure_native_1 (crypto_hash_blake2b_192) .into ()),
			("crypto-blake2b-224", procedure_native_1 (crypto_hash_blake2b_224) .into ()),
			("crypto-blake2b-256", procedure_native_1 (crypto_hash_blake2b_256) .into ()),
			("crypto-blake2b-320", procedure_native_1 (crypto_hash_blake2b_320) .into ()),
			("crypto-blake2b-384", procedure_native_1 (crypto_hash_blake2b_384) .into ()),
			("crypto-blake2b-448", procedure_native_1 (crypto_hash_blake2b_448) .into ()),
			("crypto-blake2b-512", procedure_native_1 (crypto_hash_blake2b_512) .into ()),
			("crypto-blake2s-64", procedure_native_1 (crypto_hash_blake2s_64) .into ()),
			("crypto-blake2s-128", procedure_native_1 (crypto_hash_blake2s_128) .into ()),
			("crypto-blake2s-192", procedure_native_1 (crypto_hash_blake2s_192) .into ()),
			("crypto-blake2s-224", procedure_native_1 (crypto_hash_blake2s_224) .into ()),
			("crypto-blake2s-256", procedure_native_1 (crypto_hash_blake2s_256) .into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
	definitions.extend_from_slice (&[
			
			("random-boolean", procedure_native_0 (random_generate_boolean) .into ()),
			("random-boolean-weighted", procedure_native_1 (random_generate_boolean_weighted) .into ()),
			
			("random-i64", procedure_native_0 (random_generate_i64_0) .into ()),
			("random-i64*", procedure_native_v (random_generate_i64_v) .into ()),
			("random-f64", procedure_native_0 (random_generate_f64_0) .into ()),
			("random-f64*", procedure_native_v (random_generate_f64_v) .into ()),
			
			("random-u8", procedure_native_0 (random_generate_u8) .into ()),
			("random-i8", procedure_native_0 (random_generate_i8) .into ()),
			("random-u16", procedure_native_0 (random_generate_u16) .into ()),
			("random-i16", procedure_native_0 (random_generate_i16) .into ()),
			("random-u32", procedure_native_0 (random_generate_u32) .into ()),
			("random-i32", procedure_native_0 (random_generate_i32) .into ()),
			
			("random-u7", procedure_native_0 (random_generate_u7) .into ()),
			("random-u15", procedure_native_0 (random_generate_u15) .into ()),
			("random-u31", procedure_native_0 (random_generate_u31) .into ()),
			("random-u63", procedure_native_0 (random_generate_u63) .into ()),
			
			("random-u1", procedure_native_0 (random_generate_u1) .into ()),
			("random-u2", procedure_native_0 (random_generate_u2) .into ()),
			("random-u3", procedure_native_0 (random_generate_u3) .into ()),
			("random-u4", procedure_native_0 (random_generate_u4) .into ()),
			("random-u5", procedure_native_0 (random_generate_u5) .into ()),
			("random-u6", procedure_native_0 (random_generate_u6) .into ()),
			
			("random-bytevector", procedure_native_1 (random_generate_bytes_build) .into ()),
			("random-bytevector-permutation", procedure_native_0 (random_generate_bytes_permutation) .into ()),
			("random-bytevector-append!", procedure_native_2 (random_generate_bytes_extend) .into ()),
			("random-bytevector-fill!", procedure_native_v (random_generate_bytes_fill_v) .into ()),
			("random-bytevector-shuffle!", procedure_native_v (random_generate_bytes_shuffle_v) .into ()),
			
			("random-char", procedure_native_0 (random_generate_character_0) .into ()),
			("random-char*", procedure_native_v (random_generate_character_v) .into ()),
			
			("random-char-ascii", procedure_native_0 (random_generate_character_ascii) .into ()),
			("random-char-ascii-numeric", procedure_native_0 (random_generate_character_ascii_numeric) .into ()),
			("random-char-ascii-numeric-8", procedure_native_0 (random_generate_character_ascii_numeric_base_8) .into ()),
			("random-char-ascii-numeric-16", procedure_native_0 (random_generate_character_ascii_numeric_base_16) .into ()),
			("random-char-ascii-alphabetic", procedure_native_0 (random_generate_character_ascii_alphabetic) .into ()),
			("random-char-ascii-upper-case", procedure_native_0 (random_generate_character_ascii_alphabetic_upper_case) .into ()),
			("random-char-ascii-lower-case", procedure_native_0 (random_generate_character_ascii_alphabetic_lower_case) .into ()),
			("random-char-ascii-alphabetic-or-numeric", procedure_native_0 (random_generate_character_ascii_alphabetic_or_numeric) .into ()),
			("random-char-ascii-whitespace", procedure_native_0 (random_generate_character_ascii_whitespace) .into ()),
			("random-char-ascii-control", procedure_native_0 (random_generate_character_ascii_control) .into ()),
			("random-char-ascii-punctuation", procedure_native_0 (random_generate_character_ascii_punctuation) .into ()),
			("random-char-ascii-graphic", procedure_native_0 (random_generate_character_ascii_graphic) .into ()),
			
			("random-string-ascii", procedure_native_1 (random_generate_string_build_ascii) .into ()),
			("random-string-ascii-numeric", procedure_native_1 (random_generate_string_build_ascii_numeric) .into ()),
			("random-string-ascii-numeric-8", procedure_native_1 (random_generate_string_build_ascii_numeric_base_8) .into ()),
			("random-string-ascii-numeric-16", procedure_native_1 (random_generate_string_build_ascii_numeric_base_16) .into ()),
			("random-string-ascii-alphabetic", procedure_native_1 (random_generate_string_build_ascii_alphabetic) .into ()),
			("random-string-ascii-upper-case", procedure_native_1 (random_generate_string_build_ascii_alphabetic_upper_case) .into ()),
			("random-string-ascii-lower-case", procedure_native_1 (random_generate_string_build_ascii_alphabetic_lower_case) .into ()),
			("random-string-ascii-alphabetic-or-numeric", procedure_native_1 (random_generate_string_build_ascii_alphabetic_or_numeric) .into ()),
			("random-string-ascii-whitespace", procedure_native_1 (random_generate_string_build_ascii_whitespace) .into ()),
			("random-string-ascii-control", procedure_native_1 (random_generate_string_build_ascii_control) .into ()),
			("random-string-ascii-punctuation", procedure_native_1 (random_generate_string_build_ascii_punctuation) .into ()),
			("random-string-ascii-graphic", procedure_native_1 (random_generate_string_build_ascii_graphic) .into ()),
			
			("random-string-ascii-permutation", procedure_native_0 (random_generate_string_permutation_ascii) .into ()),
			("random-string-ascii-numeric-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric) .into ()),
			("random-string-ascii-numeric-8-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric_base_8) .into ()),
			("random-string-ascii-numeric-16-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric_base_16) .into ()),
			("random-string-ascii-alphabetic-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic) .into ()),
			("random-string-ascii-upper-case-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_upper_case) .into ()),
			("random-string-ascii-lower-case-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_lower_case) .into ()),
			("random-string-ascii-alphabetic-or-numeric-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_or_numeric) .into ()),
			("random-string-ascii-whitespace-permutation", procedure_native_0 (random_generate_string_permutation_ascii_whitespace) .into ()),
			("random-string-ascii-control-permutation", procedure_native_0 (random_generate_string_permutation_ascii_control) .into ()),
			("random-string-ascii-punctuation-permutation", procedure_native_0 (random_generate_string_permutation_ascii_punctuation) .into ()),
			("random-string-ascii-graphic-permutation", procedure_native_0 (random_generate_string_permutation_ascii_graphic) .into ()),
			
			("random-string-ascii-append!", procedure_native_2 (random_generate_string_extend_ascii) .into ()),
			("random-string-ascii-numeric-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric) .into ()),
			("random-string-ascii-numeric-8-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric_base_8) .into ()),
			("random-string-ascii-numeric-16-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric_base_16) .into ()),
			("random-string-ascii-alphabetic-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic) .into ()),
			("random-string-ascii-upper-case-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_upper_case) .into ()),
			("random-string-ascii-lower-case-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_lower_case) .into ()),
			("random-string-ascii-alphabetic-or-numeric-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_or_numeric) .into ()),
			("random-string-ascii-whitespace-append!", procedure_native_2 (random_generate_string_extend_ascii_whitespace) .into ()),
			("random-string-ascii-control-append!", procedure_native_2 (random_generate_string_extend_ascii_control) .into ()),
			("random-string-ascii-punctuation-append!", procedure_native_2 (random_generate_string_extend_ascii_punctuation) .into ()),
			("random-string-ascii-graphic-append!", procedure_native_2 (random_generate_string_extend_ascii_graphic) .into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
	definitions.extend_from_slice (&[
			
			("hex-lower-encode", procedure_native_1 (encode_hex_lower_build) .into ()),
			("hex-lower-encode-append!", procedure_native_2 (encode_hex_lower_extend) .into ()),
			("hex-lower-encode-fill!", procedure_native_2 (encode_hex_lower_fill) .into ()),
			("hex-upper-encode", procedure_native_1 (encode_hex_upper_build) .into ()),
			("hex-upper-encode-append!", procedure_native_2 (encode_hex_upper_extend) .into ()),
			("hex-upper-encode-fill!", procedure_native_2 (encode_hex_upper_fill) .into ()),
			
			("hex-encode", procedure_native_1 (encode_hex_lower_build) .into ()),
			("hex-encode-append!", procedure_native_2 (encode_hex_lower_extend) .into ()),
			("hex-encode-fill!", procedure_native_2 (encode_hex_lower_fill) .into ()),
			("hex-decode", procedure_native_1 (decode_hex_build) .into ()),
			("hex-decode-append!", procedure_native_2 (decode_hex_extend) .into ()),
			("hex-decode-fill!", procedure_native_2 (decode_hex_fill) .into ()),
			
			("base32-encode", procedure_native_1 (encode_base32_build) .into ()),
			("base32-encode-append!", procedure_native_2 (encode_base32_extend) .into ()),
			("base32-encode-fill!", procedure_native_2 (encode_base32_fill) .into ()),
			("base32-decode", procedure_native_1 (decode_base32_build) .into ()),
			("base32-decode-append!", procedure_native_2 (decode_base32_extend) .into ()),
			("base32-decode-fill!", procedure_native_2 (decode_base32_fill) .into ()),
			
			("base32-nopad-encode", procedure_native_1 (encode_base32_nopad_build) .into ()),
			("base32-nopad-encode-append!", procedure_native_2 (encode_base32_nopad_extend) .into ()),
			("base32-nopad-encode-fill!", procedure_native_2 (encode_base32_nopad_fill) .into ()),
			("base32-nopad-decode", procedure_native_1 (decode_base32_nopad_build) .into ()),
			("base32-nopad-decode-append!", procedure_native_2 (decode_base32_nopad_extend) .into ()),
			("base32-nopad-decode-fill!", procedure_native_2 (decode_base32_nopad_fill) .into ()),
			
			("base32-hex-encode", procedure_native_1 (encode_base32_hex_build) .into ()),
			("base32-hex-encode-append!", procedure_native_2 (encode_base32_hex_extend) .into ()),
			("base32-hex-encode-fill!", procedure_native_2 (encode_base32_hex_fill) .into ()),
			("base32-hex-decode", procedure_native_1 (decode_base32_hex_build) .into ()),
			("base32-hex-decode-append!", procedure_native_2 (decode_base32_hex_extend) .into ()),
			("base32-hex-decode-fill!", procedure_native_2 (decode_base32_hex_fill) .into ()),
			
			("base32-hex-nopad-encode", procedure_native_1 (encode_base32_hex_nopad_build) .into ()),
			("base32-hex-nopad-encode-append!", procedure_native_2 (encode_base32_hex_nopad_extend) .into ()),
			("base32-hex-nopad-encode-fill!", procedure_native_2 (encode_base32_hex_nopad_fill) .into ()),
			("base32-hex-nopad-decode", procedure_native_1 (decode_base32_hex_nopad_build) .into ()),
			("base32-hex-nopad-decode-append!", procedure_native_2 (decode_base32_hex_nopad_extend) .into ()),
			("base32-hex-nopad-decode-fill!", procedure_native_2 (decode_base32_hex_nopad_fill) .into ()),
			
			("base64-encode", procedure_native_1 (encode_base64_build) .into ()),
			("base64-encode-append!", procedure_native_2 (encode_base64_extend) .into ()),
			("base64-encode-fill!", procedure_native_2 (encode_base64_fill) .into ()),
			("base64-decode", procedure_native_1 (decode_base64_build) .into ()),
			("base64-decode-append!", procedure_native_2 (decode_base64_extend) .into ()),
			("base64-decode-fill!", procedure_native_2 (decode_base64_fill) .into ()),
			
			("base64-nopad-encode", procedure_native_1 (encode_base64_nopad_build) .into ()),
			("base64-nopad-encode-append!", procedure_native_2 (encode_base64_nopad_extend) .into ()),
			("base64-nopad-encode-fill!", procedure_native_2 (encode_base64_nopad_fill) .into ()),
			("base64-nopad-decode", procedure_native_1 (decode_base64_nopad_build) .into ()),
			("base64-nopad-decode-append!", procedure_native_2 (decode_base64_nopad_extend) .into ()),
			("base64-nopad-decode-fill!", procedure_native_2 (decode_base64_nopad_fill) .into ()),
			
			("base64-url-encode", procedure_native_1 (encode_base64_url_build) .into ()),
			("base64-url-encode-append!", procedure_native_2 (encode_base64_url_extend) .into ()),
			("base64-url-encode-fill!", procedure_native_2 (encode_base64_url_fill) .into ()),
			("base64-url-decode", procedure_native_1 (decode_base64_url_build) .into ()),
			("base64-url-decode-append!", procedure_native_2 (decode_base64_url_extend) .into ()),
			("base64-url-decode-fill!", procedure_native_2 (decode_base64_url_fill) .into ()),
			
			("base64-url-nopad-encode", procedure_native_1 (encode_base64_url_nopad_build) .into ()),
			("base64-url-nopad-encode-append!", procedure_native_2 (encode_base64_url_nopad_extend) .into ()),
			("base64-url-nopad-encode-fill!", procedure_native_2 (encode_base64_url_nopad_fill) .into ()),
			("base64-url-nopad-decode", procedure_native_1 (decode_base64_url_nopad_build) .into ()),
			("base64-url-nopad-decode-append!", procedure_native_2 (decode_base64_url_nopad_extend) .into ()),
			("base64-url-nopad-decode-fill!", procedure_native_2 (decode_base64_url_nopad_fill) .into ()),
			
			("base64-mime-encode", procedure_native_1 (encode_base64_mime_build) .into ()),
			("base64-mime-encode-append!", procedure_native_2 (encode_base64_mime_extend) .into ()),
			("base64-mime-encode-fill!", procedure_native_2 (encode_base64_mime_fill) .into ()),
			("base64-mime-decode", procedure_native_1 (decode_base64_mime_build) .into ()),
			("base64-mime-decode-append!", procedure_native_2 (decode_base64_mime_extend) .into ()),
			("base64-mime-decode-fill!", procedure_native_2 (decode_base64_mime_fill) .into ()),
			
		]);
	
	let definitions = vec_map_into! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

