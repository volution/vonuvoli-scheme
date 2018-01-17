

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");



macro_rules! assert_size {
	( all, $assertion : tt, [ $( $type : ty ),* , ] ) => (
		eprint! ("\n----------\n");
		$( assert_size! ($type, $assertion); )*
		eprint! ("----------\n\n");
	);
	( for, [ $( $assertion : tt ),* , ] ) => (
		eprint! ("\n----------\n");
		$( assert_size! $assertion ; )*
		eprint! ("----------\n\n");
	);
	( $type : ty, ( like, $other : ty ) ) => (
		assert_size! ($type, (exact, mem::size_of::<$other> ()));
	);
	( $type : ty, ( smaller_than, $other : ty ) ) => (
		assert_size! ($type, (at_most, mem::size_of::<$other> ()));
	);
	( $type : ty, ( exact, $expected : expr ) ) => (
		assert_size! ($type, report);
		assert_eq! (mem::size_of::<$type> (), $expected);
	);
	( $type : ty, ( at_most, $expected : expr ) ) => (
		assert_size! ($type, report);
		assert! (mem::size_of::<$type> () <= $expected);
	);
	( $type : ty, report ) => (
		eprintln! ("## size-of `{}`: {}", stringify! ($type), mem::size_of::<$type> ());
	);
}



def_test! (test__0, {
	
	
	assert_size! (all, report, [
			Value,
			ValueMeta1, ValueMeta2,
			ValueClass, ValueKind,
		]);
	
	assert_size! (all, (smaller_than, *mut usize), [
			
			ValueSingleton,
			Boolean,
			NumberInteger, NumberReal,
			Character,
			
			Symbol,
			StringImmutable,
			StringMutable,
			BytesImmutable,
			BytesMutable,
			
			PairImmutable,
			PairMutable,
			ArrayImmutable,
			ArrayMutable,
			Values,
			
			Error,
			ProcedurePrimitive,
			ProcedureExtended,
			ProcedureNative,
			ProcedureLambda,
			
			SyntaxPrimitive,
			SyntaxExtended,
			SyntaxNative,
			SyntaxLambda,
			
			Port,
			Process,
			
			Context,
			Binding,
			
		]);
	
	
	assert_size! (all, report, [
			
			Expression,
			
		]);
	
	
	assert_size! (for, [
			
			(Value, (like, ([*mut usize; 2]))),
			(Expression, (smaller_than, ([*mut usize; 8]))),
			
			(StdBox<Value>, (like, [*mut usize; 1])),
			(StdBox<Expression>, (like, [*mut usize; 1])),
			(Option<StdBox<Value>>, (like, [*mut usize; 1])),
			(Option<StdBox<Expression>>, (like, [*mut usize; 1])),
			
			(StdRc<Value>, (like, [*mut usize; 1])),
			(StdRc<Expression>, (like, [*mut usize; 1])),
			(Option<StdRc<Value>>, (like, [*mut usize; 1])),
			(Option<StdRc<Expression>>, (like, [*mut usize; 1])),
			
			(StdRef<Value>, (like, [*mut usize; 2])),
			(StdRef<Expression>, (like, [*mut usize; 2])),
			(Option<StdRef<Value>>, (like, [*mut usize; 2])),
			(Option<StdRef<Expression>>, (like, [*mut usize; 2])),
			
			(StdBox<StdAny>, (like, [*mut usize; 2])),
			(Option<StdBox<StdAny>>, (like, [*mut usize; 2])),
			(StdRc<StdAny>, (like, [*mut usize; 2])),
			(Option<StdRc<StdAny>>, (like, [*mut usize; 2])),
			
		]);
	
	
	
	
	assert_size! (all, report, [
			
			StdString,
			StdVec<Value>,
			
		]);
	
	
	assert_size! (all, report, [
			
			(), u8, u16, u32, u64, usize, *mut usize,
			
			Option<()>, Option<u8>, Option<u16>, Option<u32>, Option<u64>, Option<usize>, Option<*mut usize>,
			
			Option<Option<()>>, Option<Option<u8>>, Option<Option<u16>>, Option<Option<u32>>, Option<Option<u64>>, Option<Option<usize>>, Option<Option<*mut usize>>,
			
		]);
	
	
});

