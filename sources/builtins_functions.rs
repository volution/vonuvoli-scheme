

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{call_with_values, call_with_values_builder};
	pub use super::{call_0, call_1, call_2, call_3, call_4, call_n};
	pub use super::{apply_0, apply_1, apply_2, apply_3, apply_4, apply_n};
	
	pub use super::{lists_map_1, lists_map_2, lists_map_3, lists_map_4, lists_map_n};
	pub use super::{lists_iterate_1, lists_iterate_2, lists_iterate_3, lists_iterate_4, lists_iterate_n};
	
	pub use super::{arrays_map_1, arrays_map_2, arrays_map_3, arrays_map_4, arrays_map_n};
	pub use super::{arrays_iterate_1, arrays_iterate_2, arrays_iterate_3, arrays_iterate_4, arrays_iterate_n};
	
	pub use super::{bytes_map_1, bytes_map_2, bytes_map_3, bytes_map_4, bytes_map_n};
	pub use super::{bytes_iterate_1, bytes_iterate_2, bytes_iterate_3, bytes_iterate_4, bytes_iterate_n};
	
	pub use super::{strings_map_1, strings_map_2, strings_map_3, strings_map_4, strings_map_n};
	pub use super::{strings_iterate_1, strings_iterate_2, strings_iterate_3, strings_iterate_4, strings_iterate_n};
	
	pub use super::{values_build_0, values_build_1, values_build_2, values_build_3, values_build_4, values_build_n};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_with_values (evaluator : &mut EvaluatorContext, callable : &Value, values : &Value) -> (Outcome<Value>) {
	let values = try_as_values_ref! (values);
	let values = vec_slice_to_ref (values.values_as_slice ());
	return evaluator.evaluate_procedure_call_n (callable, &values);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_with_values_builder (evaluator : &mut EvaluatorContext, callable : &Value, builder : &Value) -> (Outcome<Value>) {
	let values = try! (evaluator.evaluate_procedure_call_0 (builder));
	return call_with_values (evaluator, callable, &values);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_0 (evaluator : &mut EvaluatorContext, callable : &Value) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_0 (callable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_1 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_1 (callable, input_1);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_2 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_2 (callable, input_1, input_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_3 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_3 (callable, input_1, input_2, input_3);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_4 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_4 (callable, input_1, input_2, input_3, input_4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn call_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
	return evaluator.evaluate_procedure_call_n (callable, inputs);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_0 (evaluator : &mut EvaluatorContext, callable : &Value) -> (Outcome<Value>) {
	return call_0 (evaluator, callable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_1 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
	let inputs = try! (vec_list_ref_clone (input_1));
	return call_n (evaluator, callable, &inputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_2 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	let inputs = try! (vec_list_ref_append_2 (input_1, input_2));
	return call_n (evaluator, callable, &inputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_3 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	let inputs = try! (vec_list_ref_append_3 (input_1, input_2, input_3));
	return call_n (evaluator, callable, &inputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_4 (evaluator : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	let inputs = try! (vec_list_ref_append_4 (input_1, input_2, input_3, input_4));
	return call_n (evaluator, callable, &inputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn apply_n (evaluator : &mut EvaluatorContext, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
	let inputs = try! (vec_list_ref_append_n (inputs));
	return call_n (evaluator, callable, &inputs);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value) -> (Outcome<Value>) {
	if is_list_empty (list_1) {
		succeed! (list_empty ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator_1));
	succeed! (list_collect (outputs, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value) -> (Outcome<Value>) {
	if is_list_empty (list_1) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	try! (iterators_iterate_1 (evaluator, callable, iterator_1));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_map_2 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_2 (list_1, list_2) {
		succeed! (list_empty ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	let outputs = try! (iterators_map_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (list_collect (outputs, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_iterate_2 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_2 (list_1, list_2) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	try! (iterators_iterate_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_map_3 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_3 (list_1, list_2, list_3) {
		succeed! (list_empty ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	let iterator_3 = try! (ListIterator::new (list_3));
	let outputs = try! (iterators_map_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (list_collect (outputs, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_iterate_3 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_3 (list_1, list_2, list_3) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	let iterator_3 = try! (ListIterator::new (list_3));
	try! (iterators_iterate_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_map_4 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_4 (list_1, list_2, list_3, list_4) {
		succeed! (list_empty ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	let iterator_3 = try! (ListIterator::new (list_3));
	let iterator_4 = try! (ListIterator::new (list_4));
	let outputs = try! (iterators_map_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (list_collect (outputs, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_iterate_4 (evaluator : &mut EvaluatorContext, callable : &Value, list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<Value>) {
	if is_list_empty_all_4 (list_1, list_2, list_3, list_4) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ListIterator::new (list_1));
	let iterator_2 = try! (ListIterator::new (list_2));
	let iterator_3 = try! (ListIterator::new (list_3));
	let iterator_4 = try! (ListIterator::new (list_4));
	try! (iterators_iterate_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_map_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[&Value]) -> (Outcome<Value>) {
	if lists.is_empty () {
		fail! (0x00de54c0);
	}
	let iterators = try! (ListIterators::new (lists));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (list_collect (outputs, None));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn lists_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, lists : &[&Value]) -> (Outcome<Value>) {
	if lists.is_empty () {
		fail! (0x1022d804);
	}
	let iterators = try! (ListIterators::new (lists));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, array : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty (array)) {
		succeed! (array_empty ());
	}
	let iterator = try! (ArrayIterator::new (array));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	succeed! (array_collect (outputs));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, array : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty (array)) {
		succeed! (VOID.into ());
	}
	let iterator = try! (ArrayIterator::new (array));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_map_2 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_2 (array_1, array_2)) {
		succeed! (array_empty ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	let outputs = try! (iterators_map_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (array_collect (outputs));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_iterate_2 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_2 (array_1, array_2)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	try! (iterators_iterate_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_map_3 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_3 (array_1, array_2, array_3)) {
		succeed! (array_empty ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	let iterator_3 = try! (ArrayIterator::new (array_3));
	let outputs = try! (iterators_map_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (array_collect (outputs));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_iterate_3 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_3 (array_1, array_2, array_3)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	let iterator_3 = try! (ArrayIterator::new (array_3));
	try! (iterators_iterate_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_map_4 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_4 (array_1, array_2, array_3, array_4)) {
		succeed! (array_empty ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	let iterator_3 = try! (ArrayIterator::new (array_3));
	let iterator_4 = try! (ArrayIterator::new (array_4));
	let outputs = try! (iterators_map_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (array_collect (outputs));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_iterate_4 (evaluator : &mut EvaluatorContext, callable : &Value, array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<Value>) {
	if try! (is_array_empty_all_4 (array_1, array_2, array_3, array_4)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (ArrayIterator::new (array_1));
	let iterator_2 = try! (ArrayIterator::new (array_2));
	let iterator_3 = try! (ArrayIterator::new (array_3));
	let iterator_4 = try! (ArrayIterator::new (array_4));
	try! (iterators_iterate_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_map_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[&Value]) -> (Outcome<Value>) {
	if arrays.is_empty () {
		fail! (0x0122b23a);
	}
	let iterators = try! (ArrayIterators::new (arrays));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	succeed! (array_collect (outputs));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arrays_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, arrays : &[&Value]) -> (Outcome<Value>) {
	if arrays.is_empty () {
		fail! (0xe2d9384a);
	}
	let iterators = try! (ArrayIterators::new (arrays));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty (bytes)) {
		succeed! (bytes_empty ());
	}
	let iterator = try! (BytesIterator::new (bytes));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	return bytes_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty (bytes)) {
		succeed! (VOID.into ());
	}
	let iterator = try! (BytesIterator::new (bytes));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_map_2 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_2 (bytes_1, bytes_2)) {
		succeed! (bytes_empty ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	let outputs = try! (iterators_map_2 (evaluator, callable, iterator_1, iterator_2));
	return bytes_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_iterate_2 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_2 (bytes_1, bytes_2)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	try! (iterators_iterate_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_map_3 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_3 (bytes_1, bytes_2, bytes_3)) {
		succeed! (bytes_empty ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	let iterator_3 = try! (BytesIterator::new (bytes_3));
	let outputs = try! (iterators_map_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	return bytes_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_iterate_3 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_3 (bytes_1, bytes_2, bytes_3)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	let iterator_3 = try! (BytesIterator::new (bytes_3));
	try! (iterators_iterate_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_map_4 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_4 (bytes_1, bytes_2, bytes_3, bytes_4)) {
		succeed! (bytes_empty ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	let iterator_3 = try! (BytesIterator::new (bytes_3));
	let iterator_4 = try! (BytesIterator::new (bytes_4));
	let outputs = try! (iterators_map_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	return bytes_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_iterate_4 (evaluator : &mut EvaluatorContext, callable : &Value, bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<Value>) {
	if try! (is_bytes_empty_all_4 (bytes_1, bytes_2, bytes_3, bytes_4)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (BytesIterator::new (bytes_1));
	let iterator_2 = try! (BytesIterator::new (bytes_2));
	let iterator_3 = try! (BytesIterator::new (bytes_3));
	let iterator_4 = try! (BytesIterator::new (bytes_4));
	try! (iterators_iterate_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_map_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[&Value]) -> (Outcome<Value>) {
	if bytes.is_empty () {
		fail! (0xfa789f5a);
	}
	let iterators = try! (BytesIterators::new (bytes));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return bytes_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, bytes : &[&Value]) -> (Outcome<Value>) {
	if bytes.is_empty () {
		fail! (0xfff5829b);
	}
	let iterators = try! (BytesIterators::new (bytes));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_map_1 (evaluator : &mut EvaluatorContext, callable : &Value, string : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty (string)) {
		succeed! (string_empty ());
	}
	let iterator = try! (StringIterator::new (string));
	let outputs = try! (iterators_map_1 (evaluator, callable, iterator));
	return string_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_iterate_1 (evaluator : &mut EvaluatorContext, callable : &Value, string : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty (string)) {
		succeed! (VOID.into ());
	}
	let iterator = try! (StringIterator::new (string));
	try! (iterators_iterate_1 (evaluator, callable, iterator));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_map_2 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_2 (string_1, string_2)) {
		succeed! (string_empty ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	let outputs = try! (iterators_map_2 (evaluator, callable, iterator_1, iterator_2));
	return string_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_iterate_2 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_2 (string_1, string_2)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	try! (iterators_iterate_2 (evaluator, callable, iterator_1, iterator_2));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_map_3 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value, string_3 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_3 (string_1, string_2, string_3)) {
		succeed! (string_empty ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	let iterator_3 = try! (StringIterator::new (string_3));
	let outputs = try! (iterators_map_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	return string_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_iterate_3 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value, string_3 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_3 (string_1, string_2, string_3)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	let iterator_3 = try! (StringIterator::new (string_3));
	try! (iterators_iterate_3 (evaluator, callable, iterator_1, iterator_2, iterator_3));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_map_4 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value, string_3 : &Value, string_4 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_4 (string_1, string_2, string_3, string_4)) {
		succeed! (string_empty ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	let iterator_3 = try! (StringIterator::new (string_3));
	let iterator_4 = try! (StringIterator::new (string_4));
	let outputs = try! (iterators_map_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	return string_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_iterate_4 (evaluator : &mut EvaluatorContext, callable : &Value, string_1 : &Value, string_2 : &Value, string_3 : &Value, string_4 : &Value) -> (Outcome<Value>) {
	if try! (is_string_empty_all_4 (string_1, string_2, string_3, string_4)) {
		succeed! (VOID.into ());
	}
	let iterator_1 = try! (StringIterator::new (string_1));
	let iterator_2 = try! (StringIterator::new (string_2));
	let iterator_3 = try! (StringIterator::new (string_3));
	let iterator_4 = try! (StringIterator::new (string_4));
	try! (iterators_iterate_4 (evaluator, callable, iterator_1, iterator_2, iterator_3, iterator_4));
	succeed! (VOID.into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_map_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[&Value]) -> (Outcome<Value>) {
	if strings.is_empty () {
		fail! (0x75dac57b);
	}
	let iterators = try! (StringIterators::new (strings));
	let outputs = try! (iterators_map_n (evaluator, callable, iterators));
	return string_collect_values (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn strings_iterate_n (evaluator : &mut EvaluatorContext, callable : &Value, strings : &[&Value]) -> (Outcome<Value>) {
	if strings.is_empty () {
		fail! (0x278c8e6c);
	}
	let iterators = try! (StringIterators::new (strings));
	try! (iterators_iterate_n (evaluator, callable, iterators));
	succeed! (VOID.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_map_1 <Iterator1, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1) -> (Outcome<ValueVec>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut outputs = StdVec::new ();
	for input_1 in iterator_1 {
		let input_1 = try! (input_1);
		let input_1 = input_1.as_ref ();
		let output = try! (evaluator.evaluate_procedure_call_1 (callable, input_1));
		outputs.push (output);
	}
	succeed! (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_iterate_1 <Iterator1, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1) -> (Outcome<()>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	for input_1 in iterator_1 {
		let input_1 = try! (input_1);
		let input_1 = input_1.as_ref ();
		try! (evaluator.evaluate_procedure_call_1 (callable, input_1));
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_map_2 <Iterator1, Iterator2, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2) -> (Outcome<ValueVec>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut outputs = StdVec::new ();
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		let output = try! (evaluator.evaluate_procedure_call_2 (callable, input_1, input_2));
		outputs.push (output);
	}
	succeed! (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_iterate_2 <Iterator1, Iterator2, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2) -> (Outcome<()>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		try! (evaluator.evaluate_procedure_call_2 (callable, input_1, input_2));
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_map_3 <Iterator1, Iterator2, Iterator3, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2, iterator_3 : Iterator3) -> (Outcome<ValueVec>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator3 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut outputs = StdVec::new ();
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	let mut iterator_3 = iterator_3;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_3 = iterator_3.next (); if input_3.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_3 = try! (input_3.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		let input_3 = input_3.as_ref ();
		let output = try! (evaluator.evaluate_procedure_call_3 (callable, input_1, input_2, input_3));
		outputs.push (output);
	}
	succeed! (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_iterate_3 <Iterator1, Iterator2, Iterator3, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2, iterator_3 : Iterator3) -> (Outcome<()>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator3 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	let mut iterator_3 = iterator_3;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_3 = iterator_3.next (); if input_3.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_3 = try! (input_3.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		let input_3 = input_3.as_ref ();
		try! (evaluator.evaluate_procedure_call_3 (callable, input_1, input_2, input_3));
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_map_4 <Iterator1, Iterator2, Iterator3, Iterator4, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2, iterator_3 : Iterator3, iterator_4 : Iterator4) -> (Outcome<ValueVec>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator3 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator4 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut outputs = StdVec::new ();
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	let mut iterator_3 = iterator_3;
	let mut iterator_4 = iterator_4;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_3 = iterator_3.next (); if input_3.is_none () { break; }
		let input_4 = iterator_4.next (); if input_4.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_3 = try! (input_3.unwrap ());
		let input_4 = try! (input_4.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		let input_3 = input_3.as_ref ();
		let input_4 = input_4.as_ref ();
		let output = try! (evaluator.evaluate_procedure_call_4 (callable, input_1, input_2, input_3, input_4));
		outputs.push (output);
	}
	succeed! (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_iterate_4 <Iterator1, Iterator2, Iterator3, Iterator4, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterator_1 : Iterator1, iterator_2 : Iterator2, iterator_3 : Iterator3, iterator_4 : Iterator4) -> (Outcome<()>)
		where Iterator1 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator2 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator3 : iter::Iterator<Item = Outcome<ValueRef>>, Iterator4 : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let mut iterator_1 = iterator_1;
	let mut iterator_2 = iterator_2;
	let mut iterator_3 = iterator_3;
	let mut iterator_4 = iterator_4;
	loop {
		let input_1 = iterator_1.next (); if input_1.is_none () { break; }
		let input_2 = iterator_2.next (); if input_2.is_none () { break; }
		let input_3 = iterator_3.next (); if input_3.is_none () { break; }
		let input_4 = iterator_4.next (); if input_4.is_none () { break; }
		let input_1 = try! (input_1.unwrap ());
		let input_2 = try! (input_2.unwrap ());
		let input_3 = try! (input_3.unwrap ());
		let input_4 = try! (input_4.unwrap ());
		let input_1 = input_1.as_ref ();
		let input_2 = input_2.as_ref ();
		let input_3 = input_3.as_ref ();
		let input_4 = input_4.as_ref ();
		try! (evaluator.evaluate_procedure_call_4 (callable, input_1, input_2, &input_3, &input_4));
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_map_n <Iterators, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<ValueVec>)
		where Iterators : iter::Iterator<Item = Outcome<StdVec<ValueRef>>>, ValueRef : StdAsRef<Value>
{
	let mut outputs = StdVec::new ();
	for inputs in iterators {
		let inputs = try! (inputs);
		let inputs = vec_vec_to_ref (&inputs);
		let output = try! (evaluator.evaluate_procedure_call_n (callable, &inputs));
		outputs.push (output);
	}
	succeed! (outputs);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn iterators_iterate_n <Iterators, ValueRef> (evaluator : &mut EvaluatorContext, callable : &Value, iterators : Iterators) -> (Outcome<()>)
		where Iterators : iter::Iterator<Item = Outcome<StdVec<ValueRef>>>, ValueRef : StdAsRef<Value>
{
	for inputs in iterators {
		let inputs = try! (inputs);
		let inputs = vec_vec_to_ref (&inputs);
		try! (evaluator.evaluate_procedure_call_n (callable, &inputs));
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_0 () -> (Value) {
	return values_new (StdBox::new ([])) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_1 (value_1 : &Value) -> (Value) {
	return values_new (StdBox::new ([value_1.clone ()])) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	return values_new (StdBox::new ([value_1.clone (), value_2.clone ()])) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	return values_new (StdBox::new ([value_1.clone (), value_2.clone (), value_3.clone ()])) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	return values_new (StdBox::new ([value_1.clone (), value_2.clone (), value_3.clone (), value_4.clone ()])) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn values_build_n (values : &[&Value]) -> (Value) {
	return values_clone_slice_ref (values) .into ();
}

