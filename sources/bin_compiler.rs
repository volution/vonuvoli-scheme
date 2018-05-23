

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;


def_transcript_root! (transcript);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : super::ToolInputs) -> (Outcome<u32>) {
	
	if ! inputs.tool_commands.is_empty () {
		fail! (0x455c1320);
	}
	let (_identifier, source_path) = match inputs.tool_arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			(inputs.tool_arguments[0].to_str () .unwrap_or ("<script>"), Some (&inputs.tool_arguments[0])),
		_ =>
			fail! (0x97ad292f),
	};
	if ! inputs.rest_arguments.is_empty () {
		fail! (0x37553f8b);
	}
	
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
			trace_error! (transcript, 0x112466b9 => "failed loading script!" => (), error = &error);
			succeed! (1);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			trace_error! (transcript, 0xf25f2f7b => "failed parsing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0xa967a8dc));
			succeed! (1);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0xb181d326 => "failed compiling script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x42fa0705));
			succeed! (1);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0xf591ef0e => "failed optimizing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x8ffda0e5));
			succeed! (1);
		},
	};
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	for expression in expressions.into_iter () {
		try_or_fail! (write! (stream, "\n--------------------------------------------------------------------------------\n"), 0x25f931a1);
		try_or_fail! (write! (stream, "{:#?}\n", &expression), 0x829a2b78);
		try_or_fail! (write! (stream, "--------------------------------------------------------------------------------\n\n"), 0xbfaa9836);
	}
	
	succeed! (0);
}

