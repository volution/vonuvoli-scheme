

#[ allow (unused_imports) ]
use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_values_error" ) ]
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
use super::processes::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::Outcome;
	pub use super::Error;
	pub use super::ErrorInternals;
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	pub use super::ErrorMessage;
	
	pub use super::error_generic;
	pub use super::error_unimplemented;
	pub use super::error_panic;
	
}




pub type Outcome<T> = Result<T, Error>;




#[ derive ( Clone ) ] // OK
pub struct Error ( StdRc<ErrorInternals> );

#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub enum ErrorInternals {
	Code (u64, Option<&'static str>, StdRefCell<bool>),
	#[ cfg ( feature = "vonuvoli_backtrace" ) ]
	WithBacktrace (u64, Option<&'static str>, Backtrace, StdRefCell<bool>),
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	WithMessage (Option<u64>, StdRc<StdBox<str>>, StdRefCell<bool>),
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	WithMessageAndArguments (Option<u64>, StdRc<StdBox<str>>, StdRc<StdBox<[Value]>>, StdRefCell<bool>),
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	WithValue (Option<u64>, Value, StdRefCell<bool>),
	Exit (u32, bool),
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Exec (StdBox<ProcessConfiguration>),
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
pub type ErrorMessage = StringImmutable;

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
pub type ErrorMessage = Symbol;


impl Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (code : u64, message : Option<&'static str>) -> (Error) {
		#[ cfg ( feature = "vonuvoli_backtrace" ) ]
		let internals = if ERRORS_WITH_BACKTRACE {
			ErrorInternals::WithBacktrace (code, message, Backtrace::new (), StdRefCell::new (false))
		} else {
			ErrorInternals::Code (code, message, StdRefCell::new (false))
		};
		#[ cfg ( not ( feature = "vonuvoli_backtrace" ) ) ]
		let internals = ErrorInternals::Code (code, message, StdRefCell::new (false));
		Error (StdRc::new (internals))
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_with_message (code : Option<u64>, message : StdRc<StdBox<str>>) -> (Error) {
		let internals = ErrorInternals::WithMessage (code, message, StdRefCell::new (false));
		Error (StdRc::new (internals))
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_with_message_and_arguments (code : Option<u64>, message : StdRc<StdBox<str>>, arguments : StdRc<StdBox<[Value]>>) -> (Error) {
		let internals = ErrorInternals::WithMessageAndArguments (code, message, arguments, StdRefCell::new (false));
		Error (StdRc::new (internals))
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_with_value (code : Option<u64>, value : Value) -> (Error) {
		let internals = ErrorInternals::WithValue (code, value, StdRefCell::new (false));
		Error (StdRc::new (internals))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_exit (code : u32, emergency : bool) -> (Error) {
		let internals = ErrorInternals::Exit (code, emergency);
		Error (StdRc::new (internals))
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_exec (configuration : ProcessConfiguration) -> (Error) {
		let internals = ErrorInternals::Exec (StdBox::new (configuration));
		Error (StdRc::new (internals))
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn into_value (self) -> (Value) {
		match *self.internals_ref () {
			ErrorInternals::WithValue (_, _, _) =>
				(),
			_ =>
				return self.into (),
		}
		match StdRc::try_unwrap (self.0) {
			Ok (internals) =>
				match internals {
					ErrorInternals::WithValue (_, value, _) =>
						return value,
					_ =>
						unreachable_0! (0x3411e156, github_issue_new),
				},
			Err (internals) =>
				match *internals.as_ref () {
					ErrorInternals::WithValue (_, ref value, _) =>
						return value.clone (),
					_ =>
						unreachable_0! (0xd2d8f3b9, github_issue_new),
				},
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_interceptable (&self) -> (bool) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessage (_, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessageAndArguments (_, _, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithValue (_, _, _) =>
				true,
			ErrorInternals::Exit (_, _) =>
				false,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_traceable (&self) -> (bool) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessage (_, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessageAndArguments (_, _, _, _) =>
				true,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithValue (_, _, _) =>
				true,
			ErrorInternals::Exit (_, _) =>
				false,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code (&self) -> (u64) {
		match *self.internals_ref () {
			ErrorInternals::Code (code, _, _) =>
				code,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (code, _, _, _) =>
				code,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessage (code, _, _) =>
				code.unwrap_or (0x0000000000000000),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessageAndArguments (code, _, _, _) =>
				code.unwrap_or (0x0000000000000000),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithValue (code, _, _) =>
				code.unwrap_or (0x0000000000000000),
			ErrorInternals::Exit (code, _) =>
				0xffffffff00000000 | (code as u64),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				0,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn message (&self) -> (Option<&str>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, message, _) =>
				message,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, message, _, _) =>
				message,
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessage (_, ref message, _) =>
				Some (message.as_ref ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessageAndArguments (_, ref message, _, _) =>
				Some (message.as_ref ()),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithValue (_, _, _) =>
				None,
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn message_clone (&self) -> (Option<ErrorMessage>) {
		match *self.internals_ref () {
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ErrorInternals::Code (_, message, _) =>
				option_map! (message, string_immutable_clone_str (message)),
			#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
			ErrorInternals::Code (_, message, _) =>
				option_map! (message, symbol_clone_str (message)),
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ErrorInternals::WithBacktrace (_, message, _, _) =>
				option_map! (message, string_immutable_clone_str (message)),
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
			ErrorInternals::WithBacktrace (_, message, _, _) =>
				option_map! (message, symbol_clone_str (message)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ErrorInternals::WithMessage (_, ref message, _) =>
				Some (StringImmutable::clone_rc (message)),
			#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
			ErrorInternals::WithMessage (_, ref message, _) =>
				Some (Symbol::clone_rc (message)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ErrorInternals::WithMessageAndArguments (_, ref message, _, _) =>
				Some (StringImmutable::clone_rc (message)),
			#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
			ErrorInternals::WithMessageAndArguments (_, ref message, _, _) =>
				Some (Symbol::clone_rc (message)),
			ErrorInternals::WithValue (_, _, _) =>
				None,
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments (&self) -> (Option<&[Value]>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, _) =>
				None,
			ErrorInternals::WithMessage (_, _, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments, _) =>
				Some (arguments.as_ref ()),
			ErrorInternals::WithValue (_, _, _) =>
				None,
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments_clone_array (&self) -> (Option<ArrayImmutable>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, _) =>
				None,
			ErrorInternals::WithMessage (_, _, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments, _) =>
				Some (ArrayImmutable::clone_rc (arguments)),
			ErrorInternals::WithValue (_, _, _) =>
				None,
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments_clone_values (&self) -> (Option<Values>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, _) =>
				None,
			ErrorInternals::WithMessage (_, _, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments, _) =>
				Some (Values::clone_rc (arguments)),
			ErrorInternals::WithValue (_, _, _) =>
				None,
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn was_reported (&self) -> (bool) {
		if let Some (marker) = self.reported_marker_ref () {
			let marker = try_or_panic_0! (marker.try_borrow (), 0x5338c490, github_issue_new);
			let marker = marker.deref ();
			*marker
		} else {
			false
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn set_reported (&self, reported : bool) -> () {
		if let Some (marker) = self.reported_marker_ref () {
			let mut marker = try_or_panic_0! (marker.try_borrow_mut (), 0x6e015706, github_issue_new);
			let marker = marker.deref_mut ();
			*marker = reported;
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn reported_marker_ref (&self) -> (Option<&StdRefCell<bool>>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_, _, ref marker) =>
				Some (marker),
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, _, ref marker) =>
				Some (marker),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessage (_, _, ref marker) =>
				Some (marker),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithMessageAndArguments (_, _, _, ref marker) =>
				Some (marker),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ErrorInternals::WithValue (_, _, ref marker) =>
				Some (marker),
			ErrorInternals::Exit (_, _) =>
				None,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ErrorInternals::Exec (_) =>
				None,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code_2 (&self) -> (u32, u32) {
		let code = self.code ();
		let code_1 = ((code & 0xffffffff00000000) >> 32) as u32;
		let code_2 = ((code & 0x00000000ffffffff) >> 0) as u32;
		(code_1, code_2)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ErrorInternals) {
		self.0.as_ref ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Error) -> (bool) {
		let self_code = self.code ();
		let other_code = other.code ();
		self_code == other_code
	}
	
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ allow (unused_variables) ]
	pub fn backtrace_report <T : Transcript + ?Sized> (&self, transcript : &TranscriptTracer<T>) -> () {
		match *self.internals_ref () {
			#[ cfg ( feature = "vonuvoli_backtrace" ) ]
			ErrorInternals::WithBacktrace (_, _, ref backtrace, _) =>
				backtrace.report (transcript),
			_ =>
				(),
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_generic (code : u32, message : Option<&'static str>) -> (Error) {
	Error::new (code as u64, message)
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_unimplemented (code : u32, message : Option<&'static str>) -> (Error) {
	Error::new (code as u64, message)
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_panic (code : u32, message : Option<&'static str>) -> (Error) {
	Error::new (code as u64, message)
}

