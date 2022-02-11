

use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;
use super::builtins::exports::*;
use super::constants::exports::*;

use super::prelude::*;

use super::externals::ring::rand::SecureRandom as _;




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




pub fn crypto_generate_bytes_build (count : &Value) -> (Outcome<Value>) {
	let count = r#try! (count_coerce (count));
	let mut buffer = StdVec::new ();
	buffer.resize_with (count, Default::default);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer), 0x4089517c);
	succeed! (bytes_new (buffer, None));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_extend (bytes : &Value, count : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = r#try! (bytes.bytes_ref_mut ());
	let count = r#try! (count_coerce (count));
	let buffer_offset = buffer.len ();
	buffer.resize_with (buffer_offset + count, Default::default);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (&mut buffer [buffer_offset ..]), 0xf64cfb24);
	succeed! (VOID_VALUE);
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_fill_1 (bytes : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, None, None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_fill_2 (bytes : &Value, range_start : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), None);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_fill_3 (bytes : &Value, range_start : &Value, range_end : &Value) -> (Outcome<Value>) {
	return crypto_generate_bytes_fill_g (bytes, Some (range_start), Some (range_end));
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_fill_g (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = r#try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = r#try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xf55ce509);
	try_or_fail! (ext::ring::rand::SystemRandom::new () .fill (buffer), 0x81ac692f);
	succeed! (VOID_VALUE);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn crypto_generate_bytes_fill_v (arguments : usize) -> (Option<ProcedureNativeInternals>) {
	match arguments {
		1 =>
			Some (procedure_native_1 (crypto_generate_bytes_fill_1) .into ()),
		2 =>
			Some (procedure_native_2 (crypto_generate_bytes_fill_2) .into ()),
		3 =>
			Some (procedure_native_3 (crypto_generate_bytes_fill_3) .into ()),
		_ =>
			None,
	}
}




pub fn crypto_hash_md5 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::md5::Md5> (data);
}




pub fn crypto_hash_sha1 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha1::Sha1> (data);
}




pub fn crypto_hash_sha2_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha256> (data);
}

pub fn crypto_hash_sha2_256_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha224> (data);
}

pub fn crypto_hash_sha2_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512> (data);
}

pub fn crypto_hash_sha2_512_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512Trunc224> (data);
}

pub fn crypto_hash_sha2_512_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha512Trunc256> (data);
}

pub fn crypto_hash_sha2_512_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha2::Sha384> (data);
}




fn crypto_hash_0 <Hasher : ext::digest::Digest> (data : &Value) -> (Outcome<Value>) {
	let mut hasher = Hasher::new ();
	r#try! (bytes_consume (data, &mut |data| { hasher.update (data); succeed! (()); }));
	let hash = hasher.finalize ();
	succeed! (bytes_clone_slice (&hash, None));
}




pub fn crypto_hash_sha3_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_224> (data);
}

pub fn crypto_hash_sha3_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_256> (data);
}

pub fn crypto_hash_sha3_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_384> (data);
}

pub fn crypto_hash_sha3_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_0::<ext::sha3::Sha3_512> (data);
}




pub fn crypto_hash_blake2b_64 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (64, data);
}

pub fn crypto_hash_blake2b_128 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (128, data);
}

pub fn crypto_hash_blake2b_192 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (192, data);
}

pub fn crypto_hash_blake2b_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (224, data);
}

pub fn crypto_hash_blake2b_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (256, data);
}

pub fn crypto_hash_blake2b_320 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (320, data);
}

pub fn crypto_hash_blake2b_384 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (384, data);
}

pub fn crypto_hash_blake2b_448 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (448, data);
}

pub fn crypto_hash_blake2b_512 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2b_0 (512, data);
}

fn crypto_hash_blake2b_0 (bits : usize, data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2_0::<ext::blake2::VarBlake2b> (bits, data);
}




pub fn crypto_hash_blake2s_64 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (64, data);
}

pub fn crypto_hash_blake2s_128 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (128, data);
}

pub fn crypto_hash_blake2s_192 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (192, data);
}

pub fn crypto_hash_blake2s_224 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (224, data);
}

pub fn crypto_hash_blake2s_256 (data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2s_0 (256, data);
}

fn crypto_hash_blake2s_0 (bits : usize, data : &Value) -> (Outcome<Value>) {
	return crypto_hash_blake2_0::<ext::blake2::VarBlake2s> (bits, data);
}


fn crypto_hash_blake2_0 <Hasher : ext::digest::Update + ext::digest::VariableOutput> (bits : usize, data : &Value) -> (Outcome<Value>) {
	let size = bits / 8;
	let mut hasher = try_or_fail! (Hasher::new (size), 0xc5ffb9f6);
	r#try! (bytes_consume (data, &mut |data| { hasher.update (data); succeed! (()); }));
	let mut hash = StdVec::new ();
	hasher.finalize_variable (|hash_0| hash.extend_from_slice (hash_0));
	succeed! (bytes_new (hash, None));
}

