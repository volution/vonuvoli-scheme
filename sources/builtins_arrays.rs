

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
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
	pub use super::{array_resize, array_clear};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_push, array_pop};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{array_insert_at, array_remove_at, array_swap_at};
	
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
pub fn array_collect <Source> (values : Source) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	return array_new (iter::FromIterator::from_iter (values)) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_collect_from_generator <Source> (values : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (array_new (values) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_collect_from_generator_ref <Source, ValueRef> (values : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	let values = vec_clone_vec_ref (&values);
	succeed! (array_new (values) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_empty () -> (Value) {
	return array_new_empty () .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_1 (value_1 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (value_1.clone ());
	return array_new (buffer) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	return array_new (buffer) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	return array_new (buffer) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (value_1.clone ());
	buffer.push (value_2.clone ());
	buffer.push (value_3.clone ());
	buffer.push (value_4.clone ());
	return array_new (buffer) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_build_n (values : &[&Value]) -> (Value) {
	if values.is_empty () {
		return array_empty ();
	}
	let mut buffer = StdVec::with_capacity (values.len ());
	for value in values {
		buffer.push ((*value).clone ());
	}
	return array_new (buffer) .into ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_2 (array_1 : &Value, array_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_2 (array_1, array_2));
	succeed! (array_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_3 (array_1 : &Value, array_2 : &Value, array_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_3 (array_1, array_2, array_3));
	succeed! (array_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_4 (array_1 : &Value, array_2 : &Value, array_3 : &Value, array_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_append_4 (array_1, array_2, array_3, array_4));
	succeed! (array_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_append_n (arrays : &[&Value]) -> (Outcome<Value>) {
	if arrays.is_empty () {
		succeed! (array_empty ());
	}
	let buffer = try! (vec_array_append_n (arrays));
	succeed! (array_new (buffer) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_clone (array : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_array_clone (array));
	succeed! (array_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_reverse (array : &Value) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let buffer = try! (vec_array_clone (array));
	succeed! (array_collect (buffer.into_iter () .rev ()));
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
pub fn array_clone_range (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let array = try_as_array_ref! (array);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, array.values_length ()));
	succeed! (array_clone_slice (& array.values_as_slice () [range_start..range_end]) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_range_to_list (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = try! (array_range_iterator (array, range_start, range_end));
	return list_collect_from_generator_ref (iterator, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_range_to_array (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return array_collect_from_generator_ref (iterator);
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
pub fn array_clear (array : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	array.clear ();
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
pub fn array_pop (array : &Value) -> (Outcome<Value>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let value = try_some! (array.pop (), 0x5b6e3f49);
	succeed! (value);
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_insert_at (array : &Value, index : &Value, value : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len () + 1));
	array.insert (index, value.clone ());
	succeed! (());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_remove_at (array : &Value, index : &Value) -> (Outcome<Value>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let index = try! (offset_coerce (index, array.len ()));
	let value = array.remove (index);
	succeed! (value);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_swap_at (array : &Value, left : &Value, right : &Value) -> (Outcome<()>) {
	let array = try_as_array_mutable_ref! (array);
	let mut array = try! (array.values_ref_mut ());
	let left = try! (offset_coerce (left, array.len ()));
	let right = try! (offset_coerce (right, array.len ()));
	array.swap (left, right);
	succeed! (());
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
pub fn vec_array_append_n (arrays : &[&Value]) -> (Outcome<ValueVec>) {
	if arrays.is_empty () {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	for array in arrays {
		try! (vec_array_drain (&mut buffer, &array));
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

