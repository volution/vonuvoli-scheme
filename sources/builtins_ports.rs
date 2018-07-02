

use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;
use super::constants::exports::*;
use super::builtins::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		is_port_input_open, is_port_output_open,
		
		port_close, port_input_close, port_output_close,
		
		port_input_coerce_arguments,
		
		port_output_flush,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
		
		port_input_byte_peek, port_input_byte_read, port_input_byte_ready,
		port_output_byte_write,
		
		port_input_bytes_read_collect,
		port_input_bytes_read_collect_until,
		port_input_bytes_read_collect_line,
		port_input_bytes_read_collect_zero,
		
		port_output_bytes_write,
		port_output_bytes_write_line,
		port_output_bytes_write_zero,
		port_output_bytes_write_0,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
		
		port_input_bytes_read_collect_fold,
		port_input_bytes_read_collect_until_fold,
		port_input_bytes_read_collect_line_fold,
		port_input_bytes_read_collect_zero_fold,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
		
		port_input_bytes_read_copy_range,
		
		port_input_bytes_read_extend,
		port_input_bytes_read_extend_until,
		port_input_bytes_read_extend_line,
		port_input_bytes_read_extend_zero,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
		
		port_input_character_peek, port_input_character_read, port_input_character_ready,
		port_output_character_write,
		
		port_input_string_read_collect,
		port_input_string_read_collect_until,
		port_input_string_read_collect_line,
		port_input_string_read_collect_zero,
		
		port_output_string_write,
		port_output_string_write_line,
		port_output_string_write_zero,
		port_output_string_write_0,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
		
		port_input_string_read_collect_fold,
		port_input_string_read_collect_until_fold,
		port_input_string_read_collect_line_fold,
		port_input_string_read_collect_zero_fold,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
		
		port_input_string_read_extend,
		port_input_string_read_extend_until,
		port_input_string_read_extend_line,
		port_input_string_read_extend_zero,
		
	};
	
	pub use super::{
		
		port_call_and_close_0,
		port_call_and_close_1,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
		
		port_bytes_reader_new, port_bytes_writer_new, port_bytes_writer_finalize,
		
	};
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
		
		port_string_reader_new, port_string_writer_new, port_string_writer_finalize,
		
	};
	
	pub use super::{
		
		port_native_reader_new,
		port_native_writer_new,
		
		port_file_reader_new, port_file_reader_open, port_file_reader_open_with_options,
		port_file_writer_new, port_file_writer_open, port_file_writer_open_with_options,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	pub use super::{
		
		port_output_value_display, port_output_value_display_0, port_output_value_display_0_slice, port_output_value_display_0_iterable,
		port_output_value_write, port_output_value_write_0, port_output_value_write_0_slice, port_output_value_write_0_iterable,
		
	};
	
	pub use super::{
		
		port_output_newline, port_output_newline_byte_0, port_output_newline_character_0,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	pub use super::{
		
		port_descriptor_for,
		port_descriptor_clone,
		port_descriptor_ref,
		port_descriptor_raw_fd,
		
		port_descriptor_flag_get,
		port_descriptor_flag_set,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::{
		
		port_descriptor_path,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	pub use super::{
		
		port_temporary_create,
		port_temporary_release,
		
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::{
		
		port_temporary_path,
		
	};
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_input_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_input_open ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_output_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_output_open ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.close ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.input_close ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_close ();
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_peek ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_read ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.byte_ready ();
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_peek ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_read ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.char_ready ();
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_copy_range (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  `full` defaults to `Some(true)` if `range_end` is not `None`;
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let full = full.unwrap_or_else (|| range_end.is_some ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xb8c1be42);
	if let Some (count) = try! (port.byte_read_slice (buffer, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_coerce_arguments <'a> (port : &'a Value, count : Option<&'a Value>, full : Option<bool>, full_default : bool) -> (Outcome<(&'a Port, Option<usize>, bool, usize)>) {
	
	//!    # Arguments
	//!
	//!    * `count` defaults to `DEFAULT_PORT_BUFFER_SIZE` if (the original) `full` is `None` and `full_default` is `false`;
	//!    * `full` defaults to `Some (true)` if (the original) `count` is `Some(_)`;  else it defaults to `full_default`;
	//!    * (`full_default` mainly is used by `*_read_*_until` family of functions;)
	
	//!    # Returns
	//!
	//!    * `&Port`
	//!    * `count : Option<usize>` -- the actual `count` to be used by the above rules;
	//!    * `full : bool` -- the actual `full` to be used by the above rules;
	//!    * `buffer : usize` -- the "guessed" buffer capacity required;
	
	//!    # Notes
	//!
	//!    Combining `count` and `full` one can obtain a wide range of behaviour:
	//!    * `count == Some (_)` and `full == Some (true) | None` -- read **exactly** `count` bytes;  (could require multiple syscalls;)
	//!    * `count == Some (_)` and `full == Some (false)` -- read **at most** `count` bytes;  (should require a single syscall;)
	//!    * `count == None` and `full == Some (true)` -- read **everything** until `EOF`;  (could require multiple syscalls;)
	//!    * `count == None` and `full == Some (false) | None` -- read **any** bytes available;  (should require a single syscall;)
	//!    * `count == Some (0)` -- makes no sense!
	
	let port = try_as_port_ref! (port);
	
	let count = if let Some (count) = count {
		match count.kind_match_as_ref () {
			ValueKindMatchAsRef::Boolean (value) =>
				if value.value () {
					fail! (0x8e7a74c2);
				} else {
					None
				},
			_ =>
				Some (try! (count_coerce (count))),
		}
	} else {
		None
	};
	
	let (count, full) = (
			count.or_else (|| if full.is_none () && ! full_default { Some (DEFAULT_PORT_BUFFER_SIZE) } else { None }),
			full.unwrap_or_else (|| if count.is_some () { true } else { full_default }),
		);
	
	let buffer = count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	
	succeed! ((port, count, full, buffer));
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect (port : &Value, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut buffer = StdVec::with_capacity (buffer_size);
	if try! (port.byte_read_extend (&mut buffer, count, full)) .is_some () {
		succeed! (bytes_new (buffer, immutable));
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_fold (port : &Value, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut accumulator = accumulator.clone ();
	loop {
		TODO! ("use `Rc` of buffer and try to re-use it if the callable doesn't uses it anymore");
		let mut buffer = StdVec::with_capacity (buffer_size);
		if try! (port.byte_read_extend (&mut buffer, count, full)) .is_some () {
			let value = bytes_new (buffer, immutable);
			accumulator = try! (evaluator.evaluate_procedure_call_2 (callable, &value, &accumulator));
		} else {
			succeed! (accumulator);
		}
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend (port : &Value, bytes : &Value, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let buffer = &mut buffer;
	buffer.reserve (buffer_size);
	if let Some (count) = try! (port.byte_read_extend (buffer, count, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect (port : &Value, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut buffer = StdString::with_capacity (buffer_size);
	if try! (port.char_read_string (&mut buffer, count, full)) .is_some () {
		succeed! (string_new (buffer, immutable));
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_fold (port : &Value, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut accumulator = accumulator.clone ();
	loop {
		TODO! ("use `Rc` of buffer and try to re-use it if the callable doesn't uses it anymore");
		let mut buffer = StdString::with_capacity (buffer_size);
		if try! (port.char_read_string (&mut buffer, count, full)) .is_some () {
			let value = string_new (buffer, immutable);
			accumulator = try! (evaluator.evaluate_procedure_call_2 (callable, &value, &accumulator));
		} else {
			succeed! (accumulator);
		}
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend (port : &Value, string : &Value, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let string = try_as_string_mutable_ref! (string);
	let mut buffer = try! (string.string_ref_mut ());
	let buffer = &mut buffer;
	buffer.reserve (buffer_size);
	if let Some (count) = try! (port.char_read_string (buffer, count, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_until (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8 };
	return port_input_bytes_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_line (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8;
	return port_input_bytes_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_zero (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR as u8;
	return port_input_bytes_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_bytes_read_collect_until_0 (port : &Value, delimiter : u8, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdVec::with_capacity (buffer_size);
	if try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) .is_some () {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0x87f51301, github_issue_new);
			}
		}
		succeed! (bytes_new (buffer, immutable));
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_until_fold (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8 };
	return port_input_bytes_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_line_fold (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8;
	return port_input_bytes_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_zero_fold (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR as u8;
	return port_input_bytes_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_bytes_read_collect_until_fold_0 (port : &Value, delimiter : u8, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut accumulator = accumulator.clone ();
	loop {
		TODO! ("use `Rc` of buffer and try to re-use it if the callable doesn't uses it anymore");
		let mut buffer = StdVec::with_capacity (buffer_size);
		if try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) .is_some () {
			if ! include_delimiter {
				if let Some (last) = buffer.pop () {
					if last != delimiter {
						buffer.push (last);
					}
				} else {
					fail_panic! (0xf1ebcba1, github_issue_new);
				}
			}
			let value = bytes_new (buffer, immutable);
			accumulator = try! (evaluator.evaluate_procedure_call_2 (callable, &value, &accumulator));
		} else {
			succeed! (accumulator);
		}
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend_until (port : &Value, bytes : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8 };
	return port_input_bytes_read_extend_until_0 (port, bytes, delimiter, include_delimiter, count, full);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend_line (port : &Value, bytes : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8;
	return port_input_bytes_read_extend_until_0 (port, bytes, delimiter, include_delimiter, count, full);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend_zero (port : &Value, bytes : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR as u8;
	return port_input_bytes_read_extend_until_0 (port, bytes, delimiter, include_delimiter, count, full);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_bytes_read_extend_until_0 (port : &Value, bytes : &Value, delimiter : u8, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let buffer = &mut buffer;
	buffer.reserve (buffer_size);
	if let Some (count) = try! (port.byte_read_extend_until (buffer, delimiter, count, full)) {
		let count = if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
					count
				} else {
					count - 1
				}
			} else {
				fail_panic! (0x1ccb568e, github_issue_new);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_until (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR };
	return port_input_string_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_line (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR;
	return port_input_string_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_zero (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR;
	return port_input_string_read_collect_until_0 (port, delimiter, include_delimiter, count, full, immutable);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_string_read_collect_until_0 (port : &Value, delimiter : char, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdString::with_capacity (buffer_size);
	if try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) .is_some () {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0xec6380c4, github_issue_new);
			}
		}
		succeed! (string_new (buffer, immutable));
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_until_fold (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR };
	return port_input_string_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_line_fold (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR;
	return port_input_string_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_zero_fold (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR;
	return port_input_string_read_collect_until_fold_0 (port, delimiter, include_delimiter, count, full, callable, accumulator, evaluator, immutable);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_string_read_collect_until_fold_0 (port : &Value, delimiter : char, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>, callable : &Value, accumulator : &Value, evaluator : &mut EvaluatorContext, immutable : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut accumulator = accumulator.clone ();
	loop {
		TODO! ("use `Rc` of buffer and try to re-use it if the callable doesn't uses it anymore");
		let mut buffer = StdString::with_capacity (buffer_size);
		if try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) .is_some () {
			if ! include_delimiter {
				if let Some (last) = buffer.pop () {
					if last != delimiter {
						buffer.push (last);
					}
				} else {
					fail_panic! (0x946e6d5d, github_issue_new);
				}
			}
			let value = string_new (buffer, immutable);
			accumulator = try! (evaluator.evaluate_procedure_call_2 (callable, &value, &accumulator));
		} else {
			succeed! (accumulator);
		}
	}
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend_until (port : &Value, string : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR };
	return port_input_string_read_extend_until_0 (port, string, delimiter, include_delimiter, count, full);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend_line (port : &Value, string : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR;
	return port_input_string_read_extend_until_0 (port, string, delimiter, include_delimiter, count, full);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend_zero (port : &Value, string : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let delimiter = DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR;
	return port_input_string_read_extend_until_0 (port, string, delimiter, include_delimiter, count, full);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_input_string_read_extend_until_0 (port : &Value, string : &Value, delimiter : char, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let include_delimiter = include_delimiter.unwrap_or (false);
	let string = try_as_string_mutable_ref! (string);
	let mut buffer = try! (string.string_ref_mut ());
	let buffer = &mut buffer;
	buffer.reserve (buffer_size);
	if let Some (count) = try! (port.char_read_string_until (buffer, delimiter, count, full)) {
		let count = if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
					count
				} else {
					count - 1
				}
			} else {
				fail_panic! (0xd2798fc4, github_issue_new);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_byte_write (port : &Value, byte : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let byte = try_as_number_integer_ref! (byte);
	let byte = try! (byte.try_to_u8 ());
	return port.byte_write (byte);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_bytes_write (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_bytes_write_0 (port, bytes, range_start, range_end, None);
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_bytes_write_line (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_bytes_write_0 (port, bytes, range_start, range_end, Some (Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8)));
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_bytes_write_zero (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_bytes_write_0 (port, bytes, range_start, range_end, Some (Some (DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR as u8)));
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (option_option) ) ]
pub fn port_output_bytes_write_0 (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, separator : Option<Option<u8>>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, bytes.len ()));
	let bytes = try_some! (bytes.get (range_start .. range_end), 0xe2b7eac8);
	try! (port.byte_write_slice (bytes, true));
	if let Some (separator) = separator {
		try! (port_output_newline_byte_0 (port, separator, None));
	}
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_character_write (port : &Value, char : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let char = try_as_character_ref! (char);
	let char = char.value ();
	return port.char_write (char);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_string_write (port : &Value, string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_string_write_0 (port, string, range_start, range_end, None);
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_string_write_line (port : &Value, string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_string_write_0 (port, string, range_start, range_end, Some (Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR)));
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_string_write_zero (port : &Value, string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	return port_output_string_write_0 (port, string, range_start, range_end, Some (Some (DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR)));
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (option_option) ) ]
pub fn port_output_string_write_0 (port : &Value, string : &Value, range_start : Option<&Value>, range_end : Option<&Value>, separator : Option<Option<char>>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let string = match try! (range_coerce_unbounded (range_start, range_end)) {
		(0, None) =>
			string,
		(range_start, None) => {
			let range_start = {
				let mut indices = string.char_indices () .enumerate ();
				let mut byte_range_start;
				let mut character_index_last = 0;
				let mut byte_index_last = 0;
				loop {
					let (character_index, byte_index, reached_end) = match indices.next () {
						Some ((character_index, (byte_index, _))) => {
							character_index_last = character_index;
							byte_index_last = byte_index;
							(character_index, byte_index, false)
						},
						None =>
							(character_index_last + 1, byte_index_last + 1, true),
					};
					if character_index == range_start {
						byte_range_start = byte_index;
						break;
					}
					if reached_end {
						fail! (0x22393af0);
					}
				}
				byte_range_start
			};
			try_some! (string.get (range_start ..), 0xbbad5d30)
		},
		(range_start, Some (range_end)) => {
			let (range_start, range_end) = {
				let mut indices = string.char_indices () .enumerate ();
				let mut byte_range_start = 0;
				let mut byte_range_end;
				let mut character_index_last = 0;
				let mut byte_index_last = 0;
				loop {
					let (character_index, byte_index, reached_end) = match indices.next () {
						Some ((character_index, (byte_index, _))) => {
							character_index_last = character_index;
							byte_index_last = byte_index;
							(character_index, byte_index, false)
						},
						None =>
							(character_index_last + 1, byte_index_last + 1, true),
					};
					if character_index == range_start {
						byte_range_start = byte_index;
					}
					if character_index == range_end {
						byte_range_end = byte_index;
						break;
					}
					if reached_end {
						fail! (0x7fa8df8c);
					}
				}
				(byte_range_start, byte_range_end)
			};
			try_some! (string.get (range_start .. range_end), 0x5c4c5d20)
		},
	};
	try! (port.char_write_string (string, true));
	if let Some (separator) = separator {
		try! (port_output_newline_character_0 (port, separator, None));
	}
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_flush (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_flush ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_call_and_close_0 (port : &Value, callable : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let port_ref = try_as_port_ref! (port);
	let outcome = evaluator.evaluate_procedure_call_0 (callable);
	try! (port_ref.close ());
	return outcome;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_call_and_close_1 (port : &Value, callable : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let port_ref = try_as_port_ref! (port);
	let outcome = evaluator.evaluate_procedure_call_1 (callable, port);
	try! (port_ref.close ());
	return outcome;
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_reader_new (bytes : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let port = match bytes {
		BytesRef::Immutable (bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_immutable (bytes, 0, None))
		},
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		BytesRef::Mutable (bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_mutable (bytes, 0, None))
		},
	};
	succeed! (port.into ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_reader_new (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let port = match string {
		StringRef::Immutable (string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_immutable (string, 0, None))
		},
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		StringRef::Mutable (string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_mutable (string, 0, None))
		},
	};
	succeed! (port.into ());
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_finalize (port : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.internals_ref_mut ());
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			succeed! (bytes_new (buffer, immutable));
		},
		_ =>
			fail! (0x2c8a3119),
	}
}

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_finalize (port : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.internals_ref_mut ());
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			if let Ok (string) = StdString::from_utf8 (buffer) {
				succeed! (string_new (string, immutable));
			} else {
				fail! (0xfa7d2f1a);
			}
		},
		_ =>
			fail! (0xac1839d4),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_reader_new (reader : StdBox<dyn io::Read>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_reader_from_unbuffered (reader, buffer, descriptor));
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_writer_new (writer : StdBox<dyn io::Write>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_writer_from_unbuffered (writer, buffer, descriptor));
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_reader_new (file : fs::File, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_reader_new (file, buffer, descriptor);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_new (file : fs::File, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_writer_new (file, buffer, descriptor);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_reader_open (path : &Value, buffer : Option<usize>) -> (Outcome<Value>) {
	let mut options = fs::OpenOptions::new ();
	options.read (true);
	return port_file_reader_open_with_options (path, &options, buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_open (path : &Value, buffer : Option<usize>) -> (Outcome<Value>) {
	let mut options = fs::OpenOptions::new ();
	options.write (true);
	options.create (true);
	options.truncate (true);
	// NOTE:  A safer default would be to make sure we are creating the file!
	// options.create_new (true);
	return port_file_writer_open_with_options (path, &options, buffer);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_reader_open_with_options (path : &Value, options : &fs::OpenOptions, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = try! (port_file_open_with_options (path, options));
	return port_file_reader_new (file, buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_open_with_options (path : &Value, options : &fs::OpenOptions, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = try! (port_file_open_with_options (path, options));
	return port_file_writer_new (file, buffer);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn port_file_open_with_options (path : &Value, options : &fs::OpenOptions) -> (Outcome<fs::File>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let file = try_or_fail! (options.open (path), 0xbe1989bd);
	succeed! (file);
}




#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, newline : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	if newline.is_none () {
		return port_output_value_display_0 (port, value, flatten, separator, flush);
	} else {
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
		return port_output_newline_character_0 (port, newline, flush);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0 <Backend : PortBackendWriter> (port : &mut Backend, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	match value.class_match_as_ref () {
		
		ValueClassMatchAsRef::Null => {
			try! (port.char_write_string ("null", true));
		},
		
		ValueClassMatchAsRef::Void => {
			try! (port.char_write_string ("void", true));
		},
		
		ValueClassMatchAsRef::Undefined => {
			try! (port.char_write_string ("undefined", true));
		},
		
		ValueClassMatchAsRef::Singleton (value) => {
			let formatted = match value {
				ValueSingleton::Null =>
					"null",
				ValueSingleton::Void =>
					"void",
				ValueSingleton::Undefined =>
					"undefined",
				ValueSingleton::PortEof =>
					"end-of-file",
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Boolean (value) => {
			let value = value.value ();
			let formatted = if value {
				"true"
			} else {
				"false"
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Number (class) => {
			let formatted = match class {
				NumberMatchAsRef::Integer (value) =>
					format! ("{}", value.value ()),
				NumberMatchAsRef::Real (value) =>
					format! ("{}", value.value ()),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::Character (value) => {
			let value = value.value ();
			try! (port.char_write (value));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			let string = value.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueClassMatchAsRef::Keyword (value) => {
			let string = value.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ValueClassMatchAsRef::Unique (_value) => {
			fail_unimplemented! (0x5702df25, (github_issue, 18));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::StringRegex (_value) => {
			fail_unimplemented! (0xd8a1cb13, (github_issue, 18));
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::String (class) => {
			let string = try! (class.string_ref ());
			let string = string.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::BytesRegex (_value) => {
			fail_unimplemented! (0x992efa31, (github_issue, 18));
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (class) => {
			let bytes = try! (class.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			try! (port.byte_write_slice (bytes, true));
		},
		
		ValueClassMatchAsRef::Pair (_) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let mut iterator = try! (ListIterator::new (value, true));
				try! (port_output_value_display_0_iterable (port, &mut iterator, Some (true), separator, Some (false)));
				if let Some (dotted) = iterator.dotted () {
					let dotted = dotted.as_ref ();
					try! (port_output_value_display_0 (port, dotted, Some (true), separator, Some (false)));
				}
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let array = try! (class.array_ref ());
				let values = array.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueClassMatchAsRef::Values (values) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let values = values.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::Record (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let record = try! (class.record_ref ());
				let values = record.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueClassMatchAsRef::Path (value) => {
			let path = value.path_ref ();
			let path = path.to_string_lossy ();
			try! (port.char_write_string (&path, true));
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) => {
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::RecordKind (_) => {
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		ValueClassMatchAsRef::Error (_) => {
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
		ValueClassMatchAsRef::Opaque (_) => {
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
	}
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_slice <Backend : PortBackendWriter> (port : &mut Backend, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR);
	let mut first = true;
	
	for value in values {
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_iterable <'a, Iterator, Backend> (port : &mut Backend, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>, Backend : PortBackendWriter
{
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR);
	let mut first = true;
	
	for value in values {
		let value = try! (value);
		let value = value.as_ref ();
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, newline : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	if newline.is_none () {
		return port_output_value_write_0 (port, value, flatten, separator, flush);
	} else {
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
		return port_output_newline_character_0 (port, newline, flush);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
pub fn port_output_value_write_0 <Backend : PortBackendWriter> (port : &mut Backend, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	match value.class_match_as_ref () {
		
		ValueClassMatchAsRef::Null => {
			try! (port.char_write_string ("#null", true));
		},
		
		ValueClassMatchAsRef::Void => {
			try! (port.char_write_string ("#void", true));
		},
		
		ValueClassMatchAsRef::Undefined => {
			try! (port.char_write_string ("#undefined", true));
		},
		
		ValueClassMatchAsRef::Singleton (value) => {
			let formatted = match value {
				ValueSingleton::Null =>
					"#null",
				ValueSingleton::Void =>
					"#void",
				ValueSingleton::Undefined =>
					"#undefined",
				ValueSingleton::PortEof =>
					"#end-of-file",
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Boolean (value) => {
			let value = value.value ();
			let formatted = if value {
				"#t"
			} else {
				"#f"
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Number (class) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = match class {
				NumberMatchAsRef::Integer (value) =>
					format! ("{}", value),
				NumberMatchAsRef::Real (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::Character (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueClassMatchAsRef::Keyword (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ValueClassMatchAsRef::Unique (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::StringRegex (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::String (class) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = match class {
				StringMatchAsRef::Immutable (value) =>
					format! ("{}", value),
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				StringMatchAsRef::Mutable (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::BytesRegex (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (class) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = match class {
				BytesMatchAsRef::Immutable (value) =>
					format! ("{}", value),
				#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
				BytesMatchAsRef::Mutable (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Pair (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let mut iterator = try! (ListIterator::new (value, true));
				try! (port_output_value_write_0_iterable (port, &mut iterator, Some (true), separator, Some (false)));
				if let Some (dotted) = iterator.dotted () {
					let dotted = dotted.as_ref ();
					try! (port_output_value_write_0 (port, dotted, Some (true), separator, Some (false)));
				}
			} else {
				TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
				let formatted = match class {
					PairMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
					PairMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let array = try! (class.array_ref ());
				let values = array.values_as_slice ();
				try! (port_output_value_write_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
				let formatted = match class {
					ArrayMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
					ArrayMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueClassMatchAsRef::Values (value) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let values = value.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
				let formatted = format! ("{}", value);
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::Record (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let record = try! (class.record_ref ());
				let values = record.values_as_slice ();
				try! (port_output_value_write_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
				let formatted = match class {
					RecordMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
					RecordMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueClassMatchAsRef::Path (value) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::RecordKind (_) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		ValueClassMatchAsRef::Error (_) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
		ValueClassMatchAsRef::Opaque (_) => {
			TODO! ("implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer");
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
	}
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_slice <Backend : PortBackendWriter> (port : &mut Backend, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR);
	let mut first = true;
	
	for value in values {
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_iterable <'a, Iterator, Backend> (port : &mut Backend, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>, Backend : PortBackendWriter
{
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR);
	let mut first = true;
	
	for value in values {
		let value = try! (value);
		let value = value.as_ref ();
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline (port : &Value, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	return port_output_newline_character_0 (port, separator, flush);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline_character_0 <Backend : PortBackendWriter> (port : &mut Backend, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator = separator.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR);
	
	try! (port.char_write (separator));
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline_byte_0 <Backend : PortBackendWriter> (port : &mut Backend, separator : Option<u8>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator = separator.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR as u8);
	
	try! (port.byte_write (separator));
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_for (port : &Value) -> (Outcome<Port>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	return Port::new_descriptor (PortDescriptor::for_raw_fd (descriptor));
}

#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_clone (port : &Value) -> (Outcome<Port>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	let descriptor = try! (libc_dup (descriptor));
	return Port::new_descriptor (PortDescriptor::for_raw_fd (descriptor));
}

#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_ref (port : &Value) -> (Outcome<NumberInteger>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	succeed! (descriptor.into ());
}

#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_raw_fd (port : &Value) -> (Outcome<unix_io::RawFd>) {
	let port = try_as_port_ref! (port);
	let descriptor = try_some_2! (port.descriptor (), 0xf8fd0875);
	return descriptor.as_raw_fd ();
}


#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_path (port : &Value, for_process : bool) -> (Outcome<Path>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	let path = if for_process {
		let process_id = try! (libc_getpid ());
		format! ("/proc/{}/fd/{}", process_id, descriptor)
	} else {
		format! ("/dev/fd/{}", descriptor)
	};
	let path = Path::new_from_raw (fs_path::PathBuf::from (path), false);
	succeed! (path);
}


#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_flag_get (port : &Value, flag : &Value) -> (Outcome<bool>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	let flag = try! (port_descriptor_flag_parse (flag));
	let value = try! (libc_fcntl_flags_get (descriptor));
	let value = (value & flag) == flag;
	succeed! (value);
}

#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_descriptor_flag_set (port : &Value, flag : &Value, value : &Value) -> (Outcome<bool>) {
	let descriptor = try! (port_descriptor_raw_fd (port));
	let flag = try! (port_descriptor_flag_parse (flag));
	let value_new = try_as_boolean_ref! (value) .value ();
	let value_old = try! (libc_fcntl_flags_get (descriptor));
	let value_new = if value_new {
		value_old | flag
	} else {
		value_old & !flag
	};
	try! (libc_fcntl_flags_set (descriptor, value_new));
	let value_old = (value_old & flag) == flag;
	succeed! (value_old);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn port_descriptor_flag_parse (flag : &Value) -> (Outcome<u16>) {
	let flag = try_as_symbol_ref! (flag);
	match flag.string_as_str () {
		"close-on-exec" =>
			succeed! (ext::libc::FD_CLOEXEC as u16),
		_ =>
			fail! (0x15270be7),
	}
}




#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
struct TemporaryLock (StdRefCell<Option<ext::tempfile::TempPath>>);


#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_temporary_create (parent : Option<&Value>, prefix : Option<&Value>, suffix : Option<&Value>, keep : Option<bool>, input : Option<bool>, buffer : Option<usize>) -> (Outcome<(Value, Value, Value)>) {
	let keep = keep.unwrap_or (false);
	let input = input.unwrap_or (true);
	let (output_file, input_file, wrapper) = try! (temporary_build (parent, prefix, suffix,
		|parent, builder, path_has_template| {
			let (output_file, input_file, wrapper) = if path_has_template || keep || input {
				let wrapper = if let Some (parent) = parent {
					try_or_fail! (builder.tempfile_in (parent), 0x28e67078)
				} else {
					try_or_fail! (builder.tempfile (), 0x73f99b91)
				};
				let input_file = if input {
					Some (try_or_fail! (wrapper.reopen (), 0x47070cce))
				} else {
					None
				};
				let output_file = try_or_fail! (wrapper.as_file () .try_clone (), 0xebe84071);
				if keep {
					let wrapper = wrapper.into_temp_path ();
					(output_file, input_file, Some (wrapper))
				} else {
					try_or_fail! (wrapper.close (), 0x7ab5b7d8);
					(output_file, input_file, None)
				}
			} else {
				let output_file = if let Some (parent) = parent {
					try_or_fail! (ext::tempfile::tempfile_in (parent), 0x0a5c0df5)
				} else {
					try_or_fail! (ext::tempfile::tempfile (), 0xf86ce00b)
				};
				(output_file, None, None)
			};
			succeed! ((output_file, input_file, wrapper));
		}));
	let output_port = try! (port_file_writer_new (output_file, buffer));
	let input_port = if let Some (input_file) = input_file {
		try! (port_file_reader_new (input_file, buffer))
	} else {
		FALSE_VALUE
	};
	let wrapper = option_map! (wrapper, TemporaryLock (StdRefCell::new (Some (wrapper))));
	let wrapper = option_map! (wrapper, opaque_new (wrapper) .into ()) .unwrap_or (FALSE_VALUE);
	succeed! ((output_port, input_port, wrapper));
}


#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_temporary_release (wrapper : &Value) -> (Outcome<()>) {
	let wrapper = try_as_opaque_ref! (wrapper);
	let wrapper : &TemporaryLock = try_some! (wrapper.downcast (), 0x8f0a1d9c);
	let mut wrapper = try_or_fail! (StdRefCell::try_borrow_mut (&wrapper.0), 0x43405b80);
	if let Some (wrapper) = wrapper.take () {
		try_or_fail! (wrapper.close (), 0x50af2dca);
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
pub fn port_temporary_path (wrapper : &Value) -> (Outcome<Value>) {
	let wrapper = try_as_opaque_ref! (wrapper);
	let wrapper : &TemporaryLock = try_some! (wrapper.downcast (), 0xa0cdd31d);
	let wrapper = try_or_fail! (StdRefCell::try_borrow (&wrapper.0), 0x01fe12cd);
	let wrapper = try_some_ref! (wrapper.deref (), 0x37f37169);
	let path = Path::new_from_ref (wrapper.deref (), true);
	succeed! (path.into ());
}

