

use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::builtins::exports::*;

#[ allow (unused_imports) ]
use super::constants::exports::*;

use super::prelude::*;

use super::externals::ring::rand::SecureRandom as TraitImportSecureRandom;




pub mod exports {
	
	pub use super::{
			
			crypto_generate_bytes_build,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			crypto_generate_bytes_extend,
			
			crypto_generate_bytes_fill_1,
			crypto_generate_bytes_fill_2,
			crypto_generate_bytes_fill_3,
			crypto_generate_bytes_fill_g,
			crypto_generate_bytes_fill_v,
			
		};
	
	pub use super::{
			
			crypto_hash_md5,
			
			crypto_hash_sha1,
			
			crypto_hash_sha2_256,
			crypto_hash_sha2_256_224,
			crypto_hash_sha2_512,
			crypto_hash_sha2_512_224,
			crypto_hash_sha2_512_256,
			crypto_hash_sha2_512_384,
			
			crypto_hash_sha3_224,
			crypto_hash_sha3_256,
			crypto_hash_sha3_384,
			crypto_hash_sha3_512,
			
			crypto_hash_blake2b_64,
			crypto_hash_blake2b_128,
			crypto_hash_blake2b_192,
			crypto_hash_blake2b_224,
			crypto_hash_blake2b_256,
			crypto_hash_blake2b_320,
			crypto_hash_blake2b_384,
			crypto_hash_blake2b_448,
			crypto_hash_blake2b_512,
			
			crypto_hash_blake2s_64,
			crypto_hash_blake2s_128,
			crypto_hash_blake2s_192,
			crypto_hash_blake2s_224,
			crypto_hash_blake2s_256,
			
		};
	
}




#[ inline (never) ]
pub fn crypto_generate_bytes_build (count : &Value) -> (Outcome<Value>) {
	let count = try! (count_coerce (count));
	let mut buffer = StdVec::new ();
	buffer.resize_default (count);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer), 0x4089517c);
	succeed! (bytes_new (buffer));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn crypto_generate_bytes_extend (bytes : &Value, count : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let count = try! (count_coerce (count));
	let buffer_offset = buffer.len ();
	buffer.resize_default (buffer_offset + count);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer [buffer_offset ..]), 0xf64cfb24);
	succeed! (VOID_VALUE);
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn crypto_generate_bytes_fill_1 (bytes : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, None, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn crypto_generate_bytes_fill_2 (bytes : &Value, range_start : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ inline (never) ]
pub fn crypto_generate_bytes_fill_3 (bytes : &Value, range_start : &Value, range_end : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), Some (range_end));
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn crypto_generate_bytes_fill_g (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xf55ce509);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (buffer), 0x81ac692f);
	succeed! (VOID_VALUE);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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




#[ inline (never) ]
pub fn crypto_hash_md5 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::md5::Md5> (data);
}




#[ inline (never) ]
pub fn crypto_hash_sha1 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha1::Sha1> (data);
}




#[ inline (never) ]
pub fn crypto_hash_sha2_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha256> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha2_256_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha224> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha2_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha2_512_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512Trunc224> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha2_512_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512Trunc256> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha2_512_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha384> (data);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn crypto_hash_0 <Hasher : ext::digest::Digest> (data : &Value) -> (Outcome<Value>) {
	let mut hasher = Hasher::new ();
	try! (bytes_consume (data, &mut |data| { hasher.process (data); succeed! (()); }));
	let hash = hasher.result ();
	succeed! (bytes_clone_slice (&hash));
}




#[ inline (never) ]
pub fn crypto_hash_sha3_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_224> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha3_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_256> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha3_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_384> (data);
}

#[ inline (never) ]
pub fn crypto_hash_sha3_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_512> (data);
}




#[ inline (never) ]
pub fn crypto_hash_blake2b_64 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (64, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_128 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (128, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_192 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (192, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (224, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (256, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_320 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (320, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (384, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_448 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (448, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2b_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (512, data);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn crypto_hash_blake2b_0 (bits : usize, data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2_0::<ext::blake2::Blake2b> (bits, data);
}




#[ inline (never) ]
pub fn crypto_hash_blake2s_64 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (64, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2s_128 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (128, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2s_192 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (192, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2s_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (224, data);
}

#[ inline (never) ]
pub fn crypto_hash_blake2s_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (256, data);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn crypto_hash_blake2s_0 (bits : usize, data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2_0::<ext::blake2::Blake2s> (bits, data);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn crypto_hash_blake2_0 <Hasher : ext::digest::Input + ext::digest::VariableOutput> (bits : usize, data : &Value) -> (Outcome<Value>) {
	let size = bits / 8;
	let mut hasher = try_or_fail! (Hasher::new (size), 0xc5ffb9f6);
	try! (bytes_consume (data, &mut |data| { hasher.process (data); succeed! (()); }));
	let mut hash = StdVec::new ();
	hash.resize_default (size);
	try_or_fail! (hasher.variable_result (&mut hash), 0x695c706a);
	succeed! (bytes_new (hash));
}

