

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{string_at};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{string_at_set};
	
	pub use super::{string_empty};
	pub use super::{string_collect_chars, string_collect_values, string_collect_values_ref};
	pub use super::{string_collect_chars_from_generator, string_collect_values_from_generator, string_collect_values_from_generator_ref};
	pub use super::{string_build_1, string_build_2, string_build_3, string_build_4, string_build_n};
	pub use super::{string_append_2, string_append_3, string_append_4, string_append_n};
	pub use super::{string_make, string_clone, string_reverse};
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{string_fill_range, string_reverse_range, string_copy_range};
	pub use super::{string_clone_range};
	pub use super::{string_range_to_list, list_range_to_string};
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::{string_range_to_array, array_range_to_string};
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{string_range_to_bytes, bytes_range_to_string};
	pub use super::{string_range_iterator};
	pub use super::{string_length};
	
	pub use super::{vec_string_append_2, vec_string_append_3, vec_string_append_4, vec_string_append_n};
	pub use super::{vec_string_clone, vec_string_drain};
	
	pub use super::{
			string_to_upper_case, string_to_lower_case, string_to_fold_case,
			symbol_to_upper_case, symbol_to_lower_case, symbol_to_fold_case,
			character_to_upper_case, character_to_lower_case, character_to_fold_case,
	};
	
	pub use super::{
			string_to_symbol, symbol_to_string,
			string_to_number, number_to_string,
			character_to_number, number_to_character, character_to_digit_number,
	};
	
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	pub use super::{
			keyword_to_upper_case, keyword_to_lower_case, keyword_to_fold_case,
			string_to_keyword, keyword_to_string,
			symbol_to_keyword, keyword_to_symbol,
	};
	
	pub use super::{
			character_is_numeric,
			character_is_alphabetic, character_is_alphabetic_upper_case, character_is_alphabetic_lower_case,
			character_is_alphabetic_or_numeric,
			character_is_whitespace, character_is_control,
	};
	
	pub use super::{
			character_is_ascii,
			character_is_ascii_numeric, character_is_ascii_numeric_base_8, character_is_ascii_numeric_base_16,
			character_is_ascii_alphabetic, character_is_ascii_alphabetic_upper_case, character_is_ascii_alphabetic_lower_case,
			character_is_ascii_alphabetic_or_numeric,
			character_is_ascii_whitespace, character_is_ascii_control, character_is_ascii_punctuation, character_is_ascii_graphic,
	};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_at (string : &Value, index : usize) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	if let Some (char) = string.string_char_at_compute (index) {
		succeed! (char.into ());
	} else {
		fail! (0x0eb98e3f);
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_at_set (string : &Value, index : usize, char : &Value) -> (Outcome<Value>) {
	let string = try_as_string_mutable_ref! (string);
	let mut string = try! (string.string_ref_mut ());
	let char = try_as_character_ref! (char);
	let mut buffer = unicode_utf8_string_clone_chars (string.as_str ());
	let char_swap = if let Some (char_ref) = buffer.get_mut (index) {
		let mut char_swap = char.value ();
		mem::swap (&mut char_swap, char_ref);
		char_swap
	} else {
		fail! (0x2aefb8e7);
	};
	string.clear ();
	string.extend (buffer);
	succeed! (char_swap.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_chars <Source> (chars : Source) -> (Value)
		where Source : iter::IntoIterator<Item = char>, Source::IntoIter : iter::DoubleEndedIterator
{
	return string_new (iter::FromIterator::from_iter (chars)) .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_values <Source> (chars : Source) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = Value>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator
{
	let chars = chars.into_iter ();
	let mut buffer = StdVec::with_capacity (chars.len ());
	for char in chars {
		buffer.push (try_into_character! (char) .value ());
	}
	succeed! (string_collect_chars (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_values_ref <Source, ValueRef> (chars : Source) -> (Outcome<Value>)
		where Source : iter::IntoIterator<Item = ValueRef>, Source::IntoIter : iter::DoubleEndedIterator, Source::IntoIter : iter::ExactSizeIterator, ValueRef : StdAsRef<Value>
{
	let chars = chars.into_iter ();
	let mut buffer = StdVec::with_capacity (chars.len ());
	for char in chars {
		buffer.push (try_as_character_ref! (char.as_ref ()) .value ());
	}
	succeed! (string_collect_chars (buffer) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_chars_from_generator <Source> (chars : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<char>>
{
	TODO! ("eliminate vector allocation");
	let chars = try! (chars.collect::<Outcome<StdVec<_>>> ());
	succeed! (string_collect_chars (chars));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_values_from_generator <Source> (chars : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	TODO! ("eliminate vector allocation");
	let chars = try! (chars.collect::<Outcome<StdVec<_>>> ());
	return string_collect_values (chars);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_collect_values_from_generator_ref <Source, ValueRef> (chars : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<ValueRef>>, ValueRef : StdAsRef<Value>
{
	TODO! ("eliminate vector allocation");
	let chars = try! (chars.collect::<Outcome<StdVec<_>>> ());
	return string_collect_values_ref (chars);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_empty () -> (Value) {
	return string_new_empty () .into ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_build_1 (char_1 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (1);
	buffer.push (try_as_character_ref! (char_1) .value ());
	succeed! (string_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_build_2 (char_1 : &Value, char_2 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (2);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	succeed! (string_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_build_3 (char_1 : &Value, char_2 : &Value, char_3 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (3);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	buffer.push (try_as_character_ref! (char_3) .value ());
	succeed! (string_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_build_4 (char_1 : &Value, char_2 : &Value, char_3 : &Value, char_4 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (4);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	buffer.push (try_as_character_ref! (char_3) .value ());
	buffer.push (try_as_character_ref! (char_4) .value ());
	succeed! (string_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_build_n (chars : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
	if chars.is_empty () {
		succeed! (string_empty ());
	}
	let mut buffer = StdString::with_capacity (chars.len ());
	for char in chars {
		let char = char.as_ref ();
		buffer.push (try_as_character_ref! (char) .value ());
	}
	succeed! (string_new (buffer) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_append_2 (string_1 : &Value, string_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_2 (string_1, string_2));
	succeed! (string_collect_chars (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_append_3 (string_1 : &Value, string_2 : &Value, string_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_3 (string_1, string_2, string_3));
	succeed! (string_collect_chars (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_append_4 (string_1 : &Value, string_2 : &Value, string_3 : &Value, string_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_4 (string_1, string_2, string_3, string_4));
	succeed! (string_collect_chars (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_append_n (strings : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
	if strings.is_empty () {
		succeed! (string_empty ());
	}
	let buffer = try! (vec_string_append_n (strings));
	succeed! (string_collect_chars (buffer) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_make (length : usize, fill : Option<&Value>) -> (Outcome<Value>) {
	let fill = if let Some (fill) = fill {
		try_as_character_ref! (fill) .value ()
	} else {
		0 as char
	};
	let mut buffer = StdString::with_capacity (length);
	for _index in 0..length {
		buffer.push (fill);
	}
	succeed! (string_new (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_clone (string : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_clone (string));
	succeed! (string_collect_chars (buffer) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_reverse (string : &Value) -> (Outcome<Value>) {
	TODO! ("optimize the vector allocation");
	let buffer = try! (vec_string_clone (string));
	succeed! (string_collect_chars (buffer.into_iter () .rev ()));
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_fill_range (string : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let string = try_as_string_mutable_ref! (string);
	let mut string = try! (string.string_ref_mut ());
	let fill = if let Some (fill) = fill {
		try_as_character_ref! (fill) .value ()
	} else {
		0 as char
	};
	let mut buffer = unicode_utf8_string_clone_chars (string.as_str ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	{
		let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0x4ee3e633);
		for char_ref in buffer {
			*char_ref = fill;
		}
	}
	string.clear ();
	string.extend (buffer);
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_reverse_range (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<()>) {
	let string = try_as_string_mutable_ref! (string);
	let mut string = try! (string.string_ref_mut ());
	let mut buffer = unicode_utf8_string_clone_chars (string.as_str ());
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	{
		let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0x5c43164e);
		buffer.reverse ();
	}
	string.clear ();
	string.extend (buffer);
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_copy_range (target_string : &Value, target_start : Option<&Value>, source_string : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<()>) {
	let target_string = try_as_string_mutable_ref! (target_string);
	let mut target_string = try! (target_string.string_ref_mut ());
	let source_string = try_as_string_ref! (source_string);
	let mut target_buffer = unicode_utf8_string_clone_chars (target_string.as_str ());
	let source_buffer = unicode_utf8_string_clone_chars (source_string.string_as_str ());
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_buffer.len ()));
	let (target_start, target_end) = try! (range_coerce (target_start, None, target_buffer.len ()));
	let target_end = if (target_end - target_start) >= (source_end - source_start) {
		target_start + (source_end - source_start)
	} else {
		fail! (0x296b51f4);
	};
	{
		let target_buffer = try_some! (target_buffer.get_mut (target_start .. target_end), 0x193442e0);
		let source_buffer = try_some! (source_buffer.get (source_start .. source_end), 0x91d466f3);
		<[char]>::copy_from_slice (target_buffer, source_buffer);
	}
	target_string.clear ();
	target_string.extend (target_buffer);
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_clone_range (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let mut buffer = StdString::with_capacity (string.string_utf8_bytes_count ());
	for character in try! (RangeIterator::new (string.string_chars (), range_start, range_end)) {
		let character = try! (character);
		buffer.push (character);
	}
	succeed! (string_new (buffer) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_range_to_list (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>, immutable : Option<bool>) -> (Outcome<Value>) {
	let characters = try! (string_range_iterator (string, range_start, range_end));
	return list_collect_from_generator (characters, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_range_to_string (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (list_range_iterator (list, range_start, range_end));
	return string_collect_values_from_generator_ref (characters);
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_range_to_array (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (string_range_iterator (string, range_start, range_end));
	return array_collect_from_generator (characters);
}

#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn array_range_to_string (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (array_range_iterator (array, range_start, range_end));
	return string_collect_values_from_generator_ref (characters);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_range_to_bytes (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let mut buffer = StdString::with_capacity (string.string_utf8_bytes_count ());
	for character in try! (RangeIterator::new (string.string_chars (), range_start, range_end)) {
		let character = try! (character);
		buffer.push (character);
	}
	succeed! (bytes_new (buffer.into_bytes ()) .into ());
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_range_to_string (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, bytes.bytes_count ()));
	if let Ok (string) = str::from_utf8 (& bytes.bytes_as_slice () [range_start..range_end]) {
		succeed! (string_clone_str (string) .into ());
	} else {
		fail! (0xbc4b3840);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_range_iterator <'a> (string : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<Value, StringIterator<'a>>>) {
	let string = try_as_string_ref! (string);
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let iterator = try! (StringIterator::new_a (string));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, range_end));
	succeed! (iterator);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_length (string : &Value) -> (Outcome<usize>) {
	let string = try_as_string_ref! (string);
	succeed! (string.string_chars_count_compute ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_append_2 (string_1 : &Value, string_2 : &Value) -> (Outcome<StdVec<char>>) {
	if try! (is_string_empty_all_2 (string_1, string_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, &string_1));
	try! (vec_string_drain (&mut buffer, &string_2));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_append_3 (string_1 : &Value, string_2 : &Value, string_3 : &Value) -> (Outcome<StdVec<char>>) {
	if try! (is_string_empty_all_3 (string_1, string_2, string_3)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, &string_1));
	try! (vec_string_drain (&mut buffer, &string_2));
	try! (vec_string_drain (&mut buffer, &string_3));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_append_4 (string_1 : &Value, string_2 : &Value, string_3 : &Value, string_4 : &Value) -> (Outcome<StdVec<char>>) {
	if try! (is_string_empty_all_4 (string_1, string_2, string_3, string_4)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, &string_1));
	try! (vec_string_drain (&mut buffer, &string_2));
	try! (vec_string_drain (&mut buffer, &string_3));
	try! (vec_string_drain (&mut buffer, &string_4));
	succeed! (buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_append_n (strings : &[impl StdAsRef<Value>]) -> (Outcome<StdVec<char>>) {
	if strings.is_empty () {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	for string in strings {
		let string = string.as_ref ();
		try! (vec_string_drain (&mut buffer, string));
	}
	succeed! (buffer);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_clone (string : &Value) -> (Outcome<StdVec<char>>) {
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, string));
	succeed! (buffer);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_string_drain (buffer : &mut StdVec<char>, string : &Value) -> (Outcome<()>) {
	let string = try_as_string_ref! (string);
	buffer.extend (string.string_chars ());
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_upper_case (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let string = string.to_uppercase ();
	succeed! (string_new (string) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_lower_case (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let string = string.to_lowercase ();
	succeed! (string_new (string) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_fold_case (string : &Value) -> (Outcome<Value>) {
	TODO! ("actually implement Unicode case-folding instead of delegating to lower-case");
	return string_to_lower_case (string);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_to_upper_case (symbol : &Value) -> (Outcome<Value>) {
	let string = try_as_symbol_ref! (symbol);
	let string = string.string_as_str ();
	let string = string.to_uppercase ();
	succeed! (symbol_new (string) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_to_lower_case (symbol : &Value) -> (Outcome<Value>) {
	let string = try_as_symbol_ref! (symbol);
	let string = string.string_as_str ();
	let string = string.to_lowercase ();
	succeed! (symbol_new (string) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_to_fold_case (symbol : &Value) -> (Outcome<Value>) {
	TODO! ("actually implement Unicode case-folding instead of delegating to lower-case");
	return symbol_to_lower_case (symbol);
}


#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_to_upper_case (keyword : &Value) -> (Outcome<Value>) {
	let string = try_as_keyword_ref! (keyword);
	let string = string.string_as_str ();
	let string = string.to_uppercase ();
	succeed! (keyword_new (string) .into ());
}

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_to_lower_case (keyword : &Value) -> (Outcome<Value>) {
	let string = try_as_keyword_ref! (keyword);
	let string = string.string_as_str ();
	let string = string.to_lowercase ();
	succeed! (keyword_new (string) .into ());
}

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_to_fold_case (keyword : &Value) -> (Outcome<Value>) {
	TODO! ("actually implement Unicode case-folding instead of delegating to lower-case");
	return keyword_to_lower_case (keyword);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_to_upper_case (character : &Value) -> (Outcome<Value>) {
	let character = try_as_character_ref! (character) .value ();
	let mut iterator = character.to_uppercase ();
	if let Some (character_upper) = iterator.next () {
		if iterator.next () == None {
			succeed! (character_upper.into ());
		} else {
			succeed! (character.into ());
		}
	} else {
		succeed! (character.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_to_lower_case (character : &Value) -> (Outcome<Value>) {
	let character = try_as_character_ref! (character) .value ();
	let mut iterator = character.to_lowercase ();
	if let Some (character_lower) = iterator.next () {
		if iterator.next () == None {
			succeed! (character_lower.into ());
		} else {
			succeed! (character.into ());
		}
	} else {
		succeed! (character.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_to_fold_case (character : &Value) -> (Outcome<Value>) {
	TODO! ("actually implement Unicode case-folding instead of delegating to lower-case");
	return character_to_lower_case (character);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_symbol (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	succeed! (symbol_clone_str (string) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_to_string (symbol : &Value) -> (Outcome<Value>) {
	let string = try_as_symbol_ref! (symbol);
	let string = string.string_as_str ();
	succeed! (string_clone_str (string) .into ());
}


#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_keyword (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	succeed! (keyword_clone_str (string) .into ());
}

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_to_string (keyword : &Value) -> (Outcome<Value>) {
	let string = try_as_keyword_ref! (keyword);
	let string = string.string_as_str ();
	succeed! (string_clone_str (string) .into ());
}


#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn symbol_to_keyword (symbol : &Value) -> (Outcome<Value>) {
	let string = try_as_symbol_ref! (symbol);
	let string = string.string_as_str ();
	succeed! (keyword_clone_str (string) .into ());
}

#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_to_symbol (keyword : &Value) -> (Outcome<Value>) {
	let string = try_as_keyword_ref! (keyword);
	let string = string.string_as_str ();
	succeed! (symbol_clone_str (string) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_to_number (string : &Value, radix : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let radix = try! (number_radix_coerce (radix));
	if let Ok (number) = i64::from_str_radix (string, radix.unwrap_or (10)) {
		succeed! (number.into ());
	} else {
		match radix {
			None | Some (10) =>
				if let Ok (number) = f64::from_str (string) {
					succeed! (number.into ());
				} else {
					fail! (0xff199a71);
				},
			_ =>
				fail! (0x6a121e46),
		}
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_to_string (number : &Value, radix : Option<&Value>, sign : Option<bool>) -> (Outcome<Value>) {
	let radix = try! (number_radix_coerce (radix));
	match number.class_match_as_ref () {
		
		ValueClassMatchAsRef::Number (class) =>
			match class {
				
				NumberMatchAsRef::Integer (number) => {
					let number = number.value ();
					let string = if number != 0 {
						let (number, prefix) = if number > 0 {
							match sign {
								None | Some (false) =>
									(number, ""),
								Some (true) =>
									(number, "+"),
							}
						} else {
							if let Some (number) = number.checked_abs () {
								(number, "-")
							} else {
								fail_unimplemented! (0x231c95ca, (github_issue, 43));
							}
						};
						match radix {
							None | Some (10) =>
								format! ("{}{:}", prefix, number),
							Some (2) =>
								format! ("{}{:b}", prefix, number),
							Some (8) =>
								format! ("{}{:o}", prefix, number),
							Some (16) =>
								format! ("{}{:x}", prefix, number),
							_ =>
								fail_unimplemented! (0x3bd46548, (github_issue, 43)),
						}
					} else {
						match sign {
							None | Some (false) =>
								StdString::from ("0"),
							Some (true) =>
								StdString::from ("+0"),
						}
					};
					succeed! (string.into ());
				},
				
				NumberMatchAsRef::Real (number) => {
					let number = number.value ();
					let string = if (number != 0.0) && !number.is_nan () {
						match radix {
							None | Some (10) =>
								match sign {
									None | Some (false) =>
										format! ("{:}", number),
									Some (true) =>
										format! ("{:+}", number),
								},
							_ =>
								fail! (0x4ab1bbce),
						}
					} else if number == 0.0 {
						match sign {
							None | Some (false) =>
								StdString::from ("0"),
							Some (true) =>
								StdString::from ("+0"),
						}
					} else if number.is_nan () {
						StdString::from ("nan")
					} else {
						fail_unreachable! (0xf8a1f4d5, github_issue_new);
					};
					succeed! (string.into ());
				},
				
			},
		
		_ =>
			fail! (0xd2ebac17),
		
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn number_radix_coerce (radix : Option<&Value>) -> (Outcome<Option<u32>>) {
	if let Some (radix) = radix {
		let radix = try_as_number_integer_ref! (radix) .value ();
		if (radix >= 2) && (radix <= 36) {
			succeed! (Some (radix as u32));
		} else {
			fail! (0x32fc5ef5);
		}
	} else {
		succeed! (None);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_to_number (character : &Value) -> (Outcome<Value>) {
	let character = try_as_character_ref! (character) .value ();
	let number = NumberInteger::from (character);
	succeed! (number.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn number_to_character (number : &Value) -> (Outcome<Value>) {
	let number = try_as_number_integer_ref! (number);
	let character = try! (number.try_to_char ());
	succeed! (character.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_to_digit_number (character : &Value, radix : Option<&Value>) -> (Outcome<Value>) {
	let character = try_as_character_ref! (character) .value ();
	let radix = try! (number_radix_coerce (radix)) .unwrap_or (10);
	if let Some (value) = character.to_digit (radix) {
		succeed! (value.into ());
	} else {
		succeed! (false.into ());
	}
}




macro_rules! def_fn_character_predicate_delegate {
	( $predicate : ident, $delegate : ident ) => (
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $predicate (character : &Value) -> (Outcome<bool>) {
			let character = try_as_character_ref! (character) .value ();
			succeed! (character.$delegate () .into ());
		}
	);
}


def_fn_character_predicate_delegate! (character_is_numeric, is_numeric);
def_fn_character_predicate_delegate! (character_is_alphabetic, is_alphabetic);
def_fn_character_predicate_delegate! (character_is_alphabetic_upper_case, is_uppercase);
def_fn_character_predicate_delegate! (character_is_alphabetic_lower_case, is_lowercase);
def_fn_character_predicate_delegate! (character_is_alphabetic_or_numeric, is_alphanumeric);
def_fn_character_predicate_delegate! (character_is_whitespace, is_whitespace);
def_fn_character_predicate_delegate! (character_is_control, is_control);

def_fn_character_predicate_delegate! (character_is_ascii, is_ascii);
def_fn_character_predicate_delegate! (character_is_ascii_numeric, is_ascii_digit);
def_fn_character_predicate_delegate! (character_is_ascii_alphabetic, is_ascii_alphabetic);
def_fn_character_predicate_delegate! (character_is_ascii_alphabetic_upper_case, is_ascii_uppercase);
def_fn_character_predicate_delegate! (character_is_ascii_alphabetic_lower_case, is_ascii_lowercase);
def_fn_character_predicate_delegate! (character_is_ascii_alphabetic_or_numeric, is_ascii_alphanumeric);
def_fn_character_predicate_delegate! (character_is_ascii_whitespace, is_ascii_whitespace);
def_fn_character_predicate_delegate! (character_is_ascii_control, is_ascii_control);
def_fn_character_predicate_delegate! (character_is_ascii_punctuation, is_ascii_punctuation);
def_fn_character_predicate_delegate! (character_is_ascii_graphic, is_ascii_graphic);

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_is_ascii_numeric_base_8 (character : &Value) -> (Outcome<bool>) {
	let character = try_as_character_ref! (character) .value ();
	match character {
		'0' ... '7' =>
			succeed! (true),
		_ =>
			succeed! (false),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn character_is_ascii_numeric_base_16 (character : &Value) -> (Outcome<bool>) {
	let character = try_as_character_ref! (character) .value ();
	match character {
		'0' ... '9' | 'a' ... 'f' | 'A' ... 'F' =>
			succeed! (true),
		_ =>
			succeed! (false),
	}
}

