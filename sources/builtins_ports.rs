

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{
		
		is_port_input_open, is_port_output_open,
		
		port_close, port_input_close, port_output_close,
		
	};
	
	pub use super::{
		
		port_input_byte_peek, port_input_byte_read, port_input_byte_ready,
		port_input_character_peek, port_input_character_read, port_input_character_ready,
		
		port_input_bytes_read_collect, port_input_bytes_read_extend, port_input_bytes_read_copy_range,
		port_input_string_read_collect, port_input_string_read_extend,
		
	};
	
	pub use super::{
		
		port_output_byte_write, port_output_bytes_write,
		port_output_character_write, port_output_string_write,
		port_output_flush,
		
	};
	
	pub use super::{
		
		port_call_and_close,
		
	};
	
	pub use super::{
		
		port_bytes_reader_new, port_bytes_writer_new, port_bytes_writer_finalize,
		port_string_reader_new, port_string_writer_new, port_string_writer_finalize,
		
	};
}




pub fn is_port_input_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_input_open ());
}

pub fn is_port_output_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_output_open ());
}




pub fn port_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.close ();
}

pub fn port_input_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.input_close ();
}

pub fn port_output_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_close ();
}




pub fn port_input_byte_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_peek ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_byte_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_read ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_byte_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.byte_ready ();
}

pub fn port_input_character_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_peek ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_character_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_read ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_character_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.char_ready ();
}




pub fn port_input_bytes_read_collect (port : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let mut buffer = StdVec::with_capacity (count.unwrap_or (1024));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.byte_read_extend (&mut buffer, count, full)) {
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_bytes_read_extend (port : &Value, bytes : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let _port = try_as_port_ref! (port);
	let _bytes = try_as_bytes_ref! (bytes);
	let _count = try! (count_coerce (count));
	fail_unimplemented! (0xef49dfa9);
}

pub fn port_input_bytes_read_copy_range (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let _port = try_as_port_ref! (port);
	let bytes = try_as_bytes_ref! (bytes);
	let (_range_start, _range_end) = try! (range_coerce (range_start, range_end, bytes.values_length ()));
	fail_unimplemented! (0xb68f984b);
}


pub fn port_input_string_read_collect (port : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let mut buffer = StdString::with_capacity (count.unwrap_or (1024));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.char_read_string (&mut buffer, count, full)) {
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

pub fn port_input_string_read_extend (port : &Value, string : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let _port = try_as_port_ref! (port);
	let _string = try_as_string_ref! (string);
	let _count = try! (count_coerce (count));
	fail_unimplemented! (0x9e6b998c);
}




pub fn port_output_byte_write (port : &Value, byte : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let byte = try_as_number_integer_ref! (byte);
	let byte = try! (byte.try_to_u8 ());
	return port.byte_write (byte);
}

pub fn port_output_bytes_write (port : &Value, bytes : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.values_as_slice ();
	try! (port.byte_write_slice (bytes, true));
	succeed! (());
}

pub fn port_output_character_write (port : &Value, char : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let char = try_as_character_ref! (char);
	let char = char.value ();
	return port.char_write (char);
}

pub fn port_output_string_write (port : &Value, string : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	try! (port.char_write_string (string, true));
	succeed! (());
}

pub fn port_output_flush (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_flush ();
}




pub fn port_call_and_close (port : &Value, callable : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	try_as_port_ref! (port);
	let outcome = evaluator.evaluate_procedure_call_1 (callable, port);
	let port = try_as_port_ref! (port);
	try! (port.close ());
	return outcome;
}




pub fn port_bytes_reader_new (bytes : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.values_rc_clone ();
	let port = try! (Port::new_bytes_reader_from_bytes (bytes, 0, None));
	succeed! (port.into ());
}

pub fn port_string_reader_new (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_rc_clone ();
	let port = try! (Port::new_bytes_reader_from_string (string, 0, None));
	succeed! (port.into ());
}

pub fn port_bytes_writer_new () -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer ());
	succeed! (port.into ());
}

pub fn port_string_writer_new () -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer ());
	succeed! (port.into ());
}




pub fn port_bytes_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = port.internals_ref_mut ();
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

pub fn port_string_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = port.internals_ref_mut ();
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

