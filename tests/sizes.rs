

extern crate rust_scheme;

use rust_scheme::expressions::exports::*;
use rust_scheme::values::exports::*;
use rust_scheme::runtime::exports::*;

use std::mem::size_of;




#[ test ]
fn test () -> () {
	
	println! ("## size-of `Value`: {}", size_of::<Value> ());
	println! ("## size-of `Expression`: {}", size_of::<Expression> ());
	
	println! ("## size-of `Boolean`: {}", size_of::<Boolean> ());
	println! ("## size-of `NumberInteger`: {}", size_of::<NumberInteger> ());
	println! ("## size-of `NumberReal`: {}", size_of::<NumberReal> ());
	println! ("## size-of `Symbol`: {}", size_of::<Symbol> ());
	println! ("## size-of `Character`: {}", size_of::<Character> ());
	println! ("## size-of `String`: {}", size_of::<String> ());
	println! ("## size-of `Bytes`: {}", size_of::<Bytes> ());
	println! ("## size-of `Pair`: {}", size_of::<Pair> ());
	
	println! ("## size-of `()`: {}", size_of::<()> ());
	println! ("## size-of `Option<()>`: {}", size_of::<Option<()>> ());
	println! ("## size-of `Option<i8>`: {}", size_of::<Option<i8>> ());
	println! ("## size-of `Option<i16>`: {}", size_of::<Option<i16>> ());
	println! ("## size-of `Option<i32>`: {}", size_of::<Option<i32>> ());
	println! ("## size-of `Option<(i32, i32)>`: {}", size_of::<Option<(i32, i32)>> ());
	println! ("## size-of `Option<i64>`: {}", size_of::<Option<i64>> ());
	
}

