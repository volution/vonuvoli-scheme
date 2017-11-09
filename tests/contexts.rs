

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let mut context = Context::new (None);
	
	println! ("context -> {}", context);
	
	let mut binding_a1 = context.define_expect (&"a".into ());
	let mut binding_a2 = context.resolve_expect (&"a".into ());
	
	let mut binding_b1 = context.define_expect (&"b".into ());
	let mut binding_b2 = context.resolve_expect (&"b".into ());
	
	println! ("a1 -> {}", binding_a1);
	println! ("a2 -> {}", binding_a2);
	println! ("b1 -> {}", binding_b1);
	println! ("b2 -> {}", binding_b2);
	
	binding_a1.set (ZERO);
	binding_b1.set (ONE);
	
	println! ("a1 -> {}", binding_a1);
	println! ("a2 -> {}", binding_a2);
	println! ("b1 -> {}", binding_b1);
	println! ("b2 -> {}", binding_b2);
	
}

