

extern crate rust_scheme;

use rust_scheme::exports::*;

use std::mem::size_of;




#[ test ]
#[ allow (non_snake_case) ]
fn test__0 () -> () {
	
	println! ("----------");
	println! ("## size-of `Value`: {}", size_of::<Value> ());
	println! ("## size-of `ValueMeta1`: {}", size_of::<ValueMeta1> ());
	println! ("## size-of `ValueMeta2`: {}", size_of::<ValueMeta2> ());
	println! ("## size-of `ValueClass`: {}", size_of::<ValueClass> ());
	
	println! ("----------");
	println! ("## size-of `Boolean`: {}", size_of::<Boolean> ());
	println! ("## size-of `NumberInteger`: {}", size_of::<NumberInteger> ());
	println! ("## size-of `NumberReal`: {}", size_of::<NumberReal> ());
	println! ("## size-of `Character`: {}", size_of::<Character> ());
	println! ("## size-of `Symbol`: {}", size_of::<Symbol> ());
	println! ("## size-of `String`: {}", size_of::<String> ());
	println! ("## size-of `Bytes`: {}", size_of::<Bytes> ());
	println! ("## size-of `Pair`: {}", size_of::<Pair> ());
	println! ("## size-of `Array`: {}", size_of::<Array> ());
	println! ("## size-of `Error`: {}", size_of::<Error> ());
	println! ("## size-of `Lambda`: {}", size_of::<Lambda> ());
	println! ("## size-of `ProcedurePrimitive`: {}", size_of::<ProcedurePrimitive> ());
	println! ("## size-of `ArithmeticPrimitive1`: {}", size_of::<ArithmeticPrimitive1> ());
	println! ("## size-of `SyntaxPrimitive`: {}", size_of::<SyntaxPrimitive> ());
	println! ("## size-of `Context`: {}", size_of::<Context> ());
	println! ("## size-of `Binding`: {}", size_of::<Binding> ());
	
	println! ("----------");
	println! ("## size-of `(Value)`: {}", size_of::<(Value)> ());
	println! ("## size-of `(Value, u8)`: {}", size_of::<(Value, u8)> ());
	println! ("## size-of `(Value, u8, u8)`: {}", size_of::<(Value, u8, u8)> ());
	
	println! ("----------");
	println! ("## size-of `Expression`: {}", size_of::<Expression> ());
	
	println! ("----------");
	println! ("## size-of `()`: {}", size_of::<()> ());
	println! ("## size-of `Option<()>`: {}", size_of::<Option<()>> ());
	println! ("## size-of `Option<i8>`: {}", size_of::<Option<i8>> ());
	println! ("## size-of `Option<i16>`: {}", size_of::<Option<i16>> ());
	println! ("## size-of `Option<i32>`: {}", size_of::<Option<i32>> ());
	println! ("## size-of `Option<(i32, i32)>`: {}", size_of::<Option<(i32, i32)>> ());
	println! ("## size-of `Option<i64>`: {}", size_of::<Option<i64>> ());
	println! ("----------");
	
	assert_eq! (size_of::<Value> (), 16);
	
}

