

use super::constants::exports::*;
use super::primitives::exports::*;
use super::values_booleans::exports::*;
use super::values_numbers::exports::*;
use super::values_pairs::exports::*;
use super::values_symbols::exports::*;
use super::errors::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_characters::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_strings::exports::*;

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
use super::values_bytes::exports::*;

#[ cfg ( feature = "vonuvoli_values_array" ) ]
use super::values_arrays::exports::*;

#[ cfg ( feature = "vonuvoli_values_values" ) ]
use super::values_values::exports::*;

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
use super::values_keywords::exports::*;

#[ cfg ( feature = "vonuvoli_values_unique" ) ]
use super::values_unique::exports::*;

#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
use super::values_opaque::exports::*;

#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
use super::contexts::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
use super::values_records::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_values_native" ) ]
use super::native_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
use super::native_syntaxes::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
use super::regularex::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
use super::ports::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
use super::paths::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
use super::processes::exports::*;

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
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	pub use super::{Promise};
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ValueKind {
	
	Null,
	Void,
	Undefined,
	Singleton,
	
	Boolean,
	NumberInteger,
	NumberReal,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character,
	
	Symbol,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword,
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex,
	
	PairImmutable,
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayImmutable,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayMutable,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind,
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordImmutable,
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordMutable,
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error,
	
	ProcedurePrimitive,
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtended,
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNative,
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambda,
	
	SyntaxPrimitive,
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	SyntaxExtended,
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	SyntaxNative,
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	SyntaxLambda,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process,
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context,
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter,
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise,
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque,
	
}


pub enum ValueKindMatchAsRef <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean),
	NumberInteger (&'a NumberInteger),
	NumberReal (&'a NumberReal),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (&'a Character),
	
	Symbol (&'a Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (&'a Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (&'a Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable (&'a StringImmutable),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable (&'a StringMutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable (&'a BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable (&'a BytesMutable),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (&'a StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (&'a BytesRegex),
	
	PairImmutable (&'a PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable (&'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayImmutable (&'a ArrayImmutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayMutable (&'a ArrayMutable),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (&'a Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (&'a RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordImmutable (&'a RecordImmutable),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordMutable (&'a RecordMutable),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (&'a Error),
	
	ProcedurePrimitive (&'a ProcedurePrimitive),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtended (&'a ProcedureExtended),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNative (&'a ProcedureNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambda (&'a ProcedureLambda),
	
	SyntaxPrimitive (&'a SyntaxPrimitive),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	SyntaxExtended (&'a SyntaxExtended),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	SyntaxNative (&'a SyntaxNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	SyntaxLambda (&'a SyntaxLambda),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (&'a Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (&'a Port),
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process (&'a Process),
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context (&'a Context),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding (&'a Binding),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters (&'a Parameters),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter (&'a Parameter),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise (&'a Promise),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (&'a Opaque),
	
}


pub enum ValueKindMatchInto {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (Boolean),
	NumberInteger (NumberInteger),
	NumberReal (NumberReal),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (Character),
	
	Symbol (Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable (StringImmutable),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable (StringMutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable (BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable (BytesMutable),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (BytesRegex),
	
	PairImmutable (PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable (PairMutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayImmutable (ArrayImmutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayMutable (ArrayMutable),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordImmutable (RecordImmutable),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordMutable (RecordMutable),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (Error),
	
	ProcedurePrimitive (ProcedurePrimitive),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtended (ProcedureExtended),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNative (ProcedureNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambda (ProcedureLambda),
	
	SyntaxPrimitive (SyntaxPrimitive),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	SyntaxExtended (SyntaxExtended),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	SyntaxNative (SyntaxNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	SyntaxLambda (SyntaxLambda),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (Port),
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process (Process),
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context (Context),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding (Binding),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters (Parameters),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter (Parameter),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise (Promise),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (Opaque),
	
}


pub enum ValueKindMatchAsRef2 <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean, &'a Boolean),
	NumberInteger (&'a NumberInteger, &'a NumberInteger),
	NumberReal (&'a NumberReal, &'a NumberReal),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (&'a Character, &'a Character),
	
	Symbol (&'a Symbol, &'a Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (&'a Keyword, &'a Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (&'a Unique, &'a Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable (&'a StringImmutable, &'a StringImmutable),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable (&'a StringMutable, &'a StringMutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable (&'a BytesImmutable, &'a BytesImmutable),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable (&'a BytesMutable, &'a BytesMutable),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (&'a StringRegex, &'a StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (&'a BytesRegex, &'a BytesRegex),
	
	PairImmutable (&'a PairImmutable, &'a PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable (&'a PairMutable, &'a PairMutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayImmutable (&'a ArrayImmutable, &'a ArrayImmutable),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayMutable (&'a ArrayMutable, &'a ArrayMutable),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (&'a Values, &'a Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (&'a RecordKind, &'a RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordImmutable (&'a RecordImmutable, &'a RecordImmutable),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordMutable (&'a RecordMutable, &'a RecordMutable),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (&'a Error, &'a Error),
	
	ProcedurePrimitive (&'a ProcedurePrimitive, &'a ProcedurePrimitive),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtended (&'a ProcedureExtended, &'a ProcedureExtended),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNative (&'a ProcedureNative, &'a ProcedureNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambda (&'a ProcedureLambda, &'a ProcedureLambda),
	
	SyntaxPrimitive (&'a SyntaxPrimitive, &'a SyntaxPrimitive),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	SyntaxExtended (&'a SyntaxExtended, &'a SyntaxExtended),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	SyntaxNative (&'a SyntaxNative, &'a SyntaxNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	SyntaxLambda (&'a SyntaxLambda, &'a SyntaxLambda),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (&'a Path, &'a Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (&'a Port, &'a Port),
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process (&'a Process, &'a Process),
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context (&'a Context, &'a Context),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding (&'a Binding, &'a Binding),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters (&'a Parameters, &'a Parameters),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter (&'a Parameter, &'a Parameter),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise (&'a Promise, &'a Promise),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (&'a Opaque, &'a Opaque),
	
	Missmatched,
	
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	Singleton,
	
	Boolean,
	Number,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character,
	
	Symbol,
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword,
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes,
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex,
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex,
	
	Pair,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values,
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind,
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record,
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error,
	
	Procedure,
	Syntax,
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port,
	Resource,
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque,
	
	Internal,
	
}


pub enum ValueClassMatchAsRef <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean),
	Number (NumberMatchAsRef<'a>),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (&'a Character),
	
	Symbol (&'a Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (&'a Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (&'a Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String (StringMatchAsRef<'a>),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes (BytesMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (&'a StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (&'a BytesRegex),
	
	Pair (PairMatchAsRef<'a>),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array (ArrayMatchAsRef<'a>),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (&'a Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (&'a RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record (RecordMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (&'a Error),
	
	Procedure (ProcedureMatchAsRef<'a>),
	Syntax (SyntaxMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (&'a Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (&'a Port),
	Resource (ResourceMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (&'a Opaque),
	
	Internal (InternalMatchAsRef<'a>),
	
}


pub enum ValueClassMatchInto {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (Boolean),
	Number (NumberMatchInto),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (Character),
	
	Symbol (Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String (StringMatchInto),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes (BytesMatchInto),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (BytesRegex),
	
	Pair (PairMatchInto),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array (ArrayMatchInto),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record (RecordMatchInto),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (Error),
	
	Procedure (ProcedureMatchInto),
	Syntax (SyntaxMatchInto),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (Port),
	Resource (ResourceMatchInto),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (Opaque),
	
	Internal (InternalMatchInto),
	
}


pub enum ValueClassMatchAsRef2 <'a> {
	
	Null,
	Void,
	Undefined,
	Singleton (ValueSingleton),
	
	Boolean (&'a Boolean, &'a Boolean),
	Number (NumberMatchAsRef2<'a>),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (&'a Character, &'a Character),
	
	Symbol (&'a Symbol, &'a Symbol),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (&'a Keyword, &'a Keyword),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique (&'a Unique, &'a Unique),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String (StringMatchAsRef2<'a>),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	Bytes (BytesMatchAsRef2<'a>),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex (&'a StringRegex, &'a StringRegex),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex (&'a BytesRegex, &'a BytesRegex),
	
	Pair (PairMatchAsRef2<'a>),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array (ArrayMatchAsRef2<'a>),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (&'a Values, &'a Values),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind (&'a RecordKind, &'a RecordKind),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	Record (RecordMatchAsRef2<'a>),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error (&'a Error, &'a Error),
	
	Procedure (ProcedureMatchAsRef<'a>, ProcedureMatchAsRef<'a>),
	Syntax (SyntaxMatchAsRef<'a>, SyntaxMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path (&'a Path, &'a Path),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port (&'a Port, &'a Port),
	Resource (ResourceMatchAsRef<'a>, ResourceMatchAsRef<'a>),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque (&'a Opaque, &'a Opaque),
	
	Internal (InternalMatchAsRef<'a>, InternalMatchAsRef<'a>),
	
	Missmatched (ValueClassMatchAsRef<'a>, ValueClassMatchAsRef<'a>),
	
}




pub enum ProcedureMatchAsRef <'a> {
	Primitive (&'a ProcedurePrimitive),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Extended (&'a ProcedureExtended),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	Native (&'a ProcedureNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda (&'a ProcedureLambda),
}


pub enum ProcedureMatchInto {
	Primitive (ProcedurePrimitive),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Extended (ProcedureExtended),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	Native (ProcedureNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda (ProcedureLambda),
}


pub enum SyntaxMatchAsRef <'a> {
	Primitive (&'a SyntaxPrimitive),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Extended (&'a SyntaxExtended),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	Native (&'a SyntaxNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda (&'a SyntaxLambda),
}


pub enum SyntaxMatchInto {
	Primitive (SyntaxPrimitive),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	Extended (SyntaxExtended),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	Native (SyntaxNative),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda (SyntaxLambda),
}


pub enum ResourceMatchAsRef <'a> {
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process (&'a Process),
	Other (&'a Value),   // NOTE:  this should never be used!
}


pub enum ResourceMatchInto {
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process (Process),
	Other (Value),  // NOTE:  this should never be used!
}


pub enum InternalMatchAsRef <'a> {
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context (&'a Context),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding (&'a Binding),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters (&'a Parameters),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter (&'a Parameter),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise (&'a Promise),
	Other (&'a Value),  // NOTE:  this should never be used!
}


pub enum InternalMatchInto {
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context (Context),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding (Binding),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters (Parameters),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter (Parameter),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise (Promise),
	Other (Value),  // NOTE:  this should never be used!
}


pub enum ListMatchAsRef <'a> {
	Null,
	PairImmutable (&'a PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable (&'a PairMutable),
	Value (&'a Value),
}


pub enum ListMatchInto {
	Null,
	PairImmutable (PairImmutable),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable (PairMutable),
	Value (Value),
}




#[ derive ( Clone ) ] // OK
pub enum Value {
	
	Singleton ( ValueMeta1, ValueSingleton, ValueMeta2 ),
	
	Boolean ( ValueMeta1, Boolean, ValueMeta2 ),
	NumberInteger ( ValueMeta1, NumberInteger, ValueMeta2 ),
	NumberReal ( ValueMeta1, NumberReal, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character ( ValueMeta1, Character, ValueMeta2 ),
	
	Symbol ( ValueMeta1, Symbol, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword ( ValueMeta1, Keyword, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_unique" ) ]
	Unique ( ValueMeta1, Unique, ValueMeta2 ),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable ( ValueMeta1, StringImmutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable ( ValueMeta1, StringMutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable ( ValueMeta1, BytesImmutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable ( ValueMeta1, BytesMutable, ValueMeta2 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringRegex ( ValueMeta1, StringRegex, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesRegex ( ValueMeta1, BytesRegex, ValueMeta2 ),
	
	PairImmutable ( ValueMeta1, PairImmutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	PairMutable ( ValueMeta1, PairMutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ArrayImmutable ( ValueMeta1, ArrayImmutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	ArrayMutable ( ValueMeta1, ArrayMutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values ( ValueMeta1, Values, ValueMeta2 ),
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordKind ( ValueMeta1, RecordKind, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	RecordImmutable ( ValueMeta1, RecordImmutable, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	RecordMutable ( ValueMeta1, RecordMutable, ValueMeta2 ),
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Error ( ValueMeta1, Error, ValueMeta2 ),
	
	ProcedurePrimitive ( ValueMeta1, ProcedurePrimitive, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	ProcedureExtended ( ValueMeta1, ProcedureExtended, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	ProcedureNative ( ValueMeta1, ProcedureNative, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	ProcedureLambda ( ValueMeta1, ProcedureLambda, ValueMeta2 ),
	
	SyntaxPrimitive ( ValueMeta1, SyntaxPrimitive, ValueMeta2, ),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	SyntaxExtended ( ValueMeta1, SyntaxExtended, ValueMeta2, ),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	SyntaxNative ( ValueMeta1, SyntaxNative, ValueMeta2, ),
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	SyntaxLambda ( ValueMeta1, SyntaxLambda, ValueMeta2, ),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	Path ( ValueMeta1, Path, ValueMeta2, ),
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	Port ( ValueMeta1, Port, ValueMeta2, ),
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	Process ( ValueMeta1, Process, ValueMeta2, ),
	
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Context ( ValueMeta1, Context, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
	Binding ( ValueMeta1, Binding, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameters ( ValueMeta1, Parameters, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	Parameter ( ValueMeta1, Parameter, ValueMeta2 ),
	#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
	Promise ( ValueMeta1, Promise, ValueMeta2 ),
	
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	Opaque ( ValueMeta1, Opaque, ValueMeta2 ),
	
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueKind::Singleton,
				},
			
			Value::Boolean (_, _, _) => ValueKind::Boolean,
			Value::NumberInteger (_, _, _) => ValueKind::NumberInteger,
			Value::NumberReal (_, _, _) => ValueKind::NumberReal,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, _, _) => ValueKind::Character,
			
			Value::Symbol (_, _, _) => ValueKind::Symbol,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, _, _) => ValueKind::Keyword,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, _, _) => ValueKind::Unique,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, _, _) => ValueKind::StringImmutable,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, _, _) => ValueKind::StringMutable,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, _, _) => ValueKind::BytesImmutable,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, _, _) => ValueKind::BytesMutable,
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, _, _) => ValueKind::StringRegex,
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, _, _) => ValueKind::BytesRegex,
			
			Value::PairImmutable (_, _, _) => ValueKind::PairImmutable,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, _, _) => ValueKind::PairMutable,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, _, _) => ValueKind::ArrayImmutable,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, _, _) => ValueKind::ArrayMutable,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, _, _) => ValueKind::Values,
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, _, _) => ValueKind::RecordKind,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, _, _) => ValueKind::RecordImmutable,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, _, _) => ValueKind::RecordMutable,
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, _, _) => ValueKind::Error,
			
			Value::ProcedurePrimitive (_, _, _) => ValueKind::ProcedurePrimitive,
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, _, _) => ValueKind::ProcedureExtended,
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, _, _) => ValueKind::ProcedureNative,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, _, _) => ValueKind::ProcedureLambda,
			
			Value::SyntaxPrimitive (_, _, _) => ValueKind::SyntaxPrimitive,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, _, _) => ValueKind::SyntaxExtended,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, _, _) => ValueKind::SyntaxNative,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, _, _) => ValueKind::SyntaxLambda,
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, _, _) => ValueKind::Path,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, _, _) => ValueKind::Port,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, _, _) => ValueKind::Process,
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, _, _) => ValueKind::Context,
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, _, _) => ValueKind::Binding,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, _, _) => ValueKind::Parameters,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, _, _) => ValueKind::Parameter,
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, _, _) => ValueKind::Promise,
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, _, _) => ValueKind::Opaque,
			
			Value::__NonExhaustive => unreachable_0! (0x7bbc0f95, github_issue_new),
			
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueKindMatchAsRef::Singleton (*self_0),
				},
			
			Value::Boolean (_, ref self_0, _) => ValueKindMatchAsRef::Boolean (self_0),
			Value::NumberInteger (_, ref self_0, _) => ValueKindMatchAsRef::NumberInteger (self_0),
			Value::NumberReal (_, ref self_0, _) => ValueKindMatchAsRef::NumberReal (self_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, ref self_0, _) => ValueKindMatchAsRef::Character (self_0),
			
			Value::Symbol (_, ref self_0, _) => ValueKindMatchAsRef::Symbol (self_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, ref self_0, _) => ValueKindMatchAsRef::Keyword (self_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, ref self_0, _) => ValueKindMatchAsRef::Unique (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, ref self_0, _) => ValueKindMatchAsRef::StringImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, ref self_0, _) => ValueKindMatchAsRef::StringMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, ref self_0, _) => ValueKindMatchAsRef::BytesImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, ref self_0, _) => ValueKindMatchAsRef::BytesMutable (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, ref self_0, _) => ValueKindMatchAsRef::StringRegex (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, ref self_0, _) => ValueKindMatchAsRef::BytesRegex (self_0),
			
			Value::PairImmutable (_, ref self_0, _) => ValueKindMatchAsRef::PairImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, ref self_0, _) => ValueKindMatchAsRef::PairMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, ref self_0, _) => ValueKindMatchAsRef::ArrayImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, ref self_0, _) => ValueKindMatchAsRef::ArrayMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, ref self_0, _) => ValueKindMatchAsRef::Values (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, ref self_0, _) => ValueKindMatchAsRef::RecordKind (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, ref self_0, _) => ValueKindMatchAsRef::RecordImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, ref self_0, _) => ValueKindMatchAsRef::RecordMutable (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, ref self_0, _) => ValueKindMatchAsRef::Error (self_0),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => ValueKindMatchAsRef::ProcedurePrimitive (self_0),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureExtended (self_0),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureNative (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, ref self_0, _) => ValueKindMatchAsRef::ProcedureLambda (self_0),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxPrimitive (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxExtended (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxNative (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, ref self_0, _) => ValueKindMatchAsRef::SyntaxLambda (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, ref self_0, _) => ValueKindMatchAsRef::Path (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, ref self_0, _) => ValueKindMatchAsRef::Port (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, ref self_0, _) => ValueKindMatchAsRef::Process (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, ref self_0, _) => ValueKindMatchAsRef::Context (self_0),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, ref self_0, _) => ValueKindMatchAsRef::Binding (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, ref self_0, _) => ValueKindMatchAsRef::Parameters (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, ref self_0, _) => ValueKindMatchAsRef::Parameter (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, ref self_0, _) => ValueKindMatchAsRef::Promise (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, ref self_0, _) => ValueKindMatchAsRef::Opaque (self_0),
			
			Value::__NonExhaustive => unreachable_0! (0x60a44540, github_issue_new),
			
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueKindMatchInto::Singleton (self_0),
				},
			
			Value::Boolean (_, self_0, _) => ValueKindMatchInto::Boolean (self_0),
			Value::NumberInteger (_, self_0, _) => ValueKindMatchInto::NumberInteger (self_0),
			Value::NumberReal (_, self_0, _) => ValueKindMatchInto::NumberReal (self_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, self_0, _) => ValueKindMatchInto::Character (self_0),
			
			Value::Symbol (_, self_0, _) => ValueKindMatchInto::Symbol (self_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, self_0, _) => ValueKindMatchInto::Keyword (self_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, self_0, _) => ValueKindMatchInto::Unique (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, self_0, _) => ValueKindMatchInto::StringImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, self_0, _) => ValueKindMatchInto::StringMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, self_0, _) => ValueKindMatchInto::BytesImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, self_0, _) => ValueKindMatchInto::BytesMutable (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, self_0, _) => ValueKindMatchInto::StringRegex (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, self_0, _) => ValueKindMatchInto::BytesRegex (self_0),
			
			Value::PairImmutable (_, self_0, _) => ValueKindMatchInto::PairImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, self_0, _) => ValueKindMatchInto::PairMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, self_0, _) => ValueKindMatchInto::ArrayImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, self_0, _) => ValueKindMatchInto::ArrayMutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, self_0, _) => ValueKindMatchInto::Values (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, self_0, _) => ValueKindMatchInto::RecordKind (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, self_0, _) => ValueKindMatchInto::RecordImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, self_0, _) => ValueKindMatchInto::RecordMutable (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, self_0, _) => ValueKindMatchInto::Error (self_0),
			
			Value::ProcedurePrimitive (_, self_0, _) => ValueKindMatchInto::ProcedurePrimitive (self_0),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, self_0, _) => ValueKindMatchInto::ProcedureExtended (self_0),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, self_0, _) => ValueKindMatchInto::ProcedureNative (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, self_0, _) => ValueKindMatchInto::ProcedureLambda (self_0),
			
			Value::SyntaxPrimitive (_, self_0, _) => ValueKindMatchInto::SyntaxPrimitive (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, self_0, _) => ValueKindMatchInto::SyntaxExtended (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, self_0, _) => ValueKindMatchInto::SyntaxNative (self_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, self_0, _) => ValueKindMatchInto::SyntaxLambda (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, self_0, _) => ValueKindMatchInto::Path (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, self_0, _) => ValueKindMatchInto::Port (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, self_0, _) => ValueKindMatchInto::Process (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, self_0, _) => ValueKindMatchInto::Context (self_0),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, self_0, _) => ValueKindMatchInto::Binding (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, self_0, _) => ValueKindMatchInto::Parameters (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, self_0, _) => ValueKindMatchInto::Parameter (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, self_0, _) => ValueKindMatchInto::Promise (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, self_0, _) => ValueKindMatchInto::Opaque (self_0),
			
			Value::__NonExhaustive => unreachable_0! (0x91f4d229, github_issue_new),
			
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
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => ValueKindMatchAsRef2::Character (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => ValueKindMatchAsRef2::Symbol (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			(&Value::Keyword (_, ref self_0, _), &Value::Keyword (_, ref other_0, _)) => ValueKindMatchAsRef2::Keyword (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			(&Value::Unique (_, ref self_0, _), &Value::Unique (_, ref other_0, _)) => ValueKindMatchAsRef2::Unique (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::StringImmutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::StringMutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::StringMutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::BytesImmutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::BytesMutable (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::StringRegex (_, ref self_0, _), &Value::StringRegex (_, ref other_0, _)) => ValueKindMatchAsRef2::StringRegex (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			(&Value::BytesRegex (_, ref self_0, _), &Value::BytesRegex (_, ref other_0, _)) => ValueKindMatchAsRef2::BytesRegex (self_0, other_0),
			
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::PairImmutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::PairMutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::PairMutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::ArrayMutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => ValueKindMatchAsRef2::Values (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			(&Value::RecordKind (_, ref self_0, _), &Value::RecordKind (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordKind (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordImmutable (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueKindMatchAsRef2::RecordMutable (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => ValueKindMatchAsRef2::Error (self_0, other_0),
			
			(&Value::ProcedurePrimitive (_, ref self_0, _), &Value::ProcedurePrimitive (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			(&Value::ProcedureExtended (_, ref self_0, _), &Value::ProcedureExtended (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			(&Value::ProcedureNative (_, ref self_0, _), &Value::ProcedureNative (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureNative (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			(&Value::ProcedureLambda (_, ref self_0, _), &Value::ProcedureLambda (_, ref other_0, _)) => ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0),
			
			(&Value::SyntaxPrimitive (_, ref self_0, _), &Value::SyntaxPrimitive (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			(&Value::SyntaxExtended (_, ref self_0, _), &Value::SyntaxExtended (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			(&Value::SyntaxNative (_, ref self_0, _), &Value::SyntaxNative (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxNative (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			(&Value::SyntaxLambda (_, ref self_0, _), &Value::SyntaxLambda (_, ref other_0, _)) => ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			(&Value::Path (_, ref self_0, _), &Value::Path (_, ref other_0, _)) => ValueKindMatchAsRef2::Path (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => ValueKindMatchAsRef2::Port (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			(&Value::Process (_, ref self_0, _), &Value::Process (_, ref other_0, _)) => ValueKindMatchAsRef2::Process (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			(&Value::Context (_, ref self_0, _), &Value::Context (_, ref other_0, _)) => ValueKindMatchAsRef2::Context (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			(&Value::Binding (_, ref self_0, _), &Value::Binding (_, ref other_0, _)) => ValueKindMatchAsRef2::Binding (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			(&Value::Parameters (_, ref self_0, _), &Value::Parameters (_, ref other_0, _)) => ValueKindMatchAsRef2::Parameters (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			(&Value::Parameter (_, ref self_0, _), &Value::Parameter (_, ref other_0, _)) => ValueKindMatchAsRef2::Parameter (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			(&Value::Promise (_, ref self_0, _), &Value::Promise (_, ref other_0, _)) => ValueKindMatchAsRef2::Promise (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			(&Value::Opaque (_, ref self_0, _), &Value::Opaque (_, ref other_0, _)) => ValueKindMatchAsRef2::Opaque (self_0, other_0),
			
			(&Value::__NonExhaustive, _) => unreachable_0! (0x13867aa3, github_issue_new),
			(_, &Value::__NonExhaustive) => unreachable_0! (0x5285f71e, github_issue_new),
			
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueClass::Singleton,
				},
			
			Value::Boolean (_, _, _) => ValueClass::Boolean,
			Value::NumberInteger (_, _, _) => ValueClass::Number,
			Value::NumberReal (_, _, _) => ValueClass::Number,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, _, _) => ValueClass::Character,
			
			Value::Symbol (_, _, _) => ValueClass::Symbol,
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, _, _) => ValueClass::Keyword,
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, _, _) => ValueClass::Unique,
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, _, _) => ValueClass::String,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, _, _) => ValueClass::String,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, _, _) => ValueClass::Bytes,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, _, _) => ValueClass::Bytes,
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, _, _) => ValueClass::StringRegex,
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, _, _) => ValueClass::BytesRegex,
			
			Value::PairImmutable (_, _, _) => ValueClass::Pair,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, _, _) => ValueClass::Pair,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, _, _) => ValueClass::Array,
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, _, _) => ValueClass::Array,
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, _, _) => ValueClass::Values,
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, _, _) => ValueClass::RecordKind,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, _, _) => ValueClass::Record,
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, _, _) => ValueClass::Record,
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, _, _) => ValueClass::Error,
			
			Value::ProcedurePrimitive (_, _, _) => ValueClass::Procedure,
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, _, _) => ValueClass::Procedure,
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, _, _) => ValueClass::Procedure,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, _, _) => ValueClass::Procedure,
			
			Value::SyntaxPrimitive (_, _, _) => ValueClass::Syntax,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, _, _) => ValueClass::Syntax,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, _, _) => ValueClass::Syntax,
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, _, _) => ValueClass::Syntax,
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, _, _) => ValueClass::Path,
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, _, _) => ValueClass::Port,
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, _, _) => ValueClass::Resource,
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, _, _) => ValueClass::Internal,
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, _, _) => ValueClass::Internal,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, _, _) => ValueClass::Internal,
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, _, _) => ValueClass::Internal,
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, _, _) => ValueClass::Internal,
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, _, _) => ValueClass::Opaque,
			
			Value::__NonExhaustive => unreachable_0! (0x5f4a0853, github_issue_new),
			
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueClassMatchAsRef::Singleton (*self_0),
				},
			
			Value::Boolean (_, ref self_0, _) => ValueClassMatchAsRef::Boolean (self_0),
			Value::NumberInteger (_, ref self_0, _) => ValueClassMatchAsRef::Number (NumberMatchAsRef::Integer (self_0)),
			Value::NumberReal (_, ref self_0, _) => ValueClassMatchAsRef::Number (NumberMatchAsRef::Real (self_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, ref self_0, _) => ValueClassMatchAsRef::Character (self_0),
			
			Value::Symbol (_, ref self_0, _) => ValueClassMatchAsRef::Symbol (self_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, ref self_0, _) => ValueClassMatchAsRef::Keyword (self_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, ref self_0, _) => ValueClassMatchAsRef::Unique (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, ref self_0, _) => ValueClassMatchAsRef::String (StringMatchAsRef::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, ref self_0, _) => ValueClassMatchAsRef::String (StringMatchAsRef::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Bytes (BytesMatchAsRef::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, ref self_0, _) => ValueClassMatchAsRef::Bytes (BytesMatchAsRef::Mutable (self_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, ref self_0, _) => ValueClassMatchAsRef::StringRegex (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, ref self_0, _) => ValueClassMatchAsRef::BytesRegex (self_0),
			
			Value::PairImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Pair (PairMatchAsRef::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, ref self_0, _) => ValueClassMatchAsRef::Pair (PairMatchAsRef::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Array (ArrayMatchAsRef::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, ref self_0, _) => ValueClassMatchAsRef::Array (ArrayMatchAsRef::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, ref self_0, _) => ValueClassMatchAsRef::Values (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, ref self_0, _) => ValueClassMatchAsRef::RecordKind (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, ref self_0, _) => ValueClassMatchAsRef::Record (RecordMatchAsRef::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, ref self_0, _) => ValueClassMatchAsRef::Record (RecordMatchAsRef::Mutable (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, ref self_0, _) => ValueClassMatchAsRef::Error (self_0),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Primitive (self_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Extended (self_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Native (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, ref self_0, _) => ValueClassMatchAsRef::Procedure (ProcedureMatchAsRef::Lambda (self_0)),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Primitive (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Extended (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Native (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, ref self_0, _) => ValueClassMatchAsRef::Syntax (SyntaxMatchAsRef::Lambda (self_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, ref self_0, _) => ValueClassMatchAsRef::Path (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, ref self_0, _) => ValueClassMatchAsRef::Port (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, ref self_0, _) => ValueClassMatchAsRef::Resource (ResourceMatchAsRef::Process (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Context (self_0)),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Binding (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Parameters (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Parameter (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, ref self_0, _) => ValueClassMatchAsRef::Internal (InternalMatchAsRef::Promise (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, ref self_0, _) => ValueClassMatchAsRef::Opaque (self_0),
			
			Value::__NonExhaustive => unreachable_0! (0xeb981b3d, github_issue_new),
			
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
					#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
					ValueSingleton::PortEof => ValueClassMatchInto::Singleton (self_0),
				},
			
			Value::Boolean (_, self_0, _) => ValueClassMatchInto::Boolean (self_0),
			Value::NumberInteger (_, self_0, _) => ValueClassMatchInto::Number (NumberMatchInto::Integer (self_0)),
			Value::NumberReal (_, self_0, _) => ValueClassMatchInto::Number (NumberMatchInto::Real (self_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, self_0, _) => ValueClassMatchInto::Character (self_0),
			
			Value::Symbol (_, self_0, _) => ValueClassMatchInto::Symbol (self_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, self_0, _) => ValueClassMatchInto::Keyword (self_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, self_0, _) => ValueClassMatchInto::Unique (self_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, self_0, _) => ValueClassMatchInto::String (StringMatchInto::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::StringMutable (_, self_0, _) => ValueClassMatchInto::String (StringMatchInto::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, self_0, _) => ValueClassMatchInto::Bytes (BytesMatchInto::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::BytesMutable (_, self_0, _) => ValueClassMatchInto::Bytes (BytesMatchInto::Mutable (self_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, self_0, _) => ValueClassMatchInto::StringRegex (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, self_0, _) => ValueClassMatchInto::BytesRegex (self_0),
			
			Value::PairImmutable (_, self_0, _) => ValueClassMatchInto::Pair (PairMatchInto::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, self_0, _) => ValueClassMatchInto::Pair (PairMatchInto::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, self_0, _) => ValueClassMatchInto::Array (ArrayMatchInto::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::ArrayMutable (_, self_0, _) => ValueClassMatchInto::Array (ArrayMatchInto::Mutable (self_0)),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, self_0, _) => ValueClassMatchInto::Values (self_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, self_0, _) => ValueClassMatchInto::RecordKind (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, self_0, _) => ValueClassMatchInto::Record (RecordMatchInto::Immutable (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::RecordMutable (_, self_0, _) => ValueClassMatchInto::Record (RecordMatchInto::Mutable (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, self_0, _) => ValueClassMatchInto::Error (self_0),
			
			Value::ProcedurePrimitive (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Primitive (self_0)),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Extended (self_0)),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Native (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, self_0, _) => ValueClassMatchInto::Procedure (ProcedureMatchInto::Lambda (self_0)),
			
			Value::SyntaxPrimitive (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Primitive (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Extended (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Native (self_0)),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, self_0, _) => ValueClassMatchInto::Syntax (SyntaxMatchInto::Lambda (self_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, self_0, _) => ValueClassMatchInto::Path (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, self_0, _) => ValueClassMatchInto::Port (self_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, self_0, _) => ValueClassMatchInto::Resource (ResourceMatchInto::Process (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Context (self_0)),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Binding (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Parameters (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Parameter (self_0)),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, self_0, _) => ValueClassMatchInto::Internal (InternalMatchInto::Promise (self_0)),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, self_0, _) => ValueClassMatchInto::Opaque (self_0),
			
			Value::__NonExhaustive => unreachable_0! (0xcb4d88e4, github_issue_new),
			
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::Character (_, ref self_0, _), &Value::Character (_, ref other_0, _)) => ValueClassMatchAsRef2::Character (self_0, other_0),
			
			(&Value::Symbol (_, ref self_0, _), &Value::Symbol (_, ref other_0, _)) => ValueClassMatchAsRef2::Symbol (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			(&Value::Keyword (_, ref self_0, _), &Value::Keyword (_, ref other_0, _)) => ValueClassMatchAsRef2::Keyword (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			(&Value::Unique (_, ref self_0, _), &Value::Unique (_, ref other_0, _)) => ValueClassMatchAsRef2::Unique (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::ImmutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::StringMutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::MutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::StringImmutable (_, ref self_0, _), &Value::StringMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::StringMutable (_, ref self_0, _), &Value::StringImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::String (StringMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::ImmutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::MutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::BytesImmutable (_, ref self_0, _), &Value::BytesMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::BytesMutable (_, ref self_0, _), &Value::BytesImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Bytes (BytesMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			(&Value::StringRegex (_, ref self_0, _), &Value::StringRegex (_, ref other_0, _)) => ValueClassMatchAsRef2::StringRegex (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			(&Value::BytesRegex (_, ref self_0, _), &Value::BytesRegex (_, ref other_0, _)) => ValueClassMatchAsRef2::BytesRegex (self_0, other_0),
			
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::ImmutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::PairMutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::MutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::PairImmutable (_, ref self_0, _), &Value::PairMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::PairMutable (_, ref self_0, _), &Value::PairImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Pair (PairMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::ImmutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::MutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::ArrayImmutable (_, ref self_0, _), &Value::ArrayMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::ArrayMutable (_, ref self_0, _), &Value::ArrayImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Array (ArrayMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			(&Value::Values (_, ref self_0, _), &Value::Values (_, ref other_0, _)) => ValueClassMatchAsRef2::Values (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			(&Value::RecordKind (_, ref self_0, _), &Value::RecordKind (_, ref other_0, _)) => ValueClassMatchAsRef2::RecordKind (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::ImmutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::MutableBoth (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::RecordImmutable (_, ref self_0, _), &Value::RecordMutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::ImmutableAndMutable (self_0, other_0)),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			(&Value::RecordMutable (_, ref self_0, _), &Value::RecordImmutable (_, ref other_0, _)) => ValueClassMatchAsRef2::Record (RecordMatchAsRef2::MutableAndImmutable (self_0, other_0)),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			(&Value::Error (_, ref self_0, _), &Value::Error (_, ref other_0, _)) => ValueClassMatchAsRef2::Error (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			(&Value::Path (_, ref self_0, _), &Value::Path (_, ref other_0, _)) => ValueClassMatchAsRef2::Path (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			(&Value::Port (_, ref self_0, _), &Value::Port (_, ref other_0, _)) => ValueClassMatchAsRef2::Port (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			(&Value::Opaque (_, ref self_0, _), &Value::Opaque (_, ref other_0, _)) => ValueClassMatchAsRef2::Opaque (self_0, other_0),
			
			(&Value::__NonExhaustive, _) => unreachable_0! (0x15e280a3, github_issue_new),
			(_, &Value::__NonExhaustive) => unreachable_0! (0xf9cc335c, github_issue_new),
			
			// NOTE:  !!! match-fallback !!!
			_ =>
				match (this.class_match_as_ref (), other.class_match_as_ref ()) {
					
					(ValueClassMatchAsRef::Procedure (self_0), ValueClassMatchAsRef::Procedure (other_0)) =>
						ValueClassMatchAsRef2::Procedure (self_0, other_0),
					(ValueClassMatchAsRef::Syntax (self_0), ValueClassMatchAsRef::Syntax (other_0)) =>
						ValueClassMatchAsRef2::Syntax (self_0, other_0),
					(ValueClassMatchAsRef::Resource (self_0), ValueClassMatchAsRef::Resource (other_0)) =>
						ValueClassMatchAsRef2::Resource (self_0, other_0),
					
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, ref self_0, _) => ListMatchAsRef::PairMutable (self_0),
			Value::__NonExhaustive => unreachable_0! (0x2d521611, github_issue_new),
			// NOTE:  !!! match-fallback !!!
			_ => ListMatchAsRef::Value (self),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn list_match_into (self) -> (ListMatchInto) {
		match self {
			Value::Singleton (_, ValueSingleton::Null, _) => ListMatchInto::Null,
			Value::PairImmutable (_, self_0, _) => ListMatchInto::PairImmutable (self_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			Value::PairMutable (_, self_0, _) => ListMatchInto::PairMutable (self_0),
			Value::__NonExhaustive => unreachable_0! (0xa32b87fa, github_issue_new),
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
			
			ValueKindMatchAsRef2::Boolean (self_0, other_0) => Boolean::is_self (self_0, other_0),
			ValueKindMatchAsRef2::NumberInteger (self_0, other_0) => NumberInteger::is_self (self_0, other_0),
			ValueKindMatchAsRef2::NumberReal (self_0, other_0) => NumberReal::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::Character (self_0, other_0) => Character::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Symbol (self_0, other_0) => Symbol::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchAsRef2::Keyword (self_0, other_0) => Keyword::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchAsRef2::Unique (self_0, other_0) => Unique::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringImmutable (self_0, other_0) => StringImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef2::StringMutable (self_0, other_0) => StringMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesImmutable (self_0, other_0) => BytesImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef2::BytesMutable (self_0, other_0) => BytesMutable::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchAsRef2::StringRegex (self_0, other_0) => StringRegex::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchAsRef2::BytesRegex (self_0, other_0) => BytesRegex::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::PairImmutable (self_0, other_0) => PairImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef2::PairMutable (self_0, other_0) => PairMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchAsRef2::ArrayImmutable (self_0, other_0) => ArrayImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef2::ArrayMutable (self_0, other_0) => ArrayMutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchAsRef2::Values (self_0, other_0) => Values::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordKind (self_0, other_0) => RecordKind::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchAsRef2::RecordImmutable (self_0, other_0) => RecordImmutable::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchAsRef2::RecordMutable (self_0, other_0) => RecordMutable::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchAsRef2::Error (self_0, other_0) => Error::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::ProcedurePrimitive (self_0, other_0) => ProcedurePrimitive::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::ProcedureExtended (self_0, other_0) => ProcedureExtended::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::ProcedureNative (self_0, other_0) => ProcedureNative::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::ProcedureLambda (self_0, other_0) => ProcedureLambda::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::SyntaxPrimitive (self_0, other_0) => SyntaxPrimitive::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef2::SyntaxExtended (self_0, other_0) => SyntaxExtended::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef2::SyntaxNative (self_0, other_0) => SyntaxNative::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef2::SyntaxLambda (self_0, other_0) => SyntaxLambda::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchAsRef2::Path (self_0, other_0) => Path::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchAsRef2::Port (self_0, other_0) => Port::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchAsRef2::Process (self_0, other_0) => Process::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Context (self_0, other_0) => Context::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchAsRef2::Binding (self_0, other_0) => Binding::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameters (self_0, other_0) => Parameters::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef2::Parameter (self_0, other_0) => Parameter::is_self (self_0, other_0),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchAsRef2::Promise (self_0, other_0) => Promise::is_self (self_0, other_0),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchAsRef2::Opaque (self_0, other_0) => Opaque::is_self (self_0, other_0),
			
			ValueKindMatchAsRef2::Missmatched => false,
			
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_immutable (&self) -> (Outcome<Value>) {
		match *self {
			
			Value::Singleton (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Boolean (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::NumberInteger (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::NumberReal (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::Character (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::Symbol (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			Value::Keyword (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			Value::Unique (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringRegex (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesRegex (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			Value::PairMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			Value::Values (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordKind (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordMutable (_, ref self_0, _) => self_0.to_immutable () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Value::Error (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::ProcedurePrimitive (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::ProcedureExtended (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::ProcedureNative (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::ProcedureLambda (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::SyntaxPrimitive (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Value::SyntaxExtended (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Value::SyntaxNative (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Value::SyntaxLambda (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			Value::Path (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, _, _) => fail! (0xe4de734c),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, _, _) => fail! (0x629f6149),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Context (_, _, _) => fail! (0x7e3a414d),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			Value::Binding (_, _, _) => fail! (0xcf5a0e0d),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameters (_, _, _) => fail! (0xf71687af),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			Value::Parameter (_, _, _) => fail! (0x5e58cbae),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			Value::Promise (_, _, _) => fail! (0xdb79854e),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, _, _) => fail! (0x3a5f06fc),
			
			Value::__NonExhaustive => unreachable_0! (0xe6a3ce23, github_issue_new),
			
		}
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn to_mutable (&self) -> (Outcome<Value>) {
		match *self {
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			Value::StringMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			Value::BytesMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			Value::PairImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			Value::PairMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			Value::ArrayMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordImmutable (_, ref self_0, _) => self_0.to_mutable () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			Value::RecordMutable (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			Value::Port (_, ref self_0, _) => self_0.clone () .into_0 (),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			Value::Process (_, ref self_0, _) => self_0.clone () .into_0 (),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			Value::Opaque (_, _, _) => fail! (0x1f7ae54b),
			
			Value::__NonExhaustive => unreachable_0! (0xdbf88c4a, github_issue_new),
			
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
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchInto::Character (value) => value.into (),
			
			ValueKindMatchInto::Symbol (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueKindMatchInto::Keyword (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueKindMatchInto::Unique (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchInto::StringImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::StringMutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchInto::BytesImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::BytesMutable (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueKindMatchInto::StringRegex (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueKindMatchInto::BytesRegex (value) => value.into (),
			
			ValueKindMatchInto::PairImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::PairMutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueKindMatchInto::ArrayImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::ArrayMutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueKindMatchInto::Values (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchInto::RecordKind (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueKindMatchInto::RecordImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::RecordMutable (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueKindMatchInto::Error (value) => value.into (),
			
			ValueKindMatchInto::ProcedurePrimitive (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchInto::ProcedureExtended (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchInto::ProcedureNative (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchInto::ProcedureLambda (value) => value.into (),
			
			ValueKindMatchInto::SyntaxPrimitive (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchInto::SyntaxExtended (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchInto::SyntaxNative (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchInto::SyntaxLambda (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueKindMatchInto::Path (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueKindMatchInto::Port (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ValueKindMatchInto::Process (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchInto::Context (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			ValueKindMatchInto::Binding (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchInto::Parameters (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchInto::Parameter (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			ValueKindMatchInto::Promise (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueKindMatchInto::Opaque (value) => value.into (),
			
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
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::Character (value) => value.into (),
			
			ValueClassMatchInto::Symbol (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueClassMatchInto::Keyword (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueClassMatchInto::Unique (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::String (class) => class.value (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::Bytes (class) => class.value (),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::StringRegex (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::BytesRegex (value) => value.into (),
			
			ValueClassMatchInto::Pair (class) => class.value (),
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueClassMatchInto::Array (class) => class.value (),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueClassMatchInto::Values (value) => value.into (),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::RecordKind (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::Record (class) => class.value (),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueClassMatchInto::Error (value) => value.into (),
			
			ValueClassMatchInto::Procedure (class) => class.value (),
			ValueClassMatchInto::Syntax (class) => class.value (),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueClassMatchInto::Path (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueClassMatchInto::Port (value) => value.into (),
			ValueClassMatchInto::Resource (class) => class.value (),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueClassMatchInto::Opaque (value) => value.into (),
			
			ValueClassMatchInto::Internal (class) => class.value (),
			
		}
		
	}
}


impl ProcedureMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ProcedureMatchInto::Primitive (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ProcedureMatchInto::Extended (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ProcedureMatchInto::Native (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ProcedureMatchInto::Lambda (value) => value.into (),
		}
	}
}


impl SyntaxMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			SyntaxMatchInto::Primitive (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			SyntaxMatchInto::Extended (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			SyntaxMatchInto::Native (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_expressions" ) ]
			#[ cfg ( feature = "vonuvoli_compiler" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			SyntaxMatchInto::Lambda (value) => value.into (),
		}
	}
}


impl ResourceMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
			ResourceMatchInto::Process (value) => value.into (),
			ResourceMatchInto::Other (value) => value,
		}
	}
}


impl InternalMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			InternalMatchInto::Context (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_contexts" ) ]
			InternalMatchInto::Binding (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			InternalMatchInto::Parameters (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			InternalMatchInto::Parameter (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
			InternalMatchInto::Promise (value) => value.into (),
			InternalMatchInto::Other (value) => value,
		}
	}
}


impl ListMatchInto {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (self) -> (Value) {
		match self {
			ListMatchInto::Null => NULL.into (),
			ListMatchInto::PairImmutable (value) => value.into (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ListMatchInto::PairMutable (value) => value.into (),
			ListMatchInto::Value (value) => value,
		}
	}
}




#[ derive ( Clone ) ] // OK
pub struct ValueMeta1 ( u8, u8, u8 );

pub const VALUE_META_1 : ValueMeta1 = ValueMeta1 (0, 0, 0);




#[ derive ( Clone ) ] // OK
pub struct ValueMeta2 ( u8, u8, u8, u8 );

pub const VALUE_META_2 : ValueMeta2 = ValueMeta2 (0, 0, 0, 0);




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd ) ] // OK
pub enum ValueSingleton {
	Null,
	Undefined,
	Void,
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	PortEof,
}




pub enum ValueRef <'a> {
	Immutable (&'a Value),
	ImmutableEmbedded (StdRc<dyn StdAny>, &'a Value),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (StdRef<'a, Value>),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableEmbedded (StdRc<dyn StdAny>, StdRef<'a, Value>),
	Owned (Value),
}


impl <'a> ValueRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_ref (&self) -> (&Value) {
		match *self {
			ValueRef::Immutable (value) =>
				value,
			ValueRef::ImmutableEmbedded (_, value) =>
				value,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (ref value) =>
				value,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (_, ref value) =>
				value,
			ValueRef::Owned (ref value) =>
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (ref value) =>
				(*value) .clone (),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (_, ref value) =>
				(*value) .clone (),
			ValueRef::Owned (ref value) =>
				(*value) .clone (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value_owned (&self) -> (ValueRef<'static>) {
		match *self {
			ValueRef::Immutable (value) =>
				ValueRef::Owned ((*value) .clone ()),
			ValueRef::ImmutableEmbedded (_, value) =>
				ValueRef::Owned ((*value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (ref value) =>
				ValueRef::Owned ((*value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (_, ref value) =>
				ValueRef::Owned ((*value) .clone ()),
			ValueRef::Owned (ref value) =>
				ValueRef::Owned ((*value) .clone ()),
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
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (ref value) =>
				ValueRef::Mutable (StdRef::clone (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (ref embedded, ref value) =>
				ValueRef::MutableEmbedded (StdRc::clone (embedded), StdRef::clone (value)),
			ValueRef::Owned (ref value) =>
				ValueRef::Owned (value.clone ()),
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
			ValueRef::Owned (ref value) =>
				ValueRef::Owned (transformer (value) .clone ()),
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
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::Mutable (value) =>
				GenericRef::Mutable (StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueRef::MutableEmbedded (embedded, value) =>
				GenericRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
			ValueRef::Owned (value) =>
				GenericRef::new_owned_immutable (value, transformer),
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




TODO! ("find a way to eliminate `StdBox` from `*Owned` variants");
pub enum GenericRef <'a, T : 'a + ?Sized> {
	Immutable (&'a T),
	ImmutableEmbedded (StdRc<dyn StdAny>, &'a T),
	ImmutableOwned (StdBox<Value>, &'a T),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	Mutable (StdRef<'a, T>),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableEmbedded (StdRc<dyn StdAny>, StdRef<'a, T>),
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	MutableOwned (StdBox<Value>, StdRef<'a, T>),
}


impl <'a, T : 'a + ?Sized> GenericRef<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn generic_ref (&self) -> (&T) {
		match *self {
			GenericRef::Immutable (value) =>
				value,
			GenericRef::ImmutableEmbedded (_, value) =>
				value,
			GenericRef::ImmutableOwned (_, value) =>
				value,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::Mutable (ref value) =>
				value,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableEmbedded (_, ref value) =>
				value,
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableOwned (_, ref value) =>
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
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_embedded_mutable_from_rc <U : 'static, Accessor> (value : StdRc<U>, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a U) -> (StdRef<'a, T>)
	{
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::MutableEmbedded (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_owned_immutable <Accessor> (value : Value, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a Value) -> (&'a T)
	{
		let value = StdBox::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::ImmutableOwned (value, value_ref)
	}
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_owned_mutable <Accessor> (value : Value, accessor : Accessor) -> (GenericRef<'a, T>)
			where Accessor : FnOnce (&'a Value) -> (StdRef<'a, T>)
	{
		let value = StdBox::new (value);
		let value_ref = unsafe { mem::transmute (value.as_ref ()) };
		let value_ref = accessor (value_ref);
		GenericRef::MutableOwned (value, value_ref)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_ref (&self) -> (GenericRef<'a, T>) {
		match *self {
			GenericRef::Immutable (value) =>
				GenericRef::Immutable (value),
			GenericRef::ImmutableEmbedded (ref embedded, value) =>
				GenericRef::ImmutableEmbedded (StdRc::clone (embedded), value),
			GenericRef::ImmutableOwned (ref embedded, value) =>
				GenericRef::ImmutableOwned (embedded.clone (), value),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::Mutable (ref value) =>
				GenericRef::Mutable (StdRef::clone (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableEmbedded (ref embedded, ref value) =>
				GenericRef::MutableEmbedded (StdRc::clone (embedded), StdRef::clone (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableOwned (ref embedded, ref value) =>
				GenericRef::MutableOwned (embedded.clone (), StdRef::clone (value)),
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
			GenericRef::ImmutableOwned (_, value) =>
				ValueRef::Owned (transformer (value) .clone ()),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::Mutable (value) =>
				ValueRef::Mutable (StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableEmbedded (embedded, value) =>
				ValueRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableOwned (_, value) =>
				ValueRef::Owned (StdRef::map (value, transformer) .clone ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn map_generic <Output : ?Sized, Transformer> (self, transformer : Transformer) -> (GenericRef<'a, Output>)
			where Transformer : FnOnce (&T) -> (&Output)
	{
		match self {
			GenericRef::Immutable (value) =>
				GenericRef::Immutable (transformer (value)),
			GenericRef::ImmutableEmbedded (embedded, value) =>
				GenericRef::ImmutableEmbedded (embedded, transformer (value)),
			GenericRef::ImmutableOwned (embedded, value) =>
				GenericRef::ImmutableOwned (embedded, transformer (value)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::Mutable (value) =>
				GenericRef::Mutable (StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableEmbedded (embedded, value) =>
				GenericRef::MutableEmbedded (embedded, StdRef::map (value, transformer)),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			GenericRef::MutableOwned (embedded, value) =>
				GenericRef::MutableOwned (embedded, StdRef::map (value, transformer)),
		}
	}
}


impl <'a, T : 'a + ?Sized> StdDeref for GenericRef<'a, T> {
	
	type Target = T;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn deref (&self) -> (&T) {
		self.generic_ref ()
	}
}


impl <'a, T : 'a + ?Sized> StdAsRef<T> for GenericRef<'a, T> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref (&self) -> (&T) {
		self.generic_ref ()
	}
}




macro_rules! def_value_placeholder {
	( $identifier : ident ) => (
		
		#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd ) ] // OK
		pub struct $identifier ();
		
		impl $identifier {
			pub fn is_self (&self, _ : &Self) -> (bool) {
				false
			}
		}
		
		#[ cfg ( feature = "vonuvoli_fmt_display" ) ]
		impl fmt::Display for $identifier {
			fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
				write! (formatter, "#<unimplemented:{}>", stringify! ($identifier))
			}
		}
		
		#[ cfg ( feature = "vonuvoli_fmt_debug" ) ]
		impl fmt::Debug for $identifier {
			fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
				write! (formatter, "{}", stringify! ($identifier))
			}
		}
	);
}


#[ cfg ( feature = "vonuvoli_builtins_promises" ) ]
def_value_placeholder! (Promise);

