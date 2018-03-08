

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;

use super::externals::ring::rand::SecureRandom;




pub mod exports {
	
	pub use super::{
			
			crypto_generate_bytes_build,
			crypto_generate_bytes_extend,
			
			crypto_generate_bytes_fill_1,
			crypto_generate_bytes_fill_2,
			crypto_generate_bytes_fill_3,
			crypto_generate_bytes_fill_g,
			crypto_generate_bytes_fill_v,
			
		};
	
}




#[ inline (never) ]
pub fn crypto_generate_bytes_build (count : &Value) -> (Outcome<Value>) {
	let count = try! (count_coerce (Some (count))) .unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	let mut buffer = StdVec::new ();
	buffer.resize_default (count);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer), 0x4089517c);
	succeed! (bytes_new (buffer));
}




#[ inline (never) ]
pub fn crypto_generate_bytes_extend (bytes : &Value, count : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let count = try! (count_coerce (Some (count))) .unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	let buffer_offset = buffer.len ();
	buffer.resize_default (buffer_offset + count);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer [buffer_offset ..]), 0xf64cfb24);
	succeed! (VOID_VALUE);
}




#[ inline (never) ]
pub fn crypto_generate_bytes_fill_1 (bytes : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, None, None);
}

#[ inline (never) ]
pub fn crypto_generate_bytes_fill_2 (bytes : &Value, range_start : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), None);
}

#[ inline (never) ]
pub fn crypto_generate_bytes_fill_3 (bytes : &Value, range_start : &Value, range_end : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), Some (range_end));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn crypto_generate_bytes_fill_g (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xf55ce509);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (buffer), 0x81ac692f);
	succeed! (VOID_VALUE);
}

#[ inline (never) ]
pub fn crypto_generate_bytes_fill_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		1 =>
			succeed! (procedure_native_1 (crypto_generate_bytes_fill_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (crypto_generate_bytes_fill_2) .into ()),
		3 =>
			succeed! (procedure_native_3 (crypto_generate_bytes_fill_3) .into ()),
		_ =>
			fail! (0x36b83e70),
	}
}

