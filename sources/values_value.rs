

use super::contexts::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::extended_syntaxes::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::native_syntaxes::exports::*;
use super::ports::exports::*;
use super::primitives::exports::*;
use super::processes::exports::*;
use super::values_arrays::exports::*;
use super::values_booleans::exports::*;
use super::values_bytes::exports::*;
use super::values_characters::exports::*;
use super::values_numbers::exports::*;
use super::values_pairs::exports::*;
use super::values_symbols::exports::*;
use super::values_strings::exports::*;
use super::values_values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{ValueKind, ValueClass};
	pub use super::{Value, ValueBox, ValueVec};
	pub use super::{ValueMeta1, ValueMeta2, VALUE_META_1, VALUE_META_2};
	pub use super::{ValueSingleton};
	pub use super::{ValueRef};
	pub use super::{GenericRef};
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueKind {
	
	Null,
	Void,
	Undefined,
	Singleton,
	
	Boolean,
	NumberInteger,
	NumberReal,
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
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	Singleton,
	
	Boolean,
	Number,
	Character,
	
	Symbol,
	String,
	Bytes,
	
	Pair,
	Array,
	Values,
	
	Error,
	
	Procedure,
	Syntax,
	
	Port,
	Resource,
	
	Opaque,
	
}




#[ derive (Clone) ]
pub enum Value {
	
	Singleton ( ValueMeta1, ValueSingleton, ValueMeta2 ),
	
	Boolean ( ValueMeta1, Boolean, ValueMeta2 ),
	NumberInteger ( ValueMeta1, NumberInteger, ValueMeta2 ),
	NumberReal ( ValueMeta1, NumberReal, ValueMeta2 ),
	Character ( ValueMeta1, Character, ValueMeta2 ),
	
	Symbol ( ValueMeta1, Symbol, ValueMeta2 ),
	StringImmutable ( ValueMeta1, StringImmutable, ValueMeta2 ),
	StringMutable ( ValueMeta1, StringMutable, ValueMeta2 ),
	BytesImmutable ( ValueMeta1, BytesImmutable, ValueMeta2 ),
	BytesMutable ( ValueMeta1, BytesMutable, ValueMeta2 ),
	
	PairImmutable ( ValueMeta1, PairImmutable, ValueMeta2 ),
	PairMutable ( ValueMeta1, PairMutable, ValueMeta2 ),
	ArrayImmutable ( ValueMeta1, ArrayImmutable, ValueMeta2 ),
	ArrayMutable ( ValueMeta1, ArrayMutable, ValueMeta2 ),
	Values ( ValueMeta1, Values, ValueMeta2 ),
	
	Error ( ValueMeta1, Error, ValueMeta2 ),
	
	ProcedurePrimitive ( ValueMeta1, ProcedurePrimitive, ValueMeta2 ),
	ProcedureExtended ( ValueMeta1, ProcedureExtended, ValueMeta2 ),
	ProcedureNative ( ValueMeta1, ProcedureNative, ValueMeta2 ),
	ProcedureLambda ( ValueMeta1, ProcedureLambda, ValueMeta2 ),
	
	SyntaxPrimitive ( ValueMeta1, SyntaxPrimitive, ValueMeta2, ),
	SyntaxExtended ( ValueMeta1, SyntaxExtended, ValueMeta2, ),
	SyntaxNative ( ValueMeta1, SyntaxNative, ValueMeta2, ),
	SyntaxLambda ( ValueMeta1, SyntaxLambda, ValueMeta2, ),
	
	Port ( ValueMeta1, Port, ValueMeta2, ),
	Process ( ValueMeta1, Process, ValueMeta2, ),
	
	Context ( ValueMeta1, Context, ValueMeta2 ),
	Binding ( ValueMeta1, Binding, ValueMeta2 ),
	
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


impl Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (ValueKind) {
		match *self {
			
			Value::Singleton (_, ref value, _) =>
				match *value {
					ValueSingleton::Null => ValueKind::Null,
					ValueSingleton::Void => ValueKind::Void,
					ValueSingleton::Undefined => ValueKind::Undefined,
					ValueSingleton::PortEof => ValueKind::Singleton,
				},
			
			Value::Boolean (_, _, _) => ValueKind::Boolean,
			Value::NumberInteger (_, _, _) => ValueKind::NumberInteger,
			Value::NumberReal (_, _, _) => ValueKind::NumberReal,
			Value::Character (_, _, _) => ValueKind::Character,
			
			Value::Symbol (_, _, _) => ValueKind::Symbol,
			Value::StringImmutable (_, _, _) => ValueKind::StringImmutable,
			Value::StringMutable (_, _, _) => ValueKind::StringMutable,
			Value::BytesImmutable (_, _, _) => ValueKind::BytesImmutable,
			Value::BytesMutable (_, _, _) => ValueKind::BytesMutable,
			
			Value::PairImmutable (_, _, _) => ValueKind::PairImmutable,
			Value::PairMutable (_, _, _) => ValueKind::PairMutable,
			Value::ArrayImmutable (_, _, _) => ValueKind::ArrayImmutable,
			Value::ArrayMutable (_, _, _) => ValueKind::ArrayMutable,
			Value::Values (_, _, _) => ValueKind::Values,
			
			Value::Error (_, _, _) => ValueKind::Error,
			
			Value::ProcedurePrimitive (_, _, _) => ValueKind::ProcedurePrimitive,
			Value::ProcedureExtended (_, _, _) => ValueKind::ProcedureExtended,
			Value::ProcedureNative (_, _, _) => ValueKind::ProcedureNative,
			Value::ProcedureLambda (_, _, _) => ValueKind::ProcedureLambda,
			
			Value::SyntaxPrimitive (_, _, _) => ValueKind::SyntaxPrimitive,
			Value::SyntaxExtended (_, _, _) => ValueKind::SyntaxExtended,
			Value::SyntaxNative (_, _, _) => ValueKind::SyntaxNative,
			Value::SyntaxLambda (_, _, _) => ValueKind::SyntaxLambda,
			
			Value::Port (_, _, _) => ValueKind::Port,
			Value::Process (_, _, _) => ValueKind::Process,
			
			Value::Context (_, _, _) => ValueKind::Context,
			Value::Binding (_, _, _) => ValueKind::Binding,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn class (&self) -> (ValueClass) {
		match *self {
			
			Value::Singleton (_, ref value, _) =>
				match *value {
					ValueSingleton::Null => ValueClass::Null,
					ValueSingleton::Void => ValueClass::Void,
					ValueSingleton::Undefined => ValueClass::Undefined,
					ValueSingleton::PortEof => ValueClass::Singleton,
				},
			
			Value::Boolean (_, _, _) => ValueClass::Boolean,
			Value::NumberInteger (_, _, _) => ValueClass::Number,
			Value::NumberReal (_, _, _) => ValueClass::Number,
			Value::Character (_, _, _) => ValueClass::Character,
			
			Value::Symbol (_, _, _) => ValueClass::Symbol,
			Value::StringImmutable (_, _, _) => ValueClass::String,
			Value::StringMutable (_, _, _) => ValueClass::String,
			Value::BytesImmutable (_, _, _) => ValueClass::Bytes,
			Value::BytesMutable (_, _, _) => ValueClass::Bytes,
			
			Value::PairImmutable (_, _, _) => ValueClass::Pair,
			Value::PairMutable (_, _, _) => ValueClass::Pair,
			Value::ArrayImmutable (_, _, _) => ValueClass::Array,
			Value::ArrayMutable (_, _, _) => ValueClass::Array,
			Value::Values (_, _, _) => ValueClass::Values,
			
			Value::Error (_, _, _) => ValueClass::Error,
			
			Value::ProcedurePrimitive (_, _, _) => ValueClass::Procedure,
			Value::ProcedureExtended (_, _, _) => ValueClass::Procedure,
			Value::ProcedureNative (_, _, _) => ValueClass::Procedure,
			Value::ProcedureLambda (_, _, _) => ValueClass::Procedure,
			
			Value::SyntaxPrimitive (_, _, _) => ValueClass::Syntax,
			Value::SyntaxExtended (_, _, _) => ValueClass::Syntax,
			Value::SyntaxNative (_, _, _) => ValueClass::Syntax,
			Value::SyntaxLambda (_, _, _) => ValueClass::Syntax,
			
			Value::Port (_, _, _) => ValueClass::Port,
			Value::Process (_, _, _) => ValueClass::Resource,
			
			Value::Context (_, _, _) => ValueClass::Opaque,
			Value::Binding (_, _, _) => ValueClass::Opaque,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_kind (&self, kind : ValueKind) -> (bool) {
		self.kind () == kind
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_class (&self, class : ValueClass) -> (bool) {
		self.class () == class
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Value) -> (bool) {
		match (self, other) {
			
			(&Value::Singleton (_, ref self_0, _), &Value::Singleton (_, ref other_0, _)) => self_0 == other_0,
			
			(&Value::Boolean (_, ref self_0, _), &Value::Boolean (_, ref other_0, _)) => self_0 == other_0,
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => self_0 == other_0,
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => self_0 == other_0,
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => self_0 == other_0,
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => self_0 == other_0,
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => StringImmutable::is_self (self_0, other_0),
			(&Value::StringMutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => StringMutable::is_self (self_0, other_0),
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => BytesImmutable::is_self (self_0, other_0),
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => BytesMutable::is_self (self_0, other_0),
			
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => PairImmutable::is_self (self_0, other_0),
			(&Value::PairMutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => PairMutable::is_self (self_0, other_0),
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ArrayImmutable::is_self (self_0, other_0),
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ArrayMutable::is_self (self_0, other_0),
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => Values::is_self (self_0, other_0),
			
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => Error::is_self (self_0, other_0),
			
			(&Value::ProcedurePrimitive (_, ref self_0, _), &Value::ProcedurePrimitive (_, ref other_0, _)) => self_0 == other_0,
			(&Value::ProcedureExtended (_, ref self_0, _), &Value::ProcedureExtended (_, ref other_0, _)) => ProcedureExtended::is_self (self_0, other_0),
			(&Value::ProcedureNative (_, ref self_0, _), &Value::ProcedureNative (_, ref other_0, _)) => ProcedureNative::is_self (self_0, other_0),
			(&Value::ProcedureLambda (_, ref self_0, _), &Value::ProcedureLambda (_, ref other_0, _)) => ProcedureLambda::is_self (self_0, other_0),
			
			(&Value::SyntaxPrimitive (_, ref self_0, _), &Value::SyntaxPrimitive (_, ref other_0, _)) => self_0 == other_0,
			(&Value::SyntaxExtended (_, ref self_0, _), &Value::SyntaxExtended (_, ref other_0, _)) => SyntaxExtended::is_self (self_0, other_0),
			(&Value::SyntaxNative (_, ref self_0, _), &Value::SyntaxNative (_, ref other_0, _)) => SyntaxNative::is_self (self_0, other_0),
			(&Value::SyntaxLambda (_, ref self_0, _), &Value::SyntaxLambda (_, ref other_0, _)) => SyntaxLambda::is_self (self_0, other_0),
			
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => Port::is_self (self_0, other_0),
			(&Value::Process (_, ref self_0, _), &Value::Process (_, ref other_0, _)) => Process::is_self (self_0, other_0),
			
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => Context::is_self (self_0, other_0),
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => Binding::is_self (self_0, other_0),
			
			(_, _) => false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<Value>) {
		let value = match *self {
			
			Value::Singleton (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::Boolean (_, ref self_0, _) => self_0.clone () .into (),
			Value::NumberInteger (_, ref self_0, _) => self_0.clone () .into (),
			Value::NumberReal (_, ref self_0, _) => self_0.clone () .into (),
			Value::Character (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::Symbol (_, ref self_0, _) => self_0.clone () .into (),
			Value::StringImmutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::StringMutable (_, ref self_0, _) => self_0.to_immutable () .into (),
			Value::BytesImmutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::BytesMutable (_, ref self_0, _) => self_0.to_immutable () .into (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::PairMutable (_, ref self_0, _) => self_0.to_immutable () .into (),
			Value::ArrayImmutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::ArrayMutable (_, ref self_0, _) => self_0.to_immutable () .into (),
			Value::Values (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::Error (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => self_0.clone () .into (),
			Value::ProcedureExtended (_, ref self_0, _) => self_0.clone () .into (),
			Value::ProcedureNative (_, ref self_0, _) => self_0.clone () .into (),
			Value::ProcedureLambda (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => self_0.clone () .into (),
			Value::SyntaxExtended (_, ref self_0, _) => self_0.clone () .into (),
			Value::SyntaxNative (_, ref self_0, _) => self_0.clone () .into (),
			Value::SyntaxLambda (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::Port (_, _, _) => fail! (0xe4de734c),
			Value::Process (_, _, _) => fail! (0x629f6149),
			
			Value::Context (_, _, _) => fail! (0x7e3a414d),
			Value::Binding (_, _, _) => fail! (0xcf5a0e0d),
			
		};
		succeed! (value);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (Outcome<Value>) {
		let value = match *self {
			
			Value::StringImmutable (_, ref self_0, _) => self_0.to_mutable () .into (),
			Value::StringMutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::BytesImmutable (_, ref self_0, _) => self_0.to_mutable () .into (),
			Value::BytesMutable (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.to_mutable () .into (),
			Value::PairMutable (_, ref self_0, _) => self_0.clone () .into (),
			Value::ArrayImmutable (_, ref self_0, _) => self_0.to_mutable () .into (),
			Value::ArrayMutable (_, ref self_0, _) => self_0.clone () .into (),
			
			Value::Port (_, ref self_0, _) => self_0.clone () .into (),
			Value::Process (_, ref self_0, _) => self_0.clone () .into (),
			
			_ => fail! (0x34e2a415),
			
		};
		succeed! (value);
	}
}




#[ derive (Clone) ]
pub struct ValueMeta1 ( u8, u8, u8 );

pub const VALUE_META_1 : ValueMeta1 = ValueMeta1 (0, 0, 0);




#[ derive (Clone) ]
pub struct ValueMeta2 ( u8, u8, u8, u8 );

pub const VALUE_META_2 : ValueMeta2 = ValueMeta2 (0, 0, 0, 0);




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueSingleton {
	Null,
	Undefined,
	Void,
	PortEof,
}




#[ derive (Debug) ]
pub enum ValueRef <'a> {
	Immutable (&'a Value),
	ImmutableEmbedded (StdRc<StdAny>, &'a Value),
	Mutable (StdRef<'a, Value>),
	MutableEmbedded (StdRc<StdAny>, StdRef<'a, Value>),
}


impl <'a> ValueRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_ref (&self) -> (&Value) {
		match *self {
			ValueRef::Immutable (value) =>
				value,
			ValueRef::ImmutableEmbedded (_, value) =>
				value,
			ValueRef::Mutable (ref value) =>
				value,
			ValueRef::MutableEmbedded (_, ref value) =>
				value,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_clone (&self) -> (Value) {
		match *self {
			ValueRef::Immutable (value) =>
				(*value) .clone (),
			ValueRef::ImmutableEmbedded (_, value) =>
				(*value) .clone (),
			ValueRef::Mutable (ref value) =>
				(*value) .clone (),
			ValueRef::MutableEmbedded (_, ref value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_is_self <OtherRef : StdAsRef<Value>> (&self, other : OtherRef) -> (bool) {
		Value::is_self (self.value_ref (), other.as_ref ())
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable (value : Value) -> (ValueRef<'a>) {
		let value = StdRc::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		ValueRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable <U : 'a, Accessor> (value : Value, accessor : Accessor) -> (ValueRef<'a>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, Value>)
	{
		let value = StdRc::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		ValueRef::MutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_ref (&self) -> (ValueRef<'a>) {
		match *self {
			ValueRef::Immutable (value) =>
				ValueRef::Immutable (value),
			ValueRef::ImmutableEmbedded (ref embedded, value) =>
				ValueRef::ImmutableEmbedded (StdRc::clone (embedded), value),
			ValueRef::Mutable (ref value) =>
				ValueRef::Mutable (StdRef::clone (value)),
			ValueRef::MutableEmbedded (ref embedded, ref value) =>
				ValueRef::MutableEmbedded (StdRc::clone (embedded), StdRef::clone (value)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn map_value <Transformer> (self, transformer : Transformer) -> (ValueRef<'a>)
			where Transformer : FnOnce (&Value) -> (&Value)
	{
		match self {
			ValueRef::Immutable (value) =>
				ValueRef::Immutable (transformer (value)),
			ValueRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, transformer (value)),
			ValueRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value, transformer)),
			ValueRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn map_generic <Output, Transformer> (self, transformer : Transformer) -> (GenericRef<'a, Output>)
			where Transformer : FnOnce (&Value) -> (&Output)
	{
		match self {
			ValueRef::Immutable (value) =>
				GenericRef::Immutable (transformer (value)),
			ValueRef::ImmutableEmbedded (embedded, value) =>
				GenericRef::ImmutableEmbedded (embedded, transformer (value)),
			ValueRef::Mutable (value) =>
				GenericRef::Mutable (StdRef::map (value, transformer)),
			ValueRef::MutableEmbedded (embedded, value) =>
				GenericRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
		}
	}
}


impl <'a> StdDeref for ValueRef<'a> {
	
	type Target = Value;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&Value) {
		self.value_ref ()
	}
}


impl <'a> StdAsRef<Value> for ValueRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&Value) {
		self.value_ref ()
	}
}




#[ derive (Debug) ]
pub enum GenericRef <'a, T : 'a> {
	Immutable (&'a T),
	ImmutableEmbedded (StdRc<StdAny>, &'a T),
	Mutable (StdRef<'a, T>),
	MutableEmbedded (StdRc<StdAny>, StdRef<'a, T>),
}


impl <'a, T : 'a> GenericRef<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn generic_ref (&self) -> (&T) {
		match *self {
			GenericRef::Immutable (value) =>
				value,
			GenericRef::ImmutableEmbedded (_, value) =>
				value,
			GenericRef::Mutable (ref value) =>
				value,
			GenericRef::MutableEmbedded (_, ref value) =>
				value,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable <U : 'static, Accessor> (value : U, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (&'a T)
	{
		let value = StdRc::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable <U : 'static, Accessor> (value : U, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, T>)
	{
		let value = StdRc::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::MutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_ref (&self) -> (GenericRef<'a, T>) {
		match *self {
			GenericRef::Immutable (value) =>
				GenericRef::Immutable (value),
			GenericRef::ImmutableEmbedded (ref embedded, value) =>
				GenericRef::ImmutableEmbedded (StdRc::clone (embedded), value),
			GenericRef::Mutable (ref value) =>
				GenericRef::Mutable (StdRef::clone (value)),
			GenericRef::MutableEmbedded (ref embedded, ref value) =>
				GenericRef::MutableEmbedded (StdRc::clone (embedded), StdRef::clone (value)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn map_value <Transformer> (self, transformer : Transformer) -> (ValueRef<'a>)
			where Transformer : FnOnce (&T) -> (&Value)
	{
		match self {
			GenericRef::Immutable (value) =>
				ValueRef::Immutable (transformer (value)),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				ValueRef::ImmutableEmbedded (embedded, transformer (value)),
			GenericRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value, transformer)),
			GenericRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn map_generic <Output, Transformer> (self, transformer : Transformer) -> (GenericRef<'a, Output>)
			where Transformer : FnOnce (&T) -> (&Output)
	{
		match self {
			GenericRef::Immutable (value) =>
				GenericRef::Immutable (transformer (value)),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				GenericRef::ImmutableEmbedded (embedded, transformer (value)),
			GenericRef::Mutable (value) =>
				GenericRef::Mutable (StdRef::map (value, transformer)),
			GenericRef::MutableEmbedded (embedded, value) =>
				GenericRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
		}
	}
}


impl <'a, T : 'a> StdDeref for GenericRef<'a, T> {
	
	type Target = T;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&T) {
		self.generic_ref ()
	}
}


impl <'a, T : 'a> StdAsRef<T> for GenericRef<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&T) {
		self.generic_ref ()
	}
}

