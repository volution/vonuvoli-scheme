

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{array_at};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_at_set};
	
	pub use super::{array_empty};
	pub use super::{array_collect};
	pub use super::{array_collect_from_generator, array_collect_from_generator_ref};
	pub use super::{array_build_1, array_build_2, array_build_3, array_build_4, array_build_n};
	pub use super::{array_append_2, array_append_3, array_append_4, array_append_n};
	pub use super::{array_make, array_clone, array_reverse};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_fill_range, array_reverse_range, array_copy_range, array_extend_range};
	pub use super::{array_clone_range};
	pub use super::{array_range_to_list, list_range_to_array};
	pub use super::{array_range_iterator};
	pub use super::{array_length};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_resize, array_resize_at, array_clear, array_clear_at};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_push, array_push_n, array_push_from, array_pop};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::{array_pop_n};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_insert, array_insert_n, array_insert_from, array_remove};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::{array_remove_n};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_swap};
	
	pub use super::{array_find};
	
	pub use super::{vec_array_append_2, vec_array_append_3, vec_array_append_4, vec_array_append_n};
	pub use super::{vec_array_clone, vec_array_drain};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_at (array : &Value, index : usize) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let array = array.values_as_slice ();
	if let Some (value) = array.get (index) {
		succeed! (value.clone ());
	} else {
		fail! (0x40bcc72e);
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_at_set (array : &Value, index : usize, value : &Value) -> (Outcome<Value>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let value_ref = try_some! (array.get_mut (index), 0x51cf23d4);
	let mut value_swap = value.clone ();
	mem::swap (&mut value_swap, value_ref);
	succeed! (value_swap);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_collect <Source> (values : Source, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	return array_new (iter::FromIterator::from_iter (values), immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_collect_from_generator <Source> (values : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (array_new (values, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_collect_from_generator_ref <Source, ValueRef> (values : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	let values = vec_clone_vec_ref (&values);
	succeed! (array_new (values, immutable));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_empty (immutable : Option<bool>) -> (Value) {
	return array_new_empty (immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_1 (value_1 : &Value, immutable : Option<bool>) -> (Value) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (value_1.clone ());
	return array_new (buffer, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_2 (value_1 : &Value, value_2 : &Value, immutable : Option<bool>) -> (Value) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	return array_new (buffer, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value, immutable : Option<bool>) -> (Value) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	return array_new (buffer, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value, immutable : Option<bool>) -> (Value) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	buffer.push (value_4.clone ());
	return array_new (buffer, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_n (values : &[impl StdAsRef<Value>], immutable : Option<bool>) -> (Value) {
	if values.is_empty () {
		return array_empty (immutable);
	}
	let mut buffer = StdVec::with_capacity (values.len ());
	for value in values {
		let value = value.as_ref ();
		buffer.push (value.clone ());
	}
	return array_new (buffer, immutable);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_2 (array_1 : &Value, array_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_2 (array_1, array_2));
	succeed! (array_new (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_3 (array_1 : &Value, array_2 : &Value, array_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_3 (array_1, array_2, array_3));
	succeed! (array_new (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_4 (array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_4 (array_1, array_2, array_3, array_4));
	succeed! (array_new (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_n (arrays : &[impl StdAsRef<Value>], immutable : Option<bool>) -> (Outcome<Value>) {
	if arrays.is_empty () {
		succeed! (array_empty (immutable));
	}
	let buffer = try! (vec_array_append_n (arrays));
	succeed! (array_new (buffer, immutable));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_make (length : usize, fill : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill.clone ());
	}
	succeed! (array_new (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clone (array : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = try! (vec_array_clone (array));
	succeed! (array_new (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_reverse (array : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let buffer = try! (vec_array_clone (array));
	succeed! (array_collect (buffer.into_iter () .rev (), immutable));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_fill_range (array : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let fill = if let Some (fill) = fill {
		fill
	} else {
		&UNDEFINED_VALUE
	};
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.len ()));
	let array = try_some! (array.get_mut (range_start .. range_end), 0xa0064b49);
	for value_ref in array {
		*value_ref = fill.clone ();
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_reverse_range (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.len ()));
	let array = try_some! (array.get_mut (range_start .. range_end), 0xa3cf0255);
	array.reverse ();
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_copy_range (target_array : &Value, target_start : Option<&Value>, source_array : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<()>) {
	let target_array = try_as_array_mutable_ref! (target_array);
	let mut target_array = try! (target_array.values_ref_mut ());
	let source_array = try_as_array_ref! (source_array);
	let source_array = source_array.values_as_slice ();
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_array.len ()));
	let (target_start, target_end) = try! (range_coerce (target_start, None, target_array.len ()));
	let target_end = if (target_end - target_start) >= (source_end - source_start) {
		target_start + (source_end - source_start)
	} else {
		fail! (0x18f863a1);
	};
	let target_array = try_some! (target_array.get_mut (target_start .. target_end), 0x333deb75);
	let source_array = try_some! (source_array.get (source_start .. source_end), 0xe3774a7e);
	<[Value]>::clone_from_slice (target_array, source_array);
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_extend_range (target_array : &Value, target_start : Option<&Value>, source_array : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<()>) {
	let target_array = try_as_array_mutable_ref! (target_array);
	let mut target_array = try! (target_array.values_ref_mut ());
	let source_array = try_as_array_ref! (source_array);
	let source_array = source_array.values_as_slice ();
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_array.len ()));
	let target_start = try! (offset_coerce_option (target_start, target_array.len () + 1));
	let source_array = try_some! (source_array.get (source_start .. source_end), 0x533b9838);
	if let Some (target_start) = target_start {
		target_array.reserve (source_array.len ());
		for (index, value) in source_array.iter () .enumerate () {
			target_array.insert (target_start + index, value.clone ());
		}
	} else {
		target_array.extend_from_slice (source_array);
	};
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clone_range (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	succeed! (array_clone_slice (& array.values_as_slice () [range_start..range_end], immutable));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_range_to_list (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = try! (array_range_iterator (array, range_start, range_end));
	return list_collect_from_generator_ref (iterator, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_range_to_array (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return array_collect_from_generator_ref (iterator, immutable);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_range_iterator <'a> (array : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<&'a Value, ArrayIterator<'a>>>) {
	let array = try_as_array_ref! (array);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	let iterator = try! (ArrayIterator::new_a (array));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, Some (range_end)));
	succeed! (iterator);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_length (array : &Value) -> (Outcome<usize>) {
	let array = try_as_array_ref! (array);
	succeed! (array.values_length ());
}





#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_resize (array : &Value, size : &Value, fill : Option<&Value>) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let size = try! (count_coerce (size));
	if size == 0 {
		array.clear ();
		succeed! (());
	}
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	array.resize (size, fill);
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_resize_at (array : &Value, index : &Value, count : &Value, fill : Option<&Value>) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	let count = try! (count_coerce (count));
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	
	if count == 0 {
		succeed! (());
	}
	let size_old = array.len ();
	let size_new = size_old + count;
	if index == size_old {
		array.resize (size_new, fill);
	} else {
		array.resize (size_new, fill);
		let moved = size_old - index;
		for offset in ((size_new - moved) .. size_new) .rev () {
			array.swap (offset, offset - count);
		}
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clear (array : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	array.clear ();
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clear_at (array : &Value, index : &Value, count : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	let count = try! (count_coerce (count));
	
	if count == 0 {
		succeed! (());
	}
	let size_old = array.len ();
	if (size_old - index) < count {
		fail! (0xebcb1ff9);
	}
	let size_new = size_old - count;
	if size_new == 0 {
		array.clear ();
	} else {
		let moved = size_new - index;
		for offset in (size_old - moved) .. size_old {
			array.swap (offset, offset - count);
		}
		array.truncate (size_new);
	}
	succeed! (());
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_push (array : &Value, value : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	array.push (value.clone ());
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_push_n (array : &Value, values : &[impl StdAsRef<Value>]) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	array.reserve (values.len ());
	for value in values {
		array.push (value.as_ref () .clone ());
	}
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_push_from (array : &Value, values : &Value) -> (Outcome<()>) {
	TODO! ("add support for other sequence like values");
	let values = try_as_array_ref! (values);
	let values = values.values_as_slice ();
	return array_push_n (array, values);
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_pop (array : &Value) -> (Outcome<Value>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let value = try_some! (array.pop (), 0x5b6e3f49);
	succeed! (value);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg ( feature = "vonuvoli_values_values" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_pop_n (array : &Value, count : &Value) -> (Outcome<Values>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let count = try! (count_coerce (count));
	let size_old = array.len ();
	if size_old < count {
		fail! (0xcdd0f423);
	}
	let mut values = StdVec::with_capacity (count);
	values.resize (count, UNDEFINED_VALUE);
	<[Value]>::swap_with_slice (&mut array.as_mut_slice () [(size_old - count) ..], values.as_mut_slice ());
	let size_new = size_old - count;
	array.truncate (size_new);
	let values = values_new (values.into_boxed_slice ());
	succeed! (values);
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_insert (array : &Value, index : &Value, value : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	array.insert (index, value.clone ());
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_insert_n (array : &Value, index : &Value, values : &[impl StdAsRef<Value>]) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	let count = values.len ();
	
	if count == 0 {
		succeed! (());
	}
	let size_old = array.len ();
	let size_new = size_old + count;
	if index == size_old {
		array.reserve (count);
		for value in values {
			array.push (value.as_ref () .clone ());
		}
	} else {
		array.resize (size_new, UNDEFINED_VALUE);
		let moved = size_old - index;
		for offset in ((size_new - moved) .. size_new) .rev () {
			array.swap (offset, offset - count);
		}
		for offset in index .. index + count {
			array[offset] = values[offset - index].as_ref () .clone ();
		}
	}
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_insert_from (array : &Value, index : &Value, values : &Value) -> (Outcome<()>) {
	TODO! ("add support for other sequence like values");
	let values = try_as_array_ref! (values);
	let values = values.values_as_slice ();
	return array_insert_n (array, index, values);
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_remove (array : &Value, index : &Value) -> (Outcome<Value>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len ()));
	let value = array.remove (index);
	succeed! (value);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg ( feature = "vonuvoli_values_values" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_remove_n (array : &Value, index : &Value, count : &Value) -> (Outcome<Values>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	let count = try! (count_coerce (count));
	
	if count == 0 {
		succeed! (values_new_empty ());
	}
	
	let mut values = StdVec::with_capacity (count);
	values.resize (count, UNDEFINED_VALUE);
	<[Value]>::swap_with_slice (&mut array.as_mut_slice () [index .. (index + count)], values.as_mut_slice ());
	let values = values_new (values.into_boxed_slice ());
	
	let size_old = array.len ();
	if (size_old - index) < count {
		fail! (0x6fabec26);
	}
	let size_new = size_old - count;
	if size_new == 0 {
		array.clear ();
	} else {
		let moved = size_new - index;
		for offset in (size_old - moved) .. size_old {
			array.swap (offset, offset - count);
		}
		array.truncate (size_new);
	}
	succeed! (values);
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_swap (array : &Value, left : &Value, right : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let left = try! (offset_coerce (left, array.len ()));
	let right = try! (offset_coerce (right, array.len ()));
	array.swap (left, right);
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_find (array : &Value, predicate : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ArrayIterator::new (array));
	loop {
		match iterator.next () {
			Some (Ok (value)) => {
				let comparison = try! (evaluator.evaluate_procedure_call_1 (predicate, &value));
				if is_not_false (&comparison) {
					succeed! (value.clone ());
				}
			},
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_array_append_2 (array_1 : &Value, array_2 : &Value) -> (Outcome<ValueVec>) {
	if try! (is_array_empty_all_2 (array_1, array_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, &array_1));
	try! (vec_array_drain (&mut buffer, &array_2));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_array_append_n (arrays : &[impl StdAsRef<Value>]) -> (Outcome<ValueVec>) {
	if arrays.is_empty () {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	for array in arrays {
		let array = array.as_ref ();
		try! (vec_array_drain (&mut buffer, array));
	}
	succeed! (buffer);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_array_clone (array : &Value) -> (Outcome<ValueVec>) {
	let mut buffer = StdVec::new ();
	try! (vec_array_drain (&mut buffer, array));
	succeed! (buffer);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_array_drain (buffer : &mut ValueVec, array : &Value) -> (Outcome<()>) {
	let array = try_as_array_ref! (array);
	buffer.extend_from_slice (array.values_as_slice ());
	succeed! (());
}

