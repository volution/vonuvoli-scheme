

use ::std::fmt;
use ::std::fmt::Write;
use ::std;



#[ derive (Clone, Debug) ]
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
	Error,
}

impl fmt::Display for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
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
			&Value::Error => formatter.write_str ("#<error>"),
		}
	}
}



#[ derive (Clone, Copy, Debug) ]
pub struct Null ();

impl fmt::Display for Null {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		formatter.write_str ("#null")
	}
}



#[ derive (Clone, Copy, Debug) ]
pub struct Boolean ( bool );

impl fmt::Display for Boolean {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		match self.0 {
			true => formatter.write_str ("#true"),
			false => formatter.write_str ("#false"),
		}
	}
}



#[ derive (Clone, Copy, Debug) ]
pub struct NumberInteger ( i64 );

impl fmt::Display for NumberInteger {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		write! (formatter, "{}", self.0)
	}
}



#[ derive (Clone, Copy, Debug) ]
pub struct NumberReal ( f64 );

impl fmt::Display for NumberReal {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		write! (formatter, "{}", self.0)
	}
}



#[ derive (Clone, Copy, Debug) ]
pub struct Character ( char );

impl fmt::Display for Character {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		let character = self.0;
		match character {
			'!' ... '~' => { try! (formatter.write_str ("#\\")); try! (formatter.write_char (character)); },
			_ => try! (write! (formatter, "#\\x{:02x}", character as u32)),
		}
		return Ok(());
	}
}



#[ derive (Clone, Debug) ]
pub struct Symbol ( std::rc::Rc<std::string::String> );

impl fmt::Display for Symbol {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
		formatter.write_str (self.0.as_str ())
	}
}



#[ derive (Clone, Debug) ]
pub struct String ( std::rc::Rc<std::string::String> );

impl fmt::Display for String {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
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



#[ derive (Clone, Debug) ]
pub struct Bytes ( std::rc::Rc<std::vec::Vec<u8>> );

impl fmt::Display for Bytes {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
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



#[ derive (Clone, Debug) ]
pub struct Pair ( std::rc::Rc<(Value, Value)> );

impl fmt::Display for Pair {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
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



#[ derive (Clone, Debug) ]
pub struct Array ( std::rc::Rc<std::vec::Vec<Value>> );

impl fmt::Display for Array {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> ( fmt::Result ) {
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



pub const NULL : Value = Value::Null (Null ());
pub const TRUE : Value = Value::Boolean (Boolean (true));
pub const FALSE : Value = Value::Boolean (Boolean (false));

pub const UNIMPLEMENTED : Value = Value::Error;



pub fn number_integer ( value : i64 ) -> ( Value ) {
	Value::NumberInteger (NumberInteger (value))
}

pub fn number_real ( value : f64 ) -> ( Value ) {
	Value::NumberReal (NumberReal (value))
}

pub fn character ( value : char ) -> ( Value ) {
	Value::Character (Character (value))
}

pub fn symbol ( value : &str ) -> ( Value ) {
	Value::Symbol (Symbol (std::rc::Rc::new (std::string::String::from (value))))
}

pub fn string ( value : &str ) -> ( Value ) {
	Value::String (String (std::rc::Rc::new (std::string::String::from (value))))
}

pub fn string_from_slice ( slice : &[char] ) -> ( Value ) {
	let mut value = std::string::String::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return Value::String (String (std::rc::Rc::new (std::string::String::from (value))));
}

pub fn bytes_from_slice ( slice : &[u8] ) -> ( Value ) {
	Value::Bytes (Bytes (std::rc::Rc::new (slice.to_vec ())))
}



pub fn list_from_slice ( slice : &[Value] ) -> ( Value ) {
	list_from_slice_2 (slice, NULL)
}

pub fn list_from_slice_2 ( slice : &[Value], continuation : Value ) -> ( Value ) {
	let mut head = continuation.clone ();
	for value in slice.iter () .rev () {
		head = Value::Pair (Pair (std::rc::Rc::new ((value.clone (), head))));
	}
	return head;
}



pub fn array_from_slice ( slice : &[Value] ) -> ( Value ) {
	Value::Array (Array (std::rc::Rc::new (slice.to_vec ())))
}

