

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::processes::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		process_spawn,
		process_spawn_extended,
		process_configure,
		
		process_wait, process_wait_check,
		process_run, process_run_check,
		
		process_stdin_get, process_stdout_get, process_stderr_get,
		
		process_status_check,
		
	};
	
	pub use super::{
		
		PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_EMPTY_UNIQUE,
		PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_UNIQUE,
		PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_UNIQUE,
		PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE, PROCESS_PARAMETER_WORKING_DIRECTORY_UNIQUE,
		PROCESS_PARAMETER_STDIN_HANDLE, PROCESS_PARAMETER_STDIN_UNIQUE,
		PROCESS_PARAMETER_STDOUT_HANDLE, PROCESS_PARAMETER_STDOUT_UNIQUE,
		PROCESS_PARAMETER_STDERR_HANDLE, PROCESS_PARAMETER_STDERR_UNIQUE,
		
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn (arguments : &[&Value], evaluator : Option<&mut EvaluatorContext>) -> (Outcome<Process>) {
	
	if arguments.is_empty () {
		fail! (0x1a08f645);
	}
	
	let executable = try_as_string_ref! (arguments[0]);
	let executable = executable.string_as_str ();
	
	let arguments = try_vec_map! (arguments[1..].iter (), argument, StringRef::try (argument));
	let arguments = vec_map! (arguments.iter (), argument, argument.string_as_str ());
	let arguments  = Some (arguments.as_ref ());
	
	let configuration = try! (process_configure (executable, arguments, None, evaluator));
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn_extended (executable : &Value, arguments : Option<&Value>, options : Option<&Value>, evaluator : Option<&mut EvaluatorContext>) -> (Outcome<Process>) {
	
	let executable = try_as_string_ref! (executable);
	let executable = executable.string_as_str ();
	
	// TODO:  Accept arrays as arguments!
	let arguments = option_map! (arguments, try! (vec_list_clone (arguments)));
	let arguments = option_ref_map! (arguments, try_vec_map! (arguments.iter (), argument, StringRef::try (argument)));
	let arguments = option_ref_map! (arguments, vec_map! (arguments.iter (), argument, argument.string_as_str ()));
	let arguments = option_ref_map! (arguments, arguments.as_ref ());
	
	let configuration = try! (process_configure (executable, arguments, options, evaluator));
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_configure <'a> (executable : &'a str, arguments : Option<&'a [&'a str]>, options : Option<&Value>, _evaluator : Option<&mut EvaluatorContext>) -> (Outcome<ProcessConfiguration<'a>>) {
	
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
			arguments : arguments,
			stdin : configuration_stdin,
			stdout : configuration_stdout,
			stderr : configuration_stderr,
			.. Default::default ()
		};
	
	succeed! (configuration);
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
pub fn process_run (arguments : &[&Value], evaluator : Option<&mut EvaluatorContext>) -> (Outcome<ProcessStatus>) {
	let process = try! (process_spawn (arguments, evaluator));
	let status = try! (process.wait (true));
	succeed! (status);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_run_check (arguments : &[&Value], evaluator : Option<&mut EvaluatorContext>) -> (Outcome<()>) {
	let status = try! (process_run (arguments, evaluator));
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




const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE : u32 = 0xc1bbf12d;
pub const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE);
pub const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE);

const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE : u32 = 0xa9d0d6c4;
pub const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE);
pub const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE);

const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE : u32 = 0x1d10c0c4;
pub const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE);
pub const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE);

const PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE : u32 = 0x59078131;
pub const PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE);
pub const PROCESS_PARAMETER_WORKING_DIRECTORY_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE);

const PROCESS_PARAMETER_STDIN_HANDLE_VALUE : u32 = 0xade85184;
pub const PROCESS_PARAMETER_STDIN_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDIN_HANDLE_VALUE);
pub const PROCESS_PARAMETER_STDIN_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDIN_HANDLE_VALUE);

const PROCESS_PARAMETER_STDOUT_HANDLE_VALUE : u32 = 0xb2c1af1e;
pub const PROCESS_PARAMETER_STDOUT_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDOUT_HANDLE_VALUE);
pub const PROCESS_PARAMETER_STDOUT_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDOUT_HANDLE_VALUE);

const PROCESS_PARAMETER_STDERR_HANDLE_VALUE : u32 = 0x75d7d8fa;
pub const PROCESS_PARAMETER_STDERR_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDERR_HANDLE_VALUE);
pub const PROCESS_PARAMETER_STDERR_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDERR_HANDLE_VALUE);

