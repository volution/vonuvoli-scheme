

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("f2d87179");
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_0 () -> (Outcome<()>) {
	
	let arguments = env::args () .collect::<StdVec<_>> ();
	let (_identifier, source_path) = match arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			("<stdin>", None),
		2 =>
			(arguments[1].as_str (), Some (&arguments[1])),
		_ =>
			fail! (0x1615e2d3),
	};
	
	let transcript_backend = io::stdout ();
	let mut transcript = transcript_backend.lock ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0x72d7e122);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0xfaa982e1);
			fail! (0x08f9fa86);
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
		Err (error) =>
			match *error.internals_ref () {
				ErrorInternals::Exit (code, _) => {
					let code = if code <= 255 {
						code
					} else {
						try_or_fail! (write! (transcript, "[ee]  exit code out of range {};  using 255!\n", code), 0x2daa4ba6);
						255
					};
					process::exit (code as i32);
				},
				_ => {
					try_or_fail! (write! (transcript, "!! evaluate !! => {:#?}\n", &error), 0xe74be5c8);
					process::exit (1);
				}
			},
	}
	
}

