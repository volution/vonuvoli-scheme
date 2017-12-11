

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::extended_syntaxes::exports::*;
use super::lambdas::exports::*;
use super::ports::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::*;
}




macro_rules! impl_from_for_Expression_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_type! (Expression, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ExpressionBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ExpressionBox, &'static $from, value, StdBox::new (value.into ()));
	);
}

macro_rules! impl_from_for_Expression_1 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_enum! (Expression, $tag, $from);
		impl_from_for_Expression_0! ($tag, $from);
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_2 {
	( $tag : ident, $from : ty, $value : ident, $expression : tt ) => (
		impl_from_for_type! (Expression, $from, $value, Expression::$tag $expression);
		impl_from_for_Expression_0! ($tag, $from);
	);
}




macro_rules! impl_from_for_Value_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_type! (Value, $from, value, Value::$tag (VALUE_META_1, value.into (), VALUE_META_2));
		impl_from_for_type! (Value, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ValueBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ValueBox, &'static $from, value, StdBox::new (value.into ()));
		impl_from_for_Expression_1! (Value, $from);
	);
}

macro_rules! impl_from_for_Value_1 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_2_for_enum_3_wrapper! (Value, $tag, $from);
		impl $from {
			pub fn try_from (value : Value) -> (Outcome<$from>) {
				return StdTryInto::<$from>::try_into (value)
			}
			pub fn as_ref (value : &Value) -> (&$from) {
				return StdAsRef::<$from>::as_ref (value)
			}
			pub fn try_as_ref (value : &Value) -> (Outcome<&$from>) {
				return StdTryAsRef::<$from>::try_as_ref (value)
			}
		}
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




impl_from_for_Expression_1! (Value, Value);


impl_from_for_Value_1! (Singleton, ValueSingleton);
impl_from_for_Value_1! (Boolean, Boolean);
impl_from_for_Value_1! (NumberInteger, NumberInteger);
impl_from_for_Value_1! (NumberReal, NumberReal);
impl_from_for_Value_1! (Character, Character);
impl_from_for_Value_1! (Symbol, Symbol);
impl_from_for_Value_1! (String, String);
impl_from_for_Value_1! (Bytes, Bytes);
impl_from_for_Value_1! (Pair, Pair);
impl_from_for_Value_1! (Array, Array);
impl_from_for_Value_1! (Values, Values);
impl_from_for_Value_1! (Error, Error);
impl_from_for_Value_1! (ProcedurePrimitive, ProcedurePrimitive);
impl_from_for_Value_1! (ProcedureExtended, ProcedureExtended);
impl_from_for_Value_1! (ProcedureLambda, ProcedureLambda);
impl_from_for_Value_1! (SyntaxPrimitive, SyntaxPrimitive);
impl_from_for_Value_1! (SyntaxExtended, SyntaxExtended);
impl_from_for_Value_1! (SyntaxLambda, SyntaxLambda);
impl_from_for_Value_1! (Port, Port);
impl_from_for_Value_1! (Context, Context);
impl_from_for_Value_1! (Binding, Binding);


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

impl_from_for_Value_3! (String, String, StdString, value, string_new (value));
impl_from_for_Value_3! (String, String, &'static str, value, string_clone_str (value));

impl_from_for_type! (Symbol, StdString, value, symbol_new (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_clone_str (value));

impl_from_for_type! (Pair, (Value, Value), value, { let (left, right) = value; pair_new (left, right) });

impl_from_for_Value_3! (ProcedureExtended, ProcedureExtended, ProcedureExtendedInternals, internals, ProcedureExtended::new (internals));
impl_from_for_Value_3! (SyntaxExtended, SyntaxExtended, SyntaxExtendedInternals, internals, SyntaxExtended::new (internals));




macro_rules! impl_from_for_primitive_procedure_1 {
	( $from : ty, $tag : ident ) => (
		impl_from_for_Value_0! (ProcedurePrimitive, $from);
		impl_from_for_enum! (ProcedurePrimitive, $tag, $from);
	);
}


impl_from_for_primitive_procedure_1! (ProcedurePrimitive0, Primitive0);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive1, Primitive1);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive2, Primitive2);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive3, Primitive3);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive4, Primitive4);
impl_from_for_primitive_procedure_1! (ProcedurePrimitive5, Primitive5);
impl_from_for_primitive_procedure_1! (ProcedurePrimitiveN, PrimitiveN);




macro_rules! impl_from_for_primitive_syntax {
	( $from : ty, $tag : ident ) => (
		impl_from_for_Value_0! (SyntaxPrimitive, $from);
		impl_from_for_enum! (SyntaxPrimitive, $tag, $from);
	);
}

impl_from_for_primitive_syntax! (SyntaxPrimitive0, Primitive0);
impl_from_for_primitive_syntax! (SyntaxPrimitive1, Primitive1);
impl_from_for_primitive_syntax! (SyntaxPrimitive2, Primitive2);
impl_from_for_primitive_syntax! (SyntaxPrimitive3, Primitive3);
impl_from_for_primitive_syntax! (SyntaxPrimitive4, Primitive4);
impl_from_for_primitive_syntax! (SyntaxPrimitive5, Primitive5);
impl_from_for_primitive_syntax! (SyntaxPrimitiveN, PrimitiveN);




macro_rules! impl_from_for_primitive_procedure_2 {
	( $from : ty, $tag_1 : ident, $tag_2 : ident, $tag_3 : ident ) => (
		impl_from_for_Value_0! (ProcedurePrimitive, $from);
		impl_from_for_enum! (ProcedurePrimitive, $tag_2, $from);
		impl_from_for_enum! ($tag_1, $tag_3, $from);
	);
}

impl_from_for_primitive_procedure_2! (TypePrimitive1, ProcedurePrimitive1, Primitive1, Type);

impl_from_for_primitive_procedure_2! (BooleanPrimitive0, ProcedurePrimitive0, Primitive0, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive1, ProcedurePrimitive1, Primitive1, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive2, ProcedurePrimitive2, Primitive2, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive3, ProcedurePrimitive3, Primitive3, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive4, ProcedurePrimitive4, Primitive4, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitive5, ProcedurePrimitive5, Primitive5, Boolean);
impl_from_for_primitive_procedure_2! (BooleanPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Boolean);

impl_from_for_primitive_procedure_2! (ArithmeticPrimitive0, ProcedurePrimitive0, Primitive0, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive1, ProcedurePrimitive1, Primitive1, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive2, ProcedurePrimitive2, Primitive2, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive3, ProcedurePrimitive3, Primitive3, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive4, ProcedurePrimitive4, Primitive4, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitive5, ProcedurePrimitive5, Primitive5, Arithmetic);
impl_from_for_primitive_procedure_2! (ArithmeticPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Arithmetic);

impl_from_for_primitive_procedure_2! (BitwisePrimitive0, ProcedurePrimitive0, Primitive0, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive1, ProcedurePrimitive1, Primitive1, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive2, ProcedurePrimitive2, Primitive2, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive3, ProcedurePrimitive3, Primitive3, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive4, ProcedurePrimitive4, Primitive4, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitive5, ProcedurePrimitive5, Primitive5, Bitwise);
impl_from_for_primitive_procedure_2! (BitwisePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bitwise);

impl_from_for_primitive_procedure_2! (ComparisonPrimitive0, ProcedurePrimitive0, Primitive0, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive1, ProcedurePrimitive1, Primitive1, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive2, ProcedurePrimitive2, Primitive2, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive3, ProcedurePrimitive3, Primitive3, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive4, ProcedurePrimitive4, Primitive4, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitive5, ProcedurePrimitive5, Primitive5, Comparison);
impl_from_for_primitive_procedure_2! (ComparisonPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Comparison);

impl_from_for_primitive_procedure_2! (ListPrimitive0, ProcedurePrimitive0, Primitive0, List);
impl_from_for_primitive_procedure_2! (ListPrimitive1, ProcedurePrimitive1, Primitive1, List);
impl_from_for_primitive_procedure_2! (ListPrimitive2, ProcedurePrimitive2, Primitive2, List);
impl_from_for_primitive_procedure_2! (ListPrimitive3, ProcedurePrimitive3, Primitive3, List);
impl_from_for_primitive_procedure_2! (ListPrimitive4, ProcedurePrimitive4, Primitive4, List);
impl_from_for_primitive_procedure_2! (ListPrimitive5, ProcedurePrimitive5, Primitive5, List);
impl_from_for_primitive_procedure_2! (ListPrimitiveN, ProcedurePrimitiveN, PrimitiveN, List);

impl_from_for_primitive_procedure_2! (ArrayPrimitive0, ProcedurePrimitive0, Primitive0, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive1, ProcedurePrimitive1, Primitive1, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive2, ProcedurePrimitive2, Primitive2, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive3, ProcedurePrimitive3, Primitive3, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive4, ProcedurePrimitive4, Primitive4, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitive5, ProcedurePrimitive5, Primitive5, Array);
impl_from_for_primitive_procedure_2! (ArrayPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Array);

impl_from_for_primitive_procedure_2! (BytesPrimitive0, ProcedurePrimitive0, Primitive0, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive1, ProcedurePrimitive1, Primitive1, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive2, ProcedurePrimitive2, Primitive2, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive3, ProcedurePrimitive3, Primitive3, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive4, ProcedurePrimitive4, Primitive4, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitive5, ProcedurePrimitive5, Primitive5, Bytes);
impl_from_for_primitive_procedure_2! (BytesPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bytes);

impl_from_for_primitive_procedure_2! (StringPrimitive0, ProcedurePrimitive0, Primitive0, String);
impl_from_for_primitive_procedure_2! (StringPrimitive1, ProcedurePrimitive1, Primitive1, String);
impl_from_for_primitive_procedure_2! (StringPrimitive2, ProcedurePrimitive2, Primitive2, String);
impl_from_for_primitive_procedure_2! (StringPrimitive3, ProcedurePrimitive3, Primitive3, String);
impl_from_for_primitive_procedure_2! (StringPrimitive4, ProcedurePrimitive4, Primitive4, String);
impl_from_for_primitive_procedure_2! (StringPrimitive5, ProcedurePrimitive5, Primitive5, String);
impl_from_for_primitive_procedure_2! (StringPrimitiveN, ProcedurePrimitiveN, PrimitiveN, String);

impl_from_for_primitive_procedure_2! (FunctionsPrimitive0, ProcedurePrimitive0, Primitive0, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive1, ProcedurePrimitive1, Primitive1, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive2, ProcedurePrimitive2, Primitive2, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive3, ProcedurePrimitive3, Primitive3, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive4, ProcedurePrimitive4, Primitive4, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitive5, ProcedurePrimitive5, Primitive5, Functions);
impl_from_for_primitive_procedure_2! (FunctionsPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Functions);

impl_from_for_primitive_procedure_2! (RuntimePrimitive0, ProcedurePrimitive0, Primitive0, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive1, ProcedurePrimitive1, Primitive1, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive2, ProcedurePrimitive2, Primitive2, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive3, ProcedurePrimitive3, Primitive3, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive4, ProcedurePrimitive4, Primitive4, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitive5, ProcedurePrimitive5, Primitive5, Runtime);
impl_from_for_primitive_procedure_2! (RuntimePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Runtime);

impl_from_for_primitive_procedure_2! (PortPrimitive0, ProcedurePrimitive0, Primitive0, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive1, ProcedurePrimitive1, Primitive1, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive2, ProcedurePrimitive2, Primitive2, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive3, ProcedurePrimitive3, Primitive3, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive4, ProcedurePrimitive4, Primitive4, Port);
impl_from_for_primitive_procedure_2! (PortPrimitive5, ProcedurePrimitive5, Primitive5, Port);
impl_from_for_primitive_procedure_2! (PortPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Port);




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
				Symbol, String, Bytes,
				Pair, Array, Values,
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
				Symbol, String, Bytes,
				Pair, Array, Values,
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
				Symbol, String, Bytes,
				Pair, Array, Values,
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
				Symbol, String, Bytes,
				Pair, Array, Values,
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
				Symbol, String, Bytes,
				Pair, Array, Values,
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
				Symbol, String, Bytes,
				Pair, Array, Values,
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


impl_from_for_ProcedurePrimitiveCall1! (TypePrimitive1);

impl_from_for_ProcedurePrimitiveCall0! (BooleanPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BooleanPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BooleanPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BooleanPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BooleanPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BooleanPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BooleanPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (ArithmeticPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ArithmeticPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArithmeticPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ArithmeticPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ArithmeticPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ArithmeticPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ArithmeticPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (BitwisePrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BitwisePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BitwisePrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BitwisePrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BitwisePrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BitwisePrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BitwisePrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (ComparisonPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ComparisonPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ComparisonPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ComparisonPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ComparisonPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ComparisonPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ComparisonPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (ListPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ListPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ListPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ListPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ListPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ListPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ListPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (ArrayPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (ArrayPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArrayPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (ArrayPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (ArrayPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (ArrayPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (ArrayPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (BytesPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (BytesPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BytesPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (BytesPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (BytesPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (BytesPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (BytesPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (StringPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (StringPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (StringPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (StringPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (StringPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (StringPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (StringPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (FunctionsPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (FunctionsPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (FunctionsPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (FunctionsPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (FunctionsPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (FunctionsPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (FunctionsPrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (RuntimePrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (RuntimePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (RuntimePrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (RuntimePrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (RuntimePrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (RuntimePrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (RuntimePrimitiveN);

impl_from_for_ProcedurePrimitiveCall0! (PortPrimitive0);
impl_from_for_ProcedurePrimitiveCall1! (PortPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (PortPrimitive2);
impl_from_for_ProcedurePrimitiveCall3! (PortPrimitive3);
impl_from_for_ProcedurePrimitiveCall4! (PortPrimitive4);
impl_from_for_ProcedurePrimitiveCall5! (PortPrimitive5);
impl_from_for_ProcedurePrimitiveCallN! (PortPrimitiveN);

/ * ----- */




#[ derive (Clone, Debug) ]
pub enum NumberCoercion1 {
	Integer ( NumberInteger ),
	Real ( NumberReal ),
}

#[ derive (Clone, Debug) ]
pub enum NumberCoercion2 {
	Integer ( NumberInteger, NumberInteger ),
	Real ( NumberReal, NumberReal ),
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


pub fn number_coerce_1 (right : &Value) -> (Outcome<NumberCoercion1>) {
	match right {
		&Value::NumberInteger (_, ref right, _) =>
			Ok (NumberCoercion1::Integer (*right)),
		&Value::NumberReal (_, ref right, _) =>
			Ok (NumberCoercion1::Real (*right)),
		_ =>
			failed! (0x947fb339),
	}
}

pub fn number_coerce_2a (left : &Value, right : &Value) -> (Outcome<NumberCoercion2>) {
	match (left, right) {
		(&Value::NumberInteger (_, ref left, _), &Value::NumberInteger (_, ref right, _)) =>
			Ok (NumberCoercion2::Integer (*left, *right)),
		(&Value::NumberReal (_, ref left, _), &Value::NumberReal (_, ref right, _)) =>
			Ok (NumberCoercion2::Real (*left, *right)),
		(&Value::NumberReal (_, ref left, _), &Value::NumberInteger (_, ref right, _)) =>
			Ok (NumberCoercion2::Real (*left, (*right).into ())),
		(&Value::NumberInteger (_, ref left, _), &Value::NumberReal (_, ref right, _)) =>
			Ok (NumberCoercion2::Real ((*left).into (), *right)),
		_ =>
			failed! (0x6cfbdd37),
	}
}

pub fn number_coerce_2b (left : &NumberCoercion1, right : &Value) -> (Outcome<NumberCoercion2>) {
	match (left, right) {
		(&NumberCoercion1::Integer (ref left), &Value::NumberInteger (_, ref right, _)) =>
			Ok (NumberCoercion2::Integer (*left, *right)),
		(&NumberCoercion1::Real (ref left), &Value::NumberReal (_, ref right, _)) =>
			Ok (NumberCoercion2::Real (*left, *right)),
		(&NumberCoercion1::Real (ref left), &Value::NumberInteger (_, ref right, _)) =>
			Ok (NumberCoercion2::Real (*left, (*right).into ())),
		(&NumberCoercion1::Integer (ref left), &Value::NumberReal (_, ref right, _)) =>
			Ok (NumberCoercion2::Real ((*left).into (), *right)),
		_ =>
			failed! (0xc3883ceb),
	}
}




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


pub fn range_coerce_unbounded (start : Option<&Value>, end : Option<&Value>) -> (Outcome<(usize, Option<usize>)>) {
	let start = if let Some (start) = start {
		try! (try_as_number_integer_ref! (start) .try_to_usize ())
	} else {
		0 as usize
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




pub fn option_box_as_ref <T> (option : &Option<Box<T>>) -> (Option<&T>) {
	if let Some (ref boxed) = *option {
		return Some (boxed.as_ref ());
	} else {
		return None;
	}
}

