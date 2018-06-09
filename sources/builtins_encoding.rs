

use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;
use super::constants::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			encode_hex_lower_build,
			encode_hex_upper_build,
			decode_hex_build,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			encode_hex_lower_extend, encode_hex_lower_fill,
			encode_hex_upper_extend, encode_hex_upper_fill,
			decode_hex_extend, decode_hex_fill,
			
		};
	
	pub use super::{
			
			encode_base32_build,
			decode_base32_build,
			
			encode_base32_nopad_build,
			decode_base32_nopad_build,
			
			encode_base32_hex_build,
			decode_base32_hex_build,
			
			encode_base32_hex_nopad_build,
			decode_base32_hex_nopad_build,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			encode_base32_extend, encode_base32_fill,
			decode_base32_extend, decode_base32_fill,
			
			encode_base32_nopad_extend, encode_base32_nopad_fill,
			decode_base32_nopad_extend, decode_base32_nopad_fill,
			
			encode_base32_hex_extend, encode_base32_hex_fill,
			decode_base32_hex_extend, decode_base32_hex_fill,
			
			encode_base32_hex_nopad_extend, encode_base32_hex_nopad_fill,
			decode_base32_hex_nopad_extend, decode_base32_hex_nopad_fill,
			
		};
	
	pub use super::{
			
			encode_base64_build,
			decode_base64_build,
			
			encode_base64_nopad_build,
			decode_base64_nopad_build,
			
			encode_base64_url_build,
			decode_base64_url_build,
			
			encode_base64_url_nopad_build,
			decode_base64_url_nopad_build,
			
			encode_base64_mime_build,
			decode_base64_mime_build,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			encode_base64_extend, encode_base64_fill,
			decode_base64_extend, decode_base64_fill,
			
			encode_base64_nopad_extend, encode_base64_nopad_fill,
			decode_base64_nopad_extend, decode_base64_nopad_fill,
			
			encode_base64_url_extend, encode_base64_url_fill,
			decode_base64_url_extend, decode_base64_url_fill,
			
			encode_base64_url_nopad_extend, encode_base64_url_nopad_fill,
			decode_base64_url_nopad_extend, decode_base64_url_nopad_fill,
			
			encode_base64_mime_extend, encode_base64_mime_fill,
			decode_base64_mime_extend, decode_base64_mime_fill,
			
		};
	
}




#[ inline (never) ]
pub fn encode_hex_lower_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::HEXLOWER, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_hex_lower_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_hex_lower_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXLOWER, bytes, buffer, true);
}


#[ inline (never) ]
pub fn encode_hex_upper_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::HEXUPPER, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_hex_upper_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXUPPER, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_hex_upper_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::HEXUPPER, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_hex_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_hex_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_hex_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::HEXLOWER_PERMISSIVE, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32_NOPAD, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32_NOPAD, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32_NOPAD, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32_NOPAD, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_hex_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32HEX, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_hex_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_hex_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_hex_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32HEX, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_hex_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_hex_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base32_hex_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_hex_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base32_hex_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base32_hex_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE32HEX_NOPAD, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_hex_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base32_hex_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE32HEX_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64_NOPAD, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_NOPAD, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64_NOPAD, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_NOPAD, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_url_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64URL, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_url_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_url_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_url_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64URL, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_url_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_url_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_url_nopad_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_url_nopad_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_url_nopad_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_url_nopad_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64URL_NOPAD, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_url_nopad_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_url_nopad_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64URL_NOPAD, string, buffer, true);
}




#[ inline (never) ]
pub fn encode_base64_mime_build (bytes : &Value) -> (Outcome<Value>) {
	return encode_build_0 (&ext::data_encoding::BASE64_MIME, bytes);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_mime_extend (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_MIME, bytes, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn encode_base64_mime_fill (bytes : &Value, buffer : &Value) -> (Outcome<Value>) {
	return encode_extend_0 (&ext::data_encoding::BASE64_MIME, bytes, buffer, true);
}


#[ inline (never) ]
pub fn decode_base64_mime_build (string : &Value) -> (Outcome<Value>) {
	return decode_build_0 (&ext::data_encoding::BASE64_MIME, string, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_mime_extend (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_MIME, string, buffer, false);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn decode_base64_mime_fill (string : &Value, buffer : &Value) -> (Outcome<Value>) {
	return decode_extend_0 (&ext::data_encoding::BASE64_MIME, string, buffer, true);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_build_0 (encoding : &ext::data_encoding::Encoding, data : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::new ();
	try! (encode_0 (encoding, data, &mut buffer, false));
	succeed! (string_new (buffer));
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_extend_0 (encoding : &ext::data_encoding::Encoding, data : &Value, buffer : &Value, clear : bool) -> (Outcome<Value>) {
	let buffer = try_as_string_mutable_ref! (buffer);
	let mut buffer = try! (buffer.string_ref_mut ());
	try! (encode_0 (encoding, data, &mut buffer, clear));
	succeed! (VOID_VALUE);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn encode_0 (encoding : &ext::data_encoding::Encoding, data : &Value, buffer : &mut StdString, clear : bool) -> (Outcome<()>) {
	if clear {
		buffer.clear ();
	}
	let data = try! (bytes_slice_coerce_1a (data));
	let data = &data;
	let buffer_size = encoding.encode_len (data.len ());
	let buffer_offset = buffer.len ();
	let buffer = unsafe { buffer.as_mut_vec () };
	buffer.resize_default (buffer_offset + buffer_size);
	let buffer = buffer.as_mut_slice ();
	let buffer = &mut buffer [buffer_offset ..];
	encoding.encode_mut (data, buffer);
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_build_0 (encoding : &ext::data_encoding::Encoding, data : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut buffer = StdVec::new ();
	try! (decode_0 (encoding, data, &mut buffer, false));
	succeed! (bytes_new (buffer, immutable));
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_extend_0 (encoding : &ext::data_encoding::Encoding, data : &Value, buffer : &Value, clear : bool) -> (Outcome<Value>) {
	let buffer = try_as_bytes_mutable_ref! (buffer);
	let mut buffer = try! (buffer.bytes_ref_mut ());
	try! (decode_0 (encoding, data, &mut buffer, clear));
	succeed! (VOID_VALUE);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn decode_0 (encoding : &ext::data_encoding::Encoding, data : &Value, buffer : &mut StdVec<u8>, clear : bool) -> (Outcome<()>) {
	if clear {
		buffer.clear ();
	}
	let data = try! (bytes_slice_coerce_1a (data));
	let data = &data;
	let buffer_size = try_or_fail! (encoding.decode_len (data.len ()), 0x88b50880);
	let buffer_offset = buffer.len ();
	buffer.resize_default (buffer_offset + buffer_size);
	let buffer_size = {
		let buffer = buffer.as_mut_slice ();
		let buffer = &mut buffer [buffer_offset ..];
		let buffer_size = try_or_fail! (encoding.decode_mut (data, buffer), 0xa2903464);
		buffer_size
	};
	buffer.truncate (buffer_offset + buffer_size);
	succeed! (());
}

