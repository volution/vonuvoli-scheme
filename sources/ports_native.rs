

use super::errors::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::PortBackendNativeReader;
	pub use super::PortBackendNativeWriter;
	
}




pub struct PortBackendNativeReader {
	reader : Option<io::BufReader<StdBox<io::Read>>>,
}


impl PortBackendReader for PortBackendNativeReader {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_ready (&mut self) -> (Outcome<bool>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			if ! buffer.is_empty () {
				succeed! (true);
			} else {
				succeed! (false);
			}
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			succeed! (Some (buffer[0]));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		let (byte, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			(Some (buffer[0]), 1)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (byte);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_read_slice (&mut self, target : &mut [u8], _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), target.len ());
			<[u8]>::copy_from_slice (&mut target[..count], &buffer[..count]);
			(Some (count), count)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_read_extend (&mut self, target : &mut StdVec<u8>, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), count.unwrap_or (0));
			target.extend (&buffer[..count]);
			(Some (count), count)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_read_string (&mut self, target : &mut StdString, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), count.unwrap_or (0));
			if let Ok (buffer) = str::from_utf8 (&buffer[..count]) {
				target.push_str (buffer);
				(Some (count), count)
			} else {
				fail! (0x4c431111);
			}
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_ready (&mut self) -> (Outcome<bool>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			if ! buffer.is_empty () {
				let char_width = unicode_utf8_char_width (buffer[0]);
				if char_width <= buffer.len () {
					succeed! (true);
				} else {
					succeed! (false);
				}
			} else {
				succeed! (false);
			}
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let (char, _) = try! (unicode_utf8_char_decode_and_width (buffer));
			succeed! (Some (char));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		let (char, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let (char, char_width) = try! (unicode_utf8_char_decode_and_width (buffer));
			(Some (char), char_width)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (char);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_read_slice (&mut self, target : &mut [char], _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let mut buffer_offset = 0;
			let buffer_size = buffer.len ();
			let target_size = target.len ();
			let mut count = 0;
			while (buffer_offset < buffer_size) && (count < target_size) {
				let (char, char_width) = try! (unicode_utf8_char_decode_and_width (&buffer[buffer_offset..]));
				target[count] = char;
				buffer_offset += char_width;
				count += 1;
			};
			(Some (count), buffer_offset)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_read_extend (&mut self, target : &mut StdVec<char>, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let mut buffer_offset = 0;
			let buffer_size = buffer.len ();
			let target_size = count.unwrap_or (usize::max_value ());
			let mut count = 0;
			while (buffer_offset < buffer_size) && (count < target_size) {
				let (char, char_width) = try! (unicode_utf8_char_decode_and_width (&buffer[buffer_offset..]));
				target.push (char);
				buffer_offset += char_width;
				count += 1;
			};
			(Some (count), buffer_offset)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_read_string (&mut self, target : &mut StdString, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let mut buffer_offset = 0;
			let buffer_size = buffer.len ();
			let target_size = count.unwrap_or (usize::max_value ());
			let mut count = 0;
			while (buffer_offset < buffer_size) && (count < target_size) {
				let (char, char_width) = try! (unicode_utf8_char_decode_and_width (&buffer[buffer_offset..]));
				target.push (char);
				buffer_offset += char_width;
				count += 1;
			};
			(Some (count), buffer_offset)
		} else {
			(None, 0)
		};
		self.buffer_consume (offset_increment);
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn input_close (&mut self) -> (Outcome<()>) {
		self.reader = None;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn is_input_open (&mut self) -> (bool) {
		return self.reader.is_some ();
	}
}


impl PortBackendNativeReader {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new_from_unbuffered (reader : StdBox<io::Read>) -> (Outcome<PortBackendNativeReader>) {
		let reader = io::BufReader::new (reader);
		return PortBackendNativeReader::new_from_buffered (reader);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new_from_buffered (reader : io::BufReader<StdBox<io::Read>>) -> (Outcome<PortBackendNativeReader>) {
		let backend = PortBackendNativeReader {
				reader : Some (reader),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn buffer_ref_if_open (&mut self) -> (Outcome<Option<&[u8]>>) {
		if let Some (ref mut reader) = self.reader {
			if let Ok (buffer) = reader.fill_buf () {
				if ! buffer.is_empty () {
					succeed! (Some (buffer));
				} else {
					succeed! (None);
				}
			} else {
				fail! (0x5dcf2c12);
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn buffer_consume (&mut self, size : usize) -> () {
		if let Some (ref mut reader) = self.reader {
			reader.consume (size);
		}
	}
}




pub struct PortBackendNativeWriter {
	writer : Option<io::BufWriter<StdBox<io::Write>>>,
}


impl PortBackendWriter for PortBackendNativeWriter {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		let bytes = [byte];
		succeed_or_fail! (writer.write_all (&bytes), 0x1ebd7525);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_write_slice (&mut self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		if full {
			try_or_fail! (writer.write_all (bytes), 0x30691aa9);
			succeed! (bytes.len ());
		} else {
			succeed_or_fail! (writer.write (bytes), 0x4a7ae9ae);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn byte_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		return self.byte_write_slice (string.as_bytes (), full);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let string = char.encode_utf8 (&mut bytes);
		succeed_or_fail! (writer.write_all (string.as_bytes ()), 0xaca4d20e);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_write_slice (&mut self, chars : &[char], full : bool) -> (Outcome<usize>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in chars {
			let string = char.encode_utf8 (&mut bytes);
			let perhaps_stop = try! (Self::char_write_perhaps_full (writer, string, full));
			if perhaps_stop {
				count += 1;
				break;
			} else {
				count += 1;
			}
		}
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in string.chars () {
			let string = char.encode_utf8 (&mut bytes);
			let perhaps_stop = try! (Self::char_write_perhaps_full (writer, string, full));
			if perhaps_stop {
				count += 1;
				break;
			} else {
				count += 1;
			}
		}
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn output_flush (&mut self) -> (Outcome<()>) {
		let writer = try! (self.writer_ref_mut_check_open ());
		succeed_or_fail! (writer.flush (), 0xf10df25a);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn output_close (&mut self) -> (Outcome<()>) {
		self.writer = None;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn is_output_open (&mut self) -> (bool) {
		return self.writer.is_some ();
	}
}


impl PortBackendNativeWriter {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new_from_unbuffered (writer : StdBox<io::Write>) -> (Outcome<PortBackendNativeWriter>) {
		let writer = io::BufWriter::new (writer);
		return PortBackendNativeWriter::new_from_buffered (writer);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn new_from_buffered (writer : io::BufWriter<StdBox<io::Write>>) -> (Outcome<PortBackendNativeWriter>) {
		let backend = PortBackendNativeWriter {
				writer : Some (writer),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn writer_ref_mut_check_open (&mut self) -> (Outcome<&mut io::Write>) {
		if let Some (ref mut writer) = self.writer {
			succeed! (writer);
		} else {
			fail! (0x6f55fd9c);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn char_write_perhaps_full (writer : &mut io::Write, string : &str, full : bool) -> (Outcome<bool>) {
		let mut bytes = string.as_bytes ();
		if full {
			try_or_fail! (writer.write_all (bytes), 0xab43d083);
			succeed! (true);
		} else {
			let mut wrote = try_or_fail! (writer.write (bytes), 0xe616ccd6);
			if wrote < bytes.len () {
				while wrote < bytes.len () {
					bytes = &bytes[wrote..];
					wrote = try_or_fail! (writer.write (bytes), 0x33d2d5dd);
				}
				succeed! (false);
			} else {
				succeed! (true);
			}
		}
	}
}

