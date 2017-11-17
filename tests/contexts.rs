

extern crate rust_scheme;

use rust_scheme::exports::*;



#[ test ]
fn test () -> () {
	
	let context = Context::new (None);
	
	println! ("context -> {:#?}", context);
	
	let a_identifier = &"a";
	let b_identifier = &symbol_clone_str ("b");
	
	let a_binding_1 = context.define_expect (a_identifier);
	let a_binding_2 = context.resolve_expect (a_identifier);
	
	let b_binding_1 = context.define_expect (b_identifier);
	let b_binding_2 = context.resolve_expect (b_identifier);
	
	println! ("a:1 -> {}", a_binding_1);
	println! ("a:2 -> {}", a_binding_2);
	println! ("b:1 -> {}", b_binding_1);
	println! ("b:2 -> {}", b_binding_2);
	
	a_binding_1.set (ZERO) .unwrap ();
	b_binding_1.set (ONE) .unwrap ();
	
	println! ("a:1 -> {}", a_binding_1);
	println! ("a:2 -> {}", a_binding_2);
	println! ("b:1 -> {}", b_binding_1);
	println! ("b:2 -> {}", b_binding_2);
	
	println! ("context -> {:#?}", context);
	
}

