

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	if false {
		
		let mut context = Context::new (None);
		language_r7rs_initialize_context (&mut context) .expect ("536814be");
		
		println! ("{:#?}", context);
		
	} else {
		
		let definitions = language_r7rs_generate_definitions () .expect ("0ff72d3e");
		
		for (library, identifier, value) in definitions {
			match value {
				Value::ProcedurePrimitive (_) =>
					println! ("|| {} || procedure || {} ||", library, identifier),
				Value::SyntaxPrimitive (SyntaxPrimitive::Auxiliary) =>
					println! ("|| {} || auxiliary-syntax || {} ||", library, identifier),
				Value::SyntaxPrimitive (_) =>
					println! ("|| {} || syntax || {} ||", library, identifier),
				_ =>
					println! ("|| {} || unknown || {} ||", library, value),
			}
		}
		
	}
	
	
}
