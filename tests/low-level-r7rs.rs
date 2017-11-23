

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let definitions = language_r7rs_generate_definitions () .expect ("3bd1d93c");
	
	for (library, identifier, value) in definitions {
		match value.class () {
			ValueClass::ProcedurePrimitive =>
				println! ("|| {} || procedure || {} || {:?} ||", library, identifier, value),
			ValueClass::SyntaxPrimitive if *SyntaxPrimitive::as_ref (&value) == SyntaxPrimitive::Auxiliary  =>
				println! ("|| {} || auxiliary-syntax || {} || {:?} ||", library, identifier, value),
			ValueClass::SyntaxPrimitive =>
				println! ("|| {} || syntax || {} || {:?} ||", library, identifier, value),
			_ =>
				println! ("|| {} || unknown || {} || {:?} ||", library, identifier, value),
		}
	}
	
}
