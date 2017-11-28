

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::generate_binding_templates as language_r7rs_generate_binding_templates;
	pub use super::generate_definitions as language_r7rs_generate_definitions;
	pub use super::verify_definitions as language_r7rs_verify_definitions;
}




pub fn generate_binding_templates () -> (Outcome<StdVec<ContextBindingTemplate>>) {
	
	let definitions = try! (generate_definitions ());
	
	let definitions = vec_map! (
			definitions,
			(_library, _category, identifier, value),
			(identifier, value));
	
	let definitions =
			definitions
			.into_iter ()
			.collect::<StdMap<_, _>> ();
	
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




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Symbol, Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			// https://wiki.volution.ro/CiprianDorinCraciun/Notes/Public/R7rs/Identifiers
			// https://bitbucket.org/cowan/r7rs-wg1-infra/raw/default/SmallLanguageIdentifiers.md
			
			
			
			
			// !!!
			
			("base", "syntaxes", "_", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "...", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "=>", SyntaxPrimitive::Auxiliary.into ()),
			("base", "syntaxes", "else", SyntaxPrimitive::Auxiliary.into ()),
			
			("base", "quotation", "quote", SyntaxPrimitive1::Quote.into ()),
			("base", "quotation", "quasiquote", SyntaxPrimitive1::QuasiQuote.into ()),
			("base", "quotation", "unquote", SyntaxPrimitive1::UnQuote.into ()),
			("base", "quotation", "unquote-splicing", SyntaxPrimitive1::UnQuoteSplicing.into ()),
			
			("base", "control", "begin", SyntaxPrimitiveN::Begin.into ()),
			
			("base", "control", "if", SyntaxPrimitiveN::If.into ()),
			("base", "control", "unless", SyntaxPrimitiveN::Unless.into ()),
			("base", "control", "when", SyntaxPrimitiveN::When.into ()),
			("base", "control", "cond", SyntaxPrimitiveN::Cond.into ()),
			("base", "control", "case", SyntaxPrimitiveN::Case.into ()),
			("base", "control", "do", SyntaxPrimitiveN::Do.into ()),
			
			("base", "control", "and", SyntaxPrimitiveN::And.into ()),
			("base", "control", "or", SyntaxPrimitiveN::Or.into ()),
			
			("base", "lambda", "lambda", SyntaxPrimitiveN::Lambda.into ()),
			
			("base", "contexts", "define", SyntaxPrimitiveN::Define.into ()),
			("base", "values", "define-values", SyntaxPrimitive2::DefineValues.into ()),
			("base", "syntaxes", "define-syntax", SyntaxPrimitive::Unsupported.into ()),
			("base", "records", "define-record-type", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "contexts", "let", SyntaxPrimitiveN::LetParallel.into ()),
			("base", "contexts", "let*", SyntaxPrimitiveN::LetSequential.into ()),
			("base", "contexts", "letrec", SyntaxPrimitiveN::LetRecursiveParallel.into ()),
			("base", "contexts", "letrec*", SyntaxPrimitiveN::LetRecursiveSequential.into ()),
			("base", "values", "let-values", SyntaxPrimitiveN::LetValuesParallel.into ()),
			("base", "values", "let*-values", SyntaxPrimitiveN::LetValuesSequential.into ()),
			("base", "syntaxes", "let-syntax", SyntaxPrimitive::Unsupported.into ()),
			("base", "syntaxes", "letrec-syntax", SyntaxPrimitive::Unsupported.into ()),
			
			("base", "contexts", "set!", SyntaxPrimitive2::Set.into ()),
			
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
			
			("base", "equivalence", "eq?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "equivalence", "eqv?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "equivalence", "equal?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// math
			
			("base", "types", "number?", TypePrimitive1::IsNumber.into ()),
			("base", "types", "integer?", ArithmeticPrimitive1::IsInteger.into ()),
			("base", "types", "real?", ArithmeticPrimitive1::IsReal.into ()),
			("base", "types", "rational?", ArithmeticPrimitive1::IsRational.into ()),
			("base", "types", "complex?", ArithmeticPrimitive1::IsComplex.into ()),
			("base", "types", "exact?", ArithmeticPrimitive1::IsExact.into ()),
			("base", "types", "exact-integer?", ArithmeticPrimitive1::IsExactInteger.into ()),
			("base", "types", "inexact?", ArithmeticPrimitive1::IsInexact.into ()),
			
			("base", "arithmetic", "zero?", ArithmeticPrimitive1::IsZero.into ()),
			("base", "arithmetic", "positive?", ArithmeticPrimitive1::IsPositive.into ()),
			("base", "arithmetic", "negative?", ArithmeticPrimitive1::IsNegative.into ()),
			("base", "arithmetic", "odd?", ArithmeticPrimitive1::IsOdd.into ()),
			("base", "arithmetic", "even?", ArithmeticPrimitive1::IsEven.into ()),
			
			("base", "arithmetic", "+", ArithmeticPrimitiveN::Addition.into ()),
			("base", "arithmetic", "-", ArithmeticPrimitiveN::Subtraction.into ()),
			("base", "arithmetic", "*", ArithmeticPrimitiveN::Multiplication.into ()),
			("base", "arithmetic", "/", ArithmeticPrimitiveN::Division.into ()),
			
			("base", "arithmetic", "abs", ArithmeticPrimitive1::Absolute.into ()),
			
			("base", "arithmetic", "quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "remainder", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "modulo", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "floor", ArithmeticPrimitive1::Floor.into ()),
			("base", "arithmetic", "ceiling", ArithmeticPrimitive1::Ceiling.into ()),
			("base", "arithmetic", "truncate", ArithmeticPrimitive1::Truncate.into ()),
			("base", "arithmetic", "round", ArithmeticPrimitive1::Round.into ()),
			
			("base", "arithmetic", "rationalize", ProcedurePrimitive::Unsupported.into ()),
			("base", "arithmetic", "numerator", ProcedurePrimitive::Unsupported.into ()),
			("base", "arithmetic", "denominator", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "arithmetic", "floor/", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "floor-quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "floor-remainder", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "truncate/", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "truncate-quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "truncate-remainder", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "min", ArithmeticPrimitiveN::Minimum.into ()),
			("base", "arithmetic", "max", ArithmeticPrimitiveN::Maximum.into ()),
			
			("base", "arithmetic", "gcd", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "lcm", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "expt", ArithmeticPrimitive2::Power.into ()),
			("base", "arithmetic", "square", ArithmeticPrimitive1::Square.into ()),
			("base", "arithmetic", "exact-integer-sqrt", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "=", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "<", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", ">", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", "<=", ProcedurePrimitive::Unimplemented.into ()),
			("base", "arithmetic", ">=", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "arithmetic", "inexact", ProcedurePrimitive::Unsupported.into ()),
			("base", "arithmetic", "exact", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// boolean
			
			("base", "types", "boolean?", TypePrimitive1::IsBoolean.into ()),
			
			("base", "equivalence", "boolean=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "equivalence", "not", TypePrimitive1::IsFalse.into ()),
			
			
			
			
			// characters
			
			("base", "types", "char?", TypePrimitive1::IsCharacter.into ()),
			
			("base", "characters", "char=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "characters", "char<?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "characters", "char>?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "characters", "char<=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "characters", "char>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// symbols
			
			("base", "types", "symbol?", TypePrimitive1::IsSymbol.into ()),
			
			("base", "equivalence", "symbol=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
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
			
			("base", "lists", "list", ListPrimitiveN::ListBuild.into ()),
			("base", "lists", "make-list", ListPrimitiveN::ListMake.into ()),
			("base", "lists", "list-copy", ListPrimitive1::ListClone.into ()),
			("base", "lists", "append", ListPrimitiveN::ListAppend.into ()),
			("base", "lists", "length", ListPrimitive1::ListLength.into ()),
			
			("base", "lists", "list-ref", ListPrimitive2::ListFirstAt.into ()),
			("base", "lists", "list-tail", ListPrimitive2::ListPairAt.into ()),
			
			("base", "lists", "list-set!", ListPrimitive3::ListFirstAtSet.into ()),
			
			("base", "lists", "reverse", ListPrimitive1::ListReverse.into ()),
			
			("base", "lists", "memq", ProcedurePrimitive::Unimplemented.into ()),
			("base", "lists", "memv", ProcedurePrimitive::Unimplemented.into ()),
			("base", "lists", "member", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "lists", "assq", ProcedurePrimitive::Unimplemented.into ()),
			("base", "lists", "assv", ProcedurePrimitive::Unimplemented.into ()),
			("base", "lists", "assoc", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// vectors
			
			("base", "types", "vector?", TypePrimitive1::IsArray.into ()),
			
			("base", "vectors", "vector", ArrayPrimitiveN::ArrayBuild.into ()),
			("base", "vectors", "make-vector", ArrayPrimitiveN::ArrayMake.into ()),
			("base", "vectors", "vector-copy", ArrayPrimitiveN::ArraySliceClone.into ()),
			("base", "vectors", "vector-append", ArrayPrimitiveN::ArrayAppend.into ()),
			("base", "vectors", "vector-length", ArrayPrimitive1::ArrayLength.into ()),
			
			("base", "vectors", "vector-ref", ArrayPrimitive2::ArrayAt.into ()),
			
			("base", "vectors", "vector-set!", ArrayPrimitive3::ArrayAtSet.into ()),
			("base", "vectors", "vector-fill!", ArrayPrimitiveN::ArraySliceFill.into ()),
			("base", "vectors", "vector-copy!", ArrayPrimitiveN::ArraySliceCopy.into ()),
			
			
			
			
			// bytevectors
			
			("base", "types", "bytevector?", TypePrimitive1::IsBytes.into ()),
			
			("base", "bytes", "bytevector", BytesPrimitiveN::BytesBuild.into ()),
			("base", "bytes", "make-bytevector", BytesPrimitiveN::BytesMake.into ()),
			("base", "bytes", "bytevector-copy", BytesPrimitiveN::BytesSliceClone.into ()),
			("base", "bytes", "bytevector-append", BytesPrimitiveN::BytesAppend.into ()),
			("base", "bytes", "bytevector-length", BytesPrimitive1::BytesLength.into ()),
			
			("base", "bytes", "bytevector-u8-ref", BytesPrimitive2::BytesAt.into ()),
			
			("base", "bytes", "bytevector-u8-set!", BytesPrimitive3::BytesAtSet.into ()),
			("base", "bytes", "bytevector-copy!", BytesPrimitiveN::BytesSliceFill.into ()),
			
			
			
			
			// strings
			
			("base", "types", "string?", TypePrimitive1::IsString.into ()),
			
			("base", "strings", "string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "substring", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "make-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string-copy", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string-append", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string-length", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "string-ref", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "string-set!", ProcedurePrimitive::Unsupported.into ()),
			("base", "strings", "string-fill!", ProcedurePrimitive::Unsupported.into ()),
			("base", "strings", "string-copy!", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "strings", "string=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string<?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string>?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string<=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// converters to and from strings
			
			("base", "strings", "number->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string->number", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "symbol->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string->symbol", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "list->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string->list", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "utf8->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string->utf8", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "strings", "vector->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "strings", "string->vector", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// converters miscellaneous
			
			("base", "characters", "char->integer", ProcedurePrimitive::Unimplemented.into ()),
			("base", "characters", "integer->char", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "vectors", "vector->list", ProcedurePrimitive::Unimplemented.into ()),
			("base", "vectors", "list->vector", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ???
			
			("base", "types", "procedure?", TypePrimitive1::IsProcedure.into ()),
			
			("base", "functions", "apply", FunctionsPrimitiveN::Apply.into ()),
			
			("base", "functions", "map", FunctionsPrimitiveN::ListsMap.into ()),
			("base", "functions", "for-each", FunctionsPrimitiveN::ListsIterate.into ()),
			
			("base", "functions", "vector-map", ProcedurePrimitive::Unimplemented.into ()),
			("base", "functions", "vector-for-each", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "functions", "string-map", ProcedurePrimitive::Unimplemented.into ()),
			("base", "functions", "string-for-each", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "values", "values", ProcedurePrimitive::Unimplemented.into ()),
			("base", "values", "call-with-values", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "evaluator", "call-with-current-continuation", ProcedurePrimitive::Unsupported.into ()),
			("base", "evaluator", "call/cc", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "evaluator", "dynamic-wind", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// ???
			
			("base", "evaluator", "raise", ProcedurePrimitive::Unimplemented.into ()),
			("base", "evaluator", "raise-continuable", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "errors", "error", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "error-object?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "error-object-message", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "error-object-irritants", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "read-error?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "errors", "file-error?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "evaluator", "with-exception-handler", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports
			
			("base", "ports", "call-with-port", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "parameters", "current-input-port", ProcedurePrimitive::Unsupported.into ()),
			("base", "parameters", "current-output-port", ProcedurePrimitive::Unsupported.into ()),
			("base", "parameters", "current-error-port", ProcedurePrimitive::Unsupported.into ()),
			
			("base", "ports", "port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "input-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "input-port-open?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "output-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "output-port-open?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "textual-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "binary-port?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "open-input-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "open-output-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "get-output-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "open-input-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "open-output-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "get-output-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "close-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "close-input-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "close-output-port", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports input
			
			("base", "ports", "char-ready?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "peek-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "read-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "read-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "u8-ready?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "peek-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "read-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "read-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "read-bytevector!", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "read-line", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "eof-object", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "eof-object?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports output
			
			("base", "ports", "write-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "write-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "write-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "ports", "write-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "newline", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "ports", "flush-output-port", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme case-lambda)
			//     --> verified
			
			("case-lambda", "lambda", "case-lambda", SyntaxPrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme char)
			//     --> verified
			
			("char", "strings", "string-upcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-downcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-foldcase", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "strings", "string-ci=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-ci<?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-ci>?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-ci<=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "strings", "string-ci>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "characters", "char-alphabetic?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-upper-case?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-lower-case?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-numeric?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-whitespace?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "characters", "char-upcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-downcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-foldcase", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "characters", "char-ci=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-ci<?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-ci>?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-ci<=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "characters", "char-ci>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "characters", "digit-value", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
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
			
			("cxr", "pairs", "caaaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caaadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caadar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caaddr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cadaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cadadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cadar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caddar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cadddr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "caddr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdaaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdaadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdadar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdaddr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cddaar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cddadr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cddar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdddar", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cddddr", ProcedurePrimitive::Unsupported.into ()),
			("cxr", "pairs", "cdddr", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme eval)
			//     --> verified
			
			("eval", "evaluator", "environment", ProcedurePrimitive::Unsupported.into ()),
			("eval", "evaluator", "eval", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme file)
			//     --> verified
			
			("file", "ports", "open-input-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "ports", "open-binary-input-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "ports", "open-output-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "ports", "open-binary-output-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "ports", "call-with-input-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "ports", "call-with-output-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "parameters", "with-input-from-file", ProcedurePrimitive::Unsupported.into ()),
			("file", "parameters", "with-output-to-file", ProcedurePrimitive::Unsupported.into ()),
			
			("file", "system", "file-exists?", ProcedurePrimitive::Unimplemented.into ()),
			("file", "system", "delete-file", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
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
			
			("inexact", "arithmetic", "finite?", ArithmeticPrimitive1::IsFinite.into ()),
			("inexact", "arithmetic", "infinite?", ArithmeticPrimitive1::IsInfinite.into ()),
			("inexact", "arithmetic", "nan?", ArithmeticPrimitive1::IsNan.into ()),
			
			
			
			
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
			
			("process-context", "system", "command-line", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "system", "get-environment-variable", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "system", "get-environment-variables", ProcedurePrimitive::Unimplemented.into ()),
			
			("process-context", "system", "exit", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "system", "emergency-exit", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme r5rs)
			//     --> verified
			
			("r5rs", "evaluator", "interaction-environment", ProcedurePrimitive::Unsupported.into ()),
			("r5rs", "evaluator", "scheme-report-environment", ProcedurePrimitive::Unsupported.into ()),
			("r5rs", "evaluator", "null-environment", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme read)
			//     --> verified
			
			("read", "ports", "read", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme repl)
			//     --> verified
			
			("repl", "evaluator", "interaction-environment", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme time)
			//     --> verified
			
			("time", "system", "current-second", ProcedurePrimitive::Unimplemented.into ()),
			("time", "system", "current-jiffy", ProcedurePrimitive::Unsupported.into ()),
			("time", "system", "jiffies-per-second", ProcedurePrimitive::Unsupported.into ()),
			
			
			
			
			// (scheme write)
			//     --> verified
			
			("write", "ports", "write", ProcedurePrimitive::Unimplemented.into ()),
			("write", "ports", "write-shared", ProcedurePrimitive::Unimplemented.into ()),
			("write", "ports", "write-simple", ProcedurePrimitive::Unimplemented.into ()),
			("write", "ports", "display", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
		];
	
	let definitions = vec_map! (
			definitions,
			(library, category, identifier, value),
			(Symbol::from (library), Symbol::from (category), Symbol::from (identifier), value));
	
	succeed! (definitions);
}




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

