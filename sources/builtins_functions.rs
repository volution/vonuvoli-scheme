

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{apply_n};
	pub use super::{lists_map_n, lists_iterate_n};
	
	pub use super::{values_build_1, values_build_2, values_build_3, values_build_4, values_build_n};
	
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

