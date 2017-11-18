

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::generate_definitions as language_builtins_generate_definitions;
	pub use super::initialize_context as language_builtins_initialize_context;
}




pub fn initialize_context (context : &Context) -> (Outcome<()>) {
	let definitions = try! (generate_definitions ());
	for (identifier, value) in definitions {
		let binding = try! (context.define (&identifier));
		try! (binding.set (value));
		try! (binding.set_immutable ());
	}
	return Ok (());
}




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			("local", SyntaxPrimitiveN::Local.into ()), // https://docs.racket-lang.org/reference/local.html
			
			("and*", BooleanPrimitiveN::And.into ()),
			("or*", BooleanPrimitiveN::Or.into ()),
			("xor*", BooleanPrimitiveN::Xor.into ()),
			("nand*", BooleanPrimitiveN::Nand.into ()),
			("nor*", BooleanPrimitiveN::Nor.into ()),
			("nxor*", BooleanPrimitiveN::Nxor.into ()),
			
			
		]
		.into_iter ()
		.map (|(identifier, value)| (Symbol::from (identifier), value))
		.collect ();
	
	return Ok (definitions);
}

