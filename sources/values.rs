

use std::cmp;
use std::f64;
use std::fmt;
use std::hash;

use super::errors::exports::*;
use super::runtime::std::*;


pub mod exports {
	pub use super::{Value, Symbol};
	pub use super::{NULL, TRUE, FALSE};
	pub use super::{ONE, ZERO};
	pub use super::{INF_POSITIVE, INF_NEGATIVE, NAN_POSITIVE, NAN_NEGATIVE};
	pub use super::{number_i64, number_f64, character};
	pub use super::{symbol, string, string_from_slice, bytes_from_slice};
	pub use super::{list_from_slice, list_from_slice_2, array_from_slice};
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Value {
	Null ( Null ),
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
}


impl StdFrom<Null> for Value { fn from (value : Null) -> (Self) { Value::Null (value) } }
impl StdFrom<Boolean> for Value { fn from (value : Boolean) -> (Self) { Value::Boolean (value) } }
impl StdFrom<NumberInteger> for Value { fn from (value : NumberInteger) -> (Self) { Value::NumberInteger (value) } }
impl StdFrom<NumberReal> for Value { fn from (value : NumberReal) -> (Self) { Value::NumberReal (value) } }
impl StdFrom<Character> for Value { fn from (value : Character) -> (Self) { Value::Character (value) } }
impl StdFrom<Symbol> for Value { fn from (value : Symbol) -> (Self) { Value::Symbol (value) } }
impl StdFrom<String> for Value { fn from (value : String) -> (Self) { Value::String (value) } }
impl StdFrom<Bytes> for Value { fn from (value : Bytes) -> (Self) { Value::Bytes (value) } }
impl StdFrom<Pair> for Value { fn from (value : Pair) -> (Self) { Value::Pair (value) } }
impl StdFrom<Array> for Value { fn from (value : Array) -> (Self) { Value::Array (value) } }
impl StdFrom<Error> for Value { fn from (value : Error) -> (Self) { Value::Error (value) } }


impl fmt::Display for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self {
			&Value::Null (ref value) => value.fmt (formatter),
			&Value::Boolean (ref value) => value.fmt (formatter),
			&Value::NumberInteger (ref value) => value.fmt (formatter),
			&Value::NumberReal (ref value) => value.fmt (formatter),
			&Value::Character (ref value) => value.fmt (formatter),
			&Value::Symbol (ref value) => value.fmt (formatter),
			&Value::String (ref value) => value.fmt (formatter),
			&Value::Bytes (ref value) => value.fmt (formatter),
			&Value::Pair (ref value) => value.fmt (formatter),
			&Value::Array (ref value) => value.fmt (formatter),
			&Value::Error (ref value) => value.fmt (formatter),
		}
	}
}




#[ derive (Clone, Copy, Debug, Eq, PartialEq, Hash) ]
pub struct Null ();


impl fmt::Display for Null {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#null")
	}
}




#[ derive (Clone, Copy, Debug, Eq, PartialEq, Hash) ]
pub struct Boolean ( bool );


impl fmt::Display for Boolean {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.0 {
			true => formatter.write_str ("#true"),
			false => formatter.write_str ("#false"),
		}
	}
}




#[ derive (Clone, Copy, Debug, Eq, PartialEq, Hash) ]
pub struct NumberInteger ( i64 );


impl fmt::Display for NumberInteger {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Clone, Copy, Debug) ]
pub struct NumberReal ( f64 );


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




#[ derive (Clone, Copy, Debug, Eq, PartialEq, Hash) ]
pub struct Character ( char );


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


impl StdFrom<StdString> for Symbol { fn from (value : StdString) -> (Self) { Symbol (StdRc::new (value)) } }
impl StdFrom<&'static str> for Symbol { fn from (value : &str) -> (Self) { Symbol (StdRc::new (StdString::from (value))) } }


impl fmt::Display for Symbol {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str (self.0.as_str ())
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
pub struct Bytes ( StdRc<StdVector<u8>> );


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
				Value::Null (_) => break,
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
pub struct Array ( StdRc<StdVector<Value>> );


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




pub const NULL : Null = Null ();

pub const TRUE : Boolean = Boolean (true);
pub const FALSE : Boolean = Boolean (false);

pub const ZERO : NumberInteger = NumberInteger (0);
pub const ONE : NumberInteger = NumberInteger (1);

pub const INF_POSITIVE : NumberReal = NumberReal (f64::INFINITY);
pub const INF_NEGATIVE : NumberReal = NumberReal (f64::NEG_INFINITY);
pub const NAN_POSITIVE : NumberReal = NumberReal (f64::NAN);
pub const NAN_NEGATIVE : NumberReal = NumberReal (f64::NAN);




pub fn number_i64 (value : i64) -> (NumberInteger) {
	NumberInteger (value)
}

pub fn number_f64 (value : f64) -> (NumberReal) {
	NumberReal (value)
}

pub fn character (value : char) -> (Character) {
	Character (value)
}

pub fn symbol (value : &str) -> (Symbol) {
	Symbol (StdRc::new (StdString::from (value)))
}

pub fn string (value : &str) -> (String) {
	String (StdRc::new (StdString::from (value)))
}




pub fn string_from_slice (slice : &[char]) -> (String) {
	let mut value = StdString::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return String (StdRc::new (StdString::from (value)));
}

pub fn bytes_from_slice (slice : &[u8]) -> (Bytes) {
	Bytes (StdRc::new (slice.to_vec ()))
}




pub fn list_from_slice (slice : &[Value]) -> (Value) {
	list_from_slice_2 (slice, NULL.into ())
}

pub fn list_from_slice_2 (slice : &[Value], continuation : Value) -> (Value) {
	let mut head = continuation.clone ();
	for value in slice.iter () .rev () {
		head = Value::Pair (Pair (StdRc::new ((value.clone (), head))));
	}
	return head;
}




pub fn array_from_slice (slice : &[Value]) -> (Array) {
	Array (StdRc::new (slice.to_vec ()))
}

