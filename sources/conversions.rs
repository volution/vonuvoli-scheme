

use super::builtins::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
use super::expressions::exports::*;

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
use super::contexts::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
use super::ports::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
use super::processes::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::*;
}




#[ cfg ( feature = "vonuvoli_expressions" ) ]
macro_rules! impl_from_for_Expression_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_enum! (Expression, $tag, $from);
	);
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
macro_rules! impl_from_for_Expression_1 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_Expression_0! ($tag, $from);
		impl_unwrappers_for_enum_wrapper! (Expression, $tag, $from);
	);
}

/*~~
#[ cfg ( feature = "vonuvoli_expressions" ) ]
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
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		impl_from_for_Expression_0! (Value, $from);
	);
}

macro_rules! impl_from_for_Value_1 {
	( $tag : ident, $from : ty ) => (
		impl_as_ref_for_type! ($from);
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_2_for_enum_3_wrapper! (Value, $tag, $from);
		#[ cfg ( feature = "vonuvoli_expressions" ) ]
		impl_try_unwrappers_for_enum_wrapper! (Expression, Value, $from);
	);
}

macro_rules! impl_from_for_Value_2 {
	( $tag : ident, $to : ident, $from : ty ) => (
		impl_from_for_type! ($to, $from);
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_for_type_wrapper! ($to, $from);
	);
}

macro_rules! impl_from_for_Value_3 {
	( $tag : ident, $to : ty, $from : ty, $value : ident, $expression : expr ) => (
		impl_from_for_type! ($to, $from, $value, $expression);
		impl_from_for_Value_0! ($tag, $from);
	);
}





#[ cfg ( feature = "vonuvoli_expressions" ) ]
impl_as_ref_for_type! (Expression);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
impl_into_for_outcome! (Expression);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
impl_from_for_Expression_1! (Value, Value);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
impl_from_for_Expression_1! (Contexts, ExpressionForContexts);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
impl_from_for_Expression_1! (ProcedureGenericCall, ExpressionForProcedureGenericCall);
#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
impl_from_for_Expression_1! (ProcedurePrimitiveCall, ExpressionForProcedurePrimitiveCall);
#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_from_for_Expression_1! (ProcedureExtendedCall, ExpressionForProcedureExtendedCall);
#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_from_for_Expression_1! (ProcedureLambdaCall, ExpressionForProcedureLambdaCall);
#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_Expression_1! (ProcedureNativeCall, ExpressionForProcedureNativeCall);




impl_as_ref_for_type! (Value);
impl_into_for_outcome! (Value);

impl_from_for_Value_1! (Singleton, ValueSingleton);
impl_from_for_Value_1! (Boolean, Boolean);
impl_from_for_Value_1! (NumberInteger, NumberInteger);
impl_from_for_Value_1! (NumberReal, NumberReal);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_1! (Character, Character);
impl_from_for_Value_1! (Symbol, Symbol);
#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
impl_from_for_Value_1! (Keyword, Keyword);
#[ cfg ( feature = "vonuvoli_values_unique" ) ]
impl_from_for_Value_1! (Unique, Unique);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_1! (StringImmutable, StringImmutable);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_Value_1! (StringMutable, StringMutable);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_Value_1! (BytesImmutable, BytesImmutable);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_Value_1! (BytesMutable, BytesMutable);
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_1! (StringRegex, StringRegex);
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_Value_1! (BytesRegex, BytesRegex);
impl_from_for_Value_1! (PairImmutable, PairImmutable);
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_Value_1! (PairMutable, PairMutable);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_Value_1! (ArrayImmutable, ArrayImmutable);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_Value_1! (ArrayMutable, ArrayMutable);
#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl_from_for_Value_1! (Values, Values);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_Value_1! (RecordKind, RecordKind);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_Value_1! (RecordImmutable, RecordImmutable);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_Value_1! (RecordMutable, RecordMutable);
#[ cfg ( feature = "vonuvoli_values_error" ) ]
impl_from_for_Value_1! (Error, Error);
impl_from_for_Value_1! (ProcedurePrimitive, ProcedurePrimitive);
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_from_for_Value_1! (ProcedureExtended, ProcedureExtended);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_Value_1! (ProcedureNative, ProcedureNative);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_from_for_Value_1! (ProcedureLambda, ProcedureLambda);
impl_from_for_Value_1! (SyntaxPrimitive, SyntaxPrimitive);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_from_for_Value_1! (SyntaxExtended, SyntaxExtended);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_Value_1! (SyntaxNative, SyntaxNative);
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_from_for_Value_1! (SyntaxLambda, SyntaxLambda);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_Value_1! (Path, Path);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_Value_1! (Port, Port);
#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl_from_for_Value_1! (Process, Process);
#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
impl_from_for_Value_1! (Context, Context);
#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
impl_from_for_Value_1! (Binding, Binding);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl_from_for_Value_1! (Parameters, Parameters);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl_from_for_Value_1! (Parameter, Parameter);
#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
impl_from_for_Value_1! (Promise, Promise);
#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl_from_for_Value_1! (Opaque, Opaque);


impl_from_for_Value_3! (Singleton, ValueSingleton, (), _value, ValueSingleton::Void);

impl_from_for_Value_2! (Boolean, Boolean, bool);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_2! (Character, Character, char);

impl_from_for_Value_2! (NumberInteger, NumberInteger, i64);
impl_from_for_Value_3! (NumberInteger, NumberInteger, i8, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u8, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i16, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u16, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i32, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u32, value, number_i64 (i64::from (value)));
impl_from_for_Value_3! (NumberInteger, NumberInteger, isize, value, number_i64 (value as i64));
impl_try_from_for_type! (NumberInteger, u64, value, if value <= <i64>::max_value () as u64 { succeeded! (number_i64 (value as i64)) } else { failed! (0x78f55fb6) });
impl_try_from_for_type! (NumberInteger, usize, value, if value <= <i64>::max_value () as usize { succeeded! (number_i64 (value as i64)) } else { failed! (0xe99641f7) });
impl_try_from_for_type! (Value, u64, value, StdTryInto::<NumberInteger>::try_into (value));
impl_try_from_for_type! (Value, usize, value, StdTryInto::<NumberInteger>::try_into (value));
impl_from_for_type! (NumberInteger, char, value, number_i64 (value as i64));

impl_from_for_Value_2! (NumberReal, NumberReal, f64);
impl_from_for_type! (NumberReal, NumberInteger, value, number_f64 (i64::from (value) as f64));
impl_from_for_type! (NumberReal, f32, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, i8, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, u8, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, i16, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, u16, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, i32, value, number_f64 (f64::from (value)));
impl_from_for_type! (NumberReal, u32, value, number_f64 (f64::from (value)));
// FIXME:
impl_from_for_type! (NumberReal, i64, value, number_f64 (value as f64));
// FIXME: impl_from_for_type! (NumberReal, u64, value, number_f64 (value as f64));
// FIXME: impl_from_for_type! (NumberReal, isize, value, number_f64 (value as f64));
// FIXME: impl_from_for_type! (NumberReal, usize, value, number_f64 (value as f64));

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_3! (StringImmutable, StringImmutable, StdString, value, string_immutable_new (value));
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_Value_3! (StringImmutable, StringImmutable, &'static str, value, string_immutable_clone_str (value));

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_type! (StringMutable, StdString, value, string_mutable_new (value));
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_type! (StringMutable, &'static str, value, string_mutable_clone_str (value));

impl_from_for_type! (Symbol, StdString, value, symbol_new (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_clone_str (value));

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
impl_from_for_type! (Keyword, StdString, value, keyword_new (value));
#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
impl_from_for_type! (Keyword, &'static str, value, keyword_clone_str (value));

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_Value_3! (Path, Path, fs_path::PathBuf, value, Path::new_from_buffer (value, true));
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_Value_3! (Path, Path, &'static fs_path::Path, value, Path::new_from_ref (value, true));

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_type! (Path, StdString, value, Path::new_from_buffer (fs_path::PathBuf::from (value), true));
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_type! (Path, &'static str, value, Path::new_from_ref (fs_path::Path::new (value), true));

#[ cfg ( feature = "vonuvoli_values_unique" ) ]
impl_from_for_Value_3! (Unique, Unique, UniqueData, data, Unique::new (data));
#[ cfg ( all ( not ( feature = "vonuvoli_values_unique" ), feature = "vonuvoli_builtins_parameters" ) ) ]
impl_from_for_type! (Unique, UniqueData, data, Unique::new (data));

impl_from_for_type! (PairImmutable, (Value, Value), value, { let (left, right) = value; pair_immutable_new (left, right) });
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl_from_for_type! (PairMutable, (Value, Value), value, { let (left, right) = value; pair_mutable_new (left, right) });

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_from_for_Value_3! (ProcedureExtended, ProcedureExtended, ProcedureExtendedInternals, internals, ProcedureExtended::new (internals));
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_from_for_Value_3! (SyntaxExtended, SyntaxExtended, SyntaxExtendedInternals, internals, SyntaxExtended::new (internals));

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_Value_3! (ProcedureNative, ProcedureNative, ProcedureNativeInternals, internals, ProcedureNative::new (internals));
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_Value_3! (SyntaxNative, SyntaxNative, SyntaxNativeInternals, internals, SyntaxNative::new (internals));




#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_as_ref_for_type_wlt! (StringRef<'a>, 'a);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_as_ref_for_type_wlt! (BytesRef<'a>, 'a);
impl_as_ref_for_type_wlt! (PairRef<'a>, 'a);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_as_ref_for_type_wlt! (ArrayRef<'a>, 'a);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_as_ref_for_type_wlt! (RecordRef<'a>, 'a);




#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl_from_for_type! (Value, ProcessStatus, status, status.value ());




FIXME! ("implement this for all that implement `Into<T>`");
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




#[ cfg ( feature = "vonuvoli_values_native" ) ]
macro_rules! impl_from_for_native_procedure_1 {
	( $from : ident, $from_fn : ty, $tag : ident, $coercer : ident ) => (
		impl_from_for_enum! (ProcedureNativeInternals, $tag, $from);
		impl_from_for_Value_3! (ProcedureNative, ProcedureNative, $from, native, ProcedureNativeInternals::$tag (native) .into ());
		impl_from_for_Value_3! (ProcedureNative, ProcedureNative, $from_fn, native, $coercer (native) .into ());
		impl_unwrappers_for_enum_wrapper! (ProcedureNativeInternals, $tag, $from);
		pub fn $coercer (native : $from_fn) -> ($from) {
			$from (native)
		}
	);
}

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative0, ProcedureNativeFn0, Native0, procedure_native_0);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative1, ProcedureNativeFn1, Native1, procedure_native_1);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative2, ProcedureNativeFn2, Native2, procedure_native_2);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative3, ProcedureNativeFn3, Native3, procedure_native_3);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative4, ProcedureNativeFn4, Native4, procedure_native_4);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative5, ProcedureNativeFn5, Native5, procedure_native_5);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNativeN, ProcedureNativeFnN, NativeN, procedure_native_n);

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative0E, ProcedureNativeFn0E, Native0E, procedure_native_0e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative1E, ProcedureNativeFn1E, Native1E, procedure_native_1e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative2E, ProcedureNativeFn2E, Native2E, procedure_native_2e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative3E, ProcedureNativeFn3E, Native3E, procedure_native_3e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative4E, ProcedureNativeFn4E, Native4E, procedure_native_4e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNative5E, ProcedureNativeFn5E, Native5E, procedure_native_5e);
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNativeNE, ProcedureNativeFnNE, NativeNE, procedure_native_ne);

#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_from_for_native_procedure_1! (ProcedureNativeV, ProcedureNativeFnV, NativeV, procedure_native_v);




#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
macro_rules! impl_from_for_native_syntax_1 {
	( $from : ident, $from_fn : ty, $tag : ident, $coercer : ident ) => (
		impl_from_for_enum! (SyntaxNativeInternals, $tag, $from);
		impl_from_for_Value_3! (SyntaxNative, SyntaxNative, $from, native, SyntaxNativeInternals::$tag (native) .into ());
		impl_from_for_Value_3! (SyntaxNative, SyntaxNative, $from_fn, native, $coercer (native) .into ());
		FIXME! ("`rustc --explain E0162` ???");
		// impl_unwrappers_for_enum_wrapper! (SyntaxNativeInternals, $tag, $from);
		pub fn $coercer (native : $from_fn) -> ($from) {
			$from (native)
		}
	);
}

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
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

#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive0, ProcedurePrimitive0, Primitive0, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive1, ProcedurePrimitive1, Primitive1, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive2, ProcedurePrimitive2, Primitive2, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive3, ProcedurePrimitive3, Primitive3, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive4, ProcedurePrimitive4, Primitive4, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitive5, ProcedurePrimitive5, Primitive5, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Comparison);
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
impl_from_for_primitive_procedure_2! (ComparisonPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Comparison);

impl_from_for_primitive_procedure_2! (ListPrimitive0, ProcedurePrimitive0, Primitive0, List);
impl_from_for_primitive_procedure_2! (ListPrimitive1, ProcedurePrimitive1, Primitive1, List);
impl_from_for_primitive_procedure_2! (ListPrimitive2, ProcedurePrimitive2, Primitive2, List);
impl_from_for_primitive_procedure_2! (ListPrimitive3, ProcedurePrimitive3, Primitive3, List);
impl_from_for_primitive_procedure_2! (ListPrimitive4, ProcedurePrimitive4, Primitive4, List);
impl_from_for_primitive_procedure_2! (ListPrimitive5, ProcedurePrimitive5, Primitive5, List);
impl_from_for_primitive_procedure_2! (ListPrimitiveN, ProcedurePrimitiveN, PrimitiveN, List);
impl_from_for_primitive_procedure_2! (ListPrimitiveV, ProcedurePrimitiveV, PrimitiveV, List);

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive0, ProcedurePrimitive0, Primitive0, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive1, ProcedurePrimitive1, Primitive1, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive2, ProcedurePrimitive2, Primitive2, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive3, ProcedurePrimitive3, Primitive3, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive4, ProcedurePrimitive4, Primitive4, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitive5, ProcedurePrimitive5, Primitive5, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Array);
#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_from_for_primitive_procedure_2! (ArrayPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Array);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive0, ProcedurePrimitive0, Primitive0, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive1, ProcedurePrimitive1, Primitive1, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive2, ProcedurePrimitive2, Primitive2, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive3, ProcedurePrimitive3, Primitive3, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive4, ProcedurePrimitive4, Primitive4, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitive5, ProcedurePrimitive5, Primitive5, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bytes);
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_from_for_primitive_procedure_2! (BytesPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Bytes);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive0, ProcedurePrimitive0, Primitive0, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive1, ProcedurePrimitive1, Primitive1, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive2, ProcedurePrimitive2, Primitive2, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive3, ProcedurePrimitive3, Primitive3, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive4, ProcedurePrimitive4, Primitive4, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitive5, ProcedurePrimitive5, Primitive5, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitiveN, ProcedurePrimitiveN, PrimitiveN, String);
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_from_for_primitive_procedure_2! (StringPrimitiveV, ProcedurePrimitiveV, PrimitiveV, String);

impl_from_for_primitive_procedure_2! (FunctionsPrimitive0, ProcedurePrimitive0, Primitive0, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive1, ProcedurePrimitive1, Primitive1, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive2, ProcedurePrimitive2, Primitive2, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive3, ProcedurePrimitive3, Primitive3, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive4, ProcedurePrimitive4, Primitive4, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive5, ProcedurePrimitive5, Primitive5, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Functions);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive0, ProcedurePrimitive0, Primitive0, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive1, ProcedurePrimitive1, Primitive1, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive2, ProcedurePrimitive2, Primitive2, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive3, ProcedurePrimitive3, Primitive3, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive4, ProcedurePrimitive4, Primitive4, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitive5, ProcedurePrimitive5, Primitive5, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Record);
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_from_for_primitive_procedure_2! (RecordPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Record);

impl_from_for_primitive_procedure_2! (RuntimePrimitive0, ProcedurePrimitive0, Primitive0, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive1, ProcedurePrimitive1, Primitive1, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive2, ProcedurePrimitive2, Primitive2, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive3, ProcedurePrimitive3, Primitive3, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive4, ProcedurePrimitive4, Primitive4, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive5, ProcedurePrimitive5, Primitive5, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitiveV, ProcedurePrimitiveV, PrimitiveV, Runtime);

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive0, ProcedurePrimitive0, Primitive0, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive1, ProcedurePrimitive1, Primitive1, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive2, ProcedurePrimitive2, Primitive2, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive3, ProcedurePrimitive3, Primitive3, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive4, ProcedurePrimitive4, Primitive4, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitive5, ProcedurePrimitive5, Primitive5, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Port);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_from_for_primitive_procedure_2! (PortPrimitiveV, ProcedurePrimitiveV, PrimitiveV, Port);

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive0, ProcedurePrimitive0, Primitive0, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive1, ProcedurePrimitive1, Primitive1, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive2, ProcedurePrimitive2, Primitive2, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive3, ProcedurePrimitive3, Primitive3, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive4, ProcedurePrimitive4, Primitive4, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitive5, ProcedurePrimitive5, Primitive5, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitiveN, ProcedurePrimitiveN, PrimitiveN, FileSystem);
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_from_for_primitive_procedure_2! (FileSystemPrimitiveV, ProcedurePrimitiveV, PrimitiveV, FileSystem);




pub enum NumberCoercion1 {
	Integer ( i64 ),
	Real ( f64 ),
}

pub enum NumberCoercion2 {
	Integer ( i64, i64 ),
	Real ( f64, f64 ),
}


impl NumberCoercion1 {
	
	pub fn into_value (self) -> (Value) {
		match self {
			NumberCoercion1::Integer (number) =>
				number.into (),
			NumberCoercion1::Real (number) =>
				number.into (),
		}
	}
	
	pub fn try_to_i64 (self) -> (Outcome<i64>) {
		match self {
			NumberCoercion1::Integer (number) =>
				succeed! (number),
			NumberCoercion1::Real (_) => {
				TODO! ("implement the same method as in `NumberReal::try_to_i64`");
				fail! (0x53b7b5c8);
			},
		}
	}
	
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
	
	pub fn into_values (self) -> ((Value, Value)) {
		match self {
			NumberCoercion2::Integer (number_1, number_2) =>
				(number_1.into (), number_2.into ()),
			NumberCoercion2::Real (number_1, number_2) =>
				(number_1.into (), number_2.into ()),
		}
	}
}


pub fn number_coerce_1a (value : &Value) -> (Outcome<NumberCoercion1>) {
	let class = value.class_match_as_ref ();
	return number_coerce_1d (&class);
}

pub fn number_coerce_1d <'a> (class : &ValueClassMatchAsRef<'a>) -> (Outcome<NumberCoercion1>) {
	match *class {
		ValueClassMatchAsRef::Number (ref class) =>
			succeed! (number_coerce_1e (class)),
		_ =>
			failed! (0x5539630a),
	}
}

pub fn number_coerce_1e <'a> (class : &NumberMatchAsRef<'a>) -> (NumberCoercion1) {
	match *class {
		NumberMatchAsRef::Integer (value) =>
			NumberCoercion1::Integer (value.value ()),
		NumberMatchAsRef::Real (value) =>
			NumberCoercion1::Real (value.value ()),
	}
}


pub fn number_coerce_2a (left : &Value, right : &Value) -> (Outcome<NumberCoercion2>) {
	let class = Value::class_match_as_ref_2 (left, right);
	return number_coerce_2d (&class);
}

pub fn number_coerce_2b (left : &NumberCoercion1, right : &Value) -> (Outcome<NumberCoercion2>) {
	let right = r#try! (number_coerce_1a (right));
	succeed! (number_coerce_2c (left, &right));
}

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

pub fn number_coerce_2d <'a> (class : &ValueClassMatchAsRef2<'a>) -> (Outcome<NumberCoercion2>) {
	match *class {
		ValueClassMatchAsRef2::Number (ref class) =>
			succeed! (number_coerce_2e (class)),
		_ =>
			failed! (0x6cfbdd37),
	}
}

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




#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
pub fn value_coerce_or_boolean <'a> (value : &'a Value, if_true : Option<Option<&'a Value>>, if_false : Option<Option<&'a Value>>) -> (Outcome<Option<&'a Value>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) =>
			if value.value () {
				succeed! (try_some! (if_true, 0x65921605));
			} else {
				succeed! (try_some! (if_false, 0x86b1c6e4));
			},
		_ =>
			succeed! (Some (value)),
	}
}

#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
pub fn value_coerce_option_or_boolean <'a> (value : Option<&'a Value>, if_true : Option<Option<&'a Value>>, if_false : Option<Option<&'a Value>>) -> (Outcome<Option<&'a Value>>) {
	if let Some (value) = value {
		return value_coerce_or_boolean (value, if_true, if_false);
	} else {
		succeed! (None);
	}
}




pub fn range_coerce (start : Option<&Value>, end : Option<&Value>, length : usize) -> (Outcome<(usize, usize)>) {
	let (start, end) = r#try! (range_coerce_unbounded (start, end));
	let end = end.unwrap_or (length);
	if start > length {
		fail! (0x16e64120);
	}
	if end > length {
		fail! (0x440b8499);
	}
	succeed! ((start, end));
}


pub fn range_coerce_unbounded (start : Option<&Value>, end : Option<&Value>) -> (Outcome<(usize, Option<usize>)>) {
	let start = if let Some (start) = start {
		r#try! (try_as_number_integer_ref! (start) .try_to_usize ())
	} else {
		0
	};
	let end = if let Some (end) = end {
		let end = r#try! (try_as_number_integer_ref! (end) .try_to_usize ());
		if start > end {
			fail! (0x49a6ab02);
		}
		Some (end)
	} else {
		None
	};
	succeed! ((start, end));
}




pub fn count_coerce (value : &Value) -> (Outcome<usize>) {
	return try_as_number_integer_ref! (value) .try_to_usize ();
}

pub fn count_coerce_option (value : Option<&Value>) -> (Outcome<Option<usize>>) {
	succeed! (try_option_map! (value, count_coerce (value)));
}


#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
pub fn count_coerce_or_boolean (value : &Value, if_true : Option<Option<usize>>, if_false : Option<Option<usize>>) -> (Outcome<Option<usize>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (value) =>
			succeed! (Some (r#try! (value.try_to_usize ()))),
		ValueKindMatchAsRef::Boolean (value) =>
			if value.value () {
				succeed! (try_some! (if_true, 0x69d99f5b));
			} else {
				succeed! (try_some! (if_false, 0x49578671));
			},
		_ =>
			fail! (0xa4439c50),
	}
}

#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
pub fn count_coerce_option_or_boolean (value : Option<&Value>, if_true : Option<Option<usize>>, if_false : Option<Option<usize>>) -> (Outcome<Option<usize>>) {
	if let Some (value) = value {
		return count_coerce_or_boolean (value, if_true, if_false);
	} else {
		succeed! (None);
	}
}




pub fn offset_coerce (value : &Value, size : usize) -> (Outcome<usize>) {
	let offset = r#try! (count_coerce (value));
	if offset >= size {
		fail! (0xc1e1c0a3);
	}
	succeed! (offset);
}

pub fn offset_coerce_option (value : Option<&Value>, size : usize) -> (Outcome<Option<usize>>) {
	succeed! (try_option_map! (value, offset_coerce (value, size)));
}




pub fn boolean_coerce (value : &Value) -> (Outcome<bool>) {
	succeed! (try_as_boolean_ref! (value) .value ());
}

pub fn boolean_coerce_option (value : Option<&Value>) -> (Outcome<Option<bool>>) {
	succeed! (try_option_map! (value, boolean_coerce (value)));
}




pub fn string_clone_coerce (value : &Value) -> (Outcome<StdString>) {
	match value.kind_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringImmutable (value) =>
			succeed! (value.string_clone ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::StringMutable (value) =>
			succeed! (r#try! (value.string_ref ()) .string_clone ()),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef::Path (value) =>
			if let Some (value) = value.path_ref () .to_str () {
				succeed! (StdString::from (value));
			} else {
				fail! (0x268a7dee);
			},
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesImmutable (value) =>
			if let Ok (value) = str::from_utf8 (value.bytes_as_slice ()) {
				succeed! (StdString::from (value));
			} else {
				fail! (0x5a40beb0);
			},
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::BytesMutable (value) =>
			if let Ok (value) = str::from_utf8 (r#try! (value.bytes_ref ()) .bytes_as_slice ()) {
				succeed! (StdString::from (value));
			} else {
				fail! (0x4c4b175a);
			},
		_ =>
			fail! (0x9e0d3c33),
	}
}

pub fn string_clone_coerce_option (value : Option<&Value>) -> (Outcome<Option<StdString>>) {
	succeed! (try_option_map! (value, string_clone_coerce (value)));
}




pub fn os_string_clone_coerce (value : &Value) -> (Outcome<ffi::OsString>) {
	match value.kind_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringImmutable (value) =>
			succeed! (value.string_clone () .into ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::StringMutable (value) =>
			succeed! (r#try! (value.string_ref ()) .string_clone () .into ()),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef::Path (value) =>
			succeed! (value.path_ref () .as_os_str () .to_os_string ()),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesImmutable (value) =>
			succeed! (ffi::OsStr::from_bytes (value.bytes_as_slice ()) .to_os_string ()),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::BytesMutable (value) =>
			succeed! (ffi::OsStr::from_bytes (r#try! (value.bytes_ref ()) .bytes_as_slice ()) .to_os_string ()),
		_ =>
			fail! (0x048ce1e9),
	}
}

pub fn os_string_clone_coerce_option (value : Option<&Value>) -> (Outcome<Option<ffi::OsString>>) {
	succeed! (try_option_map! (value, os_string_clone_coerce (value)));
}




pub fn os_string_clone_into_value (string : &ffi::OsStr, immutable : Option<bool>) -> (Outcome<Value>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	{ if let Some (string) = string.to_str () {
		succeed! (string_clone_str (string, immutable));
	} }
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	succeed! (bytes_clone_slice (string.as_bytes (), immutable));
	#[ cfg ( not ( feature = "vonuvoli_values_bytes" ) ) ]
	fail! (0x4eefc5ee);
}




pub fn outcome_as_ref <T> (outcome : &Outcome<T>) -> (Outcome<&T>) {
	match *outcome {
		Ok (ref value) =>
			Ok (value),
		Err (ref error) =>
			Err (error.clone ()),
	}
}




pub fn option_unwrap_ref <T> (option : &Option<T>) -> (&T) {
	match *option {
		Some (ref value) =>
			value,
		None =>
			panic_0! (0x652a4430, github_issue_new),
	}
}




pub fn option_box_new <T> (option : Option<T>) -> (Option<StdBox<T>>) {
	match option {
		Some (value) =>
			Some (StdBox::new (value)),
		None =>
			None,
	}
}

pub fn option_box_unwrap <T> (option : Option<StdBox<T>>) -> (T) {
	match option {
		Some (value) =>
			*value,
		None =>
			panic_0! (0x75a8fcd5, github_issue_new),
	}
}

#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::match_as_ref) ) ]
pub fn option_box_as_ref <T : ?Sized> (option : &Option<StdBox<T>>) -> (Option<&T>) {
	match *option {
		Some (ref value) =>
			Some (value),
		None =>
			None,
	}
}

pub fn option_box_into_owned <T> (value : Option<StdBox<T>>) -> (Option<T>) {
	match value {
		Some (value) =>
			Some (*value),
		None =>
			None,
	}
}




pub enum BytesSliceRef <'a> {
	Immutable ( &'a [u8] ),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable ( StdRef<'a, [u8]> ),
}


impl <'a> BytesSliceRef<'a> {
	
	pub fn range (self, range_start : usize, range_end : Option<usize>) -> (Option<BytesSliceRef<'a>>) {
		if let Some (range_end) = range_end {
			self.slice (range_start .. range_end)
		} else {
			self.slice (range_start ..)
		}
	}
	
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesSliceRef::Mutable (reference) => {
				TODO! ("try to call `get` only once");
				if reference.get (slice.clone ()) .is_some () {
					let reference = StdRef::map (reference, |reference| try_some_or_panic! (reference.get (slice), 0xf11ece64, github_issue_new));
					Some (BytesSliceRef::Mutable (reference))
				} else {
					None
				}
			},
		}
	}
	
	pub fn into_generic_ref (self) -> (GenericRef<'a, [u8]>) {
		match self {
			BytesSliceRef::Immutable (reference) =>
				GenericRef::Immutable (reference),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesSliceRef::Mutable (reference) =>
				GenericRef::Mutable (reference),
		}
	}
}


impl <'a> StdDeref for BytesSliceRef<'a> {
	
	type Target = [u8];
	
	fn deref (&self) -> (&[u8]) {
		match *self {
			BytesSliceRef::Immutable (reference) =>
				reference,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesSliceRef::Mutable (ref reference) =>
				reference,
		}
	}
}


impl <'a> StdAsRef<[u8]> for BytesSliceRef<'a> {
	
	fn as_ref (&self) -> (&[u8]) {
		return self.deref ();
	}
}




impl <'a> StdFrom<&'a [u8]> for BytesSliceRef<'a> {
	
	fn from (reference : &'a [u8]) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference)
	}
}

impl <'a> StdFrom<&'a StdBox<[u8]>> for BytesSliceRef<'a> {
	
	fn from (reference : &'a StdBox<[u8]>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_ref ())
	}
}

impl <'a> StdFrom<&'a StdVec<u8>> for BytesSliceRef<'a> {
	
	fn from (reference : &'a StdVec<u8>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_ref ())
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, [u8]>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, [u8]>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference))
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, StdBox<[u8]>>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, StdBox<[u8]>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, StdVec<u8>>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, StdVec<u8>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, BytesMutableInternals>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, BytesMutableInternals>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref ()))
	}
}




impl <'a> StdFrom<&'a str> for BytesSliceRef<'a> {
	
	fn from (reference : &'a str) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

impl <'a> StdFrom<&'a StdBox<str>> for BytesSliceRef<'a> {
	
	fn from (reference : &'a StdBox<str>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

impl <'a> StdFrom<&'a StdString> for BytesSliceRef<'a> {
	
	fn from (reference : &'a StdString) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Immutable (reference.as_bytes ())
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, str>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, str>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, StdBox<str>>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, StdBox<str>>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, StdString>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, StdString>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_bytes ()))
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdFrom<StdRef<'a, StringMutableInternals>> for BytesSliceRef<'a> {
	
	fn from (reference : StdRef<'a, StringMutableInternals>) -> (BytesSliceRef<'a>) {
		BytesSliceRef::Mutable (StdRef::map (reference, |reference| reference.as_ref () .as_bytes ()))
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> StdFrom<BytesRef<'a>> for BytesSliceRef<'a> {
	
	fn from (reference : BytesRef<'a>) -> (BytesSliceRef<'a>) {
		match reference {
			BytesRef::Immutable (_, reference) =>
				reference.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			BytesRef::Mutable (_, reference) =>
				reference.into (),
		}
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> StdFrom<StringRef<'a>> for BytesSliceRef<'a> {
	
	fn from (reference : StringRef<'a>) -> (BytesSliceRef<'a>) {
		match reference {
			StringRef::Immutable (_, reference) =>
				reference.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			StringRef::Mutable (_, reference) =>
				reference.into (),
		}
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl <'a> StdFrom<&'a BytesImmutable> for BytesSliceRef<'a> {
	
	fn from (value : &'a BytesImmutable) -> (BytesSliceRef<'a>) {
		value.bytes_as_slice () .into ()
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdTryFrom<&'a BytesMutable> for BytesSliceRef<'a> {
	
	type Error = Error;
	
	fn try_from (value : &'a BytesMutable) -> (Result<BytesSliceRef<'a>, Self::Error>) {
		succeed! (try_or_fail! (value.bytes_ref (), 0x65baf4e9) .into ());
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl <'a> StdFrom<&'a StringImmutable> for BytesSliceRef<'a> {
	
	fn from (value : &'a StringImmutable) -> (BytesSliceRef<'a>) {
		value.string_as_str () .into ()
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
impl <'a> StdTryFrom<&'a StringMutable> for BytesSliceRef<'a> {
	
	type Error = Error;
	
	fn try_from (value : &'a StringMutable) -> (Result<BytesSliceRef<'a>, Self::Error>) {
		succeed! (try_or_fail! (value.string_ref (), 0x37d56111) .into ());
	}
}




pub fn bytes_slice_coerce_1a (value : &Value) -> (Outcome<BytesSliceRef>) {
	match value.kind_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesImmutable (value) =>
			succeed! (value.into ()),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::BytesMutable (value) =>
			value.try_into (),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringImmutable (value) =>
			succeed! (value.into ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::StringMutable (value) =>
			value.try_into (),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef::Path (value) =>
			succeed! (value.path_ref () .as_os_str () .as_bytes () .into ()),
		_ =>
			fail! (0x1a3502e4),
	}
}




pub fn bytes_consume <Consumer> (value : &Value, consumer : &mut Consumer) -> (Outcome<()>)
		where Consumer : FnMut (&[u8]) -> (Outcome<()>)
{
	match value.kind_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueKindMatchAsRef::BytesImmutable (value) =>
			return consumer (value.bytes_as_slice ()),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::BytesMutable (value) =>
			return consumer (try_or_fail! (value.bytes_ref (), 0x31df3caf) .bytes_as_slice ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueKindMatchAsRef::StringImmutable (value) =>
			return consumer (value.string_as_bytes ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::StringMutable (value) =>
			return consumer (try_or_fail! (value.string_ref (), 0xf1ab5928) .string_as_bytes ()),
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueKindMatchAsRef::Path (value) =>
			return consumer (value.path_ref () .as_os_str () .as_bytes ()),
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ValueKindMatchAsRef::Port (value) => {
			r#try! (value.byte_consume (consumer));
			succeed! (());
		},
		_ =>
			fail! (0xcd705412),
	}
}




pub struct PathSliceRef<'a> ( BytesSliceRef<'a> );


impl <'a> PathSliceRef<'a> {
	
	pub fn into_generic_ref (self) -> (GenericRef<'a, fs_path::Path>) {
		let reference = self.0.into_generic_ref ();
		let reference = reference.map_generic (|reference| fs_path::Path::new (ffi::OsStr::from_bytes (reference)));
		return reference;
	}
}


impl <'a> StdDeref for PathSliceRef<'a> {
	
	type Target = fs_path::Path;
	
	fn deref (&self) -> (&fs_path::Path) {
		fs_path::Path::new (ffi::OsStr::from_bytes (self.0.deref ()))
	}
}


impl <'a> StdAsRef<fs_path::Path> for PathSliceRef<'a> {
	
	fn as_ref (&self) -> (&fs_path::Path) {
		return self.deref ();
	}
}


pub fn path_slice_coerce (value : &Value) -> (Outcome<PathSliceRef>) {
	succeed! (PathSliceRef (r#try! (bytes_slice_coerce_1a (value))));
}

pub fn path_name_slice_coerce (value : &Value) -> (Outcome<PathSliceRef>) {
	let path = r#try! (path_slice_coerce (value));
	{
		let path = path.deref ();
		let path_name = try_some! (path.file_name (), 0xfdbdee59);
		if path_name != path.as_os_str () {
			fail! (0xec1decd2);
		}
	}
	succeed! (path);
}




pub struct OsStrSliceRef<'a> ( BytesSliceRef<'a> );


impl <'a> OsStrSliceRef<'a> {
	
	pub fn into_generic_ref (self) -> (GenericRef<'a, ffi::OsStr>) {
		let reference = self.0.into_generic_ref ();
		let reference = reference.map_generic (|reference| ffi::OsStr::from_bytes (reference));
		return reference;
	}
}


impl <'a> StdDeref for OsStrSliceRef<'a> {
	
	type Target = ffi::OsStr;
	
	fn deref (&self) -> (&ffi::OsStr) {
		ffi::OsStr::from_bytes (self.0.deref ())
	}
}


impl <'a> StdAsRef<ffi::OsStr> for OsStrSliceRef<'a> {
	
	fn as_ref (&self) -> (&ffi::OsStr) {
		return self.deref ();
	}
}


pub fn os_str_slice_coerce (value : &Value) -> (Outcome<OsStrSliceRef>) {
	succeed! (OsStrSliceRef (r#try! (bytes_slice_coerce_1a (value))));
}




pub fn list_or_array_coerce_clone (value : &Value) -> (Outcome<StdVec<Value>>) {
	return sequence_coerce_clone_0 (value, true, true, false, false);
}

pub fn list_or_array_or_values_coerce_clone (value : &Value) -> (Outcome<StdVec<Value>>) {
	return sequence_coerce_clone_0 (value, true, true, true, false);
}

pub fn sequence_coerce_clone (value : &Value) -> (Outcome<StdVec<Value>>) {
	return sequence_coerce_clone_0 (value, true, true, true, true);
}

pub fn sequence_coerce_clone_0 (value : &Value, list_allowed : bool, array_allowed : bool, values_allowed : bool, record_allowed : bool) -> (Outcome<StdVec<Value>>) {
	match value.class_match_as_ref () {
		ValueClassMatchAsRef::Pair (_) => {
			if ! list_allowed {
				fail! (0xe9a0f5bc);
			}
			let values = r#try! (vec_list_clone (value));
			succeed! (values);
		},
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueClassMatchAsRef::Array (value) => {
			if ! array_allowed {
				fail! (0xd7ad60bc);
			}
			let value = r#try! (value.array_ref ());
			let values = value.values_clone ();
			succeed! (values);
		},
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueClassMatchAsRef::Values (value) => {
			if ! values_allowed {
				fail! (0x4cba04b4);
			}
			let values = value.values_clone ();
			let values = StdVec::from (values);
			succeed! (values);
		},
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::Record (value) => {
			if ! record_allowed {
				fail! (0x9275ed8a);
			}
			let value = r#try! (value.record_ref ());
			let values = value.values_clone ();
			succeed! (values);
		},
		_ =>
			fail! (0x276ff320),
	}
}




pub enum CoercedRef <'a, Value : 'a> {
	Reference ( &'a Value ),
	Value ( Value ),
	StdRef ( StdRef<'a, dyn StdAny>, &'a Value ),
	StdBox ( StdBox<dyn StdAny>, &'a Value ),
	StdRc ( StdRc<dyn StdAny>, &'a Value ),
}


impl <'a, Value> CoercedRef<'a, Value> {
	
	pub fn new_reference (value : &'a Value) -> (CoercedRef<'a, Value>) {
		CoercedRef::Reference (value)
	}
	
	pub fn new_value (value : Value) -> (CoercedRef<'a, Value>) {
		CoercedRef::Value (value)
	}
	
	pub fn new_cell <U : 'static, Accessor> (value : StdRef<'a, U>, accessor : Accessor) -> (CoercedRef<'a, Value>)
			where Accessor : FnOnce (&'a U) -> (&'a Value)
	{
		let value_ref = unsafe { mem::transmute (StdRef::deref (&value)) };
		let value_ref = accessor (value_ref);
		CoercedRef::StdRef (value, value_ref)
	}
	
	pub fn new_box <U : 'static, Accessor> (value : StdBox<U>, accessor : Accessor) -> (CoercedRef<'a, Value>)
			where Accessor : FnOnce (&'a U) -> (&'a Value)
	{
		let value_ref = unsafe { mem::transmute (StdBox::deref (&value)) };
		let value_ref = accessor (value_ref);
		CoercedRef::StdBox (value, value_ref)
	}
	
	pub fn new_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (CoercedRef<'a, Value>)
			where Accessor : FnOnce (&'a U) -> (&'a Value)
	{
		let value_ref = unsafe { mem::transmute (StdRc::deref (&value)) };
		let value_ref = accessor (value_ref);
		CoercedRef::StdRc (value, value_ref)
	}
}


impl <'a, Value> StdDeref for CoercedRef<'a, Value> {
	
	type Target = Value;
	
	fn deref (&self) -> (&Value) {
		match *self {
			CoercedRef::Reference (value) =>
				value,
			CoercedRef::Value (ref value) =>
				value,
			CoercedRef::StdRef (_, value_ref) =>
				value_ref,
			CoercedRef::StdBox (_, value_ref) =>
				value_ref,
			CoercedRef::StdRc (_, value_ref) =>
				value_ref,
		}
	}
}


impl <'a, Value> StdAsRef<Value> for CoercedRef<'a, Value> {
	
	fn as_ref (&self) -> (&Value) {
		return self.deref ();
	}
}




/*
impl <From, To> StdInto0<Outcome<To>> for From where From : StdInto<To> {
	
	fn into_0 (self) -> (Outcome<To>) {
		return Ok (self.into ());
	}
}

impl <From, To> StdInto0<Outcome<To>> for Outcome<From> where From : StdInto<To> {
	
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
	
	fn try_into_0 (self) -> (Result<To, Self::Error>) {
		return self.try_into ();
	}
}

impl <From, To> StdTryInto0<To> for Outcome<From> where From : StdTryInto<To, Error = super::errors::exports::Error> {
	
	type Error = super::errors::exports::Error;
	
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




#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl <Value1, Value2> StdInto0<Outcome<Values>> for (Value1, Value2) where Value1 : StdInto<Value>, Value2 : StdInto<Value> {
	
	fn into_0 (self) -> (Outcome<Values>) {
		let (value_1, value_2) = self;
		let values = vec! [value_1.into (), value_2.into ()];
		let values = values_new (values.into_boxed_slice ());
		return Ok (values);
	}
}

#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl <Value1, Value2> StdInto0<Outcome<Values>> for Outcome<(Value1, Value2)> where Value1 : StdInto<Value>, Value2 : StdInto<Value> {
	
	fn into_0 (self) -> (Outcome<Values>) {
		match self {
			Ok (values) =>
				return values.into_0 (),
			Err (error) =>
				return Err (error),
		}
	}
}


#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl <Value1, Value2, Value3> StdInto0<Outcome<Values>> for (Value1, Value2, Value3) where Value1 : StdInto<Value>, Value2 : StdInto<Value>, Value3 : StdInto<Value> {
	
	fn into_0 (self) -> (Outcome<Values>) {
		let (value_1, value_2, value_3) = self;
		let values = vec! [value_1.into (), value_2.into (), value_3.into ()];
		let values = values_new (values.into_boxed_slice ());
		return Ok (values);
	}
}

#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl <Value1, Value2, Value3> StdInto0<Outcome<Values>> for Outcome<(Value1, Value2, Value3)> where Value1 : StdInto<Value>, Value2 : StdInto<Value>, Value3 : StdInto<Value> {
	
	fn into_0 (self) -> (Outcome<Values>) {
		match self {
			Ok (values) =>
				return values.into_0 (),
			Err (error) =>
				return Err (error),
		}
	}
}

