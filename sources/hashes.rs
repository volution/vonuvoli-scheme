

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			HashValue,
			HashMode,
		};
	
	pub use super::{
			hash_with_hasher,
			hash_value_with_hasher,
		};
	
	pub use super::{
			WriteHasher,
			hash_with_writer,
			hash_value_with_writer,
		};
	
	#[ cfg ( feature = "blake2-rfc" ) ]
	pub use super::{
			hash_value_with_blake2b,
			hash_value_with_blake2s,
		};
	
}




pub trait HashValue {
	fn hash_value <H : hash::Hasher> (&self, hasher : &mut H, mode : HashMode) -> (Outcome<()>);
}


impl <'a, T : HashValue + 'a> HashValue for (&'a T, &'a T) {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash_value <H : hash::Hasher> (&self, hasher : &mut H, mode : HashMode) -> (Outcome<()>) {
		let (left, right) = *self;
		try! (left.hash_value (hasher, mode));
		try! (right.hash_value (hasher, mode));
		succeed! (());
	}
}

impl <T : HashValue> HashValue for [T] {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash_value <H : hash::Hasher> (&self, hasher : &mut H, mode : HashMode) -> (Outcome<()>) {
		if self.is_empty () {
			hasher.write_u128 (0x8183755552f3d1bd2f0f9ae1d93f1d59);
		} else {
			hasher.write_u128 (0x4817e0635a2e46feaa6e0658aa1f71f5);
			hasher.write_u64 (self.len () as u64);
			for value in self {
				try! (value.hash_value (hasher, mode));
			}
		}
		succeed! (());
	}
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum HashMode {
	Strict,
	ValuesAcceptMutable,
	ValuesCoerceMutable,
	RelaxedAcceptMutable,
	RelaxedCoerceMutable,
}

impl HashMode {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn accept_undefined (self) -> (bool) {
		match self {
			HashMode::Strict =>
				false,
			HashMode::ValuesAcceptMutable | HashMode::ValuesCoerceMutable =>
				true,
			HashMode::RelaxedAcceptMutable | HashMode::RelaxedCoerceMutable =>
				true,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn accept_inserializable (self) -> (bool) {
		match self {
			HashMode::Strict =>
				false,
			HashMode::ValuesAcceptMutable | HashMode::ValuesCoerceMutable =>
				false,
			HashMode::RelaxedAcceptMutable | HashMode::RelaxedCoerceMutable =>
				true,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn accept_mutable (self) -> (bool) {
		match self {
			HashMode::Strict =>
				false,
			HashMode::ValuesAcceptMutable | HashMode::ValuesCoerceMutable | HashMode::RelaxedAcceptMutable | HashMode::RelaxedCoerceMutable =>
				true,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn coerce_mutable (self) -> (bool) {
		match self {
			HashMode::Strict | HashMode::ValuesAcceptMutable | HashMode::RelaxedAcceptMutable =>
				false,
			HashMode::ValuesCoerceMutable | HashMode::RelaxedCoerceMutable =>
				true,
		}
	}
}




pub struct WriteHasher <'a, W : io::Write + 'a> ( &'a mut W );


impl <'a, W : io::Write + 'a> WriteHasher<'a, W> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (writer : &mut W) -> (WriteHasher<W>) {
		WriteHasher (writer)
	}
}


impl <'a, W : io::Write + 'a> hash::Hasher for WriteHasher<'a, W> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write (&mut self, bytes : &[u8]) -> () {
		try_or_panic_0! (self.0.write_all (bytes), 0x45e8ceaa, github_issue_new);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn finish (&self) -> (u64) {
		panic_0! (0x17dfb609, github_issue_new);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_with_hasher <T : hash::Hash, R : StdAsRef<T>, H : hash::Hasher> (value : R, hasher : H) -> (u64) {
	let value = value.as_ref ();
	let mut hasher = hasher;
	value.hash (&mut hasher);
	let hash = hasher.finish ();
	return hash;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_hasher <T : HashValue, R : StdAsRef<T>, H : hash::Hasher> (value : R, hasher : H, mode : HashMode) -> (Outcome<u64>) {
	let value = value.as_ref ();
	let mut hasher = hasher;
	try! (value.hash_value (&mut hasher, mode));
	let hash = hasher.finish ();
	succeed! (hash);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_with_writer <T : hash::Hash, R : StdAsRef<T>, W : io::Write> (value : R, writer : &mut W) -> () {
	let value = value.as_ref ();
	let mut hasher = WriteHasher::new (writer);
	value.hash (&mut hasher);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_writer <T : HashValue, R : StdAsRef<T>, W : io::Write> (value : R, writer : &mut W, mode : HashMode) -> (Outcome<()>) {
	let value = value.as_ref ();
	let mut hasher = WriteHasher::new (writer);
	return value.hash_value (&mut hasher, mode);
}




#[ cfg ( feature = "blake2-rfc" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2b <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, key : Option<&[u8]>, mode : HashMode) -> (Outcome<StdBox<[u8]>>) {
	let size = bits / 8;
	let key = key.unwrap_or (&[]);
	if (size * 8) != bits {
		fail! (0x355ed665);
	}
	if size == 0 {
		fail! (0x958adc76);
	}
	if size > 64 {
		fail! (0x176342ac);
	}
	if key.len () > 64 {
		fail! (0x4be93a98);
	}
	let mut hasher = ext::blake2_rfc::blake2b::Blake2b::with_key (size, key);
	try! (hash_value_with_writer (value, &mut hasher, mode));
	let hash = hasher.finalize ();
	let hash = StdVec::from (hash.as_bytes ());
	let hash = hash.into_boxed_slice ();
	succeed! (hash);
}

#[ cfg ( feature = "blake2-rfc" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2s <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, key : Option<&[u8]>, mode : HashMode) -> (Outcome<StdBox<[u8]>>) {
	let size = bits / 8;
	let key = key.unwrap_or (&[]);
	if (size * 8) != bits {
		fail! (0x515bf4b5);
	}
	if size == 0 {
		fail! (0x6e238976);
	}
	if size > 32 {
		fail! (0x5202bc04);
	}
	if key.len () > 32 {
		fail! (0xba255824);
	}
	let mut hasher = ext::blake2_rfc::blake2s::Blake2s::with_key (size, key);
	try! (hash_value_with_writer (value, &mut hasher, mode));
	let hash = hasher.finalize ();
	let hash = StdVec::from (hash.as_bytes ());
	let hash = hash.into_boxed_slice ();
	succeed! (hash);
}




macro_rules! impl_hash {
	
	
	( restriction, none, $mode : ident ) => {
		let _ = $mode;
	};
	( restriction, inserializable, $mode : ident ) => {
		if ! $mode.accept_inserializable () {
			fail! (0x14a4668c);
		}
	};
	
	
	( $type : ty, hash ) => {
		
		#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (derive_hash_xor_eq) ) ]
		impl hash::Hash for $type {
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn hash <Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
				try_or_panic! (self.hash_value (hasher, HashMode::RelaxedAcceptMutable));
			}
		}
		
	};
	
	
	( $type : ty, unimplemented, $code : expr, $tracking : tt ) => {
		
		impl HashValue for $type {
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn hash_value <Hasher : hash::Hasher> (&self, _hasher : &mut Hasher, _mode : HashMode) -> (Outcome<()>) {
				fail_unimplemented! ($code, $tracking);
			}
		}
		
		impl_hash! ($type, hash);
		
	};
	( $type : ty, unimplemented, $code : expr ) => {
		impl_hash! ($type, unimplemented, $code, none);
	};
	
	( $type : ty, standard, $restriction : tt, $tag : expr ) => {
		
		impl HashValue for $type {
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn hash_value <Hasher : hash::Hasher> (&self, hasher : &mut Hasher, mode : HashMode) -> (Outcome<()>) {
				impl_hash! (restriction, $restriction, mode);
				hasher.write_u128 ($tag);
				hash::Hash::hash (self, hasher);
				succeed! (());
			}
		}
		
	};
	
	( $type : ty, impl, $restriction : tt, $self : ident, $hasher : ident, $mode : ident, $expression : block ) => {
		
		impl HashValue for $type {
			
			#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
			fn hash_value <Hasher : hash::Hasher> (&$self, $hasher : &mut Hasher, $mode : HashMode) -> (Outcome<()>) {
				impl_hash! (restriction, $restriction, $mode);
				$expression
				succeed! (());
			}
		}
		
		impl_hash! ($type, hash);
		
	};
	
	
	( $type : ty, self, $restriction : tt, $tag : expr, $self : ident, $value : ident, $value_expression : expr, $hasher : ident, $hasher_expression : expr, $mode : ident ) => {
		impl_hash! ($type, impl, $restriction, $self, $hasher, $mode, {
				let $value = $value_expression;
				$hasher.write_u128 ($tag);
				$hasher_expression;
			});
	};
	( $type : ty, self, $restriction : tt, $tag : expr, $self : ident, $value : ident, $value_expression : expr, $hasher : ident, $hasher_expression : expr ) => {
		impl_hash! ($type, self, $restriction, $tag, $self, $value, $value_expression, $hasher, $hasher_expression, _mode);
	};
	
	( $type : ty, accessor, $restriction : tt, $tag : expr, $value_accessor : ident, hash ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, self.$value_accessor (), hasher, hash::Hash::hash (value, hasher));
	};
	( $type : ty, accessor_2, $restriction : tt, $tag : expr, $value_accessor_1 : ident, $value_accessor_2 : ident, hash ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, self .$value_accessor_1 () .$value_accessor_2 (), hasher, hash::Hash::hash (value, hasher));
	};
	
	( $type : ty, accessor, $restriction : tt, $tag : expr, $value_accessor : ident, hash_value ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, self.$value_accessor (), hasher, try! (HashValue::hash_value (value, hasher, mode)), mode);
	};
	( $type : ty, accessor_2, $restriction : tt, $tag : expr, $value_accessor_1 : ident, $value_accessor_2 : ident, hash_value ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, self .$value_accessor_1 () .$value_accessor_2 (), hasher, try! (HashValue::hash_value (value, hasher, mode)), mode);
	};
	
	( $type : ty, accessor_pointer, $restriction : tt, $tag : expr, $value_accessor : ident ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, ptr::NonNull::from (self.$value_accessor ()), hasher, hash::Hash::hash (&value, hasher));
	};
	
	( $type : ty, handle, $restriction : tt, $tag : expr ) => {
		impl_hash! ($type, self, $restriction, $tag, self, value, self.handle (), hasher, hash::Hash::hash (&value, hasher));
	};
	
	( $type : ty, wrapper, $restriction : tt, $tag : expr, $value : ident, $hasher : ident, $hasher_expression : expr ) => {
		impl_hash! ($type, self, $restriction, $tag, self, $value, &self.0, $hasher, $hasher_expression);
	};
	( $type : ty, wrapper, $restriction : tt, $tag : expr ) => {
		impl_hash! ($type, wrapper, $restriction, $tag, value, hasher, hash::Hash::hash (value, hasher));
	};
	
	( $type : ty, wrapper_rc, $restriction : tt, $tag : expr, $value : ident, $hasher : ident, $hasher_expression : expr ) => {
		impl_hash! ($type, self, $restriction, $tag, self, $value, StdRc::deref (&self.0), $hasher, $hasher_expression);
	};
	( $type : ty, wrapper_rc, $restriction : tt, $tag : expr ) => {
		impl_hash! ($type, wrapper_rc, $restriction, $tag, value, hasher, hash::Hash::hash (value, hasher));
	};
	
	( $type : ty, wrapper_rc_box, $restriction : tt, $tag : expr, $value : ident, $hasher : ident, $hasher_expression : expr ) => {
		impl_hash! ($type, self, $restriction, $tag, self, $value, StdRc::deref (&self.0), $hasher, $hasher_expression);
	};
	( $type : ty, wrapper_rc_box, $restriction : tt, $tag : expr ) => {
		impl_hash! ($type, wrapper_rc, $restriction, $tag, value, hasher, hash::Hash::hash (value, hasher));
	};
	
	
	( value_immutable_and_mutable, $type_immutable : ty, $type_mutable : ty, $restriction : tt, $tag_immutable : expr, $tag_mutable : expr, $ref_accessor : ident, $value : ident, $hasher : ident, $mode : ident, $hasher_expression : expr ) => {
		impl_hash! ($type_immutable, impl, none, self, $hasher, $mode, {
				impl_hash! (restriction, $restriction, $mode);
				let $value = self;
				$hasher.write_u128 ($tag_immutable);
				$hasher_expression;
			});
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		impl_hash! ($type_mutable, impl, none, self, $hasher, $mode, {
				impl_hash! (restriction, $restriction, $mode);
				if ! $mode.accept_mutable () {
					fail! (0x36bb21cd);
				}
				let $value = try! (self.$ref_accessor ());
				if $mode.coerce_mutable () {
					$hasher.write_u128 ($tag_immutable);
				} else {
					$hasher.write_u128 ($tag_mutable);
				}
				$hasher_expression;
			});
	};
	( value_immutable_and_mutable, $type_immutable : ty, $type_mutable : ty, $restriction : tt, $tag_immutable : expr, $tag_mutable : expr, $ref_accessor : ident, $value_accessor : ident, $value : ident, $hasher : ident, $mode : ident, $hasher_expression : expr ) => {
		impl_hash! (value_immutable_and_mutable, $type_immutable, $type_mutable, $restriction, $tag_immutable, $tag_mutable, $ref_accessor, $value, $hasher, $mode, { let $value = $value.$value_accessor (); $hasher_expression; });
	};
	( value_immutable_and_mutable, $type_immutable : ty, $type_mutable : ty, $restriction : tt, $tag_immutable : expr, $tag_mutable : expr, $ref_accessor : ident, $value_accessor : ident, hash ) => {
		impl_hash! (value_immutable_and_mutable, $type_immutable, $type_mutable, $restriction, $tag_immutable, $tag_mutable, $ref_accessor, $value_accessor, value, hasher, mode, hash::Hash::hash (value, hasher));
	};
	( value_immutable_and_mutable, $type_immutable : ty, $type_mutable : ty, $restriction : tt, $tag_immutable : expr, $tag_mutable : expr, $ref_accessor : ident, $value_accessor : ident, hash_value ) => {
		impl_hash! (value_immutable_and_mutable, $type_immutable, $type_mutable, $restriction, $tag_immutable, $tag_mutable, $ref_accessor, $value_accessor, value, hasher, mode, try! (HashValue::hash_value (value, hasher, mode)));
	};
	( value_immutable_and_mutable, $type_immutable : ty, $type_mutable : ty, $restriction : tt, $tag_immutable : expr, $tag_mutable : expr, $ref_accessor : ident, $value_accessor : ident, hash_value_ref ) => {
		impl_hash! (value_immutable_and_mutable, $type_immutable, $type_mutable, $restriction, $tag_immutable, $tag_mutable, $ref_accessor, $value_accessor, value, hasher, mode, try! (HashValue::hash_value (&value, hasher, mode)));
	};
	
}




impl_hash! (Value, hash);

impl HashValue for Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash_value <Hasher : hash::Hasher> (&self, hasher : &mut Hasher, mode : HashMode) -> (Outcome<()>) {
		match self.kind_match_as_ref () {
			
			ValueKindMatchAsRef::Null =>
				return NULL.hash_value (hasher, mode),
			ValueKindMatchAsRef::Void =>
				return VOID.hash_value (hasher, mode),
			ValueKindMatchAsRef::Undefined =>
				return UNDEFINED.hash_value (hasher, mode),
			ValueKindMatchAsRef::Singleton (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			ValueKindMatchAsRef::Boolean (self_0) =>
				return self_0.hash_value (hasher, mode),
			ValueKindMatchAsRef::NumberInteger (self_0) =>
				return self_0.hash_value (hasher, mode),
			ValueKindMatchAsRef::NumberReal (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::Character (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			ValueKindMatchAsRef::Symbol (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef::Keyword (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef::Unique (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringImmutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::StringMutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesImmutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::BytesMutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef::StringRegex (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef::BytesRegex (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			ValueKindMatchAsRef::PairImmutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::PairMutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef::ArrayImmutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::ArrayMutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef::Values (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordKind (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef::RecordImmutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef::RecordMutable (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef::Error (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			ValueKindMatchAsRef::ProcedurePrimitive (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			ValueKindMatchAsRef::SyntaxPrimitive (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::SyntaxExtended (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::SyntaxNative (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::SyntaxLambda (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef::Path (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef::Port (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef::Process (self_0) =>
				return self_0.hash_value (hasher, mode),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Context (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef::Binding (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameters (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef::Promise (self_0) =>
				return self_0.hash_value (hasher, mode),
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef::Opaque (self_0) =>
				return self_0.hash_value (hasher, mode),
			
		}
	}
}




impl_hash! (ValueSingleton, hash);

impl HashValue for ValueSingleton {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash_value <Hasher : hash::Hasher> (&self, hasher : &mut Hasher, mode : HashMode) -> (Outcome<()>) {
		match *self {
			
			ValueSingleton::Null =>
				hasher.write_u128 (0x4045ac30d18166d4cd6ed66090869269),
			
			ValueSingleton::Undefined =>
				if mode.accept_undefined () {
					hasher.write_u128 (0xfdcc1bf260ea3c16c46f00cd4a2445ce);
				} else {
					fail! (0x5bb25d0c);
				},
			
			ValueSingleton::Void =>
				hasher.write_u128 (0x5da97915a409f23468f15313310251e8),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueSingleton::PortEof =>
				if mode.accept_inserializable () {
					hasher.write_u128 (0x34be54eb4b99f775f681cb609da3e824);
				} else {
					fail! (0xae73bf1b);
				},
			
		}
		succeed! (());
	}
}




impl_hash! (Boolean, wrapper, none, 0x25f391d6cc82cd508832c404a790eef5);
impl_hash! (NumberInteger, wrapper, none, 0x47f47462de83672230c577284ba03679);
impl_hash! (NumberReal, wrapper, none, 0x072e7de41084ec776a1160e0f64a6bea, value, hasher, hasher.write_u64 (value.to_bits ()));

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_hash! (Character, wrapper, none, 0xa622f3b966b5e12c66f01366eb8b3a48);

impl_hash! (Symbol, accessor, none, 0x6dd6706c6ea78b6cb5dd255ce96709ad, string_as_str, hash);
#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
impl_hash! (Keyword, accessor, none, 0x09c9657aa53dd76b50a570b5bd7cbe37, string_as_str, hash);
#[ cfg ( feature = "vonuvoli_values_unique" ) ]
impl_hash! (Unique, accessor, inserializable, 0x8a93010164dd5b6f23cc8142001311ca, data_ref, hash);

#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_hash! (value_immutable_and_mutable, StringImmutable, StringMutable, none, 0x3f4d8c3fbadd36f9ee979859f21a6793, 0x3f8b192c74bfbc75b6280b62db2e7759, string_ref, string_as_str, hash);

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_hash! (value_immutable_and_mutable, BytesImmutable, BytesMutable, none, 0x8da3faaaf4541a7c1f42ac7961ad9d6b, 0x52ab4587d45527eb443f5ad1a7ad867a, bytes_ref, bytes_as_slice, hash);




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl_hash! (StringRegex, accessor_2, inserializable, 0x03927212c3f305665cf93abe513b185b, regex_ref, as_str, hash);

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
impl_hash! (BytesRegex, accessor_2, inserializable, 0xc50164849cd4dbdabd53e27fefa04b7c, regex_ref, as_str, hash);




#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
impl_hash! (Path, accessor, none, 0x6d4b74a132197b882f0aca4fbbc26cb2, path_ref, hash);




impl_hash! (value_immutable_and_mutable, PairImmutable, PairMutable, none, 0x93f79cce8c0375534b0e43c88d8da92e, 0xb41a3ecff1dcad453e1c981dd6e96b6c, pair_ref, left_and_right, hash_value_ref);

#[ cfg ( feature = "vonuvoli_values_array" ) ]
impl_hash! (value_immutable_and_mutable, ArrayImmutable, ArrayMutable, none, 0xde9ddde9ae201e62a3ac777cc11f235d, 0x7c77d388f236beb5d37ec793c846ffb7, array_ref, values_as_slice, hash_value);


#[ cfg ( feature = "vonuvoli_values_values" ) ]
impl_hash! (Values, accessor, none, 0x3a49f87741490e667172c821ae2bbb96, values_as_slice, hash_value);




#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_hash! (RecordKind, handle, inserializable, 0x952dc05164a775ed3d9e2b279ba75fb9);

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
impl_hash! (value_immutable_and_mutable, RecordImmutable, RecordMutable, inserializable, 0x311b566d611c0e987955bb26a3573a51, 0x930b5f206bfa74722c07d0adb3752b34, record_ref, value, hasher, mode, {
		try! (value.kind () .hash_value (hasher, mode));
		try! (value.values_as_slice () .hash_value (hasher, mode));
	});




FIXME! ("implement this by taking into account the actual members");
impl_hash! (Error, accessor_pointer, inserializable, 0xd3b9afc31de79603bb308260ce748c02, internals_ref);




impl_hash! (ProcedurePrimitive, standard, inserializable, 0x758c28038566300601ee61e1e05d620a);

impl_hash! (SyntaxPrimitive, standard, inserializable, 0xf8e56f5c9ba6c6e19ff7e76952cfe8ba);


#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_hash! (ProcedureNative, handle, inserializable, 0xa40bd6268c631bc6b226b0dbad49b53e);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
impl_hash! (SyntaxNative, handle, inserializable, 0xe354e84bf8d94a1514be54e239d6d313);


#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_hash! (ProcedureExtended, accessor_pointer, inserializable, 0x23515b96ded4ee016f23fd8e2be926de, internals_ref);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
impl_hash! (SyntaxExtended, accessor_pointer, inserializable, 0x34567f9a978d44594fc8c47c23684823, internals_ref);


// #[ cfg ( feature = "vonuvoli_expressions" ) ]
// #[ cfg ( feature = "vonuvoli_values_lambda" ) ]
// impl_hash! (LambdaInternals, handle, inserializable, 0x4dd0c94669ae7adc9f8571c2987952ca);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_hash! (Lambda, handle, inserializable, 0x099a1f479ca3e712f2ce63d5d4eba2ae);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_hash! (ProcedureLambda, handle, inserializable, 0x5d413fb3b954f48f4fd7f219123caea1);

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
impl_hash! (SyntaxLambda, handle, inserializable, 0x64663f0bd7453054c741e04286c8bbd2);




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
impl_hash! (Port, handle, inserializable, 0x914ac1168b5ec5b34057e377346dd3ce);

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
impl_hash! (Process, handle, inserializable, 0xe88e40f39bf45560632906cc04e72fba);




impl_hash! (Context, handle, inserializable, 0xa32fdd89f20900e0784cc49fdd4b157d);
impl_hash! (Registers, handle, inserializable, 0xb070b3225a27e9beff0d81badaee7a17);
impl_hash! (Binding, handle, inserializable, 0xa0ad17e98b85776ec3964be4cf704b0c);


#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl_hash! (Parameters, handle, inserializable, 0xca0b9aabd06917df3e8a09142cdda72c);
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
impl_hash! (Parameter, handle, inserializable, 0x2295a9f1223d836010c471f303ededbb);


#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
impl_hash! (Opaque, handle, inserializable, 0xe9b99ee1c8d6fd101a9b17912d213374);

#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
impl_hash! (Promise, unimplemented, 0xddee6a18, (github_issue, 4));

