

use super::constants::exports::*;
use super::contexts::exports::*;
use super::lambdas::exports::*;
use super::values::exports::*;

use super::prelude::*;




impl hash::Hash for Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		match self.kind_match_as_ref () {
			
			ValueKindMatchAsRef::Null => { hasher.write_u32 (0xc8aa23d5); NULL.hash (hasher); },
			ValueKindMatchAsRef::Void => { hasher.write_u32 (0x87e0a1e2); VOID.hash (hasher); },
			ValueKindMatchAsRef::Undefined => { hasher.write_u32 (0x5b9e3330); UNDEFINED.hash (hasher); },
			ValueKindMatchAsRef::Singleton (self_0) => { hasher.write_u32 (0x2ff760b6); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::Boolean (self_0) => { hasher.write_u32 (0xee64c5c5); self_0.hash (hasher); },
			ValueKindMatchAsRef::NumberInteger (self_0) => { hasher.write_u32 (0xf5b45115); self_0.hash (hasher); },
			ValueKindMatchAsRef::NumberReal (self_0) => { hasher.write_u32 (0x754462d9); self_0.hash (hasher); },
			ValueKindMatchAsRef::Character (self_0) => { hasher.write_u32 (0x29e07200); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::Symbol (self_0) => { hasher.write_u32 (0x1fcc2d57); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef::Keyword (self_0) => { hasher.write_u32 (0xc1ebdc4e); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef::Unique (self_0) => { hasher.write_u32 (0x7e74b485); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::StringImmutable (self_0) => { hasher.write_u32 (0x85932088); self_0.hash (hasher); },
			ValueKindMatchAsRef::StringMutable (self_0) => { hasher.write_u32 (0x5dffe8a7); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesImmutable (self_0) => { hasher.write_u32 (0xd6ec09a4); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesMutable (self_0) => { hasher.write_u32 (0x15527940); self_0.hash (hasher); },
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef::StringRegex (self_0) => { hasher.write_u32 (0x3bd45821); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			ValueKindMatchAsRef::BytesRegex (self_0) => { hasher.write_u32 (0xd4a63fee); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::PairImmutable (self_0) => { hasher.write_u32 (0x1064fab6); self_0.hash (hasher); },
			ValueKindMatchAsRef::PairMutable (self_0) => { hasher.write_u32 (0x4bac60cf); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef::ArrayImmutable (self_0) => { hasher.write_u32 (0x0b86fd20); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef::ArrayMutable (self_0) => { hasher.write_u32 (0xb20f12de); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef::Values (self_0) => { hasher.write_u32 (0xb5f3786a); self_0.hash (hasher); },
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordKind (self_0) => { hasher.write_u32 (0x0a3b7d37); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordImmutable (self_0) => { hasher.write_u32 (0x296684da); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordMutable (self_0) => { hasher.write_u32 (0xea85f2fa); self_0.hash (hasher); },
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef::Error (self_0) => { hasher.write_u32 (0x15f15501); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::ProcedurePrimitive (self_0) => { hasher.write_u32 (0x23a51f00); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (self_0) => { hasher.write_u32 (0x50c5d416); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (self_0) => { hasher.write_u32 (0xfe96b2d7); self_0.hash (hasher); },
			ValueKindMatchAsRef::ProcedureLambda (self_0) => { hasher.write_u32 (0x3f65eccb); self_0.hash (hasher); },
			
			ValueKindMatchAsRef::SyntaxPrimitive (self_0) => { hasher.write_u32 (0xda6585c6); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::SyntaxExtended (self_0) => { hasher.write_u32 (0x3f07734c); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::SyntaxNative (self_0) => { hasher.write_u32 (0xf018c0a5); self_0.hash (hasher); },
			ValueKindMatchAsRef::SyntaxLambda (self_0) => { hasher.write_u32 (0xd5b61513); self_0.hash (hasher); },
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef::Path (self_0) => { hasher.write_u32 (0x8e9b2f47); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef::Port (self_0) => { hasher.write_u32 (0xd25641d0); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef::Process (self_0) => { hasher.write_u32 (0x87b9167c); self_0.hash (hasher); },
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Context (self_0) => { hasher.write_u32 (0x04ef2744); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Binding (self_0) => { hasher.write_u32 (0x8dd0b6ab); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameters (self_0) => { hasher.write_u32 (0x84c616f7); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (self_0) => { hasher.write_u32 (0x1937881d); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef::Promise (self_0) => { hasher.write_u32 (0x34b0d53d); self_0.hash (hasher); },
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef::Opaque (self_0) => { hasher.write_u32 (0xc749410b); self_0.hash (hasher); },
			
		}
	}
}




impl hash::Hash for NumberReal {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u64 (self.value () .to_bits ());
	}
}




impl hash::Hash for StringImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = self.string_ref ();
		string.string_as_str () .hash (hasher);
	}
}


impl hash::Hash for StringMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = try_or_return! (self.string_ref (), ());
		string.string_as_str () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl hash::Hash for BytesImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let bytes = self.bytes_ref ();
		bytes.bytes_as_slice () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl hash::Hash for BytesMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let bytes = try_or_return! (self.bytes_ref (), ());
		bytes.bytes_as_slice () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl hash::Hash for StringRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = self.regex_ref () .as_str ();
		string.hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
impl hash::Hash for BytesRegex {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let string = self.regex_ref () .as_str ();
		string.hash (hasher);
	}
}




impl hash::Hash for PairImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let pair = self.pair_ref ();
		let (left, right) = pair.left_and_right ();
		left.hash (hasher);
		right.hash (hasher);
	}
}


impl hash::Hash for PairMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let pair = try_or_return! (self.pair_ref (), ());
		let (left, right) = pair.left_and_right ();
		left.hash (hasher);
		right.hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl hash::Hash for ArrayImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = self.array_ref ();
		values.values_as_slice () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl hash::Hash for ArrayMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = try_or_return! (self.array_ref (), ());
		values.values_as_slice () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl hash::Hash for RecordKind {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl hash::Hash for RecordImmutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = self.record_ref ();
		values.kind () .hash (hasher);
		values.values_as_slice () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl hash::Hash for RecordMutable {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let values = try_or_return! (self.record_ref (), ());
		values.kind () .hash (hasher);
		values.values_as_slice () .hash (hasher);
	}
}




impl hash::Hash for Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.code () .hash (hasher);
	}
}




impl hash::Hash for LambdaInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_2.hash (hasher);
	}
}


impl hash::Hash for Lambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		self_0.hash (hasher);
	}
}


impl hash::Hash for ProcedureLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		self_0.hash (hasher);
	}
}


impl hash::Hash for SyntaxLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		self_0.hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl hash::Hash for ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl hash::Hash for SyntaxNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl hash::Hash for Path {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let path = self.path_ref ();
		path.hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl hash::Hash for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl hash::Hash for Process {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




impl hash::Hash for Context {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


impl hash::Hash for Registers {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


impl hash::Hash for Binding {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl hash::Hash for Parameters {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl hash::Hash for Parameter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}




#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl hash::Hash for Opaque {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle () .hash (hasher);
	}
}

