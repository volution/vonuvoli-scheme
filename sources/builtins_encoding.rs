

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			encode_hex_lower_build, encode_hex_lower_extend, encode_hex_lower_fill,
			encode_hex_upper_build, encode_hex_upper_extend, encode_hex_upper_fill,
			decode_hex_build, decode_hex_extend, decode_hex_fill,
			
		};
	
	pub use super::{
			
			encode_base32_build, encode_base32_extend, encode_base32_fill,
			decode_base32_build, decode_base32_extend, decode_base32_fill,
			
			encode_base32_nopad_build, encode_base32_nopad_extend, encode_base32_nopad_fill,
			decode_base32_nopad_build, decode_base32_nopad_extend, decode_base32_nopad_fill,
			
			encode_base32_hex_build, encode_base32_hex_extend, encode_base32_hex_fill,
			decode_base32_hex_build, decode_base32_hex_extend, decode_base32_hex_fill,
			
			encode_base32_hex_nopad_build, encode_base32_hex_nopad_extend, encode_base32_hex_nopad_fill,
			decode_base32_hex_nopad_build, decode_base32_hex_nopad_extend, decode_base32_hex_nopad_fill,
			
		};
	
	pub use super::{
			
			encode_base64_build, encode_base64_extend, encode_base64_fill,
			decode_base64_build, decode_base64_extend, decode_base64_fill,
			
			encode_base64_nopad_build, encode_base64_nopad_extend, encode_base64_nopad_fill,
			decode_base64_nopad_build, decode_base64_nopad_extend, decode_base64_nopad_fill,
			
			encode_base64_url_build, encode_base64_url_extend, encode_base64_url_fill,
			decode_base64_url_build, decode_base64_url_extend, decode_base64_url_fill,
			
			encode_base64_url_nopad_build, encode_base64_url_nopad_extend, encode_base64_url_nopad_fill,
			decode_base64_url_nopad_build, decode_base64_url_nopad_extend, decode_base64_url_nopad_fill,
			
			encode_base64_mime_build, encode_base64_mime_extend, encode_base64_mime_fill,
			decode_base64_mime_build, decode_base64_mime_extend, decode_base64_mime_fill,
			
		};
	
}




#[ inline (never) ]
pub fn encode_hex_lower_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::HEXLOWER, bytes);
}

#[ inline (never) ]
pub fn encode_hex_lower_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_hex_lower_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, true);
}


#[ inline (never) ]
pub fn encode_hex_upper_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::HEXUPPER, bytes);
}

#[ inline (never) ]
pub fn encode_hex_upper_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXUPPER, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_hex_upper_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXUPPER, bytes, buffer, true);
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




#[ inline (never) ]
pub fn encode_base32_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32, bytes);
}

#[ inline (never) ]
pub fn encode_base32_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base32_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32, string);
}

#[ inline (never) ]
pub fn decode_base32_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base32_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32_NOPAD, bytes);
}

#[ inline (never) ]
pub fn encode_base32_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32_NOPAD, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base32_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32_NOPAD, string);
}

#[ inline (never) ]
pub fn decode_base32_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32_NOPAD, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base32_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_hex_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32HEX, bytes);
}

#[ inline (never) ]
pub fn encode_base32_hex_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base32_hex_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_hex_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32HEX, string);
}

#[ inline (never) ]
pub fn decode_base32_hex_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base32_hex_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_hex_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes);
}

#[ inline (never) ]
pub fn encode_base32_hex_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base32_hex_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_hex_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32HEX_NOPAD, string);
}

#[ inline (never) ]
pub fn decode_base32_hex_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base32_hex_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64, bytes);
}

#[ inline (never) ]
pub fn encode_base64_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base64_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64, string);
}

#[ inline (never) ]
pub fn decode_base64_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base64_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64_NOPAD, bytes);
}

#[ inline (never) ]
pub fn encode_base64_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_NOPAD, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base64_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64_NOPAD, string);
}

#[ inline (never) ]
pub fn decode_base64_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_NOPAD, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base64_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_url_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64URL, bytes);
}

#[ inline (never) ]
pub fn encode_base64_url_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base64_url_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_url_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64URL, string);
}

#[ inline (never) ]
pub fn decode_base64_url_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base64_url_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_url_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes);
}

#[ inline (never) ]
pub fn encode_base64_url_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base64_url_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_url_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64URL_NOPAD, string);
}

#[ inline (never) ]
pub fn decode_base64_url_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base64_url_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_mime_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64_MIME, bytes);
}

#[ inline (never) ]
pub fn encode_base64_mime_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_MIME, bytes, buffer, false);
}

#[ inline (never) ]
pub fn encode_base64_mime_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_MIME, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_mime_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64_MIME, string);
}

#[ inline (never) ]
pub fn decode_base64_mime_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_MIME, string, buffer, false);
}

#[ inline (never) ]
pub fn decode_base64_mime_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_MIME, string, buffer, true);
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
	let buffer_size = {
		let buffer = buffer.as_mut_slice ();
		let buffer = &mut buffer [buffer_offset ..];
		let buffer_size = try_or_fail! (encoding.decode_mut (string, buffer), 0xa2903464);
		buffer_size
	};
	buffer.truncate (buffer_offset + buffer_size);
	succeed! (());
}

