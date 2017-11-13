

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let expressions = vec! [
			
			
			
			
			NULL.into (),
			VOID.into (),
			UNDEFINED.into (),
			
			TRUE.into (),
			FALSE.into (),
			
			true.into (),
			false.into (),
			
			ZERO.into (),
			ONE.into (),
			
			1.into (),
			1i64.into (),
			2i8.into (),
			2u8.into (),
			3i16.into (),
			3u16.into (),
			4i32.into (),
			4u32.into (),
			
			INF_POSITIVE.into (),
			INF_NEGATIVE.into (),
			NAN_POSITIVE.into (),
			NAN_NEGATIVE.into (),
			
			'a'.into (),
			"abc".into (),
			
			
			
			
			Expression::Void,
			
			Expression::ContextDefine ("a".into (), ZERO.into ()),
			Expression::ContextSelect ("a".into ()),
			Expression::ContextUpdate ("a".into (), ONE.into ()),
			Expression::ContextSelect ("a".into ()),
			
			
			
			
			TypePrimitive1::IsNull.into (),
			TypePrimitive1::IsBoolean.into (),
			
			BooleanPrimitive1::Not.into (),
			BooleanPrimitiveN::And.into (),
			BooleanPrimitiveN::Or.into (),
			
			ArithmeticPrimitive1::Negate.into (),
			ArithmeticPrimitive2::Power.into (),
			ArithmeticPrimitiveN::Addition.into (),
			ArithmeticPrimitiveN::Subtraction.into (),
			
			BitwisePrimitive1::Complement.into (),
			BitwisePrimitiveN::And.into (),
			BitwisePrimitiveN::Or.into (),
			
			
			
			
			(TypePrimitive1::IsNull, VOID) .into (),
			(TypePrimitive1::IsBoolean, VOID) .into (),
			
			
			
			
			(BooleanPrimitive1::Not, TRUE) .into (),
			(BooleanPrimitive1::Not, FALSE) .into (),
			
			(BooleanPrimitiveN::And,) .into (),
			(BooleanPrimitiveN::And, (NULL,)) .into (),
			(BooleanPrimitiveN::And, (true, true)) .into (),
			(BooleanPrimitiveN::And, (false, false)) .into (),
			(BooleanPrimitiveN::And, [true, true, true, true]) .into (),
			(BooleanPrimitiveN::And, [false, false, false, false]) .into (),
			
			
			
			
			(ArithmeticPrimitive1::Negate, 0) .into (),
			
			(ArithmeticPrimitive2::Power, 2, 4) .into (),
			
			(ArithmeticPrimitiveN::Addition,) .into (),
			(ArithmeticPrimitiveN::Addition, (0, 1)) .into (),
			(ArithmeticPrimitiveN::Addition, [0, 1, 2]) .into (),
			(ArithmeticPrimitiveN::Addition, [0, 1, 2, 3]) .into (),
			
			
			
			
			(BitwisePrimitive1::Complement, 0) .into (),
			
			(BitwisePrimitiveN::Nxor,) .into (),
			(BitwisePrimitiveN::Nxor, (0, 1)) .into (),
			(BitwisePrimitiveN::Nxor, [0, 1, 2]) .into (),
			(BitwisePrimitiveN::Nxor, [0, 1, 2, 3]) .into (),
			
			
		];
	
	
	for expression in expressions {
		println! ();
		println! (">> {:#?}", expression);
		println! ();
	}
	
}

