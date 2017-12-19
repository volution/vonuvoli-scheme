

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;
use std::slice;




pub mod exports {
	
	pub use super::{array_at, array_at_ref, array_at_set};
	
	pub use super::{array_empty};
	pub use super::{array_collect};
	pub use super::{array_collect_from_generator, array_collect_from_generator_ref};
	pub use super::{array_build_1, array_build_2, array_build_3, array_build_4, array_build_n};
	pub use super::{array_append_2, array_append_3, array_append_4, array_append_n};
	pub use super::{array_make, array_clone, array_reverse};
	pub use super::{array_fill_range, array_reverse_range, array_copy_range, array_clone_range};
	pub use super::{array_range_to_list, list_range_to_array};
	pub use super::{array_range_iterator};
	pub use super::{array_length};
	
	pub use super::{vec_array_append_2, vec_array_append_3, vec_array_append_4, vec_array_append_n};
	pub use super::{vec_array_clone, vec_array_drain};
	
	pub use super::{ArrayIterator, ArrayIterators};
	
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

pub fn array_collect_from_generator <Source> (values : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (array_new (values) .into ());
}

pub fn array_collect_from_generator_ref <Source, ValueRef> (values : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	let values = vec_clone_vec_ref (&values);
	succeed! (array_new (values) .into ());
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

pub fn array_build_n (values : &[&Value]) -> (Value) {
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
		buffer.push ((*value).clone ());
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

pub fn array_append_n (arrays : &[&Value]) -> (Outcome<Value>) {
	match arrays.len () {
		0 =>
			succeed! (array_empty ()),
		1 =>
			succeed! (arrays[0].clone ()),
		2 =>
			return array_append_2 (arrays[0], arrays[1]),
		3 =>
			return array_append_3 (arrays[0], arrays[1], arrays[2]),
		4 =>
			return array_append_4 (arrays[0], arrays[1], arrays[2], arrays[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_array_append_n (arrays));
	succeed! (array_new (buffer) .into ());
}




pub fn array_make (length : usize, fill : Option<&Value>) -> (Outcome<Value>) {
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
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




pub fn array_fill_range (array : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let _fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	let (_range_start, _range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	fail_unimplemented! (0x3c6f81b9);
}


pub fn array_reverse_range (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let (_range_start, _range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	fail_unimplemented! (0xfd9c4a54);
}


pub fn array_copy_range (target_array : &Value, target_start : Option<&Value>, source_array : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<Value>) {
	let target_array = try_as_array_ref! (target_array);
	let source_array = try_as_array_ref! (source_array);
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_array.values_length ()));
	let (target_start, target_end) = try! (range_coerce (target_start, None, target_array.values_length ()));
	if (target_end - target_start) < (source_end - source_start) {
		fail! (0x7033eb20);
	}
	fail_unimplemented! (0x3c6f81b9);
}


pub fn array_clone_range (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	succeed! (array_clone_slice (& array.values_as_slice () [range_start..range_end]) .into ());
}




pub fn array_range_to_list (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (array_range_iterator (array, range_start, range_end));
	return list_collect_from_generator_ref (iterator);
}

pub fn list_range_to_array (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return array_collect_from_generator_ref (iterator);
}




pub fn array_range_iterator <'a> (array : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<&'a Value, ArrayIterator<'a>>>) {
	let array = try_as_array_ref! (array);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	let iterator = try! (ArrayIterator::new_a (array));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, Some (range_end)));
	succeed! (iterator);
}




pub fn array_length (array : &Value) -> (Outcome<usize>) {
	let array = try_as_array_ref! (array);
	succeed! (array.values_length ());
}




pub fn vec_array_append_2 (array_1 : &Value, array_2 : &Value) -> (Outcome<ValueVec>) {
	if try! (is_array_empty_all_2 (array_1, array_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	succeed! (buffer);
}

pub fn vec_array_append_3 (array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<ValueVec>) {
	if try! (is_array_empty_all_3 (array_1, array_2, array_3)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	try! (vec_array_drain (&mut buffer, &array_3));
	succeed! (buffer);
}

pub fn vec_array_append_4 (array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<ValueVec>) {
	if try! (is_array_empty_all_4 (array_1, array_2, array_3, array_4)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	try! (vec_array_drain (&mut buffer, &array_3));
	try! (vec_array_drain (&mut buffer, &array_4));
	succeed! (buffer);
}

pub fn vec_array_append_n (arrays : &[&Value]) -> (Outcome<ValueVec>) {
	match arrays.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_array_clone (arrays[0]),
		2 =>
			return vec_array_append_2 (arrays[0], arrays[1]),
		3 =>
			return vec_array_append_3 (arrays[0], arrays[1], arrays[2]),
		4 =>
			return vec_array_append_4 (arrays[0], arrays[1], arrays[2], arrays[3]),
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




pub struct ArrayIterator <'a> ( &'a Array, slice::Iter<'a, Value> );


impl <'a> ArrayIterator <'a> {
	
	pub fn new (array : &'a Value) -> (Outcome<ArrayIterator<'a>>) {
		let array = try_as_array_ref! (array);
		return ArrayIterator::new_a (array);
	}
	
	pub fn new_a (array : &'a Array) -> (Outcome<ArrayIterator<'a>>) {
		succeed! (ArrayIterator (array, array.values_ref () .iter ()));
	}
}


impl <'a> Iterator for ArrayIterator <'a> {
	
	type Item = Outcome<&'a Value>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Value>>) {
		if let Some (value) = self.1.next () {
			return Some (succeeded! (value));
		} else {
			return None;
		}
	}
}




pub struct ArrayIterators <'a> ( StdVec<ArrayIterator<'a>> );


impl <'a> ArrayIterators <'a> {
	
	pub fn new (arrays : &'a [&'a Value]) -> (Outcome<ArrayIterators<'a>>) {
		let iterators = try! (arrays.iter () .map (|array| ArrayIterator::new (array)) .collect ());
		succeed! (ArrayIterators (iterators));
	}
}


impl <'a> Iterator for ArrayIterators <'a> {
	
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

