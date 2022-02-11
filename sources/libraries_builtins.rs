

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;
use super::builtins::exports::*;
use super::conversions::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::generate_binding_templates as library_builtins_generate_binding_templates;
	pub use super::generate_definitions as library_builtins_generate_definitions;
}




pub fn generate_binding_templates () -> (Outcome<StdVec<BindingTemplate>>) {
	
	let definitions = r#try! (generate_definitions ());
	
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




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let mut definitions = StdVec::new ();
	
	// NOTE:  R7RS extensions (natural extensions to existing constructs)
	definitions.extend_from_slice (&[
			
			("locals", SyntaxPrimitiveV::Locals.into ()),
			("redefine", SyntaxPrimitiveV::ReDefine.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("redefine-values", SyntaxPrimitiveV::ReDefineValues.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("set!-values", SyntaxPrimitiveV::SetValues.into ()),
			
			("loop", SyntaxPrimitiveV::Loop.into ()),
			("while", SyntaxPrimitiveV::While.into ()),
			("while-cond", SyntaxPrimitiveV::WhileCond.into ()),
			("until", SyntaxPrimitiveV::Until.into ()),
			("until-cond", SyntaxPrimitiveV::UntilCond.into ()),
			("do-cond", SyntaxPrimitiveV::DoCond.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("guard*", SyntaxPrimitiveV::Guard.into ()),
			
			("call", FunctionsPrimitiveV::Call.into ()),
			("call-with-list*", FunctionsPrimitive2::CallWithList.into ()),
			("call-with-list", FunctionsPrimitive2::CallWithListBuilder.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("call-with-vector*", FunctionsPrimitive2::CallWithArray.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("call-with-vector", FunctionsPrimitive2::CallWithArrayBuilder.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("call-with-values*", FunctionsPrimitive2::CallWithValues.into ()),
			
			("list*", ListPrimitiveV::ListBuildDotted.into ()),
			("make-pair", ListPrimitiveV::PairMake.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("make-error", RuntimePrimitiveV::ErrorBuild.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("error-object-irritants->vector", RuntimePrimitive1::ErrorArgumentsAsArray.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("error-object-irritants->values", RuntimePrimitive1::ErrorArgumentsAsValues.into ()),
			
			("not-null?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNull) .into ()),
			
			("void?", TypePrimitiveV::IsVoid.into ()),
			("not-void?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsVoid) .into ()),
			("undefined?", TypePrimitiveV::IsUndefined.into ()),
			("not-undefined?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsUndefined) .into ()),
			
			("true?", TypePrimitiveV::IsTrue.into ()),
			("not-true?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsTrue) .into ()),
			("true-or-equivalent?", TypePrimitiveV::IsTrueOrEquivalent.into ()),
			("not-true-or-equivalent?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsTrueOrEquivalent) .into ()),
			("false?", TypePrimitiveV::IsFalse.into ()),
			("not-false?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsFalse) .into ()),
			("false-or-equivalent?", TypePrimitiveV::IsFalseOrEquivalent.into ()),
			("not-false-or-equivalent?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsFalseOrEquivalent) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword?", TypePrimitiveV::IsKeyword.into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique?", TypePrimitiveV::IsUnique.into ()),
			
			("not-pair?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPair) .into ()),
			("not-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListProperOrEmpty) .into ()),
			
			("empty-list?", TypePrimitiveV::IsListEmpty.into ()),
			("any-list?", TypePrimitiveV::IsList.into ()),
			("any-or-empty-list?", TypePrimitiveV::IsListOrEmpty.into ()),
			("proper-or-empty-list?", TypePrimitiveV::IsListProperOrEmpty.into ()),
			("circular-or-empty-list?", TypePrimitiveV::IsListCyclicOrEmpty.into ()),
			("dotted-or-empty-list?", TypePrimitiveV::IsListDottedOrEmpty.into ()),
			
			("not-empty-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListEmpty) .into ()),
			("not-any-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsList) .into ()),
			("not-any-or-empty-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListOrEmpty) .into ()),
			("not-proper-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListProper) .into ()),
			("not-proper-or-empty-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListProperOrEmpty) .into ()),
			("not-circular-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListCyclic) .into ()),
			("not-circular-or-empty-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListCyclicOrEmpty) .into ()),
			("not-dotted-list?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListDotted) .into ()),
			("not-dotted-list-or-emtpy?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsListDottedOrEmpty) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("port-error?", TypePrimitiveV::IsErrorPort.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("write-error?", TypePrimitiveV::IsErrorPortOutput.into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("syntax-error?", TypePrimitiveV::IsErrorSyntax.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("binary-input-port?", TypePrimitiveV::IsPortInputBinary.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("binary-output-port?", TypePrimitiveV::IsPortOutputBinary.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("textual-input-port?", TypePrimitiveV::IsPortInputTextual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("textual-output-port?", TypePrimitiveV::IsPortOutputTextual.into ()),
			
			("not-boolean?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBoolean) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacter) .into ()),
			("not-symbol?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsSymbol) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsKeyword) .into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsUnique) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArray) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytes) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsString) .into ()),
			("not-procedure?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsProcedure) .into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("not-error-object?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsError) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-port-error?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsErrorPort) .into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-read-error?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsErrorPortInput) .into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-write-error?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsErrorPortOutput) .into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-file-error?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsErrorFile) .into ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			("not-syntax-error?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsErrorSyntax) .into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPort) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-input-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortInput) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-output-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortOutput) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-binary-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortBinary) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-binary-input-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortInputBinary) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-binary-output-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortOutputBinary) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-textual-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortTextual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-textual-input-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortInputTextual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-textual-output-port?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortOutputTextual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("not-eof-object?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPortEof) .into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			("not-promise?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPromise) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii?", TypePrimitiveV::IsCharacterAscii.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-numeric?", TypePrimitiveV::IsCharacterAsciiNumeric.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-numeric-8?", TypePrimitiveV::IsCharacterAsciiNumericBase8.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-numeric-16?", TypePrimitiveV::IsCharacterAsciiNumericBase16.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-alphabetic?", TypePrimitiveV::IsCharacterAsciiAlphabetic.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-upper-case?", TypePrimitiveV::IsCharacterAsciiAlphabeticUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-lower-case?", TypePrimitiveV::IsCharacterAsciiAlphabeticLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-alphabetic-or-numeric?", TypePrimitiveV::IsCharacterAsciiAlphabeticOrNumeric.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-whitespace?", TypePrimitiveV::IsCharacterAsciiWhitespace.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-control?", TypePrimitiveV::IsCharacterAsciiControl.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-punctuation?", TypePrimitiveV::IsCharacterAsciiPunctuation.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-ascii-graphic?", TypePrimitiveV::IsCharacterAsciiGraphic.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAscii) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-numeric?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiNumeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-numeric-8?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiNumericBase8) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-numeric-16?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiNumericBase16) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-alphabetic?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiAlphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-upper-case?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiAlphabeticUpperCase) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-lower-case?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiAlphabeticLowerCase) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-alphabetic-or-numeric?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiAlphabeticOrNumeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-whitespace?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiWhitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-control?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiControl) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-punctuation?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiPunctuation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ascii-graphic?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAsciiGraphic) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-alphabetic-or-numeric?", TypePrimitiveV::IsCharacterAlphabeticOrNumeric.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("char-control?", TypePrimitiveV::IsCharacterControl.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-alphabetic-or-numeric?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAlphabeticOrNumeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-alphabetic?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAlphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-upper-case?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAlphabeticUpperCase) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-lower-case?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterAlphabeticLowerCase) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-numeric?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterNumeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-whitespace?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterWhitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-control?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCharacterControl) .into ()),
			
			("not-number?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumber) .into ()),
			("not-integer?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberInteger) .into ()),
			("not-real?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberReal) .into ()),
			("not-rational?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberRational) .into ()),
			("not-complex?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberComplex) .into ()),
			("not-exact?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberExact) .into ()),
			("not-exact-integer?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberExactInteger) .into ()),
			("not-inexact?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberInexact) .into ()),
			
			("not-finite?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberFinite) .into ()),
			("not-infinite?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberInfinite) .into ()),
			("not-nan?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberNan) .into ()),
			
			("not-zero?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberZero) .into ()),
			("not-positive?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberPositive) .into ()),
			("not-negative?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberNegative) .into ()),
			("not-odd?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberOdd) .into ()),
			("not-even?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsNumberEven) .into ()),
			
			("not*", BooleanPrimitive1::Not.into ()),
			("and*", BooleanPrimitiveV::And.into ()),
			("or*", BooleanPrimitiveV::Or.into ()),
			("ior*", BooleanPrimitiveV::Or.into ()),
			("xor*", BooleanPrimitiveV::Xor.into ()),
			("nand*", BooleanPrimitiveV::Nand.into ()),
			("nor*", BooleanPrimitiveV::Nor.into ()),
			("nior*", BooleanPrimitiveV::Nor.into ()),
			("nxor*", BooleanPrimitiveV::Nxor.into ()),
			
			("negative", ArithmeticPrimitive1::Negate.into ()),
			("signum", ArithmeticPrimitive1::Signum.into ()),
			("fractional", ArithmeticPrimitive1::Fractional.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("string-reverse", StringPrimitive1::StringCloneReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("string-reverse!", StringPrimitiveV::StringRangeReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("string->immutable", StringPrimitive1::StringToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("string->mutable", StringPrimitive1::StringToMutable.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("string-empty?", TypePrimitiveV::IsStringEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("string-immutable?", TypePrimitiveV::IsStringImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("string-immutable-empty?", TypePrimitiveV::IsStringImmutableEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("string-mutable?", TypePrimitiveV::IsStringMutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("string-mutable-empty?", TypePrimitiveV::IsStringMutableEmpty.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-immutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringImmutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-immutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringImmutableEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-string-mutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringMutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-string-mutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringMutableEmpty) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword->symbol", StringPrimitive1::KeywordToSymbol.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("symbol->keyword", StringPrimitive1::SymbolToKeyword.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword->string", StringPrimitive1::KeywordToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("string->keyword", StringPrimitive1::StringToKeyword.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("symbol-upcase", StringPrimitive1::SymbolToUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("symbol-downcase", StringPrimitive1::SymbolToLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("symbol-foldcase", StringPrimitive1::SymbolToFoldCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-upcase", StringPrimitive1::KeywordToUpperCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-downcase", StringPrimitive1::KeywordToLowerCase.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-foldcase", StringPrimitive1::KeywordToFoldCase.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-reverse", BytesPrimitive1::BytesCloneReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector-reverse!", BytesPrimitiveV::BytesRangeReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector-u8-fill!", BytesPrimitiveV::BytesRangeFill.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-u8-map", FunctionsPrimitiveV::BytesMap.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-u8-for-each", FunctionsPrimitiveV::BytesIterate.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("bytevector->vector", BytesPrimitiveV::BytesRangeToArray.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector->bytevector", BytesPrimitiveV::ArrayRangeToBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector->list", BytesPrimitiveV::BytesRangeToList.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("list->bytevector", BytesPrimitiveV::ListRangeToBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector->immutable", BytesPrimitive1::BytesToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector->mutable", BytesPrimitive1::BytesToMutable.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-empty?", TypePrimitiveV::IsBytesEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-immutable?", TypePrimitiveV::IsBytesImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector-immutable-empty?", TypePrimitiveV::IsBytesImmutableEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector-mutable?", TypePrimitiveV::IsBytesMutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("bytevector-mutable-empty?", TypePrimitiveV::IsBytesMutableEmpty.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector-immutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesImmutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector-immutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesImmutableEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-bytevector-mutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesMutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-bytevector-mutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesMutableEmpty) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("pair->immutable", ListPrimitive1::PairToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("pair->mutable", ListPrimitive1::PairToMutable.into ()),
			("pair-immutable?", TypePrimitiveV::IsPairImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("pair-mutable?", TypePrimitiveV::IsPairMutable.into ()),
			("not-pair-immutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPairImmutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-pair-mutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPairMutable) .into ()),
			
			("list-ref-cons", ListPrimitive2::ListPairAt.into ()),
			("list-ref-car", ListPrimitive2::ListFirstAt.into ()),
			("list-ref-cdr", ListPrimitive2::ListRestAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list-set-car!", ListPrimitive3::ListFirstAtSet.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list-set-cdr!", ListPrimitive3::ListRestAtSet.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list-copy!", ListPrimitiveV::ListRangeCopy.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list-fill!", ListPrimitiveV::ListRangeFill.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list-reverse!", ListPrimitiveV::ListRangeReverse.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list->immutable", ListPrimitive1::ListToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("list->mutable", ListPrimitive1::ListToMutable.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector-reverse", ArrayPrimitive1::ArrayCloneReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-reverse!", ArrayPrimitiveV::ArrayRangeReverse.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-append!", ArrayPrimitiveV::ArrayRangeExtend.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector->immutable", ArrayPrimitive1::ArrayToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector->mutable", ArrayPrimitive1::ArrayToMutable.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector-empty?", TypePrimitiveV::IsArrayEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector-immutable?", TypePrimitiveV::IsArrayImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector-immutable-empty?", TypePrimitiveV::IsArrayImmutableEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-mutable?", TypePrimitiveV::IsArrayMutable.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-mutable-empty?", TypePrimitiveV::IsArrayMutableEmpty.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArrayEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector-immutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArrayImmutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector-immutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArrayImmutableEmpty) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-vector-mutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArrayMutable) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-vector-mutable-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsArrayMutableEmpty) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-pop!", ArrayPrimitiveV::ArrayPop.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-push!", ArrayPrimitiveV::ArrayPush.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-push-from!", ArrayPrimitive2::ArrayPushFrom.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-insert!", ArrayPrimitiveV::ArrayInsert.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-insert-from!", ArrayPrimitive3::ArrayInsertFrom.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-remove!", ArrayPrimitiveV::ArrayRemove.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-swap!", ArrayPrimitive3::ArraySwap.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-resize!", ArrayPrimitiveV::ArrayResize.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-resize-at!", ArrayPrimitiveV::ArrayResizeAt.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-clear!", ArrayPrimitive1::ArrayClear.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("vector-clear-at!", ArrayPrimitive3::ArrayClearAt.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector-find", ArrayPrimitive2::ArrayFind.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values?", TypePrimitiveV::IsValues.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsValues) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values-empty?", TypePrimitiveV::IsValuesEmpty.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values-empty?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsValuesEmpty) .into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type?", TypePrimitiveV::IsRecordKind.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record-type?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsRecordKind) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type", RecordPrimitive1::RecordKindGet.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-predicate", RecordPrimitiveV::RecordKindIsFn.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-constructor", RecordPrimitiveV::RecordBuildFnN.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-constructor*", RecordPrimitiveV::RecordBuildFnC.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-accessor", RecordPrimitiveV::RecordGetFn.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("record-type-mutator", RecordPrimitiveV::RecordSetFn.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-identifier", RecordPrimitive1::RecordKindIdentifier.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-type-size", RecordPrimitive1::RecordKindSize.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("make-record-type", RecordPrimitiveV::RecordKindBuild.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record?", TypePrimitiveV::IsRecord.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsRecord) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-of?", RecordPrimitiveV::RecordKindIs.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-immutable?", TypePrimitiveV::IsRecordImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record-immutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsRecordImmutable) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("record-mutable?", TypePrimitiveV::IsRecordMutable.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("not-record-mutable?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsRecordMutable) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("make-record", RecordPrimitiveV::RecordBuild.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("make-record*", RecordPrimitiveV::RecordBuildC.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record-ref", RecordPrimitiveV::RecordGet.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("record-set!", RecordPrimitiveV::RecordSet.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("record->immutable", RecordPrimitive1::RecordToImmutable.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("record->mutable", RecordPrimitive1::RecordToMutable.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("record->vector", RecordPrimitiveV::RecordToArray.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector->record", RecordPrimitiveV::RecordFromArray.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("record->values", RecordPrimitiveV::RecordToValues.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values->record", RecordPrimitiveV::RecordFromValues.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record->list", RecordPrimitiveV::RecordToList.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("list->record", RecordPrimitiveV::RecordFromList.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("call-with-binary-input-file", PortPrimitive2::OpenBinaryInputThenCallAndClose.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			("call-with-binary-output-file", PortPrimitive2::OpenBinaryOutputThenCallAndClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			#[ cfg ( feature = "vonuvoli_evaluator" ) ]
			("with-binary-input-file", PortPrimitive2::WithOpenBinaryInputThenCallAndClose.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			#[ cfg ( feature = "vonuvoli_evaluator" ) ]
			("with-binary-output-file", PortPrimitive2::WithOpenBinaryOutputThenCallAndClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-chunk", PortPrimitiveV::BytesReadChunk.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-line", PortPrimitiveV::BytesReadLine.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-zero", PortPrimitiveV::BytesReadZero.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-fold", PortPrimitiveV::BytesReadCollectFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-chunk-fold", PortPrimitiveV::BytesReadChunkFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-line-fold", PortPrimitiveV::BytesReadLineFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("read-bytevector-zero-fold", PortPrimitiveV::BytesReadZeroFold.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("read-bytevector-append!", PortPrimitiveV::BytesReadExtend.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-chunk", PortPrimitiveV::StringReadChunk.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-line", PortPrimitiveV::StringReadLine.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-zero", PortPrimitiveV::StringReadZero.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-fold", PortPrimitiveV::StringReadCollectFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-chunk-fold", PortPrimitiveV::StringReadChunkFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-line-fold", PortPrimitiveV::StringReadLineFold.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("read-string-zero-fold", PortPrimitiveV::StringReadZeroFold.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("read-string-append!", PortPrimitiveV::StringReadExtend.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("write-bytevector-line", PortPrimitiveV::BytesWriteLine.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("write-bytevector-zero", PortPrimitiveV::BytesWriteZero.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("write-string-line", PortPrimitiveV::StringWriteLine.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("write-string-zero", PortPrimitiveV::StringWriteZero.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("write-line", PortPrimitiveV::ValueWriteAndNewLine.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
			("display-line", PortPrimitiveV::ValueDisplayAndNewLine.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
			("read-fold", PortPrimitiveV::ValueReadFold.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			("port-descriptor", PortPrimitive1::DescriptorGet.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			("port-descriptor-clone", PortPrimitive1::DescriptorClone.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			("port-descriptor-ref", PortPrimitive1::DescriptorValue.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			("port-descriptor-path", PortPrimitive1::DescriptorPath.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			("port-descriptor-flag-ref", PortPrimitive2::DescriptorFlagGet.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
			("port-descriptor-flag-set!", PortPrimitive3::DescriptorFlagSet.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
			("open-temporary", PortPrimitiveV::TemporaryTextualCreate.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
			("open-binary-temporary", PortPrimitiveV::TemporaryBinaryCreate.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
			("port-temporary-release", PortPrimitive1::TemporaryRelease.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			("port-temporary-path", PortPrimitive1::TemporaryPath.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("parameter?", TypePrimitiveV::IsParameter.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("not-parameter?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsParameter) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("parameter-ref", RuntimePrimitiveV::ParameterResolve.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("parameter-set!", RuntimePrimitiveV::ParameterConfigure.into ()),
			
			("command-line-ref", RuntimePrimitive1::ProcessArgument.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("command-line->vector", RuntimePrimitive0::ProcessArgumentsAsArray.into ()),
			("command-line-length", RuntimePrimitive0::ProcessArgumentsCount.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("get-environment-variables->vector", RuntimePrimitive0::ProcessEnvironmentVariablesAsArray.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "blake2-rfc" ) ]
			("get-environment-fingerprint", RuntimePrimitive0::ProcessEnvironmentFingerprint.into ()),
			
			("abort", RuntimePrimitiveV::Abort.into ()),
			("pause", RuntimePrimitiveV::Pause.into ()),
			
		]);
	
	// NOTE:  value extensions
	definitions.extend_from_slice (&[
			
			("syntax?", TypePrimitiveV::IsSyntax.into ()),
			("not-syntax?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsSyntax) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			("context?", TypePrimitiveV::IsContext.into ()),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			("not-context?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsContext) .into ()),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			("binding?", TypePrimitiveV::IsBinding.into ()),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			("not-binding?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBinding) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("parameters?", TypePrimitiveV::IsParameters.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("not-parameters?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsParameters) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			("opaque?", TypePrimitiveV::IsOpaque.into ()),
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			("not-opaque?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsOpaque) .into ()),
			
			("resource?", TypePrimitiveV::IsResource.into ()),
			("not-resource?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsResource) .into ()),
			("internal?", TypePrimitiveV::IsInternal.into ()),
			("not-internal?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsInternal) .into ()),
			
		]);
	
	// NOTE:  R7RS extensions (comparisons)
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	definitions.extend_from_slice (&[
			
			("not-eq?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByIdentity) .into ()),
			("not-eqv?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueStrict) .into ()),
			("not-equal?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueStrictRecursive) .into ()),
			
			("equivalent-by-identity?", ComparisonPrimitiveV::EquivalentByIdentity.into ()),
			("equivalent-by-value-strict?", ComparisonPrimitiveV::EquivalentByValueStrict.into ()),
			("equivalent-by-value-strict-recursive?", ComparisonPrimitiveV::EquivalentByValueStrictRecursive.into ()),
			("equivalent-by-value-coerced?", ComparisonPrimitiveV::EquivalentByValueCoerced.into ()),
			("equivalent-by-value-coerced-recursive?", ComparisonPrimitiveV::EquivalentByValueCoercedRecursive.into ()),
			
			("not-equivalent-by-identity?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByIdentity) .into ()),
			("not-equivalent-by-value-strict?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueStrict) .into ()),
			("not-equivalent-by-value-strict-recursive?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueStrictRecursive) .into ()),
			("not-equivalent-by-value-coerced?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueCoerced) .into ()),
			("not-equivalent-by-value-coerced-recursive?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::EquivalentByValueCoercedRecursive) .into ()),
			
			("not-=", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::NumberEqual) .into ()),
			("not-<", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::NumberLesser) .into ()),
			("not->", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::NumberGreater) .into ()),
			("not-<=", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::NumberLesserOrEqual) .into ()),
			("not->=", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::NumberGreaterOrEqual) .into ()),
			
			("boolean<?", ComparisonPrimitiveV::BooleanLesser.into ()),
			("boolean>?", ComparisonPrimitiveV::BooleanGreater.into ()),
			("boolean<=?", ComparisonPrimitiveV::BooleanLesserOrEqual.into ()),
			("boolean>=?", ComparisonPrimitiveV::BooleanGreaterOrEqual.into ()),
			
			("not-boolean=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BooleanEqual) .into ()),
			("not-boolean<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BooleanLesser) .into ()),
			("not-boolean>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BooleanGreater) .into ()),
			("not-boolean<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BooleanLesserOrEqual) .into ()),
			("not-boolean>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BooleanGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseSensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseSensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseSensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseSensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseSensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ci=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseInsensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ci<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseInsensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ci>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseInsensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ci<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseInsensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-char-ci>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::CharacterCaseInsensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseSensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseSensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseSensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseSensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseSensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-ci=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseInsensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-ci<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseInsensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-ci>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseInsensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-ci<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseInsensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("not-string-ci>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::StringCaseInsensitiveGreaterOrEqual) .into ()),
			
			("symbol<?", ComparisonPrimitiveV::SymbolCaseSensitiveLesser.into ()),
			("symbol>?", ComparisonPrimitiveV::SymbolCaseSensitiveGreater.into ()),
			("symbol<=?", ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual.into ()),
			("symbol>=?", ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual.into ()),
			
			("not-symbol=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseSensitiveEqual) .into ()),
			("not-symbol<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseSensitiveLesser) .into ()),
			("not-symbol>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseSensitiveGreater) .into ()),
			("not-symbol<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseSensitiveLesserOrEqual) .into ()),
			("not-symbol>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseSensitiveGreaterOrEqual) .into ()),
			
			("symbol-ci=?", ComparisonPrimitiveV::SymbolCaseInsensitiveEqual.into ()),
			("symbol-ci<?", ComparisonPrimitiveV::SymbolCaseInsensitiveLesser.into ()),
			("symbol-ci>?", ComparisonPrimitiveV::SymbolCaseInsensitiveGreater.into ()),
			("symbol-ci<=?", ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual.into ()),
			("symbol-ci>=?", ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual.into ()),
			
			("not-symbol-ci=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseInsensitiveEqual) .into ()),
			("not-symbol-ci<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseInsensitiveLesser) .into ()),
			("not-symbol-ci>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseInsensitiveGreater) .into ()),
			("not-symbol-ci<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseInsensitiveLesserOrEqual) .into ()),
			("not-symbol-ci>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::SymbolCaseInsensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector=?", ComparisonPrimitiveV::BytesEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector<?", ComparisonPrimitiveV::BytesLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector>?", ComparisonPrimitiveV::BytesGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector<=?", ComparisonPrimitiveV::BytesLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector>=?", ComparisonPrimitiveV::BytesGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BytesEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BytesLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BytesGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BytesLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("not-bytevector>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::BytesGreaterOrEqual) .into ()),
			
			("pair=?", ComparisonPrimitiveV::PairEqual.into ()),
			("pair<?", ComparisonPrimitiveV::PairLesser.into ()),
			("pair>?", ComparisonPrimitiveV::PairGreater.into ()),
			("pair<=?", ComparisonPrimitiveV::PairLesserOrEqual.into ()),
			("pair>=?", ComparisonPrimitiveV::PairGreaterOrEqual.into ()),
			
			("not-pair=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PairEqual) .into ()),
			("not-pair<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PairLesser) .into ()),
			("not-pair>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PairGreater) .into ()),
			("not-pair<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PairLesserOrEqual) .into ()),
			("not-pair>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PairGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector=?", ComparisonPrimitiveV::ArrayEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector<?", ComparisonPrimitiveV::ArrayLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector>?", ComparisonPrimitiveV::ArrayGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector<=?", ComparisonPrimitiveV::ArrayLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("vector>=?", ComparisonPrimitiveV::ArrayGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ArrayEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ArrayLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ArrayGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ArrayLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("not-vector>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ArrayGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values=?", ComparisonPrimitiveV::ValuesEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values<?", ComparisonPrimitiveV::ValuesLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values>?", ComparisonPrimitiveV::ValuesGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values<=?", ComparisonPrimitiveV::ValuesLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("values>=?", ComparisonPrimitiveV::ValuesGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ValuesEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ValuesLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ValuesGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ValuesLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			("not-values>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::ValuesGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record=?", ComparisonPrimitiveV::RecordEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record<?", ComparisonPrimitiveV::RecordLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record>?", ComparisonPrimitiveV::RecordGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record<=?", ComparisonPrimitiveV::RecordLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("record>=?", ComparisonPrimitiveV::RecordGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::RecordEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::RecordLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::RecordGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::RecordLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			("not-record>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::RecordGreaterOrEqual) .into ()),
			
			("generic=?", ComparisonPrimitiveV::GenericEqual.into ()),
			("generic<?", ComparisonPrimitiveV::GenericLesser.into ()),
			("generic>?", ComparisonPrimitiveV::GenericGreater.into ()),
			("generic<=?", ComparisonPrimitiveV::GenericLesserOrEqual.into ()),
			("generic>=?", ComparisonPrimitiveV::GenericGreaterOrEqual.into ()),
			
			("not-generic=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::GenericEqual) .into ()),
			("not-generic<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::GenericLesser) .into ()),
			("not-generic>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::GenericGreater) .into ()),
			("not-generic<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::GenericLesserOrEqual) .into ()),
			("not-generic>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::GenericGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword=?", ComparisonPrimitiveV::KeywordCaseSensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword<?", ComparisonPrimitiveV::KeywordCaseSensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword>?", ComparisonPrimitiveV::KeywordCaseSensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword<=?", ComparisonPrimitiveV::KeywordCaseSensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword>=?", ComparisonPrimitiveV::KeywordCaseSensitiveGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseSensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseSensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseSensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseSensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseSensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-ci=?", ComparisonPrimitiveV::KeywordCaseInsensitiveEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-ci<?", ComparisonPrimitiveV::KeywordCaseInsensitiveLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-ci>?", ComparisonPrimitiveV::KeywordCaseInsensitiveGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-ci<=?", ComparisonPrimitiveV::KeywordCaseInsensitiveLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("keyword-ci>=?", ComparisonPrimitiveV::KeywordCaseInsensitiveGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword-ci=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseInsensitiveEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword-ci<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseInsensitiveLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword-ci>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseInsensitiveGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword-ci<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseInsensitiveLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			("not-keyword-ci>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::KeywordCaseInsensitiveGreaterOrEqual) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique=?", ComparisonPrimitiveV::UniqueEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique<?", ComparisonPrimitiveV::UniqueLesser.into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique>?", ComparisonPrimitiveV::UniqueGreater.into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique<=?", ComparisonPrimitiveV::UniqueLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("unique>=?", ComparisonPrimitiveV::UniqueGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::UniqueEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::UniqueLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::UniqueGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::UniqueLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			("not-unique>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::UniqueGreaterOrEqual) .into ()),
			
		]);
	
	// NOTE:  bitwise expressions
	definitions.extend_from_slice (&[
			
			("bitwise-not", BitwisePrimitive1::Complement.into ()),
			
			("bitwise-and", BitwisePrimitiveV::And.into ()),
			("bitwise-or", BitwisePrimitiveV::Or.into ()),
			("bitwise-ior", BitwisePrimitiveV::Or.into ()),
			("bitwise-xor", BitwisePrimitiveV::Xor.into ()),
			
			("bitwise-nand", BitwisePrimitiveV::Nand.into ()),
			("bitwise-nor", BitwisePrimitiveV::Nor.into ()),
			("bitwise-nior", BitwisePrimitiveV::Nor.into ()),
			("bitwise-nxor", BitwisePrimitiveV::Nxor.into ()),
			
			("bitwise-rotate-left", BitwisePrimitive2::RotateLeft.into ()),
			("bitwise-rotate-right", BitwisePrimitive2::RotateRight.into ()),
			("bitwise-shift-left", BitwisePrimitive2::ShiftLeft.into ()),
			("bitwise-shift-right", BitwisePrimitive2::ShiftRight.into ()),
			
		]);
	
	// NOTE:  string regular expressions
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	definitions.extend_from_slice (&[
			
			("string-regex?", TypePrimitiveV::IsStringRegex.into ()),
			("not-string-regex?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsStringRegex) .into ()),
			("make-string-regex", StringPrimitive1::StringRegexCompile.into ()),
			
			("string-regex-match?", StringPrimitive2::StringRegexMatches.into ()),
			
			("string-regex-match", StringPrimitive2::StringRegexMatchExtractFirst.into ()),
			("string-regex-match-all", StringPrimitive2::StringRegexMatchExtractAllAsList.into ()),
			("string-regex-match-all->vector", StringPrimitive2::StringRegexMatchExtractAllAsArray.into ()),
			
			("string-regex-match-position", StringPrimitive2::StringRegexMatchPositionFirst.into ()),
			("string-regex-match-position-all", StringPrimitive2::StringRegexMatchPositionAllAsList.into ()),
			("string-regex-match-position-all->vector", StringPrimitive2::StringRegexMatchPositionAllAsArray.into ()),
			
			("string-regex-match-captures", StringPrimitive2::StringRegexMatchCapturesExtractFirstAsList.into ()),
			("string-regex-match-captures->assoc", StringPrimitive2::StringRegexMatchCapturesExtractFirstAsAssoc.into ()),
			("string-regex-match-captures->vector", StringPrimitive2::StringRegexMatchCapturesExtractFirstAsArray.into ()),
			
			("string-regex-match-captures-all", StringPrimitive2::StringRegexMatchCapturesExtractAllAsList.into ()),
			("string-regex-match-captures-all->assoc", StringPrimitive2::StringRegexMatchCapturesExtractAllAsAssoc.into ()),
			("string-regex-match-captures-all->vector", StringPrimitive2::StringRegexMatchCapturesExtractAllAsArray.into ()),
			
			("string-regex-match-captures-position", StringPrimitive2::StringRegexMatchCapturesPositionFirstAsList.into ()),
			("string-regex-match-captures-position->assoc", StringPrimitive2::StringRegexMatchCapturesPositionFirstAsAssoc.into ()),
			("string-regex-match-captures-position->vector", StringPrimitive2::StringRegexMatchCapturesPositionFirstAsArray.into ()),
			
			("string-regex-match-captures-position-all", StringPrimitive2::StringRegexMatchCapturesPositionAllAsList.into ()),
			("string-regex-match-captures-position-all->assoc", StringPrimitive2::StringRegexMatchCapturesPositionAllAsAssoc.into ()),
			("string-regex-match-captures-position-all->vector", StringPrimitive2::StringRegexMatchCapturesPositionAllAsArray.into ()),
			
		]);
	
	// NOTE:  bytes regular expressions
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	definitions.extend_from_slice (&[
			
			("bytevector-regex?", TypePrimitiveV::IsBytesRegex.into ()),
			("not-bytevector-regex?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsBytesRegex) .into ()),
			("make-bytevector-regex", BytesPrimitive1::BytesRegexCompile.into ()),
			
			("bytevector-regex-match?", BytesPrimitive2::BytesRegexMatches.into ()),
			
			("bytevector-regex-match", BytesPrimitive2::BytesRegexMatchExtractFirst.into ()),
			("bytevector-regex-match-all", BytesPrimitive2::BytesRegexMatchExtractAllAsList.into ()),
			("bytevector-regex-match-all->vector", BytesPrimitive2::BytesRegexMatchExtractAllAsArray.into ()),
			
			("bytevector-regex-match-position", BytesPrimitive2::BytesRegexMatchPositionFirst.into ()),
			("bytevector-regex-match-position-all", BytesPrimitive2::BytesRegexMatchPositionAllAsList.into ()),
			("bytevector-regex-match-position-all->vector", BytesPrimitive2::BytesRegexMatchPositionAllAsArray.into ()),
			
			("bytevector-regex-match-captures", BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsList.into ()),
			("bytevector-regex-match-captures->assoc", BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsAssoc.into ()),
			("bytevector-regex-match-captures->vector", BytesPrimitive2::BytesRegexMatchCapturesExtractFirstAsArray.into ()),
			
			("bytevector-regex-match-captures-all", BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsList.into ()),
			("bytevector-regex-match-captures-all->assoc", BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsAssoc.into ()),
			("bytevector-regex-match-captures-all->vector", BytesPrimitive2::BytesRegexMatchCapturesExtractAllAsArray.into ()),
			
			("bytevector-regex-match-captures-position", BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsList.into ()),
			("bytevector-regex-match-captures-position->assoc", BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsAssoc.into ()),
			("bytevector-regex-match-captures-position->vector", BytesPrimitive2::BytesRegexMatchCapturesPositionFirstAsArray.into ()),
			
			("bytevector-regex-match-captures-position-all", BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsList.into ()),
			("bytevector-regex-match-captures-position-all->assoc", BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsAssoc.into ()),
			("bytevector-regex-match-captures-position-all->vector", BytesPrimitive2::BytesRegexMatchCapturesPositionAllAsArray.into ()),
			
		]);
	
	// NOTE:  R7RS functional equivalents
	definitions.extend_from_slice (&[
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("curry", FunctionsPrimitiveV::CurryLeft.into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("curry-last", FunctionsPrimitiveV::CurryRight.into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("compose", FunctionsPrimitiveV::Compose1.into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("compose*", FunctionsPrimitiveV::ComposeV.into ()),
			
			("identity", FunctionsPrimitive1::Identity.into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("constant-fn", FunctionsPrimitive1::Constant.into ()),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("constant-fn*", FunctionsPrimitive1::ConstantStar.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			("not-fn", FunctionsPrimitive1::Not.into ()),
			("and-fn", ProcedurePrimitive::Unimplemented.into ()),
			("or-fn", ProcedurePrimitive::Unimplemented.into ()),
			("xor-fn", ProcedurePrimitive::Unimplemented.into ()),
			("nand-fn", ProcedurePrimitive::Unimplemented.into ()),
			("nor-fn", ProcedurePrimitive::Unimplemented.into ()),
			("nxor-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("map-fn", ProcedurePrimitive::Unimplemented.into ()),
			("for-each-fn", ProcedurePrimitive::Unimplemented.into ()),
			("vector-map-fn", ProcedurePrimitive::Unimplemented.into ()),
			("vector-for-each-fn", ProcedurePrimitive::Unimplemented.into ()),
			("string-map-fn", ProcedurePrimitive::Unimplemented.into ()),
			("string-for-each-fn", ProcedurePrimitive::Unimplemented.into ()),
			("bytevector-u8-map-fn", ProcedurePrimitive::Unimplemented.into ()),
			("bytevector-u8-for-each-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("if-fn", ProcedurePrimitive::Unimplemented.into ()),
			("unless-fn", ProcedurePrimitive::Unimplemented.into ()),
			("when-fn", ProcedurePrimitive::Unimplemented.into ()),
			("cond-fn", ProcedurePrimitive::Unimplemented.into ()),
			("case-fn", ProcedurePrimitive::Unimplemented.into ()),
			("do-fn", ProcedurePrimitive::Unimplemented.into ()),
			("begin-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("eq-fn", ProcedurePrimitive::Unimplemented.into ()),
			("eqv-fn", ProcedurePrimitive::Unimplemented.into ()),
			("equal-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("<fn", ProcedurePrimitive::Unimplemented.into ()),
			("<=fn", ProcedurePrimitive::Unimplemented.into ()),
			("=fn", ProcedurePrimitive::Unimplemented.into ()),
			(">=fn", ProcedurePrimitive::Unimplemented.into ()),
			(">fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("+fn", ProcedurePrimitive::Unimplemented.into ()),
			("-fn", ProcedurePrimitive::Unimplemented.into ()),
			("-fn-last", ProcedurePrimitive::Unimplemented.into ()),
			("*fn", ProcedurePrimitive::Unimplemented.into ()),
			("/fn", ProcedurePrimitive::Unimplemented.into ()),
			("/fn-last", ProcedurePrimitive::Unimplemented.into ()),
			
			("cons-car-fn", ProcedurePrimitive::Unimplemented.into ()),
			("cons-cdr-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("member-fn", ProcedurePrimitive::Unimplemented.into ()),
			("assoc-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("vector-ref-fn", ProcedurePrimitive::Unimplemented.into ()),
			("vector-set-fn", ProcedurePrimitive::Unimplemented.into ()),
			
			("bytevector-u8-ref-fn", ProcedurePrimitive::Unimplemented.into ()),
			("bytevector-u8-set-fn", ProcedurePrimitive::Unimplemented.into ()),
			
		]);
	
	// NOTE:  file-system operations
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	definitions.extend_from_slice (&[
			
			("fs-canonicalize", FileSystemPrimitive1::PathCanonicalize.into ()),
			("fs-link-resolve", FileSystemPrimitiveV::SymLinkResolve.into ()),
			
			("fs-directory-list", FileSystemPrimitiveV::DirectoryListAsList.into ()),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			("fs-directory-list->vector", FileSystemPrimitiveV::DirectoryListAsArray.into ()),
			("fs-directory-fold", FileSystemPrimitiveV::DirectoryListFold.into ()),
			("fs-directory-fold-recursive", FileSystemPrimitiveV::DirectoryListFoldRecursive.into ()),
			
			("fs-metadata?", TypePrimitiveV::IsFileSystemMetadata.into ()),
			("not-fs-metadata?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsFileSystemMetadata) .into ()),
			
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
			
			("fs-exists?", FileSystemPrimitiveV::AnyExists.into ()),
			("fs-file-exists?", FileSystemPrimitiveV::FileExists.into ()),
			("fs-file-delete", FileSystemPrimitive1::FileDelete.into ()),
			("fs-directory-exists?", FileSystemPrimitiveV::DirectoryExists.into ()),
			("fs-directory-delete", FileSystemPrimitive1::DirectoryDelete.into ()),
			("fs-symlink-exists?", FileSystemPrimitive1::SymLinkExists.into ()),
			("fs-mount?", FileSystemPrimitiveV::MountPointIs.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
			("fs-temporary-file", FileSystemPrimitiveV::TemporaryCreateFile.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
			("fs-temporary-directory", FileSystemPrimitiveV::TemporaryCreateDirectory.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_filesystem_temporary" ) ]
			("fs-temporary-release", FileSystemPrimitive1::TemporaryRelease.into ()),
			
		]);
	
	// NOTE:  file-system paths
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	definitions.extend_from_slice (&[
			
			("path?", TypePrimitiveV::IsPath.into ()),
			("not-path?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPath) .into ()),
			
			("path", FileSystemPrimitive1::PathCoerce.into ()),
			
			("path-absolute?", TypePrimitiveV::IsPathAbsolute.into ()),
			("not-path-absolute?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPathAbsolute) .into ()),
			("path-relative?", TypePrimitiveV::IsPathRelative.into ()),
			("not-path-relative?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPathRelative) .into ()),
			
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("path->string", FileSystemPrimitive1::PathToString.into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("string->path", FileSystemPrimitive1::StringToPath.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("path->bytevector", FileSystemPrimitive1::PathToBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("bytevector->path", FileSystemPrimitive1::BytesToPath.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("path=?", ComparisonPrimitiveV::PathEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("path<?", ComparisonPrimitiveV::PathLesser.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("path>?", ComparisonPrimitiveV::PathGreater.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("path<=?", ComparisonPrimitiveV::PathLesserOrEqual.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("path>=?", ComparisonPrimitiveV::PathGreaterOrEqual.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("not-path=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PathEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("not-path<?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PathLesser) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("not-path>?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PathGreater) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("not-path<=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PathLesserOrEqual) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			("not-path>=?", ProcedurePrimitiveV::ComparisonNegated (ComparisonPrimitiveV::PathGreaterOrEqual) .into ()),
			
		]);
	
	// NOTE:  sub-processes
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	definitions.extend_from_slice (&[
			
			("process?", TypePrimitiveV::IsProcess.into ()),
			("not-process?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsProcess) .into ()),
			
			("process-spawn", RuntimePrimitiveN::ProcessSpawn.into ()),
			("process-spawn*", RuntimePrimitiveV::ProcessSpawnExtended.into ()),
			("process-exec", RuntimePrimitiveN::ProcessExec.into ()),
			("process-exec*", RuntimePrimitiveV::ProcessExecExtended.into ()),
			("process-wait-poll", RuntimePrimitive1::ProcessWaitPoll.into ()),
			("process-wait", RuntimePrimitive1::ProcessWaitCheck.into ()),
			("process-wait-try", RuntimePrimitive1::ProcessWaitTry.into ()),
			("process-run", RuntimePrimitiveN::ProcessRunCheck.into ()),
			("process-run-try", RuntimePrimitiveN::ProcessRunTry.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:env-empty", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-empty"), PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:env-include", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-include"), PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:env-exclude", Parameter::for_builtin (symbol_clone_str ("process-spawn:environment-exclude"), PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:directory", Parameter::for_builtin (symbol_clone_str ("process-spawn:working-directory"), PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:stdin", Parameter::for_builtin (symbol_clone_str ("process-spawn:stdin"), PROCESS_PARAMETER_STDIN_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:stdout", Parameter::for_builtin (symbol_clone_str ("process-spawn:stdout"), PROCESS_PARAMETER_STDOUT_HANDLE_VALUE, false) .into ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			("process-spawn:stderr", Parameter::for_builtin (symbol_clone_str ("process-spawn:stderr"), PROCESS_PARAMETER_STDERR_HANDLE_VALUE, false) .into ()),
			
			("process-stdin", RuntimePrimitive1::ProcessStdinGet.into ()),
			("process-stdout", RuntimePrimitive1::ProcessStdoutGet.into ()),
			("process-stderr", RuntimePrimitive1::ProcessStderrGet.into ()),
			
		]);
	
	// NOTE:  transcripts
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
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
			("xcons", ListPrimitive2::PairConsExchanged.into ()),
			("map-in-order", FunctionsPrimitive2::ListsMap.into ()),
			
			// NOTE:  SRFI-1 unimplemented procedures!
			("cons*", ProcedurePrimitive::Unimplemented.into ()),
			("list-tabulate", ProcedurePrimitive::Unimplemented.into ()),
			("circular-list", ProcedurePrimitive::Unimplemented.into ()),
			("iota", ProcedurePrimitive::Unimplemented.into ()),
			("proper-list?", TypePrimitiveV::IsListProper.into ()),
			("circular-list?", TypePrimitiveV::IsListCyclic.into ()),
			("dotted-list?", TypePrimitiveV::IsListDotted.into ()),
			("null-list?", TypePrimitiveV::IsListEmpty.into ()),
			// ("not-pair?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsPair) .into ()),
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
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("crypto-bytevector", procedure_native_1 (crypto_generate_bytes_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("crypto-bytevector-append!", procedure_native_2 (crypto_generate_bytes_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
			("crypto-blake2b-8", procedure_native_1 (crypto_hash_blake2b_8) .into ()),
			("crypto-blake2b-16", procedure_native_1 (crypto_hash_blake2b_16) .into ()),
			("crypto-blake2b-32", procedure_native_1 (crypto_hash_blake2b_32) .into ()),
			("crypto-blake2b-64", procedure_native_1 (crypto_hash_blake2b_64) .into ()),
			("crypto-blake2b-128", procedure_native_1 (crypto_hash_blake2b_128) .into ()),
			("crypto-blake2b-192", procedure_native_1 (crypto_hash_blake2b_192) .into ()),
			("crypto-blake2b-224", procedure_native_1 (crypto_hash_blake2b_224) .into ()),
			("crypto-blake2b-256", procedure_native_1 (crypto_hash_blake2b_256) .into ()),
			("crypto-blake2b-320", procedure_native_1 (crypto_hash_blake2b_320) .into ()),
			("crypto-blake2b-384", procedure_native_1 (crypto_hash_blake2b_384) .into ()),
			("crypto-blake2b-448", procedure_native_1 (crypto_hash_blake2b_448) .into ()),
			("crypto-blake2b-512", procedure_native_1 (crypto_hash_blake2b_512) .into ()),
			("crypto-blake2s-8", procedure_native_1 (crypto_hash_blake2s_8) .into ()),
			("crypto-blake2s-16", procedure_native_1 (crypto_hash_blake2s_16) .into ()),
			("crypto-blake2s-32", procedure_native_1 (crypto_hash_blake2s_32) .into ()),
			("crypto-blake2s-64", procedure_native_1 (crypto_hash_blake2s_64) .into ()),
			("crypto-blake2s-128", procedure_native_1 (crypto_hash_blake2s_128) .into ()),
			("crypto-blake2s-192", procedure_native_1 (crypto_hash_blake2s_192) .into ()),
			("crypto-blake2s-224", procedure_native_1 (crypto_hash_blake2s_224) .into ()),
			("crypto-blake2s-256", procedure_native_1 (crypto_hash_blake2s_256) .into ()),
			("crypto-blake3-8", procedure_native_1 (crypto_hash_blake3_8) .into ()),
			("crypto-blake3-16", procedure_native_1 (crypto_hash_blake3_16) .into ()),
			("crypto-blake3-32", procedure_native_1 (crypto_hash_blake3_32) .into ()),
			("crypto-blake3-64", procedure_native_1 (crypto_hash_blake3_64) .into ()),
			("crypto-blake3-128", procedure_native_1 (crypto_hash_blake3_128) .into ()),
			("crypto-blake3-192", procedure_native_1 (crypto_hash_blake3_192) .into ()),
			("crypto-blake3-224", procedure_native_1 (crypto_hash_blake3_224) .into ()),
			("crypto-blake3-256", procedure_native_1 (crypto_hash_blake3_256) .into ()),
			("crypto-blake3-320", procedure_native_1 (crypto_hash_blake3_320) .into ()),
			("crypto-blake3-384", procedure_native_1 (crypto_hash_blake3_384) .into ()),
			("crypto-blake3-448", procedure_native_1 (crypto_hash_blake3_448) .into ()),
			("crypto-blake3-512", procedure_native_1 (crypto_hash_blake3_512) .into ()),
			("crypto-blake3-1024", procedure_native_1 (crypto_hash_blake3_1024) .into ()),
			
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
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("random-bytevector", procedure_native_1 (random_generate_bytes_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("random-bytevector-permutation", procedure_native_0 (random_generate_bytes_permutation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-bytevector-append!", procedure_native_2 (random_generate_bytes_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-bytevector-fill!", procedure_native_v (random_generate_bytes_fill_v) .into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-bytevector-shuffle!", procedure_native_v (random_generate_bytes_shuffle_v) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char", procedure_native_0 (random_generate_character_0) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char*", procedure_native_v (random_generate_character_v) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii", procedure_native_0 (random_generate_character_ascii) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-numeric", procedure_native_0 (random_generate_character_ascii_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-numeric-8", procedure_native_0 (random_generate_character_ascii_numeric_base_8) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-numeric-16", procedure_native_0 (random_generate_character_ascii_numeric_base_16) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-alphabetic", procedure_native_0 (random_generate_character_ascii_alphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-upper-case", procedure_native_0 (random_generate_character_ascii_alphabetic_upper_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-lower-case", procedure_native_0 (random_generate_character_ascii_alphabetic_lower_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-alphabetic-or-numeric", procedure_native_0 (random_generate_character_ascii_alphabetic_or_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-whitespace", procedure_native_0 (random_generate_character_ascii_whitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-control", procedure_native_0 (random_generate_character_ascii_control) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-punctuation", procedure_native_0 (random_generate_character_ascii_punctuation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-char-ascii-graphic", procedure_native_0 (random_generate_character_ascii_graphic) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii", procedure_native_1 (random_generate_string_build_ascii) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric", procedure_native_1 (random_generate_string_build_ascii_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric-8", procedure_native_1 (random_generate_string_build_ascii_numeric_base_8) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric-16", procedure_native_1 (random_generate_string_build_ascii_numeric_base_16) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-alphabetic", procedure_native_1 (random_generate_string_build_ascii_alphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-upper-case", procedure_native_1 (random_generate_string_build_ascii_alphabetic_upper_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-lower-case", procedure_native_1 (random_generate_string_build_ascii_alphabetic_lower_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-alphabetic-or-numeric", procedure_native_1 (random_generate_string_build_ascii_alphabetic_or_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-whitespace", procedure_native_1 (random_generate_string_build_ascii_whitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-control", procedure_native_1 (random_generate_string_build_ascii_control) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-punctuation", procedure_native_1 (random_generate_string_build_ascii_punctuation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-graphic", procedure_native_1 (random_generate_string_build_ascii_graphic) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-permutation", procedure_native_0 (random_generate_string_permutation_ascii) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric-8-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric_base_8) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-numeric-16-permutation", procedure_native_0 (random_generate_string_permutation_ascii_numeric_base_16) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-alphabetic-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-upper-case-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_upper_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-lower-case-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_lower_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-alphabetic-or-numeric-permutation", procedure_native_0 (random_generate_string_permutation_ascii_alphabetic_or_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-whitespace-permutation", procedure_native_0 (random_generate_string_permutation_ascii_whitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-control-permutation", procedure_native_0 (random_generate_string_permutation_ascii_control) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-punctuation-permutation", procedure_native_0 (random_generate_string_permutation_ascii_punctuation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			("random-string-ascii-graphic-permutation", procedure_native_0 (random_generate_string_permutation_ascii_graphic) .into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-append!", procedure_native_2 (random_generate_string_extend_ascii) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-numeric-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-numeric-8-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric_base_8) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-numeric-16-append!", procedure_native_2 (random_generate_string_extend_ascii_numeric_base_16) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-alphabetic-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-upper-case-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_upper_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-lower-case-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_lower_case) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-alphabetic-or-numeric-append!", procedure_native_2 (random_generate_string_extend_ascii_alphabetic_or_numeric) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-whitespace-append!", procedure_native_2 (random_generate_string_extend_ascii_whitespace) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-control-append!", procedure_native_2 (random_generate_string_extend_ascii_control) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-punctuation-append!", procedure_native_2 (random_generate_string_extend_ascii_punctuation) .into ()),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("random-string-ascii-graphic-append!", procedure_native_2 (random_generate_string_extend_ascii_graphic) .into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
	definitions.extend_from_slice (&[
			
			("hex-lower-encode", procedure_native_1 (encode_hex_lower_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-lower-encode-append!", procedure_native_2 (encode_hex_lower_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-lower-encode-fill!", procedure_native_2 (encode_hex_lower_fill) .into ()),
			("hex-upper-encode", procedure_native_1 (encode_hex_upper_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-upper-encode-append!", procedure_native_2 (encode_hex_upper_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-upper-encode-fill!", procedure_native_2 (encode_hex_upper_fill) .into ()),
			
			("hex-encode", procedure_native_1 (encode_hex_lower_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-encode-append!", procedure_native_2 (encode_hex_lower_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-encode-fill!", procedure_native_2 (encode_hex_lower_fill) .into ()),
			("hex-decode", procedure_native_1 (decode_hex_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-decode-append!", procedure_native_2 (decode_hex_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("hex-decode-fill!", procedure_native_2 (decode_hex_fill) .into ()),
			
			("base32-encode", procedure_native_1 (encode_base32_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-encode-append!", procedure_native_2 (encode_base32_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-encode-fill!", procedure_native_2 (encode_base32_fill) .into ()),
			("base32-decode", procedure_native_1 (decode_base32_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-decode-append!", procedure_native_2 (decode_base32_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-decode-fill!", procedure_native_2 (decode_base32_fill) .into ()),
			
			("base32-nopad-encode", procedure_native_1 (encode_base32_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-nopad-encode-append!", procedure_native_2 (encode_base32_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-nopad-encode-fill!", procedure_native_2 (encode_base32_nopad_fill) .into ()),
			("base32-nopad-decode", procedure_native_1 (decode_base32_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-nopad-decode-append!", procedure_native_2 (decode_base32_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-nopad-decode-fill!", procedure_native_2 (decode_base32_nopad_fill) .into ()),
			
			("base32-hex-encode", procedure_native_1 (encode_base32_hex_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-encode-append!", procedure_native_2 (encode_base32_hex_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-encode-fill!", procedure_native_2 (encode_base32_hex_fill) .into ()),
			("base32-hex-decode", procedure_native_1 (decode_base32_hex_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-decode-append!", procedure_native_2 (decode_base32_hex_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-decode-fill!", procedure_native_2 (decode_base32_hex_fill) .into ()),
			
			("base32-hex-nopad-encode", procedure_native_1 (encode_base32_hex_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-nopad-encode-append!", procedure_native_2 (encode_base32_hex_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-nopad-encode-fill!", procedure_native_2 (encode_base32_hex_nopad_fill) .into ()),
			("base32-hex-nopad-decode", procedure_native_1 (decode_base32_hex_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-nopad-decode-append!", procedure_native_2 (decode_base32_hex_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base32-hex-nopad-decode-fill!", procedure_native_2 (decode_base32_hex_nopad_fill) .into ()),
			
			("base64-encode", procedure_native_1 (encode_base64_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-encode-append!", procedure_native_2 (encode_base64_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-encode-fill!", procedure_native_2 (encode_base64_fill) .into ()),
			("base64-decode", procedure_native_1 (decode_base64_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-decode-append!", procedure_native_2 (decode_base64_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-decode-fill!", procedure_native_2 (decode_base64_fill) .into ()),
			
			("base64-nopad-encode", procedure_native_1 (encode_base64_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-nopad-encode-append!", procedure_native_2 (encode_base64_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-nopad-encode-fill!", procedure_native_2 (encode_base64_nopad_fill) .into ()),
			("base64-nopad-decode", procedure_native_1 (decode_base64_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-nopad-decode-append!", procedure_native_2 (decode_base64_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-nopad-decode-fill!", procedure_native_2 (decode_base64_nopad_fill) .into ()),
			
			("base64-url-encode", procedure_native_1 (encode_base64_url_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-encode-append!", procedure_native_2 (encode_base64_url_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-encode-fill!", procedure_native_2 (encode_base64_url_fill) .into ()),
			("base64-url-decode", procedure_native_1 (decode_base64_url_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-decode-append!", procedure_native_2 (decode_base64_url_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-decode-fill!", procedure_native_2 (decode_base64_url_fill) .into ()),
			
			("base64-url-nopad-encode", procedure_native_1 (encode_base64_url_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-nopad-encode-append!", procedure_native_2 (encode_base64_url_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-nopad-encode-fill!", procedure_native_2 (encode_base64_url_nopad_fill) .into ()),
			("base64-url-nopad-decode", procedure_native_1 (decode_base64_url_nopad_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-nopad-decode-append!", procedure_native_2 (decode_base64_url_nopad_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-url-nopad-decode-fill!", procedure_native_2 (decode_base64_url_nopad_fill) .into ()),
			
			("base64-mime-encode", procedure_native_1 (encode_base64_mime_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-mime-encode-append!", procedure_native_2 (encode_base64_mime_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-mime-encode-fill!", procedure_native_2 (encode_base64_mime_fill) .into ()),
			("base64-mime-decode", procedure_native_1 (decode_base64_mime_build) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-mime-decode-append!", procedure_native_2 (decode_base64_mime_extend) .into ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			("base64-mime-decode-fill!", procedure_native_2 (decode_base64_mime_fill) .into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	definitions.extend_from_slice (&[
			
			("cache?", TypePrimitiveV::IsCache.into ()),
			("not-cache?", ProcedurePrimitiveV::TypeNegated (TypePrimitiveV::IsCache) .into ()),
			
			("cache-open", RuntimePrimitiveV::CacheOpen.into ()),
			("cache-close", RuntimePrimitive1::CacheClose.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("cache-select-bytevector", RuntimePrimitiveV::CacheSelectBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("cache-include-bytevector", RuntimePrimitiveV::CacheIncludeBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("cache-exclude-bytevector", RuntimePrimitiveV::CacheExcludeBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("cache-resolve-bytevector", RuntimePrimitiveV::CacheResolveBytes.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
			("cache-select", RuntimePrimitiveV::CacheSelectSerde.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
			("cache-include", RuntimePrimitiveV::CacheIncludeSerde.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
			("cache-exclude", RuntimePrimitiveV::CacheExcludeSerde.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
			("cache-resolve", RuntimePrimitiveV::CacheResolveSerde.into ()),
			
			("cache-exclude-all", RuntimePrimitiveV::CacheExcludeAll.into ()),
			("cache-prune-all", RuntimePrimitiveV::CachePruneAll.into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
	definitions.extend_from_slice (&[
			
			("hash", RuntimePrimitive1::DefaultHash.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
			("hash-sip-seeded", RuntimePrimitiveV::SipHashSeeded.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
			("hash-sip-unseeded", RuntimePrimitive1::SipHashUnseeded.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
			("hash-sea-seeded", RuntimePrimitiveV::SeaHashSeeded.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
			("hash-sea-unseeded", RuntimePrimitive1::SeaHashUnseeded.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
			("hash-blake2b-seeded", RuntimePrimitiveV::Blake2bHashSeeded.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
			("hash-blake2b-unseeded", RuntimePrimitive2::Blake2bHashUnseeded.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
			("hash-blake2s-seeded", RuntimePrimitiveV::Blake2sHashSeeded.into ()),
			#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
			("hash-blake2s-unseeded", RuntimePrimitive2::Blake2sHashUnseeded.into ()),
			
		]);
	
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	definitions.extend_from_slice (&[
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("serialize-bytevector", RuntimePrimitive1::SerdeSerializeBytes.into ()),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			("deserialize-bytevector", RuntimePrimitive1::SerdeDeserializeBytes.into ()),
			
		]);
	
	let definitions = vec_map_into! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

