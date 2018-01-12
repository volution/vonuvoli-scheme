

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("eefe92a1");
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0xfda55994);
			fail! (0x4d3a5ff8);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! parse !! => {:#?}\n", &error), 0xf25f2f7b);
			return Err (error);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! compile !! => {:#?}\n", &error), 0xb181d326);
			return Err (error);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! optimize !! => {:#?}\n", &error), 0xf591ef0e);
			return Err (error);
		},
	};
	
	for expression in expressions.into_iter () {
		try_or_fail! (write! (transcript, "\n--------------------------------------------------------------------------------\n"), 0x25f931a1);
		try_or_fail! (write! (transcript, "{:#?}\n", &expression), 0x829a2b78);
		try_or_fail! (write! (transcript, "--------------------------------------------------------------------------------\n\n"), 0xbfaa9836);
	}
	
	return Ok (());
}

