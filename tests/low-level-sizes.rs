

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_test! (test__0, {
	
	println! ("----------");
	println! ("## size-of `Value`: {}", mem::size_of::<Value> ());
	println! ("## size-of `ValueMeta1`: {}", mem::size_of::<ValueMeta1> ());
	println! ("## size-of `ValueMeta2`: {}", mem::size_of::<ValueMeta2> ());
	println! ("## size-of `ValueClass`: {}", mem::size_of::<ValueClass> ());
	
	println! ("----------");
	println! ("## size-of `Boolean`: {}", mem::size_of::<Boolean> ());
	println! ("## size-of `NumberInteger`: {}", mem::size_of::<NumberInteger> ());
	println! ("## size-of `NumberReal`: {}", mem::size_of::<NumberReal> ());
	println! ("## size-of `Character`: {}", mem::size_of::<Character> ());
	println! ("## size-of `Symbol`: {}", mem::size_of::<Symbol> ());
	println! ("## size-of `String`: {}", mem::size_of::<String> ());
	println! ("## size-of `Bytes`: {}", mem::size_of::<Bytes> ());
	println! ("## size-of `Pair`: {}", mem::size_of::<Pair> ());
	println! ("## size-of `Array`: {}", mem::size_of::<Array> ());
	println! ("## size-of `Error`: {}", mem::size_of::<Error> ());
	println! ("## size-of `Lambda`: {}", mem::size_of::<Lambda> ());
	println! ("## size-of `ProcedurePrimitive`: {}", mem::size_of::<ProcedurePrimitive> ());
	println! ("## size-of `ProcedureExtended`: {}", mem::size_of::<ProcedureExtended> ());
	println! ("## size-of `ProcedureNative`: {}", mem::size_of::<ProcedureNative> ());
	println! ("## size-of `ArithmeticPrimitive1`: {}", mem::size_of::<ArithmeticPrimitive1> ());
	println! ("## size-of `SyntaxPrimitive`: {}", mem::size_of::<SyntaxPrimitive> ());
	println! ("## size-of `SyntaxExtended`: {}", mem::size_of::<SyntaxExtended> ());
	println! ("## size-of `SyntaxNative`: {}", mem::size_of::<SyntaxNative> ());
	println! ("## size-of `Context`: {}", mem::size_of::<Context> ());
	println! ("## size-of `Binding`: {}", mem::size_of::<Binding> ());
	
	println! ("----------");
	println! ("## size-of `(Value)`: {}", mem::size_of::<(Value)> ());
	println! ("## size-of `(Value, u8)`: {}", mem::size_of::<(Value, u8)> ());
	println! ("## size-of `(Value, u8, u8)`: {}", mem::size_of::<(Value, u8, u8)> ());
	
	println! ("----------");
	println! ("## size-of `Expression`: {}", mem::size_of::<Expression> ());
	
	println! ("----------");
	println! ("## size-of `()`: {}", mem::size_of::<()> ());
	println! ("## size-of `Option<()>`: {}", mem::size_of::<Option<()>> ());
	println! ("## size-of `Option<i8>`: {}", mem::size_of::<Option<i8>> ());
	println! ("## size-of `Option<i16>`: {}", mem::size_of::<Option<i16>> ());
	println! ("## size-of `Option<i32>`: {}", mem::size_of::<Option<i32>> ());
	println! ("## size-of `Option<(i32, i32)>`: {}", mem::size_of::<Option<(i32, i32)>> ());
	println! ("## size-of `Option<i64>`: {}", mem::size_of::<Option<i64>> ());
	println! ("----------");
	
	assert_eq! (mem::size_of::<Value> (), 16);
	
});

