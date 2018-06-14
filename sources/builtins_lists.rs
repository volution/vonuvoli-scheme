

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{pair};
	pub use super::{pair_left, pair_right};
	pub use super::{pair_left_ref, pair_right_ref};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{pair_left_set, pair_right_set};
	
	pub use super::{pair_left as list_first, pair_right as list_rest};
	pub use super::{pair_left_ref as list_first_ref, pair_right_ref as list_rest_ref};
	pub use super::{list_first_at, list_rest_at};
	pub use super::{list_first_at_ref, list_rest_at_ref};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{list_first_at_set, list_rest_at_set};
	pub use super::{list_pair_at, list_pair_at_ref};
	
	pub use super::{list_empty};
	pub use super::{list_collect, list_collect_dotted};
	pub use super::{list_collect_ref, list_collect_dotted_ref};
	pub use super::{list_collect_from_generator, list_collect_dotted_from_generator};
	pub use super::{list_collect_from_generator_ref, list_collect_dotted_from_generator_ref};
	pub use super::{list_build_1, list_build_2, list_build_3, list_build_4, list_build_n, list_build_n_dotted};
	pub use super::{list_append_2, list_append_3, list_append_4, list_append_n};
	pub use super::{list_make, list_clone, list_reverse};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{list_fill_range, list_reverse_range, list_copy_range};
	pub use super::{list_clone_range};
	pub use super::{list_range_iterator, list_pair_range_iterator};
	pub use super::{list_length};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{list_to_immutable, list_to_mutable};
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::{list_member_by_comparison, list_assoc_by_comparison};
	pub use super::{list_member_by_comparator, list_assoc_by_comparator};
	
	pub use super::{list_find};
	
	pub use super::{vec_list_append_2, vec_list_append_3, vec_list_append_4, vec_list_append_n};
	pub use super::{vec_list_append_2_dotted, vec_list_append_3_dotted, vec_list_append_4_dotted, vec_list_append_n_dotted};
	pub use super::{vec_list_clone, vec_list_clone_dotted, vec_list_drain, vec_list_drain_dotted};
	
	pub use super::{vec_list_ref_append_2, vec_list_ref_append_3, vec_list_ref_append_4, vec_list_ref_append_n};
	pub use super::{vec_list_ref_append_2_dotted, vec_list_ref_append_3_dotted, vec_list_ref_append_4_dotted, vec_list_ref_append_n_dotted};
	pub use super::{vec_list_ref_clone, vec_list_ref_clone_dotted, vec_list_ref_drain, vec_list_ref_drain_dotted};
	
	pub use super::{build_list_or_array, build_list_or_array_or_false_if_empty};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair <ValueRef1 : StdAsRef<Value>, ValueRef2 : StdAsRef<Value>> (left : ValueRef1, right : ValueRef2, immutable : Option<bool>) -> (Value) {
	let left = left.as_ref ();
	let left = left.clone ();
	let right = right.as_ref ();
	let right = right.clone ();
	return pair_new (left, right, immutable) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_left <ValueRef : StdAsRef<Value>> (pair : ValueRef) -> (Outcome<Value>) {
	let pair = pair.as_ref ();
	let left = try! (pair_left_ref (pair));
	let left = (*left) .clone ();
	succeed! (left);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_right <ValueRef : StdAsRef<Value>> (pair : ValueRef) -> (Outcome<Value>) {
	let pair = pair.as_ref ();
	let right = try! (pair_right_ref (pair));
	let right = (*right) .clone ();
	succeed! (right);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_left_ref <'a> (pair : &'a Value) -> (Outcome<ValueRef<'a>>) {
	let pair = pair.as_ref ();
	let pair = try_as_pair_ref! (pair);
	succeed! (pair.left_ref_into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_right_ref <'a> (pair : &'a Value) -> (Outcome<ValueRef<'a>>) {
	let pair = pair.as_ref ();
	let pair = try_as_pair_ref! (pair);
	succeed! (pair.right_ref_into ());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_left_set <ValueRef1 : StdAsRef<Value>, ValueRef2 : StdAsRef<Value>> (pair : ValueRef1, value : ValueRef2) -> (Outcome<Value>) {
	let pair = try_as_pair_mutable_ref! (pair.as_ref ());
	let mut pair = try! (pair.internals_ref_mut ());
	let mut value_swap = value.as_ref () .clone ();
	mem::swap (&mut value_swap, &mut pair.left);
	succeed! (value_swap);
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn pair_right_set <ValueRef1 : StdAsRef<Value>, ValueRef2 : StdAsRef<Value>> (pair : ValueRef1, value : ValueRef2) -> (Outcome<Value>) {
	let pair = try_as_pair_mutable_ref! (pair.as_ref ());
	let mut pair = try! (pair.internals_ref_mut ());
	let mut value_swap = value.as_ref () .clone ();
	mem::swap (&mut value_swap, &mut pair.right);
	succeed! (value_swap);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_first_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_first_at_ref (list, index)) .clone ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_rest_at (list : &Value, index : usize) -> (Outcome<Value>) {
	succeed! (try! (list_rest_at_ref (list, index)) .clone ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_first_at_ref (list : &Value, index : usize) -> (Outcome<ValueRef>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		return pair.left_ref_into ();
	} else {
		fail! (0xf3b2488a);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_rest_at_ref (list : &Value, index : usize) -> (Outcome<ValueRef>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		return pair.right_ref_into ();
	} else {
		fail! (0x9ea1c42c);
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_first_at_set (list : &Value, index : usize, value : &Value) -> (Outcome<Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		let mut pair = try! (pair.internals_ref_mut ());
		let mut value_swap = value.clone ();
		mem::swap (&mut value_swap, &mut pair.left);
		succeed! (value_swap);
	} else {
		fail! (0x1802bb11);
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_rest_at_set (list : &Value, index : usize, value : &Value) -> (Outcome<Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		let mut pair = try! (pair.internals_ref_mut ());
		let mut value_swap = value.clone ();
		mem::swap (&mut value_swap, &mut pair.right);
		succeed! (value_swap);
	} else {
		fail! (0xa3bc2627);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_pair_at (list : &Value, index : usize) -> (Outcome<Value>) {
	let pair = try! (list_pair_at_ref (list, index));
	if let Some (pair) = pair {
		succeed! (pair.clone () .into ());
	} else {
		succeed! (NULL.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_pair_at_ref (list : &Value, index : usize) -> (Outcome<Option<PairAsRef>>) {
	let mut iterator = try! (ListPairIterator::new (list, false));
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect <Source> (values : Source, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	return list_collect_dotted (values, None, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_dotted <Source> (values : Source, last : Option<Value>, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator
{
	let last = if let Some (last) = last {
		last
	} else {
		NULL.into ()
	};
	return values.into_iter () .rev () .fold (last, |last, value| pair_new (value, last, immutable) .into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_ref <Source, ValueRef> (values : Source, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = ValueRef>, Source::IntoIter : iter::DoubleEndedIterator, ValueRef : StdAsRef<Value>
{
	return list_collect_dotted_ref (values, None, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_dotted_ref <Source, ValueRef> (values : Source, last : Option<ValueRef>, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = ValueRef>, Source::IntoIter : iter::DoubleEndedIterator, ValueRef : StdAsRef<Value>
{
	let last = if let Some (last) = last {
		last.as_ref () .clone ()
	} else {
		NULL.into ()
	};
	return values.into_iter () .rev () .fold (last, |last, value| pair_new (value.as_ref () .clone (), last, immutable) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_from_generator <Source> (values : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	return list_collect_dotted_from_generator (values, None, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_dotted_from_generator <Source> (values : Source, last : Option<Value>, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	TODO! ("remove vector allocation");
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (list_collect_dotted (values, last, immutable));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_from_generator_ref <Source, ValueRef> (values : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	return list_collect_dotted_from_generator_ref (values, None, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_collect_dotted_from_generator_ref <Source, ValueRef> (values : Source, last : Option<ValueRef>, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	TODO! ("remove vector allocation");
	let values = try! (values.collect::<Outcome<StdVec<_>>> ());
	succeed! (list_collect_dotted_ref (values, last, immutable));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_empty () -> (Value) {
	return NULL.into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_1 (value_1 : &Value, dotted : Option<&Value>, immutable : Option<bool>) -> (Value) {
	let dotted = list_dotted_coerce (dotted);
	return pair_new (value_1.clone (), dotted, immutable) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_2 (value_1 : &Value, value_2 : &Value, dotted : Option<&Value>, immutable : Option<bool>) -> (Value) {
	let dotted = list_dotted_coerce (dotted);
	return pair_new (value_1.clone (), pair_new (value_2.clone (), dotted, immutable) .into (), immutable) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value, dotted : Option<&Value>, immutable : Option<bool>) -> (Value) {
	let dotted = list_dotted_coerce (dotted);
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), dotted, immutable) .into (), immutable) .into (), immutable) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value, dotted : Option<&Value>, immutable : Option<bool>) -> (Value) {
	let dotted = list_dotted_coerce (dotted);
	return pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), pair_new (value_4.clone (), dotted, immutable) .into (), immutable) .into (), immutable) .into (), immutable) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_n <ValueRef : StdAsRef<Value>> (values : &[ValueRef], dotted : Option<&Value>, immutable : Option<bool>) -> (Value) {
	if values.is_empty () {
		return list_empty ();
	}
	let dotted = list_dotted_coerce (dotted);
	return values.iter () .rev () .fold (dotted, |last, value| pair_new (value.as_ref () .clone (), last, immutable) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_build_n_dotted <ValueRef : StdAsRef<Value>> (values : &[ValueRef], immutable : Option<bool>) -> (Value) {
	let mut values = values.iter () .rev ();
	let dotted = if let Some (dotted) = values.next () {
		dotted.as_ref () .clone ()
	} else {
		return list_empty ();
	};
	return values.fold (dotted, |last, value| pair_new (value.as_ref () .clone (), last, immutable) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn list_dotted_coerce (value : Option<&Value>) -> (Value) {
	if let Some (value) = value {
		return value.clone ();
	} else {
		return NULL.into ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_append_2 (list_1 : &Value, list_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let (buffer, last) = try! (vec_list_append_2_dotted (list_1, list_2));
	succeed! (list_collect_dotted (buffer, last, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let (buffer, last) = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	succeed! (list_collect_dotted (buffer, last, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let (buffer, last) = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	succeed! (list_collect_dotted (buffer, last, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_append_n (lists : &[impl StdAsRef<Value>], immutable : Option<bool>) -> (Outcome<Value>) {
	if lists.is_empty () {
		succeed! (list_empty ());
	}
	TODO! ("optimize the vector allocation");
	let (buffer, last) = try! (vec_list_append_n_dotted (lists));
	succeed! (list_collect_dotted (buffer, last, immutable));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_make (length : usize, fill : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill.clone ());
	}
	succeed! (list_collect (buffer, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_clone (list : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	FIXME! ("if `immutable == None` keep the mutability status of each cloned pair", (github_issue, 88));
	let (buffer, last) = try! (vec_list_clone_dotted (list));
	succeed! (list_collect_dotted (buffer, last, immutable));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_reverse (list : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	FIXME! ("if `immutable == None` keep the mutability status of each cloned pair", (github_issue, 88));
	let buffer = try! (vec_list_clone (list));
	succeed! (list_collect (buffer.into_iter () .rev (), immutable));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_fill_range (list : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let fill = if let Some (fill) = fill {
		fill.clone ()
	} else {
		UNDEFINED.into ()
	};
	let iterator = try! (list_pair_range_iterator (list, range_start, range_end, true));
	for pair in iterator {
		let pair = try! (pair);
		let mut pair = try! (pair.internals_ref_mut ());
		pair.left = fill.clone ();
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_reverse_range (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	TODO! ("try to optimize this");
	let iterator = try! (list_pair_range_iterator (list, range_start, range_end, true));
	let pairs = try! (iterator.collect::<Outcome<StdVec<_>>> ());
	let pairs_count = pairs.len ();
	for offset in 0 .. pairs_count / 2 {
		let left = &pairs[offset];
		let right = &pairs[pairs_count - offset - 1];
		let mut left = try! (left.internals_ref_mut ());
		let mut right = try! (right.internals_ref_mut ());
		mem::swap (&mut left.left, &mut right.left);
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_copy_range (target_list : &Value, target_start : Option<&Value>, source_list : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<()>) {
	let mut target_iterator = try! (list_pair_range_iterator (target_list, target_start, None, true));
	let mut source_iterator = try! (list_range_iterator (source_list, source_start, source_end));
	loop {
		match (target_iterator.next (), source_iterator.next ()) {
			(None, None) =>
				succeed! (()),
			(Some (Ok (_)), None) =>
				succeed! (()),
			(None, Some (Ok (_))) =>
				fail! (0xc2a963e5),
			(Some (Ok (target_pair)), Some (Ok (source_value))) => {
				let mut target_pair = try! (target_pair.internals_ref_mut ());
				let source_value = source_value.value_clone ();
				target_pair.left = source_value;
			},
			(Some (Err (error)), _) =>
				return Err (error),
			(_, Some (Err (error))) =>
				return Err (error),
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_clone_range (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return list_collect_from_generator_ref (iterator, immutable);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_range_iterator <'a> (list : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<ValueRef<'a>, ListIterator<'a>>>) {
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let iterator = try! (ListIterator::new (list, false));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, range_end));
	succeed! (iterator);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_pair_range_iterator <'a> (list : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>, cloned : bool) -> (Outcome<RangeIteratorForOutcome<PairAsRef<'a>, ListPairIterator<'a>>>) {
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let iterator = try! (ListPairIterator::new_extended (list, false, cloned));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, range_end));
	succeed! (iterator);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_length (list : &Value) -> (Outcome<usize>) {
	let mut length : usize = 0;
	let mut iterator = try! (ListIterator::new (list, false));
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




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_to_immutable (list : &Value) -> (Outcome<Value>) {
	return list_clone (list, Some (true));
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_to_mutable (list : &Value) -> (Outcome<Value>) {
	return list_clone (list, Some (false));
}




#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_member_by_comparison (list : &Value, value : &Value, comparison : Comparison) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list, false));
	loop {
		match iterator.next () {
			Some (Ok (pair)) =>
				if try! (compare_2 (value, try! (pair.pair_ref ()) .left (), comparison)) {
					succeed! (pair.clone () .into ());
				}
			Some (Err (error)) =>
				return Err (error),
			None =>
				succeed! (false.into ()),
		}
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_member_by_comparator (list : &Value, value : &Value, comparator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ListPairIterator::new (list, false));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let comparison = try! (evaluator.evaluate_procedure_call_2 (comparator, value, try! (pair.pair_ref ()) .left ()));
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


#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_assoc_by_comparison (list : &Value, value : &Value, comparison : Comparison) -> (Outcome<Value>) {
	let mut iterator = try! (ListIterator::new (list, false));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let pair = try_as_pair_ref! (pair.as_ref ());
				if try! (compare_2 (value, pair.left (), comparison)) {
					succeed! (pair.value_clone () .into ());
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
pub fn list_assoc_by_comparator (list : &Value, value : &Value, comparator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ListIterator::new (list, false));
	loop {
		match iterator.next () {
			Some (Ok (pair)) => {
				let pair = try_as_pair_ref! (pair.as_ref ());
				let comparison = try! (evaluator.evaluate_procedure_call_2 (comparator, value, pair.left ()));
				if is_not_false (&comparison) {
					succeed! (pair.value_clone () .into ());
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
pub fn list_find (list : &Value, predicate : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let mut iterator = try! (ListIterator::new (list, false));
	loop {
		match iterator.next () {
			Some (Ok (value)) => {
				let comparison = try! (evaluator.evaluate_procedure_call_1 (predicate, &value));
				if is_not_false (&comparison) {
					succeed! (value.value_clone ());
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
pub fn vec_list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_2_dotted (list_1, list_2));
	return vec_list_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_3_dotted (list_1, list_2, list_3));
	return vec_list_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<ValueVec>) {
	let buffer = try! (vec_list_append_4_dotted (list_1, list_2, list_3, list_4));
	return vec_list_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_append_n (lists : &[impl StdAsRef<Value>]) -> (Outcome<ValueVec>) {
	if lists.is_empty () {
		succeed! (StdVec::new ());
	}
	let buffer = try! (vec_list_append_n_dotted (lists));
	return vec_list_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn vec_list_append_return ((buffer, last) : (ValueVec, Option<Value>)) -> (Outcome<ValueVec>) {
	match last {
		Some (_) =>
			fail! (0xe037d833),
		None =>
			succeed! (buffer),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_append_2_dotted (list_1 : &Value, list_2 : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	if is_null_all_2 (list_1, list_2) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_drain (&mut buffer, &list_1));
	let last = try! (vec_list_drain_dotted (&mut buffer, &list_2));
	succeed! ((buffer, last));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_append_n_dotted (lists : &[impl StdAsRef<Value>]) -> (Outcome<(ValueVec, Option<Value>)>) {
	if lists.is_empty () {
		succeed! ((StdVec::new (), None));
	}
	match lists.split_last () {
		Some ((list_last, lists_first)) => {
			let list_last = list_last.as_ref ();
			if lists_first.is_empty () {
				return vec_list_clone_dotted (list_last);
			} else {
				let mut buffer = StdVec::new ();
				for list in lists_first {
					let list = list.as_ref ();
					try! (vec_list_drain (&mut buffer, list));
				}
				let last = try! (vec_list_drain_dotted (&mut buffer, list_last));
				succeed! ((buffer, last));
			}
		}
		None =>
			succeed! ((StdVec::new (), None)),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_clone (list : &Value) -> (Outcome<ValueVec>) {
	let (buffer, last) = try! (vec_list_clone_dotted (list));
	match last {
		Some (_) =>
			fail! (0xc38cb8df),
		None =>
			succeed! (buffer),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_clone_dotted (list : &Value) -> (Outcome<(ValueVec, Option<Value>)>) {
	let mut buffer = StdVec::new ();
	let last = try! (vec_list_drain_dotted (&mut buffer, list));
	succeed! ((buffer, last));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_drain (buffer : &mut ValueVec, list : &Value) -> (Outcome<()>) {
	let last = try! (vec_list_drain_dotted (buffer, list));
	match last {
		Some (_) =>
			fail! (0xae634ad2),
		None =>
			succeed! (()),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_drain_dotted (buffer : &mut ValueVec, list : &Value) -> (Outcome<Option<Value>>) {
	let mut iterator = try! (ListIterator::new (list, true));
	loop {
		match iterator.next () {
			Some (Ok (value)) =>
				buffer.push (value.clone ()),
			Some (Err (error)) =>
				return Err (error),
			None =>
				return Ok (option_map! (iterator.dotted (), value, value.clone ())),
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_2 <'a> (list_1 : &'a Value, list_2 : &'a Value) -> (Outcome<StdVec<ValueRef<'a>>>) {
	let buffer = try! (vec_list_ref_append_2_dotted (list_1, list_2));
	return vec_list_ref_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_3 <'a> (list_1 : &'a Value, list_2 : &'a Value, list_3 : &'a Value) -> (Outcome<StdVec<ValueRef<'a>>>) {
	let buffer = try! (vec_list_ref_append_3_dotted (list_1, list_2, list_3));
	return vec_list_ref_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_4 <'a> (list_1 : &'a Value, list_2 : &'a Value, list_3 : &'a Value, list_4 : &'a Value) -> (Outcome<StdVec<ValueRef<'a>>>) {
	let buffer = try! (vec_list_ref_append_4_dotted (list_1, list_2, list_3, list_4));
	return vec_list_ref_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_n <'a> (lists : &'a [&'a Value]) -> (Outcome<StdVec<ValueRef<'a>>>) {
	if lists.is_empty () {
		succeed! (StdVec::new ());
	}
	let buffer = try! (vec_list_ref_append_n_dotted (lists));
	return vec_list_ref_append_return (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn vec_list_ref_append_return <'a> ((buffer, last) : (StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)) -> (Outcome<StdVec<ValueRef<'a>>>) {
	match last {
		Some (_) =>
			fail! (0x48f9af8f),
		None =>
			succeed! (buffer),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_2_dotted <'a> (list_1 : &'a Value, list_2 : &'a Value) -> (Outcome<(StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)>) {
	if is_null_all_2 (list_1, list_2) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_ref_drain (&mut buffer, &list_1));
	let last = try! (vec_list_ref_drain_dotted (&mut buffer, &list_2));
	succeed! ((buffer, last));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_3_dotted <'a> (list_1 : &'a Value, list_2 : &'a Value, list_3 : &'a Value) -> (Outcome<(StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)>) {
	if is_null_all_3 (list_1, list_2, list_3) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_ref_drain (&mut buffer, &list_1));
	try! (vec_list_ref_drain (&mut buffer, &list_2));
	let last = try! (vec_list_ref_drain_dotted (&mut buffer, &list_3));
	succeed! ((buffer, last));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_4_dotted <'a> (list_1 : &'a Value, list_2 : &'a Value, list_3 : &'a Value, list_4 : &'a Value) -> (Outcome<(StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)>) {
	if is_null_all_4 (list_1, list_2, list_3, list_4) {
		succeed! ((StdVec::new (), None));
	}
	let mut buffer = StdVec::new ();
	try! (vec_list_ref_drain (&mut buffer, &list_1));
	try! (vec_list_ref_drain (&mut buffer, &list_2));
	try! (vec_list_ref_drain (&mut buffer, &list_3));
	let last = try! (vec_list_ref_drain_dotted (&mut buffer, &list_4));
	succeed! ((buffer, last));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_append_n_dotted <'a> (lists : &'a [&'a Value]) -> (Outcome<(StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)>) {
	if lists.is_empty () {
		succeed! ((StdVec::new (), None));
	}
	match lists.split_last () {
		Some ((list_last, lists_first)) =>
			if lists_first.is_empty () {
				return vec_list_ref_clone_dotted (list_last);
			} else {
				let mut buffer = StdVec::new ();
				for list in lists_first {
					try! (vec_list_ref_drain (&mut buffer, &list));
				}
				let last = try! (vec_list_ref_drain_dotted (&mut buffer, &list_last));
				succeed! ((buffer, last));
			},
		None =>
			succeed! ((StdVec::new (), None)),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_clone <'a> (list : &'a Value) -> (Outcome<StdVec<ValueRef<'a>>>) {
	let (buffer, last) = try! (vec_list_ref_clone_dotted (list));
	match last {
		Some (_) =>
			fail! (0x096d7253),
		None =>
			succeed! (buffer),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_clone_dotted <'a> (list : &'a Value) -> (Outcome<(StdVec<ValueRef<'a>>, Option<ValueRef<'a>>)>) {
	let mut buffer = StdVec::new ();
	let last = try! (vec_list_ref_drain_dotted (&mut buffer, list));
	succeed! ((buffer, last));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_drain <'a : 'b, 'b> (buffer : &'b mut StdVec<ValueRef<'a>>, list : &'a Value) -> (Outcome<()>) {
	let last = try! (vec_list_ref_drain_dotted (buffer, list));
	match last {
		Some (_) =>
			fail! (0x83b605ae),
		None =>
			succeed! (()),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_list_ref_drain_dotted <'a : 'b, 'b> (buffer : &'b mut StdVec<ValueRef<'a>>, list : &'a Value) -> (Outcome<Option<ValueRef<'a>>>) {
	let mut iterator = try! (ListIterator::new (list, true));
	loop {
		match iterator.next () {
			Some (Ok (value)) =>
				buffer.push (value),
			Some (Err (error)) =>
				return Err (error),
			None =>
				return Ok (iterator.dotted ()),
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn build_list_or_array (values : StdVec<Value>, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	if return_array {
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		succeed! (array_new (values, immutable) .into ());
		#[ cfg ( not ( feature = "vonuvoli_values_array" ) ) ]
		fail_panic! (0x2b2ec760);
	} else {
		succeed! (list_collect (values, immutable));
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn build_list_or_array_or_false_if_empty (values : StdVec<Value>, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	if values.is_empty () {
		succeed! (FALSE_VALUE);
	} else {
		return build_list_or_array (values, return_array, immutable);
	}
}

