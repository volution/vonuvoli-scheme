

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::generate_binding_templates as language_r7rs_generate_binding_templates;
	pub use super::generate_definitions as language_r7rs_generate_definitions;
	pub use super::verify_definitions as language_r7rs_verify_definitions;
}




#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




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
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "control", "case", SyntaxPrimitiveV::Case.into ()),
			("base", "control", "do", SyntaxPrimitiveV::Do.into ()),
			
			("base", "control", "and", SyntaxPrimitiveV::And.into ()),
			("base", "control", "or", SyntaxPrimitiveV::Or.into ()),
			
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			("base", "lambda", "lambda", SyntaxPrimitiveV::Lambda.into ()),
			
			("base", "contexts", "define", SyntaxPrimitiveV::Define.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("base", "values", "define-values", SyntaxPrimitiveV::DefineValues.into ()),
			("base", "syntaxes", "define-syntax", SyntaxPrimitive::Unsupported.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("base", "records", "define-record-type", SyntaxPrimitiveV::DefineRecord.into ()),
			
			("base", "contexts", "let", SyntaxPrimitiveV::LetParallel.into ()),
			("base", "contexts", "let*", SyntaxPrimitiveV::LetSequential.into ()),
			("base", "contexts", "letrec", SyntaxPrimitiveV::LetRecursiveParallel.into ()),
			("base", "contexts", "letrec*", SyntaxPrimitiveV::LetRecursiveSequential.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("base", "values", "let-values", SyntaxPrimitiveV::LetValuesParallel.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("base", "values", "let*-values", SyntaxPrimitiveV::LetValuesSequential.into ()),
			("base", "syntaxes", "let-syntax", SyntaxPrimitive::Unsupported.into ()),
			("base", "syntaxes", "letrec-syntax", SyntaxPrimitive::Unsupported.into ()),
			
			("base", "contexts", "set!", SyntaxPrimitiveV::Set.into ()),
			
			("base", "modules", "import", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "include", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "include-ci", SyntaxPrimitive::Unsupported.into ()),
			("base", "modules", "cond-expand", SyntaxPrimitive::Unsupported.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("base", "parameters", "parameterize", SyntaxPrimitiveV::LetParameters.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("base", "parameters", "make-parameter", RuntimePrimitiveV::ParameterBuild.into ()),
			
			("base", "syntaxes", "syntax-error", SyntaxPrimitive::Unsupported.into ()),
			("base", "syntaxes", "syntax-rules", SyntaxPrimitive::Unsupported.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "evaluator", "guard", SyntaxPrimitiveV::GuardCond.into ()),
			
			
			
			
			// ???
			
			("base", "modules", "features", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "types", "null?", TypePrimitiveV::IsNull.into ()),
			
			
			
			
			// equivalences
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "equivalence", "eq?", ComparisonPrimitiveV::EquivalentByIdentity.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "equivalence", "eqv?", ComparisonPrimitiveV::EquivalentByValueStrict.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "equivalence", "equal?", ComparisonPrimitiveV::EquivalentByValueStrictRecursive.into ()),
			
			
			
			
			// math
			
			("base", "types", "number?", TypePrimitiveV::IsNumber.into ()),
			("base", "types", "integer?", TypePrimitiveV::IsNumberInteger.into ()),
			("base", "types", "real?", TypePrimitiveV::IsNumberReal.into ()),
			("base", "types", "rational?", TypePrimitiveV::IsNumberRational.into ()),
			("base", "types", "complex?", TypePrimitiveV::IsNumberComplex.into ()),
			("base", "types", "exact?", TypePrimitiveV::IsNumberExact.into ()),
			("base", "types", "exact-integer?", TypePrimitiveV::IsNumberExactInteger.into ()),
			("base", "types", "inexact?", TypePrimitiveV::IsNumberInexact.into ()),
			
			("base", "arithmetic", "zero?", TypePrimitiveV::IsNumberZero.into ()),
			("base", "arithmetic", "positive?", TypePrimitiveV::IsNumberPositive.into ()),
			("base", "arithmetic", "negative?", TypePrimitiveV::IsNumberNegative.into ()),
			("base", "arithmetic", "odd?", TypePrimitiveV::IsNumberOdd.into ()),
			("base", "arithmetic", "even?", TypePrimitiveV::IsNumberEven.into ()),
			
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
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
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
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "arithmetic", "=", ComparisonPrimitiveV::NumberEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "arithmetic", "<", ComparisonPrimitiveV::NumberLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "arithmetic", ">", ComparisonPrimitiveV::NumberGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "arithmetic", "<=", ComparisonPrimitiveV::NumberLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "arithmetic", ">=", ComparisonPrimitiveV::NumberGreaterOrEqual.into ()),
			
			("base", "arithmetic", "inexact", ArithmeticPrimitive1::CoerceToInexact.into ()),
			("base", "arithmetic", "exact", ArithmeticPrimitive1::CoerceToExact.into ()),
			
			
			
			
			// boolean
			
			("base", "types", "boolean?", TypePrimitiveV::IsBoolean.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "equivalence", "boolean=?", ComparisonPrimitiveV::BooleanEqual.into ()),
			
			("base", "equivalence", "not", TypePrimitive1::IsFalse.into ()),
			
			
			
			
			// characters
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "types", "char?", TypePrimitiveV::IsCharacter.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char=?", ComparisonPrimitiveV::CharacterCaseSensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char<?", ComparisonPrimitiveV::CharacterCaseSensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char>?", ComparisonPrimitiveV::CharacterCaseSensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char<=?", ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char>=?", ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual.into ()),
			
			
			
			
			// symbols
			
			("base", "types", "symbol?", TypePrimitiveV::IsSymbol.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "equivalence", "symbol=?", ComparisonPrimitiveV::SymbolCaseSensitiveEqual.into ()),
			
			
			
			
			// pairs
			
			("base", "types", "pair?", TypePrimitiveV::IsPair.into ()),
			
			("base", "pairs", "cons", ListPrimitive2::Pair.into ()),
			("base", "pairs", "car", ListPrimitive1::PairLeft.into ()),
			("base", "pairs", "cdr", ListPrimitive1::PairRight.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "pairs", "set-car!", ListPrimitive2::PairLeftSet.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "pairs", "set-cdr!", ListPrimitive2::PairRightSet.into ()),
			
			("base", "pairs", "caar", ListPrimitive1::ListFirstOfFirst.into ()),
			("base", "pairs", "cdar", ListPrimitive1::ListRestOfFirst.into ()),
			
			("base", "pairs", "cadr", ListPrimitive1::ListFirstAt2.into ()),
			("base", "pairs", "cddr", ListPrimitive1::ListRestAt2.into ()),
			
			
			
			
			// lists
			
			("base", "types", "list?", TypePrimitiveV::IsListProperOrEmpty.into ()),
			
			("base", "lists", "list", ListPrimitiveV::ListBuild.into ()),
			("base", "lists", "make-list", ListPrimitiveV::ListMake.into ()),
			("base", "lists", "list-copy", ListPrimitiveV::ListRangeClone.into ()),
			("base", "lists", "append", ListPrimitiveV::ListAppend.into ()),
			("base", "lists", "length", ListPrimitive1::ListLength.into ()),
			
			("base", "lists", "list-ref", ListPrimitive2::ListFirstAt.into ()),
			("base", "lists", "list-tail", ListPrimitive2::ListPairAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "lists", "list-set!", ListPrimitive3::ListFirstAtSet.into ()),
			
			("base", "lists", "reverse", ListPrimitive1::ListCloneReverse.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "lists", "memq", ListPrimitive2::ListMemberByIdentity.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "lists", "memv", ListPrimitive2::ListMemberByValue.into ()),
			("base", "lists", "member", ListPrimitiveV::ListMember.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "lists", "assq", ListPrimitive2::ListAssocByIdentity.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("base", "lists", "assv", ListPrimitive2::ListAssocByValue.into ()),
			("base", "lists", "assoc", ListPrimitiveV::ListAssoc.into ()),
			
			
			
			
			// vectors
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "types", "vector?", TypePrimitiveV::IsArray.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector", ArrayPrimitiveV::ArrayBuild.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "make-vector", ArrayPrimitiveV::ArrayMake.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector-copy", ArrayPrimitiveV::ArrayRangeClone.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector-append", ArrayPrimitiveV::ArrayAppend.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector-length", ArrayPrimitive1::ArrayLength.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector-ref", ArrayPrimitive2::ArrayAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "vectors", "vector-set!", ArrayPrimitive3::ArrayAtSet.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "vectors", "vector-fill!", ArrayPrimitiveV::ArrayRangeFill.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "vectors", "vector-copy!", ArrayPrimitiveV::ArrayRangeCopy.into ()),
			
			
			
			
			// bytevectors
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "types", "bytevector?", TypePrimitiveV::IsBytes.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "bytevector", BytesPrimitiveV::BytesBuild.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "make-bytevector", BytesPrimitiveV::BytesMake.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "bytevector-copy", BytesPrimitiveV::BytesRangeClone.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "bytevector-append", BytesPrimitiveV::BytesAppend.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "bytevector-length", BytesPrimitive1::BytesLength.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "bytes", "bytevector-u8-ref", BytesPrimitive2::BytesAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "bytes", "bytevector-u8-set!", BytesPrimitive3::BytesAtSet.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "bytes", "bytevector-copy!", BytesPrimitiveV::BytesRangeCopy.into ()),
			
			
			
			
			// strings
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "types", "string?", TypePrimitiveV::IsString.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string", StringPrimitiveV::StringBuild.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "make-string", StringPrimitiveV::StringMake.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string-copy", StringPrimitiveV::StringRangeClone.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string-append", StringPrimitiveV::StringAppend.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string-length", StringPrimitive1::StringLength.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string-ref", StringPrimitive2::StringAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "strings", "string-set!", StringPrimitive3::StringAtSet.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "strings", "string-fill!", StringPrimitiveV::StringRangeFill.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "strings", "string-copy!", StringPrimitiveV::StringRangeCopy.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "substring", StringPrimitiveV::StringRangeClone.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string=?", ComparisonPrimitiveV::StringCaseSensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string<?", ComparisonPrimitiveV::StringCaseSensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string>?", ComparisonPrimitiveV::StringCaseSensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string<=?", ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string>=?", ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual.into ()),
			
			
			
			
			// converters to and from strings
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "number->string", StringPrimitiveV::NumberToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string->number", StringPrimitiveV::StringToNumber.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "symbol->string", StringPrimitive1::SymbolToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string->symbol", StringPrimitive1::StringToSymbol.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "list->string", StringPrimitiveV::ListRangeToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "strings", "string->list", StringPrimitiveV::StringRangeToList.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "strings", "utf8->string", StringPrimitiveV::BytesRangeToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "strings", "string->utf8", StringPrimitiveV::StringRangeToBytes.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "strings", "vector->string", StringPrimitiveV::ArrayRangeToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "strings", "string->vector", StringPrimitiveV::StringRangeToArray.into ()),
			
			
			
			
			// converters miscellaneous
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "char->integer", StringPrimitive1::CharacterToNumber.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "characters", "integer->char", StringPrimitive1::NumberToCharacter.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "vector->list", ArrayPrimitiveV::ArrayRangeToList.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "vectors", "list->vector", ArrayPrimitiveV::ListRangeToArray.into ()),
			
			
			
			
			// ???
			
			("base", "types", "procedure?", TypePrimitiveV::IsProcedure.into ()),
			
			("base", "functions", "apply", FunctionsPrimitiveV::Apply.into ()),
			
			("base", "functions", "map", FunctionsPrimitiveV::ListsMap.into ()),
			("base", "functions", "for-each", FunctionsPrimitiveV::ListsIterate.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "functions", "vector-map", FunctionsPrimitiveV::ArraysMap.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("base", "functions", "vector-for-each", FunctionsPrimitiveV::ArraysIterate.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "functions", "string-map", FunctionsPrimitiveV::StringsMap.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "functions", "string-for-each", FunctionsPrimitiveV::StringsIterate.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("base", "values", "values", FunctionsPrimitiveV::Values.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("base", "values", "call-with-values", FunctionsPrimitive2::CallWithValuesBuilder.into ()),
			
			("base", "evaluator", "call-with-current-continuation", ProcedurePrimitive::Unsupported.into ()),
			("base", "evaluator", "call/cc", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "evaluator", "dynamic-wind", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// ???
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "errors", "raise", RuntimePrimitive1::ValueRaise.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "evaluator", "raise-continuable", ProcedurePrimitive::Unsupported.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "errors", "error", RuntimePrimitiveV::ErrorRaise.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "errors", "error-object?", TypePrimitiveV::IsError.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "errors", "error-object-message", RuntimePrimitive1::ErrorMessage.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "errors", "error-object-irritants", RuntimePrimitive1::ErrorArgumentsAsList.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "errors", "read-error?", TypePrimitiveV::IsErrorPortInput.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "errors", "file-error?", TypePrimitiveV::IsErrorFile.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("base", "evaluator", "with-exception-handler", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// ports
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "call-with-port", PortPrimitive2::CallAndClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "parameters", "current-input-port", PortPrimitive0::CurrentInput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "parameters", "current-output-port", PortPrimitive0::CurrentOutput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "parameters", "current-error-port", PortPrimitive0::CurrentError.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "port?", TypePrimitiveV::IsPort.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "input-port?", TypePrimitiveV::IsPortInput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "input-port-open?", PortPrimitiveV::IsInputOpen.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "output-port?", TypePrimitiveV::IsPortOutput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "output-port-open?", PortPrimitiveV::IsOutputOpen.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "binary-port?", TypePrimitiveV::IsPortBinary.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "textual-port?", TypePrimitiveV::IsPortTextual.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "open-input-string", PortPrimitive1::InputFromString.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "open-output-string", PortPrimitiveV::OutputToString.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "get-output-string", PortPrimitive1::OutputToStringFinalize.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "open-input-bytevector", PortPrimitive1::InputFromBytes.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "open-output-bytevector", PortPrimitiveV::OutputToBytes.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "get-output-bytevector", PortPrimitive1::OutputToBytesFinalize.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "close-port", PortPrimitiveV::Close.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "close-input-port", PortPrimitiveV::CloseInput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "close-output-port", PortPrimitiveV::CloseOutput.into ()),
			
			
			
			
			// ports input
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "char-ready?", PortPrimitiveV::CharacterReady.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "peek-char", PortPrimitiveV::CharacterPeek.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "read-char", PortPrimitiveV::CharacterRead.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "read-string", PortPrimitiveV::StringReadCollect.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "u8-ready?", PortPrimitiveV::ByteReady.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "peek-u8", PortPrimitiveV::BytePeek.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "read-u8", PortPrimitiveV::ByteRead.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "read-bytevector", PortPrimitiveV::BytesReadCollect.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base", "ports", "read-bytevector!", PortPrimitiveV::BytesReadCopy.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "read-line", PortPrimitiveV::StringReadLine.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "eof-object", PortPrimitive0::Eof.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "eof-object?", TypePrimitiveV::IsPortEof.into ()),
			
			
			
			
			// ports output
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "write-char", PortPrimitiveV::CharacterWrite.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("base", "ports", "write-string", PortPrimitiveV::StringWrite.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "write-u8", PortPrimitiveV::ByteWrite.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("base", "ports", "write-bytevector", PortPrimitiveV::BytesWrite.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "newline", PortPrimitiveV::NewLine.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("base", "ports", "flush-output-port", PortPrimitiveV::FlushOutput.into ()),
			
			
			
			
			// (scheme case-lambda)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			("case-lambda", "lambda", "case-lambda", SyntaxPrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme char)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-upcase", StringPrimitive1::StringToUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-downcase", StringPrimitive1::StringToLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-foldcase", StringPrimitive1::StringToFoldCase.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-ci=?", ComparisonPrimitiveV::StringCaseInsensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-ci<?", ComparisonPrimitiveV::StringCaseInsensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-ci>?", ComparisonPrimitiveV::StringCaseInsensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-ci<=?", ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "strings", "string-ci>=?", ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-alphabetic?", TypePrimitiveV::IsCharacterAlphabetic.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-upper-case?", TypePrimitiveV::IsCharacterAlphabeticUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-lower-case?", TypePrimitiveV::IsCharacterAlphabeticLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-numeric?", TypePrimitiveV::IsCharacterNumeric.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-whitespace?", TypePrimitiveV::IsCharacterWhitespace.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-upcase", StringPrimitive1::CharacterToUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-downcase", StringPrimitive1::CharacterToLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-foldcase", StringPrimitive1::CharacterToFoldCase.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-ci=?", ComparisonPrimitiveV::CharacterCaseInsensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-ci<?", ComparisonPrimitiveV::CharacterCaseInsensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-ci>?", ComparisonPrimitiveV::CharacterCaseInsensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-ci<=?", ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char", "characters", "char-ci>=?", ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
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
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caaaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caaadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caaddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cadaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cadadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "caddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cadddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdaaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdaadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdadar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdaddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cddaar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cddadr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cdddar", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairLeft.into ()])) .into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("cxr", "pairs", "cddddr", ProcedureExtendedInternals::ComposedPrimitive1 (StdBox::new ([ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into (), ListPrimitive1::PairRight.into ()])) .into ()),
			
			
			
			
			// (scheme eval)
			//     --> verified
			
			("eval", "evaluator", "environment", ProcedurePrimitive::Unimplemented.into ()),
			("eval", "evaluator", "eval", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme file)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "open-input-file", PortPrimitive1::OpenTextualInput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "open-binary-input-file", PortPrimitive1::OpenBinaryInput.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "open-output-file", PortPrimitive1::OpenTextualOutput.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "open-binary-output-file", PortPrimitive1::OpenBinaryOutput.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "call-with-input-file", PortPrimitive2::OpenTextualInputThenCallAndClose.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("file", "ports", "call-with-output-file", PortPrimitive2::OpenTextualOutputThenCallAndClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			#[ cfg ( feature = "vonuvoli_evaluator" ) ]
			("file", "parameters", "with-input-from-file", PortPrimitive2::WithOpenTextualInputThenCallAndClose.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			#[ cfg ( feature = "vonuvoli_evaluator" ) ]
			("file", "parameters", "with-output-to-file", PortPrimitive2::WithOpenTextualOutputThenCallAndClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			("file", "system", "file-exists?", FileSystemPrimitive1::FileExists.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			("file", "system", "delete-file", FileSystemPrimitive1::FileDelete.into ()),
			
			
			
			
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
			
			("inexact", "arithmetic", "finite?", TypePrimitiveV::IsNumberFinite.into ()),
			("inexact", "arithmetic", "infinite?", TypePrimitiveV::IsNumberInfinite.into ()),
			("inexact", "arithmetic", "nan?", TypePrimitiveV::IsNumberNan.into ()),
			
			
			
			
			// (scheme lazy)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("lazy", "promises", "delay", SyntaxPrimitive::Unimplemented.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("lazy", "promises", "delay-force", SyntaxPrimitive::Unimplemented.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("lazy", "promises", "promise?", TypePrimitiveV::IsPromise.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("lazy", "promises", "make-promise", ProcedurePrimitive::Unimplemented.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("lazy", "promises", "force", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme load)
			//     --> verified
			
			("load", "modules", "load", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme process-context)
			//     --> verified
			
			("process-context", "system", "command-line", RuntimePrimitive0::ProcessArgumentsAsList.into ()),
			("process-context", "system", "get-environment-variable", RuntimePrimitive1::ProcessEnvironmentVariable.into ()),
			("process-context", "system", "get-environment-variables", RuntimePrimitive0::ProcessEnvironmentVariablesAsList.into ()),
			
			("process-context", "system", "exit", RuntimePrimitiveV::ProcessExit.into ()),
			("process-context", "system", "emergency-exit", RuntimePrimitiveV::ProcessExitEmergency.into ()),
			
			
			
			
			// (scheme r5rs)
			//     --> verified
			
			("r5rs", "evaluator", "interaction-environment", ProcedurePrimitive::Unimplemented.into ()),
			("r5rs", "evaluator", "scheme-report-environment", ProcedurePrimitive::Unimplemented.into ()),
			("r5rs", "evaluator", "null-environment", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme read)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
			("read", "ports", "read", PortPrimitiveV::ValueRead.into ()),
			
			
			
			
			// (scheme repl)
			//     --> verified
			
			("repl", "evaluator", "interaction-environment", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme time)
			//     --> verified
			
			("time", "system", "current-second", RuntimePrimitive0::PosixTimestamp.into ()),
			("time", "system", "current-jiffy", RuntimePrimitive0::JiffiesTimestamp.into ()),
			("time", "system", "jiffies-per-second", RuntimePrimitive0::JiffiesPerSecond.into ()),
			
			
			
			
			// (scheme write)
			//     --> verified
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("write", "ports", "write", PortPrimitiveV::ValueWrite.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("write", "ports", "write-shared", PortPrimitiveV::ValueWriteShared.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("write", "ports", "write-simple", PortPrimitiveV::ValueWriteSimple.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("write", "ports", "display", PortPrimitiveV::ValueDisplay.into ()),
			
			
			
			
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
			if ! Value::is_self (existing, value) {
				#[ cfg ( feature = "vonuvoli_transcript" ) ]
				trace_error! (transcript, 0x470d2ba5 => "duplicate missmatched mapping for `{}`!" => (identifier.string_as_str ()));
				errors = true;
			}
		}
	}
	
	for &(ref library, ref category, _, _) in definitions {
		if let Some (count) = libraries.get_mut (library) {
			*count += 1;
		} else {
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0x6bcf5b92 => "unknown library `{}`!" => (library.string_as_str ()));
			errors = true;
		}
		if let Some (count) = categories.get_mut (category) {
			*count += 1;
		} else {
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0x915ff763 => "unknown category `{}`!" => (category.string_as_str ()));
			errors = true;
		}
	}
	
	for (library, count) in libraries {
		if count == 0 {
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_warning! (transcript, 0x17d45f75 => "unused library `{}`!" => (library.string_as_str ()));
			errors = true;
		}
	}
	for (category, count) in categories {
		if count == 0 {
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_warning! (transcript, 0xc4227311 => "unused category `{}`!" => (category.string_as_str ()));
			errors = true;
		}
	}
	
	
	if !errors {
		succeed! (());
	} else {
		fail! (0x24ce2821);
	}
	
}

