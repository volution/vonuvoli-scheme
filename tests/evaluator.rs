

#[ macro_use ]
extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	let expressions = vec! [
			
			Expression::Void,
			Expression::Value (NULL),
			
			NULL.into (),
			
			ProcedurePrimitive::Unimplemented.into (),
			SyntaxPrimitive::Unimplemented.into (),
			
			Expression::ContextDefine ("a".into (), ZERO.into ()),
			Expression::ContextSelect ("a".into ()),
			Expression::ContextUpdate ("a".into (), ONE.into ()),
			Expression::ContextSelect ("a".into ()),
			
			Expression::RegisterGet (0),
			Expression::RegisterSet (0, ONE.into ()),
			Expression::RegisterGet (0),
			
			Expression::ProcedurePrimitiveCall1 (BooleanPrimitive1::Not.into (), TRUE.into ()),
			Expression::ProcedurePrimitiveCallN (BooleanPrimitiveN::And.into (), vec_into! [TRUE, TRUE]),
			
		];
	
	let mut context = Context::new (None);
	let mut evaluator = Evaluator::new ();
	
	for expression in expressions {
		println! (">> {:?}", expression);
		let outcome = evaluator.evaluate_top (&mut context, &expression);
		match outcome {
			Ok (value) =>
				println! ("== {:?} {}", expression, value),
			Err (error) =>
				println! ("== {:?} {}", expression, error),
		}
	}
	
}

