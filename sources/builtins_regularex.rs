

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::{
			
			string_regex_compile,
			
			string_regex_matches,
			
			string_regex_match_extract_first,
			string_regex_match_extract_all,
			string_regex_match_position_first,
			string_regex_match_position_all,
			
			string_regex_match_captures_extract_first,
			string_regex_match_captures_extract_all,
			string_regex_match_captures_position_first,
			string_regex_match_captures_position_all,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
			
			bytes_regex_compile,
			
			bytes_regex_matches,
			
			bytes_regex_match_extract_first,
			bytes_regex_match_extract_all,
			bytes_regex_match_position_first,
			bytes_regex_match_position_all,
			
			bytes_regex_match_captures_extract_first,
			bytes_regex_match_captures_extract_all,
			bytes_regex_match_captures_position_first,
			bytes_regex_match_captures_position_all,
			
		};
	
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_compile (pattern : &Value) -> (Outcome<StringRegex>) {
	let pattern = try_as_string_ref! (pattern);
	let pattern = pattern.string_as_str ();
	let builder = ext::regex::RegexBuilder::new (pattern);
	let pattern = try_or_fail! (builder.build (), 0xb949aaf5);
	succeed! (StringRegex::new (pattern));
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_matches (pattern : &Value, string : &Value) -> (Outcome<bool>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let matches = pattern.is_match (string);
	succeed! (matches);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_extract_first (pattern : &Value, string : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (matched) = pattern.find (string) {
		let extract = string_clone_str (matched.as_str (), immutable);
		succeed! (extract);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_extract_all (pattern : &Value, string : &Value, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut extracts = StdVec::new ();
	for matched in pattern.find_iter (string) {
		let extract = string_clone_str (matched.as_str (), immutable);
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array, immutable);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_position_first (pattern : &Value, string : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (matched) = pattern.find (string) {
		return string_regex_match_position_0 (string, &matched, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_position_all (pattern : &Value, string : &Value, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut positions = StdVec::new ();
	for matched in pattern.find_iter (string) {
		let position = r#try! (string_regex_match_position_0 (string, &matched, immutable));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_position_0 (string : &str, matched : &ext::regex::Match, immutable : Option<bool>) -> (Outcome<Value>) {
	TODO! ("optimize this code by caching previously seen character indices");
	let byte_start = matched.start ();
	let byte_end = matched.end ();
	let mut character_start = None;
	let mut character_end = None;
	let mut character_count = 0;
	for (character_offset, (byte_offset, _character)) in string.char_indices () .enumerate () {
		if byte_offset == byte_start {
			character_start = Some (character_offset);
		}
		if byte_offset == byte_end {
			character_end = Some (character_offset);
			break;
		}
		character_count += 1;
	}
	let character_start = character_start.unwrap_or (character_count);
	let character_end = character_end.unwrap_or (character_count);
	let start = r#try! (NumberInteger::try_from (character_start));
	let end = r#try! (NumberInteger::try_from (character_end));
	let position = pair_new (start.into (), end.into (), immutable);
	succeed! (position);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_extract_first (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (captures) = pattern.captures (string) {
		return string_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_extract_all (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut extracts = StdVec::new ();
	for captures in pattern.captures_iter (string) {
		let extract = r#try! (string_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names, immutable));
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_captures_extract_0 (pattern : &ext::regex::Regex, captures : &ext::regex::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut extracts = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let extract = if let Some (matched) = matched {
			string_clone_str (matched.as_str (), immutable)
		} else {
			FALSE_VALUE
		};
		let extract = if return_assoc {
			let name = if assoc_use_names {
				if let Some (name) = name {
					symbol_clone_str (name) .into ()
				} else {
					number_i64 (index as i64) .into ()
				}
			} else {
				number_i64 (index as i64) .into ()
			};
			pair_new (name, extract, immutable)
		} else {
			extract
		};
		extracts.push (extract);
	}
	return build_list_or_array (extracts, return_array, immutable);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_position_first (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (captures) = pattern.captures (string) {
		return string_regex_match_captures_position_0 (pattern, string, &captures, return_array, return_assoc, assoc_use_names, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_position_all (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut positions = StdVec::new ();
	for captures in pattern.captures_iter (string) {
		let position = r#try! (string_regex_match_captures_position_0 (pattern, string, &captures, return_array, return_assoc, assoc_use_names, immutable));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_captures_position_0 (pattern : &ext::regex::Regex, string : &str, captures : &ext::regex::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut positions = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let position = if let Some (matched) = matched {
			r#try! (string_regex_match_position_0 (string, &matched, immutable))
		} else {
			FALSE_VALUE
		};
		let position = if return_assoc {
			let name = if assoc_use_names {
				if let Some (name) = name {
					symbol_clone_str (name) .into ()
				} else {
					number_i64 (index as i64) .into ()
				}
			} else {
				number_i64 (index as i64) .into ()
			};
			pair_new (name, position, immutable)
		} else {
			position
		};
		positions.push (position);
	}
	return build_list_or_array (positions, return_array, immutable);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_compile (pattern : &Value) -> (Outcome<BytesRegex>) {
	let pattern = try_as_string_ref! (pattern);
	let pattern = pattern.string_as_str ();
	let builder = ext::regex::bytes::RegexBuilder::new (pattern);
	let pattern = try_or_fail! (builder.build (), 0xab76df41);
	succeed! (BytesRegex::new (pattern));
}

#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( not ( feature = "vonuvoli_values_string" ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_compile (pattern : &Value) -> (Outcome<BytesRegex>) {
	let pattern = try_as_bytes_ref! (pattern);
	let pattern = pattern.bytes_as_slice ();
	let pattern = try_or_fail! (str::from_utf8 (pattern), 0xe9fd2a3f);
	let builder = ext::regex::bytes::RegexBuilder::new (pattern);
	let pattern = try_or_fail! (builder.build (), 0x6fbbc584);
	succeed! (BytesRegex::new (pattern));
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_matches (pattern : &Value, bytes : &Value) -> (Outcome<bool>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let matches = pattern.is_match (bytes);
	succeed! (matches);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_extract_first (pattern : &Value, bytes : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (matched) = pattern.find (bytes) {
		let extract = bytes_clone_slice (matched.as_bytes (), immutable);
		succeed! (extract);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_extract_all (pattern : &Value, bytes : &Value, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut extracts = StdVec::new ();
	for matched in pattern.find_iter (bytes) {
		let extract = bytes_clone_slice (matched.as_bytes (), immutable);
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array, immutable);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_position_first (pattern : &Value, bytes : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (matched) = pattern.find (bytes) {
		return bytes_regex_match_position_0 (bytes, &matched, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_position_all (pattern : &Value, bytes : &Value, return_array : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut positions = StdVec::new ();
	for matched in pattern.find_iter (bytes) {
		let position = r#try! (bytes_regex_match_position_0 (bytes, &matched, immutable));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_position_0 (_bytes : &[u8], matched : &ext::regex::bytes::Match, immutable : Option<bool>) -> (Outcome<Value>) {
	let start = matched.start ();
	let end = matched.end ();
	let start = r#try! (NumberInteger::try_from (start));
	let end = r#try! (NumberInteger::try_from (end));
	let position = pair_new (start.into (), end.into (), immutable);
	succeed! (position);
}





#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_extract_first (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (captures) = pattern.captures (bytes) {
		return bytes_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_extract_all (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut extracts = StdVec::new ();
	for captures in pattern.captures_iter (bytes) {
		let extract = r#try! (bytes_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names, immutable));
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_captures_extract_0 (pattern : &ext::regex::bytes::Regex, captures : &ext::regex::bytes::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut extracts = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let extract = if let Some (matched) = matched {
			bytes_clone_slice (matched.as_bytes (), immutable)
		} else {
			FALSE_VALUE
		};
		let extract = if return_assoc {
			let name = if assoc_use_names {
				if let Some (name) = name {
					symbol_clone_str (name) .into ()
				} else {
					number_i64 (index as i64) .into ()
				}
			} else {
				number_i64 (index as i64) .into ()
			};
			pair_new (name, extract, immutable)
		} else {
			extract
		};
		extracts.push (extract);
	}
	return build_list_or_array (extracts, return_array, immutable);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_position_first (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (captures) = pattern.captures (bytes) {
		return bytes_regex_match_captures_position_0 (pattern, bytes, &captures, return_array, return_assoc, assoc_use_names, immutable);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_position_all (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut positions = StdVec::new ();
	for captures in pattern.captures_iter (bytes) {
		let position = r#try! (bytes_regex_match_captures_position_0 (pattern, bytes, &captures, return_array, return_assoc, assoc_use_names, immutable));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array, immutable);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_captures_position_0 (pattern : &ext::regex::bytes::Regex, bytes : &[u8], captures : &ext::regex::bytes::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool, immutable : Option<bool>) -> (Outcome<Value>) {
	let mut positions = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let position = if let Some (matched) = matched {
			r#try! (bytes_regex_match_position_0 (bytes, &matched, immutable))
		} else {
			FALSE_VALUE
		};
		let position = if return_assoc {
			let name = if assoc_use_names {
				if let Some (name) = name {
					symbol_clone_str (name) .into ()
				} else {
					number_i64 (index as i64) .into ()
				}
			} else {
				number_i64 (index as i64) .into ()
			};
			pair_new (name, position, immutable)
		} else {
			position
		};
		positions.push (position);
	}
	return build_list_or_array (positions, return_array, immutable);
}

