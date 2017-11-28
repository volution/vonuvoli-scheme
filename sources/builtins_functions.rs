

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{apply_n};
	pub use super::{lists_map_n, lists_iterate_n};
	pub use super::{arrays_map_n, arrays_iterate_n};
	pub use super::{bytes_map_n, bytes_iterate_n};
	pub use super::{strings_map_n, strings_iterate_n};
	
	pub use super::{values_build_1, values_build_2, values_build_3, values_build_4, values_build_n};
	
}




pub fn apply_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs = try! (vec_list_append_n (inputs));
	return evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, &inputs);
}




pub fn lists_map_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (ListIterators::new (lists));
	// FIXME:  Transform this into an iterator map!
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (list_collect (outputs));
}

pub fn lists_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (ListIterators::new (lists));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}


pub fn arrays_map_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (ArrayIterators::new (arrays));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (array_collect (outputs));
}

pub fn arrays_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (ArrayIterators::new (arrays));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}


pub fn bytes_map_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (BytesIterators::new (bytes));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return bytes_collect_values (outputs);
}

pub fn bytes_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (BytesIterators::new (bytes));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}


pub fn strings_map_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (StringIterators::new (strings));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return string_collect_values (outputs);
}

pub fn strings_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[Value]) -> (Outcome<Value>) {
	let iterators = try! (StringIterators::new (strings));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}


pub fn iterators_map_n <Iterators> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<ValueVec>)
		where Iterators : Iterator<Item = Outcome<StdVec<Value>>>
{
	let mut outputs = StdVec::new ();
	for inputs in iterators {
		let inputs_ref = try! (inputs);
		let mut inputs = StdVec::with_capacity (inputs_ref.len ());
		inputs.extend (inputs_ref.into_iter ());
		let output = try! (evaluator.evaluator.evaluate_procedure_call_with_values (evaluator, callable, inputs.as_ref ()));
		outputs.push (output);
	}
	succeed! (outputs);
}

pub fn iterators_iterate_n <Iterators> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<()>)
		where Iterators : Iterator<Item = Outcome<StdVec<Value>>>
{
	for inputs in iterators {
		let inputs_ref = try! (inputs);
		let mut inputs = StdVec::with_capacity (inputs_ref.len ());
		inputs.extend (inputs_ref.into_iter ());
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

