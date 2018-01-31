

use super::constants::exports::*;
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
use super::values_records::exports::*;
use super::values_symbols::exports::*;
use super::values_strings::exports::*;
use super::values_values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::{ValueKind, ValueKindMatchAsRef, ValueKindMatchInto, ValueKindMatchAsRef2};
	pub use super::{ValueClass, ValueClassMatchAsRef, ValueClassMatchInto, ValueClassMatchAsRef2};
	pub use super::{Value, ValueBox, ValueVec};
	pub use super::{ValueMeta1, ValueMeta2, VALUE_META_1, VALUE_META_2};
	pub use super::{ValueSingleton};
	pub use super::{ProcedureMatchAsRef, ProcedureMatchInto};
	pub use super::{SyntaxMatchAsRef, SyntaxMatchInto};
	pub use super::{ResourceMatchAsRef, ResourceMatchInto};
	pub use super::{InternalMatchAsRef, InternalMatchInto};
	pub use super::{ListMatchAsRef, ListMatchInto};
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
	
	RecordKind,
	RecordImmutable,
	RecordMutable,
	
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
pub enum ValueKindMatchAsRef <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean),
	NumberInteger (&'a NumberInteger),
	NumberReal (&'a NumberReal),
	Character (&'a Character),
	
	Symbol (&'a Symbol),
	StringImmutable (&'a StringImmutable),
	StringMutable (&'a StringMutable),
	BytesImmutable (&'a BytesImmutable),
	BytesMutable (&'a BytesMutable),
	
	PairImmutable (&'a PairImmutable),
	PairMutable (&'a PairMutable),
	ArrayImmutable (&'a ArrayImmutable),
	ArrayMutable (&'a ArrayMutable),
	Values (&'a Values),
	
	RecordKind (&'a RecordKind),
	RecordImmutable (&'a RecordImmutable),
	RecordMutable (&'a RecordMutable),
	
	Error (&'a Error),
	
	ProcedurePrimitive (&'a ProcedurePrimitive),
	ProcedureExtended (&'a ProcedureExtended),
	ProcedureNative (&'a ProcedureNative),
	ProcedureLambda (&'a ProcedureLambda),
	
	SyntaxPrimitive (&'a SyntaxPrimitive),
	SyntaxExtended (&'a SyntaxExtended),
	SyntaxNative (&'a SyntaxNative),
	SyntaxLambda (&'a SyntaxLambda),
	
	Port (&'a Port),
	Process (&'a Process),
	
	Context (&'a Context),
	Binding (&'a Binding),
	
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueKindMatchInto {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (Boolean),
	NumberInteger (NumberInteger),
	NumberReal (NumberReal),
	Character (Character),
	
	Symbol (Symbol),
	StringImmutable (StringImmutable),
	StringMutable (StringMutable),
	BytesImmutable (BytesImmutable),
	BytesMutable (BytesMutable),
	
	PairImmutable (PairImmutable),
	PairMutable (PairMutable),
	ArrayImmutable (ArrayImmutable),
	ArrayMutable (ArrayMutable),
	Values (Values),
	
	RecordKind (RecordKind),
	RecordImmutable (RecordImmutable),
	RecordMutable (RecordMutable),
	
	Error (Error),
	
	ProcedurePrimitive (ProcedurePrimitive),
	ProcedureExtended (ProcedureExtended),
	ProcedureNative (ProcedureNative),
	ProcedureLambda (ProcedureLambda),
	
	SyntaxPrimitive (SyntaxPrimitive),
	SyntaxExtended (SyntaxExtended),
	SyntaxNative (SyntaxNative),
	SyntaxLambda (SyntaxLambda),
	
	Port (Port),
	Process (Process),
	
	Context (Context),
	Binding (Binding),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueKindMatchAsRef2 <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean, &'a Boolean),
	NumberInteger (&'a NumberInteger, &'a NumberInteger),
	NumberReal (&'a NumberReal, &'a NumberReal),
	Character (&'a Character, &'a Character),
	
	Symbol (&'a Symbol, &'a Symbol),
	StringImmutable (&'a StringImmutable, &'a StringImmutable),
	StringMutable (&'a StringMutable, &'a StringMutable),
	BytesImmutable (&'a BytesImmutable, &'a BytesImmutable),
	BytesMutable (&'a BytesMutable, &'a BytesMutable),
	
	PairImmutable (&'a PairImmutable, &'a PairImmutable),
	PairMutable (&'a PairMutable, &'a PairMutable),
	ArrayImmutable (&'a ArrayImmutable, &'a ArrayImmutable),
	ArrayMutable (&'a ArrayMutable, &'a ArrayMutable),
	Values (&'a Values, &'a Values),
	
	RecordKind (&'a RecordKind, &'a RecordKind),
	RecordImmutable (&'a RecordImmutable, &'a RecordImmutable),
	RecordMutable (&'a RecordMutable, &'a RecordMutable),
	
	Error (&'a Error, &'a Error),
	
	ProcedurePrimitive (&'a ProcedurePrimitive, &'a ProcedurePrimitive),
	ProcedureExtended (&'a ProcedureExtended, &'a ProcedureExtended),
	ProcedureNative (&'a ProcedureNative, &'a ProcedureNative),
	ProcedureLambda (&'a ProcedureLambda, &'a ProcedureLambda),
	
	SyntaxPrimitive (&'a SyntaxPrimitive, &'a SyntaxPrimitive),
	SyntaxExtended (&'a SyntaxExtended, &'a SyntaxExtended),
	SyntaxNative (&'a SyntaxNative, &'a SyntaxNative),
	SyntaxLambda (&'a SyntaxLambda, &'a SyntaxLambda),
	
	Port (&'a Port, &'a Port),
	Process (&'a Process, &'a Process),
	
	Context (&'a Context, &'a Context),
	Binding (&'a Binding, &'a Binding),
	
	Missmatched,
	
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
	
	RecordKind,
	Record,
	
	Error,
	
	Procedure,
	Syntax,
	
	Port,
	Resource,
	
	Internal,
	Opaque,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueClassMatchAsRef <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean),
	Number (NumberMatchAsRef<'a>),
	Character (&'a Character),
	
	Symbol (&'a Symbol),
	String (StringMatchAsRef<'a>),
	Bytes (BytesMatchAsRef<'a>),
	
	Pair (PairMatchAsRef<'a>),
	Array (ArrayMatchAsRef<'a>),
	Values (&'a Values),
	
	RecordKind (&'a RecordKind),
	Record (RecordMatchAsRef<'a>),
	
	Error (&'a Error),
	
	Procedure (ProcedureMatchAsRef<'a>),
	Syntax (SyntaxMatchAsRef<'a>),
	
	Port (&'a Port),
	Resource (ResourceMatchAsRef<'a>),
	
	Internal (InternalMatchAsRef<'a>),
	Opaque (&'a Value),
	
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueClassMatchInto {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (Boolean),
	Number (NumberMatchInto),
	Character (Character),
	
	Symbol (Symbol),
	String (StringMatchInto),
	Bytes (BytesMatchInto),
	
	Pair (PairMatchInto),
	Array (ArrayMatchInto),
	Values (Values),
	
	RecordKind (RecordKind),
	Record (RecordMatchInto),
	
	Error (Error),
	
	Procedure (ProcedureMatchInto),
	Syntax (SyntaxMatchInto),
	
	Port (Port),
	Resource (ResourceMatchInto),
	
	Internal (InternalMatchInto),
	Opaque (Value),
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ValueClassMatchAsRef2 <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean, &'a Boolean),
	Number (NumberMatchAsRef2<'a>),
	Character (&'a Character, &'a Character),
	
	Symbol (&'a Symbol, &'a Symbol),
	String (StringMatchAsRef2<'a>),
	Bytes (BytesMatchAsRef2<'a>),
	
	Pair (PairMatchAsRef2<'a>),
	Array (ArrayMatchAsRef2<'a>),
	Values (&'a Values, &'a Values),
	
	RecordKind (&'a RecordKind, &'a RecordKind),
	Record (RecordMatchAsRef2<'a>),
	
	Error (&'a Error, &'a Error),
	
	Procedure (ProcedureMatchAsRef<'a>, ProcedureMatchAsRef<'a>),
	Syntax (SyntaxMatchAsRef<'a>, SyntaxMatchAsRef<'a>),
	
	Port (&'a Port, &'a Port),
	Resource (ResourceMatchAsRef<'a>, ResourceMatchAsRef<'a>),
	
	Internal (InternalMatchAsRef<'a>, InternalMatchAsRef<'a>),
	Opaque (&'a Value, &'a Value),
	
	Missmatched (ValueClassMatchAsRef<'a>, ValueClassMatchAsRef<'a>),
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureMatchAsRef <'a> {
	Primitive (&'a ProcedurePrimitive),
	Extended (&'a ProcedureExtended),
	Native (&'a ProcedureNative),
	Lambda (&'a ProcedureLambda),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureMatchInto {
	Primitive (ProcedurePrimitive),
	Extended (ProcedureExtended),
	Native (ProcedureNative),
	Lambda (ProcedureLambda),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxMatchAsRef <'a> {
	Primitive (&'a SyntaxPrimitive),
	Extended (&'a SyntaxExtended),
	Native (&'a SyntaxNative),
	Lambda (&'a SyntaxLambda),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxMatchInto {
	Primitive (SyntaxPrimitive),
	Extended (SyntaxExtended),
	Native (SyntaxNative),
	Lambda (SyntaxLambda),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ResourceMatchAsRef <'a> {
	Process (&'a Process),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ResourceMatchInto {
	Process (Process),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum InternalMatchAsRef <'a> {
	Context (&'a Context),
	Binding (&'a Binding),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum InternalMatchInto {
	Context (Context),
	Binding (Binding),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListMatchAsRef <'a> {
	Null,
	PairImmutable (&'a PairImmutable),
	PairMutable (&'a PairMutable),
	Value (&'a Value),
}


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListMatchInto {
	Null,
	PairImmutable (PairImmutable),
	PairMutable (PairMutable),
	Value (Value),
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
	
	RecordKind ( ValueMeta1, RecordKind, ValueMeta2 ),
	RecordImmutable ( ValueMeta1, RecordImmutable, ValueMeta2 ),
	RecordMutable ( ValueMeta1, RecordMutable, ValueMeta2 ),
	
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
	
	__NonExhaustive,
	
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


impl Value {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind (&self) -> (ValueKind) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) =>
				match *self_0 {
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
			
			Value::RecordKind (_, _, _) => ValueKind::RecordKind,
			Value::RecordImmutable (_, _, _) => ValueKind::RecordImmutable,
			Value::RecordMutable (_, _, _) => ValueKind::RecordMutable,
			
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
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind_match_as_ref (&self) -> (ValueKindMatchAsRef) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) =>
				match *self_0 {
					ValueSingleton::Null => ValueKindMatchAsRef::Null,
					ValueSingleton::Void => ValueKindMatchAsRef::Void,
					ValueSingleton::Undefined => ValueKindMatchAsRef::Undefined,
					ValueSingleton::PortEof => ValueKindMatchAsRef::Singleton (*self_0),
				},
			
			Value::Boolean (_, ref self_0, _) => ValueKindMatchAsRef::Boolean (self_0),
			Value::NumberInteger (_, ref self_0, _) => ValueKindMatchAsRef::NumberInteger (self_0),
			Value::NumberReal (_, ref self_0, _) => ValueKindMatchAsRef::NumberReal (self_0),
			Value::Character (_, ref self_0, _) => ValueKindMatchAsRef::Character (self_0),
			
			Value::Symbol (_, ref self_0, _) => ValueKindMatchAsRef::Symbol (self_0),
			Value::StringImmutable (_, ref self_0, _) => ValueKindMatchAsRef::StringImmutable (self_0),
			Value::StringMutable (_, ref self_0, _) => ValueKindMatchAsRef::StringMutable (self_0),
			Value::BytesImmutable (_, ref self_0, _) => ValueKindMatchAsRef::BytesImmutable (self_0),
			Value::BytesMutable (_, ref self_0, _) => ValueKindMatchAsRef::BytesMutable (self_0),
			
			Value::PairImmutable (_, ref self_0, _) => ValueKindMatchAsRef::PairImmutable (self_0),
			Value::PairMutable (_, ref self_0, _) => ValueKindMatchAsRef::PairMutable (self_0),
			Value::ArrayImmutable (_, ref self_0, _) => ValueKindMatchAsRef::ArrayImmutable (self_0),
			Value::ArrayMutable (_, ref self_0, _) => ValueKindMatchAsRef::ArrayMutable (self_0),
			Value::Values (_, ref self_0, _) => ValueKindMatchAsRef::Values (self_0),
			
			Value::RecordKind (_, ref self_0, _) => ValueKindMatchAsRef::RecordKind (self_0),
			Value::RecordImmutable (_, ref self_0, _) => ValueKindMatchAsRef::RecordImmutable (self_0),
			Value::RecordMutable (_, ref self_0, _) => ValueKindMatchAsRef::RecordMutable (self_0),
			
			Value::Error (_, ref self_0, _) => ValueKindMatchAsRef::Error (self_0),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => ValueKindMatchAsRef::ProcedurePrimitive (self_0),
			Value::ProcedureExtended (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureExtended (self_0),
			Value::ProcedureNative (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureNative (self_0),
			Value::ProcedureLambda (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureLambda (self_0),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxPrimitive (self_0),
			Value::SyntaxExtended (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxExtended (self_0),
			Value::SyntaxNative (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxNative (self_0),
			Value::SyntaxLambda (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxLambda (self_0),
			
			Value::Port (_, ref self_0, _) => ValueKindMatchAsRef::Port (self_0),
			Value::Process (_, ref self_0, _) => ValueKindMatchAsRef::Process (self_0),
			
			Value::Context (_, ref self_0, _) => ValueKindMatchAsRef::Context (self_0),
			Value::Binding (_, ref self_0, _) => ValueKindMatchAsRef::Binding (self_0),
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind_match_into (self) -> (ValueKindMatchInto) {
		match self {
			
			Value::Singleton (_, self_0, _) =>
				match self_0 {
					ValueSingleton::Null => ValueKindMatchInto::Null,
					ValueSingleton::Void => ValueKindMatchInto::Void,
					ValueSingleton::Undefined => ValueKindMatchInto::Undefined,
					ValueSingleton::PortEof => ValueKindMatchInto::Singleton (self_0),
				},
			
			Value::Boolean (_, self_0, _) => ValueKindMatchInto::Boolean (self_0),
			Value::NumberInteger (_, self_0, _) => ValueKindMatchInto::NumberInteger (self_0),
			Value::NumberReal (_, self_0, _) => ValueKindMatchInto::NumberReal (self_0),
			Value::Character (_, self_0, _) => ValueKindMatchInto::Character (self_0),
			
			Value::Symbol (_, self_0, _) => ValueKindMatchInto::Symbol (self_0),
			Value::StringImmutable (_, self_0, _) => ValueKindMatchInto::StringImmutable (self_0),
			Value::StringMutable (_, self_0, _) => ValueKindMatchInto::StringMutable (self_0),
			Value::BytesImmutable (_, self_0, _) => ValueKindMatchInto::BytesImmutable (self_0),
			Value::BytesMutable (_, self_0, _) => ValueKindMatchInto::BytesMutable (self_0),
			
			Value::PairImmutable (_, self_0, _) => ValueKindMatchInto::PairImmutable (self_0),
			Value::PairMutable (_, self_0, _) => ValueKindMatchInto::PairMutable (self_0),
			Value::ArrayImmutable (_, self_0, _) => ValueKindMatchInto::ArrayImmutable (self_0),
			Value::ArrayMutable (_, self_0, _) => ValueKindMatchInto::ArrayMutable (self_0),
			Value::Values (_, self_0, _) => ValueKindMatchInto::Values (self_0),
			
			Value::RecordKind (_, self_0, _) => ValueKindMatchInto::RecordKind (self_0),
			Value::RecordImmutable (_, self_0, _) => ValueKindMatchInto::RecordImmutable (self_0),
			Value::RecordMutable (_, self_0, _) => ValueKindMatchInto::RecordMutable (self_0),
			
			Value::Error (_, self_0, _) => ValueKindMatchInto::Error (self_0),
			
			Value::ProcedurePrimitive (_, self_0, _) => ValueKindMatchInto::ProcedurePrimitive (self_0),
			Value::ProcedureExtended (_, self_0, _) => ValueKindMatchInto::ProcedureExtended (self_0),
			Value::ProcedureNative (_, self_0, _) => ValueKindMatchInto::ProcedureNative (self_0),
			Value::ProcedureLambda (_, self_0, _) => ValueKindMatchInto::ProcedureLambda (self_0),
			
			Value::SyntaxPrimitive (_, self_0, _) => ValueKindMatchInto::SyntaxPrimitive (self_0),
			Value::SyntaxExtended (_, self_0, _) => ValueKindMatchInto::SyntaxExtended (self_0),
			Value::SyntaxNative (_, self_0, _) => ValueKindMatchInto::SyntaxNative (self_0),
			Value::SyntaxLambda (_, self_0, _) => ValueKindMatchInto::SyntaxLambda (self_0),
			
			Value::Port (_, self_0, _) => ValueKindMatchInto::Port (self_0),
			Value::Process (_, self_0, _) => ValueKindMatchInto::Process (self_0),
			
			Value::Context (_, self_0, _) => ValueKindMatchInto::Context (self_0),
			Value::Binding (_, self_0, _) => ValueKindMatchInto::Binding (self_0),
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn kind_match_as_ref_2 <'a> (this : &'a Value, other : &'a Value) -> (ValueKindMatchAsRef2<'a>) {
		match (this, other) {
			
			(&Value::Singleton (_, self_0, _), &Value::Singleton (_, other_0, _)) =>
				match (self_0, other_0) {
					(ValueSingleton::Null, ValueSingleton::Null) => ValueKindMatchAsRef2::Null,
					(ValueSingleton::Void, ValueSingleton::Void) => ValueKindMatchAsRef2::Void,
					(ValueSingleton::Undefined, ValueSingleton::Undefined) => ValueKindMatchAsRef2::Void,
					(self_0, other_0) =>
						if self_0 == other_0 {
							ValueKindMatchAsRef2::Singleton (self_0)
						} else {
							ValueKindMatchAsRef2::Missmatched
						},
				},
			
			(&Value::Boolean (_, ref self_0, _), &Value::Boolean (_, ref other_0, _)) => ValueKindMatchAsRef2::Boolean (self_0, other_0),
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => ValueKindMatchAsRef2::NumberInteger (self_0, other_0),
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => ValueKindMatchAsRef2::NumberReal (self_0, other_0),
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => ValueKindMatchAsRef2::Character (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => ValueKindMatchAsRef2::Symbol (self_0, other_0),
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::StringImmutable (self_0, other_0),
			(&Value::StringMutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::StringMutable (self_0, other_0),
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::BytesImmutable (self_0, other_0),
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::BytesMutable (self_0, other_0),
			
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::PairImmutable (self_0, other_0),
			(&Value::PairMutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::PairMutable (self_0, other_0),
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0),
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::ArrayMutable (self_0, other_0),
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => ValueKindMatchAsRef2::Values (self_0, other_0),
			
			(&Value::RecordKind (_, ref self_0, _), &Value::RecordKind (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordKind (self_0, other_0),
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordImmutable (self_0, other_0),
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordMutable (self_0, other_0),
			
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => ValueKindMatchAsRef2::Error (self_0, other_0),
			
			(&Value::ProcedurePrimitive (_, ref self_0, _), &Value::ProcedurePrimitive (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0),
			(&Value::ProcedureExtended (_, ref self_0, _), &Value::ProcedureExtended (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0),
			(&Value::ProcedureNative (_, ref self_0, _), &Value::ProcedureNative (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureNative (self_0, other_0),
			(&Value::ProcedureLambda (_, ref self_0, _), &Value::ProcedureLambda (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0),
			
			(&Value::SyntaxPrimitive (_, ref self_0, _), &Value::SyntaxPrimitive (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0),
			(&Value::SyntaxExtended (_, ref self_0, _), &Value::SyntaxExtended (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0),
			(&Value::SyntaxNative (_, ref self_0, _), &Value::SyntaxNative (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxNative (self_0, other_0),
			(&Value::SyntaxLambda (_, ref self_0, _), &Value::SyntaxLambda (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0),
			
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => ValueKindMatchAsRef2::Port (self_0, other_0),
			(&Value::Process (_, ref self_0, _), &Value::Process (_, ref other_0, _)) => ValueKindMatchAsRef2::Process (self_0, other_0),
			
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => ValueKindMatchAsRef2::Context (self_0, other_0),
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => ValueKindMatchAsRef2::Binding (self_0, other_0),
			
			(&Value::__NonExhaustive, _) => unreachable! (),
			(_, &Value::__NonExhaustive) => unreachable! (),
			
			// NOTE:  !!! match-fallback !!!
			(_, _) => ValueKindMatchAsRef2::Missmatched,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn class (&self) -> (ValueClass) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) =>
				match *self_0 {
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
			
			Value::RecordKind (_, _, _) => ValueClass::RecordKind,
			Value::RecordImmutable (_, _, _) => ValueClass::Record,
			Value::RecordMutable (_, _, _) => ValueClass::Record,
			
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
			
			Value::Context (_, _, _) => ValueClass::Internal,
			Value::Binding (_, _, _) => ValueClass::Internal,
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn class_match_as_ref (&self) -> (ValueClassMatchAsRef) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) =>
				match *self_0 {
					ValueSingleton::Null => ValueClassMatchAsRef::Null,
					ValueSingleton::Void => ValueClassMatchAsRef::Void,
					ValueSingleton::Undefined => ValueClassMatchAsRef::Undefined,
					ValueSingleton::PortEof => ValueClassMatchAsRef::Singleton (*self_0),
				},
			
			Value::Boolean (_, ref self_0, _) => ValueClassMatchAsRef::Boolean (self_0),
			Value::NumberInteger (_, ref self_0, _) => ValueClassMatchAsRef::Number (NumberMatchAsRef::Integer (self_0)),
			Value::NumberReal (_, ref self_0, _) => ValueClassMatchAsRef::Number (NumberMatchAsRef::Real (self_0)),
			Value::Character (_, ref self_0, _) => ValueClassMatchAsRef::Character (self_0),
			
			Value::Symbol (_, ref self_0, _) => ValueClassMatchAsRef::Symbol (self_0),
			Value::StringImmutable (_, ref self_0, _) => ValueClassMatchAsRef::String (StringMatchAsRef::Immutable (self_0)),
			Value::StringMutable (_, ref self_0, _) => ValueClassMatchAsRef::String (StringMatchAsRef::Mutable (self_0)),
			
			Value::BytesImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Bytes (BytesMatchAsRef::Immutable (self_0)),
			Value::BytesMutable (_, ref self_0, _) => ValueClassMatchAsRef::Bytes (BytesMatchAsRef::Mutable (self_0)),
			
			Value::PairImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Pair (PairMatchAsRef::Immutable (self_0)),
			Value::PairMutable (_, ref self_0, _) => ValueClassMatchAsRef::Pair (PairMatchAsRef::Mutable (self_0)),
			Value::ArrayImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Array (ArrayMatchAsRef::Immutable (self_0)),
			Value::ArrayMutable (_, ref self_0, _) => ValueClassMatchAsRef::Array (ArrayMatchAsRef::Mutable (self_0)),
			Value::Values (_, ref self_0, _) => ValueClassMatchAsRef::Values (self_0),
			
			Value::RecordKind (_, ref self_0, _) => ValueClassMatchAsRef::RecordKind (self_0),
			Value::RecordImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Record (RecordMatchAsRef::Immutable (self_0)),
			Value::RecordMutable (_, ref self_0, _) => ValueClassMatchAsRef::Record (RecordMatchAsRef::Mutable (self_0)),
			
			Value::Error (_, ref self_0, _) => ValueClassMatchAsRef::Error (self_0),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Primitive (self_0)),
			Value::ProcedureExtended (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Extended (self_0)),
			Value::ProcedureNative (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Native (self_0)),
			Value::ProcedureLambda (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Lambda (self_0)),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Primitive (self_0)),
			Value::SyntaxExtended (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Extended (self_0)),
			Value::SyntaxNative (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Native (self_0)),
			Value::SyntaxLambda (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Lambda (self_0)),
			
			Value::Port (_, ref self_0, _) => ValueClassMatchAsRef::Port (self_0),
			Value::Process (_, ref self_0, _) => ValueClassMatchAsRef::Resource (ResourceMatchAsRef::Process (self_0)),
			
			Value::Context (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Context (self_0)),
			Value::Binding (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Binding (self_0)),
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn class_match_into (self) -> (ValueClassMatchInto) {
		match self {
			
			Value::Singleton (_, self_0, _) =>
				match self_0 {
					ValueSingleton::Null => ValueClassMatchInto::Null,
					ValueSingleton::Void => ValueClassMatchInto::Void,
					ValueSingleton::Undefined => ValueClassMatchInto::Undefined,
					ValueSingleton::PortEof => ValueClassMatchInto::Singleton (self_0),
				},
			
			Value::Boolean (_, self_0, _) => ValueClassMatchInto::Boolean (self_0),
			Value::NumberInteger (_, self_0, _) => ValueClassMatchInto::Number (NumberMatchInto::Integer (self_0)),
			Value::NumberReal (_, self_0, _) => ValueClassMatchInto::Number (NumberMatchInto::Real (self_0)),
			Value::Character (_, self_0, _) => ValueClassMatchInto::Character (self_0),
			
			Value::Symbol (_, self_0, _) => ValueClassMatchInto::Symbol (self_0),
			Value::StringImmutable (_, self_0, _) => ValueClassMatchInto::String (StringMatchInto::Immutable (self_0)),
			Value::StringMutable (_, self_0, _) => ValueClassMatchInto::String (StringMatchInto::Mutable (self_0)),
			
			Value::BytesImmutable (_, self_0, _) => ValueClassMatchInto::Bytes (BytesMatchInto::Immutable (self_0)),
			Value::BytesMutable (_, self_0, _) => ValueClassMatchInto::Bytes (BytesMatchInto::Mutable (self_0)),
			
			Value::PairImmutable (_, self_0, _) => ValueClassMatchInto::Pair (PairMatchInto::Immutable (self_0)),
			Value::PairMutable (_, self_0, _) => ValueClassMatchInto::Pair (PairMatchInto::Mutable (self_0)),
			Value::ArrayImmutable (_, self_0, _) => ValueClassMatchInto::Array (ArrayMatchInto::Immutable (self_0)),
			Value::ArrayMutable (_, self_0, _) => ValueClassMatchInto::Array (ArrayMatchInto::Mutable (self_0)),
			Value::Values (_, self_0, _) => ValueClassMatchInto::Values (self_0),
			
			Value::RecordKind (_, self_0, _) => ValueClassMatchInto::RecordKind (self_0),
			Value::RecordImmutable (_, self_0, _) => ValueClassMatchInto::Record (RecordMatchInto::Immutable (self_0)),
			Value::RecordMutable (_, self_0, _) => ValueClassMatchInto::Record (RecordMatchInto::Mutable (self_0)),
			
			Value::Error (_, self_0, _) => ValueClassMatchInto::Error (self_0),
			
			Value::ProcedurePrimitive (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Primitive (self_0)),
			Value::ProcedureExtended (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Extended (self_0)),
			Value::ProcedureNative (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Native (self_0)),
			Value::ProcedureLambda (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Lambda (self_0)),
			
			Value::SyntaxPrimitive (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Primitive (self_0)),
			Value::SyntaxExtended (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Extended (self_0)),
			Value::SyntaxNative (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Native (self_0)),
			Value::SyntaxLambda (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Lambda (self_0)),
			
			Value::Port (_, self_0, _) => ValueClassMatchInto::Port (self_0),
			Value::Process (_, self_0, _) => ValueClassMatchInto::Resource (ResourceMatchInto::Process (self_0)),
			
			Value::Context (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Context (self_0)),
			Value::Binding (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Binding (self_0)),
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn class_match_as_ref_2 <'a> (this : &'a Value, other : &'a Value) -> (ValueClassMatchAsRef2<'a>) {
		match (this, other) {
			
			(&Value::Singleton (_, self_0, _), &Value::Singleton (_, other_0, _)) =>
				match (self_0, other_0) {
					(ValueSingleton::Null, ValueSingleton::Null) => ValueClassMatchAsRef2::Null,
					(ValueSingleton::Void, ValueSingleton::Void) => ValueClassMatchAsRef2::Void,
					(ValueSingleton::Undefined, ValueSingleton::Undefined) => ValueClassMatchAsRef2::Void,
					(self_0, other_0) =>
						if self_0 == other_0 {
							ValueClassMatchAsRef2::Singleton (self_0)
						} else {
							ValueClassMatchAsRef2::Missmatched (ValueClassMatchAsRef::Singleton (self_0), ValueClassMatchAsRef::Singleton (other_0))
						},
				},
			
			(&Value::Boolean (_, ref self_0, _), &Value::Boolean (_, ref other_0, _)) => ValueClassMatchAsRef2::Boolean (self_0, other_0),
			
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => ValueClassMatchAsRef2::Number (NumberMatchAsRef2::IntegerBoth (self_0, other_0)),
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => ValueClassMatchAsRef2::Number (NumberMatchAsRef2::RealBoth (self_0, other_0)),
			(&Value::NumberInteger (_, ref self_0, _), &Value::NumberReal (_, ref other_0, _)) => ValueClassMatchAsRef2::Number (NumberMatchAsRef2::IntegerAndReal (self_0, other_0)),
			(&Value::NumberReal (_, ref self_0, _), &Value::NumberInteger (_, ref other_0, _)) => ValueClassMatchAsRef2::Number (NumberMatchAsRef2::RealAndInteger (self_0, other_0)),
			
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => ValueClassMatchAsRef2::Character (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => ValueClassMatchAsRef2::Symbol (self_0, other_0),
			
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::ImmutableBoth (self_0, other_0)),
			(&Value::StringMutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::MutableBoth (self_0, other_0)),
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			(&Value::StringMutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::ImmutableBoth (self_0, other_0)),
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::MutableBoth (self_0, other_0)),
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::ImmutableBoth (self_0, other_0)),
			(&Value::PairMutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::MutableBoth (self_0, other_0)),
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			(&Value::PairMutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::ImmutableBoth (self_0, other_0)),
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::MutableBoth (self_0, other_0)),
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => ValueClassMatchAsRef2::Values (self_0, other_0),
			
			(&Value::RecordKind (_, ref self_0, _), &Value::RecordKind (_, ref other_0, _)) => ValueClassMatchAsRef2::RecordKind (self_0, other_0),
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::ImmutableBoth (self_0, other_0)),
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::MutableBoth (self_0, other_0)),
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => ValueClassMatchAsRef2::Error (self_0, other_0),
			
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => ValueClassMatchAsRef2::Port (self_0, other_0),
			
			(&Value::__NonExhaustive, _) => unreachable! (),
			(_, &Value::__NonExhaustive) => unreachable! (),
			
			// NOTE:  !!! match-fallback !!!
			_ =>
				match (this.class_match_as_ref (), other.class_match_as_ref ()) {
					
					(ValueClassMatchAsRef::Procedure (self_0), ValueClassMatchAsRef::Procedure (other_0)) =>
						ValueClassMatchAsRef2::Procedure (self_0, other_0),
					(ValueClassMatchAsRef::Syntax (self_0), ValueClassMatchAsRef::Syntax (other_0)) =>
						ValueClassMatchAsRef2::Syntax (self_0, other_0),
					(ValueClassMatchAsRef::Resource (self_0), ValueClassMatchAsRef::Resource (other_0)) =>
						ValueClassMatchAsRef2::Resource (self_0, other_0),
					(ValueClassMatchAsRef::Opaque (self_0), ValueClassMatchAsRef::Opaque (other_0)) =>
						ValueClassMatchAsRef2::Opaque (self_0, other_0),
					
					// NOTE:  !!! match-fallback !!!
					(self_0, other_0) =>
						ValueClassMatchAsRef2::Missmatched (self_0, other_0),
					
				},
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn list_match_as_ref (&self) -> (ListMatchAsRef) {
		match *self {
			Value::Singleton (_, ValueSingleton::Null, _) => ListMatchAsRef::Null,
			Value::PairImmutable (_, ref self_0, _) => ListMatchAsRef::PairImmutable (self_0),
			Value::PairMutable (_, ref self_0, _) => ListMatchAsRef::PairMutable (self_0),
			Value::__NonExhaustive => unreachable! (),
			// NOTE:  !!! match-fallback !!!
			_ => ListMatchAsRef::Value (self),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn list_match_into (self) -> (ListMatchInto) {
		match self {
			Value::Singleton (_, ValueSingleton::Null, _) => ListMatchInto::Null,
			Value::PairImmutable (_, self_0, _) => ListMatchInto::PairImmutable (self_0),
			Value::PairMutable (_, self_0, _) => ListMatchInto::PairMutable (self_0),
			Value::__NonExhaustive => unreachable! (),
			// NOTE:  !!! match-fallback !!!
			_ => ListMatchInto::Value (self),
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
		match Value::kind_match_as_ref_2 (self, other) {
			
			ValueKindMatchAsRef2::Null => true,
			ValueKindMatchAsRef2::Void => true,
			ValueKindMatchAsRef2::Undefined => true,
			ValueKindMatchAsRef2::Singleton (_) => true,
			
			ValueKindMatchAsRef2::Boolean (self_0, other_0) => Boolean::eq (self_0, other_0),
			ValueKindMatchAsRef2::NumberInteger (self_0, other_0) => NumberInteger::eq (self_0, other_0),
			ValueKindMatchAsRef2::NumberReal (self_0, other_0) => NumberReal::eq (self_0, other_0),
			ValueKindMatchAsRef2::Character (self_0, other_0) => Character::eq (self_0, other_0),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Symbol::is_self (self_0, other_0),
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => StringImmutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => StringMutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => BytesImmutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => BytesMutable::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => PairImmutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => PairMutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => ArrayImmutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => ArrayMutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::Values (self_0, other_0) => Values::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::RecordKind (self_0, other_0) => RecordKind::is_self (self_0, other_0),
			ValueKindMatchAsRef2::RecordImmutable (self_0, other_0) => RecordImmutable::is_self (self_0, other_0),
			ValueKindMatchAsRef2::RecordMutable (self_0, other_0) => RecordMutable::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Error (self_0, other_0) => Error::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => ProcedurePrimitive::is_self (self_0, other_0),
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => ProcedureExtended::is_self (self_0, other_0),
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => ProcedureNative::is_self (self_0, other_0),
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => ProcedureLambda::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => SyntaxPrimitive::is_self (self_0, other_0),
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => SyntaxExtended::is_self (self_0, other_0),
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => SyntaxNative::is_self (self_0, other_0),
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => SyntaxLambda::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Port (self_0, other_0) => Port::is_self (self_0, other_0),
			ValueKindMatchAsRef2::Process (self_0, other_0) => Process::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Context (self_0, other_0) => Context::is_self (self_0, other_0),
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Binding::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Missmatched => false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<Value>) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Boolean (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::NumberInteger (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::NumberReal (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::Character (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Symbol (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::StringImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::StringMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			Value::BytesImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::BytesMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::PairMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			Value::ArrayImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::ArrayMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			Value::Values (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::RecordKind (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::RecordImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::RecordMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			
			Value::Error (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::ProcedureExtended (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::ProcedureNative (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::ProcedureLambda (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::SyntaxExtended (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::SyntaxNative (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::SyntaxLambda (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Port (_, _, _) => fail! (0xe4de734c),
			Value::Process (_, _, _) => fail! (0x629f6149),
			
			Value::Context (_, _, _) => fail! (0x7e3a414d),
			Value::Binding (_, _, _) => fail! (0xcf5a0e0d),
			
			Value::__NonExhaustive => unreachable! (),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (Outcome<Value>) {
		match *self {
			
			Value::StringImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::StringMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::BytesImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::BytesMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::PairMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::ArrayImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::ArrayMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::RecordImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::RecordMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Port (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::Process (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::__NonExhaustive => unreachable! (),
			
			// NOTE:  !!! match-fallback !!!
			_ => fail! (0x34e2a415),
			
		}
	}
}


impl ValueKindMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			
			ValueKindMatchInto::Null => NULL.into (),
			ValueKindMatchInto::Void => VOID.into (),
			ValueKindMatchInto::Undefined => UNDEFINED.into (),
			ValueKindMatchInto::Singleton (value) => value.into (),
			
			ValueKindMatchInto::Boolean (value) => value.into (),
			ValueKindMatchInto::NumberInteger (value) => value.into (),
			ValueKindMatchInto::NumberReal (value) => value.into (),
			ValueKindMatchInto::Character (value) => value.into (),
			
			ValueKindMatchInto::Symbol (value) => value.into (),
			ValueKindMatchInto::StringImmutable (value) => value.into (),
			ValueKindMatchInto::StringMutable (value) => value.into (),
			ValueKindMatchInto::BytesImmutable (value) => value.into (),
			ValueKindMatchInto::BytesMutable (value) => value.into (),
			
			ValueKindMatchInto::PairImmutable (value) => value.into (),
			ValueKindMatchInto::PairMutable (value) => value.into (),
			ValueKindMatchInto::ArrayImmutable (value) => value.into (),
			ValueKindMatchInto::ArrayMutable (value) => value.into (),
			ValueKindMatchInto::Values (value) => value.into (),
			
			ValueKindMatchInto::RecordKind (value) => value.into (),
			ValueKindMatchInto::RecordImmutable (value) => value.into (),
			ValueKindMatchInto::RecordMutable (value) => value.into (),
			
			ValueKindMatchInto::Error (value) => value.into (),
			
			ValueKindMatchInto::ProcedurePrimitive (value) => value.into (),
			ValueKindMatchInto::ProcedureExtended (value) => value.into (),
			ValueKindMatchInto::ProcedureNative (value) => value.into (),
			ValueKindMatchInto::ProcedureLambda (value) => value.into (),
			
			ValueKindMatchInto::SyntaxPrimitive (value) => value.into (),
			ValueKindMatchInto::SyntaxExtended (value) => value.into (),
			ValueKindMatchInto::SyntaxNative (value) => value.into (),
			ValueKindMatchInto::SyntaxLambda (value) => value.into (),
			
			ValueKindMatchInto::Port (value) => value.into (),
			ValueKindMatchInto::Process (value) => value.into (),
			
			ValueKindMatchInto::Context (value) => value.into (),
			ValueKindMatchInto::Binding (value) => value.into (),
			
		}
		
	}
}


impl ValueClassMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			
			ValueClassMatchInto::Null => NULL.into (),
			ValueClassMatchInto::Void => VOID.into (),
			ValueClassMatchInto::Undefined => UNDEFINED.into (),
			ValueClassMatchInto::Singleton (value) => value.into (),
			
			ValueClassMatchInto::Boolean (value) => value.into (),
			ValueClassMatchInto::Number (class) => class.value (),
			ValueClassMatchInto::Character (value) => value.into (),
			
			ValueClassMatchInto::Symbol (value) => value.into (),
			ValueClassMatchInto::String (class) => class.value (),
			ValueClassMatchInto::Bytes (class) => class.value (),
			
			ValueClassMatchInto::Pair (class) => class.value (),
			ValueClassMatchInto::Array (class) => class.value (),
			ValueClassMatchInto::Values (value) => value.into (),
			
			ValueClassMatchInto::RecordKind (value) => value.into (),
			ValueClassMatchInto::Record (class) => class.value (),
			
			ValueClassMatchInto::Error (value) => value.into (),
			
			ValueClassMatchInto::Procedure (class) => class.value (),
			ValueClassMatchInto::Syntax (class) => class.value (),
			
			ValueClassMatchInto::Port (value) => value.into (),
			ValueClassMatchInto::Resource (class) => class.value (),
			
			ValueClassMatchInto::Internal (class) => class.value (),
			ValueClassMatchInto::Opaque (value) => value,
			
		}
		
	}
}


impl ProcedureMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ProcedureMatchInto::Primitive (value) => value.into (),
			ProcedureMatchInto::Extended (value) => value.into (),
			ProcedureMatchInto::Native (value) => value.into (),
			ProcedureMatchInto::Lambda (value) => value.into (),
		}
	}
}


impl SyntaxMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			SyntaxMatchInto::Primitive (value) => value.into (),
			SyntaxMatchInto::Extended (value) => value.into (),
			SyntaxMatchInto::Native (value) => value.into (),
			SyntaxMatchInto::Lambda (value) => value.into (),
		}
	}
}


impl ResourceMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ResourceMatchInto::Process (value) => value.into (),
		}
	}
}


impl InternalMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			InternalMatchInto::Binding (value) => value.into (),
			InternalMatchInto::Context (value) => value.into (),
		}
	}
}


impl ListMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ListMatchInto::Null => NULL.into (),
			ListMatchInto::PairImmutable (value) => value.into (),
			ListMatchInto::PairMutable (value) => value.into (),
			ListMatchInto::Value (value) => value,
		}
	}
}




#[ derive (Clone) ]
pub struct ValueMeta1 ( u8, u8, u8 );

pub const VALUE_META_1 : ValueMeta1 = ValueMeta1 (0, 0, 0);




#[ derive (Clone) ]
pub struct ValueMeta2 ( u8, u8, u8, u8 );

pub const VALUE_META_2 : ValueMeta2 = ValueMeta2 (0, 0, 0, 0);




#[ derive (Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
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
	pub fn new_embedded_immutable <U : 'static, Accessor> (value : U, accessor : Accessor) -> (ValueRef<'a>)
			where Accessor : FnOnce (&'a U) -> (&'a Value)
	{
		let value = StdRc::new (value);
		ValueRef::new_embedded_immutable_from_rc (value, accessor)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable <U : 'static, Accessor> (value : U, accessor : Accessor) -> (ValueRef<'a>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, Value>)
	{
		let value = StdRc::new (value);
		ValueRef::new_embedded_mutable_from_rc (value, accessor)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable_from_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (ValueRef<'a>)
			where Accessor : FnOnce (&'a U) -> (&'a Value)
	{
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		ValueRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable_from_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (ValueRef<'a>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, Value>)
	{
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
		GenericRef::new_embedded_immutable_from_rc (value, accessor)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable <U : 'static, Accessor> (value : U, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, T>)
	{
		let value = StdRc::new (value);
		GenericRef::new_embedded_mutable_from_rc (value, accessor)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_immutable_from_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (&'a T)
	{
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::ImmutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable_from_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, T>)
	{
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

