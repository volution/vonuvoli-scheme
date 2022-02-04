

use super::errors::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		unicode_utf8_char_width,
		unicode_utf8_char_decode,
		unicode_utf8_char_decode_and_width,
		
		unicode_utf8_char_decode_slice_consume,
		unicode_utf8_char_decode_slice_copy_slice,
		unicode_utf8_char_decode_slice_extend_vector,
		unicode_utf8_char_decode_slice_extend_string,
		
	};
	
	pub use super::{
		
		unicode_utf8_chars_clone_string,
		unicode_utf8_string_clone_chars,
		
	};
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_width (byte : u8) -> (usize) {
	// FIXME:  Replace the following with a library perhaps?
	// return core_str::utf8_char_width (byte);
	return UTF8_CHAR_WIDTH[byte as usize] as usize;
}

// FIXME:  From: https://github.com/rust-lang/rust/blob/9bb55dc8642d811d66a7599812009cc063577e00/library/core/src/str/validations.rs#L233
static UTF8_CHAR_WIDTH : [u8; 256] = [
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
	0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode (bytes : &[u8]) -> (Outcome<char>) {
	if let Some (code) = unsafe { core_str::next_code_point (&mut bytes.iter ()) } {
		unsafe {
			let char = core_char::from_u32_unchecked (code);
			succeed! (char);
		}
	} else {
		fail! (0xbfa8c535);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode_and_width (bytes : &[u8]) -> (Outcome<(char, usize)>) {
	let char_byte_0 = bytes[0];
	let char_width = unicode_utf8_char_width (char_byte_0);
	if char_width == 0 {
		fail! (0x89540f2c);
	} else if char_width > bytes.len () {
		fail! (0x85f3391e);
	} else {
		let char = r#try! (unicode_utf8_char_decode (bytes));
		succeed! ((char, char_width));
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode_slice_consume <Consumer> (bytes : &[u8], limit_count : Option<usize>, limit_char : Option<char>, consumer : Consumer) -> (Outcome<(usize, usize, bool)>)
		where Consumer : FnMut (char) -> ()
{
	let mut consumer = consumer;
	let mut chars_remaining = limit_count.unwrap_or (usize::max_value ());
	let mut bytes_remaining = bytes;
	let mut chars_accumulated = 0;
	let mut bytes_accumulated = 0;
	let mut matched = false;
	loop {
		if (chars_remaining == 0) || bytes_remaining.is_empty () {
			break;
		}
		let char_byte_0 = bytes_remaining[0];
		let char_width = unicode_utf8_char_width (char_byte_0);
		if char_width == 0 {
			if chars_accumulated > 0 {
				break;
			} else {
				fail! (0x000704f5);
			}
		} else if char_width > bytes_remaining.len () {
			break;
		} else {
			let char = if let Some (code) = unsafe { core_str::next_code_point (&mut bytes_remaining.iter ()) } {
				unsafe { core_char::from_u32_unchecked (code) }
			} else {
				if chars_accumulated > 0 {
					break;
				} else {
					fail! (0xc721276f);
				}
			};
			consumer (char);
			chars_accumulated += 1;
			bytes_accumulated += char_width;
			chars_remaining -= 1;
			bytes_remaining = &bytes_remaining[char_width..];
			if let Some (limit_char) = limit_char {
				if limit_char == char {
					matched = true;
					break;
				}
			}
		}
	}
	succeed! ((chars_accumulated, bytes_accumulated, matched));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode_slice_copy_slice (bytes : &[u8], limit_count : Option<usize>, limit_char : Option<char>, target : &mut [char]) -> (Outcome<(usize, usize, bool)>) {
	let limit_count = Some (usize::min (target.len (), limit_count.unwrap_or (usize::max_value ())));
	let mut offset = 0;
	return unicode_utf8_char_decode_slice_consume (bytes, limit_count, limit_char, |char| { target[offset] = char; offset += 1; });
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode_slice_extend_vector (bytes : &[u8], limit_count : Option<usize>, limit_char : Option<char>, target : &mut StdVec<char>) -> (Outcome<(usize, usize, bool)>) {
	return unicode_utf8_char_decode_slice_consume (bytes, limit_count, limit_char, |char| target.push (char));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_char_decode_slice_extend_string (bytes : &[u8], limit_count : Option<usize>, limit_char : Option<char>, target : &mut StdString) -> (Outcome<(usize, usize, bool)>) {
	return unicode_utf8_char_decode_slice_consume (bytes, limit_count, limit_char, |char| target.push (char));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_chars_clone_string (characters : &[char]) -> (StdString) {
	let mut buffer = StdString::with_capacity (characters.len ());
	for character in characters {
		buffer.push (*character);
	}
	buffer
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn unicode_utf8_string_clone_chars (characters : &str) -> (StdVec<char>) {
	let mut buffer = StdVec::with_capacity (characters.len ());
	for character in characters.chars () {
		buffer.push (character);
	}
	buffer
}

