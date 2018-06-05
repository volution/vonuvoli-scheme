

use super::compiler::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::languages::exports::*;
use super::parser::exports::*;
use super::tools::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
use super::processes::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::main;
}




def_transcript_root! (transcript);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	if ! inputs.tool_commands.is_empty () {
		fail! (0x9a65fc47);
	}
	let (_identifier, source_path) = match inputs.tool_arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			(inputs.tool_arguments[0].to_str () .unwrap_or ("<script>"), Some (&inputs.tool_arguments[0])),
		_ =>
			fail! (0x1615e2d3),
	};
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let parameters = Some (try! (Parameters::new_standard (inputs.rest_arguments, inputs.rest_environment)));
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let parameters = None;
	
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
	
	let expressions = match parse_script (&source, None) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			trace_error! (transcript, 0x4b546a75 => "failed parsing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0xfcb837f6));
			succeed! (1);
		},
	};
	
	let expressions = match compile_script (&context, &expressions, None) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0xeaf9b7f2 => "failed compiling script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x0a87f029));
			succeed! (1);
		},
	};
	
	let expressions = match optimize_script (expressions, None) {
		Ok (expression) =>
			expression,
		Err (error) => {
			trace_error! (transcript, 0x89f48a5b => "failed optimizing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x5e46732f));
			succeed! (1);
		},
	};
	
	match evaluate_script (expressions.into_iter (), Some (&context), parameters.as_ref (), None) {
		Ok (()) =>
			succeed! (0),
		Err (error) =>
			match *error.internals_ref () {
				ErrorInternals::Exit (code, _) =>
					succeed! (code),
				#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
				ErrorInternals::Exec (ref configuration) =>
					match Process::exec (&configuration) {
						Ok (_) =>
							unreachable_0! (0x765f3997, github_issue_new),
						Err (error) => {
							trace_error! (transcript, 0x9d5d1d35 => "failed executing!" => (), error = &error);
							error.backtrace_report (tracer_error! (transcript, 0xd2bda446));
							succeed! (1);
						},
					},
				_ => {
					trace_error! (transcript, 0xe74be5c8 => "failed evaluating script!" => (), error = &error);
					error.backtrace_report (tracer_error! (transcript, 0x5c04a150));
					succeed! (1);
				}
			},
	}
}

