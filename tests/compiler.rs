

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	
	let tests = vec! [
			
			("()", Ok (NULL.into ())),
			("#null", Ok (NULL.into ())),
			("#t", Ok (TRUE.into ())),
			("#f", Ok (FALSE.into ())),
			("0", Ok (ZERO.into ())),
			
			("null?", Ok (TypePrimitive1::IsNull.into ())),
			("(null? #null)", Ok (TRUE.into ())),
			("(null? 0)", Ok (FALSE.into ())),
			
			("boolean?", Ok (TypePrimitive1::IsBoolean.into ())),
			("(boolean? #t)", Ok (TRUE.into ())),
			("(boolean? 0)", Ok (FALSE.into ())),
			
			("number?", Ok (TypePrimitive1::IsNumber.into ())),
			("(number? 0)", Ok (TRUE.into ())),
			("(number? #null)", Ok (FALSE.into ())),
			
			("procedure?", Ok (TypePrimitive1::IsProcedure.into ())),
			("(procedure? +)", Ok (TRUE.into ())),
			("(procedure? 0)", Ok (FALSE.into ())),
			
			("not", Ok (BooleanPrimitive1::Not.into ())),
			("(not #t)", Ok (FALSE.into ())),
			("(not #f)", Ok (TRUE.into ())),
			("(not 0)", Err (error_generic (0x19768613))),
			
			("(zero? 0)", Ok (TRUE.into ())),
			("(zero? 1)", Ok (FALSE.into ())),
			("(positive? 1)", Ok (TRUE.into ())),
			("(positive? 0)", Ok (FALSE.into ())),
			("(positive? -1)", Ok (FALSE.into ())),
			("(negative? -1)", Ok (TRUE.into ())),
			("(negative? 0)", Ok (FALSE.into ())),
			("(negative? 1)", Ok (FALSE.into ())),
			("(even? 2)", Ok (TRUE.into ())),
			("(even? 1)", Ok (FALSE.into ())),
			("(odd? 1)", Ok (TRUE.into ())),
			("(odd? 2)", Ok (FALSE.into ())),
			
			("(zero? 0.0)", Ok (TRUE.into ())),
			("(zero? 1.0)", Ok (FALSE.into ())),
			("(positive? 1.0)", Ok (TRUE.into ())),
			("(positive? 0.0)", Ok (FALSE.into ())),
			("(positive? -1.0)", Ok (FALSE.into ())),
			("(negative? -1.0)", Ok (TRUE.into ())),
			("(negative? 0.0)", Ok (FALSE.into ())),
			("(negative? 1.0)", Ok (FALSE.into ())),
			("(even? 2.0)", Ok (TRUE.into ())),
			("(even? 1.0)", Ok (FALSE.into ())),
			("(odd? 1.0)", Ok (TRUE.into ())),
			("(odd? 2.0)", Ok (FALSE.into ())),
			
			("(zero? #f)", Err (error_generic (0x947fb339))),
			("(even? #f)", Err (error_generic (0x947fb339))),
			("(odd? #f)", Err (error_generic (0x947fb339))),
			("(positive? #f)", Err (error_generic (0x947fb339))),
			("(negative? #f)", Err (error_generic (0x947fb339))),
			
			("(min 0 1)", Ok (ZERO.into ())),
			("(max 0 1)", Ok (ONE.into ())),
			
			("+", Ok (ArithmeticPrimitiveN::Addition.into ())),
			("(+ 0 0)", Ok (ZERO.into ())),
			("(+ 0 1)", Ok (ONE.into ())),
			("(+ 1 0)", Ok (ONE.into ())),
			("(+ 0.0 0.0)", Ok (ZERO_REAL_POSITIVE.into ())),
			("(+ 0.0 0)", Ok (ZERO_REAL_POSITIVE.into ())),
			("(+ 0 0.0)", Ok (ZERO_REAL_POSITIVE.into ())),
			
			("'+", Ok (Symbol::from ("+").into ())),
			("'()", Ok (NULL.into ())),
			("'#t", Ok (TRUE.into ())),
			("'1", Ok (ONE.into ())),
			
			("`1", Ok (ONE.into ())),
			("`,1", Ok (ONE.into ())),
			("`(1)", Ok (pair_new (ONE.into (), NULL.into ()).into ())),
			("`(,1)", Ok (pair_new (ONE.into (), NULL.into ()).into ())),
			("`(,@1)", Ok (ONE.into ())),
			
			("(if #t 1 0)", Ok (ONE.into ())),
			("(if #f 1 0)", Ok (ZERO.into ())),
			("(if 1 1 0)", Ok (ONE.into ())),
			("(if 0 1 0)", Ok (ONE.into ())),
			("(if #null 1 0)", Ok (ONE.into ())),
			
			("(begin)", Ok (VOID.into ())),
			("(begin 0)", Ok (ZERO.into ())),
			("(begin 0 1)", Ok (ONE.into ())),
			
		];
	
	
	let context = language_r7rs_generate_context () .unwrap ();
	let evaluator = Evaluator::new ();
	
	
	for (source, expected) in tests {
		
		println! ();
		println! (">> {}", source);
		
		let data = parse_value (source) .unwrap ();
		println! ("## parse >> `{}`", data);
		
		let expression = compile (&context, &data) .unwrap ();
		println! ("## compile ##\n{:#?}", expression);
		
		let outcome = evaluator.evaluate_top (&context, &expression);
		println! ("## evaluate ##\n{:#?}", outcome);
		
		assert_eq! (outcome, expected);
		
		println! ();
	}
	
}
