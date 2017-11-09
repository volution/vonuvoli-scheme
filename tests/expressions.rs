

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let expressions = vec! [
			
			Expression::Void,
			
			NULL.into (),
			VOID.into (),
			UNDEFINED.into (),
			
			ZERO.into (),
			ONE.into (),
			
			2.into (),
			'a'.into (),
			"a".into (),
			
		];
	
	for expression in expressions {
		println! (">> {:?}", expression);
	}
	
}

