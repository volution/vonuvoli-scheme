

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		is_port_input_open, is_port_output_open,
		
		port_close, port_input_close, port_output_close,
		
	};
	
	pub use super::{
		
		port_input_coerce_arguments,
		
		port_input_byte_peek, port_input_byte_read, port_input_byte_ready,
		port_input_character_peek, port_input_character_read, port_input_character_ready,
		
		port_input_bytes_read_copy_range,
		
		port_input_bytes_read_collect, port_input_bytes_read_extend,
		port_input_string_read_collect, port_input_string_read_extend,
		
		port_input_bytes_read_collect_until, port_input_bytes_read_extend_until,
		port_input_string_read_collect_until, port_input_string_read_extend_until,
		
		port_input_bytes_read_line,
		port_input_string_read_line,
		
	};
	
	pub use super::{
		
		port_output_byte_write, port_output_bytes_write,
		port_output_character_write, port_output_string_write,
		port_output_flush,
		
	};
	
	pub use super::{
		
		port_call_and_close_0,
		port_call_and_close_1,
		
	};
	
	pub use super::{
		
		port_bytes_reader_new, port_bytes_writer_new, port_bytes_writer_finalize,
		port_string_reader_new, port_string_writer_new, port_string_writer_finalize,
		
	};
	
	pub use super::{
		
		port_native_reader_new,
		port_native_writer_new,
		
		port_file_reader_open, port_file_reader_open_with_options,
		port_file_writer_open, port_file_writer_open_with_options,
		
	};
	
	pub use super::{
		
		port_output_value_display, port_output_value_display_0, port_output_value_display_0_slice, port_output_value_display_0_iterable,
		port_output_value_write, port_output_value_write_0, port_output_value_write_0_slice, port_output_value_write_0_iterable,
		port_output_newline, port_output_newline_0,
		
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_peek ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_read ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.byte_ready ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_peek ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_read ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.char_ready ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_copy_range (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  `full` defaults to `Some(true)` if `range_end` is not `None`;
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let full = full.unwrap_or (range_end.is_some ());
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
	let count = try! (count_coerce_option (count));
	
	let (count, full) = (
			count.or_else (|| if full.is_none () && ! full_default { Some (DEFAULT_PORT_BUFFER_SIZE) } else { None }),
			full.unwrap_or_else (|| if count.is_some () { true } else { full_default }),
		);
	
	let buffer = count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	
	succeed! ((port, count, full, buffer));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect (port : &Value, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut buffer = StdVec::with_capacity (buffer_size);
	if let Some (_) = try! (port.byte_read_extend (&mut buffer, count, full)) {
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect (port : &Value, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, false));
	let mut buffer = StdString::with_capacity (buffer_size);
	if let Some (_) = try! (port.char_read_string (&mut buffer, count, full)) {
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_until (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { '\n' as u8 };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdVec::with_capacity (buffer_size);
	if let Some (_) = try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0x87f51301);
			}
		}
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend_until (port : &Value, bytes : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { '\n' as u8 };
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
				fail_panic! (0x1ccb568e);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_until (port : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { '\n' };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdString::with_capacity (buffer_size);
	if let Some (_) = try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0xec6380c4);
			}
		}
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend_until (port : &Value, string : &Value, delimiter : Option<&Value>, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { '\n' };
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
				fail_panic! (0xd2798fc4);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_line (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = '\n' as u8;
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdVec::with_capacity (buffer_size);
	if let Some (_) = try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0x7e705788);
			}
		}
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_line (port : &Value, include_delimiter : Option<bool>, count : Option<&Value>, full : Option<bool>) -> (Outcome<Value>) {
	//! NOTE:  For `count` and `full` handling see the documentation for [`port_input_coerce_arguments`]!
	let (port, count, full, buffer_size) = try! (port_input_coerce_arguments (port, count, full, true));
	let delimiter = '\n';
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdString::with_capacity (buffer_size);
	if let Some (_) = try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0xca581872);
			}
		}
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_byte_write (port : &Value, byte : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let byte = try_as_number_integer_ref! (byte);
	let byte = try! (byte.try_to_u8 ());
	return port.byte_write (byte);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_bytes_write (port : &Value, bytes : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	try! (port.byte_write_slice (bytes, true));
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_character_write (port : &Value, char : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let char = try_as_character_ref! (char);
	let char = char.value ();
	return port.char_write (char);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_string_write (port : &Value, string : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	try! (port.char_write_string (string, true));
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_reader_new (bytes : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let port = match bytes {
		BytesRef::Immutable (ref bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_immutable (bytes, 0, None))
		},
		BytesRef::Mutable (ref bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_mutable (bytes, 0, None))
		},
	};
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_reader_new (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let port = match string {
		StringRef::Immutable (ref string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_immutable (string, 0, None))
		},
		StringRef::Mutable (ref string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_mutable (string, 0, None))
		},
	};
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.internals_ref_mut ());
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			succeed! (bytes_new (buffer) .into ());
		},
		_ =>
			fail! (0x2c8a3119),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.internals_ref_mut ());
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			if let Ok (string) = StdString::from_utf8 (buffer) {
				succeed! (string_new (string) .into ());
			} else {
				fail! (0xfa7d2f1a);
			}
		},
		_ =>
			fail! (0xac1839d4),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_reader_new (reader : StdBox<io::Read>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_reader_from_unbuffered (reader, buffer, descriptor));
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_writer_new (writer : StdBox<io::Write>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_writer_from_unbuffered (writer, buffer, descriptor));
	succeed! (port.into ());
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
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_reader_new (file, buffer, descriptor);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_open_with_options (path : &Value, options : &fs::OpenOptions, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = try! (port_file_open_with_options (path, options));
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_writer_new (file, buffer, descriptor);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn port_file_open_with_options (path : &Value, options : &fs::OpenOptions) -> (Outcome<fs::File>) {
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	let file = try_or_fail! (options.open (path), 0xbe1989bd);
	succeed! (file);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, newline : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	if newline.is_none () {
		return port_output_value_display_0 (port, value, flatten, separator, flush);
	} else {
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
		return port_output_newline_0 (port, newline, flush);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0 (port : &mut PortBackendWriter, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
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
		
		ValueClassMatchAsRef::Character (value) => {
			let value = value.value ();
			try! (port.char_write (value));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			let string = value.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		ValueClassMatchAsRef::Keyword (value) => {
			let string = value.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		ValueClassMatchAsRef::Unique (_value) => {
			fail_unimplemented! (0x5702df25);
		},
		
		ValueClassMatchAsRef::StringRegex (_value) => {
			fail_unimplemented! (0xd8a1cb13);
		},
		
		ValueClassMatchAsRef::String (class) => {
			let string = try! (class.string_ref ());
			let string = string.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		ValueClassMatchAsRef::BytesRegex (_value) => {
			fail_unimplemented! (0x992efa31);
		},
		
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
		
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let array = try! (class.array_ref ());
				let values = array.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Values (values) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let values = values.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Record (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let record = try! (class.record_ref ());
				let values = record.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Path (value) => {
			let path = value.path_ref ();
			let path = path.to_string_lossy ();
			try! (port.char_write_string (&path, true));
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::RecordKind (_) |
		ValueClassMatchAsRef::Error (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) |
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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_slice (port : &mut PortBackendWriter, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_iterable <'a, Iterator> (port : &mut PortBackendWriter, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, newline : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	if newline.is_none () {
		return port_output_value_write_0 (port, value, flatten, separator, flush);
	} else {
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
		return port_output_newline_0 (port, newline, flush);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0 (port : &mut PortBackendWriter, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
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
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				NumberMatchAsRef::Integer (value) =>
					format! ("{}", value),
				NumberMatchAsRef::Real (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Character (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Keyword (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Unique (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::StringRegex (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::String (class) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				StringMatchAsRef::Immutable (value) =>
					format! ("{}", value),
				StringMatchAsRef::Mutable (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::BytesRegex (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Bytes (class) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				BytesMatchAsRef::Immutable (value) =>
					format! ("{}", value),
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
				// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = match class {
					PairMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					PairMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let array = try! (class.array_ref ());
				let values = array.values_as_slice ();
				try! (port_output_value_write_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = match class {
					ArrayMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					ArrayMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Values (value) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let values = value.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = format! ("{}", value);
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Record (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let record = try! (class.record_ref ());
				let values = record.values_as_slice ();
				try! (port_output_value_write_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = match class {
					RecordMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					RecordMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Path (value) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::RecordKind (_) |
		ValueClassMatchAsRef::Error (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) |
		ValueClassMatchAsRef::Opaque (_) => {
			// TODO:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
	}
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_slice (port : &mut PortBackendWriter, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
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


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_iterable <'a, Iterator> (port : &mut PortBackendWriter, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>
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
	return port_output_newline_0 (port, separator, flush);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline_0 (port : &mut PortBackendWriter, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator = separator.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR);
	
	try! (port.char_write (separator));
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}

