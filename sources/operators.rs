

use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::{pair};
	pub use super::{list_build_1, list_build_2, list_build_3, list_build_4, list_build_n};
	pub use super::{list_append_2, list_append_3, list_append_4, list_append_n};
	pub use super::{vec_list_append_2, vec_list_append_3, vec_list_append_4, vec_list_append_n};
	pub use super::{vec_list_append_2_dotted, vec_list_append_3_dotted, vec_list_append_4_dotted, vec_list_append_n_dotted};
	pub use super::{vec_list_clone, vec_list_clone_dotted, vec_list_drain, vec_list_drain_dotted};
	pub use super::{ListIterator, ListsIterator};
	pub use super::{is_true, is_false, is_not_false, is_true_or_equivalent, is_false_or_equivalent};
	pub use super::{apply_n};
	pub use super::{lists_map_n, lists_iterate_n};
	pub use super::{values_build_1, values_build_2, values_build_3, values_build_4, values_build_n};
}




pub fn pair (left : &Value, right : &Value) -> (Value) {
	pair_new (left.clone (), right.clone ()) .into ()
}




pub fn list_build_1 (value_1 : &Value) -> (Value) {
	pair_new (value_1.clone (), NULL) .into ()
}

pub fn list_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), NULL) .into ()) .into ()
}

pub fn list_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), NULL) .into ()) .into ()) .into ()
}

pub fn list_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), pair_new (value_4.clone (), NULL) .into ()) .into ()) .into ()) .into ()
}

pub fn list_build_n (values : &[Value]) -> (Value) {
	if values.is_empty () {
		NULL
	} else {
		values.iter () .rev () .fold (NULL, |last, value| Value::Pair (pair_new (value.clone (), last)))
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
	if (*list_1 == NULL) && (*list_2 == NULL) {
		succeed! ((StdVec::new (), None));
	}
	let mut values = ValueVec::new ();
	try! (vec_list_drain (&mut values, &list_1));
	let last = try! (vec_list_drain_dotted (&mut values, &list_2));
	succeed! ((values, last));
}

pub fn vec_list_append_3_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if (*list_1 == NULL) && (*list_2 == NULL) && (*list_3 == NULL) {
		succeed! ((StdVec::new (), None));
	}
	let mut values = ValueVec::new ();
	try! (vec_list_drain (&mut values, &list_1));
	try! (vec_list_drain (&mut values, &list_2));
	let last = try! (vec_list_drain_dotted (&mut values, &list_3));
	succeed! ((values, last));
}

pub fn vec_list_append_4_dotted (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if (*list_1 == NULL) && (*list_2 == NULL) && (*list_3 == NULL) && (*list_4 == NULL) {
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
		match cursor {
			&Value::Pair (ref pair) => {
				vector.push (pair.left () .clone ());
				cursor = pair.right ();
			},
			&Value::Null =>
				succeed! (None),
			ref value =>
				succeed! (Some ((*value).clone ())),
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
		let (cursor, value) = match self.0 {
			&Value::Pair (ref pair) =>
				(
					pair.right (),
					pair.left (),
				),
			&Value::Null =>
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




pub fn is_true (value : &Value) -> (bool) {
	*value == TRUE.into ()
}

pub fn is_false (value : &Value) -> (bool) {
	*value == FALSE.into ()
}

pub fn is_not_false (value : &Value) -> (bool) {
	*value != FALSE.into ()
}

pub fn is_true_or_equivalent (value : &Value) -> (bool) {
	!is_false_or_equivalent (value)
}

pub fn is_false_or_equivalent (value : &Value) -> (bool) {
	match *value {
		Value::Null | Value::Void | Value::Undefined =>
			true,
		Value::Boolean (FALSE) =>
			true,
		Value::Error (_) =>
			true,
		_ =>
			false,
	}
}




pub fn apply_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs = try! (vec_list_append_n (inputs));
	return evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, &inputs);
}




pub fn lists_map_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
	let lists_iterator = ListsIterator::new (inputs);
	let mut outputs = StdVec::new ();
	for inputs in lists_iterator {
		let inputs_ref = try! (inputs);
		let mut inputs = StdVec::with_capacity (inputs_ref.len ());
		inputs.extend (inputs_ref.into_iter () .cloned ());
		let output = try! (evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, inputs.as_ref ()));
		outputs.push (output);
	}
	succeed! (list_new (outputs));
}

pub fn lists_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
	let lists_iterator = ListsIterator::new (inputs);
	for inputs in lists_iterator {
		let inputs_ref = try! (inputs);
		let mut inputs = StdVec::with_capacity (inputs_ref.len ());
		inputs.extend (inputs_ref.into_iter () .cloned ());
		try! (evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, inputs.as_ref ()));
	}
	succeed! (VOID.into ());
}




pub fn values_build_1 (_value_1 : &Value) -> (Value) {
	panic! ("bb8da879");
}

pub fn values_build_2 (_value_1 : &Value, _value_2 : &Value) -> (Value) {
	panic! ("1bb069bf");
}

pub fn values_build_3 (_value_1 : &Value, _value_2 : &Value, _value_3 : &Value) -> (Value) {
	panic! ("a60e100f");
}

pub fn values_build_4 (_value_1 : &Value, _value_2 : &Value, _value_3 : &Value, _value_4 : &Value) -> (Value) {
	panic! ("2474f5ff");
}

pub fn values_build_n (_values : &[Value]) -> (Value) {
	panic! ("cea42387");
}

