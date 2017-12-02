

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
	pub use super::{string_collect_chars_from_generator, string_collect_values_from_generator};
	pub use super::{string_build_1, string_build_2, string_build_3, string_build_4, string_build_n};
	pub use super::{string_append_2, string_append_3, string_append_4, string_append_n};
	pub use super::{string_make, string_clone, string_reverse};
	pub use super::{string_fill_range, string_reverse_range, string_copy_range, string_clone_range};
	pub use super::{string_range_to_list, list_range_to_string};
	pub use super::{string_range_to_array, array_range_to_string};
	pub use super::{string_range_to_bytes, bytes_range_to_string};
	pub use super::{string_range_iterator};
	pub use super::{string_length};
	
	pub use super::{vec_string_append_2, vec_string_append_3, vec_string_append_4, vec_string_append_n};
	pub use super::{vec_string_clone, vec_string_drain};
	
	pub use super::{StringIterator, StringIterators};
	
	pub use super::{
			string_to_upper_case, string_to_lower_case, string_to_fold_case,
			character_to_upper_case, character_to_lower_case, character_to_fold_case,
	};
	
	pub use super::{
			string_to_symbol, symbol_to_string,
			string_to_number, number_to_string,
			character_to_number, number_to_character, character_to_digit_number,
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




pub fn string_at (string : &Value, index : usize) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	if let Some (char) = string.string_char_at_compute (index) {
		succeed! (char.into ());
	} else {
		fail! (0x0eb98e3f);
	}
}

pub fn string_at_set (_string : &Value, _index : usize, _char : &Value) -> (Outcome<Value>) {
	fail_unimplemented! (0xc8a46002); // deferred
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




pub fn string_collect_chars_from_generator <Source> (chars : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<char>>
{
	let chars = try! (chars.collect::<Outcome<StdVec<_>>> ());
	succeed! (string_collect_chars (chars));
}

pub fn string_collect_values_from_generator <Source> (chars : Source) -> (Outcome<Value>)
		where Source : iter::Iterator<Item = Outcome<Value>>
{
	let chars = try! (chars.collect::<Outcome<StdVec<_>>> ());
	return string_collect_values (chars);
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




pub fn string_fill_range (string : &Value, fill : Option<&Value>, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let _string = try_as_string_ref! (string);
	let _fill = if let Some (fill) = fill {
		try_as_character_ref! (fill) .value ()
	} else {
		0 as char
	};
	let (_range_start, _range_end) = try! (range_coerce_unbounded (range_start, range_end));
	fail_unimplemented! (0x7eaac146); // deferred
}


pub fn string_reverse_range (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let _string = try_as_string_ref! (string);
	let (_range_start, _range_end) = try! (range_coerce_unbounded (range_start, range_end));
	fail_unimplemented! (0x46884d97); // deferred
}


pub fn string_copy_range (target_string : &Value, target_start : Option<&Value>, source_string : &Value, source_start : Option<&Value>, source_end : Option<&Value>) -> (Outcome<Value>) {
	let _target_string = try_as_string_ref! (target_string);
	let _source_string = try_as_string_ref! (source_string);
	let (_source_start, _source_end) = try! (range_coerce_unbounded (source_start, source_end));
	let (_target_start, _target_end) = try! (range_coerce_unbounded (target_start, None));
	fail_unimplemented! (0xf216023f); // deferred
}


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




pub fn string_range_to_list (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (string_range_iterator (string, range_start, range_end));
	return list_collect_from_generator (characters);
}

pub fn list_range_to_string (list : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (list_range_iterator (list, range_start, range_end));
	return string_collect_values_from_generator (characters);
}


pub fn string_range_to_array (string : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (string_range_iterator (string, range_start, range_end));
	return array_collect_from_generator (characters);
}

pub fn array_range_to_string (array : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let characters = try! (array_range_iterator (array, range_start, range_end));
	return string_collect_values_from_generator (characters);
}


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

pub fn bytes_range_to_string (bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, bytes.values_length ()));
	if let Ok (string) = str::from_utf8 (& bytes.values_as_slice () [range_start..range_end]) {
		succeed! (string_clone_str (string) .into ());
	} else {
		fail! (0xbc4b3840);
	}
}




pub fn string_range_iterator <'a> (string : &'a Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<RangeIteratorForOutcome<Value, StringIterator<'a>>>) {
	let string = try_as_string_ref! (string);
	let (range_start, range_end) = try! (range_coerce_unbounded (range_start, range_end));
	let iterator = try! (StringIterator::new_a (string));
	let iterator = try! (RangeIteratorForOutcome::new (iterator, range_start, range_end));
	succeed! (iterator);
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
		return StringIterator::new_a (string);
	}
	
	pub fn new_a (string : &'a String) -> (Outcome<StringIterator<'a>>) {
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




pub fn string_to_upper_case (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string) .string_as_str ();
	let string = string.to_uppercase ();
	succeed! (string_new (string) .into ());
}

pub fn string_to_lower_case (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string) .string_as_str ();
	let string = string.to_lowercase ();
	succeed! (string_new (string) .into ());
}

pub fn string_to_fold_case (string : &Value) -> (Outcome<Value>) {
	let _string = try_as_string_ref! (string);
	fail_unimplemented! (0x36a59db6); // deferred
}


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

pub fn character_to_fold_case (character : &Value) -> (Outcome<Value>) {
	let _character = try_as_character_ref! (character);
	fail_unimplemented! (0xd2078d69); // deferred
}




pub fn string_to_symbol (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string) .string_as_str ();
	succeed! (symbol_clone_str (string) .into ());
}

pub fn symbol_to_string (symbol : &Value) -> (Outcome<Value>) {
	let string = try_as_symbol_ref! (symbol) .string_as_str ();
	succeed! (string_clone_str (string) .into ());
}




pub fn string_to_number (string : &Value, radix : Option<&Value>) -> (Outcome<Value>) {
	use std::str::FromStr;
	let string = try_as_string_ref! (string) .string_as_str ();
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

pub fn number_to_string (number : &Value, radix : Option<&Value>, sign : Option<bool>) -> (Outcome<Value>) {
	let radix = try! (number_radix_coerce (radix));
	match number.class () {
		
		ValueClass::NumberInteger => {
			let number = NumberInteger::as_ref (number) .value ();
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
						fail_unimplemented! (0x231c95ca); // deferred
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
						fail_unimplemented! (0x3bd46548), // deferred
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
		
		ValueClass::NumberReal => {
			let number = NumberReal::as_ref (number) .value ();
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
				fail_unreachable! (0xf8a1f4d5);
			};
			succeed! (string.into ());
		},
		
		_ =>
			fail! (0xd2ebac17),
		
	}
}

pub fn number_radix_coerce (radix : Option<&Value>) -> (Outcome<Option<u32>>) {
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




pub fn character_to_number (character : &Value) -> (Outcome<Value>) {
	let character = try_as_character_ref! (character) .value ();
	let number = NumberInteger::from (character);
	succeed! (number.into ());
}

pub fn number_to_character (number : &Value) -> (Outcome<Value>) {
	let number = try_as_number_integer_ref! (number);
	let character = try! (number.try_to_char ());
	succeed! (character.into ());
}




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

pub fn character_is_ascii_numeric_base_8 (character : &Value) -> (Outcome<bool>) {
	let character = try_as_character_ref! (character) .value ();
	match character {
		'0' ... '7' =>
			succeed! (true),
		_ =>
			succeed! (false),
	}
}

pub fn character_is_ascii_numeric_base_16 (character : &Value) -> (Outcome<bool>) {
	let character = try_as_character_ref! (character) .value ();
	match character {
		'0' ... '9' | 'a' ... 'f' | 'A' ... 'F' =>
			succeed! (true),
		_ =>
			succeed! (false),
	}
}

