

use super::errors::exports::*;
use super::ports::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{
		
		is_port_input_open, is_port_output_open,
		
		port_close,
		port_close_input, port_close_output,
		
	};
	
	pub use super::{
		
		port_peek_byte, port_read_byte,
		port_peek_char, port_read_char,
		
	};
}




pub fn is_port_input_open (port : &Value) -> (Outcome<bool>) {
	use super::ports::PortQueries;
	succeed! (try_as_port_ref! (port) .is_read_open ());
}

pub fn is_port_output_open (port : &Value) -> (Outcome<bool>) {
	use super::ports::PortQueries;
	succeed! (try_as_port_ref! (port) .is_write_open ());
}




pub fn port_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.close ();
}

pub fn port_close_input (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.close_input ();
}

pub fn port_close_output (port : &Value) -> (Outcome<()>) {
	let _port = try_as_port_ref! (port);
	// return port.close_output ();
	fail_unimplemented! (0xf3971c3d);
}




pub fn port_peek_byte (port : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x82088b9b);
}

pub fn port_read_byte (port : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xa6eb7459);
}

pub fn port_peek_char (port : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x8fc04c70);
}

pub fn port_read_char (port : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xffc3a322);
}

