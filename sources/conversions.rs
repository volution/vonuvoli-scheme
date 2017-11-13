

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::*;
	pub use super::vec_into;
	pub use super::vec_from_slice;
	pub use super::vec_from_list;
}




#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_1 {
	( $from : ty, $tag : ident ) => (
		impl_from_for_enum_wrapper! (Expression, $from, $tag);
		impl_from_for_type! (Expression, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ExpressionBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ExpressionBox, &'static $from, value, StdBox::new (value.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_2 {
	( $from : ty, $value : ident, $expression : expr ) => (
		impl_from_for_type! (Expression, $from, $value, $expression);
		impl_from_for_type! (Expression, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ExpressionBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ExpressionBox, &'static $from, value, StdBox::new (value.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_0 {
	( $from : ty, $tag : ident ) => (
		impl_from_for_Expression_1! ($from, Value);
		impl_from_for_enum_wrapper! (Value, $from, $tag);
		impl_from_for_type! (Value, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ValueBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ValueBox, &'static $from, value, StdBox::new (value.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_1 {
	( $from : ident, $tag : ident ) => (
		impl_from_for_Value_0! ($from, $tag);
		impl StdTryFrom<Value> for $from {
			type Error = super::errors::Error;
			#[ inline (always) ]
			fn try_from (value : Value) -> (Outcome<Self>) {
				if let Value::$tag (value) = value {
					Ok (value)
				} else {
					failed! (0x64d097b5)
				}
			}
		}
		impl StdFrom<Value> for $from {
			#[ inline (always) ]
			fn from (value : Value) -> (Self) {
				StdTryFrom::try_from (value) .unwrap ()
			}
		}
		impl<'a> StdTryAsRef<$from> for Value {
			type Error = super::errors::Error;
			#[ inline (always) ]
			fn try_as_ref (&self) -> (Outcome<&$from>) {
				if let Value::$tag (ref value) = *self {
					Ok (value)
				} else {
					failed! (0x19768613)
				}
			}
		}
		impl<'a> StdAsRef<$from> for Value {
			#[ inline (always) ]
			fn as_ref (&self) -> (&$from) {
				StdTryAsRef::try_as_ref (self) .unwrap ()
			}
		}
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_2 {
	( $from : ty, $to : ty, $tag : ident, $value : ident, $expression : expr ) => (
		impl_from_for_Value_0! ($from, $tag);
		impl_from_for_type! ($to, $from, $value, $expression);
	);
}




impl_from_for_Expression_1! (Value, Value);

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
impl_from_for_Value_1! (Lambda, Lambda);
impl_from_for_Value_1! (ProcedurePrimitive, ProcedurePrimitive);
impl_from_for_Value_1! (SyntaxPrimitive, SyntaxPrimitive);
impl_from_for_Value_1! (Context, Context);
impl_from_for_Value_1! (Binding, Binding);

impl_from_for_Value_2! (bool, Boolean, Boolean, value, boolean (value));
impl_from_for_Value_2! (char, Character, Character, value, character (value));

impl_from_for_Value_2! (i8, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (u8, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (i16, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (u16, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (i32, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (u32, NumberInteger, NumberInteger, value, number_i64 (value as i64));
impl_from_for_Value_2! (i64, NumberInteger, NumberInteger, value, number_i64 (value));

impl_from_for_Value_2! (f32, NumberReal, NumberReal, value, number_f64 (value as f64));
impl_from_for_Value_2! (f64, NumberReal, NumberReal, value, number_f64 (value));

impl_from_for_Value_2! (StdString, String, String, value, string (value));
impl_from_for_Value_2! (&'static str, String, String, value, string_from_slice (value));

impl_from_for_type! (Symbol, StdString, value, symbol (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_from_slice (value));




#[ allow (unused_macros) ]
macro_rules! impl_from_for_primitive_procedure {
	( $from : ty, $tag_1 : ident, $tag_2 : ident, $tag_3 : ident ) => (
		impl_from_for_Value_0! ($from, ProcedurePrimitive);
		impl_from_for_enum_wrapper! (ProcedurePrimitive, $from, $tag_2);
		impl_from_for_enum_wrapper! ($tag_1, $from, $tag_3);
	);
}


impl_from_for_primitive_procedure! (TypePrimitive1, ProcedurePrimitive1, Primitive1, Type);

impl_from_for_primitive_procedure! (BooleanPrimitive1, ProcedurePrimitive1, Primitive1, Boolean);
impl_from_for_primitive_procedure! (BooleanPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Boolean);

impl_from_for_primitive_procedure! (ArithmeticPrimitive1, ProcedurePrimitive1, Primitive1, Arithmetic);
impl_from_for_primitive_procedure! (ArithmeticPrimitive2, ProcedurePrimitive2, Primitive2, Arithmetic);
impl_from_for_primitive_procedure! (ArithmeticPrimitiveN, ProcedurePrimitiveN, PrimitiveN, Arithmetic);

impl_from_for_primitive_procedure! (BitwisePrimitive1, ProcedurePrimitive1, Primitive1, Bitwise);
impl_from_for_primitive_procedure! (BitwisePrimitive2, ProcedurePrimitive2, Primitive2, Bitwise);
impl_from_for_primitive_procedure! (BitwisePrimitiveN, ProcedurePrimitiveN, PrimitiveN, Bitwise);




#[ allow (unused_macros) ]
macro_rules! impl_from_for_primitive_syntax {
	( $from : ty, $tag : ident ) => (
		impl_from_for_Value_0! ($from, SyntaxPrimitive);
		impl_from_for_enum_wrapper! (SyntaxPrimitive, $from, $tag);
	);
}

impl_from_for_primitive_syntax! (SyntaxPrimitive1, Primitive1);
impl_from_for_primitive_syntax! (SyntaxPrimitive2, Primitive2);
impl_from_for_primitive_syntax! (SyntaxPrimitive3, Primitive3);
impl_from_for_primitive_syntax! (SyntaxPrimitiveN, PrimitiveN);




#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall1 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall1! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, String, Bytes,
				Pair, Array,
				Error,
				Lambda,
				ProcedurePrimitive, SyntaxPrimitive,
				Context, Binding,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall1! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (($primitive, $value), value, Expression::ProcedurePrimitiveCall1 (value.0.into (), value.1.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall2 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall2! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, String, Bytes,
				Pair, Array,
				Error,
				Lambda,
				ProcedurePrimitive, SyntaxPrimitive,
				Context, Binding,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall2! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (($primitive, $value, $value), value, Expression::ProcedurePrimitiveCall2 (value.0.into (), value.1.into (), value.2.into ()));
		impl_from_for_Expression_2! (($primitive, [$value; 2]), value, Expression::ProcedurePrimitiveCall2 (value.0.into (), (value.1)[0].clone () .into (), (value.1)[1].clone ().into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall3 {
	( $primitive : ty ) => (
		impl_from_for_ProcedurePrimitiveCall3! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, String, Bytes,
				Pair, Array,
				Error,
				Lambda,
				ProcedurePrimitive, SyntaxPrimitive,
				Context, Binding,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCall3! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		impl_from_for_Expression_2! (($primitive, $value, $value, $value), value, Expression::ProcedurePrimitiveCall3 (value.0.into (), vec! [value.1.into (), value.2.into (), value.3.into ()]));
		impl_from_for_Expression_2! (($primitive, [$value; 3]), value, Expression::ProcedurePrimitiveCall3 (value.0.into (), vec_into (value.1[..].to_owned ())));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCallN {
	( $primitive : ty ) => (
		impl_from_for_Expression_2! (($primitive,), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec! []));
		impl_from_for_ProcedurePrimitiveCallN! ($primitive, [
				Expression,
				Value,
				Boolean, NumberInteger, NumberReal, Character,
				Symbol, String, Bytes,
				Pair, Array,
				Error,
				Lambda,
				ProcedurePrimitive, SyntaxPrimitive,
				Context, Binding,
				bool, i64, f64, char,
				StdString, &'static str
			]);
	);
	( $primitive : ty, [ $( $value : ty ),* ] ) => (
		$( impl_from_for_ProcedurePrimitiveCallN! ($primitive, $value); )*
	);
	( $primitive : ty, $value : ty ) => (
		
		impl_from_for_Expression_2! (($primitive, StdVec<$value>), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1)));
		impl_from_for_Expression_2! (($primitive, &'static [$value]), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1.to_owned ())));
		
		impl_from_for_Expression_2! (($primitive, ($value,)), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec! [(value.1).0.into ()]));
		impl_from_for_Expression_2! (($primitive, ($value, $value)), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into ()]));
		impl_from_for_Expression_2! (($primitive, ($value, $value, $value)), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into ()]));
		impl_from_for_Expression_2! (($primitive, ($value, $value, $value, $value)), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into ()]));
		
		impl_from_for_Expression_2! (($primitive, [$value; 1]), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (($primitive, [$value; 2]), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (($primitive, [$value; 3]), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (($primitive, [$value; 4]), value, Expression::ProcedurePrimitiveCallN (value.0.into (), vec_into (value.1[..].to_owned ())));
		
	);
}


/* * /
impl_from_for_ProcedurePrimitiveCall1! (TypePrimitive1);

impl_from_for_ProcedurePrimitiveCall1! (BooleanPrimitive1);
impl_from_for_ProcedurePrimitiveCallN! (BooleanPrimitiveN);

impl_from_for_ProcedurePrimitiveCall1! (ArithmeticPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArithmeticPrimitive2);
impl_from_for_ProcedurePrimitiveCallN! (ArithmeticPrimitiveN);

impl_from_for_ProcedurePrimitiveCall1! (BitwisePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BitwisePrimitive2);
impl_from_for_ProcedurePrimitiveCallN! (BitwisePrimitiveN);
/ * */




#[ inline (always) ]
pub fn vec_into <From, To : StdFrom<From>> (from : Vec<From>) -> (Vec<To>) {
	from.into_iter () .map (|value| value.into ()) .collect ()
}

#[ inline (always) ]
pub fn vec_from_slice <From : Clone, To : StdFrom<From>> (from : &[From]) -> (Vec<To>) {
	from.iter () .cloned () .map (|value| value.into ()) .collect ()
}


#[ inline (always) ]
pub fn vec_from_list (list : &Value) -> (Outcome<ValueVec>) {
	let mut vector = Vec::new ();
	let mut head = list;
	while *head != Value::Null {
		let pair : &Pair = try! (StdTryAsRef::try_as_ref (head));
		vector.push (pair.left () .clone ());
		head = pair.right ();
	}
	return Ok (vector);
}

