

use super::contexts::exports::*;
use super::errors::exports::*;
use super::libraries::exports::*;
use super::tests::exports::*;
use super::tools::exports::*;
use super::transcript::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::main;
}




def_transcript_root! (transcript);




pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	if ! inputs.tool_commands.is_empty () {
		fail! (0xa290f927);
	}
	let (identifier, source_path) = match inputs.tool_arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			(inputs.tool_arguments[0].to_str () .unwrap_or ("<script>"), Some (&inputs.tool_arguments[0])),
		_ =>
			fail! (0x9f085526),
	};
	if ! inputs.rest_arguments.is_empty () {
		fail! (0xefaf5921);
	}
	
	let context = Context::new (None);
	r#try! (context.define_all (r#try! (library_r7rs_generate_binding_templates ()) .as_ref ()));
	r#try! (context.define_all (r#try! (library_builtins_generate_binding_templates ()) .as_ref ()));
	r#try! (context.define_all_with_prefix (r#try! (library_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0xa9216a08);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			trace_error! (transcript, 0x367496ef => "failed loading script!" => (), error = &error);
			succeed! (1);
		},
	}
	
	match execute_tests_main (identifier, &source, Some (&context), Some (transcript.backend ()), None) {
		Ok (()) =>
			succeed! (0),
		Err (error) => {
			trace_error! (transcript, 0x367496ef => "failed testing script!" => (), error = &error);
			// NOTE:  The `execute_tests_main` always reports the backtrace itself.
			// error.backtrace_report (tracer_error! (transcript, 0x5681c4ca));
			succeed! (1);
		},
	}
}

