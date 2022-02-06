

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{bytes_at};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{bytes_at_set};
	
	pub use super::{bytes_empty};
	pub use super::{bytes_collect_bytes, bytes_collect_values, bytes_collect_values_ref};
	pub use super::{bytes_collect_bytes_from_generator, bytes_collect_values_from_generator, bytes_collect_values_from_generator_ref};
	pub use super::{bytes_build_1, bytes_build_2, bytes_build_3, bytes_build_4, bytes_build_n};
	pub use super::{bytes_append_2, bytes_append_3, bytes_append_4, bytes_append_n};
	pub use super::{bytes_make, bytes_clone, bytes_reverse};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{bytes_fill_range, bytes_reverse_range, bytes_copy_range};
	pub use super::{bytes_clone_range};
	pub use super::{bytes_range_to_list, list_range_to_bytes};
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::{bytes_range_to_array, array_range_to_bytes};
	pub use super::{bytes_range_iterator};
	pub use super::{bytes_length};
	
	pub use super::{vec_bytes_append_2, vec_bytes_append_3, vec_bytes_append_4, vec_bytes_append_n};
	pub use super::{vec_bytes_clone, vec_bytes_drain};
	
}




pub fn bytes_at (bytes : &Value, index : usize) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (byte) = bytes.get (index) {
		succeed! (byte.clone () .into ());
	} else {
		fail! (0x9a4ad939);
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn bytes_at_set (bytes : &Value, index : usize, byte : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut bytes = r#try! (bytes.bytes_ref_mut ());
	let byte = try_as_number_integer_ref! (byte);
	let byte_ref = try_some! (bytes.get_mut (index), 0x3cf4282c);
	let mut byte_swap = r#try! (byte.try_to_u8 ());
	mem::swap (&mut byte_swap, byte_ref);
	succeed! (byte_swap.into ());
}




pub fn bytes_collect_bytes <Source> (bytes : Source, immutable : Option<bool>) -> (Value)
		where Source : iter::IntoIterator<Item = u8>, Source::IntoIter : iter::DoubleEndedIterator
{
	return bytes_new (iter::FromIterator::from_iter (bytes), immutable);
}

pub fn bytes_collect_values <Source> (bytes : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator
{
	let bytes = bytes.into_iter ();
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (r#try! (try_into_number_integer! (byte) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_collect_values_ref <Source, ValueRef> (bytes : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = ValueRef>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator, ValueRef : StdAsRef<Value>
{
	let bytes = bytes.into_iter ();
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (r#try! (try_as_number_integer_ref! (byte.as_ref ()) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer, immutable));
}




pub fn bytes_collect_bytes_from_generator <Source> (bytes : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<u8>>
{
	TODO! ("eliminate vector allocation");
	let bytes = r#try! (bytes.collect::<Outcome<StdVec<_>>> ());
	succeed! (bytes_collect_bytes (bytes, immutable));
}

pub fn bytes_collect_values_from_generator <Source> (bytes : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	TODO! ("eliminate vector allocation");
	let bytes = r#try! (bytes.collect::<Outcome<StdVec<_>>> ());
	return bytes_collect_values (bytes, immutable);
}

pub fn bytes_collect_values_from_generator_ref <Source, ValueRef> (bytes : Source, immutable : Option<bool>) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	TODO! ("eliminate vector allocation");
	let bytes = r#try! (bytes.collect::<Outcome<StdVec<_>>> ());
	return bytes_collect_values_ref (bytes, immutable);
}




pub fn bytes_empty (immutable : Option<bool>) -> (Value) {
	return bytes_new_empty (immutable);
}

pub fn bytes_build_1 (byte_1 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (r#try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_build_2 (byte_1 : &Value, byte_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (r#try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_build_3 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (r#try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_build_4 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value, byte_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (r#try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	buffer.push (r#try! (try_as_number_integer_ref! (byte_4) .try_to_u8 ()));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_build_n (bytes : &[impl StdAsRef<Value>], immutable : Option<bool>) -> (Outcome<Value>) {
	if bytes.is_empty () {
		succeed! (bytes_empty (immutable));
	}
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		let byte = byte.as_ref ();
		buffer.push (r#try! (try_as_number_integer_ref! (byte) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer, immutable));
}




pub fn bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = r#try! (vec_bytes_append_2 (bytes_1, bytes_2));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = r#try! (vec_bytes_append_3 (bytes_1, bytes_2, bytes_3));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = r#try! (vec_bytes_append_4 (bytes_1, bytes_2, bytes_3, bytes_4));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_append_n (bytes : &[impl StdAsRef<Value>], immutable : Option<bool>) -> (Outcome<Value>) {
	if bytes.is_empty () {
		succeed! (bytes_empty (immutable));
	}
	let buffer = r#try! (vec_bytes_append_n (bytes));
	succeed! (bytes_new (buffer, immutable));
}




pub fn bytes_make (length : usize, fill : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let fill = if let Some (fill) = fill {
		r#try! (try_as_number_integer_ref! (fill) .try_to_u8 ())
	} else {
		0
	};
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill);
	}
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_clone (bytes : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = r#try! (vec_bytes_clone (bytes));
	succeed! (bytes_new (buffer, immutable));
}

pub fn bytes_reverse (bytes : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let buffer = r#try! (vec_bytes_clone (bytes));
	succeed! (bytes_collect_bytes (buffer.into_iter () .rev (), immutable));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn bytes_fill_range (bytes : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut bytes = r#try! (bytes.bytes_ref_mut ());
	let fill = if let Some (fill) = fill {
		r#try! (try_as_number_integer_ref! (fill) .try_to_u8 ())
	} else {
		0
	};
	let (range_start, range_end) = r#try! (range_coerce (range_start, range_end, bytes.len ()));
	let bytes = try_some! (bytes.get_mut (range_start .. range_end), 0x79902e57);
	for byte_ref in bytes {
		*byte_ref = fill;
	}
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn bytes_reverse_range (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut bytes = r#try! (bytes.bytes_ref_mut ());
	let (range_start, range_end) = r#try! (range_coerce (range_start, range_end, bytes.len ()));
	let bytes = try_some! (bytes.get_mut (range_start .. range_end), 0x31d6fbe3);
	bytes.reverse ();
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn bytes_copy_range (target_bytes : &Value, target_start : Option<&Value>, source_bytes : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<()>) {
	let target_bytes = try_as_bytes_mutable_ref! (target_bytes);
	let mut target_bytes = r#try! (target_bytes.bytes_ref_mut ());
	let source_bytes = try_as_bytes_ref! (source_bytes);
	let source_bytes = source_bytes.bytes_as_slice ();
	let (source_start, source_end) = r#try! (range_coerce (source_start, source_end, source_bytes.len ()));
	let (target_start, target_end) = r#try! (range_coerce (target_start, None, target_bytes.len ()));
	let target_end = if (target_end - target_start) >= (source_end - source_start) {
		target_start + (source_end - source_start)
	} else {
		fail! (0x7033eb20);
	};
	let target_bytes = try_some! (target_bytes.get_mut (target_start .. target_end), 0xbd28374b);
	let source_bytes = try_some! (source_bytes.get (source_start .. source_end), 0xb0b58937);
	<[u8]>::copy_from_slice (target_bytes, source_bytes);
	succeed! (());
}


pub fn bytes_clone_range (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = r#try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	succeed! (bytes_clone_slice (& bytes.bytes_as_slice () [range_start..range_end], immutable));
}




pub fn bytes_range_to_list (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = r#try! (bytes_range_iterator (bytes, range_start, range_end));
	return list_collect_from_generator (iterator, immutable);
}

pub fn list_range_to_bytes (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = r#try! (list_range_iterator (list, range_start, range_end));
	return bytes_collect_values_from_generator_ref (iterator, immutable);
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub fn bytes_range_to_array (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = r#try! (bytes_range_iterator (bytes, range_start, range_end));
	return array_collect_from_generator (iterator, immutable);
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub fn array_range_to_bytes (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let iterator = r#try! (array_range_iterator (array, range_start, range_end));
	return bytes_collect_values_from_generator_ref (iterator, immutable);
}




pub fn bytes_range_iterator <'a> (bytes : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<Value, BytesIterator<'a>>>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = r#try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	let iterator = r#try! (BytesIterator::new_a (bytes));
	let iterator = r#try! (RangeIteratorForOutcome::new (iterator, range_start, Some (range_end)));
	succeed! (iterator);
}




pub fn bytes_length (bytes : &Value) -> (Outcome<usize>) {
	let bytes = try_as_bytes_ref! (bytes);
	succeed! (bytes.bytes_count ());
}




pub fn vec_bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<StdVec<u8>>) {
	if r#try! (is_bytes_empty_all_2 (bytes_1, bytes_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	r#try! (vec_bytes_drain (&mut buffer, bytes_1));
	r#try! (vec_bytes_drain (&mut buffer, bytes_2));
	succeed! (buffer);
}

pub fn vec_bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<StdVec<u8>>) {
	if r#try! (is_bytes_empty_all_3 (bytes_1, bytes_2, bytes_3)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	r#try! (vec_bytes_drain (&mut buffer, bytes_1));
	r#try! (vec_bytes_drain (&mut buffer, bytes_2));
	r#try! (vec_bytes_drain (&mut buffer, bytes_3));
	succeed! (buffer);
}

pub fn vec_bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<StdVec<u8>>) {
	if r#try! (is_bytes_empty_all_4 (bytes_1, bytes_2, bytes_3, bytes_4)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	r#try! (vec_bytes_drain (&mut buffer, bytes_1));
	r#try! (vec_bytes_drain (&mut buffer, bytes_2));
	r#try! (vec_bytes_drain (&mut buffer, bytes_3));
	r#try! (vec_bytes_drain (&mut buffer, bytes_4));
	succeed! (buffer);
}

pub fn vec_bytes_append_n (bytes : &[impl StdAsRef<Value>]) -> (Outcome<StdVec<u8>>) {
	if bytes.is_empty () {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	for bytes in bytes {
		let bytes = bytes.as_ref ();
		r#try! (vec_bytes_drain (&mut buffer, bytes));
	}
	succeed! (buffer);
}




pub fn vec_bytes_clone (bytes : &Value) -> (Outcome<StdVec<u8>>) {
	let mut buffer = StdVec::new ();
	r#try! (vec_bytes_drain (&mut buffer, bytes));
	succeed! (buffer);
}


pub fn vec_bytes_drain (buffer : &mut StdVec<u8>, bytes : &Value) -> (Outcome<()>) {
	let bytes = try_as_bytes_ref! (bytes);
	buffer.extend_from_slice (bytes.bytes_as_slice ());
	succeed! (());
}

