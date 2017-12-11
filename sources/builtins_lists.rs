

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;




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
	pub use super::{list_pair_at, list_pair_at_ref};
	
	pub use super::{list_empty};
	pub use super::{list_collect, list_collect_dotted};
	pub use super::{list_collect_from_generator, list_collect_dotted_from_generator};
	pub use super::{list_build_1, list_build_2, list_build_3, list_build_4, list_build_n};
	pub use super::{list_append_2, list_append_3, list_append_4, list_append_n};
	pub use super::{list_make, list_clone, list_reverse};
	pub use super::{list_fill_range, list_reverse_range, list_copy_range, list_clone_range};
	pub use super::{list_range_iterator};
	pub use super::{list_length};
	
	pub use super::{list_member_by_comparison, list_member_by_comparator};
	pub use super::{list_assoc_by_comparison, list_assoc_by_comparator};
	
	pub use super::{vec_list_append_2, vec_list_append_3, vec_list_append_4, vec_list_append_n};
	pub use super::{vec_list_append_2_dotted, vec_list_append_3_dotted, vec_list_append_4_dotted, vec_list_append_n_dotted};
	pub use super::{vec_list_clone, vec_list_clone_dotted, vec_list_drain, vec_list_drain_dotted};
	
	pub use super::{ListIterator, ListIterators};
	
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
	fail_unimplemented! (0x2073d5a3); // deferred
}

pub fn pair_right_set (_pair : &Value, _right : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xa223165c); // deferred
}




pub fn list_first_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_first_at_ref (list, index)) .clone ());
}

pub fn list_rest_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_rest_at_ref (list, index)) .clone ());
}


pub fn list_first_at_ref (list : &Value, index : usize) -> (Outcome<&Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		succeed! (pair.left ());
	} else {
		fail! (0xf3b2488a);
	}
}

pub fn list_rest_at_ref (list : &Value, index : usize) -> (Outcome<&Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		succeed! (pair.right ());
	} else {
		fail! (0x9ea1c42c);
	}
}

pub fn list_first_at_set (_list : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x562f049a); // deferred
}

pub fn list_rest_at_set (_list : &Value, _index : usize, _value : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0x2ef281ce); // deferred
}

pub fn list_pair_at (list : &Value, index : usize) -> (Outcome<Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		succeed! (pair.clone () .into ());
	} else {
		succeed! (NULL.into ());
	}
}

pub fn list_pair_at_ref (list : &Value, index : usize) -> (Outcome<Option<&Pair>>) {
	let mut iterator = try! (ListPairIterator::new (list));
	let mut offset = 0;
	loop {
		match iterator.next () {
			Some (Ok (pair)) =>
				if offset == index {
					succeed! (Some (pair));
				} else {
					offset += 1;
				},
			Some (Err (error)) =>
				return Err (error),
			None =>
				if offset == index {
					succeed! (None);
				} else {
					fail! (0xeb7ddd79);
				},
		}
	}
}




pub fn list_collect <Source> (values : Source) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	return list_collect_dotted (values, None);
}

pub fn list_collect_dotted <Source> (values : Source, last : Option<Value>) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	let last = if let Some (last) = last {
		last
	} else {
		NULL.into ()
	};
	return values.into_iter () .rev () .fold (last, |last, value| pair_new (value, last) .into ());
}




pub fn list_collect_from_generator <Source> (values : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	return list_collect_dotted_from_generator (values, None);
}

pub fn list_collect_dotted_from_generator <Source> (values : Source, last : Option<Value>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	// FIXME:  Remove vector allocation!
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (list_collect_dotted (values, last));
}




pub fn list_empty () -> (Value) {
	return NULL.into ();
}

pub fn list_build_1 (value_1 : &Value) -> (Value) {
	return pair_new (value_1.clone (), NULL.into ()) .into ();
}

pub fn list_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), NULL.into ()) .into ()) .into ();
}

pub fn list_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), NULL.into ()) .into ()) .into ()) .into ();
}

pub fn list_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), pair_new (value_4.clone (), NULL.into ()) .into ()) .into ()) .into ()) .into ();
}

pub fn list_build_n (values : &[Value]) -> (Value) {
	match values.len () {
		0 =>
			return list_empty (),
		1 =>
			return list_build_1 (&values[0]),
		2 =>
			return list_build_2 (&values[0], &values[1]),
		3 =>
			return list_build_3 (&values[0], &values[1], &values[2]),
		4 =>
			return list_build_4 (&values[0], &values[1], &values[2], &values[3]),
		_ =>
			(),
	}
	return values.iter () .rev () .fold (NULL.into (), |last, value| pair_new (value.clone (), last) .into ());
}




pub fn list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let (buffer, last) = try! (vec_list_append_2_dotted (list_1, list_2));
	succeed! (list_collect_dotted (buffer, last));
}

pub fn list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let (buffer, last) = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	succeed! (list_collect_dotted (buffer, last));
}

pub fn list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let (buffer, last) = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	succeed! (list_collect_dotted (buffer, last));
}

pub fn list_append_n (lists : &[Value]) -> (Outcome<Value>) {
	match lists.len () {
		0 =>
			succeed! (list_empty ()),
		1 =>
			succeed! (lists[0].clone ()),
		2 =>
			return list_append_2 (&lists[0], &lists[1]),
		3 =>
			return list_append_3 (&lists[0], &lists[1], &lists[2]),
		4 =>
			return list_append_4 (&lists[0], &lists[1], &lists[2], &lists[3]),
		_ =>
			(),
	}
	// FIXME:  Optimize the vector allocation!
	let (buffer, last) = try! (vec_list_append_n_dotted (lists));
	succeed! (list_collect_dotted (buffer, last));
}




pub fn list_make (length : usize, fill : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill.clone ());
	}
	succeed! (list_collect (buffer));
}

pub fn list_clone (list : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let (buffer, last) = try! (vec_list_clone_dotted (list));
	succeed! (list_collect_dotted (buffer, last));
}

pub fn list_reverse (list : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let buffer = try! (vec_list_clone (list));
	succeed! (list_collect (buffer.into_iter () .rev ()));
}




pub fn list_fill_range (_list : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let _fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	let (_range_start, _range_end) = try! (range_coerce_unbounded (range_start, range_end));
	fail_unimplemented! (0x2abbe2f5); // deferred
}


pub fn list_reverse_range (_list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let (_range_start, _range_end) = try! (range_coerce_unbounded (range_start, range_end));
	fail_unimplemented! (0x562a1252); // deferred
}


pub fn list_copy_range (_target_list : &Value, target_start : Option<&Value>, _source_list : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<Value>) {
	let (_source_start, _source_end) = try! (range_coerce_unbounded (source_start, source_end));
	let (_target_start, _target_end) = try! (range_coerce_unbounded (target_start, None));
	fail_unimplemented! (0xb5cd48df); // deferred
}


pub fn list_clone_range (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return list_collect_from_generator (iterator);
}




pub fn list_range_iterator <'a> (list : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<Value, ListIterator<'a>>>) {
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let iterator = try! (ListIterator::new (list));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, range_end));
	succeed! (iterator);
}




pub fn list_length (list : &Value) -> (Outcome<usize>) {
	let mut length : usize = 0;
	let mut iterator = try! (ListPairIterator::new (list));
	loop {
		match iterator.next () {
			Some (Ok (_)) =>
				length += 1,
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (length),
		}
	}
}




pub fn list_member_by_comparison (list : &Value, value : &Value, comparison : Comparison) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list));
	loop {
		match iterator.next () {
			Some (Ok (pair)) =>
				if try! (compare_2 (value, pair.left (), comparison)) {
					succeed! (pair.clone () .into ());
				}
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}

pub fn list_member_by_comparator (list : &Value, value : &Value, comparator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let comparison = try! (evaluator.evaluator.evaluate_procedure_call_2_with_values (evaluator, comparator, value, pair.left ()));
				if is_not_false (&comparison) {
					succeed! (pair.clone () .into ());
				}
			},
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}


pub fn list_assoc_by_comparison (list : &Value, value : &Value, comparison : Comparison) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let pair = try_as_pair_ref! (pair.left ());
				if try! (compare_2 (value, pair.left (), comparison)) {
					succeed! (pair.clone () .into ());
				}
			},
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}

pub fn list_assoc_by_comparator (list : &Value, value : &Value, comparator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let pair = try_as_pair_ref! (pair.left ());
				let comparison = try! (evaluator.evaluator.evaluate_procedure_call_2_with_values (evaluator, comparator, value, pair.left ()));
				if is_not_false (&comparison) {
					succeed! (pair.clone () .into ());
				}
			},
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}




pub fn vec_list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_2_dotted (list_1, list_2));
	return vec_list_append_return (buffer);
}

pub fn vec_list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	return vec_list_append_return (buffer);
}

pub fn vec_list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	return vec_list_append_return (buffer);
}

pub fn vec_list_append_n (lists : &[Value]) -> (Outcome<ValueVec>) {
	match lists.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_list_clone (&lists[0]),
		2 =>
			return vec_list_append_2 (&lists[0], &lists[1]),
		3 =>
			return vec_list_append_3 (&lists[0], &lists[1], &lists[2]),
		4 =>
			return vec_list_append_4 (&lists[0], &lists[1], &lists[2], &lists[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_list_append_n_dotted (lists));
	return vec_list_append_return (buffer);
}

fn vec_list_append_return ((buffer, last) : (ValueVec, Option<Value>)) -> (Outcome<ValueVec>) {
	match last {
		Some (_) =>
			fail! (0x48f9af8f),
		None =>
			succeed! (buffer),
	}
}




pub fn vec_list_append_2_dotted (list_1 : &Value, list_2 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_2 (list_1, list_2) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_drain (&mut buffer, &list_1));
	let last = try! (vec_list_drain_dotted (&mut buffer, &list_2));
	succeed! ((buffer, last));
}

pub fn vec_list_append_3_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_3 (list_1, list_2, list_3) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_drain (&mut buffer, &list_1));
	try! (vec_list_drain (&mut buffer, &list_2));
	let last = try! (vec_list_drain_dotted (&mut buffer, &list_3));
	succeed! ((buffer, last));
}

pub fn vec_list_append_4_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_4 (list_1, list_2, list_3, list_4) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_drain (&mut buffer, &list_1));
	try! (vec_list_drain (&mut buffer, &list_2));
	try! (vec_list_drain (&mut buffer, &list_3));
	let last = try! (vec_list_drain_dotted (&mut buffer, &list_4));
	succeed! ((buffer, last));
}

pub fn vec_list_append_n_dotted (lists : &[Value]) -> (Outcome<(ValueVec, Option<Value>)>) {
	match lists.len () {
		0 =>
			succeed! ((StdVec::new (), None)),
		1 =>
			return vec_list_clone_dotted (&lists[0]),
		2 =>
			return vec_list_append_2_dotted (&lists[0], &lists[1]),
		3 =>
			return vec_list_append_3_dotted (&lists[0], &lists[1], &lists[2]),
		4 =>
			return vec_list_append_4_dotted (&lists[0], &lists[1], &lists[2], &lists[3]),
		_ =>
			(),
	}
	match lists.split_last () {
		Some ((list_last, lists_first)) =>
			if lists_first.is_empty () {
				return vec_list_clone_dotted (list_last);
			} else {
				let mut buffer = StdVec::new ();
				for list in lists_first {
					try! (vec_list_drain (&mut buffer, &list));
				}
				let last = try! (vec_list_drain_dotted (&mut buffer, &list_last));
				succeed! ((buffer, last));
			},
		None =>
			succeed! ((StdVec::new (), None)),
	}
}




pub fn vec_list_clone (list : &Value) -> (Outcome<ValueVec>) {
	let (buffer, last) = try! (vec_list_clone_dotted (list));
	match last {
		Some (_) =>
			fail! (0x096d7253),
		None =>
			succeed! (buffer),
	}
}


pub fn vec_list_clone_dotted (list : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	let mut buffer = StdVec::new ();
	let last = try! (vec_list_drain_dotted (&mut buffer, list));
	succeed! ((buffer, last));
}


pub fn vec_list_drain (buffer : &mut ValueVec, list : &Value) -> (Outcome<()>) {
	let last = try! (vec_list_drain_dotted (buffer, list));
	match last {
		Some (_) =>
			fail! (0x57ebb8de),
		None =>
			succeed! (()),
	}
}


pub fn vec_list_drain_dotted (buffer : &mut ValueVec, list : &Value) -> (Outcome<Option<Value>>) {
	let mut cursor = list;
	loop {
		match cursor.class () {
			ValueClass::Pair => {
				let (left, right) = Pair::as_ref (cursor) .left_and_right ();
				buffer.push (left.clone ());
				cursor = right;
			},
			ValueClass::Null =>
				succeed! (None),
			_ =>
				succeed! (Some (cursor.clone ())),
		}
		if list.is_self (cursor) {
			fail! (0x7b9aae29);
		}
	}
}




pub struct ListIterator <'a> ( &'a Value );


impl <'a> ListIterator <'a> {
	
	pub fn new (value : &'a Value) -> (Outcome<ListIterator<'a>>) {
		succeed! (ListIterator (value));
	}
}


impl <'a> Iterator for ListIterator <'a> {
	
	type Item = Outcome<Value>;
	
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		let cursor = self.0;
		let (value, cursor) = match cursor.class () {
			ValueClass::Pair =>
				Pair::as_ref (cursor) .left_and_right (),
			ValueClass::Null =>
				return None,
			_ =>
				return Some (failed! (0xed511f9c)),
		};
		if self.0.is_self (cursor) {
			return Some (failed! (0x2f6495d9));
		}
		self.0 = cursor;
		return Some (succeeded! (value.clone ()));
	}
}




pub struct ListPairIterator <'a> ( &'a Value );


impl <'a> ListPairIterator <'a> {
	
	pub fn new (value : &'a Value) -> (Outcome<ListPairIterator<'a>>) {
		succeed! (ListPairIterator (value));
	}
}


impl <'a> Iterator for ListPairIterator <'a> {
	
	type Item = Outcome<&'a Pair>;
	
	fn next (&mut self) -> (Option<Outcome<&'a Pair>>) {
		
		let cursor = self.0;
		let (pair, cursor) = match cursor.class () {
			ValueClass::Pair => {
				let pair = Pair::as_ref (cursor);
				let cursor = pair.right ();
				(pair, cursor)
			},
			ValueClass::Null =>
				return None,
			_ =>
				return Some (failed! (0x1f8fea4c)),
		};
		if self.0.is_self (cursor) {
			return Some (failed! (0xa8ab23fb));
		}
		self.0 = cursor;
		return Some (succeeded! (pair));
	}
}




pub struct ListIterators <'a> ( StdVec<ListIterator<'a>> );


impl <'a> ListIterators <'a> {
	
	pub fn new (lists : &'a [Value]) -> (Outcome<ListIterators<'a>>) {
		let iterators = try! (lists.iter () .map (|list| ListIterator::new (list)) .collect ());
		succeed! (ListIterators (iterators));
	}
}


impl <'a> Iterator for ListIterators <'a> {
	
	type Item = Outcome<StdVec<Value>>;
	
	fn next (&mut self) -> (Option<Outcome<StdVec<Value>>>) {
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

