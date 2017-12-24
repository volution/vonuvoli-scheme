

use super::contexts::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::extended_syntaxes::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::native_syntaxes::exports::*;
use super::numbers::exports::*;
use super::ports::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{Value, ValueRef, ValueBox, ValueVec, ValueClass, ValueSingleton};
	
	pub use super::Boolean;
	pub use super::Character;
	pub use super::Symbol;
	pub use super::{String, StringRef, StringImmutable, StringMutable};
	pub use super::{Bytes, BytesRef, BytesImmutable, BytesMutable};
	pub use super::{Pair, PairRef, PairImmutable, PairMutable};
	pub use super::{Array, ArrayRef, ArrayImmutable, ArrayMutable};
	pub use super::Values;
	
	pub use super::{boolean, number_i64, number_f64, character};
	pub use super::{symbol_new, symbol_clone_str, symbol_clone_characters};
	pub use super::{string_immutable_new, string_immutable_clone_str, string_immutable_clone_characters};
	pub use super::{string_mutable_new, string_mutable_clone_str, string_mutable_clone_characters};
	pub use super::{bytes_immutable_new, bytes_immutable_clone_slice, bytes_immutable_clone_str, bytes_immutable_clone_characters};
	pub use super::{bytes_mutable_new, bytes_mutable_clone_slice, bytes_mutable_clone_str, bytes_mutable_clone_characters};
	pub use super::{array_immutable_new, array_immutable_clone_slice, array_immutable_clone_slice_ref};
	pub use super::{array_mutable_new, array_mutable_clone_slice, array_mutable_clone_slice_ref};
	pub use super::{values_new, values_new_from_vec, values_clone_slice, values_clone_slice_ref};
	pub use super::{pair_immutable_new};
	pub use super::{pair_mutable_new};
	
	pub use super::{string_new, bytes_new, array_new, pair_new};
	pub use super::{string_clone_str, bytes_clone_str};
	pub use super::{bytes_clone_slice, array_clone_slice};
	
	pub use super::{ValueMeta1, ValueMeta2, VALUE_META_1, VALUE_META_2};
	
	pub use super::super::numbers::NumberInteger;
	pub use super::super::numbers::NumberReal;
	
	pub use super::super::errors::Error;
	pub use super::super::contexts::Context;
	pub use super::super::contexts::Binding;
	pub use super::super::ports::Port;
	
	pub use super::super::primitives::exports::ProcedurePrimitive;
	pub use super::super::extended_procedures::exports::ProcedureExtended;
	pub use super::super::native_procedures::exports::ProcedureNative;
	pub use super::super::lambdas::exports::ProcedureLambda;
	
	pub use super::super::primitives::exports::SyntaxPrimitive;
	pub use super::super::extended_syntaxes::exports::SyntaxExtended;
	pub use super::super::native_syntaxes::exports::SyntaxNative;
	pub use super::super::lambdas::exports::SyntaxLambda;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	Singleton,
	
	Boolean,
	NumberInteger,
	NumberReal,
	Character,
	
	Symbol,
	String,
	Bytes,
	
	Pair,
	Array,
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
	
	Context,
	Binding,
	
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
	
	Context ( ValueMeta1, Context, ValueMeta2 ),
	Binding ( ValueMeta1, Binding, ValueMeta2 ),
	
}


#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueSingleton {
	Null,
	Undefined,
	Void,
	PortEof,
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


impl Value {
	
	#[ inline (always) ]
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
			Value::NumberInteger (_, _, _) => ValueClass::NumberInteger,
			Value::NumberReal (_, _, _) => ValueClass::NumberReal,
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
			
			Value::ProcedurePrimitive (_, _, _) => ValueClass::ProcedurePrimitive,
			Value::ProcedureExtended (_, _, _) => ValueClass::ProcedureExtended,
			Value::ProcedureNative (_, _, _) => ValueClass::ProcedureNative,
			Value::ProcedureLambda (_, _, _) => ValueClass::ProcedureLambda,
			
			Value::SyntaxPrimitive (_, _, _) => ValueClass::SyntaxPrimitive,
			Value::SyntaxExtended (_, _, _) => ValueClass::SyntaxExtended,
			Value::SyntaxNative (_, _, _) => ValueClass::SyntaxNative,
			Value::SyntaxLambda (_, _, _) => ValueClass::SyntaxLambda,
			
			Value::Port (_, _, _) => ValueClass::Port,
			
			Value::Context (_, _, _) => ValueClass::Context,
			Value::Binding (_, _, _) => ValueClass::Binding,
			
		}
	}
	
	#[ inline (always) ]
	pub fn is (&self, class : ValueClass) -> (bool) {
		self.class () == class
	}
	
	#[ inline (always) ]
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
			
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => Context::is_self (self_0, other_0),
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => Binding::is_self (self_0, other_0),
			
			(_, _) => false,
			
		}
	}
}




#[ derive (Clone) ]
pub struct ValueMeta1 ( u8, u8, u8 );
pub const VALUE_META_1 : ValueMeta1 = ValueMeta1 (0, 0, 0);

#[ derive (Clone) ]
pub struct ValueMeta2 ( u8, u8, u8, u8 );
pub const VALUE_META_2 : ValueMeta2 = ValueMeta2 (0, 0, 0, 0);




#[ derive (Debug) ]
pub enum ValueRef <'a> {
	Immutable (&'a Value),
	Mutable (StdRef<'a, Value>),
}


impl <'a> ValueRef<'a> {
	
	#[ inline (always) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			ValueRef::Immutable (value) =>
				(*value) .clone () .into (),
			ValueRef::Mutable (ref value) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ inline (always) ]
	pub fn is_self <OtherRef : StdAsRef<Value>> (&self, other : OtherRef) -> (bool) {
		Value::is_self (self.value_ref (), other.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn value_ref (&self) -> (&Value) {
		match *self {
			ValueRef::Immutable (value) =>
				value,
			ValueRef::Mutable (ref value) =>
				value,
		}
	}
	
	#[ inline (always) ]
	pub fn map_value <Transformer> (self, transformer : Transformer) -> (ValueRef<'a>)
			where Transformer : FnOnce (&Value) -> (&Value)
	{
		match self {
			ValueRef::Immutable (value) =>
				ValueRef::Immutable (transformer (value)),
			ValueRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value, transformer)),
		}
	}
	
	#[ inline (always) ]
	pub fn map_generic <Transformer, Output> (self, transformer : Transformer) -> (GenericRef<'a, Output>)
			where Transformer : FnOnce (&Value) -> (&Output)
	{
		match self {
			ValueRef::Immutable (value) =>
				GenericRef::Immutable (transformer (value)),
			ValueRef::Mutable (value) =>
				GenericRef::Mutable (StdRef::map (value, transformer)),
		}
	}
}


impl <'a> StdDeref for ValueRef<'a> {
	
	type Target = Value;
	
	#[ inline (always) ]
	fn deref (&self) -> (&Value) {
		match *self {
			ValueRef::Immutable (value) =>
				value,
			ValueRef::Mutable (ref value) =>
				&value,
		}
	}
}


impl <'a> StdAsRef<Value> for ValueRef<'a> {
	
	#[ inline (always) ]
	fn as_ref (&self) -> (&Value) {
		&self
	}
}




#[ derive (Debug) ]
pub enum GenericRef <'a, T : 'a> {
	Immutable (&'a T),
	Mutable (StdRef<'a, T>),
}


impl <'a, T : 'a> GenericRef<'a, T> {
	
	#[ inline (always) ]
	pub fn generic_ref (&self) -> (&T) {
		match *self {
			GenericRef::Immutable (value) =>
				value,
			GenericRef::Mutable (ref value) =>
				value,
		}
	}
	
	#[ inline (always) ]
	pub fn is_self <OtherRef : StdAsRef<T>> (&self, other : OtherRef) -> (bool) {
		ptr::eq (self.generic_ref (), other.as_ref ())
	}
}


impl <'a, T : 'a> StdDeref for GenericRef<'a, T> {
	
	type Target = T;
	
	#[ inline (always) ]
	fn deref (&self) -> (&T) {
		match *self {
			GenericRef::Immutable (value) =>
				value,
			GenericRef::Mutable (ref value) =>
				&value,
		}
	}
}


impl <'a, T : 'a> StdAsRef<T> for GenericRef<'a, T> {
	
	#[ inline (always) ]
	fn as_ref (&self) -> (&T) {
		&self
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Boolean ( pub bool );


impl Boolean {
	
	#[ inline (always) ]
	pub fn value (&self) -> (bool) {
		self.0
	}
}


impl Boolean {
	
	#[ inline (always) ]
	pub fn not (&self) -> (Boolean) {
		(!self.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn and (&self, other : &Boolean) -> (Boolean) {
		(self.0 && other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn or (&self, other : &Boolean) -> (Boolean) {
		(self.0 || other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn xor (&self, other : &Boolean) -> (Boolean) {
		(self.0 ^ other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn nand (&self, other : &Boolean) -> (Boolean) {
		self.and (other) .not ()
	}
	
	#[ inline (always) ]
	pub fn nor (&self, other : &Boolean) -> (Boolean) {
		self.or (other) .not ()
	}
	
	#[ inline (always) ]
	pub fn nxor (&self, other : &Boolean) -> (Boolean) {
		self.xor (other) .not ()
	}
}


impl ops::Not for Boolean {
	
	type Output = Boolean;
	
	#[ inline (always) ]
	fn not (self) -> (Boolean) {
		Boolean::not (&self)
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Character ( pub char );


impl Character {
	
	#[ inline (always) ]
	pub fn value (&self) -> (char) {
		self.0
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Symbol ( StdRc<StdString> );


impl Symbol {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Symbol) -> (bool) {
		let self_0 = self.0.as_ref ();
		let other_0 = other.0.as_ref ();
		ptr::eq (self_0, other_0) && (self_0 == other_0)
	}
	
	#[ inline (always) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdString>) {
		self.0.clone ()
	}
}


impl String for Symbol {
	
	#[ inline (always) ]
	fn string_as_string (&self) -> (&StdString) {
		self.0.as_ref ()
	}
}




pub trait String {
	
	fn string_as_string (&self) -> (&StdString);
	
	#[ inline (always) ]
	fn string_as_str (&self) -> (&str) {
		self.string_as_string () .as_str ()
	}
	
	#[ inline (always) ]
	fn string_as_bytes (&self) -> (&[u8]) {
		self.string_as_string () .as_bytes ()
	}
	
	#[ inline (always) ]
	fn string_chars (&self) -> (str::Chars) {
		self.string_as_string () .chars ()
	}
	
	#[ inline (always) ]
	fn string_clone (&self) -> (StdString) {
		self.string_as_string () .clone ()
	}
	
	#[ inline (always) ]
	fn string_is_empty (&self) -> (bool) {
		self.string_as_string () .is_empty ()
	}
	
	#[ inline (always) ]
	fn string_is_not_empty (&self) -> (bool) {
		! self.string_as_string () .is_empty ()
	}
	
	#[ inline (always) ]
	fn string_eq (&self, other : &str) -> (bool) {
		self.string_as_string () .eq (other)
	}
	
	#[ inline (always) ]
	fn string_utf8_bytes_count (&self) -> (usize) {
		self.string_as_string () .len ()
	}
	
	#[ inline (always) ]
	fn string_chars_count_compute (&self) -> (usize) {
		self.string_chars () .count ()
	}
	
	#[ inline (always) ]
	fn string_char_at_compute (&self, index : usize) -> (Option<char>) {
		self.string_chars () .nth (index)
	}
}




#[ derive (Debug) ]
pub enum StringRef <'a> {
	Immutable (&'a StringImmutable, &'a StdString),
	Mutable (&'a StringMutable, StdRef<'a, StdString>),
}


impl <'a> StringRef<'a> {
	
	#[ inline (always) ]
	pub fn try (value : &'a Value) -> (Outcome<StringRef<'a>>) {
		match *value {
			Value::StringImmutable (_, ref value, _) =>
				succeed! (value.string_ref ()),
			Value::StringMutable (_, ref value, _) =>
				succeed! (value.string_ref ()),
			_ =>
				fail! (0x20d78ff4),
		}
	}
	
	#[ inline (always) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			StringRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			StringRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &StringRef) -> (bool) {
		match (self, other) {
			(&StringRef::Immutable (self_0, _), &StringRef::Immutable (other_0, _)) =>
				StringImmutable::is_self (self_0, other_0),
			(&StringRef::Mutable (self_0, _), &StringRef::Mutable (other_0, _)) =>
				StringMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> String for StringRef<'a> {
	
	#[ inline (always) ]
	fn string_as_string (&self) -> (&StdString) {
		match *self {
			StringRef::Immutable (_, string) =>
				string,
			StringRef::Mutable (_, ref string) =>
				string,
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct StringImmutable ( StdRc<StdString> );


impl StringImmutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &StringImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn string_ref (&self) -> (StringRef) {
		StringRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdString>) {
		self.0.clone ()
	}
}


impl String for StringImmutable {
	
	#[ inline (always) ]
	fn string_as_string (&self) -> (&StdString) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct StringMutable ( StdRc<StdRefCell<StdString>> );


impl StringMutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &StringMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn string_ref (&self) -> (StringRef) {
		StringRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ inline (always) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdRefCell<StdString>>) {
		self.0.clone ()
	}
}




pub trait Bytes {
	
	fn bytes_as_vec (&self) -> (&StdVec<u8>);
	
	#[ inline (always) ]
	fn bytes_as_slice (&self) -> (&[u8]) {
		self.bytes_as_vec () .as_slice ()
	}
	
	#[ inline (always) ]
	fn bytes_iter (&self) -> (slice::Iter<u8>) {
		self.bytes_as_vec () .iter ()
	}
	
	#[ inline (always) ]
	fn bytes_clone (&self) -> (StdVec<u8>) {
		self.bytes_as_vec () .clone ()
	}
	
	#[ inline (always) ]
	fn values_is_empty (&self) -> (bool) {
		self.bytes_as_vec () .is_empty ()
	}
	
	#[ inline (always) ]
	fn values_is_not_empty (&self) -> (bool) {
		! self.bytes_as_vec () .is_empty ()
	}
	
	#[ inline (always) ]
	fn bytes_count (&self) -> (usize) {
		self.bytes_as_vec () .len ()
	}
}




#[ derive (Debug) ]
pub enum BytesRef <'a> {
	Immutable (&'a BytesImmutable, &'a StdVec<u8>),
	Mutable (&'a BytesMutable, StdRef<'a, StdVec<u8>>),
}


impl <'a> BytesRef<'a> {
	
	#[ inline (always) ]
	pub fn try (value : &'a Value) -> (Outcome<BytesRef<'a>>) {
		match *value {
			Value::BytesImmutable (_, ref value, _) =>
				succeed! (value.bytes_ref ()),
			Value::BytesMutable (_, ref value, _) =>
				succeed! (value.bytes_ref ()),
			_ =>
				fail! (0xb6042061),
		}
	}
	
	#[ inline (always) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			BytesRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			BytesRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &BytesRef) -> (bool) {
		match (self, other) {
			(&BytesRef::Immutable (self_0, _), &BytesRef::Immutable (other_0, _)) =>
				BytesImmutable::is_self (self_0, other_0),
			(&BytesRef::Mutable (self_0, _), &BytesRef::Mutable (other_0, _)) =>
				BytesMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
}


impl <'a> Bytes for BytesRef<'a> {
	
	#[ inline (always) ]
	fn bytes_as_vec (&self) -> (&StdVec<u8>) {
		match *self {
			BytesRef::Immutable (_, bytes) =>
				bytes,
			BytesRef::Mutable (_, ref bytes) =>
				bytes,
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct BytesImmutable ( StdRc<StdVec<u8>> );


impl BytesImmutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &BytesImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn bytes_ref (&self) -> (BytesRef) {
		BytesRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdVec<u8>>) {
		self.0.clone ()
	}
}


impl Bytes for BytesImmutable {
	
	#[ inline (always) ]
	fn bytes_as_vec (&self) -> (&StdVec<u8>) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct BytesMutable ( StdRc<StdRefCell<StdVec<u8>>> );


impl BytesMutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &BytesMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn bytes_ref (&self) -> (BytesRef) {
		BytesRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ inline (always) ]
	pub fn bytes_rc_clone (&self) -> (StdRc<StdRefCell<StdVec<u8>>>) {
		self.0.clone ()
	}
}




pub trait Pair {
	
	fn values_as_ref (&self) -> (&(Value, Value));
	
	#[ inline (always) ]
	fn left (&self) -> (&Value) {
		let values = self.values_as_ref ();
		&values.0
	}
	
	#[ inline (always) ]
	fn right (&self) -> (&Value) {
		let values = self.values_as_ref ();
		&values.1
	}
	
	#[ inline (always) ]
	fn left_and_right (&self) -> ((&Value, &Value)) {
		let values = self.values_as_ref ();
		(&values.0, &values.1)
	}
}




#[ derive (Debug) ]
pub enum PairRef <'a> {
	Immutable (&'a PairImmutable, &'a (Value, Value)),
	Mutable (&'a PairMutable, StdRef<'a, (Value, Value)>),
}


impl <'a> PairRef<'a> {
	
	#[ inline (always) ]
	pub fn try (value : &'a Value) -> (Outcome<PairRef<'a>>) {
		match *value {
			Value::PairImmutable (_, ref value, _) =>
				succeed! (value.pair_ref ()),
			Value::PairMutable (_, ref value, _) =>
				succeed! (value.pair_ref ()),
			_ =>
				fail! (0x0bb90a73),
		}
	}
	
	#[ inline (always) ]
	pub fn clone (&self) -> (Value) {
		match *self {
			PairRef::Immutable (value, _) =>
				(*value) .clone () .into (),
			PairRef::Mutable (value, _) =>
				(*value) .clone () .into (),
		}
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &PairRef) -> (bool) {
		match (self, other) {
			(&PairRef::Immutable (self_0, _), &PairRef::Immutable (other_0, _)) =>
				PairImmutable::is_self (self_0, other_0),
			(&PairRef::Mutable (self_0, _), &PairRef::Mutable (other_0, _)) =>
				PairMutable::is_self (self_0, other_0),
			_ =>
				false,
		}
	}
	
	#[ inline (always) ]
	pub fn left_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.0),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.0)),
		}
	}
	
	#[ inline (always) ]
	pub fn right_ref_into (self) -> (ValueRef<'a>) {
		match self {
			PairRef::Immutable (_, value) =>
				ValueRef::Immutable (&value.1),
			PairRef::Mutable (_, value) =>
				ValueRef::Mutable (StdRef::map (value, |value| &value.1)),
		}
	}
}


impl <'a> Pair for PairRef<'a> {
	
	#[ inline (always) ]
	fn values_as_ref (&self) -> (&(Value, Value)) {
		match *self {
			PairRef::Immutable (_, values) =>
				values,
			PairRef::Mutable (_, ref values) =>
				values,
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct PairImmutable ( StdRc<(Value, Value)> );


impl PairImmutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &PairImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Immutable (self, self.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn values_rc_clone (&self) -> (StdRc<(Value, Value)>) {
		self.0.clone ()
	}
}


impl Pair for PairImmutable {
	
	#[ inline (always) ]
	fn values_as_ref (&self) -> (&(Value, Value)) {
		self.0.as_ref ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd) ]
pub struct PairMutable ( StdRc<StdRefCell<(Value, Value)>> );


impl PairMutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &PairMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn pair_ref (&self) -> (PairRef) {
		PairRef::Mutable (self, self.0.as_ref () .borrow ())
	}
	
	#[ inline (always) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdRefCell<(Value, Value)>>) {
		self.0.clone ()
	}
}




pub trait Array {
	
	fn values_as_slice (&self) -> (&[Value]);
	fn values_as_iter (&self) -> (slice::Iter<Value>);
	fn values_ref (&self) -> (&StdVec<Value>);
	fn values_clone (&self) -> (StdVec<Value>);
	fn values_is_empty (&self) -> (bool);
	fn values_is_not_empty (&self) -> (bool);
	fn values_length (&self) -> (usize);
}




#[ derive (Debug) ]
pub enum ArrayRef <'a> {
	Immutable (&'a ArrayImmutable, &'a StdVec<Value>),
	Mutable (&'a ArrayMutable, StdRef<'a, StdVec<Value>>),
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ArrayImmutable ( StdRc<StdVec<Value>> );


impl ArrayImmutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ArrayImmutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdVec<Value>>) {
		self.0.clone ()
	}
}


impl Array for ArrayImmutable {
	
	#[ inline (always) ]
	fn values_as_slice (&self) -> (&[Value]) {
		self.values_ref () .as_slice ()
	}
	
	#[ inline (always) ]
	fn values_as_iter (&self) -> (slice::Iter<Value>) {
		self.values_ref () .iter ()
	}
	
	#[ inline (always) ]
	fn values_ref (&self) -> (&StdVec<Value>) {
		self.0.as_ref ()
	}
	
	#[ inline (always) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		self.values_ref () .clone ()
	}
	
	#[ inline (always) ]
	fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	fn values_is_not_empty (&self) -> (bool) {
		!self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ArrayMutable ( StdRc<StdVec<Value>> );


impl ArrayMutable {
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ArrayMutable) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdVec<Value>>) {
		self.0.clone ()
	}
}


impl Array for ArrayMutable {
	
	#[ inline (always) ]
	fn values_as_slice (&self) -> (&[Value]) {
		self.values_ref () .as_slice ()
	}
	
	#[ inline (always) ]
	fn values_as_iter (&self) -> (slice::Iter<Value>) {
		self.values_ref () .iter ()
	}
	
	#[ inline (always) ]
	fn values_ref (&self) -> (&StdVec<Value>) {
		self.0.as_ref ()
	}
	
	#[ inline (always) ]
	fn values_clone (&self) -> (StdVec<Value>) {
		self.values_ref () .clone ()
	}
	
	#[ inline (always) ]
	fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	fn values_is_not_empty (&self) -> (bool) {
		!self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Values ( StdRc<StdBox<[Value]>> );


impl Values {
	
	#[ inline (always) ]
	pub fn values_as_slice (&self) -> (&[Value]) {
		self.values_ref () .as_ref ()
	}
	
	#[ inline (always) ]
	pub fn values_as_iter (&self) -> (slice::Iter<Value>) {
		self.values_ref () .iter ()
	}
	
	#[ inline (always) ]
	pub fn values_ref (&self) -> (&StdBox<[Value]>) {
		self.0.as_ref ()
	}
	
	#[ inline (always) ]
	pub fn values_clone (&self) -> (StdBox<[Value]>) {
		self.values_ref () .clone ()
	}
	
	#[ inline (always) ]
	pub fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	pub fn values_is_not_empty (&self) -> (bool) {
		!self.values_ref () .is_empty ()
	}
	
	#[ inline (always) ]
	pub fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Values) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
	
	#[ inline (always) ]
	pub fn values_rc_clone (&self) -> (StdRc<StdBox<[Value]>>) {
		self.0.clone ()
	}
}




#[ inline (always) ]
pub fn boolean (value : bool) -> (Boolean) {
	Boolean (value)
}

#[ inline (always) ]
pub fn number_i64 (value : i64) -> (NumberInteger) {
	NumberInteger (value)
}

#[ inline (always) ]
pub fn number_f64 (value : f64) -> (NumberReal) {
	NumberReal (value)
}

#[ inline (always) ]
pub fn character (value : char) -> (Character) {
	Character (value)
}




#[ inline (always) ]
pub fn symbol_new (value : StdString) -> (Symbol) {
	Symbol (StdRc::new (value))
}




#[ inline (always) ]
pub fn string_immutable_new (value : StdString) -> (StringImmutable) {
	StringImmutable (StdRc::new (value))
}

#[ inline (always) ]
pub fn string_mutable_new (value : StdString) -> (StringMutable) {
	StringMutable (StdRc::new (StdRefCell::new (value)))
}




#[ inline (always) ]
pub fn bytes_immutable_new (values : StdVec<u8>) -> (BytesImmutable) {
	BytesImmutable (StdRc::new (values))
}

#[ inline (always) ]
pub fn bytes_mutable_new (values : StdVec<u8>) -> (BytesMutable) {
	BytesMutable (StdRc::new (StdRefCell::new (values)))
}




#[ inline (always) ]
pub fn symbol_clone_str (value : &str) -> (Symbol) {
	symbol_new (StdString::from (value))
}

#[ inline (always) ]
pub fn string_immutable_clone_str (value : &str) -> (StringImmutable) {
	string_immutable_new (StdString::from (value))
}

#[ inline (always) ]
pub fn string_mutable_clone_str (value : &str) -> (StringMutable) {
	string_mutable_new (StdString::from (value))
}

#[ inline (always) ]
pub fn bytes_immutable_clone_str (value : &str) -> (BytesImmutable) {
	bytes_immutable_new (StdString::from (value) .into_bytes ())
}

#[ inline (always) ]
pub fn bytes_mutable_clone_str (value : &str) -> (BytesMutable) {
	bytes_mutable_new (StdString::from (value) .into_bytes ())
}




#[ inline (always) ]
pub fn symbol_clone_characters (characters : &[char]) -> (Symbol) {
	symbol_new (characters_clone (characters))
}

#[ inline (always) ]
pub fn string_immutable_clone_characters (characters : &[char]) -> (StringImmutable) {
	string_immutable_new (characters_clone (characters))
}

#[ inline (always) ]
pub fn string_mutable_clone_characters (characters : &[char]) -> (StringMutable) {
	string_mutable_new (characters_clone (characters))
}

#[ inline (always) ]
pub fn bytes_immutable_clone_characters (characters : &[char]) -> (BytesImmutable) {
	bytes_immutable_new (characters_clone (characters) .into_bytes ())
}

#[ inline (always) ]
pub fn bytes_mutable_clone_characters (characters : &[char]) -> (BytesMutable) {
	bytes_mutable_new (characters_clone (characters) .into_bytes ())
}

#[ inline (always) ]
fn characters_clone (characters : &[char]) -> (StdString) {
	let mut value = StdString::with_capacity (characters.len ());
	for character in characters {
		value.push (*character);
	}
	StdString::from (value)
}




#[ inline (always) ]
pub fn bytes_immutable_clone_slice (values : &[u8]) -> (BytesImmutable) {
	bytes_immutable_new (vec_clone_slice (values))
}

#[ inline (always) ]
pub fn bytes_mutable_clone_slice (values : &[u8]) -> (BytesMutable) {
	bytes_mutable_new (vec_clone_slice (values))
}




#[ inline (always) ]
pub fn array_immutable_new (values : StdVec<Value>) -> (ArrayImmutable) {
	ArrayImmutable (StdRc::new (values))
}

#[ inline (always) ]
pub fn array_mutable_new (values : StdVec<Value>) -> (ArrayMutable) {
	ArrayMutable (StdRc::new (values))
}




#[ inline (always) ]
pub fn array_immutable_clone_slice (values : &[Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice (values))
}

#[ inline (always) ]
pub fn array_mutable_clone_slice (values : &[Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice (values))
}

#[ inline (always) ]
pub fn array_immutable_clone_slice_ref (values : &[&Value]) -> (ArrayImmutable) {
	array_immutable_new (vec_clone_slice_ref (values))
}

#[ inline (always) ]
pub fn array_mutable_clone_slice_ref (values : &[&Value]) -> (ArrayMutable) {
	array_mutable_new (vec_clone_slice_ref (values))
}




#[ inline (always) ]
pub fn values_new (values : StdBox<[Value]>) -> (Values) {
	Values (StdRc::new (values))
}

#[ inline (always) ]
pub fn values_new_from_vec (values : StdVec<Value>) -> (Values) {
	values_new (values.into_boxed_slice ())
}

#[ inline (always) ]
pub fn values_clone_slice (values : &[Value]) -> (Values) {
	values_new_from_vec (vec_clone_slice (values))
}

#[ inline (always) ]
pub fn values_clone_slice_ref (values : &[&Value]) -> (Values) {
	values_new_from_vec (vec_clone_slice_ref (values))
}




#[ inline (always) ]
pub fn pair_immutable_new (left : Value, right : Value) -> (PairImmutable) {
	PairImmutable (StdRc::new ((left, right)))
}

#[ inline (always) ]
pub fn pair_mutable_new (left : Value, right : Value) -> (PairMutable) {
	PairMutable (StdRc::new (StdRefCell::new ((left, right))))
}




#[ inline (always) ]
pub fn string_new (value : StdString) -> (Value) {
	if true {
		string_immutable_new (value) .into ()
	} else {
		string_mutable_new (value) .into ()
	}
}

#[ inline (always) ]
pub fn bytes_new (values : StdVec<u8>) -> (Value) {
	if true {
		bytes_immutable_new (values) .into ()
	} else {
		bytes_mutable_new (values) .into ()
	}
}

#[ inline (always) ]
pub fn array_new (values : StdVec<Value>) -> (Value) {
	if true {
		array_immutable_new (values) .into ()
	} else {
		array_mutable_new (values) .into ()
	}
}

#[ inline (always) ]
pub fn pair_new (left : Value, right : Value) -> (Value) {
	if true {
		pair_immutable_new (left, right) .into ()
	} else {
		pair_mutable_new (left, right) .into ()
	}
}




#[ inline (always) ]
pub fn string_clone_str (value : &str) -> (Value) {
	if true {
		string_immutable_clone_str (value) .into ()
	} else {
		string_mutable_clone_str (value) .into ()
	}
}

#[ inline (always) ]
pub fn bytes_clone_str (value : &str) -> (Value) {
	if true {
		bytes_immutable_clone_str (value) .into ()
	} else {
		bytes_mutable_clone_str (value) .into ()
	}
}




#[ inline (always) ]
pub fn bytes_clone_slice (values : &[u8]) -> (Value) {
	if true {
		bytes_immutable_clone_slice (values) .into ()
	} else {
		bytes_mutable_clone_slice (values) .into ()
	}
}

#[ inline (always) ]
pub fn array_clone_slice (values : &[Value]) -> (Value) {
	if true {
		array_immutable_clone_slice (values) .into ()
	} else {
		array_mutable_clone_slice (values) .into ()
	}
}

