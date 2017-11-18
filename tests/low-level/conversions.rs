

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
			
			
			
			
			#[ cfg (feature = "conversions-all") ]
			(TypePrimitive1::IsNull, VOID) .into (),
			#[ cfg (feature = "conversions-all") ]
			(TypePrimitive1::IsBoolean, VOID) .into (),
			
			
			
			
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitive1::Not, TRUE) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitive1::Not, FALSE) .into (),
			
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And,) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And, (NULL,)) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And, (true, true)) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And, (false, false)) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And, [true, true, true, true]) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BooleanPrimitiveN::And, [false, false, false, false]) .into (),
			
			
			
			
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitive1::Negate, 0) .into (),
			
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitive2::Power, 2, 4) .into (),
			
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitiveN::Addition,) .into (),
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitiveN::Addition, (0, 1)) .into (),
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitiveN::Addition, [0, 1, 2]) .into (),
			#[ cfg (feature = "conversions-all") ]
			(ArithmeticPrimitiveN::Addition, [0, 1, 2, 3]) .into (),
			
			
			
			
			#[ cfg (feature = "conversions-all") ]
			(BitwisePrimitive1::Complement, 0) .into (),
			
			#[ cfg (feature = "conversions-all") ]
			(BitwisePrimitiveN::Nxor,) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BitwisePrimitiveN::Nxor, (0, 1)) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BitwisePrimitiveN::Nxor, [0, 1, 2]) .into (),
			#[ cfg (feature = "conversions-all") ]
			(BitwisePrimitiveN::Nxor, [0, 1, 2, 3]) .into (),
			
			
		];
	
	
	for expression in expressions {
		eprintln! ();
		eprintln! (">> {:#?}", expression);
		eprintln! ();
	}
	
}

