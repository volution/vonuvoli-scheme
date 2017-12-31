

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{bytes_at, bytes_at_set};
	
	pub use super::{bytes_empty};
	pub use super::{bytes_collect_bytes, bytes_collect_values, bytes_collect_values_ref};
	pub use super::{bytes_collect_bytes_from_generator, bytes_collect_values_from_generator, bytes_collect_values_from_generator_ref};
	pub use super::{bytes_build_1, bytes_build_2, bytes_build_3, bytes_build_4, bytes_build_n};
	pub use super::{bytes_append_2, bytes_append_3, bytes_append_4, bytes_append_n};
	pub use super::{bytes_make, bytes_clone, bytes_reverse};
	pub use super::{bytes_fill_range, bytes_reverse_range, bytes_copy_range, bytes_clone_range};
	pub use super::{bytes_range_to_list, list_range_to_bytes};
	pub use super::{bytes_range_to_array, array_range_to_bytes};
	pub use super::{bytes_range_iterator};
	pub use super::{bytes_length};
	
	pub use super::{vec_bytes_append_2, vec_bytes_append_3, vec_bytes_append_4, vec_bytes_append_n};
	pub use super::{vec_bytes_clone, vec_bytes_drain};
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_at (bytes : &Value, index : usize) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_vec ();
	if let Some (byte) = bytes.get (index) {
		succeed! (byte.clone () .into ());
	} else {
		fail! (0x9a4ad939);
	}
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_at_set (_bytes : &Value, _index : usize, _byte : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xd606bd1c);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_bytes <Source> (bytes : Source) -> (Value)
		where Source : iter::IntoIterator<Item = u8>, Source::IntoIter : iter::DoubleEndedIterator
{
	return bytes_new (iter::FromIterator::from_iter (bytes)) .into ();
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_values <Source> (bytes : Source) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator
{
	let bytes = bytes.into_iter ();
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (try! (try_into_number_integer! (byte) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_values_ref <Source, ValueRef> (bytes : Source) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = ValueRef>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator, ValueRef : StdAsRef<Value>
{
	let bytes = bytes.into_iter ();
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (try! (try_as_number_integer_ref! (byte.as_ref ()) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer) .into ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_bytes_from_generator <Source> (bytes : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<u8>>
{
	// FIXME:  Eliminate vector allocation!
	let bytes = try! (bytes.collect::<Outcome<StdVec<_>>> ());
	succeed! (bytes_collect_bytes (bytes));
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_values_from_generator <Source> (bytes : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	// FIXME:  Eliminate vector allocation!
	let bytes = try! (bytes.collect::<Outcome<StdVec<_>>> ());
	return bytes_collect_values (bytes);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_collect_values_from_generator_ref <Source, ValueRef> (bytes : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	// FIXME:  Eliminate vector allocation!
	let bytes = try! (bytes.collect::<Outcome<StdVec<_>>> ());
	return bytes_collect_values_ref (bytes);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_empty () -> (Value) {
	return bytes_new (StdVec::new ()) .into ();
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_build_1 (byte_1 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (1);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_build_2 (byte_1 : &Value, byte_2 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (2);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_build_3 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (3);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_build_4 (byte_1 : &Value, byte_2 : &Value, byte_3 : &Value, byte_4 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdVec::with_capacity (4);
	buffer.push (try! (try_as_number_integer_ref! (byte_1) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_2) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_3) .try_to_u8 ()));
	buffer.push (try! (try_as_number_integer_ref! (byte_4) .try_to_u8 ()));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_build_n (bytes : &[&Value]) -> (Outcome<Value>) {
	match bytes.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			return bytes_build_1 (bytes[0]),
		2 =>
			return bytes_build_2 (bytes[0], bytes[1]),
		3 =>
			return bytes_build_3 (bytes[0], bytes[1], bytes[2]),
		4 =>
			return bytes_build_4 (bytes[0], bytes[1], bytes[2], bytes[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::with_capacity (bytes.len ());
	for byte in bytes {
		buffer.push (try! (try_as_number_integer_ref! (*byte) .try_to_u8 ()));
	}
	succeed! (bytes_new (buffer) .into ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_2 (bytes_1, bytes_2));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_3 (bytes_1, bytes_2, bytes_3));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_append_4 (bytes_1, bytes_2, bytes_3, bytes_4));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_append_n (bytes : &[&Value]) -> (Outcome<Value>) {
	match bytes.len () {
		0 =>
			succeed! (bytes_empty ()),
		1 =>
			succeed! (bytes[0].clone ()),
		2 =>
			return bytes_append_2 (bytes[0], bytes[1]),
		3 =>
			return bytes_append_3 (bytes[0], bytes[1], bytes[2]),
		4 =>
			return bytes_append_4 (bytes[0], bytes[1], bytes[2], bytes[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_bytes_append_n (bytes));
	succeed! (bytes_new (buffer) .into ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_make (length : usize, fill : Option<&Value>) -> (Outcome<Value>) {
	let fill = if let Some (fill) = fill {
		try! (try_as_number_integer_ref! (fill) .try_to_u8 ())
	} else {
		0 as u8
	};
	let mut buffer = StdVec::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill);
	}
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_clone (bytes : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_bytes_clone (bytes));
	succeed! (bytes_new (buffer) .into ());
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_reverse (bytes : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let buffer = try! (vec_bytes_clone (bytes));
	succeed! (bytes_collect_bytes (buffer.into_iter () .rev ()));
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_fill_range (bytes : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let _fill = if let Some (fill) = fill {
		try! (try_as_number_integer_ref! (fill) .try_to_u8 ())
	} else {
		0 as u8
	};
	let (_range_start, _range_end) = try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	fail_unimplemented! (0xfc14ec8b);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_reverse_range (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (_range_start, _range_end) = try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	fail_unimplemented! (0xff6acb00);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_copy_range (target_bytes : &Value, target_start : Option<&Value>, source_bytes : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<Value>) {
	let target_bytes = try_as_bytes_ref! (target_bytes);
	let source_bytes = try_as_bytes_ref! (source_bytes);
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_bytes.bytes_count ()));
	let (target_start, target_end) = try! (range_coerce (target_start, None, target_bytes.bytes_count ()));
	if (target_end - target_start) < (source_end - source_start) {
		fail! (0x7033eb20);
	}
	fail_unimplemented! (0x00cfa730);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_clone_range (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	succeed! (bytes_clone_slice (& bytes.bytes_as_slice () [range_start..range_end]) .into ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_range_to_list (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (bytes_range_iterator (bytes, range_start, range_end));
	return list_collect_from_generator (iterator);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn list_range_to_bytes (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (list_range_iterator (list, range_start, range_end));
	return bytes_collect_values_from_generator_ref (iterator);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_range_to_array (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (bytes_range_iterator (bytes, range_start, range_end));
	return array_collect_from_generator (iterator);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn array_range_to_bytes (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let iterator = try! (array_range_iterator (array, range_start, range_end));
	return bytes_collect_values_from_generator_ref (iterator);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_range_iterator <'a> (bytes : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<Value, BytesIterator<'a>>>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	let iterator = try! (BytesIterator::new_a (bytes));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, Some (range_end)));
	succeed! (iterator);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn bytes_length (bytes : &Value) -> (Outcome<usize>) {
	let bytes = try_as_bytes_ref! (bytes);
	succeed! (bytes.bytes_count ());
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_append_2 (bytes_1 : &Value, bytes_2 : &Value) -> (Outcome<StdVec<u8>>) {
	if try! (is_bytes_empty_all_2 (bytes_1, bytes_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_append_3 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value) -> (Outcome<StdVec<u8>>) {
	if try! (is_bytes_empty_all_3 (bytes_1, bytes_2, bytes_3)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	try! (vec_bytes_drain (&mut buffer, &bytes_3));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_append_4 (bytes_1 : &Value, bytes_2 : &Value, bytes_3 : &Value, bytes_4 : &Value) -> (Outcome<StdVec<u8>>) {
	if try! (is_bytes_empty_all_4 (bytes_1, bytes_2, bytes_3, bytes_4)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, &bytes_1));
	try! (vec_bytes_drain (&mut buffer, &bytes_2));
	try! (vec_bytes_drain (&mut buffer, &bytes_3));
	try! (vec_bytes_drain (&mut buffer, &bytes_4));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_append_n (bytes : &[&Value]) -> (Outcome<StdVec<u8>>) {
	match bytes.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_bytes_clone (bytes[0]),
		2 =>
			return vec_bytes_append_2 (bytes[0], bytes[1]),
		3 =>
			return vec_bytes_append_3 (bytes[0], bytes[1], bytes[2]),
		4 =>
			return vec_bytes_append_4 (bytes[0], bytes[1], bytes[2], bytes[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::new ();
	for bytes in bytes {
		try! (vec_bytes_drain (&mut buffer, &bytes));
	}
	succeed! (buffer);
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_clone (bytes : &Value) -> (Outcome<StdVec<u8>>) {
	let mut buffer = StdVec::new ();
	try! (vec_bytes_drain (&mut buffer, bytes));
	succeed! (buffer);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn vec_bytes_drain (buffer : &mut StdVec<u8>, bytes : &Value) -> (Outcome<()>) {
	let bytes = try_as_bytes_ref! (bytes);
	buffer.extend_from_slice (bytes.bytes_as_slice ());
	succeed! (());
}

