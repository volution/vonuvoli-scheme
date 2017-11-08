

extern crate rust_scheme;

use rust_scheme::values;




#[ test ]
fn test () -> () {
	
	println! ("## size-of `Boolean`: {}", std::mem::size_of::<values::Boolean> ());
	println! ("## size-of `NumberInteger`: {}", std::mem::size_of::<values::NumberInteger> ());
	println! ("## size-of `NumberReal`: {}", std::mem::size_of::<values::NumberReal> ());
	println! ("## size-of `Character`: {}", std::mem::size_of::<values::Character> ());
	println! ("## size-of `String`: {}", std::mem::size_of::<values::String> ());
	println! ("## size-of `Symbol`: {}", std::mem::size_of::<values::Symbol> ());
	println! ("## size-of `Bytes`: {}", std::mem::size_of::<values::Bytes> ());
	println! ("## size-of `Pair`: {}", std::mem::size_of::<values::Pair> ());
	println! ("## size-of `Value`: {}", std::mem::size_of::<values::Value> ());
	
	println! ("## size-of `()`: {}", std::mem::size_of::<()> ());
	println! ("## size-of `Option<()>`: {}", std::mem::size_of::<Option<()>> ());
	println! ("## size-of `Option<i8>`: {}", std::mem::size_of::<Option<i8>> ());
	println! ("## size-of `Option<i16>`: {}", std::mem::size_of::<Option<i16>> ());
	println! ("## size-of `Option<i32>`: {}", std::mem::size_of::<Option<i32>> ());
	println! ("## size-of `Option<(i32, i32)>`: {}", std::mem::size_of::<Option<(i32, i32)>> ());
	println! ("## size-of `Option<i64>`: {}", std::mem::size_of::<Option<i64>> ());
	
}

