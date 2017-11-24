

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::generate_binding_templates as language_r7rs_generate_binding_templates;
	pub use super::generate_definitions as language_r7rs_generate_definitions;
}




pub fn generate_binding_templates () -> (Outcome<StdVec<ContextBindingTemplate>>) {
	let definitions = try! (generate_definitions ());
	let templates = vec_map! (
			definitions,
			(_library, identifier, value),
			ContextBindingTemplate {
					identifier : identifier,
					value : Some (value),
					immutable : true,
				});
	succeed! (templates);
}




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			// https://wiki.volution.ro/CiprianDorinCraciun/Notes/Public/R7rs/Identifiers
			// https://bitbucket.org/cowan/r7rs-wg1-infra/raw/default/SmallLanguageIdentifiers.md
			
			
			
			
			// !!!
			
			("base", "_", SyntaxPrimitive::Auxiliary.into ()),
			("base", "...", SyntaxPrimitive::Auxiliary.into ()),
			("base", "=>", SyntaxPrimitive::Auxiliary.into ()),
			("base", "else", SyntaxPrimitive::Auxiliary.into ()),
			
			("base", "quote", SyntaxPrimitive1::Quote.into ()),
			("base", "quasiquote", SyntaxPrimitive1::QuasiQuote.into ()),
			("base", "unquote", SyntaxPrimitive1::UnQuote.into ()),
			("base", "unquote-splicing", SyntaxPrimitive1::UnQuoteSplicing.into ()),
			
			("base", "begin", SyntaxPrimitiveN::Begin.into ()),
			("base", "if", SyntaxPrimitiveN::If.into ()),
			("base", "unless", SyntaxPrimitiveN::Unless.into ()),
			("base", "when", SyntaxPrimitiveN::When.into ()),
			("base", "cond", SyntaxPrimitiveN::Cond.into ()),
			("base", "case", SyntaxPrimitiveN::Case.into ()),
			("base", "do", SyntaxPrimitiveN::Do.into ()),
			("base", "guard", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "lambda", SyntaxPrimitiveN::Lambda.into ()),
			
			("base", "and", SyntaxPrimitiveN::And.into ()),
			("base", "or", SyntaxPrimitiveN::Or.into ()),
			
			("base", "define", SyntaxPrimitiveN::Define.into ()),
			("base", "define-values", SyntaxPrimitive2::DefineValues.into ()),
			("base", "define-syntax", SyntaxPrimitive::Unimplemented.into ()),
			("base", "define-record-type", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "let", SyntaxPrimitiveN::LetParallel.into ()),
			("base", "let*", SyntaxPrimitiveN::LetSequential.into ()),
			
			("base", "letrec", SyntaxPrimitiveN::LetRecursiveParallel.into ()),
			("base", "letrec*", SyntaxPrimitiveN::LetRecursiveSequential.into ()),
			
			("base", "let-values", SyntaxPrimitiveN::LetValuesParallel.into ()),
			("base", "let*-values", SyntaxPrimitiveN::LetValuesSequential.into ()),
			
			("base", "let-syntax", SyntaxPrimitive::Unimplemented.into ()),
			("base", "letrec-syntax", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "set!", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "import", SyntaxPrimitive::Unimplemented.into ()),
			("base", "include", SyntaxPrimitive::Unimplemented.into ()),
			("base", "include-ci", SyntaxPrimitive::Unimplemented.into ()),
			("base", "cond-expand", SyntaxPrimitive::Unimplemented.into ()),
			
			("base", "parameterize", SyntaxPrimitive::Unimplemented.into ()),
			("base", "make-parameter", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "syntax-error", SyntaxPrimitive::Unimplemented.into ()),
			("base", "syntax-rules", SyntaxPrimitive::Unimplemented.into ()),
			
			
			
			
			// ???
			
			("base", "features", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "null?", TypePrimitive1::IsNull.into ()),
			
			
			
			
			// equivalences
			
			("base", "eq?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "eqv?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "equal?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// math
			
			("base", "number?", TypePrimitive1::IsNumber.into ()),
			("base", "integer?", ArithmeticPrimitive1::IsInteger.into ()),
			("base", "real?", ArithmeticPrimitive1::IsReal.into ()),
			("base", "rational?", ArithmeticPrimitive1::IsRational.into ()),
			("base", "complex?", ArithmeticPrimitive1::IsComplex.into ()),
			("base", "exact?", ArithmeticPrimitive1::IsExact.into ()),
			("base", "exact-integer?", ArithmeticPrimitive1::IsExactInteger.into ()),
			("base", "inexact?", ArithmeticPrimitive1::IsInexact.into ()),
			
			("base", "zero?", ArithmeticPrimitive1::IsZero.into ()),
			("base", "positive?", ArithmeticPrimitive1::IsPositive.into ()),
			("base", "negative?", ArithmeticPrimitive1::IsNegative.into ()),
			("base", "odd?", ArithmeticPrimitive1::IsOdd.into ()),
			("base", "even?", ArithmeticPrimitive1::IsEven.into ()),
			
			("base", "+", ArithmeticPrimitiveN::Addition.into ()),
			("base", "-", ArithmeticPrimitiveN::Subtraction.into ()),
			("base", "*", ArithmeticPrimitiveN::Multiplication.into ()),
			("base", "/", ArithmeticPrimitiveN::Division.into ()),
			
			("base", "abs", ArithmeticPrimitive1::Absolute.into ()),
			
			("base", "quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "remainder", ProcedurePrimitive::Unimplemented.into ()),
			("base", "modulo", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "floor", ArithmeticPrimitive1::Floor.into ()),
			("base", "ceiling", ArithmeticPrimitive1::Ceiling.into ()),
			("base", "truncate", ArithmeticPrimitive1::Truncate.into ()),
			("base", "round", ArithmeticPrimitive1::Round.into ()),
			
			("base", "rationalize", ProcedurePrimitive::Unimplemented.into ()),
			("base", "numerator", ProcedurePrimitive::Unimplemented.into ()),
			("base", "denominator", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "floor/", ProcedurePrimitive::Unimplemented.into ()),
			("base", "floor-quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "floor-remainder", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "truncate/", ProcedurePrimitive::Unimplemented.into ()),
			("base", "truncate-quotient", ProcedurePrimitive::Unimplemented.into ()),
			("base", "truncate-remainder", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "min", ArithmeticPrimitiveN::Minimum.into ()),
			("base", "max", ArithmeticPrimitiveN::Maximum.into ()),
			
			("base", "gcd", ProcedurePrimitive::Unimplemented.into ()),
			("base", "lcm", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "expt", ArithmeticPrimitive2::Power.into ()),
			("base", "square", ArithmeticPrimitive1::Square.into ()),
			("base", "exact-integer-sqrt", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "=", ProcedurePrimitive::Unimplemented.into ()),
			("base", "<", ProcedurePrimitive::Unimplemented.into ()),
			("base", ">", ProcedurePrimitive::Unimplemented.into ()),
			("base", "<=", ProcedurePrimitive::Unimplemented.into ()),
			("base", ">=", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "inexact", ProcedurePrimitive::Unimplemented.into ()),
			("base", "exact", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// boolean
			
			("base", "boolean?", TypePrimitive1::IsBoolean.into ()),
			("base", "boolean=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "not", TypePrimitive1::IsFalse.into ()),
			
			
			
			
			// characters
			
			("base", "char?", TypePrimitive1::IsCharacter.into ()),
			
			("base", "char=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "char<?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "char>?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "char<=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "char>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// symbols
			
			("base", "symbol?", TypePrimitive1::IsSymbol.into ()),
			("base", "symbol=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// pairs
			
			("base", "pair?", TypePrimitive1::IsPair.into ()),
			
			("base", "cons", ListPrimitive2::Pair.into ()),
			("base", "car", ListPrimitive1::PairLeft.into ()),
			("base", "cdr", ListPrimitive1::PairRight.into ()),
			
			("base", "set-car!", ListPrimitive2::PairLeftSet.into ()),
			("base", "set-cdr!", ListPrimitive2::PairRightSet.into ()),
			
			("base", "caar", ListPrimitive1::ListFirstOfFirst.into ()),
			("base", "cdar", ListPrimitive1::ListRestOfFirst.into ()),
			
			("base", "cadr", ListPrimitive1::ListFirstAt2.into ()),
			("base", "cddr", ListPrimitive1::ListRestAt2.into ()),
			
			
			
			
			// lists
			
			("base", "list?", TypePrimitive1::IsListProperOrEmpty.into ()),
			
			("base", "list", ListPrimitiveN::ListBuild.into ()),
			("base", "make-list", ListPrimitiveN::ListMake.into ()),
			("base", "list-copy", ListPrimitive1::ListClone.into ()),
			("base", "append", ListPrimitiveN::ListAppend.into ()),
			("base", "length", ListPrimitive1::ListLength.into ()),
			
			("base", "list-ref", ListPrimitive2::ListFirstAt.into ()),
			("base", "list-tail", ListPrimitive2::ListPairAt.into ()),
			
			("base", "list-set!", ListPrimitive3::ListFirstAtSet.into ()),
			
			("base", "reverse", ListPrimitive1::ListReverse.into ()),
			
			("base", "memq", ProcedurePrimitive::Unimplemented.into ()),
			("base", "memv", ProcedurePrimitive::Unimplemented.into ()),
			("base", "member", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "assq", ProcedurePrimitive::Unimplemented.into ()),
			("base", "assv", ProcedurePrimitive::Unimplemented.into ()),
			("base", "assoc", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			// vectors
			
			("base", "vector?", TypePrimitive1::IsArray.into ()),
			
			("base", "vector", ArrayPrimitiveN::ArrayBuild.into ()),
			("base", "make-vector", ArrayPrimitiveN::ArrayMake.into ()),
			("base", "vector-copy", ArrayPrimitive1::ArrayClone.into ()),
			("base", "vector-append", ArrayPrimitiveN::ArrayAppend.into ()),
			("base", "vector-length", ArrayPrimitive1::ArrayLength.into ()),
			
			("base", "vector-ref", ArrayPrimitive2::ArrayAt.into ()),
			
			("base", "vector-set!", ArrayPrimitive3::ArrayAtSet.into ()),
			("base", "vector-fill!", ArrayPrimitiveN::ArraySliceFill.into ()),
			("base", "vector-copy!", ArrayPrimitiveN::ArraySliceCopy.into ()),
			
			
			
			
			// bytevectors
			
			("base", "bytevector?", TypePrimitive1::IsBytes.into ()),
			
			("base", "bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "make-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "bytevector-copy", ProcedurePrimitive::Unimplemented.into ()),
			("base", "bytevector-append", ProcedurePrimitive::Unimplemented.into ()),
			("base", "bytevector-length", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "bytevector-u8-ref", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "bytevector-u8-set!", ProcedurePrimitive::Unimplemented.into ()),
			("base", "bytevector-copy!", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// strings
			
			("base", "string?", TypePrimitive1::IsString.into ()),
			
			("base", "string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "substring", ProcedurePrimitive::Unimplemented.into ()),
			("base", "make-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-copy", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-append", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-length", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "string-ref", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "string-set!", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-fill!", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-copy!", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "string=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string<?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string>?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string<=?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// converters to and from strings
			
			("base", "number->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string->number", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "symbol->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string->symbol", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "list->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string->list", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "utf8->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string->utf8", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "vector->string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string->vector", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// converters miscellaneous
			
			("base", "char->integer", ProcedurePrimitive::Unimplemented.into ()),
			("base", "integer->char", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "vector->list", ProcedurePrimitive::Unimplemented.into ()),
			("base", "list->vector", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ???
			
			("base", "procedure?", TypePrimitive1::IsProcedure.into ()),
			
			("base", "apply", FunctionsPrimitiveN::Apply.into ()),
			
			("base", "map", FunctionsPrimitiveN::ListsMap.into ()),
			("base", "for-each", FunctionsPrimitiveN::ListsIterate.into ()),
			
			("base", "vector-map", ProcedurePrimitive::Unimplemented.into ()),
			("base", "vector-for-each", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "string-map", ProcedurePrimitive::Unimplemented.into ()),
			("base", "string-for-each", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "values", ProcedurePrimitive::Unimplemented.into ()),
			("base", "call-with-values", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "call-with-current-continuation", ProcedurePrimitive::Unimplemented.into ()),
			("base", "call/cc", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "dynamic-wind", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ???
			
			("base", "raise", ProcedurePrimitive::Unimplemented.into ()),
			("base", "raise-continuable", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "error", ProcedurePrimitive::Unimplemented.into ()),
			("base", "error-object?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "error-object-message", ProcedurePrimitive::Unimplemented.into ()),
			("base", "error-object-irritants", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-error?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "file-error?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "with-exception-handler", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports
			
			("base", "call-with-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "current-input-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "current-output-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "current-error-port", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "input-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "input-port-open?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "output-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "output-port-open?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "textual-port?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "binary-port?", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "open-input-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "open-output-string", ProcedurePrimitive::Unimplemented.into ()),
			("base", "get-output-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "open-input-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "open-output-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "get-output-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "close-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "close-input-port", ProcedurePrimitive::Unimplemented.into ()),
			("base", "close-output-port", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports input
			
			("base", "char-ready?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "peek-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "u8-ready?", ProcedurePrimitive::Unimplemented.into ()),
			("base", "peek-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			("base", "read-bytevector!", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "read-line", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "eof-object", ProcedurePrimitive::Unimplemented.into ()),
			("base", "eof-object?", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// ports output
			
			("base", "write-char", ProcedurePrimitive::Unimplemented.into ()),
			("base", "write-string", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "write-u8", ProcedurePrimitive::Unimplemented.into ()),
			("base", "write-bytevector", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "newline", ProcedurePrimitive::Unimplemented.into ()),
			
			("base", "flush-output-port", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme case-lambda)
			//     --> verified
			("case-lambda", "case-lambda", SyntaxPrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme char)
			//     --> verified
			
			("char", "string-upcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-downcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-foldcase", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "string-ci=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-ci<?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-ci>?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-ci<=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "string-ci>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "char-alphabetic?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-upper-case?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-lower-case?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-numeric?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-whitespace?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "char-upcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-downcase", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-foldcase", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "char-ci=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-ci<?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-ci>?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-ci<=?", ProcedurePrimitive::Unimplemented.into ()),
			("char", "char-ci>=?", ProcedurePrimitive::Unimplemented.into ()),
			
			("char", "digit-value", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme complex)
			//     --> verified
			
			("complex", "make-rectangular", ProcedurePrimitive::Unimplemented.into ()),
			("complex", "real-part", ProcedurePrimitive::Unimplemented.into ()),
			("complex", "imag-part", ProcedurePrimitive::Unimplemented.into ()),
			
			("complex", "make-polar", ProcedurePrimitive::Unimplemented.into ()),
			("complex", "magnitude", ProcedurePrimitive::Unimplemented.into ()),
			("complex", "angle", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme cxr)
			//     --> verified
			
			("cxr", "caaaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caaadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caadar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caaddr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cadaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cadadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cadar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caddar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cadddr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "caddr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdaaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdaadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdadar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdaddr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cddaar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cddadr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cddar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdddar", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cddddr", ProcedurePrimitive::Unimplemented.into ()),
			("cxr", "cdddr", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme eval)
			//     --> verified
			
			("eval", "environment", ProcedurePrimitive::Unimplemented.into ()),
			("eval", "eval", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme file)
			//     --> verified
			
			("file", "open-input-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "open-binary-input-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "open-output-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "open-binary-output-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "call-with-input-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "call-with-output-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "with-input-from-file", ProcedurePrimitive::Unimplemented.into ()),
			("file", "with-output-to-file", ProcedurePrimitive::Unimplemented.into ()),
			
			("file", "file-exists?", ProcedurePrimitive::Unimplemented.into ()),
			("file", "delete-file", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme inexact)
			//     --> verified
			
			("inexact", "sqrt", ArithmeticPrimitive1::SquareRoot.into ()),
			("inexact", "exp", ArithmeticPrimitive1::Exponential.into ()),
			("inexact", "log", ArithmeticPrimitive1::Logarithm.into ()),
			
			("inexact", "sin", ArithmeticPrimitive1::Sin.into ()),
			("inexact", "cos", ArithmeticPrimitive1::Cos.into ()),
			("inexact", "tan", ArithmeticPrimitive1::Tan.into ()),
			
			("inexact", "asin", ArithmeticPrimitive1::Asin.into ()),
			("inexact", "acos", ArithmeticPrimitive1::Acos.into ()),
			("inexact", "atan", ArithmeticPrimitive1::Atan.into ()),
			
			("inexact", "finite?", ArithmeticPrimitive1::IsFinite.into ()),
			("inexact", "infinite?", ArithmeticPrimitive1::IsInfinite.into ()),
			("inexact", "nan?", ArithmeticPrimitive1::IsNan.into ()),
			
			
			
			
			// (scheme lazy)
			//     --> verified
			
			("lazy", "delay", SyntaxPrimitive::Unimplemented.into ()),
			("lazy", "delay-force", SyntaxPrimitive::Unimplemented.into ()),
			
			("lazy", "promise?", ProcedurePrimitive::Unimplemented.into ()),
			("lazy", "make-promise", ProcedurePrimitive::Unimplemented.into ()),
			("lazy", "force", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme load)
			//     --> verified
			
			("load", "load", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme process-context)
			//     --> verified
			
			("process-context", "command-line", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "get-environment-variable", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "get-environment-variables", ProcedurePrimitive::Unimplemented.into ()),
			
			("process-context", "exit", ProcedurePrimitive::Unimplemented.into ()),
			("process-context", "emergency-exit", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme r5rs)
			//     --> verified
			
			("r5rs", "interaction-environment", ProcedurePrimitive::Unimplemented.into ()),
			("r5rs", "scheme-report-environment", ProcedurePrimitive::Unimplemented.into ()),
			("r5rs", "null-environment", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme read)
			//     --> verified
			
			("read", "read", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme repl)
			//     --> verified
			
			// !! duplicate
			// ("repl", "interaction-environment", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme time)
			//     --> verified
			
			("time", "current-second", ProcedurePrimitive::Unimplemented.into ()),
			("time", "current-jiffy", ProcedurePrimitive::Unimplemented.into ()),
			("time", "jiffies-per-second", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
			// (scheme write)
			//     --> verified
			
			("write", "write", ProcedurePrimitive::Unimplemented.into ()),
			("write", "write-shared", ProcedurePrimitive::Unimplemented.into ()),
			("write", "write-simple", ProcedurePrimitive::Unimplemented.into ()),
			("write", "display", ProcedurePrimitive::Unimplemented.into ()),
			
			
			
			
		];
	
	let definitions = vec_map! (
			definitions,
			(library, identifier, value),
			(Symbol::from (library), Symbol::from (identifier), value));
	
	return Ok (definitions);
}

