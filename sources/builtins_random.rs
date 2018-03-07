

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
			random_generate_bytes_extend,
			
			random_generate_bytes_fill_1,
			random_generate_bytes_fill_2,
			random_generate_bytes_fill_3,
			random_generate_bytes_fill_g,
			random_generate_bytes_fill_v,
			
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn generator () -> (ext::rand::ThreadRng) {
	return ext::rand::thread_rng ();
}

