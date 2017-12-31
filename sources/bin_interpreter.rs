

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate rust_scheme;

use rust_scheme::exports::*;
use rust_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("f2d87179");
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
fn main_0 () -> (Outcome<()>) {
	
	let mut transcript = io::stdout ();
	let mut source_stream = io::stdin ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	let mut source = StdString::new ();
	match source_stream.read_to_string (&mut source) {
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0xb498c8f0);
			fail! (0x68c9854b);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! parse !! => {:#?}\n", &error), 0x4b546a75);
			return Err (error);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! compile !! => {:#?}\n", &error), 0xeaf9b7f2);
			return Err (error);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! optimize !! => {:#?}\n", &error), 0x89f48a5b);
			return Err (error);
		},
	};
	
	match evaluate_script (&context, expressions.into_iter ()) {
		Ok (()) =>
			return Ok (()),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! evaluate !! => {:#?}\n", &error), 0xe74be5c8);
			return Err (error);
		},
	}
	
}

