

use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;




pub mod exports {
	
	pub use super::{bytes_at, bytes_at_ref, bytes_at_set};
	
	pub use super::{bytes_empty};
	pub use super::{bytes_collect};
	pub use super::{bytes_build_1, bytes_build_2, bytes_build_3, bytes_build_4, bytes_build_n};
	pub use super::{bytes_append_2, bytes_append_3, bytes_append_4, bytes_append_n};
	pub use super::{bytes_make, bytes_clone, bytes_reverse};
	pub use super::{bytes_length};
	
	pub use super::{vec_bytes_append_2, vec_bytes_append_3, vec_bytes_append_4, vec_bytes_append_n};
	pub use super::{vec_bytes_clone, vec_bytes_drain};
	
	pub use super::{BytesIterator, BytessIterator};
	
}




pub fn bytes_at (bytes : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (bytes_at_ref (bytes, index)) .clone () .into ());
}

pub fn bytes_at_ref (bytes : &Value, index : usize) -> (Outcome<&u8>) {
	let bytes = try_as_bytes_ref! (bytes);
	if let Some (byte) = bytes.values_ref () .get (index) {
		succeed! (byte);
	} else {
		fail! (0x9a4ad939);
	}
}

pub fn bytes_at_set (_bytes : &Value, _index : usize, _byte : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xd606bd1c);
}




pub fn bytes_collect <Source> (bytes : Source) -> (Value)
		where Source : iter::IntoIterator<Item = u8>, Source::IntoIter : iter::DoubleEndedIterator
{
	use std::iter::FromIterator;
	return bytes_new (FromIterator::from_iter (bytes)) .into ();
}




pub fn bytes_empty () -> (Value) {
	return bytes_new (StdVec::new ()) .into ();
}

pub fn bytes_build_1 (byte_1 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_build_2 (byte_1 : &Value, byte_2 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_build_3 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_build_4 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value, byte_4 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_4) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_build_n (bytes : &[Value]) -> (Outcome<Value>) {
	match bytes.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			return bytes_build_1 (&bytes[0]),
		2 =>
			return bytes_build_2 (&bytes[0], &bytes[1]),
		3 =>
			return bytes_build_3 (&bytes[0], &bytes[1], &bytes[2]),
		4 =>
			return bytes_build_4 (&bytes[0], &bytes[1], &bytes[2], &bytes[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (try! (try_as_number_integer_ref! (byte) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer) .into ());
}




pub fn bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_2 (bytes_1, bytes_2));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_3 (bytes_1, bytes_2, bytes_3));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_4 (bytes_1, bytes_2, bytes_3, bytes_4));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_append_n (bytes : &[Value]) -> (Outcome<Value>) {
	match bytes.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			succeed! (bytes[0].clone ()),
		2 =>
			return bytes_append_2 (&bytes[0], &bytes[1]),
		3 =>
			return bytes_append_3 (&bytes[0], &bytes[1], &bytes[2]),
		4 =>
			return bytes_append_4 (&bytes[0], &bytes[1], &bytes[2], &bytes[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_bytes_append_n (bytes));
	succeed! (bytes_new (buffer) .into ());
}




pub fn bytes_make (length : usize, fill : &Value) -> (Outcome<Value>) {
	let fill = try! (try_as_number_integer_ref! (fill) .try_to_u8 ());
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill);
	}
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_clone (bytes : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_clone (bytes));
	succeed! (bytes_new (buffer) .into ());
}

pub fn bytes_reverse (bytes : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let buffer = try! (vec_bytes_clone (bytes));
	succeed! (bytes_collect (buffer.into_iter () .rev ()));
}




pub fn bytes_length (bytes : &Value) -> (Outcome<usize>) {
	let bytes = try_as_bytes_ref! (bytes);
	succeed! (bytes.values_length ());
}




pub fn vec_bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	succeed! (buffer);
}

pub fn vec_bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) && is_bytes_empty (bytes_3) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	try! (vec_bytes_drain (&mut buffer, &bytes_3));
	succeed! (buffer);
}

pub fn vec_bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) && is_bytes_empty (bytes_3) && is_bytes_empty (bytes_4) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	try! (vec_bytes_drain (&mut buffer, &bytes_3));
	try! (vec_bytes_drain (&mut buffer, &bytes_4));
	succeed! (buffer);
}

pub fn vec_bytes_append_n (bytes : &[Value]) -> (Outcome<StdVec<u8>>) {
	match bytes.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_bytes_clone (&bytes[0]),
		2 =>
			return vec_bytes_append_2 (&bytes[0], &bytes[1]),
		3 =>
			return vec_bytes_append_3 (&bytes[0], &bytes[1], &bytes[2]),
		4 =>
			return vec_bytes_append_4 (&bytes[0], &bytes[1], &bytes[2], &bytes[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::new ();
	for bytes in bytes {
		try! (vec_bytes_drain (&mut buffer, &bytes));
	}
	succeed! (buffer);
}




pub fn vec_bytes_clone (bytes : &Value) -> (Outcome<StdVec<u8>>) {
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, bytes));
	succeed! (buffer);
}


pub fn vec_bytes_drain (buffer : &mut StdVec<u8>, bytes : &Value) -> (Outcome<()>) {
	let bytes = try_as_bytes_ref! (bytes);
	buffer.extend_from_slice (bytes.values_as_slice ());
	succeed! (());
}




pub struct BytesIterator <'a> ( &'a Value );


impl <'a> BytesIterator <'a> {
	
	pub fn new (bytes : &'a Value) -> (BytesIterator<'a>) {
		return BytesIterator (bytes);
	}
}


impl <'a> Iterator for BytesIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		return Some (failed_unimplemented! (0x8379f462));
	}
}




pub struct BytessIterator <'a> ( StdVec<BytesIterator<'a>> );


impl <'a> BytessIterator <'a> {
	
	pub fn new (bytes : &'a [Value]) -> (BytessIterator<'a>) {
		let iterators = bytes.iter () .map (|bytes| BytesIterator::new (bytes)) .collect ();
		return BytessIterator (iterators);
	}
}


impl <'a> Iterator for BytessIterator <'a> {
	
	type Item = Outcome<StdVec<&'a Value>>;
	
	fn next (&mut self) -> (Option<Outcome<StdVec<&'a Value>>>) {
		let mut outcomes = StdVec::with_capacity (self.0.len ());
		for mut iterator in self.0.iter_mut () {
			match iterator.next () {
				Some (Ok (outcome)) =>
					outcomes.push (outcome),
				Some (Err (error)) =>
					return Some (Err (error)),
				None =>
					return None,
			}
		}
		return Some (succeeded! (outcomes));
	}
}

