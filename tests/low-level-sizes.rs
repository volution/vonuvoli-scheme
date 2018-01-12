

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_test! (test__0, {
	
	eprintln! ("----------");
	eprintln! ("## size-of `Value`: {}", mem::size_of::<Value> ());
	eprintln! ("## size-of `ValueMeta1`: {}", mem::size_of::<ValueMeta1> ());
	eprintln! ("## size-of `ValueMeta2`: {}", mem::size_of::<ValueMeta2> ());
	eprintln! ("## size-of `ValueClass`: {}", mem::size_of::<ValueClass> ());
	
	eprintln! ("----------");
	eprintln! ("## size-of `Boolean`: {}", mem::size_of::<Boolean> ());
	eprintln! ("## size-of `NumberInteger`: {}", mem::size_of::<NumberInteger> ());
	eprintln! ("## size-of `NumberReal`: {}", mem::size_of::<NumberReal> ());
	eprintln! ("## size-of `Character`: {}", mem::size_of::<Character> ());
	eprintln! ("## size-of `Symbol`: {}", mem::size_of::<Symbol> ());
	eprintln! ("## size-of `StringImmutable`: {}", mem::size_of::<StringImmutable> ());
	eprintln! ("## size-of `StringMutable`: {}", mem::size_of::<StringMutable> ());
	eprintln! ("## size-of `BytesImmutable`: {}", mem::size_of::<BytesImmutable> ());
	eprintln! ("## size-of `BytesMutable`: {}", mem::size_of::<BytesMutable> ());
	eprintln! ("## size-of `PairImmutable`: {}", mem::size_of::<PairImmutable> ());
	eprintln! ("## size-of `PairMutable`: {}", mem::size_of::<PairMutable> ());
	eprintln! ("## size-of `ArrayImmutable`: {}", mem::size_of::<ArrayImmutable> ());
	eprintln! ("## size-of `ArrayMutable`: {}", mem::size_of::<ArrayMutable> ());
	eprintln! ("## size-of `Error`: {}", mem::size_of::<Error> ());
	eprintln! ("## size-of `Lambda`: {}", mem::size_of::<Lambda> ());
	eprintln! ("## size-of `ProcedurePrimitive`: {}", mem::size_of::<ProcedurePrimitive> ());
	eprintln! ("## size-of `ProcedureExtended`: {}", mem::size_of::<ProcedureExtended> ());
	eprintln! ("## size-of `ProcedureNative`: {}", mem::size_of::<ProcedureNative> ());
	eprintln! ("## size-of `ArithmeticPrimitive1`: {}", mem::size_of::<ArithmeticPrimitive1> ());
	eprintln! ("## size-of `SyntaxPrimitive`: {}", mem::size_of::<SyntaxPrimitive> ());
	eprintln! ("## size-of `SyntaxExtended`: {}", mem::size_of::<SyntaxExtended> ());
	eprintln! ("## size-of `SyntaxNative`: {}", mem::size_of::<SyntaxNative> ());
	eprintln! ("## size-of `Context`: {}", mem::size_of::<Context> ());
	eprintln! ("## size-of `Binding`: {}", mem::size_of::<Binding> ());
	
	eprintln! ("----------");
	eprintln! ("## size-of `(Value)`: {}", mem::size_of::<(Value)> ());
	eprintln! ("## size-of `(Value, u8)`: {}", mem::size_of::<(Value, u8)> ());
	eprintln! ("## size-of `(Value, u8, u8)`: {}", mem::size_of::<(Value, u8, u8)> ());
	
	eprintln! ("----------");
	eprintln! ("## size-of `Expression`: {}", mem::size_of::<Expression> ());
	
	eprintln! ("----------");
	eprintln! ("## size-of `()`: {}", mem::size_of::<()> ());
	eprintln! ("## size-of `Option<()>`: {}", mem::size_of::<Option<()>> ());
	eprintln! ("## size-of `Option<i8>`: {}", mem::size_of::<Option<i8>> ());
	eprintln! ("## size-of `Option<i16>`: {}", mem::size_of::<Option<i16>> ());
	eprintln! ("## size-of `Option<i32>`: {}", mem::size_of::<Option<i32>> ());
	eprintln! ("## size-of `Option<(i32, i32)>`: {}", mem::size_of::<Option<(i32, i32)>> ());
	eprintln! ("## size-of `Option<i64>`: {}", mem::size_of::<Option<i64>> ());
	eprintln! ("----------");
	
	assert_eq! (mem::size_of::<Value> (), 16);
	
});

