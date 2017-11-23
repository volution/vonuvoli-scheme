

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let definitions = language_r7rs_generate_definitions () .expect ("3bd1d93c");
	
	for (library, identifier, value) in definitions {
		match value {
			Value::ProcedurePrimitive (_, _) =>
				println! ("|| {} || procedure || {} || {:?} ||", library, identifier, value),
			Value::SyntaxPrimitive (SyntaxPrimitive::Auxiliary, _) =>
				println! ("|| {} || auxiliary-syntax || {} || {:?} ||", library, identifier, value),
			Value::SyntaxPrimitive (_, _) =>
				println! ("|| {} || syntax || {} || {:?} ||", library, identifier, value),
			_ =>
				println! ("|| {} || unknown || {} || {:?} ||", library, identifier, value),
		}
	}
	
}
