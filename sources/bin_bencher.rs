

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("ce1cf1fd");
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
fn main_0 () -> (Outcome<()>) {
	
	let transcript_backend = io::stdout ();
	let mut transcript = transcript_backend.lock ();
	let mut source_stream = io::stdin ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all_with_prefix (try! (language_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let mut source = StdString::new ();
	match source_stream.read_to_string (&mut source) {
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0x2b7ccee9);
			fail! (0x70b2c4b1);
		},
	}
	
	return benchmark_tests_main ("<stdin>", &source, Some (context), None, Some (&mut transcript), None, None);
}

