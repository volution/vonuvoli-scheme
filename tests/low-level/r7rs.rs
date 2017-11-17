

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	if false {
		
		let context = Context::new (None);
		language_r7rs_initialize_context (&context) .unwrap ();
		
		println! ("{:#?}", context);
		
	} else {
		
		let definitions = language_r7rs_generate_definitions () .unwrap ();
		
		for (library, identifier, value) in definitions {
			match value {
				Value::ProcedurePrimitive (_) =>
					println! ("|| {} || procedure || {} || {:?} ||", library, identifier, value),
				Value::SyntaxPrimitive (SyntaxPrimitive::Auxiliary) =>
					println! ("|| {} || auxiliary-syntax || {} || {:?} ||", library, identifier, value),
				Value::SyntaxPrimitive (_) =>
					println! ("|| {} || syntax || {} || {:?} ||", library, identifier, value),
				_ =>
					println! ("|| {} || unknown || {} || {:?} ||", library, identifier, value),
			}
		}
		
	}
	
}
