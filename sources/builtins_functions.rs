

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{apply_n};
	pub use super::{lists_map_1, lists_iterate_1, lists_map_n, lists_iterate_n};
	pub use super::{arrays_map_1, arrays_iterate_1, arrays_map_n, arrays_iterate_n};
	pub use super::{bytes_map_1, bytes_iterate_1, bytes_map_n, bytes_iterate_n};
	pub use super::{strings_map_1, strings_iterate_1, strings_map_n, strings_iterate_n};
	
	pub use super::{values_build_1, values_build_2, values_build_3, values_build_4, values_build_n};
	
}




pub fn apply_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs = try! (vec_list_append_n (inputs));
	return evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, &inputs);
}




pub fn lists_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, list : &Value) -> (Outcome<Value>) {
	if is_list_empty (list) {
		succeed! (list_empty ());
	}
	let iterator = try! (ListIterator::new (list));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	succeed! (list_collect (outputs));
}

pub fn lists_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, list : &Value) -> (Outcome<Value>) {
	if is_list_empty (list) {
		succeed! (VOID.into ());
	}
	let iterator = try! (ListIterator::new (list));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


pub fn lists_map_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[Value]) -> (Outcome<Value>) {
	match lists.len () {
		1 =>
			return lists_map_1 (evaluator, callable, &lists[0]),
		0 =>
			fail! (0x00de54c0),
		_ =>
			(),
	}
	let iterators = try! (ListIterators::new (lists));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (list_collect (outputs));
}

pub fn lists_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[Value]) -> (Outcome<Value>) {
	match lists.len () {
		1 =>
			return lists_iterate_1 (evaluator, callable, &lists[0]),
		0 =>
			fail! (0x1022d804),
		_ =>
			(),
	}
	let iterators = try! (ListIterators::new (lists));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




pub fn arrays_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, array : &Value) -> (Outcome<Value>) {
	if is_array_empty (array) {
		succeed! (array_empty ());
	}
	let iterator = try! (ArrayIterator::new (array));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	succeed! (array_collect (outputs));
}

pub fn arrays_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, array : &Value) -> (Outcome<Value>) {
	if is_array_empty (array) {
		succeed! (VOID.into ());
	}
	let iterator = try! (ArrayIterator::new (array));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


pub fn arrays_map_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[Value]) -> (Outcome<Value>) {
	match arrays.len () {
		1 =>
			return arrays_map_1 (evaluator, callable, &arrays[0]),
		0 =>
			fail! (0x0122b23a),
		_ =>
			(),
	}
	let iterators = try! (ArrayIterators::new (arrays));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (array_collect (outputs));
}

pub fn arrays_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[Value]) -> (Outcome<Value>) {
	match arrays.len () {
		1 =>
			return arrays_iterate_1 (evaluator, callable, &arrays[0]),
		0 =>
			fail! (0xe2d9384a),
		_ =>
			(),
	}
	let iterators = try! (ArrayIterators::new (arrays));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




pub fn bytes_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &Value) -> (Outcome<Value>) {
	if is_bytes_empty (bytes) {
		succeed! (bytes_empty ());
	}
	let iterator = try! (BytesIterator::new (bytes));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	return bytes_collect_values (outputs);
}

pub fn bytes_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &Value) -> (Outcome<Value>) {
	if is_bytes_empty (bytes) {
		succeed! (VOID.into ());
	}
	let iterator = try! (BytesIterator::new (bytes));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


pub fn bytes_map_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[Value]) -> (Outcome<Value>) {
	match bytes.len () {
		1 =>
			return bytes_map_1 (evaluator, callable, &bytes[0]),
		0 =>
			fail! (0xfa789f5a),
		_ =>
			(),
	}
	let iterators = try! (BytesIterators::new (bytes));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return bytes_collect_values (outputs);
}

pub fn bytes_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[Value]) -> (Outcome<Value>) {
	match bytes.len () {
		1 =>
			return bytes_iterate_1 (evaluator, callable, &bytes[0]),
		0 =>
			fail! (0xfff5829b),
		_ =>
			(),
	}
	let iterators = try! (BytesIterators::new (bytes));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




pub fn strings_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, string : &Value) -> (Outcome<Value>) {
	if is_string_empty (string) {
		succeed! (string_empty ());
	}
	let iterator = try! (StringIterator::new (string));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	return string_collect_values (outputs);
}

pub fn strings_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, string : &Value) -> (Outcome<Value>) {
	if is_string_empty (string) {
		succeed! (VOID.into ());
	}
	let iterator = try! (StringIterator::new (string));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


pub fn strings_map_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[Value]) -> (Outcome<Value>) {
	match strings.len () {
		1 =>
			return strings_map_1 (evaluator, callable, &strings[0]),
		0 =>
			fail! (0x75dac57b),
		_ =>
			(),
	}
	let iterators = try! (StringIterators::new (strings));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return string_collect_values (outputs);
}

pub fn strings_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[Value]) -> (Outcome<Value>) {
	match strings.len () {
		1 =>
			return strings_iterate_1 (evaluator, callable, &strings[0]),
		0 =>
			fail! (0x278c8e6c),
		_ =>
			(),
	}
	let iterators = try! (StringIterators::new (strings));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




pub fn iterators_map_1 <Iterator1> (evaluator : &mut EvaluatorContext, callable : &Value, iterator : Iterator1) -> (Outcome<ValueVec>)
		where Iterator1 : Iterator<Item = Outcome<Value>>
{
	let mut outputs = StdVec::new ();
	for input in iterator {
		let input = try! (input);
		let output = try! (evaluator.evaluator.evaluate_procedure_call_1_with_values (evaluator, callable, &input));
		outputs.push (output);
	}
	succeed! (outputs);
}

pub fn iterators_iterate_1 <Iterator1> (evaluator : &mut EvaluatorContext, callable : &Value, iterator : Iterator1) -> (Outcome<()>)
		where Iterator1 : Iterator<Item = Outcome<Value>>
{
	for input in iterator {
		let input = try! (input);
		try! (evaluator.evaluator.evaluate_procedure_call_1_with_values (evaluator, callable, &input));
	}
	succeed! (());
}




pub fn iterators_map_n <Iterators> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<ValueVec>)
		where Iterators : Iterator<Item = Outcome<StdVec<Value>>>
{
	let mut outputs = StdVec::new ();
	for inputs in iterators {
		let inputs = try! (inputs);
		let output = try! (evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, inputs.as_ref ()));
		outputs.push (output);
	}
	succeed! (outputs);
}

pub fn iterators_iterate_n <Iterators> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<()>)
		where Iterators : Iterator<Item = Outcome<StdVec<Value>>>
{
	for inputs in iterators {
		let inputs = try! (inputs);
		try! (evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, inputs.as_ref ()));
	}
	succeed! (());
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

