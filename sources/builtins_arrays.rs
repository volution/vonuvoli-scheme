

use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;




pub mod exports {
	
	pub use super::{array_at, array_at_ref, array_at_set};
	
	pub use super::{array_empty};
	pub use super::{array_collect};
	pub use super::{array_build_1, array_build_2, array_build_3, array_build_4, array_build_n};
	pub use super::{array_append_2, array_append_3, array_append_4, array_append_n};
	pub use super::{array_make, array_clone, array_reverse};
	pub use super::{array_length};
	
	pub use super::{vec_array_append_2, vec_array_append_3, vec_array_append_4, vec_array_append_n};
	pub use super::{vec_array_clone, vec_array_drain};
	
	pub use super::{ArrayIterator, ArraysIterator};
	
}




pub fn array_at (array : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (array_at_ref (array, index)) .clone ());
}

pub fn array_at_ref (array : &Value, index : usize) -> (Outcome<&Value>) {
	let array = try_as_array_ref! (array);
	if let Some (value) = array.values_ref () .get (index) {
		succeed! (value);
	} else {
		fail! (0x40bcc72e);
	}
}

pub fn array_at_set (_array : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x562f049a);
}




pub fn array_collect <Source> (values : Source) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	use std::iter::FromIterator;
	return array_new (FromIterator::from_iter (values)) .into ();
}




pub fn array_empty () -> (Value) {
	return array_new (StdVec::new ()) .into ();
}

pub fn array_build_1 (value_1 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (value_1.clone ());
	return array_new (buffer) .into ();
}

pub fn array_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	return array_new (buffer) .into ();
}

pub fn array_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	return array_new (buffer) .into ();
}

pub fn array_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	buffer.push (value_4.clone ());
	return array_new (buffer) .into ();
}

pub fn array_build_n (values : &[Value]) -> (Value) {
	match values.len () {
		0 =>
			return array_empty (),
		1 =>
			return array_build_1 (&values[0]),
		2 =>
			return array_build_2 (&values[0], &values[1]),
		3 =>
			return array_build_3 (&values[0], &values[1], &values[2]),
		4 =>
			return array_build_4 (&values[0], &values[1], &values[2], &values[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::with_capacity (values.len ());
	for value in values {
		buffer.push (value.clone ());
	}
	return array_new (buffer) .into ();
}




pub fn array_append_2 (array_1 : &Value, array_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_2 (array_1, array_2));
	succeed! (array_new (buffer) .into ());
}

pub fn array_append_3 (array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_3 (array_1, array_2, array_3));
	succeed! (array_new (buffer) .into ());
}

pub fn array_append_4 (array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_4 (array_1, array_2, array_3, array_4));
	succeed! (array_new (buffer) .into ());
}

pub fn array_append_n (arrays : &[Value]) -> (Outcome<Value>) {
	match arrays.len () {
		0 =>
			succeed! (array_empty ()),
		1 =>
			succeed! (arrays[0].clone ()),
		2 =>
			return array_append_2 (&arrays[0], &arrays[1]),
		3 =>
			return array_append_3 (&arrays[0], &arrays[1], &arrays[2]),
		4 =>
			return array_append_4 (&arrays[0], &arrays[1], &arrays[2], &arrays[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_array_append_n (arrays));
	succeed! (array_new (buffer) .into ());
}




pub fn array_make (length : usize, fill : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill.clone ());
	}
	succeed! (array_new (buffer) .into ());
}

pub fn array_clone (array : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_clone (array));
	succeed! (array_new (buffer) .into ());
}

pub fn array_reverse (array : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let buffer = try! (vec_array_clone (array));
	succeed! (array_collect (buffer.into_iter () .rev ()));
}




pub fn array_length (array : &Value) -> (Outcome<usize>) {
	let array = try_as_array_ref! (array);
	succeed! (array.values_length ());
}




pub fn vec_array_append_2 (array_1 : &Value, array_2 : &Value) -> (Outcome<ValueVec>) {
	if is_array_empty (array_1) && is_array_empty (array_2) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	succeed! (buffer);
}

pub fn vec_array_append_3 (array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<ValueVec>) {
	if is_array_empty (array_1) && is_array_empty (array_2) && is_array_empty (array_3) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	try! (vec_array_drain (&mut buffer, &array_3));
	succeed! (buffer);
}

pub fn vec_array_append_4 (array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<ValueVec>) {
	if is_array_empty (array_1) && is_array_empty (array_2) && is_array_empty (array_3) && is_array_empty (array_4) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	try! (vec_array_drain (&mut buffer, &array_3));
	try! (vec_array_drain (&mut buffer, &array_4));
	succeed! (buffer);
}

pub fn vec_array_append_n (arrays : &[Value]) -> (Outcome<ValueVec>) {
	match arrays.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_array_clone (&arrays[0]),
		2 =>
			return vec_array_append_2 (&arrays[0], &arrays[1]),
		3 =>
			return vec_array_append_3 (&arrays[0], &arrays[1], &arrays[2]),
		4 =>
			return vec_array_append_4 (&arrays[0], &arrays[1], &arrays[2], &arrays[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::new ();
	for array in arrays {
		try! (vec_array_drain (&mut buffer, &array));
	}
	succeed! (buffer);
}




pub fn vec_array_clone (array : &Value) -> (Outcome<ValueVec>) {
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, array));
	succeed! (buffer);
}


pub fn vec_array_drain (buffer : &mut ValueVec, array : &Value) -> (Outcome<()>) {
	let array = try_as_array_ref! (array);
	buffer.extend_from_slice (array.values_as_slice ());
	succeed! (());
}




pub struct ArrayIterator <'a> ( &'a Value );


impl <'a> ArrayIterator <'a> {
	
	pub fn new (array : &'a Value) -> (ArrayIterator<'a>) {
		return ArrayIterator (array);
	}
}


impl <'a> Iterator for ArrayIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		return Some (failed_unimplemented! (0x408f72b0));
	}
}




pub struct ArraysIterator <'a> ( StdVec<ArrayIterator<'a>> );


impl <'a> ArraysIterator <'a> {
	
	pub fn new (arrays : &'a [Value]) -> (ArraysIterator<'a>) {
		let iterators = arrays.iter () .map (|array| ArrayIterator::new (array)) .collect ();
		return ArraysIterator (iterators);
	}
}


impl <'a> Iterator for ArraysIterator <'a> {
	
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

