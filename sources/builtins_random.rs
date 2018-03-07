

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;

use super::externals::rand::Rng;




pub mod exports {
	
	pub use super::{
			
			random_generate_i64_0, random_generate_f64_0,
			random_generate_i64_1, random_generate_f64_1,
			random_generate_i64_2, random_generate_f64_2,
			random_generate_i64_v, random_generate_f64_v,
			
			random_generate_u8, random_generate_i8,
			random_generate_u16, random_generate_i16,
			random_generate_u32, random_generate_i32,
			
			random_generate_u7,
			random_generate_u15,
			random_generate_u31,
			random_generate_u63,
			
			random_generate_u1,
			random_generate_u2,
			random_generate_u3,
			random_generate_u4,
			random_generate_u5,
			random_generate_u6,
			
		};
	
	pub use super::{
			
			random_generate_bytes_build,
			random_generate_bytes_permutation,
			random_generate_bytes_extend,
			
			random_generate_bytes_fill_1,
			random_generate_bytes_fill_2,
			random_generate_bytes_fill_3,
			random_generate_bytes_fill_g,
			random_generate_bytes_fill_v,
			
			random_generate_bytes_shuffle_1,
			random_generate_bytes_shuffle_2,
			random_generate_bytes_shuffle_3,
			random_generate_bytes_shuffle_g,
			random_generate_bytes_shuffle_v,
			
		};
	
	pub use super::{
			
			random_generate_character_0,
			random_generate_character_1,
			random_generate_character_2,
			random_generate_character_v,
			
			random_generate_character_ascii,
			random_generate_character_ascii_numeric,
			random_generate_character_ascii_numeric_base_8,
			random_generate_character_ascii_numeric_base_16,
			random_generate_character_ascii_alphabetic,
			random_generate_character_ascii_alphabetic_upper_case,
			random_generate_character_ascii_alphabetic_lower_case,
			random_generate_character_ascii_alphabetic_or_numeric,
			random_generate_character_ascii_whitespace,
			random_generate_character_ascii_control,
			random_generate_character_ascii_punctuation,
			random_generate_character_ascii_graphic,
			
		};
	
}




#[ inline (never) ]
pub fn random_generate_i64_0 () -> (Outcome<Value>) {
	succeed! (number_i64 (generator () .gen ()) .into ());
}

#[ inline (never) ]
pub fn random_generate_f64_0 () -> (Outcome<Value>) {
	succeed! (number_f64 (generator () .gen ()) .into ());
}


#[ inline (never) ]
pub fn random_generate_i64_1 (max : &Value) -> (Outcome<Value>) {
	let min = 0;
	let max = try! (number_coerce_1a (max)); let max = try! (max.try_to_i64 ());
	if min >= max {
		fail! (0xbbe00f3b);
	}
	succeed! (number_i64 (generator () .gen_range (min, max)) .into ());
}

#[ inline (never) ]
pub fn random_generate_f64_1 (max : &Value) -> (Outcome<Value>) {
	let min = 0.0;
	let max = try! (number_coerce_1a (max)); let max = try! (max.try_to_f64 ());
	if min >= max {
		fail! (0x78d5a769);
	}
	succeed! (number_f64 (generator () .gen_range (min, max)) .into ());
}


#[ inline (never) ]
pub fn random_generate_i64_2 (min : &Value, max : &Value) -> (Outcome<Value>) {
	let min = try! (number_coerce_1a (min)); let min = try! (min.try_to_i64 ());
	let max = try! (number_coerce_1a (max)); let max = try! (max.try_to_i64 ());
	if min >= max {
		fail! (0xa37ceef9);
	}
	succeed! (number_i64 (generator () .gen_range (min, max)) .into ());
}

#[ inline (never) ]
pub fn random_generate_f64_2 (min : &Value, max : &Value) -> (Outcome<Value>) {
	let min = try! (number_coerce_1a (min)); let min = try! (min.try_to_f64 ());
	let max = try! (number_coerce_1a (max)); let max = try! (max.try_to_f64 ());
	if min >= max {
		fail! (0x21cbce17);
	}
	succeed! (number_f64 (generator () .gen_range (min, max)) .into ());
}




#[ inline (never) ]
pub fn random_generate_i64_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		0 =>
			succeed! (procedure_native_0 (random_generate_i64_0) .into ()),
		1 =>
			succeed! (procedure_native_1 (random_generate_i64_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (random_generate_i64_2) .into ()),
		_ =>
			fail! (0xcc8d01b1),
	}
}

#[ inline (never) ]
pub fn random_generate_f64_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		0 =>
			succeed! (procedure_native_0 (random_generate_f64_0) .into ()),
		1 =>
			succeed! (procedure_native_1 (random_generate_f64_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (random_generate_f64_2) .into ()),
		_ =>
			fail! (0x1ef8afa3),
	}
}




#[ inline (never) ]
pub fn random_generate_u8 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<u8> () .into ());
}

#[ inline (never) ]
pub fn random_generate_i8 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<i8> () .into ());
}

#[ inline (never) ]
pub fn random_generate_u16 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<u16> () .into ());
}

#[ inline (never) ]
pub fn random_generate_i16 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<i16> () .into ());
}

#[ inline (never) ]
pub fn random_generate_u32 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<u32> () .into ());
}

#[ inline (never) ]
pub fn random_generate_i32 () -> (Outcome<Value>) {
	succeed! (generator () .gen::<i32> () .into ());
}




#[ inline (never) ]
pub fn random_generate_u7 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, i8::max_value ()) .into ());
}

#[ inline (never) ]
pub fn random_generate_u15 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i16> (0, i16::max_value ()) .into ());
}

#[ inline (never) ]
pub fn random_generate_u31 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i32> (0, i32::max_value ()) .into ());
}

#[ inline (never) ]
pub fn random_generate_u63 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i64> (0, i64::max_value ()) .into ());
}




#[ inline (never) ]
pub fn random_generate_u1 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 1) .into ());
}

#[ inline (never) ]
pub fn random_generate_u2 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 2) .into ());
}

#[ inline (never) ]
pub fn random_generate_u3 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 3) .into ());
}

#[ inline (never) ]
pub fn random_generate_u4 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 4) .into ());
}

#[ inline (never) ]
pub fn random_generate_u5 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 5) .into ());
}

#[ inline (never) ]
pub fn random_generate_u6 () -> (Outcome<Value>) {
	succeed! (generator () .gen_range::<i8> (0, 1 << 6) .into ());
}




#[ inline (never) ]
pub fn random_generate_bytes_build (count : &Value) -> (Outcome<Value>) {
	let count = try! (count_coerce (Some (count))) .unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	let mut buffer = StdVec::new ();
	buffer.resize_default (count);
	generator () .fill_bytes (&mut buffer);
	succeed! (bytes_new (buffer));
}

#[ inline (never) ]
pub fn random_generate_bytes_permutation () -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (255);
	for byte in 0 .. 256 {
		buffer.push (byte as u8)
	}
	generator () .shuffle (&mut buffer);
	succeed! (bytes_new (buffer));
}

#[ inline (never) ]
pub fn random_generate_bytes_extend (bytes : &Value, count : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let count = try! (count_coerce (Some (count))) .unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
	let buffer_offset = buffer.len ();
	buffer.resize_default (buffer_offset + count);
	generator () .fill_bytes (&mut buffer [buffer_offset ..]);
	succeed! (VOID_VALUE);
}


#[ inline (never) ]
pub fn random_generate_bytes_fill_1 (bytes : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_fill_g (bytes, None, None);
}

#[ inline (never) ]
pub fn random_generate_bytes_fill_2 (bytes : &Value, range_start : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_fill_g (bytes, Some (range_start), None);
}

#[ inline (never) ]
pub fn random_generate_bytes_fill_3 (bytes : &Value, range_start : &Value, range_end : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_fill_g (bytes, Some (range_start), Some (range_end));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn random_generate_bytes_fill_g (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xfc93cb6d);
	generator () .fill_bytes (buffer);
	succeed! (VOID_VALUE);
}

#[ inline (never) ]
pub fn random_generate_bytes_fill_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		1 =>
			succeed! (procedure_native_1 (random_generate_bytes_fill_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (random_generate_bytes_fill_2) .into ()),
		3 =>
			succeed! (procedure_native_3 (random_generate_bytes_fill_3) .into ()),
		_ =>
			fail! (0xd4f36aab),
	}
}


#[ inline (never) ]
pub fn random_generate_bytes_shuffle_1 (bytes : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_shuffle_g (bytes, None, None);
}

#[ inline (never) ]
pub fn random_generate_bytes_shuffle_2 (bytes : &Value, range_start : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_shuffle_g (bytes, Some (range_start), None);
}

#[ inline (never) ]
pub fn random_generate_bytes_shuffle_3 (bytes : &Value, range_start : &Value, range_end : &Value) -> (Outcome<Value>) {
	return random_generate_bytes_shuffle_g (bytes, Some (range_start), Some (range_end));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn random_generate_bytes_shuffle_g (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xfc93cb6d);
	generator () .shuffle (buffer);
	succeed! (VOID_VALUE);
}

#[ inline (never) ]
pub fn random_generate_bytes_shuffle_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		1 =>
			succeed! (procedure_native_1 (random_generate_bytes_shuffle_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (random_generate_bytes_shuffle_2) .into ()),
		3 =>
			succeed! (procedure_native_3 (random_generate_bytes_shuffle_3) .into ()),
		_ =>
			fail! (0xd4f36aab),
	}
}




#[ inline (never) ]
pub fn random_generate_character_0 () -> (Outcome<Value>) {
	succeed! (character (generator () .gen ()) .into ());
}

#[ inline (never) ]
pub fn random_generate_character_1 (max : &Value) -> (Outcome<Value>) {
	let min = 0;
	let max = try_as_character_ref! (max) .value () as u32;
	if min >= max {
		fail! (0x9dcab850);
	}
	loop {
		let character = generator () .gen_range (min, max);
		if let Some (character) = char::from_u32 (character) {
			succeed! (character.into ());
		}
	}
}

#[ inline (never) ]
pub fn random_generate_character_2 (min : &Value, max : &Value) -> (Outcome<Value>) {
	let min = try_as_character_ref! (min) .value () as u32;
	let max = try_as_character_ref! (max) .value () as u32;
	if min >= max {
		fail! (0x76b6213d);
	}
	loop {
		let character = generator () .gen_range (min, max);
		if let Some (character) = char::from_u32 (character) {
			succeed! (character.into ());
		}
	}
}

#[ inline (never) ]
pub fn random_generate_character_v (arguments : usize) -> (Outcome<ProcedureNativeInternals>) {
	match arguments {
		0 =>
			succeed! (procedure_native_0 (random_generate_character_0) .into ()),
		1 =>
			succeed! (procedure_native_1 (random_generate_character_1) .into ()),
		2 =>
			succeed! (procedure_native_2 (random_generate_character_2) .into ()),
		_ =>
			fail! (0x95fa9cdc),
	}
}




#[ inline (never) ]
pub fn random_generate_character_ascii () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_numeric () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_digit () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_numeric_base_8 () -> (Outcome<Value>) {
	// NOTE:  for c in '0' as u8 ..'7' as u8 { print! ("{}, ", c); }
	const CHARACTERS : &'static [u8] = &[ 48, 49, 50, 51, 52, 53, 54, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_numeric_base_16 () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_hexdigit () && (c.is_ascii_digit () || c.is_ascii_lowercase ()) { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_alphabetic () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_alphabetic () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_alphabetic_upper_case () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_uppercase () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_alphabetic_lower_case () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_lowercase () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_alphabetic_or_numeric () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_alphanumeric () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_whitespace () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_whitespace () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 9, 10, 12, 13, 32, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_control () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_control () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_punctuation () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_punctuation () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ inline (never) ]
pub fn random_generate_character_ascii_graphic () -> (Outcome<Value>) {
	// NOTE:  for c in 0u8..128u8 { if c.is_ascii_graphic () { print! ("{}, ", c); } }
	const CHARACTERS : &'static [u8] = &[ 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, ];
	return random_generate_character_ascii_from (CHARACTERS);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn random_generate_character_ascii_from (characters : &[u8]) -> (Outcome<Value>) {
	let index = generator () .gen_range (0, characters.len ());
	let character = characters[index] as char;
	succeed! (character.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn generator () -> (ext::rand::ThreadRng) {
	return ext::rand::thread_rng ();
}

