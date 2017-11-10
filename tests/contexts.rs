

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let mut context = Context::new (None);
	
	println! ("context -> {:#?}", context);
	
	let a_identifier = &"a";
	let b_identifier = &symbol_from_slice ("b");
	
	let mut a_binding_1 = context.define_expect (a_identifier);
	let mut a_binding_2 = context.resolve_expect (a_identifier);
	
	let mut b_binding_1 = context.define_expect (b_identifier);
	let mut b_binding_2 = context.resolve_expect (b_identifier);
	
	println! ("a:1 -> {}", a_binding_1);
	println! ("a:2 -> {}", a_binding_2);
	println! ("b:1 -> {}", b_binding_1);
	println! ("b:2 -> {}", b_binding_2);
	
	a_binding_1.set (ZERO);
	b_binding_1.set (ONE);
	
	println! ("a:1 -> {}", a_binding_1);
	println! ("a:2 -> {}", a_binding_2);
	println! ("b:1 -> {}", b_binding_1);
	println! ("b:2 -> {}", b_binding_2);
	
	println! ("context -> {:#?}", context);
	
}

