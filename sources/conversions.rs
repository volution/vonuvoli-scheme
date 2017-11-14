

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::*;
}




#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_type! (Expression, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ExpressionBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ExpressionBox, &'static $from, value, StdBox::new (value.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_1 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_enum! (Expression, $tag, $from);
		impl_from_for_Expression_0! ($tag, $from);
		// impl_unwrappers_for_enum_wrapper! (Expression, $tag, $from);
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Expression_2 {
	( $tag : ident, $from : ty, $value : ident, $expression : tt ) => (
		impl_from_for_type! (Expression, $from, $value, Expression::$tag $expression);
		impl_from_for_Expression_0! ($tag, $from);
	);
}




#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_0 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_enum! (Value, $tag, $from);
		impl_from_for_type! (Value, &'static $from, value, value.clone () .into ());
		impl_from_for_type! (ValueBox, $from, value, StdBox::new (value.into ()));
		impl_from_for_type! (ValueBox, &'static $from, value, StdBox::new (value.into ()));
		impl_from_for_Expression_1! (Value, $from);
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_1 {
	( $tag : ident, $from : ty ) => (
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_for_enum_wrapper! (Value, $tag, $from);
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_2 {
	( $tag : ident, $to : ident, $from : ty ) => (
		impl_from_for_type! ($to, $from);
		impl_from_for_Value_0! ($tag, $from);
		impl_unwrappers_for_type_wrapper! ($to, $from);
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_Value_3 {
	( $tag : ident, $to : ty, $from : ty, $value : ident, $expression : expr ) => (
		impl_from_for_type! ($to, $from, $value, $expression);
		impl_from_for_Value_0! ($tag, $from);
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

impl_from_for_Value_2! (Boolean, Boolean, bool);
impl_from_for_Value_2! (Character, Character, char);

impl_from_for_Value_2! (NumberInteger, NumberInteger, i64);
impl_from_for_Value_3! (NumberInteger, NumberInteger, i8, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u8, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i16, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u16, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, i32, value, number_i64 (value as i64));
impl_from_for_Value_3! (NumberInteger, NumberInteger, u32, value, number_i64 (value as i64));

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

impl_from_for_Value_3! (String, String, StdString, value, string (value));
impl_from_for_Value_3! (String, String, &'static str, value, string_from_slice (value));

impl_from_for_type! (Symbol, StdString, value, symbol (value));
impl_from_for_type! (Symbol, &'static str, value, symbol_from_slice (value));




#[ allow (unused_macros) ]
macro_rules! impl_from_for_primitive_procedure {
	( $from : ty, $tag_1 : ident, $tag_2 : ident, $tag_3 : ident ) => (
		impl_from_for_Value_0! (ProcedurePrimitive, $from);
		impl_from_for_enum! (ProcedurePrimitive, $tag_2, $from);
		impl_from_for_enum! ($tag_1, $tag_3, $from);
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
		impl_from_for_Value_0! (SyntaxPrimitive, $from);
		impl_from_for_enum! (SyntaxPrimitive, $tag, $from);
	);
}

impl_from_for_primitive_syntax! (SyntaxPrimitive1, Primitive1);
impl_from_for_primitive_syntax! (SyntaxPrimitive2, Primitive2);
impl_from_for_primitive_syntax! (SyntaxPrimitive3, Primitive3);
impl_from_for_primitive_syntax! (SyntaxPrimitiveN, PrimitiveN);




#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall1 {
	( $primitive : ty ) => (
		#[ cfg (feature = "conversions-all") ]
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
		impl_from_for_Expression_2! (ProcedurePrimitiveCall1, ($primitive, $value), value, (value.0.into (), value.1.into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall2 {
	( $primitive : ty ) => (
		#[ cfg (feature = "conversions-all") ]
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
		impl_from_for_Expression_2! (ProcedurePrimitiveCall2, ($primitive, $value, $value), value, (value.0.into (), value.1.into (), value.2.into ()));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall2, ($primitive, [$value; 2]), value, (value.0.into (), (value.1)[0].clone () .into (), (value.1)[1].clone ().into ()));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCall3 {
	( $primitive : ty ) => (
		#[ cfg (feature = "conversions-all") ]
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
		impl_from_for_Expression_2! (ProcedurePrimitiveCall3, ($primitive, $value, $value, $value), value, (value.0.into (), vec! [value.1.into (), value.2.into (), value.3.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCall3, ($primitive, [$value; 3]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
	);
}

#[ allow (unused_macros) ]
macro_rules! impl_from_for_ProcedurePrimitiveCallN {
	( $primitive : ty ) => (
		#[ cfg (feature = "conversions-all") ]
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive,), value, (value.0.into (), vec! []));
		#[ cfg (feature = "conversions-all") ]
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
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, StdVec<$value>), value, (value.0.into (), vec_into (value.1)));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, &'static [$value]), value, (value.0.into (), vec_into (value.1.to_owned ())));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value,)), value, (value.0.into (), vec! [(value.1).0.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into ()]));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, ($value, $value, $value, $value)), value, (value.0.into (), vec! [(value.1).0.into (), (value.1).1.into (), (value.1).2.into (), (value.1).3.into ()]));
		
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 1]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 2]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 3]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		impl_from_for_Expression_2! (ProcedurePrimitiveCallN, ($primitive, [$value; 4]), value, (value.0.into (), vec_into (value.1[..].to_owned ())));
		
	);
}


/** /
impl_from_for_ProcedurePrimitiveCall1! (TypePrimitive1);

impl_from_for_ProcedurePrimitiveCall1! (BooleanPrimitive1);
impl_from_for_ProcedurePrimitiveCallN! (BooleanPrimitiveN);

impl_from_for_ProcedurePrimitiveCall1! (ArithmeticPrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (ArithmeticPrimitive2);
impl_from_for_ProcedurePrimitiveCallN! (ArithmeticPrimitiveN);

impl_from_for_ProcedurePrimitiveCall1! (BitwisePrimitive1);
impl_from_for_ProcedurePrimitiveCall2! (BitwisePrimitive2);
impl_from_for_ProcedurePrimitiveCallN! (BitwisePrimitiveN);
/ **/




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




pub enum NumberCoercion1 {
	Integer ( NumberInteger ),
	Real ( NumberReal ),
}

pub enum NumberCoercion2 {
	Integer ( NumberInteger, NumberInteger ),
	Real ( NumberReal, NumberReal ),
}

#[ inline (always) ]
pub fn number_coerce_1 (right : &Value) -> (Outcome<NumberCoercion1>) {
	match right {
		&Value::NumberInteger (ref right) =>
			Ok (NumberCoercion1::Integer (*right)),
		&Value::NumberReal (ref right) =>
			Ok (NumberCoercion1::Real (*right)),
		_ =>
			failed! (0x947fb339),
	}
}

#[ inline (always) ]
pub fn number_coerce_2a (left : &Value, right : &Value) -> (Outcome<NumberCoercion2>) {
	match (left, right) {
		(&Value::NumberInteger (ref left), &Value::NumberInteger (ref right)) =>
			Ok (NumberCoercion2::Integer (*left, *right)),
		(&Value::NumberReal (ref left), &Value::NumberReal (ref right)) =>
			Ok (NumberCoercion2::Real (*left, *right)),
		(&Value::NumberReal (ref left), &Value::NumberInteger (ref right)) =>
			Ok (NumberCoercion2::Real (*left, (*right).into ())),
		(&Value::NumberInteger (ref left), &Value::NumberReal (ref right)) =>
			Ok (NumberCoercion2::Real ((*left).into (), *right)),
		_ =>
			failed! (0x6cfbdd37),
	}
}

#[ inline (always) ]
pub fn number_coerce_2b (left : &NumberCoercion1, right : &Value) -> (Outcome<NumberCoercion2>) {
	match (left, right) {
		(&NumberCoercion1::Integer (ref left), &Value::NumberInteger (ref right)) =>
			Ok (NumberCoercion2::Integer (*left, *right)),
		(&NumberCoercion1::Real (ref left), &Value::NumberReal (ref right)) =>
			Ok (NumberCoercion2::Real (*left, *right)),
		(&NumberCoercion1::Real (ref left), &Value::NumberInteger (ref right)) =>
			Ok (NumberCoercion2::Real (*left, (*right).into ())),
		(&NumberCoercion1::Integer (ref left), &Value::NumberReal (ref right)) =>
			Ok (NumberCoercion2::Real ((*left).into (), *right)),
		_ =>
			failed! (0xc3883ceb),
	}
}

