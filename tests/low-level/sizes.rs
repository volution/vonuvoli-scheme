

extern crate rust_scheme;

use rust_scheme::exports::*;

use std::mem::size_of;




#[ test ]
fn test () -> () {
	
	eprintln! ("## size-of `Value`: {}", size_of::<Value> ());
	eprintln! ("## size-of `Expression`: {}", size_of::<Expression> ());
	
	eprintln! ("## size-of `Boolean`: {}", size_of::<Boolean> ());
	eprintln! ("## size-of `NumberInteger`: {}", size_of::<NumberInteger> ());
	eprintln! ("## size-of `NumberReal`: {}", size_of::<NumberReal> ());
	eprintln! ("## size-of `Character`: {}", size_of::<Character> ());
	eprintln! ("## size-of `Symbol`: {}", size_of::<Symbol> ());
	eprintln! ("## size-of `String`: {}", size_of::<String> ());
	eprintln! ("## size-of `Bytes`: {}", size_of::<Bytes> ());
	eprintln! ("## size-of `Pair`: {}", size_of::<Pair> ());
	eprintln! ("## size-of `Array`: {}", size_of::<Array> ());
	eprintln! ("## size-of `Error`: {}", size_of::<Error> ());
	eprintln! ("## size-of `Lambda`: {}", size_of::<Lambda> ());
	eprintln! ("## size-of `Context`: {}", size_of::<Context> ());
	eprintln! ("## size-of `Binding`: {}", size_of::<Binding> ());
	
	eprintln! ("## size-of `()`: {}", size_of::<()> ());
	eprintln! ("## size-of `Option<()>`: {}", size_of::<Option<()>> ());
	eprintln! ("## size-of `Option<i8>`: {}", size_of::<Option<i8>> ());
	eprintln! ("## size-of `Option<i16>`: {}", size_of::<Option<i16>> ());
	eprintln! ("## size-of `Option<i32>`: {}", size_of::<Option<i32>> ());
	eprintln! ("## size-of `Option<(i32, i32)>`: {}", size_of::<Option<(i32, i32)>> ());
	eprintln! ("## size-of `Option<i64>`: {}", size_of::<Option<i64>> ());
	
}

