

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::builtins::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::constants::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::conversions::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::primitives::exports::*;

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
			process_argument,
			process_arguments,
			process_environment_variable,
			process_environment_variables,
		};
	
	pub use super::{
			posix_timestamp,
			jiffies_timestamp,
			jiffies_per_second,
		};
	
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_arguments_as_list (error : &Value) -> (Outcome<Value>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments () {
		let arguments = list_build_n (arguments, Some (true));
		succeed! (arguments);
	} else {
		succeed! (list_empty ());
	}
}


#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_arguments_as_values (error : &Value) -> (Outcome<Values>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments_clone_values () {
		succeed! (arguments);
	} else {
		succeed! (values_new_empty ());
	}
}




#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_0 (code : Option<u64>, message : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let error = Error::new_with_message (code, message);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_1 (code : Option<u64>, message : &Value, argument_1 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_2 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_3 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_4 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value, argument_4 : &Value) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone (), argument_4.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_n (code : Option<u64>, message : &Value, arguments : &[&Value]) -> (Outcome<Error>) {
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	let message = try! (try_as_string_as_ref! (message) .string_rc_clone ());
	#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
	let message = try_as_symbol_ref! (message) .string_rc_clone ();
	let arguments = vec_clone_slice_ref (arguments);
	let arguments = StdRc::new (arguments.into_boxed_slice ());
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_exit (code : Option<&Value>, emergency : bool) -> (Outcome<Error>) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	fn build (code : &Value, emergency : bool) -> (Outcome<Error>) {
		match code.kind_match_as_ref () {
			ValueKindMatchAsRef::NumberInteger (value) =>
				succeed! (Error::new_exit (try! (value.try_to_u32 ()), emergency)),
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
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_coerce (code : Option<u64>, value : &Value) -> (Error) {
	let value = value.clone ();
	return error_coerce_from (code, value);
}

#[ cfg ( feature = "vonuvoli_values_error" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_resolve (parameter : &Value, default : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_resolve (parameter, default),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return try! (evaluator.parameters ()) .resolve_stdin_value_or (default),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return try! (evaluator.parameters ()) .resolve_stdout_value_or (default),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return try! (evaluator.parameters ()) .resolve_stderr_value_or (default),
				_ =>
					fail! (0x4ce4065b),
			},
		_ =>
			fail! (0xf44e6fc0),
	}
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_configure (parameter : &Value, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_configure (parameter, value),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (value)),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (value)),
				#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return try! (evaluator.parameters ()) .configure_stderr (try_as_port_ref! (value)),
				_ =>
					fail! (0x5970c2fd),
			},
		_ =>
			fail! (0xb05cfc27),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_argument (index : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let index = try! (count_coerce (index));
	let arguments = try! (try! (evaluator.parameters ()) .resolve_process_arguments ());
	let argument = try_some! (arguments.get (index), 0x4a3957c9);
	let argument = try! (os_string_clone_into_value (argument));
	succeed! (argument);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_arguments (evaluator : &mut EvaluatorContext, return_array : bool) -> (Outcome<Value>) {
	let arguments = try! (try! (evaluator.parameters ()) .resolve_process_arguments ());
	let arguments = try_vec_map! (arguments.iter (), argument, os_string_clone_into_value (argument));
	return build_list_or_array (arguments, return_array);
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_environment_variable (variable : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let variable = try! (os_str_slice_coerce (variable));
	let variable = variable.deref ();
	let variables = try! (try! (evaluator.parameters ()) .resolve_process_environment ());
	for &(ref name, ref value) in variables.iter () {
		if ffi::OsStr::eq (name, variable) {
			let value = try! (os_string_clone_into_value (value));
			succeed! (value);
		}
	}
	succeed! (FALSE_VALUE);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_environment_variables (evaluator : &mut EvaluatorContext, return_array : bool) -> (Outcome<Value>) {
	let variables = try! (try! (evaluator.parameters ()) .resolve_process_environment ());
	let variables = try_vec_map! (variables.iter (), &(ref name, ref value), succeeded! (pair_new (try! (os_string_clone_into_value (name)), try! (os_string_clone_into_value (value)), None)));
	return build_list_or_array (variables, return_array);
}




#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_argument (_index : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x281325c0);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_arguments (_evaluator : &mut EvaluatorContext, _return_array : bool) -> (Outcome<Value>) {
	fail_unimplemented! (0xf1e93d2d);
}


#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_environment_variable (_variable : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x8b409a18);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn process_environment_variables (_evaluator : &mut EvaluatorContext, _return_array : bool) -> (Outcome<Value>) {
	fail_unimplemented! (0x0aa2b0bf);
}




#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn transcript_trace_g (level : TranscriptLevel, arguments : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	if arguments.is_empty () {
		fail! (0xdd72e2ce);
	}
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	let transcript = try! (try! (evaluator.parameters ()) .resolve_transcript ());
	#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
	let transcript = transcript_for_script ();
	if ! transcript.is_active (level) {
		succeed! (());
	}
	let format = arguments[0];
	let format = try_as_string_ref! (format);
	let format = format.string_as_str ();
	let arguments = &arguments[1..];
	let code = transcript_code_for_message_value (format, None, None);
	try! (transcript.trace_values (level, code, format, arguments, None));
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn posix_timestamp () -> (NumberReal) {
	let elapsed = match time::UNIX_EPOCH.elapsed () {
		Ok (elapsed) =>
			elapsed,
		Err (_) =>
			// NOTE:  It is impossible for the clock to be before the epoch!
			panic_0! (0x09bcf425),
	};
	let elapsed =
			(elapsed.as_secs () as f64)
			+ ((elapsed.subsec_nanos () as f64) / 1_000_000_000f64);
	return elapsed.into ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
							+ (elapsed.subsec_nanos () as u64);
					return elapsed.expect_into_0 ();
				} else {
					panic_0! (0x70f11280);
				}
			},
			None => {
				JIFFIES_INSTANT = Some (time::Instant::now ());
				return jiffies_timestamp ();
			}
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn jiffies_per_second () -> (NumberInteger) {
	return 1_000_000_000.into ();
}


static mut JIFFIES_INSTANT : Option<time::Instant> = None;

