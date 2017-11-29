

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;
use std::str;




pub mod exports {
	
	pub use super::{string_at, string_at_set};
	
	pub use super::{string_empty};
	pub use super::{string_collect_chars, string_collect_values};
	pub use super::{string_build_1, string_build_2, string_build_3, string_build_4, string_build_n};
	pub use super::{string_append_2, string_append_3, string_append_4, string_append_n};
	pub use super::{string_make, string_clone, string_reverse};
	pub use super::{string_fill_range, string_reverse_range, string_copy_range, string_clone_range};
	pub use super::{string_length};
	
	pub use super::{vec_string_append_2, vec_string_append_3, vec_string_append_4, vec_string_append_n};
	pub use super::{vec_string_clone, vec_string_drain};
	
	pub use super::{StringIterator, StringIterators};
	
}




pub fn string_at (string : &Value, index : usize) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	if let Some (char) = string.string_char_at_compute (index) {
		succeed! (char.into ());
	} else {
		fail! (0x0eb98e3f);
	}
}

pub fn string_at_set (_string : &Value, _index : usize, _char : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xc8a46002);
}




pub fn string_collect_chars <Source> (chars : Source) -> (Value)
		where Source : iter::IntoIterator<Item = char>, Source::IntoIter : iter::DoubleEndedIterator
{
	use std::iter::FromIterator;
	return string_new (FromIterator::from_iter (chars)) .into ();
}

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




pub fn string_empty () -> (Value) {
	return string_new (StdString::new ()) .into ();
}

pub fn string_build_1 (char_1 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (1);
	buffer.push (try_as_character_ref! (char_1) .value ());
	succeed! (string_new (buffer) .into ());
}

pub fn string_build_2 (char_1 : &Value, char_2 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (2);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	succeed! (string_new (buffer) .into ());
}

pub fn string_build_3 (char_1 : &Value, char_2 : &Value, char_3 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (3);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	buffer.push (try_as_character_ref! (char_3) .value ());
	succeed! (string_new (buffer) .into ());
}

pub fn string_build_4 (char_1 : &Value, char_2 : &Value, char_3 : &Value, char_4 : &Value) -> (Outcome<Value>) {
	let mut buffer = StdString::with_capacity (4);
	buffer.push (try_as_character_ref! (char_1) .value ());
	buffer.push (try_as_character_ref! (char_2) .value ());
	buffer.push (try_as_character_ref! (char_3) .value ());
	buffer.push (try_as_character_ref! (char_4) .value ());
	succeed! (string_new (buffer) .into ());
}

pub fn string_build_n (chars : &[Value]) -> (Outcome<Value>) {
	match chars.len () {
		0 =>
			succeed! (string_empty ()),
		1 =>
			return string_build_1 (&chars[0]),
		2 =>
			return string_build_2 (&chars[0], &chars[1]),
		3 =>
			return string_build_3 (&chars[0], &chars[1], &chars[2]),
		4 =>
			return string_build_4 (&chars[0], &chars[1], &chars[2], &chars[3]),
		_ =>
			(),
	}
	let mut buffer = StdString::with_capacity (chars.len ());
	for char in chars {
		buffer.push (try_as_character_ref! (char) .value ());
	}
	succeed! (string_new (buffer) .into ());
}




pub fn string_append_2 (string_1 : &Value, string_2 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_2 (string_1, string_2));
	succeed! (string_collect_chars (buffer) .into ());
}

pub fn string_append_3 (string_1 : &Value, string_2 : &Value, string_3 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_3 (string_1, string_2, string_3));
	succeed! (string_collect_chars (buffer) .into ());
}

pub fn string_append_4 (string_1 : &Value, string_2 : &Value, string_3 : &Value, string_4 : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_append_4 (string_1, string_2, string_3, string_4));
	succeed! (string_collect_chars (buffer) .into ());
}

pub fn string_append_n (strings : &[Value]) -> (Outcome<Value>) {
	match strings.len () {
		0 =>
			succeed! (string_empty ()),
		1 =>
			succeed! (strings[0].clone ()),
		2 =>
			return string_append_2 (&strings[0], &strings[1]),
		3 =>
			return string_append_3 (&strings[0], &strings[1], &strings[2]),
		4 =>
			return string_append_4 (&strings[0], &strings[1], &strings[2], &strings[3]),
		_ =>
			(),
	}
	let buffer = try! (vec_string_append_n (strings));
	succeed! (string_collect_chars (buffer) .into ());
}




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

pub fn string_clone (string : &Value) -> (Outcome<Value>) {
	let buffer = try! (vec_string_clone (string));
	succeed! (string_collect_chars (buffer) .into ());
}

pub fn string_reverse (string : &Value) -> (Outcome<Value>) {
	// FIXME:  Optimize the vector allocation!
	let buffer = try! (vec_string_clone (string));
	succeed! (string_collect_chars (buffer.into_iter () .rev ()));
}




pub fn string_fill_range (string : &Value, fill : Option<&Value>, start : Option<&Value>, end : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let _fill = if let Some (fill) = fill {
		try_as_character_ref! (fill) .value ()
	} else {
		0 as char
	};
	let (_start, _end) = try! (range_coerce (start, end, string.string_chars_count_compute ()));
	fail_unimplemented! (0x7eaac146);
}


pub fn string_reverse_range (string : &Value, start : Option<&Value>, end : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let (_start, _end) = try! (range_coerce (start, end, string.string_chars_count_compute ()));
	fail_unimplemented! (0x46884d97);
}


pub fn string_copy_range (target_string : &Value, start : Option<&Value>, source_string : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<Value>) {
	let target_string = try_as_string_ref! (target_string);
	let source_string = try_as_string_ref! (source_string);
	let (source_start, source_end) = try! (range_coerce (source_start, source_end, source_string.string_chars_count_compute ()));
	let (target_start, target_end) = try! (range_coerce (start, None, target_string.string_chars_count_compute ()));
	if (target_end - target_start) < (source_end - source_start) {
		fail! (0x7033eb20);
	}
	fail_unimplemented! (0xf216023f);
}


pub fn string_clone_range (string : &Value, start : Option<&Value>, end : Option<&Value>) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let (_start, _end) = try! (range_coerce (start, end, string.string_chars_count_compute ()));
	fail_unimplemented! (0x78c93665);
}




pub fn string_length (string : &Value) -> (Outcome<usize>) {
	let string = try_as_string_ref! (string);
	succeed! (string.string_chars_count_compute ());
}




pub fn vec_string_append_2 (string_1 : &Value, string_2 : &Value) -> (Outcome<StdVec<char>>) {
	if try! (is_string_empty_all_2 (string_1, string_2)) {
		succeed! (StdVec::new ());
	}
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, &string_1));
	try! (vec_string_drain (&mut buffer, &string_2));
	succeed! (buffer);
}

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

pub fn vec_string_append_n (strings : &[Value]) -> (Outcome<StdVec<char>>) {
	match strings.len () {
		0 =>
			succeed! (StdVec::new ()),
		1 =>
			return vec_string_clone (&strings[0]),
		2 =>
			return vec_string_append_2 (&strings[0], &strings[1]),
		3 =>
			return vec_string_append_3 (&strings[0], &strings[1], &strings[2]),
		4 =>
			return vec_string_append_4 (&strings[0], &strings[1], &strings[2], &strings[3]),
		_ =>
			(),
	}
	let mut buffer = StdVec::new ();
	for string in strings {
		try! (vec_string_drain (&mut buffer, &string));
	}
	succeed! (buffer);
}




pub fn vec_string_clone (string : &Value) -> (Outcome<StdVec<char>>) {
	let mut buffer = StdVec::new ();
	try! (vec_string_drain (&mut buffer, string));
	succeed! (buffer);
}


pub fn vec_string_drain (buffer : &mut StdVec<char>, string : &Value) -> (Outcome<()>) {
	let string = try_as_string_ref! (string);
	buffer.extend (string.string_chars ());
	succeed! (());
}




pub struct StringIterator <'a> ( &'a String, str::Chars<'a> );


impl <'a> StringIterator <'a> {
	
	pub fn new (string : &'a Value) -> (Outcome<StringIterator<'a>>) {
		let string = try_as_string_ref! (string);
		succeed! (StringIterator (string, string.string_chars ()));
	}
}


impl <'a> Iterator for StringIterator <'a> {
	
	type Item = Outcome<Value>;
	
	fn next (&mut self) -> (Option<Outcome<Value>>) {
		if let Some (value) = self.1.next () {
			return Some (succeeded! (character (value) .into ()));
		} else {
			return None;
		}
	}
}




pub struct StringIterators <'a> ( StdVec<StringIterator<'a>> );


impl <'a> StringIterators <'a> {
	
	pub fn new (strings : &'a [Value]) -> (Outcome<StringIterators<'a>>) {
		let iterators = try! (strings.iter () .map (|string| StringIterator::new (string)) .collect ());
		succeed! (StringIterators (iterators));
	}
}


impl <'a> Iterator for StringIterators <'a> {
	
	type Item = Outcome<StdVec<Value>>;
	
	fn next (&mut self) -> (Option<Outcome<StdVec<Value>>>) {
		let mut outcomes = StdVec::with_capacity (self.0.len ());
		for mut iterator in self.0.iter_mut () {
			match iterator.next () {
				Some (Ok (outcome)) =>
					outcomes.push (outcome),
				Some (Err (error)) =>
					return Some (Err (error)),
				None =>
					return None,
			}
		}
		return Some (succeeded! (outcomes));
	}
}


