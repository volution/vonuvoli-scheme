

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::generate_binding_templates as language_builtins_generate_binding_templates;
	pub use super::generate_definitions as language_builtins_generate_definitions;
}




pub fn generate_binding_templates () -> (Outcome<StdVec<ContextBindingTemplate>>) {
	
	let definitions = try! (generate_definitions ());
	
	let templates = vec_map! (
			definitions,
			(identifier, value),
			ContextBindingTemplate {
					identifier : identifier,
					value : Some (value),
					immutable : true,
				}
		);
	
	succeed! (templates);
}




pub fn generate_definitions () -> (Outcome<StdVec<(Symbol, Value)>>) {
	
	let definitions = vec! [
			
			
			("locals", SyntaxPrimitiveN::Locals.into ()), // https://docs.racket-lang.org/reference/local.html
			
			("and", BooleanPrimitiveN::And.into ()),
			("or", BooleanPrimitiveN::Or.into ()),
			("xor", BooleanPrimitiveN::Xor.into ()),
			("nand", BooleanPrimitiveN::Nand.into ()),
			("nor", BooleanPrimitiveN::Nor.into ()),
			("nxor", BooleanPrimitiveN::Nxor.into ()),
			
			("string-reverse", StringPrimitive1::StringReverse.into ()),
			
			("bytevector-reverse", BytesPrimitive1::BytesReverse.into ()),
			("bytevector-u8-fill", BytesPrimitiveN::BytesSliceFill.into ()),
			
			("vector-reverse", ArrayPrimitive1::ArrayReverse.into ()),
			
		];
	
	let definitions = vec_map! (
			definitions,
			(identifier, value),
			(Symbol::from (identifier), value));
	
	succeed! (definitions);
}

