

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	
	let tests = vec! [
			
			"()", "#null", "#t", "#f", "0",
			
			"a", "+",
			
			"(+ 1 2)",
			
		];
	
	
	let mut context = language_r7rs_generate_context () .unwrap ();
	let mut evaluator = Evaluator::new ();
	
	
	for source in tests {
		
		println! ();
		println! (">> {}", source);
		
		let data = parse (source) .unwrap ();
		println! ("## parse >> `{}`", data);
		
		let expression = compile (&context, data) .unwrap ();
		println! ("## compile ##\n{:#?}", expression);
		
		let outcome = evaluator.evaluate_top (&mut context, &expression) .map_err (|error| Value::from (error));
		println! ("## evaluate ##\n{:#?}", outcome);
		
		println! ();
	}
	
}
