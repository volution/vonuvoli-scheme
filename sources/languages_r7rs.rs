

use super::contexts::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::generate_binding_templates as language_r7rs_generate_binding_templates;
	pub use super::generate_definitions as language_r7rs_generate_definitions;
	pub use super::verify_definitions as language_r7rs_verify_definitions;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn generate_binding_templates () -> (Outcome<StdVec<BindingTemplate>>) {
	
	let definitions = try! (generate_definitions ());
	
	let definitions = vec_map_into! (
			definitions,
			(_library, _category, identifier, value),
			(identifier, value));
	
	let definitions =
			definitions
			.into_iter ()
			.collect::<StdMap<_, _>> ();
	
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
pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Symbol, Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			// https://wiki.volution.ro/CiprianDorinCraciun/Notes/Public/R7rs/Identifiers
			// https://bitbucket.org/cowan/r7rs-wg1-infra/raw/default/SmallLanguageIdentifiers.md
			
			
			
			
			// !!!
			
			("base", "syntaxes", "_", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "...", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "=>", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "else", SyntaxPrimitive::Auxiliary.into ()),
			
			("base", "quotation", "quote", SyntaxPrimitiveV::Quote.into ()),
			("base", "quotation", "quasiquote", SyntaxPrimitiveV::QuasiQuote.into ()),
			("base", "quotation", "unquote", SyntaxPrimitiveV::UnQuote.into ()),
			("base", "quotation", "unquote-splicing", SyntaxPrimitiveV::UnQuoteSplicing.into ()),
			
			("base", "control", "begin", SyntaxPrimitiveV::Begin.into ()),
			
			("base", "control", "if", SyntaxPrimitiveV::If.into ()),
			("base", "control", "unless", SyntaxPrimitiveV::Unless.into ()),
			("base", "control", "when", SyntaxPrimitiveV::When.into ()),
			("base", "control", "cond", SyntaxPrimitiveV::Cond.into ()),
			("base", "control", "case", SyntaxPrimitiveV::Case.into ()),
			("base", "control", "do", SyntaxPrimitiveV::Do.into ()),
			
			("base", "control", "and", SyntaxPrimitiveV::And.into ()),
			("base", "control", "or", SyntaxPrimitiveV::Or.into ()),
			
			("base", "lambda", "lambda", SyntaxPrimitiveV::Lambda.into ()),
			
			("base", "contexts", "define", SyntaxPrimitiveV::Define.into ()),
			("base", "values", "define-values", SyntaxPrimitiveV::DefineValues.into ()),
			("base", "syntaxes", "define-syntax", SyntaxPrimitive::Unsupported.into ()),
			("base", "records", "define-record-type", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "contexts", "let", SyntaxPrimitiveV::LetParallel.into ()),
			("base", "contexts", "let*", SyntaxPrimitiveV::LetSequential.into ()),
			("base", "contexts", "letrec", SyntaxPrimitiveV::LetRecursiveParallel.into ()),
			("base", "contexts", "letrec*", SyntaxPrimitiveV::LetRecursiveSequential.into ()),
			("base", "values", "let-values", SyntaxPrimitiveV::LetValuesParallel.into ()),
			("base", "values", "let*-values", SyntaxPrimitiveV::LetValuesSequential.into ()),
			("base", "syntaxes", "let-syntax", SyntaxPrimitive::Unsupported.into ()),
			("base", "syntaxes", "letrec-syntax", SyntaxPrimitive::Unsupported.into ()),
			
			("base", "contexts", "set!", SyntaxPrimitiveV::Set.into ()),
			
			("base", "modules", "import", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "include", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "include-ci", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "cond-expand", SyntaxPrimitive::Unsupported.into ()),
			
			("base", "parameters", "parameterize", SyntaxPrimitive::Unsupported.into ()),
			("base", "parameters", "make-parameter", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "syntaxes", "syntax-error", SyntaxPrimitive::Unsupported.into ()),
			("base", "syntaxes", "syntax-rules", SyntaxPrimitive::Unsupported.into ()),
			
			("base", "evaluator", "guard", SyntaxPrimitive::Unsupported.into ()),
			
			
			
			
			// ???
			
			("base", "modules", "features", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "types", "null?", TypePrimitive1::IsNull.into ()),
			
			
			
			
			// equivalences
			
			("base", "equivalence", "eq?", ComparisonPrimitiveV::EquivalentByIdentity.into ()),
			("base", "equivalence", "eqv?", ComparisonPrimitiveV::EquivalentByValueStrict.into ()),
			("base", "equivalence", "equal?", ComparisonPrimitiveV::EquivalentByValueStrictRecursive.into ()),
			
			
			
			
			// math
			
			("base", "types", "number?", TypePrimitive1::IsNumber.into ()),
			("base", "types", "integer?", TypePrimitive1::IsNumberInteger.into ()),
			("base", "types", "real?", TypePrimitive1::IsNumberReal.into ()),
			("base", "types", "rational?", TypePrimitive1::IsNumberRational.into ()),
			("base", "types", "complex?", TypePrimitive1::IsNumberComplex.into ()),
			("base", "types", "exact?", TypePrimitive1::IsNumberExact.into ()),
			("base", "types", "exact-integer?", TypePrimitive1::IsNumberExactInteger.into ()),
			("base", "types", "inexact?", TypePrimitive1::IsNumberInexact.into ()),
			
			("base", "arithmetic", "zero?", TypePrimitive1::IsNumberZero.into ()),
			("base", "arithmetic", "positive?", TypePrimitive1::IsNumberPositive.into ()),
			("base", "arithmetic", "negative?", TypePrimitive1::IsNumberNegative.into ()),
			("base", "arithmetic", "odd?", TypePrimitive1::IsNumberOdd.into ()),
			("base", "arithmetic", "even?", TypePrimitive1::IsNumberEven.into ()),
			
			("base", "arithmetic", "+", ArithmeticPrimitiveV::Addition.into ()),
			("base", "arithmetic", "-", ArithmeticPrimitiveV::Subtraction.into ()),
			("base", "arithmetic", "*", ArithmeticPrimitiveV::Multiplication.into ()),
			("base", "arithmetic", "/", ArithmeticPrimitiveV::Division.into ()),
			
			("base", "arithmetic", "abs", ArithmeticPrimitive1::Absolute.into ()),
			
			("base", "arithmetic", "quotient", ArithmeticPrimitive2::DivisionTruncateQuotient.into ()),
			("base", "arithmetic", "remainder", ArithmeticPrimitive2::DivisionTruncateRemainder.into ()),
			("base", "arithmetic", "modulo", ArithmeticPrimitive2::DivisionFloorRemainder.into ()),
			
			("base", "arithmetic", "floor", ArithmeticPrimitive1::Floor.into ()),
			("base", "arithmetic", "ceiling", ArithmeticPrimitive1::Ceiling.into ()),
			("base", "arithmetic", "truncate", ArithmeticPrimitive1::Truncate.into ()),
			("base", "arithmetic", "round", ArithmeticPrimitive1::Round.into ()),
			
			("base", "arithmetic", "rationalize", ProcedurePrimitive::Unsupported.into ()),
			("base", "arithmetic", "numerator", ProcedurePrimitive::Unsupported.into ()),
			("base", "arithmetic", "denominator", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "arithmetic", "floor/", ArithmeticPrimitive2::DivisionFloor.into ()),
			("base", "arithmetic", "floor-quotient", ArithmeticPrimitive2::DivisionFloorQuotient.into ()),
			("base", "arithmetic", "floor-remainder", ArithmeticPrimitive2::DivisionFloorRemainder.into ()),
			
			("base", "arithmetic", "truncate/", ArithmeticPrimitive2::DivisionTruncate.into ()),
			("base", "arithmetic", "truncate-quotient", ArithmeticPrimitive2::DivisionTruncateQuotient.into ()),
			("base", "arithmetic", "truncate-remainder", ArithmeticPrimitive2::DivisionTruncateRemainder.into ()),
			
			("base", "arithmetic", "min", ArithmeticPrimitiveV::Minimum.into ()),
			("base", "arithmetic", "max", ArithmeticPrimitiveV::Maximum.into ()),
			
			("base", "arithmetic", "gcd", ArithmeticPrimitiveV::GreatestCommonDivisor.into ()),
			("base", "arithmetic", "lcm", ArithmeticPrimitiveV::LeastCommonMultiple.into ()),
			
			("base", "arithmetic", "expt", ArithmeticPrimitive2::Power.into ()),
			("base", "arithmetic", "square", ArithmeticPrimitive1::Square.into ()),
			("base", "arithmetic", "exact-integer-sqrt", ArithmeticPrimitive1::SquareRootWithRemainder.into ()),
			
			("base", "arithmetic", "=", ComparisonPrimitiveV::NumberEqual.into ()),
			("base", "arithmetic", "<", ComparisonPrimitiveV::NumberLesser.into ()),
			("base", "arithmetic", ">", ComparisonPrimitiveV::NumberGreater.into ()),
			("base", "arithmetic", "<=", ComparisonPrimitiveV::NumberLesserOrEqual.into ()),
			("base", "arithmetic", ">=", ComparisonPrimitiveV::NumberGreaterOrEqual.into ()),
			
			("base", "arithmetic", "inexact", ArithmeticPrimitive1::CoerceToInexact.into ()),
			("base", "arithmetic", "exact", ArithmeticPrimitive1::CoerceToExact.into ()),
			
			
			
			
			// boolean
			
			("base", "types", "boolean?", TypePrimitive1::IsBoolean.into ()),
			
			("base", "equivalence", "boolean=?", ComparisonPrimitiveV::BooleanEqual.into ()),
			
			("base", "equivalence", "not", TypePrimitive1::IsFalse.into ()),
			
			
			
			
			// characters
			
			("base", "types", "char?", TypePrimitive1::IsCharacter.into ()),
			
			("base", "characters", "char=?", ComparisonPrimitiveV::CharacterCaseSensitiveEqual.into ()),
			("base", "characters", "char<?", ComparisonPrimitiveV::CharacterCaseSensitiveLesser.into ()),
			("base", "characters", "char>?", ComparisonPrimitiveV::CharacterCaseSensitiveGreater.into ()),
			("base", "characters", "char<=?", ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual.into ()),
			("base", "characters", "char>=?", ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual.into ()),
			
			
			
			
			// symbols
			
			("base", "types", "symbol?", TypePrimitive1::IsSymbol.into ()),
			
			("base", "equivalence", "symbol=?", ComparisonPrimitiveV::SymbolCaseSensitiveEqual.into ()),
			
			
			
			
			// pairs
			
			("base", "types", "pair?", TypePrimitive1::IsPair.into ()),
			
			("base", "pairs", "cons", ListPrimitive2::Pair.into ()),
			("base", "pairs", "car", ListPrimitive1::PairLeft.into ()),
			("base", "pairs", "cdr", ListPrimitive1::PairRight.into ()),
			
			("base", "pairs", "set-car!", ListPrimitive2::PairLeftSet.into ()),
			("base", "pairs", "set-cdr!", ListPrimitive2::PairRightSet.into ()),
			
			("base", "pairs", "caar", ListPrimitive1::ListFirstOfFirst.into ()),
			("base", "pairs", "cdar", ListPrimitive1::ListRestOfFirst.into ()),
			
			("base", "pairs", "cadr", ListPrimitive1::ListFirstAt2.into ()),
			("base", "pairs", "cddr", ListPrimitive1::ListRestAt2.into ()),
			
			
			
			
			// lists
			
			("base", "types", "list?", TypePrimitive1::IsListProperOrEmpty.into ()),
			
			("base", "lists", "list", ListPrimitiveV::ListBuild.into ()),
			("base", "lists", "make-list", ListPrimitiveV::ListMake.into ()),
			("base", "lists", "list-copy", ListPrimitiveV::ListRangeClone.into ()),
			("base", "lists", "append", ListPrimitiveV::ListAppend.into ()),
			("base", "lists", "length", ListPrimitive1::ListLength.into ()),
			
			("base", "lists", "list-ref", ListPrimitive2::ListFirstAt.into ()),
			("base", "lists", "list-tail", ListPrimitive2::ListPairAt.into ()),
			
			("base", "lists", "list-set!", ListPrimitive3::ListFirstAtSet.into ()),
			
			("base", "lists", "reverse", ListPrimitive1::ListReverse.into ()),
			
			("base", "lists", "memq", ListPrimitive2::ListMemberByIdentity.into ()),
			("base", "lists", "memv", ListPrimitive2::ListMemberByValue.into ()),
			("base", "lists", "member", ListPrimitiveV::ListMember.into ()),
			
			("base", "lists", "assq", ListPrimitive2::ListAssocByIdentity.into ()),
			("base", "lists", "assv", ListPrimitive2::ListAssocByValue.into ()),
			("base", "lists", "assoc", ListPrimitiveV::ListAssoc.into ()),
			
			
			
			
			// vectors
			
			("base", "types", "vector?", TypePrimitive1::IsArray.into ()),
			
			("base", "vectors", "vector", ArrayPrimitiveV::ArrayBuild.into ()),
			("base", "vectors", "make-vector", ArrayPrimitiveV::ArrayMake.into ()),
			("base", "vectors", "vector-copy", ArrayPrimitiveV::ArrayRangeClone.into ()),
			("base", "vectors", "vector-append", ArrayPrimitiveV::ArrayAppend.into ()),
			("base", "vectors", "vector-length", ArrayPrimitive1::ArrayLength.into ()),
			
			("base", "vectors", "vector-ref", ArrayPrimitive2::ArrayAt.into ()),
			
			("base", "vectors", "vector-set!", ArrayPrimitive3::ArrayAtSet.into ()),
			("base", "vectors", "vector-fill!", ArrayPrimitiveV::ArrayRangeFill.into ()),
			("base", "vectors", "vector-copy!", ArrayPrimitiveV::ArrayRangeCopy.into ()),
			
			
			
			
			// bytevectors
			
			("base", "types", "bytevector?", TypePrimitive1::IsBytes.into ()),
			
			("base", "bytes", "bytevector", BytesPrimitiveV::BytesBuild.into ()),
			("base", "bytes", "make-bytevector", BytesPrimitiveV::BytesMake.into ()),
			("base", "bytes", "bytevector-copy", BytesPrimitiveV::BytesRangeClone.into ()),
			("base", "bytes", "bytevector-append", BytesPrimitiveV::BytesAppend.into ()),
			("base", "bytes", "bytevector-length", BytesPrimitive1::BytesLength.into ()),
			
			("base", "bytes", "bytevector-u8-ref", BytesPrimitive2::BytesAt.into ()),
			
			("base", "bytes", "bytevector-u8-set!", BytesPrimitive3::BytesAtSet.into ()),
			("base", "bytes", "bytevector-copy!", BytesPrimitiveV::BytesRangeCopy.into ()),
			
			
			
			
			// strings
			
			("base", "types", "string?", TypePrimitive1::IsString.into ()),
			
			("base", "strings", "string", StringPrimitiveV::StringBuild.into ()),
			("base", "strings", "make-string", StringPrimitiveV::StringMake.into ()),
			("base", "strings", "string-copy", StringPrimitiveV::StringRangeClone.into ()),
			("base", "strings", "string-append", StringPrimitiveV::StringAppend.into ()),
			("base", "strings", "string-length", StringPrimitive1::StringLength.into ()),
			
			("base", "strings", "string-ref", StringPrimitive2::StringAt.into ()),
			
			("base", "strings", "string-set!", StringPrimitive3::StringAtSet.into ()),
			("base", "strings", "string-fill!", StringPrimitiveV::StringRangeFill.into ()),
			("base", "strings", "string-copy!", StringPrimitiveV::StringRangeCopy.into ()),
			
			("base", "strings", "substring", StringPrimitiveV::StringRangeClone.into ()),
			
			("base", "strings", "string=?", ComparisonPrimitiveV::StringCaseSensitiveEqual.into ()),
			("base", "strings", "string<?", ComparisonPrimitiveV::StringCaseSensitiveLesser.into ()),
			("base", "strings", "string>?", ComparisonPrimitiveV::StringCaseSensitiveGreater.into ()),
			("base", "strings", "string<=?", ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual.into ()),
			("base", "strings", "string>=?", ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual.into ()),
			
			
			
			
			// converters to and from strings
			
			("base", "strings", "number->string", StringPrimitiveV::NumberToString.into ()),
			("base", "strings", "string->number", StringPrimitiveV::StringToNumber.into ()),
			
			("base", "strings", "symbol->string", StringPrimitive1::SymbolToString.into ()),
			("base", "strings", "string->symbol", StringPrimitive1::StringToSymbol.into ()),
			
			("base", "strings", "list->string", StringPrimitiveV::ListRangeToString.into ()),
			("base", "strings", "string->list", StringPrimitiveV::StringRangeToList.into ()),
			
			("base", "strings", "utf8->string", StringPrimitiveV::BytesRangeToString.into ()),
			("base", "strings", "string->utf8", StringPrimitiveV::StringRangeToBytes.into ()),
			
			("base", "strings", "vector->string", StringPrimitiveV::ArrayRangeToString.into ()),
			("base", "strings", "string->vector", StringPrimitiveV::StringRangeToArray.into ()),
			
			
			
			
			// converters miscellaneous
			
			("base", "characters", "char->integer", StringPrimitive1::CharacterToNumber.into ()),
			("base", "characters", "integer->char", StringPrimitive1::NumberToCharacter.into ()),
			
			("base", "vectors", "vector->list", ArrayPrimitiveV::ArrayRangeToList.into ()),
			("base", "vectors", "list->vector", ArrayPrimitiveV::ListRangeToArray.into ()),
			
			
			
			
			// ???
			
			("base", "types", "procedure?", TypePrimitive1::IsProcedure.into ()),
			
			("base", "functions", "apply", FunctionsPrimitiveV::Apply.into ()),
			
			("base", "functions", "map", FunctionsPrimitiveV::ListsMap.into ()),
			("base", "functions", "for-each", FunctionsPrimitiveV::ListsIterate.into ()),
			
			("base", "functions", "vector-map", FunctionsPrimitiveV::ArraysMap.into ()),
			("base", "functions", "vector-for-each", FunctionsPrimitiveV::ArraysIterate.into ()),
			
			("base", "functions", "string-map", FunctionsPrimitiveV::StringsMap.into ()),
			("base", "functions", "string-for-each", FunctionsPrimitiveV::StringsIterate.into ()),
			
			("base", "values", "values", FunctionsPrimitiveV::Values.into ()),
			("base", "values", "call-with-values", FunctionsPrimitive2::CallWithValuesBuilder.into ()),
			
			("base", "evaluator", "call-with-current-continuation", ProcedurePrimitive::Unsupported.into ()),
			("base", "evaluator", "call/cc", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "evaluator", "dynamic-wind", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// ???
			
			("base", "evaluator", "raise", ProcedurePrimitive::Unimplemented.into ()),
			("base", "evaluator", "raise-continuable", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "errors", "error", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "error-object?", TypePrimitive1::IsError.into ()),
			("base", "errors", "error-object-message", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "error-object-irritants", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "read-error?", TypePrimitive1::IsErrorPortInput.into ()),
			("base", "errors", "file-error?", TypePrimitive1::IsErrorFile.into ()),
			
			("base", "evaluator", "with-exception-handler", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports
			
			("base", "ports", "call-with-port", PortPrimitive2::CallAndClose.into ()),
			
			("base", "parameters", "current-input-port", ProcedurePrimitive::Unsupported.into ()),
			("base", "parameters", "current-output-port", ProcedurePrimitive::Unsupported.into ()),
			("base", "parameters", "current-error-port", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "ports", "port?", TypePrimitive1::IsPort.into ()),
			("base", "ports", "input-port?", TypePrimitive1::IsPortInput.into ()),
			("base", "ports", "input-port-open?", PortPrimitive1::IsInputOpen.into ()),
			("base", "ports", "output-port?", TypePrimitive1::IsPortOutput.into ()),
			("base", "ports", "output-port-open?", PortPrimitive1::IsOutputOpen.into ()),
			("base", "ports", "binary-port?", TypePrimitive1::IsPortBinary.into ()),
			("base", "ports", "textual-port?", TypePrimitive1::IsPortTextual.into ()),
			
			("base", "ports", "open-input-string", PortPrimitive1::InputFromString.into ()),
			("base", "ports", "open-output-string", PortPrimitive0::OutputToString.into ()),
			("base", "ports", "get-output-string", PortPrimitive1::OutputToStringFinalize.into ()),
			
			("base", "ports", "open-input-bytevector", PortPrimitive1::InputFromBytes.into ()),
			("base", "ports", "open-output-bytevector", PortPrimitive0::OutputToBytes.into ()),
			("base", "ports", "get-output-bytevector", PortPrimitive1::OutputToBytesFinalize.into ()),
			
			("base", "ports", "close-port", PortPrimitive1::Close.into ()),
			("base", "ports", "close-input-port", PortPrimitive1::CloseInput.into ()),
			("base", "ports", "close-output-port", PortPrimitive1::CloseOutput.into ()),
			
			
			
			
			// ports input
			
			("base", "ports", "char-ready?", PortPrimitive1::CharacterReady.into ()),
			("base", "ports", "peek-char", PortPrimitive1::CharacterPeek.into ()),
			("base", "ports", "read-char", PortPrimitive1::CharacterRead.into ()),
			("base", "ports", "read-string", PortPrimitive2::StringReadCollect.into ()),
			
			("base", "ports", "u8-ready?", PortPrimitive1::ByteReady.into ()),
			("base", "ports", "peek-u8", PortPrimitive1::BytePeek.into ()),
			("base", "ports", "read-u8", PortPrimitive1::ByteRead.into ()),
			("base", "ports", "read-bytevector", PortPrimitive2::BytesReadCollect.into ()),
			("base", "ports", "read-bytevector!", PortPrimitive2::BytesReadCopy.into ()),
			
			("base", "ports", "read-line", PortPrimitive1::StringReadLine.into ()),
			
			("base", "ports", "eof-object", PortPrimitive0::Eof.into ()),
			("base", "ports", "eof-object?", TypePrimitive1::IsPortEof.into ()),
			
			
			
			
			// ports output
			
			("base", "ports", "write-char", PortPrimitive2::CharacterWrite.into ()),
			("base", "ports", "write-string", PortPrimitive2::StringWrite.into ()),
			
			("base", "ports", "write-u8", PortPrimitive2::ByteWrite.into ()),
			("base", "ports", "write-bytevector", PortPrimitive2::BytesWrite.into ()),
			
			("base", "ports", "newline", PortPrimitive0::NewLine.into ()),
			
			("base", "ports", "flush-output-port", PortPrimitive1::FlushOutput.into ()),
			
			
			
			
			// (scheme case-lambda)
			//     --> verified
			
			("case-lambda", "lambda", "case-lambda", SyntaxPrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme char)
			//     --> verified
			
			("char", "strings", "string-upcase", StringPrimitive1::StringToUpperCase.into ()),
			("char", "strings", "string-downcase", StringPrimitive1::StringToLowerCase.into ()),
			("char", "strings", "string-foldcase", StringPrimitive1::StringToFoldCase.into ()),
			
			("char", "strings", "string-ci=?", ComparisonPrimitiveV::StringCaseInsensitiveEqual.into ()),
			("char", "strings", "string-ci<?", ComparisonPrimitiveV::StringCaseInsensitiveLesser.into ()),
			("char", "strings", "string-ci>?", ComparisonPrimitiveV::StringCaseInsensitiveGreater.into ()),
			("char", "strings", "string-ci<=?", ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual.into ()),
			("char", "strings", "string-ci>=?", ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual.into ()),
			
			("char", "characters", "char-alphabetic?", TypePrimitive1::IsCharacterAlphabetic.into ()),
			("char", "characters", "char-upper-case?", TypePrimitive1::IsCharacterAlphabeticUpperCase.into ()),
			("char", "characters", "char-lower-case?", TypePrimitive1::IsCharacterAlphabeticLowerCase.into ()),
			("char", "characters", "char-numeric?", TypePrimitive1::IsCharacterNumeric.into ()),
			("char", "characters", "char-whitespace?", TypePrimitive1::IsCharacterWhitespace.into ()),
			
			("char", "characters", "char-upcase", StringPrimitive1::CharacterToUpperCase.into ()),
			("char", "characters", "char-downcase", StringPrimitive1::CharacterToLowerCase.into ()),
			("char", "characters", "char-foldcase", StringPrimitive1::CharacterToFoldCase.into ()),
			
			("char", "characters", "char-ci=?", ComparisonPrimitiveV::CharacterCaseInsensitiveEqual.into ()),
			("char", "characters", "char-ci<?", ComparisonPrimitiveV::CharacterCaseInsensitiveLesser.into ()),
			("char", "characters", "char-ci>?", ComparisonPrimitiveV::CharacterCaseInsensitiveGreater.into ()),
			("char", "characters", "char-ci<=?", ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual.into ()),
			("char", "characters", "char-ci>=?", ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual.into ()),
			
			("char", "characters", "digit-value", StringPrimitiveV::CharacterToDigitNumber.into ()),
			
			
			
			
			// (scheme complex)
			//     --> verified
			
			("complex", "arithmetic", "make-rectangular", ProcedurePrimitive::Unsupported.into ()),
			("complex", "arithmetic", "real-part", ProcedurePrimitive::Unsupported.into ()),
			("complex", "arithmetic", "imag-part", ProcedurePrimitive::Unsupported.into ()),
			
			("complex", "arithmetic", "make-polar", ProcedurePrimitive::Unsupported.into ()),
			("complex", "arithmetic", "magnitude", ProcedurePrimitive::Unsupported.into ()),
			("complex", "arithmetic", "angle", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme cxr)
			//     --> verified
			
			("cxr", "pairs", "caaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "caadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "caddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cdaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cdadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cdddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "caaaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "caaadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "caadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "caaddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cadaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cadadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "caddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cadddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cdaaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cdaadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cdadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cdaddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cddaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cddadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			("cxr", "pairs", "cdddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			("cxr", "pairs", "cddddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			
			
			
			// (scheme eval)
			//     --> verified
			
			("eval", "evaluator", "environment", ProcedurePrimitive::Unsupported.into ()),
			("eval", "evaluator", "eval", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme file)
			//     --> verified
			
			("file", "ports", "open-input-file", PortPrimitive1::OpenTextualInput.into ()),
			("file", "ports", "open-binary-input-file", PortPrimitive1::OpenBinaryInput.into ()),
			
			("file", "ports", "open-output-file", PortPrimitive1::OpenTextualOutput.into ()),
			("file", "ports", "open-binary-output-file", PortPrimitive1::OpenBinaryOutput.into ()),
			
			("file", "ports", "call-with-input-file", PortPrimitive2::OpenTextualInputThenCallAndClose.into ()),
			("file", "ports", "call-with-output-file", PortPrimitive2::OpenTextualOutputThenCallAndClose.into ()),
			
			("file", "parameters", "with-input-from-file", ProcedurePrimitive::Unsupported.into ()),
			("file", "parameters", "with-output-to-file", ProcedurePrimitive::Unsupported.into ()),
			
			("file", "system", "file-exists?", PortPrimitive1::FileExists.into ()),
			("file", "system", "delete-file", PortPrimitive1::FileDelete.into ()),
			
			
			
			
			// (scheme inexact)
			//     --> verified
			
			("inexact", "arithmetic", "sqrt", ArithmeticPrimitive1::SquareRoot.into ()),
			("inexact", "arithmetic", "exp", ArithmeticPrimitive1::Exponential.into ()),
			("inexact", "arithmetic", "log", ArithmeticPrimitive1::Logarithm.into ()),
			
			("inexact", "arithmetic", "sin", ArithmeticPrimitive1::Sin.into ()),
			("inexact", "arithmetic", "cos", ArithmeticPrimitive1::Cos.into ()),
			("inexact", "arithmetic", "tan", ArithmeticPrimitive1::Tan.into ()),
			
			("inexact", "arithmetic", "asin", ArithmeticPrimitive1::Asin.into ()),
			("inexact", "arithmetic", "acos", ArithmeticPrimitive1::Acos.into ()),
			("inexact", "arithmetic", "atan", ArithmeticPrimitive1::Atan.into ()),
			
			("inexact", "arithmetic", "finite?", TypePrimitive1::IsNumberFinite.into ()),
			("inexact", "arithmetic", "infinite?", TypePrimitive1::IsNumberInfinite.into ()),
			("inexact", "arithmetic", "nan?", TypePrimitive1::IsNumberNan.into ()),
			
			
			
			
			// (scheme lazy)
			//     --> verified
			
			("lazy", "promises", "delay", SyntaxPrimitive::Unsupported.into ()),
			("lazy", "promises", "delay-force", SyntaxPrimitive::Unsupported.into ()),
			
			("lazy", "promises", "promise?", ProcedurePrimitive::Unsupported.into ()),
			("lazy", "promises", "make-promise", ProcedurePrimitive::Unsupported.into ()),
			("lazy", "promises", "force", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme load)
			//     --> verified
			
			("load", "modules", "load", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme process-context)
			//     --> verified
			
			("process-context", "system", "command-line", RuntimePrimitive0::ProcessArguments.into ()),
			("process-context", "system", "get-environment-variable", RuntimePrimitive1::ProcessEnvironment.into ()),
			("process-context", "system", "get-environment-variables", RuntimePrimitive0::ProcessEnvironment.into ()),
			
			("process-context", "system", "exit", RuntimePrimitiveV::ProcessExit.into ()),
			("process-context", "system", "emergency-exit", RuntimePrimitiveV::ProcessExitEmergency.into ()),
			
			
			
			
			// (scheme r5rs)
			//     --> verified
			
			("r5rs", "evaluator", "interaction-environment", ProcedurePrimitive::Unsupported.into ()),
			("r5rs", "evaluator", "scheme-report-environment", ProcedurePrimitive::Unsupported.into ()),
			("r5rs", "evaluator", "null-environment", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme read)
			//     --> verified
			
			("read", "ports", "read", PortPrimitive1::ValueRead.into ()),
			
			
			
			
			// (scheme repl)
			//     --> verified
			
			("repl", "evaluator", "interaction-environment", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme time)
			//     --> verified
			
			("time", "system", "current-second", RuntimePrimitive0::PosixTimestamp.into ()),
			("time", "system", "current-jiffy", RuntimePrimitive0::JiffiesTimestamp.into ()),
			("time", "system", "jiffies-per-second", RuntimePrimitive0::JiffiesPerSecond.into ()),
			
			
			
			
			// (scheme write)
			//     --> verified
			
			("write", "ports", "write", PortPrimitive2::ValueWrite.into ()),
			("write", "ports", "write-shared", PortPrimitive2::ValueWriteShared.into ()),
			("write", "ports", "write-simple", PortPrimitive2::ValueWriteSimple.into ()),
			("write", "ports", "display", PortPrimitive1::ValueDisplay.into ()),
			
			
			
			
		];
	
	let definitions = vec_map_into! (
			definitions,
			(library, category, identifier, value),
			(Symbol::from (library), Symbol::from (category), Symbol::from (identifier), value));
	
	succeed! (definitions);
}




#[ inline (never) ]
pub fn verify_definitions (definitions : &StdVec<(Symbol, Symbol, Symbol, Value)>) -> (Outcome<()>) {
	
	
	let mut libraries = vec! [
			"base",
			"case-lambda",
			"char",
			"complex",
			"cxr",
			"eval",
			"file",
			"inexact",
			"lazy",
			"load",
			"process-context",
			"r5rs",
			"read",
			"repl",
			"time",
			"write",
		]
		.into_iter ()
		.map (|library| (Symbol::from (library), 0))
		.collect::<StdMap<_, _>> ();
	
	
	let mut categories = vec! [
			"arithmetic",
			"bytes",
			"characters",
			"contexts",
			"control",
			"equivalence",
			"errors",
			"evaluator",
			"functions",
			"lambda",
			"lists",
			"modules",
			"pairs",
			"parameters",
			"ports",
			"promises",
			"quotation",
			"records",
			"strings",
			"syntaxes",
			"system",
			"types",
			"values",
			"vectors",
		]
		.into_iter ()
		.map (|category| (Symbol::from (category), 0))
		.collect::<StdMap<_, _>> ();
	
	
	let mut mappings = StdMap::new ();
	let mut errors = false;
	
	
	for &(_, _, ref identifier, ref value) in definitions {
		if let Some (existing) = mappings.insert (identifier.clone (), value) {
			if existing != value {
				eprintln! ("[ee]  duplicate missmatched mapping for `{}`!", identifier.string_as_str ());
				errors = true;
			}
		}
	}
	
	for &(ref library, ref category, _, _) in definitions {
		if let Some (count) = libraries.get_mut (library) {
			*count += 1;
		} else {
			eprintln! ("[ee]  unknown library `{}`!", library.string_as_str ());
			errors = true;
		}
		if let Some (count) = categories.get_mut (category) {
			*count += 1;
		} else {
			eprintln! ("[ee]  unknown category `{}`!", category.string_as_str ());
			errors = true;
		}
	}
	
	for (library, count) in libraries {
		if count == 0 {
			eprintln! ("[ee]  unused library `{}`!", library.string_as_str ());
			errors = true;
		}
	}
	for (category, count) in categories {
		if count == 0 {
			eprintln! ("[ee]  unused category `{}`!", category.string_as_str ());
			errors = true;
		}
	}
	
	
	if !errors {
		succeed! (());
	} else {
		fail! (0x24ce2821);
	}
	
}

