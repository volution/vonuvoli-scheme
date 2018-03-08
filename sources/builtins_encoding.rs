

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			encode_hex_build, encode_hex_extend, encode_hex_fill,
			decode_hex_build, decode_hex_extend, decode_hex_fill,
			
		};
	
}




#[ inline (never) ]
pub fn encode_hex_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::HEXLOWER, bytes);
}

#[ inline (never) ]
pub fn encode_hex_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_hex_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_hex_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string);
}

#[ inline (never) ]
pub fn decode_hex_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_hex_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string, buffer, true);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_build_0 (encoding : &ext::data_encoding::Encoding, bytes : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut buffer = StdString::new ();
	try! (encode_0 (encoding, bytes, &mut buffer, false));
	succeed! (string_new (buffer));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_extend_0 (encoding : &ext::data_encoding::Encoding, bytes : &Value, buffer : &Value, clear : bool) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let buffer = try_as_string_mutable_ref! (buffer);
	let mut buffer = try! (buffer.string_ref_mut ());
	try! (encode_0 (encoding, bytes, &mut buffer, clear));
	succeed! (VOID_VALUE);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_0 (encoding : &ext::data_encoding::Encoding, bytes : &[u8], buffer : &mut StdString, clear : bool) -> (Outcome<()>) {
	if clear {
		buffer.clear ();
	}
	let buffer_size = encoding.encode_len (bytes.len ());
	let buffer_offset = buffer.len ();
	let buffer = unsafe { buffer.as_mut_vec () };
	buffer.resize_default (buffer_offset + buffer_size);
	let buffer = buffer.as_mut_slice ();
	let buffer = &mut buffer [buffer_offset ..];
	encoding.encode_mut (bytes, buffer);
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_build_0 (encoding : &ext::data_encoding::Encoding, string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_bytes ();
	let mut buffer = StdVec::new ();
	try! (decode_0 (encoding, string, &mut buffer, false));
	succeed! (bytes_new (buffer));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_extend_0 (encoding : &ext::data_encoding::Encoding, string : &Value, buffer : &Value, clear : bool) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_bytes ();
	let buffer = try_as_bytes_mutable_ref! (buffer);
	let mut buffer = try! (buffer.bytes_ref_mut ());
	try! (decode_0 (encoding, string, &mut buffer, clear));
	succeed! (VOID_VALUE);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_0 (encoding : &ext::data_encoding::Encoding, string : &[u8], buffer : &mut StdVec<u8>, clear : bool) -> (Outcome<()>) {
	if clear {
		buffer.clear ();
	}
	let buffer_size = try_or_fail! (encoding.decode_len (string.len ()), 0x88b50880);
	let buffer_offset = buffer.len ();
	buffer.resize_default (buffer_offset + buffer_size);
	let buffer = buffer.as_mut_slice ();
	let buffer = &mut buffer [buffer_offset ..];
	try_or_fail! (encoding.decode_mut (string, buffer), 0xa2903464);
	succeed! (());
}

