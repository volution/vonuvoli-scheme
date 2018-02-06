

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::parameters::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			error_message, error_arguments_as_list, error_arguments_as_array, error_arguments_as_values,
			error_build_0, error_build_1, error_build_2, error_build_3, error_build_4, error_build_n,
			error_exit,
			error_coerce, error_coerce_from,
		};
	
	pub use super::{
			parameter_build,
			parameter_resolve,
			parameter_configure,
		};
	
	pub use super::{posix_timestamp, jiffies_timestamp, jiffies_per_second};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_message (error : &Value) -> (Outcome<StringImmutable>) {
	let error = try_as_error_ref! (error);
	if let Some (message) = error.message_clone () {
		succeed! (message);
	} else {
		succeed! (string_immutable_new_empty ());
	}
}


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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_arguments_as_array (error : &Value) -> (Outcome<ArrayImmutable>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments_clone_array () {
		succeed! (arguments);
	} else {
		succeed! (array_immutable_new_empty ());
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_arguments_as_values (error : &Value) -> (Outcome<Values>) {
	let error = try_as_error_ref! (error);
	if let Some (arguments) = error.arguments_clone_values () {
		succeed! (arguments);
	} else {
		succeed! (values_new_empty ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_0 (code : Option<u64>, message : &Value) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
	let error = Error::new_with_message (code, message);
	succeed! (error);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_1 (code : Option<u64>, message : &Value, argument_1 : &Value) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_2 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_3 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_4 (code : Option<u64>, message : &Value, argument_1 : &Value, argument_2 : &Value, argument_3 : &Value, argument_4 : &Value) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
	let arguments : StdBox<[Value]> = StdBox::new ([argument_1.clone (), argument_2.clone (), argument_3.clone (), argument_4.clone ()]);
	let arguments = StdRc::new (arguments);
	let error = Error::new_with_message_and_arguments (code, message, arguments);
	succeed! (error);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_build_n (code : Option<u64>, message : &Value, arguments : &[&Value]) -> (Outcome<Error>) {
	let message = try_as_string_as_ref! (message);
	let message = try! (message.string_rc_clone ());
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_coerce (code : Option<u64>, value : &Value) -> (Error) {
	let value = value.clone ();
	return error_coerce_from (code, value);
}

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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_build (identifier : Option<&Value>, global : Option<&Value>, converter : Option<&Value>, immutable : Option<bool>, _evaluator : &mut EvaluatorContext) -> (Outcome<Parameter>) {
	let identifier = option_map! (identifier, try_as_symbol_ref! (identifier)) .cloned ();
	let global = global.cloned ();
	let conversion = if let Some (converter) = converter {
		ParameterConversion::OnConfigure (converter.clone ())
	} else {
		ParameterConversion::None
	};
	let immutable = immutable.unwrap_or (PARAMETER_NEW_IMMUTABLE);
	let parameter = Parameter::new (identifier, global, conversion, immutable);
	succeed! (parameter);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_resolve (parameter : &Value, default : Option<&Value>, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_resolve (parameter, default),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return evaluator.environment () .stdin_value_or (default),
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return evaluator.environment () .stdout_value_or (default),
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return evaluator.environment () .stderr_value_or (default),
				_ =>
					fail! (0x4ce4065b),
			},
		_ =>
			fail! (0xf44e6fc0),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parameter_configure (parameter : &Value, value : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<()>) {
	match parameter.kind_match_as_ref () {
		ValueKindMatchAsRef::Parameter (parameter) =>
			return evaluator.parameter_configure (parameter, value),
		ValueKindMatchAsRef::ProcedurePrimitive (primitive) =>
			match *primitive {
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentInput)) =>
					return try! (evaluator.environment_mut ()) .stdin_set (try_as_port_ref! (value)),
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentOutput)) =>
					return try! (evaluator.environment_mut ()) .stdout_set (try_as_port_ref! (value)),
				ProcedurePrimitive::Primitive0 (ProcedurePrimitive0::Port (PortPrimitive0::CurrentError)) =>
					return try! (evaluator.environment_mut ()) .stderr_set (try_as_port_ref! (value)),
				_ =>
					fail! (0x5970c2fd),
			},
		_ =>
			fail! (0xb05cfc27),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn posix_timestamp () -> (NumberReal) {
	let elapsed = match time::UNIX_EPOCH.elapsed () {
		Ok (elapsed) =>
			elapsed,
		Err (_) =>
			// NOTE:  It is impossible for the clock to be before the epoch!
			panic! ("09bcf425"),
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
					panic! ("70f11280");
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

