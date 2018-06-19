

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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	if ! inputs.tool_commands.is_empty () {
		fail! (0xc5685396);
	}
	let (identifier, source_path) = match inputs.tool_arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			(inputs.tool_arguments[0].to_str () .unwrap_or ("<script>"), Some (&inputs.tool_arguments[0])),
		_ =>
			fail! (0x351c9060),
	};
	if ! inputs.rest_arguments.is_empty () {
		fail! (0xd90cbe83);
	}
	
	let context = Context::new (None);
	try! (context.define_all (try! (library_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (library_builtins_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all_with_prefix (try! (library_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0xe5edc9fc);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			trace_error! (transcript, 0x8968ee53 => "failed loading script!" => (), error = &error);
			succeed! (1);
		},
	}
	
	match benchmark_tests_main (identifier, &source, Some (&context), None, Some (transcript.backend ()), None, None) {
		Ok (()) =>
			succeed! (0),
		Err (error) => {
			trace_error! (transcript, 0xdec8989e => "failed benchmarking script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0xafac76dd));
			succeed! (1);
		},
	}
}

