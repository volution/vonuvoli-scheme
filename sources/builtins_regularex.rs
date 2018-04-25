

#[ allow (unused_imports) ]
use super::builtins::exports::*;
#[ allow (unused_imports) ]
use super::constants::exports::*;
#[ allow (unused_imports) ]
use super::errors::exports::*;
#[ allow (unused_imports) ]
use super::values::exports::*;

#[ allow (unused_imports) ]
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
pub fn string_regex_match_extract_first (pattern : &Value, string : &Value) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (matched) = pattern.find (string) {
		let extract = string_clone_str (matched.as_str ());
		succeed! (extract.into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_extract_all (pattern : &Value, string : &Value, return_array : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut extracts = StdVec::new ();
	for matched in pattern.find_iter (string) {
		let extract = string_clone_str (matched.as_str ());
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_position_first (pattern : &Value, string : &Value) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (matched) = pattern.find (string) {
		return string_regex_match_position_0 (string, &matched);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_position_all (pattern : &Value, string : &Value, return_array : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut positions = StdVec::new ();
	for matched in pattern.find_iter (string) {
		let position = try! (string_regex_match_position_0 (string, &matched));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_position_0 (_string : &str, _matched : &ext::regex::Match) -> (Outcome<Value>) {
	fail_unimplemented! (0x74964611);  // FIXME:  Implement This!
}





#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_extract_first (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (captures) = pattern.captures (string) {
		return string_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_extract_all (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut extracts = StdVec::new ();
	for captures in pattern.captures_iter (string) {
		let extract = try! (string_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names));
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_captures_extract_0 (pattern : &ext::regex::Regex, captures : &ext::regex::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let mut extracts = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let extract = if let Some (matched) = matched {
			string_clone_str (matched.as_str ()) .into ()
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
			pair_new (name, extract, None) .into ()
		} else {
			extract
		};
		extracts.push (extract);
	}
	return build_list_or_array (extracts, return_array);
}




#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_position_first (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	if let Some (captures) = pattern.captures (string) {
		return string_regex_match_captures_position_0 (pattern, string, &captures, return_array, return_assoc, assoc_use_names);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn string_regex_match_captures_position_all (pattern : &Value, string : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_string_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	let mut positions = StdVec::new ();
	for captures in pattern.captures_iter (string) {
		let position = try! (string_regex_match_captures_position_0 (pattern, string, &captures, return_array, return_assoc, assoc_use_names));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array);
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn string_regex_match_captures_position_0 (pattern : &ext::regex::Regex, string : &str, captures : &ext::regex::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let mut positions = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let position = if let Some (matched) = matched {
			try! (string_regex_match_position_0 (string, &matched))
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
			pair_new (name, position, None) .into ()
		} else {
			position
		};
		positions.push (position);
	}
	return build_list_or_array (positions, return_array);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_compile (pattern : &Value) -> (Outcome<BytesRegex>) {
	let pattern = try_as_string_ref! (pattern);
	let pattern = pattern.string_as_str ();
	let builder = ext::regex::bytes::RegexBuilder::new (pattern);
	let pattern = try_or_fail! (builder.build (), 0xab76df41);
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
pub fn bytes_regex_match_extract_first (pattern : &Value, bytes : &Value) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (matched) = pattern.find (bytes) {
		let extract = bytes_clone_slice (matched.as_bytes ());
		succeed! (extract.into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_extract_all (pattern : &Value, bytes : &Value, return_array : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut extracts = StdVec::new ();
	for matched in pattern.find_iter (bytes) {
		let extract = bytes_clone_slice (matched.as_bytes ());
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_position_first (pattern : &Value, bytes : &Value) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (matched) = pattern.find (bytes) {
		return bytes_regex_match_position_0 (bytes, &matched);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_position_all (pattern : &Value, bytes : &Value, return_array : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut positions = StdVec::new ();
	for matched in pattern.find_iter (bytes) {
		let position = try! (bytes_regex_match_position_0 (bytes, &matched));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_position_0 (_bytes : &[u8], _matched : &ext::regex::bytes::Match) -> (Outcome<Value>) {
	fail_unimplemented! (0x1c538088);  // FIXME:  Implement This!
}





#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_extract_first (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (captures) = pattern.captures (bytes) {
		return bytes_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_extract_all (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut extracts = StdVec::new ();
	for captures in pattern.captures_iter (bytes) {
		let extract = try! (bytes_regex_match_captures_extract_0 (pattern, &captures, return_array, return_assoc, assoc_use_names));
		extracts.push (extract);
	}
	return build_list_or_array_or_false_if_empty (extracts, return_array);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_captures_extract_0 (pattern : &ext::regex::bytes::Regex, captures : &ext::regex::bytes::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let mut extracts = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let extract = if let Some (matched) = matched {
			bytes_clone_slice (matched.as_bytes ()) .into ()
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
			pair_new (name, extract, None) .into ()
		} else {
			extract
		};
		extracts.push (extract);
	}
	return build_list_or_array (extracts, return_array);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_position_first (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	if let Some (captures) = pattern.captures (bytes) {
		return bytes_regex_match_captures_position_0 (pattern, bytes, &captures, return_array, return_assoc, assoc_use_names);
	} else {
		succeed! (FALSE_VALUE);
	}
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bytes_regex_match_captures_position_all (pattern : &Value, bytes : &Value, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let pattern = try_as_bytes_regex_ref! (pattern);
	let pattern = pattern.regex_ref ();
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	let mut positions = StdVec::new ();
	for captures in pattern.captures_iter (bytes) {
		let position = try! (bytes_regex_match_captures_position_0 (pattern, bytes, &captures, return_array, return_assoc, assoc_use_names));
		positions.push (position);
	}
	return build_list_or_array_or_false_if_empty (positions, return_array);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn bytes_regex_match_captures_position_0 (pattern : &ext::regex::bytes::Regex, bytes : &[u8], captures : &ext::regex::bytes::Captures, return_array : bool, return_assoc : bool, assoc_use_names : bool) -> (Outcome<Value>) {
	let mut positions = StdVec::new ();
	for (index, (name, matched)) in pattern.capture_names () .zip (captures.iter ()) .enumerate () {
		let position = if let Some (matched) = matched {
			try! (bytes_regex_match_position_0 (bytes, &matched))
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
			pair_new (name, position, None) .into ()
		} else {
			position
		};
		positions.push (position);
	}
	return build_list_or_array (positions, return_array);
}

