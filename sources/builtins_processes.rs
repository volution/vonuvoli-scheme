

use super::builtins::exports::*;
use super::errors::exports::*;
use super::processes::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		process_spawn,
		process_spawn_extended,
		
		process_wait, process_wait_check,
		process_run, process_run_check,
		
		process_stdin_get, process_stdout_get, process_stderr_get,
		
		process_status_check,
		
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn (arguments : &[&Value]) -> (Outcome<Process>) {
	
	if arguments.is_empty () {
		fail! (0x1a08f645);
	}
	
	let executable = try_as_string_ref! (arguments[0]);
	let executable = executable.string_as_str ();
	
	let arguments = try_vec_map! (arguments[1..].iter (), argument, StringRef::try (argument));
	let arguments = vec_map! (arguments.iter (), argument, argument.string_as_str ());
	
	let configuration = ProcessConfiguration {
			executable :  executable,
			arguments : Some (arguments.as_ref ()),
			.. Default::default ()
		};
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn_extended (executable : &Value, arguments : Option<&Value>, options : Option<&Value>) -> (Outcome<Process>) {
	
	let executable = try_as_string_ref! (executable);
	let executable = executable.string_as_str ();
	
	// TODO:  Accept arrays as arguments!
	let arguments = option_map! (arguments, try! (vec_list_clone (arguments)));
	let arguments = option_ref_map! (arguments, try_vec_map! (arguments.iter (), argument, StringRef::try (argument)));
	let arguments = option_ref_map! (arguments, vec_map! (arguments.iter (), argument, argument.string_as_str ()));
	
	let mut configuration_stdin = Some (ProcessConfigurationStream::Null);
	let mut configuration_stdout = Some (ProcessConfigurationStream::Null);
	let mut configuration_stderr = Some (ProcessConfigurationStream::Inherited);
	
	let options = option_map! (options, try! (vec_list_clone (options)));
	if let Some (options) = options {
		for option in options {
			match option.class_match_as_ref () {
				ValueClassMatchAsRef::Pair (class) => {
					let option = try! (class.pair_ref ());
					let (option, value) = option.left_and_right ();
					match option.class_match_as_ref () {
						ValueClassMatchAsRef::Symbol (option) =>
							match option.string_as_str () {
								
								"stdin" | "stdout" | "stderr" => {
									let stream = match value.class_match_as_ref () {
										ValueClassMatchAsRef::Symbol (value) =>
											match value.string_as_str () {
												"inherited" =>
													ProcessConfigurationStream::Inherited,
												"piped" =>
													ProcessConfigurationStream::Piped,
												"null" =>
													ProcessConfigurationStream::Null,
												_ =>
													fail! (0x1ed756ce),
											},
										ValueClassMatchAsRef::Port (port) =>
											ProcessConfigurationStream::Port (port.clone ()),
										_ =>
											fail! (0x425000b7),
									};
									match option.string_as_str () {
										"stdin" =>
											configuration_stdin = Some (stream),
										"stdout" =>
											configuration_stdout = Some (stream),
										"stderr" =>
											configuration_stderr = Some (stream),
										_ =>
											fail_unreachable! (0xec023a35),
									}
								},
								
								// TODO:  Add support for:
								//        * argument0
								//        * environment-empty
								//        * environment-include
								//        * environment-exclude
								//        * working-directory
								//        * other descriptors
								
								_ =>
									fail! (0xeb97ad7f),
							},
						_ =>
							fail! (0x9d32ecbc),
					}
				},
				_ =>
					fail! (0xec86ca64),
			}
		}
	}
	
	let configuration = ProcessConfiguration {
			executable :  executable,
			arguments : option_ref_map! (arguments, arguments.as_ref ()),
			stdin : configuration_stdin.as_ref (),
			stdout : configuration_stdout.as_ref (),
			stderr : configuration_stderr.as_ref (),
			.. Default::default ()
		};
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_wait (process : &Value, block : bool) -> (Outcome<ProcessStatus>) {
	let process = try_as_process_ref! (process);
	let status = try! (process.wait (block));
	succeed! (status);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_wait_check (process : &Value, block : bool) -> (Outcome<()>) {
	let status = try! (process_wait (process, block));
	return process_status_check (status);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_run (arguments : &[&Value]) -> (Outcome<ProcessStatus>) {
	let process = try! (process_spawn (arguments));
	let status = try! (process.wait (true));
	succeed! (status);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_run_check (arguments : &[&Value]) -> (Outcome<()>) {
	let status = try! (process_run (arguments));
	return process_status_check (status);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_status_check (status : ProcessStatus) -> (Outcome<()>) {
	match status {
		ProcessStatus::Running =>
			fail_panic! (0x38f2a39a),
		ProcessStatus::Succeeded =>
			succeed! (()),
		ProcessStatus::Failed (_) =>
			fail! (0xef5554f1),
		ProcessStatus::Killed (_) =>
			fail! (0x5c2a4699),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_stdin_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some_2! (process.stdin (), 0x0f6f72aa);
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_stdout_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some_2! (process.stdout (), 0xf389596d);
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_stderr_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some_2! (process.stderr (), 0xa1fc1b22);
	succeed! (port.into ());
}

