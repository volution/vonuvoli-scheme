

use super::errors::exports::*;
use super::processes::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		process_spawn,
		
		process_wait, process_wait_check,
		process_run, process_run_check,
		
		process_stdin_get, process_stdout_get, process_stderr_get,
		
		process_status_check,
		
	};
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_wait (process : &Value, block : bool) -> (Outcome<ProcessStatus>) {
	let process = try_as_process_ref! (process);
	let status = try! (process.wait (block));
	succeed! (status);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_wait_check (process : &Value, block : bool) -> (Outcome<()>) {
	let status = try! (process_wait (process, block));
	return process_status_check (status);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_run (arguments : &[&Value]) -> (Outcome<ProcessStatus>) {
	let process = try! (process_spawn (arguments));
	let status = try! (process.wait (true));
	succeed! (status);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_run_check (arguments : &[&Value]) -> (Outcome<()>) {
	let status = try! (process_run (arguments));
	return process_status_check (status);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_stdin_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some! (process.stdin (), 0x0f6f72aa);
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_stdout_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some! (process.stdout (), 0xf389596d);
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn process_stderr_get (process : &Value) -> (Outcome<Value>) {
	let process = try_as_process_ref! (process);
	let port = try_some! (process.stderr (), 0xa1fc1b22);
	succeed! (port.into ());
}

