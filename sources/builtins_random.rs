

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;

use super::externals::rand::Rng;




pub mod exports {
	
	pub use super::{
			random_generate_i64_0, random_generate_f64_0,
			random_generate_i64_1, random_generate_f64_1,
			random_generate_i64_2, random_generate_f64_2,
			random_generate_i64_v, random_generate_f64_v,
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn generator () -> (ext::rand::ThreadRng) {
	return ext::rand::thread_rng ();
}

