

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::lambdas::exports::*;
use super::values::exports::*;

use super::prelude::*;




impl hash::Hash for Value {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		match *self {
			
			Value::Singleton (_, ref value, _) => { hasher.write_u32 (0xc8aa23d5); value.hash (hasher); },
			
			Value::Boolean (_, ref value, _) => { hasher.write_u32 (0xee64c5c5); value.hash (hasher); },
			Value::NumberInteger (_, ref value, _) => { hasher.write_u32 (0xf5b45115); value.hash (hasher); },
			Value::NumberReal (_, ref value, _) => { hasher.write_u32 (0x754462d9); value.hash (hasher); },
			Value::Character (_, ref value, _) => { hasher.write_u32 (0x29e07200); value.hash (hasher); },
			
			Value::Symbol (_, ref value, _) => { hasher.write_u32 (0x1fcc2d57); value.hash (hasher); },
			Value::StringImmutable (_, ref value, _) => { hasher.write_u32 (0x85932088); value.hash (hasher); },
			Value::StringMutable (_, ref value, _) => { hasher.write_u32 (0x5dffe8a7); value.hash (hasher); },
			Value::BytesImmutable (_, ref value, _) => { hasher.write_u32 (0xd6ec09a4); value.hash (hasher); },
			Value::BytesMutable (_, ref value, _) => { hasher.write_u32 (0x15527940); value.hash (hasher); },
			
			Value::PairImmutable (_, ref value, _) => { hasher.write_u32 (0x1064fab6); value.hash (hasher); },
			Value::PairMutable (_, ref value, _) => { hasher.write_u32 (0x4bac60cf); value.hash (hasher); },
			Value::ArrayImmutable (_, ref value, _) => { hasher.write_u32 (0x0b86fd20); value.hash (hasher); },
			Value::ArrayMutable (_, ref value, _) => { hasher.write_u32 (0xb20f12de); value.hash (hasher); },
			Value::Values (_, ref value, _) => { hasher.write_u32 (0xb5f3786a); value.hash (hasher); },
			
			Value::Error (_, ref value, _) => { hasher.write_u32 (0x15f15501); value.hash (hasher); },
			
			Value::ProcedurePrimitive (_, ref value, _) => { hasher.write_u32 (0x23a51f00); value.hash (hasher); },
			Value::ProcedureExtended (_, ref value, _) => { hasher.write_u32 (0x50c5d416); value.hash (hasher); },
			Value::ProcedureNative (_, ref value, _) => { hasher.write_u32 (0xfe96b2d7); value.hash (hasher); },
			Value::ProcedureLambda (_, ref value, _) => { hasher.write_u32 (0x3f65eccb); value.hash (hasher); },
			
			Value::SyntaxPrimitive (_, ref value, _) => { hasher.write_u32 (0xda6585c6); value.hash (hasher); },
			Value::SyntaxExtended (_, ref value, _) => { hasher.write_u32 (0x3f07734c); value.hash (hasher); },
			Value::SyntaxNative (_, ref value, _) => { hasher.write_u32 (0xf018c0a5); value.hash (hasher); },
			Value::SyntaxLambda (_, ref value, _) => { hasher.write_u32 (0xd5b61513); value.hash (hasher); },
			
			Value::Port (_, ref value, _) => { hasher.write_u32 (0xd25641d0); value.hash (hasher); },
			
			Value::Context (_, ref value, _) => { hasher.write_u32 (0x04ef2744); value.hash (hasher); },
			Value::Binding (_, ref value, _) => { hasher.write_u32 (0x8dd0b6ab); value.hash (hasher); },
			
		}
	}
}




impl hash::Hash for NumberReal {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u64 (self.value () .to_bits ());
	}
}




impl hash::Hash for StringImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = self.string_ref ();
		string.string_as_string () .hash (hasher);
	}
}


impl hash::Hash for StringMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = self.string_ref ();
		string.string_as_string () .hash (hasher);
	}
}




impl hash::Hash for BytesImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let bytes = self.bytes_ref ();
		bytes.bytes_as_vec () .hash (hasher);
	}
}


impl hash::Hash for BytesMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let bytes = self.bytes_ref ();
		bytes.bytes_as_vec () .hash (hasher);
	}
}




impl hash::Hash for PairImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let pair = self.pair_ref ();
		pair.values_as_tuple () .hash (hasher);
	}
}


impl hash::Hash for PairMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let pair = self.pair_ref ();
		pair.values_as_tuple () .hash (hasher);
	}
}




impl hash::Hash for ArrayImmutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = self.array_ref ();
		values.values_as_vec () .hash (hasher);
	}
}


impl hash::Hash for ArrayMutable {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = self.array_ref ();
		values.values_as_vec () .hash (hasher);
	}
}




impl hash::Hash for LambdaInternals {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_2.hash (hasher);
	}
}




impl hash::Hash for ProcedureNative {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_value () .hash (hasher);
	}
}


impl hash::Hash for SyntaxNative {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_value () .hash (hasher);
	}
}




impl hash::Hash for Port {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




impl hash::Hash for Context {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


impl hash::Hash for Registers {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


impl hash::Hash for Binding {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




impl hash::Hash for ExpressionForProcedureNativeCall {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		// FIXME:  Implement this!
		hasher.write_u32 (0);
	}
}

