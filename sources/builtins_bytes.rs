

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
	if let Some (value) = bytes.values_ref () .get (index) {
		succeed! (value);
	} else {
		fail! (0x40bcc72e);
	}
}

pub fn bytes_at_set (_bytes : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x562f049a);
}




pub fn bytes_collect <Source> (values : Source) -> (Value)
		where Source : iter::IntoIterator<Item = u8>, Source::IntoIter : iter::DoubleEndedIterator
{
	let values = values.into_iter () .collect::<StdVec<u8>> ();
	return bytes_new (values) .into ();
}




pub fn bytes_empty () -> (Value) {
	let values = StdVec::new ();
	return bytes_new (values) .into ();
}

pub fn bytes_build_1 (value_1 : &Value) -> (Outcome<Value>) {
	let mut values = StdVec::with_capacity (1);
	values.push (try! (try_as_number_integer_ref! (value_1) .try_to_u8 ()));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_build_2 (value_1 : &Value, value_2 : &Value) -> (Outcome<Value>) {
	let mut values = StdVec::with_capacity (2);
	values.push (try! (try_as_number_integer_ref! (value_1) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_2) .try_to_u8 ()));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Outcome<Value>) {
	let mut values = StdVec::with_capacity (3);
	values.push (try! (try_as_number_integer_ref! (value_1) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_2) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_3) .try_to_u8 ()));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Outcome<Value>) {
	let mut values = StdVec::with_capacity (4);
	values.push (try! (try_as_number_integer_ref! (value_1) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_2) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_3) .try_to_u8 ()));
	values.push (try! (try_as_number_integer_ref! (value_4) .try_to_u8 ()));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_build_n (values : &[Value]) -> (Outcome<Value>) {
	match values.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			return bytes_build_1 (&values[0]),
		2 =>
			return bytes_build_2 (&values[0], &values[1]),
		3 =>
			return bytes_build_3 (&values[0], &values[1], &values[2]),
		4 =>
			return bytes_build_4 (&values[0], &values[1], &values[2], &values[3]),
		_ =>
			(),
	}
	let mut bytes = StdVec::with_capacity (values.len ());
	for value in values {
		bytes.push (try! (try_as_number_integer_ref! (value) .try_to_u8 ()));
	}
	succeed! (bytes_new (bytes) .into ());
}




pub fn bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<Value>) {
	let values = try! (vec_bytes_append_2 (bytes_1, bytes_2));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<Value>) {
	let values = try! (vec_bytes_append_3 (bytes_1, bytes_2, bytes_3));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<Value>) {
	let values = try! (vec_bytes_append_4 (bytes_1, bytes_2, bytes_3, bytes_4));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_append_n (bytess : &[Value]) -> (Outcome<Value>) {
	match bytess.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			succeed! (bytess[0].clone ()),
		2 =>
			return bytes_append_2 (&bytess[0], &bytess[1]),
		3 =>
			return bytes_append_3 (&bytess[0], &bytess[1], &bytess[2]),
		4 =>
			return bytes_append_4 (&bytess[0], &bytess[1], &bytess[2], &bytess[3]),
		_ =>
			(),
	}
	let values = try! (vec_bytes_append_n (bytess));
	succeed! (bytes_new (values) .into ());
}




pub fn bytes_make (length : usize, fill : &Value) -> (Outcome<Value>) {
	let fill = try! (try_as_number_integer_ref! (fill) .try_to_u8 ());
	let mut values = StdVec::with_capacity (length);
	for _index in 0..length {
		values.push (fill);
	}
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_clone (bytes : &Value) -> (Outcome<Value>) {
	let values = try! (vec_bytes_clone (bytes));
	succeed! (bytes_new (values) .into ());
}

pub fn bytes_reverse (bytes : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let values = try! (vec_bytes_clone (bytes));
	succeed! (bytes_collect (values.into_iter () .rev ()));
}




pub fn bytes_length (bytes : &Value) -> (Outcome<usize>) {
	let bytes = try_as_bytes_ref! (bytes);
	succeed! (bytes.values_length ());
}




pub fn vec_bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) {
		succeed! (StdVec::new ());
	}
	let mut values = StdVec::new ();
	try! (vec_bytes_drain (&mut values, &bytes_1));
	try! (vec_bytes_drain (&mut values, &bytes_2));
	succeed! (values);
}

pub fn vec_bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) && is_bytes_empty (bytes_3) {
		succeed! (StdVec::new ());
	}
	let mut values = StdVec::new ();
	try! (vec_bytes_drain (&mut values, &bytes_1));
	try! (vec_bytes_drain (&mut values, &bytes_2));
	try! (vec_bytes_drain (&mut values, &bytes_3));
	succeed! (values);
}

pub fn vec_bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<StdVec<u8>>) {
	if is_bytes_empty (bytes_1) && is_bytes_empty (bytes_2) && is_bytes_empty (bytes_3) && is_bytes_empty (bytes_4) {
		succeed! (StdVec::new ());
	}
	let mut values = StdVec::new ();
	try! (vec_bytes_drain (&mut values, &bytes_1));
	try! (vec_bytes_drain (&mut values, &bytes_2));
	try! (vec_bytes_drain (&mut values, &bytes_3));
	try! (vec_bytes_drain (&mut values, &bytes_4));
	succeed! (values);
}

pub fn vec_bytes_append_n (bytess : &[Value]) -> (Outcome<StdVec<u8>>) {
	match bytess.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_bytes_clone (&bytess[0]),
		2 =>
			return vec_bytes_append_2 (&bytess[0], &bytess[1]),
		3 =>
			return vec_bytes_append_3 (&bytess[0], &bytess[1], &bytess[2]),
		4 =>
			return vec_bytes_append_4 (&bytess[0], &bytess[1], &bytess[2], &bytess[3]),
		_ =>
			(),
	}
	let mut values = StdVec::new ();
	for bytes in bytess {
		try! (vec_bytes_drain (&mut values, &bytes));
	}
	succeed! (values);
}




pub fn vec_bytes_clone (bytes : &Value) -> (Outcome<StdVec<u8>>) {
	let mut values = StdVec::new ();
	try! (vec_bytes_drain (&mut values, bytes));
	succeed! (values);
}


pub fn vec_bytes_drain (values : &mut StdVec<u8>, bytes : &Value) -> (Outcome<()>) {
	let bytes = try_as_bytes_ref! (bytes);
	values.extend_from_slice (bytes.values_as_slice ());
	succeed! (());
}




pub struct BytesIterator <'a> ( &'a Value );


impl <'a> BytesIterator <'a> {
	
	pub fn new (value : &'a Value) -> (BytesIterator<'a>) {
		return BytesIterator (value);
	}
}


impl <'a> Iterator for BytesIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		return Some (failed_unimplemented! (0x408f72b0));
	}
}




pub struct BytessIterator <'a> ( StdVec<BytesIterator<'a>> );


impl <'a> BytessIterator <'a> {
	
	pub fn new (values : &'a [Value]) -> (BytessIterator<'a>) {
		let iterators = values.iter () .map (|value| BytesIterator::new (value)) .collect ();
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

