

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::extended_syntaxes::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::native_syntaxes::exports::*;
use super::ports::exports::*;
use super::primitives::exports::*;
use super::processes::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::*;
}




macro_rules! impl_from_for_Expression_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_enum! (Expression, $tag, $from);
	);
}

macro_rules! impl_from_for_Expression_1 {
	( $tag : ident, $from : ty ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_Expression_0! ($tag, $from);
		impl_unwrappers_for_enum_wrapper! (Expression, $tag, $from);
	);
}

/*~~
macro_rules! impl_from_for_Expression_2 {
	( $tag : ident, $from : ty, $value : ident, $expression : tt ) => (
		impl_from_for_type! (Expression, $from, $value, Expression::$tag $expression);
		impl_from_for_Expression_0! ($tag, $from);
	);
}
*/




macro_rules! impl_from_for_Value_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_type! (Value, $from, value, Value::$tag (VALUE_META_1, value.into (), VALUE_META_2));
		impl_from_for_Expression_0! (Value, $from);
	);
}

macro_rules! impl_from_for_Value_1 {
	( $tag : ident, $from : ty ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_2_for_enum_3_wrapper! (Value, $tag, $from);
		impl_try_unwrappers_for_enum_wrapper! (Expression, Value, $from);
	);
}

macro_rules! impl_from_for_Value_2 {
	( $tag : ident, $to : ident, $from : ty ) => (
		// impl_as_ref_for_type! ($from);
		impl_from_for_type! ($to, $from);
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_for_type_wrapper! ($to, $from);
	);
}

macro_rules! impl_from_for_Value_3 {
	( $tag : ident, $to : ty, $from : ty, $value : ident, $expression : expr ) => (
		// impl_as_ref_for_type! ($from);
		impl_from_for_type! ($to, $from, $value, $expression);
		impl_from_for_Value_0! ($tag, $from);
	);
}




impl_as_ref_for_type! (Expression);
impl_into_for_outcome! (Expression);

impl_from_for_Expression_1! (Value, Value);
impl_from_for_Expression_1! (Contexts, ExpressionForContexts);
impl_from_for_Expression_1! (ProcedureGenericCall, ExpressionForProcedureGenericCall);
impl_from_for_Expression_1! (ProcedurePrimitiveCall, ExpressionForProcedurePrimitiveCall);
impl_from_for_Expression_1! (ProcedureExtendedCall, ExpressionForProcedureExtendedCall);
impl_from_for_Expression_1! (ProcedureLambdaCall, ExpressionForProcedureLambdaCall);
impl_from_for_Expression_1! (ProcedureNativeCall, ExpressionForProcedureNativeCall);




// impl_as_ref_for_type! (Value);
impl_into_for_outcome! (Value);

impl_from_for_Value_1! (Singleton, ValueSingleton);
impl_from_for_Value_1! (Boolean, Boolean);
impl_from_for_Value_1! (NumberInteger, NumberInteger);
impl_from_for_Value_1! (NumberReal, NumberReal);
impl_from_for_Value_1! (Character, Character);
impl_from_for_Value_1! (Symbol, Symbol);
impl_from_for_Value_1! (Keyword, Keyword);
impl_from_for_Value_1! (Unique, Unique);
impl_from_for_Value_1! (StringRegex, StringRegex);
impl_from_for_Value_1! (StringImmutable, StringImmutable);
impl_from_for_Value_1! (StringMutable, StringMutable);
impl_from_for_Value_1! (BytesImmutable, BytesImmutable);
impl_from_for_Value_1! (BytesMutable, BytesMutable);
impl_from_for_Value_1! (PairImmutable, PairImmutable);
impl_from_for_Value_1! (PairMutable, PairMutable);
impl_from_for_Value_1! (ArrayImmutable, ArrayImmutable);
impl_from_for_Value_1! (ArrayMutable, ArrayMutable);
impl_from_for_Value_1! (Values, Values);
impl_from_for_Value_1! (RecordKind, RecordKind);
impl_from_for_Value_1! (RecordImmutable, RecordImmutable);
impl_from_for_Value_1! (RecordMutable, RecordMutable);
impl_from_for_Value_1! (Error, Error);
impl_from_for_Value_1! (ProcedurePrimitive, ProcedurePrimitive);
impl_from_for_Value_1! (ProcedureExtended, ProcedureExtended);
impl_from_for_Value_1! (ProcedureNative, ProcedureNative);
impl_from_for_Value_1! (ProcedureLambda, ProcedureLambda);
impl_from_for_Value_1! (SyntaxPrimitive, SyntaxPrimitive);
impl_from_for_Value_1! (SyntaxExtended, SyntaxExtended);
impl_from_for_Value_1! (SyntaxNative, SyntaxNative);
impl_from_for_Value_1! (SyntaxLambda, SyntaxLambda);
impl_from_for_Value_1! (Port, Port);
impl_from_for_Value_1! (Process, Process);
impl_from_for_Value_1! (Context, Context);
impl_from_for_Value_1! (Binding, Binding);
impl_from_for_Value_1! (Parameters, Parameters);
impl_from_for_Value_1! (Parameter, Parameter);
impl_from_for_Value_1! (Promise, Promise);


impl_from_for_Value_3! (Singleton, ValueSingleton, (), _value, ValueSingleton::Void);

impl_from_for_Value_2! (Boolean, Boolean, bool);
impl_from_for_Value_2! (Character, Character, char);

impl_from_for_Value_2! (NumberInteger, NumberInteger, i64);
impl_from_for_Value_3! (NumberInteger, NumberInteger, i8, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u8, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i16, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u16, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i32, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u32, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, isize, value, number_i64 (value as i64));
impl_try_from_for_type! (NumberInteger, u64, value, if value <= <i64>::max_value () as u64 { succeeded! (number_i64 (value as i64)) } else { failed! (0x78f55fb6) });
impl_try_from_for_type! (NumberInteger, usize, value, if value <= <i64>::max_value () as usize { succeeded! (number_i64 (value as i64)) } else { failed! (0xe99641f7) });
impl_try_from_for_type! (Value, u64, value, StdTryInto::<NumberInteger>::try_into (value) .into ());
impl_try_from_for_type! (Value, usize, value, StdTryInto::<NumberInteger>::try_into (value) .into ());
impl_from_for_type! (NumberInteger, char, value, number_i64 (value as i64));

impl_from_for_Value_2! (NumberReal, NumberReal, f64);
impl_from_for_type! (NumberReal, NumberInteger, value, <i64>::from (value) .into ());
impl_from_for_type! (NumberReal, f32, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, i8, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, u8, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, i16, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, u16, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, i32, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, u32, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, i64, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, u64, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, isize, value, number_f64 (value as f64));
impl_from_for_type! (NumberReal, usize, value, number_f64 (value as f64));

impl_from_for_Value_3! (StringImmutable, StringImmutable, StdString, value, string_immutable_new (value));
impl_from_for_Value_3! (StringImmutable, StringImmutable, &'static str, value, string_immutable_clone_str (value));

impl_from_for_type! (StringMutable, StdString, value, string_mutable_new (value));
impl_from_for_type! (StringMutable, &'static str, value, string_mutable_clone_str (value));

impl_from_for_type! (Symbol, StdString, value, symbol_new (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_clone_str (value));

impl_from_for_type! (Keyword, StdString, value, keyword_new (value));
impl_from_for_type! (Keyword, &'static str, value, keyword_clone_str (value));

impl_from_for_Value_3! (Unique, Unique, UniqueData, data, Unique::new (data));

impl_from_for_type! (PairImmutable, (Value, Value), value, { let (left, right) = value; pair_immutable_new (left, right) });
impl_from_for_type! (PairMutable, (Value, Value), value, { let (left, right) = value; pair_mutable_new (left, right) });

impl_from_for_Value_3! (ProcedureExtended, ProcedureExtended, ProcedureExtendedInternals, internals, ProcedureExtended::new (internals));
impl_from_for_Value_3! (SyntaxExtended, SyntaxExtended, SyntaxExtendedInternals, internals, SyntaxExtended::new (internals));

impl_from_for_Value_3! (ProcedureNative, ProcedureNative, ProcedureNativeInternals, internals, ProcedureNative::new (internals));
impl_from_for_Value_3! (SyntaxNative, SyntaxNative, SyntaxNativeInternals, internals, SyntaxNative::new (internals));




impl_as_ref_for_type_wlt! (StringRef<'a>, 'a);
impl_as_ref_for_type_wlt! (BytesRef<'a>, 'a);
impl_as_ref_for_type_wlt! (PairRef<'a>, 'a);
impl_as_ref_for_type_wlt! (ArrayRef<'a>, 'a);
impl_as_ref_for_type_wlt! (RecordRef<'a>, 'a);




impl_from_for_type! (Value, ProcessStatus, status, status.value ());




// FIXME:  Implement this for all that implement `Into<T>`!
impl_into_for_outcome! (bool);




macro_rules! impl_from_for_primitive_procedure_1 {
	( $from : ty, $tag : ident ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_enum! (ProcedurePrimitive, $tag, $from);
		impl_from_for_Value_0! (ProcedurePrimitive, $from);
		impl_unwrappers_for_enum_wrapper! (ProcedurePrimitive, $tag, $from);
	);
}


impl_from_for_primitive_procedure_1! (ProcedurePrimitive0, Primitive0);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive1, Primitive1);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive2, Primitive2);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive3, Primitive3);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive4, Primitive4);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive5, Primitive5);
impl_from_for_primitive_procedure_1! (ProcedurePrimitiveN, PrimitiveN);
impl_from_for_primitive_procedure_1! (ProcedurePrimitiveV, PrimitiveV);




macro_rules! impl_from_for_primitive_syntax {
	( $from : ty, $tag : ident ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_enum! (SyntaxPrimitive, $tag, $from);
		impl_from_for_Value_0! (SyntaxPrimitive, $from);
		impl_unwrappers_for_enum_wrapper! (SyntaxPrimitive, $tag, $from);
	);
}

impl_from_for_primitive_syntax! (SyntaxPrimitiveV, PrimitiveV);




macro_rules! impl_from_for_native_procedure_1 {
	( $from : ident, $from_fn : ty, $tag : ident, $coercer : ident ) => (
		impl_from_for_enum! (ProcedureNativeInternals, $tag, $from);
		impl_from_for_Value_3! (ProcedureNative, ProcedureNative, $from, native, ProcedureNativeInternals::$tag (native) .into ());
		impl_from_for_Value_3! (ProcedureNative, ProcedureNative, $from_fn, native, $coercer (native) .into ());
		impl_unwrappers_for_enum_wrapper! (ProcedureNativeInternals, $tag, $from);
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $coercer (native : $from_fn) -> ($from) {
			$from (native)
		}
	);
}

impl_from_for_native_procedure_1! (ProcedureNative0, ProcedureNativeFn0, Native0, procedure_native_0);
impl_from_for_native_procedure_1! (ProcedureNative1, ProcedureNativeFn1, Native1, procedure_native_1);
impl_from_for_native_procedure_1! (ProcedureNative2, ProcedureNativeFn2, Native2, procedure_native_2);
impl_from_for_native_procedure_1! (ProcedureNative3, ProcedureNativeFn3, Native3, procedure_native_3);
impl_from_for_native_procedure_1! (ProcedureNative4, ProcedureNativeFn4, Native4, procedure_native_4);
impl_from_for_native_procedure_1! (ProcedureNative5, ProcedureNativeFn5, Native5, procedure_native_5);
impl_from_for_native_procedure_1! (ProcedureNativeN, ProcedureNativeFnN, NativeN, procedure_native_n);

impl_from_for_native_procedure_1! (ProcedureNative0E, ProcedureNativeFn0E, Native0E, procedure_native_0e);
impl_from_for_native_procedure_1! (ProcedureNative1E, ProcedureNativeFn1E, Native1E, procedure_native_1e);
impl_from_for_native_procedure_1! (ProcedureNative2E, ProcedureNativeFn2E, Native2E, procedure_native_2e);
impl_from_for_native_procedure_1! (ProcedureNative3E, ProcedureNativeFn3E, Native3E, procedure_native_3e);
impl_from_for_native_procedure_1! (ProcedureNative4E, ProcedureNativeFn4E, Native4E, procedure_native_4e);
impl_from_for_native_procedure_1! (ProcedureNative5E, ProcedureNativeFn5E, Native5E, procedure_native_5e);
impl_from_for_native_procedure_1! (ProcedureNativeNE, ProcedureNativeFnNE, NativeNE, procedure_native_ne);

impl_from_for_native_procedure_1! (ProcedureNativeV, ProcedureNativeFnV, NativeV, procedure_native_v);




macro_rules! impl_from_for_native_syntax_1 {
	( $from : ident, $from_fn : ty, $tag : ident, $coercer : ident ) => (
		impl_from_for_enum! (SyntaxNativeInternals, $tag, $from);
		impl_from_for_Value_3! (SyntaxNative, SyntaxNative, $from, native, SyntaxNativeInternals::$tag (native) .into ());
		impl_from_for_Value_3! (SyntaxNative, SyntaxNative, $from_fn, native, $coercer (native) .into ());
		// FIXME:  `rustc --explain E0162`
		// impl_unwrappers_for_enum_wrapper! (SyntaxNativeInternals, $tag, $from);
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $coercer (native : $from_fn) -> ($from) {
			$from (native)
		}
	);
}

impl_from_for_native_syntax_1! (SyntaxNativeG, SyntaxNativeFnG, NativeG, syntax_native_g);




macro_rules! impl_from_for_primitive_procedure_2 {
	( $from : ty, $tag_1 : ident, $tag_2 : ident, $tag_3 : ident ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_enum! ($tag_1, $tag_3, $from);
		impl_from_for_enum! (ProcedurePrimitive, $tag_2, $from);
		impl_from_for_Value_0! (ProcedurePrimitive, $from);
		impl_unwrappers_for_enum_wrapper! ($tag_1, $tag_3, $from);
	);
}

impl_from_for_primitive_procedure_2! (TypePrimitive0, ProcedurePrimitive0, Primitive0, Type);
impl_from_for_primitive_procedure_2! (TypePrimitive1, ProcedurePrimitive1, Primitive1, Type);
impl_from_for_primitive_procedure_2! (TypePrimitive2, ProcedurePrimitive2, Primitive2, Type);
impl_from_for_primitive_procedure_2! (TypePrimitive3, ProcedurePrimitive3, Primitive3, Type);
impl_from_for_primitive_procedure_2! (TypePrimitive4, ProcedurePrimitive4, Primitive4, Type);
impl_from_for_primitive_procedure_2! (TypePrimitive5, ProcedurePrimitive5, Primitive5, Type);
impl_from_for_primitive_procedure_2! (TypePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Type);
impl_from_for_primitive_procedure_2! (TypePrimitiveV, ProcedurePrimitiveV, PrimitiveV, Type);

impl_from_for_primitive_procedure_2! (BooleanPrimitive0, ProcedurePrimitive0, Primitive0, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive1, ProcedurePrimitive1, Primitive1, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive2, ProcedurePrimitive2, Primitive2, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive3, ProcedurePrimitive3, Primitive3, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive4, ProcedurePrimitive4, Primitive4, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive5, ProcedurePrimitive5, Primitive5, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Boolean);

impl_from_for_primitive_procedure_2! (ArithmeticPrimitive0, ProcedurePrimitive0, Primitive0, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive1, ProcedurePrimitive1, Primitive1, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive2, ProcedurePrimitive2, Primitive2, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive3, ProcedurePrimitive3, Primitive3, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive4, ProcedurePrimitive4, Primitive4, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive5, ProcedurePrimitive5, Primitive5, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Arithmetic);

impl_from_for_primitive_procedure_2! (BitwisePrimitive0, ProcedurePrimitive0, Primitive0, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive1, ProcedurePrimitive1, Primitive1, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive2, ProcedurePrimitive2, Primitive2, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive3, ProcedurePrimitive3, Primitive3, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive4, ProcedurePrimitive4, Primitive4, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive5, ProcedurePrimitive5, Primitive5, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitiveV, ProcedurePrimitiveV, PrimitiveV, Bitwise);

impl_from_for_primitive_procedure_2! (ComparisonPrimitive0, ProcedurePrimitive0, Primitive0, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive1, ProcedurePrimitive1, Primitive1, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive2, ProcedurePrimitive2, Primitive2, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive3, ProcedurePrimitive3, Primitive3, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive4, ProcedurePrimitive4, Primitive4, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive5, ProcedurePrimitive5, Primitive5, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Comparison);

impl_from_for_primitive_procedure_2! (ListPrimitive0, ProcedurePrimitive0, Primitive0, List);
impl_from_for_primitive_procedure_2! (ListPrimitive1, ProcedurePrimitive1, Primitive1, List);
impl_from_for_primitive_procedure_2! (ListPrimitive2, ProcedurePrimitive2, Primitive2, List);
impl_from_for_primitive_procedure_2! (ListPrimitive3, ProcedurePrimitive3, Primitive3, List);
impl_from_for_primitive_procedure_2! (ListPrimitive4, ProcedurePrimitive4, Primitive4, List);
impl_from_for_primitive_procedure_2! (ListPrimitive5, ProcedurePrimitive5, Primitive5, List);
impl_from_for_primitive_procedure_2! (ListPrimitiveN, ProcedurePrimitiveN, PrimitiveN, List);
impl_from_for_primitive_procedure_2! (ListPrimitiveV, ProcedurePrimitiveV, PrimitiveV, List);

impl_from_for_primitive_procedure_2! (ArrayPrimitive0, ProcedurePrimitive0, Primitive0, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive1, ProcedurePrimitive1, Primitive1, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive2, ProcedurePrimitive2, Primitive2, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive3, ProcedurePrimitive3, Primitive3, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive4, ProcedurePrimitive4, Primitive4, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive5, ProcedurePrimitive5, Primitive5, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Array);

impl_from_for_primitive_procedure_2! (BytesPrimitive0, ProcedurePrimitive0, Primitive0, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive1, ProcedurePrimitive1, Primitive1, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive2, ProcedurePrimitive2, Primitive2, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive3, ProcedurePrimitive3, Primitive3, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive4, ProcedurePrimitive4, Primitive4, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive5, ProcedurePrimitive5, Primitive5, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Bytes);

impl_from_for_primitive_procedure_2! (StringPrimitive0, ProcedurePrimitive0, Primitive0, String);
impl_from_for_primitive_procedure_2! (StringPrimitive1, ProcedurePrimitive1, Primitive1, String);
impl_from_for_primitive_procedure_2! (StringPrimitive2, ProcedurePrimitive2, Primitive2, String);
impl_from_for_primitive_procedure_2! (StringPrimitive3, ProcedurePrimitive3, Primitive3, String);
impl_from_for_primitive_procedure_2! (StringPrimitive4, ProcedurePrimitive4, Primitive4, String);
impl_from_for_primitive_procedure_2! (StringPrimitive5, ProcedurePrimitive5, Primitive5, String);
impl_from_for_primitive_procedure_2! (StringPrimitiveN, ProcedurePrimitiveN, PrimitiveN, String);
impl_from_for_primitive_procedure_2! (StringPrimitiveV, ProcedurePrimitiveV, PrimitiveV, String);

impl_from_for_primitive_procedure_2! (FunctionsPrimitive0, ProcedurePrimitive0, Primitive0, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive1, ProcedurePrimitive1, Primitive1, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive2, ProcedurePrimitive2, Primitive2, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive3, ProcedurePrimitive3, Primitive3, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive4, ProcedurePrimitive4, Primitive4, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive5, ProcedurePrimitive5, Primitive5, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Functions);

impl_from_for_primitive_procedure_2! (RecordPrimitive0, ProcedurePrimitive0, Primitive0, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitive1, ProcedurePrimitive1, Primitive1, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitive2, ProcedurePrimitive2, Primitive2, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitive3, ProcedurePrimitive3, Primitive3, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitive4, ProcedurePrimitive4, Primitive4, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitive5, ProcedurePrimitive5, Primitive5, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Record);
impl_from_for_primitive_procedure_2! (RecordPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Record);

impl_from_for_primitive_procedure_2! (RuntimePrimitive0, ProcedurePrimitive0, Primitive0, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive1, ProcedurePrimitive1, Primitive1, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive2, ProcedurePrimitive2, Primitive2, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive3, ProcedurePrimitive3, Primitive3, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive4, ProcedurePrimitive4, Primitive4, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive5, ProcedurePrimitive5, Primitive5, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitiveV, ProcedurePrimitiveV, PrimitiveV, Runtime);

impl_from_for_primitive_procedure_2! (PortPrimitive0, ProcedurePrimitive0, Primitive0, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive1, ProcedurePrimitive1, Primitive1, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive2, ProcedurePrimitive2, Primitive2, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive3, ProcedurePrimitive3, Primitive3, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive4, ProcedurePrimitive4, Primitive4, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive5, ProcedurePrimitive5, Primitive5, Port);
impl_from_for_primitive_procedure_2! (PortPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Port);
impl_from_for_primitive_procedure_2! (PortPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Port);




/* ----- * /

macro_rules! impl_from_for_ProcedurePrimitiveCall0 {
	( $primitive : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall0, ($primitive,), value, (value.0.into (),));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCall1 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall1! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall1! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall1, ($primitive, $value), value, (value.0.into (), value.1.into ()));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCall2 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall2! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall2! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall2, ($primitive, $value, $value), value, (value.0.into (), value.1.into (), value.2.into ()));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall2, ($primitive, [$value; 2]), value, (value.0.into (), (value.1)[0].clone () .into (), (value.1)[1].clone ().into ()));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCall3 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall3! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall3! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall3, ($primitive, $value, $value, $value), value, (value.0.into (), vec! [value.1.into (), value.2.into (), value.3.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall3, ($primitive, [$value; 3]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCall4 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall4! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall4! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall4, ($primitive, $value, $value, $value, $value), value, (value.0.into (), vec! [value.1.into (), value.2.into (), value.3.into (), value.4.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall4, ($primitive, [$value; 4]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCall5 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall5! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall5! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCall5, ($primitive, $value, $value, $value, $value, $value), value, (value.0.into (), vec! [value.1.into (), value.2.into (), value.3.into (), value.4.into (), value.5.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall5, ($primitive, [$value; 5]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCallN {
	( $primitive : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive,), value, (value.0.into (), vec! []));
		impl_from_for_ProcedurePrimitiveCallN! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCallN! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, StdVec<$value>), value, (value.0.into (), vec_into (value.1)));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, &'static [$value]), value, (value.0.into (), vec_into (value.1.to_owned ())));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value,)), value, (value.0.into (), vec! [(value.1).0.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value, $value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into (), (value.1).4.into ()]));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 1]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 2]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 3]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 4]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 5]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		
	);
}

macro_rules! impl_from_for_ProcedurePrimitiveCallV {
	( $primitive : ty ) => (
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive,), value, (value.0.into (), vec! []));
		impl_from_for_ProcedurePrimitiveCallV! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, Keyword, Unique,
				StringRegex, StringImmutable, StringMutable, BytesImmutable, BytesMutable,
				PairImmutable, PairMutable, ArrayImmutable, ArrayMutable, Values,
				RecordKind, RecordImmutable, RecordMutable,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCallV! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, StdVec<$value>), value, (value.0.into (), vec_into (value.1)));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, &'static [$value]), value, (value.0.into (), vec_into (value.1.to_owned ())));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, ($value,)), value, (value.0.into (), vec! [(value.1).0.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, ($value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, ($value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, ($value, $value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, ($value, $value, $value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into (), (value.1).4.into ()]));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, [$value; 1]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, [$value; 2]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, [$value; 3]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, [$value; 4]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallV, ($primitive, [$value; 5]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		
	);
}


impl_from_for_ProcedurePrimitiveCall0! (TypePrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (TypePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (TypePrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (TypePrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (TypePrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (TypePrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (TypePrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (TypePrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (BooleanPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BooleanPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BooleanPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BooleanPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BooleanPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BooleanPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BooleanPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (BooleanPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (ArithmeticPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ArithmeticPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArithmeticPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ArithmeticPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ArithmeticPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ArithmeticPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ArithmeticPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (ArithmeticPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (BitwisePrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BitwisePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BitwisePrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BitwisePrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BitwisePrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BitwisePrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BitwisePrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (BitwisePrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (ComparisonPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ComparisonPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ComparisonPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ComparisonPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ComparisonPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ComparisonPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ComparisonPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (ComparisonPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (ListPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ListPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ListPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ListPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ListPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ListPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ListPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (ListPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (ArrayPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ArrayPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArrayPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ArrayPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ArrayPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ArrayPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ArrayPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (ArrayPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (BytesPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BytesPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BytesPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BytesPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BytesPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BytesPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BytesPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (BytesPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (StringPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (StringPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (StringPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (StringPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (StringPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (StringPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (StringPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (StringPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (FunctionsPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (FunctionsPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (FunctionsPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (FunctionsPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (FunctionsPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (FunctionsPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (FunctionsPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (FunctionsPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (RecordPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (RecordPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (RecordPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (RecordPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (RecordPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (RecordPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (RecordPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (RecordPrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (RuntimePrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (RuntimePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (RuntimePrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (RuntimePrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (RuntimePrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (RuntimePrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (RuntimePrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (RuntimePrimitiveV);

impl_from_for_ProcedurePrimitiveCall0! (PortPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (PortPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (PortPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (PortPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (PortPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (PortPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (PortPrimitiveN);
impl_from_for_ProcedurePrimitiveCallV! (PortPrimitiveV);

/ * ----- */




#[ derive (Clone, Debug) ]
pub enum NumberCoercion1 {
	Integer ( i64 ),
	Real ( f64 ),
}

#[ derive (Clone, Debug) ]
pub enum NumberCoercion2 {
	Integer ( i64, i64 ),
	Real ( f64, f64 ),
}


impl NumberCoercion1 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_value (self) -> (Value) {
		match self {
			NumberCoercion1::Integer (number) =>
				number.into (),
			NumberCoercion1::Real (number) =>
				number.into (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_to_i64 (self) -> (Outcome<i64>) {
		match self {
			NumberCoercion1::Integer (number) =>
				succeed! (number),
			NumberCoercion1::Real (_) =>
				// TODO:  Implement the same method as in `NumberReal::try_to_i64`!
				fail! (0x53b7b5c8),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn try_to_f64 (self) -> (Outcome<f64>) {
		match self {
			NumberCoercion1::Integer (number) =>
				succeed! (number as f64),
			NumberCoercion1::Real (number) =>
				succeed! (number),
		}
	}
}


impl NumberCoercion2 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_values (self) -> ((Value, Value)) {
		match self {
			NumberCoercion2::Integer (number_1, number_2) =>
				(number_1.into (), number_2.into ()),
			NumberCoercion2::Real (number_1, number_2) =>
				(number_1.into (), number_2.into ()),
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_1a (value : &Value) -> (Outcome<NumberCoercion1>) {
	let class = value.class_match_as_ref ();
	return number_coerce_1d (&class);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_1d <'a> (class : &ValueClassMatchAsRef<'a>) -> (Outcome<NumberCoercion1>) {
	match *class {
		ValueClassMatchAsRef::Number (ref class) =>
			succeed! (number_coerce_1e (class)),
		_ =>
			failed! (0x5539630a),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_1e <'a> (class : &NumberMatchAsRef<'a>) -> (NumberCoercion1) {
	match *class {
		NumberMatchAsRef::Integer (value) =>
			NumberCoercion1::Integer (value.value ()),
		NumberMatchAsRef::Real (value) =>
			NumberCoercion1::Real (value.value ()),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_2a (left : &Value, right : &Value) -> (Outcome<NumberCoercion2>) {
	let class = Value::class_match_as_ref_2 (left, right);
	return number_coerce_2d (&class);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_2b (left : &NumberCoercion1, right : &Value) -> (Outcome<NumberCoercion2>) {
	let right = try! (number_coerce_1a (right));
	succeed! (number_coerce_2c (left, &right));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_2c (left : &NumberCoercion1, right : &NumberCoercion1) -> (NumberCoercion2) {
	match (left, right) {
		(&NumberCoercion1::Integer (left), &NumberCoercion1::Integer (right)) =>
			NumberCoercion2::Integer (left, right),
		(&NumberCoercion1::Real (left), &NumberCoercion1::Real (right)) =>
			NumberCoercion2::Real (left, right),
		(&NumberCoercion1::Integer (left), &NumberCoercion1::Real (right)) =>
			NumberCoercion2::Real (left as f64, right),
		(&NumberCoercion1::Real (left), &NumberCoercion1::Integer (right)) =>
			NumberCoercion2::Real (left, right as f64),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_2d <'a> (class : &ValueClassMatchAsRef2<'a>) -> (Outcome<NumberCoercion2>) {
	match *class {
		ValueClassMatchAsRef2::Number (ref class) =>
			succeed! (number_coerce_2e (class)),
		_ =>
			failed! (0x6cfbdd37),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_coerce_2e <'a> (class : &NumberMatchAsRef2<'a>) -> (NumberCoercion2) {
	match *class {
		NumberMatchAsRef2::IntegerBoth (left, right) =>
			NumberCoercion2::Integer (left.value (), right.value ()),
		NumberMatchAsRef2::RealBoth (left, right) =>
			NumberCoercion2::Real (left.value (), right.value ()),
		NumberMatchAsRef2::IntegerAndReal (left, right) =>
			NumberCoercion2::Real (left.value () as f64, right.value ()),
		NumberMatchAsRef2::RealAndInteger (left, right) =>
			NumberCoercion2::Real (left.value (), right.value () as f64),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn range_coerce (start : Option<&Value>, end : Option<&Value>, length : usize) -> (Outcome<(usize, usize)>) {
	let (start, end) = try! (range_coerce_unbounded (start, end));
	let end = end.unwrap_or (length);
	if start > length {
		fail! (0x16e64120);
	}
	if end > length {
		fail! (0x440b8499);
	}
	succeed! ((start, end));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn range_coerce_unbounded (start : Option<&Value>, end : Option<&Value>) -> (Outcome<(usize, Option<usize>)>) {
	let start = if let Some (start) = start {
		try! (try_as_number_integer_ref! (start) .try_to_usize ())
	} else {
		0
	};
	let end = if let Some (end) = end {
		let end = try! (try_as_number_integer_ref! (end) .try_to_usize ());
		if start > end {
			fail! (0x49a6ab02);
		}
		Some (end)
	} else {
		None
	};
	succeed! ((start, end));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn count_coerce (count : Option<&Value>) -> (Outcome<Option<usize>>) {
	let count = if let Some (count) = count {
		Some (try! (try_as_number_integer_ref! (count) .try_to_usize ()))
	} else {
		None
	};
	succeed! (count);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn boolean_coerce (boolean : Option<&Value>) -> (Outcome<Option<bool>>) {
	let boolean = if let Some (boolean) = boolean {
		Some (try_as_boolean_ref! (boolean) .value ())
	} else {
		None
	};
	succeed! (boolean);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_coerce (string : Option<&Value>) -> (Outcome<Option<StdString>>) {
	let string = if let Some (string) = string {
		Some (try_as_string_ref! (string) .string_clone ())
	} else {
		None
	};
	succeed! (string);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn outcome_as_ref <T> (outcome : &Outcome<T>) -> (Outcome<&T>) {
	match *outcome {
		Ok (ref value) =>
			Ok (value),
		Err (ref error) =>
			Err (error.clone ()),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn option_unwrap_ref <T> (option : &Option<T>) -> (&T) {
	match *option {
		Some (ref value) =>
			value,
		None =>
			panic! ("652a4430"),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn option_box_new <T> (option : Option<T>) -> (Option<StdBox<T>>) {
	match option {
		Some (value) =>
			Some (StdBox::new (value)),
		None =>
			None,
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn option_box_unwrap <T> (option : Option<StdBox<T>>) -> (T) {
	match option {
		Some (value) =>
			*value,
		None =>
			panic! ("75a8fcd5"),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn option_box_as_ref <T : ?Sized> (option : &Option<StdBox<T>>) -> (Option<&T>) {
	match *option {
		Some (ref value) =>
			Some (&value),
		None =>
			None,
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn option_box_into_owned <T> (value : Option<StdBox<T>>) -> (Option<T>) {
	match value {
		Some (value) =>
			Some (*value),
		None =>
			None,
	}
}




#[ derive (Debug) ]
pub enum BytesSliceRef <'a> {
	Immutable ( &'a [u8] ),
	Mutable ( StdRef<'a, [u8]> ),
}


impl <'a> BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn range (self, range_start : usize, range_end : Option<usize>) -> (Option<BytesSliceRef<'a>>) {
		if let Some (range_end) = range_end {
			self.slice (range_start .. range_end)
		} else {
			self.slice (range_start ..)
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn slice <Slice> (self, slice : Slice) -> (Option<BytesSliceRef<'a>>)
		where Slice : slice::SliceIndex<[u8], Output = [u8]> + Clone
	{
		match self {
			BytesSliceRef::Immutable (reference) =>
				if let Some (reference) = reference.get (slice) {
					Some (BytesSliceRef::Immutable (reference))
				} else {
					None
				},
			BytesSliceRef::Mutable (reference) => {
				// TODO:  Try to call `get` only once!
				if let Some (_) = reference.get (slice.clone ()) {
					let reference = StdRef::map (reference, |reference| reference.get (slice) .unwrap ());
					Some (BytesSliceRef::Mutable (reference))
				} else {
					None
				}
			},
		}
	}
}


impl <'a> StdDeref for BytesSliceRef<'a> {
	
	type Target = [u8];
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&[u8]) {
		match *self {
			BytesSliceRef::Immutable (reference) =>
				reference,
			BytesSliceRef::Mutable (ref reference) =>
				&reference,
		}
	}
}




impl <'a> StdFrom<&'a [u8]> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a [u8]) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference)
	}
}

impl <'a> StdFrom<&'a StdBox<[u8]>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a StdBox<[u8]>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_ref ())
	}
}

impl <'a> StdFrom<&'a StdVec<u8>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a StdVec<u8>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_ref ())
	}
}

impl <'a> StdFrom<StdRef<'a, [u8]>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, [u8]>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference))
	}
}

impl <'a> StdFrom<StdRef<'a, StdBox<[u8]>>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, StdBox<[u8]>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}

impl <'a> StdFrom<StdRef<'a, StdVec<u8>>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, StdVec<u8>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}

impl <'a> StdFrom<StdRef<'a, BytesMutableInternals>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, BytesMutableInternals>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}




impl <'a> StdFrom<&'a str> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a str) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

impl <'a> StdFrom<&'a StdBox<str>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a StdBox<str>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

impl <'a> StdFrom<&'a StdString> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : &'a StdString) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

impl <'a> StdFrom<StdRef<'a, str>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, str>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

impl <'a> StdFrom<StdRef<'a, StdBox<str>>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, StdBox<str>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

impl <'a> StdFrom<StdRef<'a, StdString>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, StdString>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

impl <'a> StdFrom<StdRef<'a, StringMutableInternals>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StdRef<'a, StringMutableInternals>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref () .as_bytes ()))
	}
}




impl <'a> StdFrom<BytesRef<'a>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : BytesRef<'a>) -> (BytesSliceRef<'a>) {
		match reference {
			BytesRef::Immutable (_, reference) =>
				reference.into (),
			BytesRef::Mutable (_, reference) =>
				reference.into (),
		}
	}
}

impl <'a> StdFrom<StringRef<'a>> for BytesSliceRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (reference : StringRef<'a>) -> (BytesSliceRef<'a>) {
		match reference {
			StringRef::Immutable (_, reference) =>
				reference.into (),
			StringRef::Mutable (_, reference) =>
				reference.into (),
		}
	}
}




/*
impl <From, To> StdInto0<Outcome<To>> for From where From : StdInto<To> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn into_0 (self) -> (Outcome<To>) {
		return Ok (self.into ());
	}
}

impl <From, To> StdInto0<Outcome<To>> for Outcome<From> where From : StdInto<To> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn into_0 (self) -> (Outcome<To>) {
		match self {
			Ok (value) =>
				return Ok (value.into ()),
			Err (error) =>
				return Err (error),
		}
	}
}


impl <From, To> StdTryInto0<To> for From where From : StdTryInto<To, Error = super::errors::exports::Error> {
	
	type Error = super::errors::exports::Error;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_into_0 (self) -> (Result<To, Self::Error>) {
		return self.try_into ();
	}
}

impl <From, To> StdTryInto0<To> for Outcome<From> where From : StdTryInto<To, Error = super::errors::exports::Error> {
	
	type Error = super::errors::exports::Error;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_into_0 (self) -> (Result<To, Self::Error>) {
		match self {
			Ok (value) =>
				return value.try_into (),
			Err (error) =>
				return Err (error),
		}
	}
}
*/

