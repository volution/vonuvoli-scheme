

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;
use super::builtins::exports::*;
use super::constants::exports::*;
use super::primitives::exports::*;
use super::conversions::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			error_exit,
		};
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	pub use super::{
			error_message, error_arguments_as_list,
			error_build_0, error_build_1, error_build_2, error_build_3, error_build_4, error_build_n,
			error_coerce, error_coerce_from,
		};
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::{
			error_arguments_as_array,
		};
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::{
			error_arguments_as_values,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	pub use super::{
			parameter_build,
			parameter_resolve,
			parameter_configure,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	pub use super::{
			transcript_trace_g,
		};
	
	pub use super::{
			abort_g,
		};
	
	pub use super::{
			process_argument,
			process_arguments,
			process_arguments_count,
			process_environment_variable,
			process_environment_variables,
		};
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "blake2-rfc" ) ]
	pub use super::{
			process_environment_fingerprint,
		};
	
	pub use super::{
			posix_timestamp,
			jiffies_timestamp,
			jiffies_per_second,
			pause,
		};
	
	#[ cfg ( feature = "tempfile" ) ]
	pub use super::{
			temporary_build,
		};
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_message (error : &Value) -> (Outcome<ErrorMessage>) {
	let error = try_as_error_ref! (error);
	if let Some (message) = error.message_clone () {
		succeed! (message);
	} else {
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		succeed! (string_immutable_new_empty ());
		#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
		succeed! (symbol_clone_str (""));
	}
}


#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_arguments_as_list (error : &Value) -> (Outcome<Value>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments () {
		let arguments = list_build_n (arguments, None, Some (true));
		succeed! (arguments);
	} else {
		succeed! (list_empty ());
	}
}


#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub fn error_arguments_as_array (error : &Value) -> (Outcome<ArrayImmutable>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments_clone_array () {
		succeed! (arguments);
	} else {
		succeed! (array_immutable_new_empty ());
	}
}


#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg ( feature = "vonuvoli_values_values" ) ]
pub fn error_arguments_as_values (error : &Value) -> (Outcome<Values>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments_clone_values () {
		succeed! (arguments);
	} else {
		succeed! (values_new_empty ());
	}
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_0 (code : Option<u64>, message : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let error = Error::new_with_message (code, message);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_1 (code : Option<u64>, message : &Value, argument_1 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_2 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_3 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_4 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value, argument_4 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone (), argument_4.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_build_n (code : Option<u64>, message : &Value, arguments : &[impl StdAsRef<Value>]) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = r#try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments = vec_clone_slice_ref (arguments);
	let arguments = StdRc::new (arguments.into_boxed_slice ());
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}




pub fn error_exit (code : Option<&Value>, emergency : bool) -> (Outcome<Error>) {
	
	fn build (code : &Value, emergency : bool) -> (Outcome<Error>) {
		match code.kind_match_as_ref () {
			ValueKindMatchAsRef::NumberInteger (value) =>
				succeed! (Error::new_exit (r#try! (value.try_to_u32 ()), emergency)),
			ValueKindMatchAsRef::Boolean (value) =>
				if value.value () {
					succeed! (Error::new_exit (0, emergency));
				} else {
					succeed! (Error::new_exit (1, emergency));
				},
			_ =>
				fail! (0x33ebdcdc),
		}
	}
	
	if let Some (code) = code {
		match build (code, emergency) {
			outcome @ Ok (_) =>
				return outcome,
			outcome @ Err (_) =>
				if ! emergency {
					return outcome;
				} else {
					succeed! (Error::new_exit (1, emergency));
				},
		}
	} else {
		succeed! (Error::new_exit (0, emergency));
	}
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_coerce (code : Option<u64>, value : &Value) -> (Error) {
	let value = value.clone ();
	return error_coerce_from (code, value);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
pub fn error_coerce_from (code : Option<u64>, value : Value) -> (Error) {
	match value.kind_match_into () {
		ValueKindMatchInto::Error (error) =>
			return error,
		kind => {
			let value = kind.value ();
			let error = Error::new_with_value (code, value);
			return error;
		},
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn parameter_build (identifier : Option<&Value>, global : Option<&Value>, converter : Option<&Value>, immutable : Option<bool>, _evaluator : &mut EvaluatorContext) -> (Outcome<Parameter>) {
	let identifier = option_map! (identifier, try_as_symbol_ref! (identifier)) .cloned ();
	let global = if let Some (global) = global {
		match global.kind () {
			ValueKind::Undefined =>
				None,
			_ =>
				Some (global.clone ()),
		}
	} else {
		None
	};
	let conversion = if let Some (converter) = converter {
		match converter.class_match_as_ref () {
			ValueClassMatchAsRef::Boolean (converter) =>
				if ! converter.value () {
					ParameterConversion::None
				} else {
					fail! (0x0037d553);
				},
			ValueClassMatchAsRef::Procedure (_) =>
				ParameterConversion::OnConfigure (converter.clone ()),
			_ =>
				fail! (0xb3103841),
		}
	} else {
		ParameterConversion::None
	};
	let immutable = immutable.unwrap_or (PARAMETER_NEW_IMMUTABLE);
	let parameter = Parameter::new (identifier, global, conversion, immutable);
	succeed! (parameter);
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn parameter_resolve (parameter : &Value, default : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_resolve (parameter, default),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return r#try! (evaluator.parameters ()) .resolve_stdin_value_or (default),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return r#try! (evaluator.parameters ()) .resolve_stdout_value_or (default),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return r#try! (evaluator.parameters ()) .resolve_stderr_value_or (default),
				_ =>
					fail! (0x4ce4065b),
			},
		_ =>
			fail! (0xf44e6fc0),
	}
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn parameter_configure (parameter : &Value, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_configure (parameter, value),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return r#try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (value)),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return r#try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (value)),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return r#try! (evaluator.parameters ()) .configure_stderr (try_as_port_ref! (value)),
				_ =>
					fail! (0x5970c2fd),
			},
		_ =>
			fail! (0xb05cfc27),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn process_argument (index : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	let index = r#try! (count_coerce (index));
	let arguments = r#try! (r#try! (evaluator.parameters ()) .resolve_process_arguments ());
	if index == 0 {
		succeed! (FALSE_VALUE);
	}
	let index = index - 1;
	let argument = try_some! (arguments.get (index), 0x4a3957c9);
	let argument = r#try! (os_string_clone_into_value (argument, immutable));
	succeed! (argument);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn process_arguments (evaluator : &mut EvaluatorContext, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let arguments = r#try! (r#try! (evaluator.parameters ()) .resolve_process_arguments ());
	let mut arguments_all = StdVec::new ();
	arguments_all.push (FALSE_VALUE);
	for argument in arguments.iter () {
		let argument = r#try! (os_string_clone_into_value (argument, immutable));
		arguments_all.push (argument);
	}
	return build_list_or_array (arguments_all, return_array, immutable);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn process_arguments_count (evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let arguments = r#try! (r#try! (evaluator.parameters ()) .resolve_process_arguments ());
	let count = arguments.len () + 1;
	let count = r#try! (NumberInteger::try_from (count));
	succeed! (count.into ());
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn process_environment_variable (variable : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	let variable = r#try! (os_str_slice_coerce (variable));
	let variable = variable.deref ();
	let variables = r#try! (r#try! (evaluator.parameters ()) .resolve_process_environment ());
	for &(ref name, ref value) in variables.iter () {
		if ffi::OsStr::eq (name, variable) {
			let value = r#try! (os_string_clone_into_value (value, immutable));
			succeed! (value);
		}
	}
	succeed! (FALSE_VALUE);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub fn process_environment_variables (evaluator : &mut EvaluatorContext, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let variables = r#try! (r#try! (evaluator.parameters ()) .resolve_process_environment ());
	let variables = try_vec_map! (variables.iter (), &(ref name, ref value), succeeded! (pair_new (r#try! (os_string_clone_into_value (name, immutable)), r#try! (os_string_clone_into_value (value, immutable)), None)));
	return build_list_or_array (variables, return_array, immutable);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "blake2-rfc" ) ]
pub fn process_environment_fingerprint (evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let fingerprint = r#try! (r#try! (evaluator.parameters ()) .resolve_process_environment_fingerprint ());
	succeed! (BytesImmutable::from_rc (fingerprint) .into ());
}




#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
pub fn process_argument (_index : &Value, _evaluator : &mut EvaluatorContext, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x281325c0, OK);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
pub fn process_arguments (_evaluator : &mut EvaluatorContext, _return_array : bool, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0xf1e93d2d, OK);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
pub fn process_arguments_count (_evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x802b2f86, OK);
}


#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
pub fn process_environment_variable (_variable : &Value, _evaluator : &mut EvaluatorContext, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x8b409a18, OK);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
pub fn process_environment_variables (_evaluator : &mut EvaluatorContext, _return_array : bool, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x0aa2b0bf, OK);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "blake2-rfc" ) ]
pub fn process_environment_fingerprint (_evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x6c766a0b, OK);
}




#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
pub fn transcript_trace_g (level : TranscriptLevel, arguments : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	if arguments.is_empty () {
		fail! (0xdd72e2ce);
	}
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let transcript = r#try! (r#try! (evaluator.parameters ()) .resolve_transcript ());
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let transcript = r#try! (transcript_for_script ());
	if ! transcript.is_active (level) {
		succeed! (());
	}
	let format = arguments[0].as_ref ();
	let format = try_as_string_ref! (format);
	let format = format.string_as_str ();
	let arguments = &arguments[1..];
	let code = transcript_code_for_message_value (format, None, None);
	r#try! (transcript.trace_values (level, code, format, arguments, None));
	succeed! (());
}




pub fn abort_g (arguments : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Error>) {
	if arguments.is_empty () {
		fail! (0x3370e774);
	}
	let code = arguments[0].as_ref ();
	let code = try_as_number_integer_ref! (code);
	let code = code.value ();
	if (code < 0) || (code > 0xffffffff) {
		fail! (0xc1bfe393);
	}
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	{
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		let transcript = r#try! (r#try! (evaluator.parameters ()) .resolve_transcript ());
		#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
		let transcript = r#try! (transcript_for_script ());
		let level = TranscriptLevel::Critical;
		if transcript.is_active (level) && arguments.len () > 1 {
			let format = arguments[1].as_ref ();
			let format = try_as_string_ref! (format);
			let format = format.string_as_str ();
			let arguments = &arguments[2..];
			let code = transcript_code_for_message_value (format, None, None);
			r#try! (transcript.trace_values (level, code, format, arguments, None));
		}
		if transcript.is_active (level) {
			transcript.trace_format (level, transcript_code_new (0x9d9bd7aa), format_args! ("aborting with code: {:08x}!", code), true, None, None);
		}
	}
	succeed! (Error::new_exit (255, false));
}




pub fn posix_timestamp () -> (NumberReal) {
	let elapsed = match time::UNIX_EPOCH.elapsed () {
		Ok (elapsed) =>
			elapsed,
		Err (_) =>
			// NOTE:  It is impossible for the clock to be before the epoch!
			panic_0! (0x09bcf425, github_issue_new),
	};
	let elapsed =
			(elapsed.as_secs () as f64)
			+ (f64::from (elapsed.subsec_nanos ()) / 1_000_000_000_f64);
	return elapsed.into ();
}




pub fn jiffies_timestamp () -> (NumberInteger) {
	unsafe {
		match JIFFIES_INSTANT {
			Some (instant) => {
				let elapsed = instant.elapsed ();
				let elapsed_seconds = elapsed.as_secs ();
				// NOTE:  If this process runs for more than 100 years we shall panic!
				if elapsed_seconds < (25 * 1461 * 24 * 3600) {
					let elapsed =
							(elapsed_seconds * 1_000_000_000)
							+ u64::from (elapsed.subsec_nanos ());
					return elapsed.expect_into_0 ();
				} else {
					panic_0! (0x70f11280, github_issue_new);
				}
			},
			None => {
				JIFFIES_INSTANT = Some (time::Instant::now ());
				return jiffies_timestamp ();
			}
		}
	}
}


pub fn jiffies_per_second () -> (NumberInteger) {
	return 1_000_000_000.into ();
}


static mut JIFFIES_INSTANT : Option<time::Instant> = None;




pub fn pause (timeout : &Value, randomize : Option<&Value>) -> (Outcome<()>) {
	let timeout = match r#try! (number_coerce_1a (timeout)) {
		NumberCoercion1::Integer (timeout) =>
			if timeout >= 0 {
				// NOTE:  If the user wants the process to sleep for more than 100 years something is wrong...
				if timeout < (25 * 1461 * 24 * 3600) {
					(timeout as u64) * 1_000_000_000
				} else {
					fail! (0x10ec1cb4);
				}
			} else {
				fail! (0xdbb3168a);
			},
		NumberCoercion1::Real (timeout) =>
			if timeout >= 0.0 {
				// NOTE:  If the user wants the process to sleep for more than 100 years something is wrong...
				if timeout < (25.0 * 1461.0 * 24.0 * 3600.0) {
					(timeout * 1_000_000_000.0) as u64
				} else {
					fail! (0xeb065a44);
				}
			} else {
				fail! (0xb5586192);
			},
	};
	if timeout == 0 {
		thread::yield_now ();
		succeed! (());
	}
	let randomize = r#try! (boolean_coerce_option (randomize)) .unwrap_or (false);
	let timeout = if randomize {
		#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
		{
			use super::externals::rand::Rng;
			random_generator () .gen_range (0 ..= timeout)
		}
		#[ cfg ( not ( feature = "vonuvoli_builtins_random" ) ) ]
		{
			fail! (0x98dc2d9e);
		}
	} else {
		timeout
	};
	let timeout = time::Duration::from_nanos (timeout);
	thread::sleep (timeout);
	succeed! (());
}




#[ cfg ( feature = "tempfile" ) ]
pub fn temporary_build <Thunk, ThunkOutput> (parent : Option<&Value>, prefix : Option<&Value>, suffix : Option<&Value>, thunk : Thunk) -> (Outcome<ThunkOutput>)
		where Thunk : Fn (Option<&fs_path::Path>, &ext::tempfile::Builder, bool) -> (Outcome<ThunkOutput>)
{
	let parent = r#try! (value_coerce_option_or_boolean (parent, None, Some (None)));
	let parent = try_option_map! (parent, path_slice_coerce (parent));
	let parent = option_ref_map! (parent, parent.deref ());
	let prefix = r#try! (value_coerce_option_or_boolean (prefix, None, Some (None)));
	let prefix = try_option_map! (prefix, path_name_slice_coerce (prefix));
	let prefix = option_ref_map! (prefix, prefix.deref ());
	let suffix = r#try! (value_coerce_option_or_boolean (suffix, None, Some (None)));
	let suffix = try_option_map! (suffix, path_name_slice_coerce (suffix));
	let suffix = option_ref_map! (suffix, suffix.deref ());
	let mut builder = ext::tempfile::Builder::new ();
	if let Some (prefix) = prefix {
		TODO! ("the `tempfile` crate requires for the moment an `str`");
		let prefix = try_some! (prefix.to_str (), 0x7ba86ec6);
		builder.prefix (prefix);
	}
	if let Some (suffix) = suffix {
		TODO! ("the `tempfile` crate requires for the moment an `str`");
		let suffix = try_some! (suffix.to_str (), 0x7eb9f789);
		builder.suffix (suffix);
	}
	builder.rand_bytes (8);
	let path_has_template = prefix.is_some () || suffix.is_some ();
	return thunk (parent, &builder, path_has_template);
}

