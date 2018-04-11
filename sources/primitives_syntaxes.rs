

pub mod exports {
	
	pub use super::SyntaxPrimitiveV;
	pub use super::SyntaxPrimitive;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive {
	
	PrimitiveV ( SyntaxPrimitiveV ),
	
	Auxiliary,
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitiveV {
	
	Quote,
	QuasiQuote,
	UnQuote,
	UnQuoteSplicing,
	
	Begin,
	And,
	Or,
	
	If,
	When,
	Unless,
	Cond,
	Case,
	
	Do,
	DoCond,
	While,
	WhileCond,
	Until,
	UntilCond,
	Loop,
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Guard,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	GuardCond,
	
	Locals,
	LetParallel,
	LetSequential,
	LetRecursiveParallel,
	LetRecursiveSequential,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	LetValuesParallel,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	LetValuesSequential,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	LetParameters,
	
	Define,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	DefineValues,
	
	Set,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	SetValues,
	
	Lambda,
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	DefineRecord,
	
}




impl SyntaxPrimitive {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &SyntaxPrimitive) -> (bool) {
		*self == *other
	}
}

