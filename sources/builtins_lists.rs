

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{pair};
	pub use super::{pair_left, pair_right};
	pub use super::{pair_left_ref, pair_right_ref};
	pub use super::{pair_left_set, pair_right_set};
	
	pub use super::{pair_left as list_first, pair_right as list_rest};
	pub use super::{pair_left_ref as list_first_ref, pair_right_ref as list_rest_ref};
	pub use super::{list_first_at, list_rest_at};
	pub use super::{list_first_at_ref, list_rest_at_ref};
	pub use super::{list_first_at_set, list_rest_at_set};
	pub use super::{list_pair_at_ref};
	
	pub use super::{list_build_1, list_build_2, list_build_3, list_build_4, list_build_n};
	pub use super::{list_append_2, list_append_3, list_append_4, list_append_n};
	pub use super::{list_reverse};
	pub use super::{list_length};
	
	pub use super::{vec_list_append_2, vec_list_append_3, vec_list_append_4, vec_list_append_n};
	pub use super::{vec_list_append_2_dotted, vec_list_append_3_dotted, vec_list_append_4_dotted, vec_list_append_n_dotted};
	pub use super::{vec_list_clone, vec_list_clone_dotted, vec_list_drain, vec_list_drain_dotted};
	
	pub use super::{ListIterator, ListsIterator};
	
}




pub fn pair (left : &Value, right : &Value) -> (Value) {
	return pair_new (left.clone (), right.clone ()) .into ();
}

pub fn pair_left (pair : &Value) -> (Outcome<Value>) {
	succeed! (try! (pair_left_ref (pair)) .clone ());
}

pub fn pair_right (pair : &Value) -> (Outcome<Value>) {
	succeed! (try! (pair_right_ref (pair)) .clone ());
}

pub fn pair_left_ref (pair : &Value) -> (Outcome<&Value>) {
	succeed! (try_as_pair_ref! (pair) .left ());
}

pub fn pair_right_ref (pair : &Value) -> (Outcome<&Value>) {
	succeed! (try_as_pair_ref! (pair) .right ());
}

pub fn pair_left_set (_pair : &Value, _left : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x2073d5a3);
}

pub fn pair_right_set (_pair : &Value, _right : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xa223165c);
}




pub fn list_first_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_first_at_ref (list, index)) .clone ());
}

pub fn list_rest_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_rest_at_ref (list, index)) .clone ());
}


pub fn list_first_at_ref (list : &Value, index : usize) -> (Outcome<&Value>) {
	succeed! (try! (list_pair_at_ref (list, index)) .left ());
}

pub fn list_rest_at_ref (list : &Value, index : usize) -> (Outcome<&Value>) {
	succeed! (try! (list_pair_at_ref (list, index)) .right ());
}

pub fn list_first_at_set (_list : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x562f049a);
}

pub fn list_rest_at_set (_list : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x2ef281ce);
}

pub fn list_pair_at_ref (list : &Value, index : usize) -> (Outcome<&Pair>) {
	let mut cursor = list;
	for _index in 0..index {
		match cursor.class () {
			ValueClass::Pair =>
				cursor = Pair::as_ref (cursor) .right (),
			ValueClass::Null =>
				fail! (0xeb7ddd79),
			_ =>
				fail! (0x4cf78d93),
		}
	}
	return Pair::try_as_ref (cursor);
}




pub fn list_build_1 (value_1 : &Value) -> (Value) {
	return pair_new (value_1.clone (), NULL) .into ();
}

pub fn list_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), NULL) .into ()) .into ();
}

pub fn list_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), NULL) .into ()) .into ()) .into ();
}

pub fn list_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), pair_new (value_4.clone (), NULL) .into ()) .into ()) .into ()) .into ();
}

pub fn list_build_n (values : &[Value]) -> (Value) {
	if values.is_empty () {
		return NULL;
	} else {
		return values.iter () .rev () .fold (NULL, |last, value| pair_new (value.clone (), last) .into ());
	}
}




pub fn list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<Value>) {
	let output = try! (vec_list_append_2_dotted (list_1, list_2));
	return list_append_return (output);
}

pub fn list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<Value>) {
	let output = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	return list_append_return (output);
}

pub fn list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<Value>) {
	let output = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	return list_append_return (output);
}

pub fn list_append_n (lists : &[Value]) -> (Outcome<Value>) {
	let output = try! (vec_list_append_n_dotted (lists));
	return list_append_return (output);
}

fn list_append_return ((values, last) : (ValueVec, Option<Value>)) -> (Outcome<Value>) {
	match last {
		Some (last) =>
			succeed! (list_dotted_new (values, last)),
		None =>
			succeed! (list_new (values)),
	}
}




pub fn list_reverse (_list : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x410a9463);
}




pub fn list_length (list : &Value) -> (Outcome<usize>) {
	let mut cursor = list;
	let mut length : usize = 0;
	loop {
		match cursor.class () {
			ValueClass::Pair => {
				length += 1;
				cursor = Pair::as_ref (cursor) .right ();
			},
			ValueClass::Null =>
				succeed! (length),
			_ =>
				fail! (0x573e319c),
		}
		if cursor == list {
			fail! (0xc0c2b870);
		}
	}
}




pub fn vec_list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<ValueVec>) {
	let output = try! (vec_list_append_2_dotted (list_1, list_2));
	return vec_list_append_return (output);
}

pub fn vec_list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<ValueVec>) {
	let output = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	return vec_list_append_return (output);
}

pub fn vec_list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<ValueVec>) {
	let output = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	return vec_list_append_return (output);
}

pub fn vec_list_append_n (lists : &[Value]) -> (Outcome<ValueVec>) {
	let output = try! (vec_list_append_n_dotted (lists));
	return vec_list_append_return (output);
}

fn vec_list_append_return ((values, last) : (ValueVec, Option<Value>)) -> (Outcome<ValueVec>) {
	match last {
		Some (_) =>
			fail! (0x48f9af8f),
		None =>
			succeed! (values),
	}
}




pub fn vec_list_append_2_dotted (list_1 : &Value, list_2 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_2 (list_1, list_2) {
		succeed! ((StdVec::new (), None));
	}
	let mut values = ValueVec::new ();
	try! (vec_list_drain (&mut values, &list_1));
	let last = try! (vec_list_drain_dotted (&mut values, &list_2));
	succeed! ((values, last));
}

pub fn vec_list_append_3_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_3 (list_1, list_2, list_3) {
		succeed! ((StdVec::new (), None));
	}
	let mut values = ValueVec::new ();
	try! (vec_list_drain (&mut values, &list_1));
	try! (vec_list_drain (&mut values, &list_2));
	let last = try! (vec_list_drain_dotted (&mut values, &list_3));
	succeed! ((values, last));
}

pub fn vec_list_append_4_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_4 (list_1, list_2, list_3, list_4) {
		succeed! ((StdVec::new (), None));
	}
	let mut values = ValueVec::new ();
	try! (vec_list_drain (&mut values, &list_1));
	try! (vec_list_drain (&mut values, &list_2));
	try! (vec_list_drain (&mut values, &list_3));
	let last = try! (vec_list_drain_dotted (&mut values, &list_4));
	succeed! ((values, last));
}

pub fn vec_list_append_n_dotted (lists : &[Value]) -> (Outcome<(ValueVec, Option<Value>)>) {
	match lists.split_last () {
		Some ((list_last, lists_first)) =>
			if lists_first.is_empty () {
				return vec_list_clone_dotted (list_last);
			} else {
				let mut values = ValueVec::new ();
				for list in lists_first {
					try! (vec_list_drain (&mut values, &list));
				}
				let last = try! (vec_list_drain_dotted (&mut values, &list_last));
				succeed! ((values, last));
			},
		None =>
			succeed! ((StdVec::new (), None)),
	}
}




pub fn vec_list_clone (list : &Value) -> (Outcome<ValueVec>) {
	let (vector, last) = try! (vec_list_clone_dotted (list));
	match last {
		Some (_) =>
			fail! (0x096d7253),
		None =>
			succeed! (vector),
	}
}


pub fn vec_list_clone_dotted (list : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	let mut vector = ValueVec::new ();
	let last = try! (vec_list_drain_dotted (&mut vector, list));
	succeed! ((vector, last));
}


pub fn vec_list_drain (vector : &mut ValueVec, list : &Value) -> (Outcome<()>) {
	let last = try! (vec_list_drain_dotted (vector, list));
	match last {
		Some (_) =>
			fail! (0x57ebb8de),
		None =>
			succeed! (()),
	}
}


pub fn vec_list_drain_dotted (vector : &mut ValueVec, list : &Value) -> (Outcome<Option<Value>>) {
	let mut cursor = list;
	loop {
		match cursor.class () {
			ValueClass::Pair => {
				let (left, right) = Pair::as_ref (cursor) .left_and_right ();
				vector.push (left.clone ());
				cursor = right;
			},
			ValueClass::Null =>
				succeed! (None),
			_ =>
				succeed! (Some (cursor.clone ())),
		}
	}
}




pub struct ListIterator <'a> ( &'a Value );


impl <'a> ListIterator <'a> {
	
	pub fn new (value : &'a Value) -> (ListIterator<'a>) {
		return ListIterator (value);
	}
}


impl <'a> Iterator for ListIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		let cursor = self.0;
		let (value, cursor) = match cursor.class () {
			ValueClass::Pair =>
				Pair::as_ref (cursor) .left_and_right (),
			ValueClass::Null =>
				return None,
			_ =>
				return Some (failed! (0xed511f9c)),
		};
		self.0 = cursor;
		return Some (succeeded! (value));
	}
}




pub struct ListsIterator <'a> ( StdVec<ListIterator<'a>> );


impl <'a> ListsIterator <'a> {
	
	pub fn new (values : &'a [Value]) -> (ListsIterator<'a>) {
		let iterators = values.iter () .map (|value| ListIterator::new (value)) .collect ();
		return ListsIterator (iterators);
	}
}


impl <'a> Iterator for ListsIterator <'a> {
	
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

