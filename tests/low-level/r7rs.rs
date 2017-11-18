

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	if false {
		
		let context = Context::new (None);
		language_r7rs_initialize_context (&context) .expect ("2f7e5513");
		
		eprintln! ("{:#?}", context);
		
	} else {
		
		let definitions = language_r7rs_generate_definitions () .expect ("3bd1d93c");
		
		for (library, identifier, value) in definitions {
			match value {
				Value::ProcedurePrimitive (_) =>
					eprintln! ("|| {} || procedure || {} || {:?} ||", library, identifier, value),
				Value::SyntaxPrimitive (SyntaxPrimitive::Auxiliary) =>
					eprintln! ("|| {} || auxiliary-syntax || {} || {:?} ||", library, identifier, value),
				Value::SyntaxPrimitive (_) =>
					eprintln! ("|| {} || syntax || {} || {:?} ||", library, identifier, value),
				_ =>
					eprintln! ("|| {} || unknown || {} || {:?} ||", library, identifier, value),
			}
		}
		
	}
	
}
