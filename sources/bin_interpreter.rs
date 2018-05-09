

#![ no_implicit_prelude ]
#![ feature (stmt_expr_attributes) ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;

def_transcript_root! (transcript);




fn main () -> () {
	execute_main (main_0, &transcript);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_0 () -> (Outcome<u32>) {
	
	let (interpreter_arguments, process_arguments) = try! (parse_os_arguments ());
	let (_interpreter_environment, process_environment) = try! (parse_os_environment ());
	
	let (_identifier, source_path) = match interpreter_arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			("<stdin>", None),
		2 =>
			(interpreter_arguments[1].to_str () .unwrap_or ("<script>"), Some (&interpreter_arguments[1])),
		_ =>
			fail! (0x1615e2d3),
	};
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let parameters = Some (try! (Parameters::new_standard (process_arguments, process_environment)));
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let parameters = { mem::drop (process_arguments); mem::drop (process_environment); None };
	
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
			trace_error! (transcript, 0xfaa982e1 => "failed loading script!" => (), error = &error);
			succeed! (1);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			trace_error! (transcript, 0x4b546a75 => "failed parsing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0xfcb837f6));
			succeed! (1);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0xeaf9b7f2 => "failed compiling script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x0a87f029));
			succeed! (1);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0x89f48a5b => "failed optimizing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x5e46732f));
			succeed! (1);
		},
	};
	
	match evaluate_script (expressions.into_iter (), Some (&context), parameters.as_ref ()) {
		Ok (()) =>
			succeed! (0),
		Err (error) =>
			match *error.internals_ref () {
				ErrorInternals::Exit (code, _) =>
					succeed! (code),
				_ => {
					trace_error! (transcript, 0xe74be5c8 => "failed evaluating script!" => (), error = &error);
					error.backtrace_report (tracer_error! (transcript, 0x5c04a150));
					succeed! (1);
				}
			},
	}
}

