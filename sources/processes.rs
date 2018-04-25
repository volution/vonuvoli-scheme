

use super::constants::exports::*;
use super::errors::exports::*;
use super::globals::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		Process, ProcessInternals,
		ProcessState,
		ProcessStatus,
		ProcessSignal,
	};
	
	pub use super::{
		ProcessConfiguration, ProcessConfigurationStream,
	};
}




#[ derive (Clone) ]
pub struct Process ( StdRc<StdRefCell<ProcessInternals>> );


#[ derive (Debug) ]
pub struct ProcessInternals {
	pub state : ProcessState,
	pub process : process::Child,
	pub process_id : ext::libc::pid_t,
	pub stdin : Option<Port>,
	pub stdout : Option<Port>,
	pub stderr : Option<Port>,
	pub handle : Handle,
}


#[ derive (Debug) ]
pub enum ProcessState {
	Running,
	Terminated (process::ExitStatus),
	Failed (Error),
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcessStatus {
	Running,
	Succeeded,
	Failed (u32),
	Killed (u32),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcessSignal {
	Terminate,
	Interrupt,
	Kill,
	Stop,
	Continue,
	Opaque (u32),
}


#[ derive (Debug, Default) ]
pub struct ProcessConfiguration {
	pub executable : ffi::OsString,
	pub argument0 : Option<ffi::OsString>,
	pub arguments : Option<StdBox<[ffi::OsString]>>,
	pub environment_empty : bool,
	pub environment_include : Option<StdBox<[(ffi::OsString, ffi::OsString)]>>,
	pub environment_exclude : Option<StdBox<[ffi::OsString]>>,
	pub working_directory : Option<ffi::OsString>,
	pub stdin : Option<ProcessConfigurationStream>,
	pub stdout : Option<ProcessConfigurationStream>,
	pub stderr : Option<ProcessConfigurationStream>,
}


#[ derive (Debug) ]
pub enum ProcessConfigurationStream {
	Inherited,
	Piped,
	Null,
	Port (Port),
	PortDescriptor (PortDescriptor),
	File (fs::File),
}




impl Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn spawn (configuration : &ProcessConfiguration) -> (Outcome<Process>) {
		let configuration = try! (configuration.build ());
		return Process::spawn_command (configuration);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn spawn_command (configuration : process::Command) -> (Outcome<Process>) {
		let mut configuration = configuration;
		let child = try_or_fail! (configuration.spawn (), 0x4b026d76);
		return Process::new (child);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (process : process::Child) -> (Outcome<Process>) {
		
		let mut process = process;
		let process_id = process.id () as ext::libc::pid_t;
		
		// TODO:  Add support for specifying the stdin/stdout/stderr buffer size!
		let mut stdin = None;
		let mut stdout = None;
		let mut stderr = None;
		mem::swap (&mut stdin, &mut process.stdin);
		mem::swap (&mut stdout, &mut process.stdout);
		mem::swap (&mut stderr, &mut process.stderr);
		let stdin = option_map! (stdin, {
			let descriptor = PortDescriptor::for_child_stdin (&stdin);
			try! (Port::new_native_writer_from_unbuffered (StdBox::new (stdin), None, descriptor))
		});
		let stdout = option_map! (stdout, {
			let descriptor = PortDescriptor::for_child_stdout (&stdout);
			try! (Port::new_native_reader_from_unbuffered (StdBox::new (stdout), None, descriptor))
		});
		let stderr = option_map! (stderr, {
			let descriptor = PortDescriptor::for_child_stderr (&stderr);
			try! (Port::new_native_reader_from_unbuffered (StdBox::new (stderr), None, descriptor))
		});
		
		let internals = ProcessInternals {
				state : ProcessState::Running,
				process : process,
				process_id : process_id,
				stdin : stdin,
				stdout : stdout,
				stderr : stderr,
				handle : process_handles_next (),
			};
		
		let process = Process (StdRc::new (StdRefCell::new (internals)));
		succeed! (process);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn id (&self) -> (Outcome<u32>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.process_id as u32);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn status (&self) -> (Outcome<ProcessStatus>) {
		let self_0 = try! (self.internals_ref ());
		return self_0.state.status ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn wait (&self, block : bool) -> (Outcome<ProcessStatus>) {
		{
			let self_0 = try! (self.internals_ref ());
			match self_0.state {
				ProcessState::Running =>
					if ! block {
						return self_0.state.status ();
					},
				ProcessState::Terminated (_) =>
					return self_0.state.status (),
				ProcessState::Failed (_) =>
					return self_0.state.status (),
			}
		}
		{
			let mut self_0 = try! (self.internals_ref_mut ());
			let exit = if block {
				match self_0.process.wait () {
					Ok (exit) =>
						Ok (Some (exit)),
					Err (_) =>
						failed! (0x72e7f374),
				}
			} else {
				match self_0.process.try_wait () {
					Ok (exit) =>
						Ok (exit),
					Err (_) =>
						failed! (0x1721311f),
				}
			};
			match exit {
				Ok (None) =>
					(),
				Ok (Some (exit)) =>
					self_0.state = ProcessState::Terminated (exit),
				Err (error) =>
					self_0.state = ProcessState::Failed (error),
			}
			return self_0.state.status ();
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn signal (&self, signal : ProcessSignal) -> (Outcome<()>) {
		{
			let self_0 = try! (self.internals_ref ());
			match self_0.state {
				ProcessState::Running =>
					(),
				_ =>
					fail! (0x248e1acf),
			}
			try! (libc_kill (self_0.process_id, signal.code ()));
		}
		try! (self.wait (false));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn stdin (&self) -> (Outcome<Option<Port>>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.stdin.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn stdout (&self) -> (Outcome<Option<Port>>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.stdout.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn stderr (&self) -> (Outcome<Option<Port>>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.stderr.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<ProcessInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0xb01737f5));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<ProcessInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0x463b17c3));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Process) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}




impl ProcessState {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn status (&self) -> (Outcome<ProcessStatus>) {
		match *self {
			ProcessState::Running =>
				succeed! (ProcessStatus::Running),
			ProcessState::Terminated (ref exit) => {
				if let Some (code) = exit.code () {
					if code == 0 {
						succeed! (ProcessStatus::Succeeded);
					} else {
						succeed! (ProcessStatus::Failed (code as u32));
					}
				} else if let Some (code) = unix_process::ExitStatusExt::signal (exit) {
					succeed! (ProcessStatus::Killed (code as u32));
				} else {
					fail_panic! (0x4075cf67, github_issue_new);
				}
			},
			ProcessState::Failed (ref error) =>
				return Err (error.clone ()),
		}
	}
}




impl ProcessStatus {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code (&self) -> (Option<i32>) {
		match *self {
			ProcessStatus::Running =>
				None,
			ProcessStatus::Succeeded =>
				Some (0),
			ProcessStatus::Failed (code) =>
				Some ((0 + code) as i32),
			ProcessStatus::Killed (code) =>
				Some ((0 - code) as i32),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (&self) -> (Value) {
		match *self {
			ProcessStatus::Running =>
				FALSE_VALUE,
			ProcessStatus::Succeeded =>
				TRUE_VALUE,
			ProcessStatus::Failed (code) =>
				((0 + code) as i32) .into (),
			ProcessStatus::Killed (code) =>
				((0 - code) as i32) .into (),
		}
	}
}




impl ProcessSignal {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code (&self) -> (i32) {
		match *self {
			ProcessSignal::Interrupt =>
				ext::libc::SIGINT,
			ProcessSignal::Terminate =>
				ext::libc::SIGTERM,
			ProcessSignal::Kill =>
				ext::libc::SIGKILL,
			ProcessSignal::Stop =>
				ext::libc::SIGSTOP,
			ProcessSignal::Continue =>
				ext::libc::SIGCONT,
			ProcessSignal::Opaque (value) =>
				value as i32,
		}
	}
}




impl ProcessConfiguration {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn build (&self) -> (Outcome<process::Command>) {
		let mut command = process::Command::new (&self.executable);
		if let Some (ref _argument0) = self.argument0 {
			fail! (0x6c2f442c);
		}
		if let Some (ref arguments) = self.arguments {
			for argument in arguments.iter () {
				command.arg (argument);
			}
		}
		if self.environment_empty {
			command.env_clear ();
		}
		if let Some (ref excludes) = self.environment_exclude {
			for variable in excludes.iter () {
				command.env_remove (variable);
			}
		}
		if let Some (ref includes) = self.environment_include {
			for &(ref variable, ref value) in includes.iter () {
				command.env (variable, value);
			}
		}
		if let Some (ref path) = self.working_directory {
			command.current_dir (path);
		}
		if let Some (ref stdin) = self.stdin {
			let stdin = try! (stdin.build ());
			command.stdin (stdin);
		}
		if let Some (ref stdout) = self.stdout {
			let stdout = try! (stdout.build ());
			command.stdout (stdout);
		}
		if let Some (ref stderr) = self.stderr {
			let stderr = try! (stderr.build ());
			command.stderr (stderr);
		}
		succeed! (command);
	}
}




impl ProcessConfigurationStream {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn build (&self) -> (Outcome<process::Stdio>) {
		match *self {
			ProcessConfigurationStream::Inherited =>
				succeed! (process::Stdio::inherit ()),
			ProcessConfigurationStream::Piped =>
				succeed! (process::Stdio::piped ()),
			ProcessConfigurationStream::Null =>
				succeed! (process::Stdio::null ()),
			ProcessConfigurationStream::PortDescriptor (ref descriptor) =>
				match *descriptor {
					PortDescriptor::RawFd (descriptor) =>
						unsafe {
							succeed! (unix_io::FromRawFd::from_raw_fd (descriptor));
						},
					PortDescriptor::Stdin =>
						unsafe {
							succeed! (unix_io::FromRawFd::from_raw_fd (unix_io::AsRawFd::as_raw_fd (&io::stdin ())));
						},
					PortDescriptor::Stdout =>
						unsafe {
							succeed! (unix_io::FromRawFd::from_raw_fd (unix_io::AsRawFd::as_raw_fd (&io::stdout ())));
						},
					PortDescriptor::Stderr =>
						unsafe {
							succeed! (unix_io::FromRawFd::from_raw_fd (unix_io::AsRawFd::as_raw_fd (&io::stderr ())));
						},
				},
			ProcessConfigurationStream::Port (ref port) => {
				let descriptor = try! (port.descriptor ());
				let descriptor = try_some! (descriptor, 0x2426d518);
				let configuration = ProcessConfigurationStream::PortDescriptor (descriptor);
				return configuration.build ();
			},
			ProcessConfigurationStream::File (ref file) => {
				let descriptor = PortDescriptor::for_file (file);
				let descriptor = try_some! (descriptor, 0x44aa4b5b);
				let configuration = ProcessConfigurationStream::PortDescriptor (descriptor);
				return configuration.build ();
			},
		}
	}
}

