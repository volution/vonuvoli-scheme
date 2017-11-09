

use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::*;
}




macro_rules! impl_from_for_Expression {
	( $from : ty, $tag : ident ) => (
		impl_from_for_enum_wrapper! (Expression, $from, $tag);
		impl_from_for_type! (ExpressionBox, $from, value, StdBox::new (value.into ()));
	);
}

macro_rules! impl_from_for_Value_1 {
	( $from : ty, $tag : ident ) => (
		impl_from_for_Expression! ($from, Value);
		impl_from_for_enum_wrapper! (Value, $from, $tag);
		impl_from_for_type! (ValueBox, $from, value, StdBox::new (value.into ()));
	);
}

macro_rules! impl_from_for_Value_2 {
	( $from : ty, $to : ty, $tag : ident, $value : ident, $expression : expr ) => (
		impl_from_for_Value_1! ($from, $tag);
		impl_from_for_type! ( $to, $from, $value, $expression );
	);
}

macro_rules! impl_from_for_primitive_procedure {
	( $from : ty, $tag_1 : ident, $tag_2 : ident, $tag_3 : ident ) => (
		impl_from_for_Value_1! ($from, ProcedurePrimitive);
		impl_from_for_enum_wrapper! (ProcedurePrimitive, $from, $tag_2);
		impl_from_for_enum_wrapper! ($tag_1, $from, $tag_3);
	);
}




impl_from_for_Expression! (Value, Value);

impl_from_for_Value_1! (Boolean, Boolean);
impl_from_for_Value_1! (NumberInteger, NumberInteger);
impl_from_for_Value_1! (NumberReal, NumberReal);
impl_from_for_Value_1! (Character, Character);
impl_from_for_Value_1! (Symbol, Symbol);
impl_from_for_Value_1! (String, String);
impl_from_for_Value_1! (Bytes, Bytes);
impl_from_for_Value_1! (Pair, Pair);
impl_from_for_Value_1! (Array, Array);
impl_from_for_Value_1! (Error, Error);

impl_from_for_Value_2! (bool, Boolean, Boolean, value, boolean (value));
impl_from_for_Value_2! (i64, NumberInteger, NumberInteger, value, number_i64 (value));
impl_from_for_Value_2! (f64, NumberReal, NumberReal, value, number_f64 (value));
impl_from_for_Value_2! (char, Character, Character, value, character (value));
impl_from_for_Value_2! (StdString, String, String, value, string (value));
impl_from_for_Value_2! (&'static str, String, String, value, string_from_slice (value));

impl_from_for_type! (Symbol, StdString, value, symbol (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_from_slice (value));




impl_from_for_primitive_procedure! (TypePrimitive1, ProcedurePrimitive1, Primitive1, Type);

impl_from_for_primitive_procedure! (BooleanPrimitive1, ProcedurePrimitive1, Primitive1, Boolean);
impl_from_for_primitive_procedure! (BooleanPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Boolean);

impl_from_for_primitive_procedure! (ArithmeticPrimitive1, ProcedurePrimitive1, Primitive1, Arithmetic);
impl_from_for_primitive_procedure! (ArithmeticPrimitive2, ProcedurePrimitive2, Primitive2, Arithmetic);
impl_from_for_primitive_procedure! (ArithmeticPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Arithmetic);

impl_from_for_primitive_procedure! (BitwisePrimitive1, ProcedurePrimitive1, Primitive1, Bitwise);
impl_from_for_primitive_procedure! (BitwisePrimitive2, ProcedurePrimitive2, Primitive2, Bitwise);
impl_from_for_primitive_procedure! (BitwisePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bitwise);

