

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("eefe92a1");
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
			fail! (0x97ad292f),
	};
	
	let transcript_backend = io::stdout ();
	let mut transcript = transcript_backend.lock ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0x771891a5);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0x112466b9);
			fail! (0x1cc9038b);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! parse !! => {:#?}\n", &error), 0xf25f2f7b);
			try_or_fail! (error.backtrace_report (&mut transcript), 0xa967a8dc);
			return Err (error);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! compile !! => {:#?}\n", &error), 0xb181d326);
			try_or_fail! (error.backtrace_report (&mut transcript), 0x42fa0705);
			return Err (error);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! optimize !! => {:#?}\n", &error), 0xf591ef0e);
			try_or_fail! (error.backtrace_report (&mut transcript), 0x8ffda0e5);
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

