

use super::errors::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		unicode_utf8_char_width,
		unicode_utf8_char_decode,
		unicode_utf8_char_decode_and_width,
	};
	
	pub use super::{
		unicode_utf8_chars_clone,
	};
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn unicode_utf8_char_width (byte : u8) -> (usize) {
	return core_str::utf8_char_width (byte);
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn unicode_utf8_char_decode (bytes : &[u8]) -> (Outcome<char>) {
	if let Some (code) = core_str::next_code_point (&mut bytes.iter ()) {
		unsafe {
			let char = core_char::from_u32_unchecked (code);
			succeed! (char);
		}
	} else {
		fail! (0xbfa8c535);
	}
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn unicode_utf8_char_decode_and_width (bytes : &[u8]) -> (Outcome<(char, usize)>) {
	let char_byte_0 = bytes[0];
	let char_width = unicode_utf8_char_width (char_byte_0);
	if char_width == 0 {
		fail! (0x89540f2c);
	} else if char_width > bytes.len () {
		fail! (0x85f3391e);
	} else {
		let char = try! (unicode_utf8_char_decode (bytes));
		succeed! ((char, char_width));
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn unicode_utf8_chars_clone (characters : &[char]) -> (StdString) {
	let mut value = StdString::with_capacity (characters.len ());
	for character in characters {
		value.push (*character);
	}
	StdString::from (value)
}

