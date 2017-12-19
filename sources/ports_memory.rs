

use super::errors::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;

use std::str;




pub mod exports {
	
	pub use super::PortBackendBytesReader;
	pub use super::PortBackendBytesWriter;
	
}




pub struct PortBackendBytesReader {
	source : PortBackendBytesReaderSource,
	range_start : usize,
	range_end : Option<usize>,
	offset : usize,
}

enum PortBackendBytesReaderSource {
	Bytes ( StdRc<StdVec<u8>> ),
	String ( StdRc<StdString> ),
	None,
}


impl PortBackendReader for PortBackendBytesReader {
	
	fn byte_ready (&mut self) -> (Outcome<bool>) {
		succeed! (true);
	}
	
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			succeed! (Some (buffer[0]));
		} else {
			succeed! (None);
		}
	}
	
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		let (byte, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			(Some (buffer[0]), 1)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (byte);
	}
	
	fn byte_read_slice (&mut self, target : &mut [u8], _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), target.len ());
			<[u8]>::copy_from_slice (&mut target[..count], &buffer[..count]);
			(Some (count), count)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	fn byte_read_extend (&mut self, target : &mut StdVec<u8>, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), count.unwrap_or (0));
			target.extend (&buffer[..count]);
			(Some (count), count)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	fn byte_read_string (&mut self, target : &mut StdString, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let count = usize::min (buffer.len (), count.unwrap_or (0));
			if let Ok (buffer) = str::from_utf8 (&buffer[..count]) {
				target.push_str (buffer);
				(Some (count), count)
			} else {
				fail! (0xe560db58);
			}
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	fn char_ready (&mut self) -> (Outcome<bool>) {
		succeed! (true);
	}
	
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let (char, _) = try! (unicode_utf8_char_decode_and_width (buffer));
			succeed! (Some (char));
		} else {
			succeed! (None);
		}
	}
	
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		let (char, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let (char, char_width) = try! (unicode_utf8_char_decode_and_width (buffer));
			(Some (char), char_width)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (char);
	}
	
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
		self.offset += offset_increment;
		succeed! (count);
	}
	
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
		self.offset += offset_increment;
		succeed! (count);
	}
	
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
		self.offset += offset_increment;
		succeed! (count);
	}
	
	fn input_close (&mut self) -> (Outcome<()>) {
		self.source = PortBackendBytesReaderSource::None;
		succeed! (());
	}
	
	fn is_input_open (&mut self) -> (bool) {
		match self.source {
			PortBackendBytesReaderSource::Bytes (_) =>
				return true,
			PortBackendBytesReaderSource::String (_) =>
				return true,
			PortBackendBytesReaderSource::None =>
				return false,
		}
	}
}


impl PortBackendBytesReader {
	
	pub fn new_from_bytes (bytes : StdRc<StdVec<u8>>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::Bytes (bytes), range_start, range_end);
	}
	
	pub fn new_from_string (string : StdRc<StdString>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::String (string), range_start, range_end);
	}
	
	fn new_from_source (source : PortBackendBytesReaderSource, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		if let Some (range_end) = range_end {
			if range_end < range_start {
				fail! (0xc8068e78);
			}
		}
		let backend = PortBackendBytesReader {
				source : source,
				range_start : range_start,
				range_end : range_end,
				offset : 0,
			};
		succeed! (backend);
	}
	
	fn buffer_ref_if_open (&mut self) -> (Outcome<Option<&[u8]>>) {
		
		let buffer = match self.source {
			PortBackendBytesReaderSource::Bytes (ref source) =>
				source.as_ref (),
			PortBackendBytesReaderSource::String (ref source) =>
				source.as_ref () .as_bytes (),
			PortBackendBytesReaderSource::None =>
				succeed! (None),
		};
		
		let range_start = self.range_start + self.offset;
		let buffer = if let Some (range_end) = self.range_end {
			buffer.get (range_start .. range_end)
		} else {
			buffer.get (range_start ..)
		};
		
		if let Some (buffer) = buffer {
			if ! buffer.is_empty () {
				succeed! (Some (buffer));
			} else {
				succeed! (None);
			}
		} else {
			succeed! (None);
		}
	}
}




pub struct PortBackendBytesWriter {
	buffer : Option<StdVec<u8>>,
}


impl PortBackendWriter for PortBackendBytesWriter {
	
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.push (byte);
		succeed! (());
	}
	
	fn byte_write_slice (&mut self, bytes : &[u8], _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.extend_from_slice (bytes);
		succeed! (bytes.len ());
	}
	
	fn byte_write_string (&mut self, string : &str, _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.extend_from_slice (string.as_bytes ());
		succeed! (string.len ());
	}
	
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let string = char.encode_utf8 (&mut bytes);
		buffer.extend_from_slice (string.as_bytes ());
		succeed! (());
	}
	
	fn char_write_slice (&mut self, chars : &[char], _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in chars {
			let string = char.encode_utf8 (&mut bytes);
			buffer.extend_from_slice (string.as_bytes ());
			count += 1;
		}
		succeed! (count);
	}
	
	fn char_write_string (&mut self, string : &str, _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in string.chars () {
			let string = char.encode_utf8 (&mut bytes);
			buffer.extend_from_slice (string.as_bytes ());
			count += 1;
		}
		succeed! (count);
	}
	
	fn output_flush (&mut self) -> (Outcome<()>) {
		succeed! (());
	}
	
	fn output_close (&mut self) -> (Outcome<()>) {
		succeed! (());
	}
	
	fn is_output_open (&mut self) -> (bool) {
		return self.buffer.is_some ();
	}
}


impl PortBackendBytesWriter {
	
	pub fn new () -> (Outcome<PortBackendBytesWriter>) {
		let buffer = StdVec::new ();
		let backend = PortBackendBytesWriter {
				buffer : Some (buffer),
			};
		succeed! (backend);
	}
	
	pub fn finalize (&mut self) -> (Outcome<StdVec<u8>>) {
		if let Some (buffer) = self.buffer.take () {
			succeed! (buffer);
		} else {
			fail! (0x461ed3a2);
		}
	}
	
	fn buffer_ref_mut_check_open (&mut self) -> (Outcome<&mut StdVec<u8>>) {
		if let Some (ref mut buffer) = self.buffer {
			succeed! (buffer);
		} else {
			fail! (0xd507a546);
		}
	}
}

