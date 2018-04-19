

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
	fail_unimplemented! (0x74964611);
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

