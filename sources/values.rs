

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::ops;




pub mod exports {
	pub use super::{ValueClass};
	pub use super::{Value, ValueBox, ValueVec};
	pub use super::{Boolean, NumberInteger, NumberReal, Character, Symbol, String, Bytes, Pair, Array};
	pub use super::{boolean, number_i64, number_f64, character};
	pub use super::{symbol, symbol_from_slice, symbol_from_characters};
	pub use super::{string, string_from_slice, string_from_characters};
	pub use super::{bytes_from_slice, array_from_slice};
	pub use super::{list_from_slice, list_from_slice_2};
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	
	Boolean,
	NumberInteger,
	NumberReal,
	Character,
	
	Symbol,
	String,
	Bytes,
	
	Pair,
	Array,
	
	Error,
	
	Lambda,
	ProcedurePrimitive,
	SyntaxPrimitive,
	
	Context,
	Binding,
	
	Number,
	List,
	Procedure,
	Syntax,
	
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Value {
	
	Null,
	Void,
	Undefined,
	
	Boolean ( Boolean ),
	NumberInteger ( NumberInteger ),
	NumberReal ( NumberReal ),
	Character ( Character ),
	
	Symbol ( Symbol ),
	String ( String ),
	Bytes ( Bytes ),
	
	Pair ( Pair ),
	Array ( Array ),
	
	Error ( Error ),
	
	Lambda ( Lambda ),
	ProcedurePrimitive ( ProcedurePrimitive ),
	SyntaxPrimitive ( SyntaxPrimitive ),
	
	Context ( Context ),
	Binding ( Binding ),
	
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


impl Value {
	
	#[ inline (always) ]
	pub fn class (&self) -> (ValueClass) {
		match *self {
			
			Value::Null => ValueClass::Null,
			Value::Void => ValueClass::Void,
			Value::Undefined => ValueClass::Undefined,
			
			Value::Boolean (_) => ValueClass::Boolean,
			Value::NumberInteger (_) => ValueClass::NumberInteger,
			Value::NumberReal (_) => ValueClass::NumberReal,
			Value::Character (_) => ValueClass::Character,
			
			Value::Symbol (_) => ValueClass::Symbol,
			Value::String (_) => ValueClass::String,
			Value::Bytes (_) => ValueClass::Bytes,
			
			Value::Pair (_) => ValueClass::Pair,
			Value::Array (_) => ValueClass::Array,
			
			Value::Error (_) => ValueClass::Error,
			
			Value::Lambda (_) => ValueClass::Lambda,
			Value::ProcedurePrimitive (_) => ValueClass::ProcedurePrimitive,
			Value::SyntaxPrimitive (_) => ValueClass::SyntaxPrimitive,
			
			Value::Context (_) => ValueClass::Context,
			Value::Binding (_) => ValueClass::Binding,
			
		}
	}
	
	#[ inline (always) ]
	pub fn is (&self, class : ValueClass) -> (bool) {
		let class_actual = self.class ();
		if class_actual == class {
			return true;
		} else {
			match class {
				ValueClass::List =>
					return (class_actual == ValueClass::Null) || (class_actual == ValueClass::Pair),
				ValueClass::Number =>
					return (class_actual == ValueClass::NumberInteger) || (class_actual == ValueClass::NumberReal),
				ValueClass::Procedure =>
					return (class_actual == ValueClass::ProcedurePrimitive) || (class_actual == ValueClass::Lambda),
				ValueClass::Syntax =>
					return (class_actual == ValueClass::SyntaxPrimitive) || false,
				_ =>
					panic! ("1fa4f499"),
			}
		}
	}
	
}




impl fmt::Display for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			Value::Null => formatter.write_str ("#null"),
			Value::Void => formatter.write_str ("#void"),
			Value::Undefined => formatter.write_str ("#undefined"),
			Value::Boolean (ref value) => value.fmt (formatter),
			Value::NumberInteger (ref value) => value.fmt (formatter),
			Value::NumberReal (ref value) => value.fmt (formatter),
			Value::Character (ref value) => value.fmt (formatter),
			Value::Symbol (ref value) => value.fmt (formatter),
			Value::String (ref value) => value.fmt (formatter),
			Value::Bytes (ref value) => value.fmt (formatter),
			Value::Pair (ref value) => value.fmt (formatter),
			Value::Array (ref value) => value.fmt (formatter),
			Value::Error (ref value) => value.fmt (formatter),
			Value::Lambda (ref value) => value.fmt (formatter),
			Value::ProcedurePrimitive (ref value) => write! (formatter, "#<procedure:{:?}>", value),
			Value::SyntaxPrimitive (ref value) => write! (formatter, "#<syntax:{:?}>", value),
			Value::Context (ref value) => value.fmt (formatter),
			Value::Binding (ref value) => value.fmt (formatter),
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Boolean ( pub bool );


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


impl fmt::Display for Boolean {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.0 {
			true => formatter.write_str ("#true"),
			false => formatter.write_str ("#false"),
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct NumberInteger ( pub i64 );


impl fmt::Display for NumberInteger {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug) ]
pub struct NumberReal ( pub f64 );


impl cmp::Eq for NumberReal {}

impl cmp::PartialEq for NumberReal {
	fn eq (&self, other : &NumberReal) -> (bool) {
		self.0.to_bits () == other.0.to_bits ()
	}
}

impl hash::Hash for NumberReal {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u64 (self.0.to_bits ());
	}
}

impl fmt::Display for NumberReal {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Character ( pub char );


impl fmt::Display for Character {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		let character = self.0;
		match character {
			'!' ... '~' => { try! (formatter.write_str ("#\\")); try! (formatter.write_char (character)); },
			_ => try! (write! (formatter, "#\\x{:02x}", character as u32)),
		}
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Symbol ( StdRc<StdString> );


impl fmt::Display for Symbol {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		if self.0.is_empty () {
			return formatter.write_str ("||");
		} else {
			// return formatter.write_str (self.0.as_str ());
			use std::fmt::Write;
			try! (formatter.write_char ('|'));
			for character in self.0.chars () {
				match character {
					'|' | '\\' => { try! (formatter.write_char ('\\')); try! (formatter.write_char (character)); },
					' ' ... '~' => try! (formatter.write_char (character)),
					_ => try! (write! (formatter, "#\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
			return Ok(());
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct String ( StdRc<StdString> );


impl fmt::Display for String {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('"'));
		for character in self.0.chars () {
			match character {
				'"' | '\\' => { try! (formatter.write_char ('\\')); try! (formatter.write_char (character)); },
				' ' ... '~' => try! (formatter.write_char (character)),
				_ => try! (write! (formatter, "#\\x{:02x};", character as u32)),
			}
		}
		try! (formatter.write_char ('"'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Bytes ( StdRc<StdVec<u8>> );


impl fmt::Display for Bytes {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#u8("));
		let mut is_first = true;
		for byte in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (write! (formatter, "{}", byte));
		}
		try! (formatter.write_char (')'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Pair ( StdRc<(Value, Value)> );


impl fmt::Display for Pair {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('('));
		let mut head = self;
		loop {
			try! ((head.0).0.fmt (formatter));
			match (head.0).1 {
				Value::Null => break,
				Value::Pair (ref tail) => {
					try! (formatter.write_char (' '));
					head = tail;
				},
				_ => {
					try! (formatter.write_char (' '));
					try! (formatter.write_char ('.'));
					try! (formatter.write_char (' '));
					try! ((head.0).1.fmt (formatter));
					break;
				},
			}
		}
		try! (formatter.write_char (')'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Array ( StdRc<StdVec<Value>> );


impl fmt::Display for Array {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#("));
		let mut is_first = true;
		for element in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (element.fmt (formatter));
		}
		try! (formatter.write_char (')'));
		return Ok(());
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
pub fn symbol (value : StdString) -> (Symbol) {
	Symbol (StdRc::new (value))
}

#[ inline (always) ]
pub fn string (value : StdString) -> (String) {
	String (StdRc::new (value))
}




#[ inline (always) ]
pub fn symbol_from_slice (value : &str) -> (Symbol) {
	symbol (StdString::from (value))
}

#[ inline (always) ]
pub fn string_from_slice (value : &str) -> (String) {
	string (StdString::from (value))
}




#[ inline (always) ]
pub fn symbol_from_characters (slice : &[char]) -> (Symbol) {
	let mut value = StdString::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return symbol (StdString::from (value));
}

#[ inline (always) ]
pub fn string_from_characters (slice : &[char]) -> (String) {
	let mut value = StdString::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return string (StdString::from (value));
}




#[ inline (always) ]
pub fn bytes_from_slice (slice : &[u8]) -> (Bytes) {
	Bytes (StdRc::new (slice.to_vec ()))
}

#[ inline (always) ]
pub fn array_from_slice (slice : &[Value]) -> (Array) {
	Array (StdRc::new (slice.to_vec ()))
}




#[ inline (always) ]
pub fn list_from_slice (slice : &[Value]) -> (Value) {
	list_from_slice_2 (slice, Value::Null)
}

#[ inline (always) ]
pub fn list_from_slice_2 (slice : &[Value], continuation : Value) -> (Value) {
	let mut head = continuation;
	for value in slice.iter () .rev () {
		head = Value::Pair (Pair (StdRc::new ((value.clone (), head))));
	}
	return head;
}

