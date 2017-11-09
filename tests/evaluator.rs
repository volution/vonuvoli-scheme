

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	let expressions = vec! [
			
			Expression::Void,
			Expression::Value (NULL),
			
			Expression::ContextDefine (symbol ("a"), ZERO.into ()),
			Expression::ContextSelect (symbol ("a")),
			Expression::ContextUpdate (symbol ("a"), ONE.into ()),
			Expression::ContextSelect (symbol ("a")),
			
			Expression::ProcedurePrimitiveCall1 (ProcedurePrimitive1::Boolean (BooleanPrimitive1::Negate), TRUE.into ()),
			Expression::ProcedurePrimitiveCall1 (ProcedurePrimitive1::Boolean (BooleanPrimitive1::Negate), FALSE.into ()),
			
			Expression::ProcedurePrimitiveCallN (ProcedurePrimitiveN::Boolean (BooleanPrimitiveN::And), vec! [TRUE.into (), TRUE.into ()]),
			Expression::ProcedurePrimitiveCallN (ProcedurePrimitiveN::Boolean (BooleanPrimitiveN::And), vec! [TRUE.into (), FALSE.into ()]),
			Expression::ProcedurePrimitiveCallN (ProcedurePrimitiveN::Boolean (BooleanPrimitiveN::And), vec! [FALSE.into (), TRUE.into ()]),
			Expression::ProcedurePrimitiveCallN (ProcedurePrimitiveN::Boolean (BooleanPrimitiveN::And), vec! [FALSE.into (), FALSE.into ()]),
			
		];
	
	let mut context = Context::new (None);
	let mut evaluator = Evaluator::new ();
	
	for expression in expressions {
		println! (">> {:?}", expression);
		let outcome = evaluator.evaluate (&mut context, &expression);
		match outcome {
			Ok (value) =>
				println! ("== {:?} {}", expression, value),
			Err (error) =>
				println! ("== {:?} {}", expression, error),
		}
	}
	
}

