

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::processes::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		process_spawn,
		process_spawn_extended,
		
		process_configure,
		process_configure_stream,
		
		process_wait, process_wait_check,
		process_run, process_run_check,
		
		process_stdin_get, process_stdout_get, process_stderr_get,
		
		process_status_check,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	pub use super::{
		
		PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE, PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_EMPTY_UNIQUE,
		PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE, PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_UNIQUE,
		PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE, PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE, PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_UNIQUE,
		PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE, PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE, PROCESS_PARAMETER_WORKING_DIRECTORY_UNIQUE,
		PROCESS_PARAMETER_STDIN_HANDLE_VALUE, PROCESS_PARAMETER_STDIN_HANDLE, PROCESS_PARAMETER_STDIN_UNIQUE,
		PROCESS_PARAMETER_STDOUT_HANDLE_VALUE, PROCESS_PARAMETER_STDOUT_HANDLE, PROCESS_PARAMETER_STDOUT_UNIQUE,
		PROCESS_PARAMETER_STDERR_HANDLE_VALUE, PROCESS_PARAMETER_STDERR_HANDLE, PROCESS_PARAMETER_STDERR_UNIQUE,
		
	};
	
}




#[ cfg ( not ( any ( feature = "vonuvoli_values_unique", feature = "vonuvoli_builtins_parameters" ) ) ) ]
type UniqueData = !;




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn (arguments : &[&Value], evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<Process>) {
	// TODO:  Accept arrays as arguments!
	
	if arguments.is_empty () {
		fail! (0x1a08f645);
	}
	
	let executable = try! (os_string_clone_coerce (arguments[0]));
	let arguments = try_vec_map! (arguments[1..].iter (), argument, os_string_clone_coerce (argument)) .into_boxed_slice ();
	
	let configuration = try! (process_configure (executable, Some (arguments), None, evaluator));
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_spawn_extended (executable : &Value, arguments : Option<&Value>, options : Option<&Value>, evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<Process>) {
	// TODO:  Accept arrays as arguments!
	
	let executable = try! (os_string_clone_coerce (executable));
	let arguments = option_map! (arguments, try! (vec_list_clone (arguments)));
	let arguments = option_map! (arguments, try_vec_map_into! (arguments, argument, os_string_clone_coerce (&argument)) .into_boxed_slice ());
	
	let configuration = try! (process_configure (executable, arguments, options, evaluator));
	
	return Process::spawn (&configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_configure (executable : ffi::OsString, arguments : Option<StdBox<[ffi::OsString]>>, options : Option<&Value>, evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<ProcessConfiguration>) {
	
	let mut option_argument0 = None;
	let mut option_working_directory = None;
	let mut option_environment_empty = None;
	let mut option_environment_include = None;
	let mut option_environment_exclude = None;
	let mut option_stdin = None;
	let mut option_stdout = None;
	let mut option_stderr = None;
	
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
								
								"argument0" | "arg0" => {
									if option_argument0.is_some () {
										fail! (0x7cc6ff5a);
									}
									option_argument0 = Some (value.clone ());
								},
								"working-directory" | "directory" => {
									if option_working_directory.is_some () {
										fail! (0xee4d2587);
									}
									option_working_directory = Some (value.clone ());
								},
								
								"environment-empty" | "env-empty" => {
									if option_environment_empty.is_some () {
										fail! (0x5d4c3efb);
									}
									option_environment_empty = Some (value.clone ());
								},
								"environment-include" | "env-include" => {
									if option_environment_include.is_some () {
										fail! (0xd2e1ec50);
									}
									option_environment_include = Some (value.clone ());
								},
								"environment-exclude" | "env-exclude" => {
									if option_environment_exclude.is_some () {
										fail! (0xd13e85de);
									}
									option_environment_exclude = Some (value.clone ());
								},
								
								"stdin" => {
									if option_stdin.is_some () {
										fail! (0xc4ac797f);
									}
									option_stdin = Some (value.clone ());
								},
								"stdout" => {
									if option_stdout.is_some () {
										fail! (0xc0a90402);
									}
									option_stdout = Some (value.clone ());
								},
								"stderr" => {
									if option_stderr.is_some () {
										fail! (0xb8929fd3);
									}
									option_stderr = Some (value.clone ());
								}
								
								// TODO:  Add support for:
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
	
	let argument0 = option_argument0;
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let working_directory = try! (parameter_resolve_value (option_working_directory, &PROCESS_PARAMETER_WORKING_DIRECTORY_UNIQUE, evaluator));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let environment_empty = try! (parameter_resolve_value (option_environment_empty, &PROCESS_PARAMETER_ENVIRONMENT_EMPTY_UNIQUE, evaluator));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let environment_include = try! (parameter_resolve_value (option_environment_include, &PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_UNIQUE, evaluator));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let environment_exclude = try! (parameter_resolve_value (option_environment_exclude, &PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_UNIQUE, evaluator));
	
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let working_directory = option_working_directory;
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let environment_empty = option_environment_empty;
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let environment_include = option_environment_include;
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let environment_exclude = option_environment_exclude;
	
	let argument0 = try! (os_string_clone_coerce_option (argument0.as_ref ()));
	let working_directory = try! (os_string_clone_coerce_option (working_directory.as_ref ()));
	
	let environment_empty = try! (boolean_coerce_option (environment_empty.as_ref ()));
	
	let environment_include = option_map! (environment_include, try! (vec_list_clone (&environment_include)));
	#[ allow (trivial_casts) ]
	let environment_include = option_map! (environment_include, try_vec_map_into! (environment_include, pair, {
			let pair = try_as_pair_ref! (&pair);
			let (name, value) = pair.left_and_right ();
			let name = try! (os_string_clone_coerce (name));
			let value = try! (os_string_clone_coerce (value));
			succeeded! ((name, value)) as Outcome<(ffi::OsString, ffi::OsString)>
		}) .into_boxed_slice ());
	
	let environment_exclude = option_map! (environment_exclude, try! (vec_list_clone (&environment_exclude)));
	#[ allow (trivial_casts) ]
	let environment_exclude = option_map! (environment_exclude, try_vec_map_into! (environment_exclude, name, {
			let name = try! (os_string_clone_coerce (&name));
			succeeded! (name) as Outcome<ffi::OsString>
		}) .into_boxed_slice ());
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let configuration_stdin = try! (process_configure_stream_0 (option_stdin, Some (ProcessConfigurationStream::Null), Some (&PROCESS_PARAMETER_STDIN_UNIQUE), evaluator));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let configuration_stdout = try! (process_configure_stream_0 (option_stdout, Some (ProcessConfigurationStream::Null), Some (&PROCESS_PARAMETER_STDOUT_UNIQUE), evaluator));
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let configuration_stderr = try! (process_configure_stream_0 (option_stderr, Some (ProcessConfigurationStream::Inherited), Some (&PROCESS_PARAMETER_STDERR_UNIQUE), evaluator));
	
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let configuration_stdin = try! (process_configure_stream_0 (option_stdin, Some (ProcessConfigurationStream::Null), None, evaluator));
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let configuration_stdout = try! (process_configure_stream_0 (option_stdout, Some (ProcessConfigurationStream::Null), None, evaluator));
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let configuration_stderr = try! (process_configure_stream_0 (option_stderr, Some (ProcessConfigurationStream::Inherited), None, evaluator));
	
	let configuration = ProcessConfiguration {
			executable :  executable,
			arguments : arguments,
			argument0 : argument0,
			working_directory : working_directory,
			environment_empty : environment_empty.unwrap_or (false),
			environment_include : environment_include,
			environment_exclude : environment_exclude,
			stdin : configuration_stdin,
			stdout : configuration_stdout,
			stderr : configuration_stderr,
			.. Default::default ()
		};
	
	succeed! (configuration);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_configure_stream (option : Value, _evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<ProcessConfigurationStream>) {
	match option.class_match_into () {
		ValueClassMatchInto::Symbol (value) =>
			match value.string_as_str () {
				"inherited" =>
					succeed! (ProcessConfigurationStream::Inherited),
				"piped" =>
					succeed! (ProcessConfigurationStream::Piped),
				"null" =>
					succeed! (ProcessConfigurationStream::Null),
				_ =>
					fail! (0x1ed756ce),
			},
		ValueClassMatchInto::Port (port) =>
			succeed! (ProcessConfigurationStream::Port (port)),
		_ =>
			fail! (0x425000b7),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn process_configure_stream_0 (option : Option<Value>, default : Option<ProcessConfigurationStream>, parameter : Option<&UniqueData>, evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<Option<ProcessConfigurationStream>>) {
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let value = if let Some (parameter) = parameter {
		try! (parameter_resolve_value (option, parameter, evaluator))
	} else {
		None
	};
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let value = {
		let _parameter = parameter;
		option
	};
	if let Some (value) = value {
		succeed! (Some (try! (process_configure_stream (value, evaluator))));
	} else if let Some (default) = default {
		succeed! (Some (default));
	} else {
		succeed! (None);
	}
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
pub fn process_run (arguments : &[&Value], evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<ProcessStatus>) {
	let process = try! (process_spawn (arguments, evaluator));
	let status = try! (process.wait (true));
	succeed! (status);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_run_check (arguments : &[&Value], evaluator : &mut Option<&mut EvaluatorContext>) -> (Outcome<()>) {
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




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE : u32 = 0xc1bbf12d;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EMPTY_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_EMPTY_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE : u32 = 0xa9d0d6c4;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_INCLUDE_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE : u32 = 0x1d10c0c4;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_ENVIRONMENT_EXCLUDE_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE : u32 = 0x59078131;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_WORKING_DIRECTORY_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_WORKING_DIRECTORY_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDIN_HANDLE_VALUE : u32 = 0xade85184;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDIN_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDIN_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDIN_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDIN_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDOUT_HANDLE_VALUE : u32 = 0xb2c1af1e;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDOUT_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDOUT_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDOUT_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDOUT_HANDLE_VALUE);

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDERR_HANDLE_VALUE : u32 = 0x75d7d8fa;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDERR_HANDLE : Handle = Handle::for_builtin (PROCESS_PARAMETER_STDERR_HANDLE_VALUE);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PROCESS_PARAMETER_STDERR_UNIQUE : UniqueData = UniqueData::for_parameter_builtin (PROCESS_PARAMETER_STDERR_HANDLE_VALUE);

