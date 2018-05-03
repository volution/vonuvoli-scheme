

use super::conversions::exports::*;
use super::errors::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;

#[ allow (unused_imports) ]
use super::values::exports::*;

use super::prelude::*;




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
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesImmutable ( StdRc<StdBox<[u8]>> ),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesMutable ( StdRc<StdRefCell<BytesMutableInternals>> ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringImmutable ( StdRc<StdBox<str>> ),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringMutable ( StdRc<StdRefCell<StringMutableInternals>> ),
	None,
}


impl PortBackendReader for PortBackendBytesReader {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_ready (&mut self) -> (Outcome<bool>) {
		succeed! (true);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			succeed! (Some (buffer[0]));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		let (byte, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			(Some (buffer[0]), 1)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (byte);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_slice (&mut self, target : &mut [u8], _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let limit = usize::min (buffer.len (), target.len ());
			<[u8]>::copy_from_slice (&mut target[..limit], &buffer[..limit]);
			(Some (limit), limit)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend (&mut self, target : &mut StdVec<u8>, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let limit = usize::min (buffer.len (), count.unwrap_or (usize::max_value ()));
			target.extend (&buffer[..limit]);
			(Some (limit), limit)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string (&mut self, target : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let limit = usize::min (buffer.len (), count.unwrap_or (usize::max_value ()));
			let (_, offset, _) = try! (unicode_utf8_char_decode_slice_extend_string (&buffer[..limit], None, None, target));
			if full && (offset != limit) {
				fail! (0xf7c810bb);
			}
			(Some (offset), offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend_until (&mut self, target : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, _full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let limit = match libc_memchr (delimiter, buffer) {
				Some (offset) =>
					usize::min (offset + 1, count.unwrap_or (usize::max_value ())),
				None =>
					usize::min (buffer.len (), count.unwrap_or (usize::max_value ())),
			};
			target.extend (&buffer[..limit]);
			(Some (limit), limit)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string_until (&mut self, target : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let limit = match libc_memchr (delimiter, buffer) {
				Some (offset) =>
					usize::min (offset + 1, count.unwrap_or (usize::max_value ())),
				None =>
					usize::min (buffer.len (), count.unwrap_or (usize::max_value ())),
			};
			let (_, offset, _) = try! (unicode_utf8_char_decode_slice_extend_string (&buffer[..limit], None, None, target));
			if full && (offset != limit) {
				fail! (0x8fce22d7);
			}
			(Some (offset), offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_consume <Consumer> (&mut self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			try! (consumer (buffer));
			let limit = buffer.len ();
			(Some (limit), limit)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		if let Some (count) = count {
			succeed! (count);
		} else {
			succeed! (0);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_ready (&mut self) -> (Outcome<bool>) {
		succeed! (true);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (char, _) = try! (unicode_utf8_char_decode_and_width (buffer));
			succeed! (Some (char));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		let (char, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (char, char_width) = try! (unicode_utf8_char_decode_and_width (buffer));
			(Some (char), char_width)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (char);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_slice (&mut self, target : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (offset, buffer_offset, _) = try! (unicode_utf8_char_decode_slice_copy_slice (buffer, None, None, target));
			if full && (offset != target.len ()) && (buffer_offset != buffer.len ()) {
				fail! (0xeffc98f7);
			}
			(Some (offset), buffer_offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend (&mut self, target : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (offset, buffer_offset, _) = try! (unicode_utf8_char_decode_slice_extend_vector (buffer, count, None, target));
			if full && (offset != count.unwrap_or (offset)) && (buffer_offset != buffer.len ()) {
				fail! (0x1f8749f1);
			}
			(Some (offset), buffer_offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string (&mut self, target : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (offset, buffer_offset, _) = try! (unicode_utf8_char_decode_slice_extend_string (buffer, count, None, target));
			if full && (offset != count.unwrap_or (offset)) && (buffer_offset != buffer.len ()) {
				fail! (0x603a7b23);
			}
			(Some (offset), buffer_offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend_until (&mut self, target : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (offset, buffer_offset, matched) = try! (unicode_utf8_char_decode_slice_extend_vector (buffer, count, Some (delimiter), target));
			if full && ! matched && (offset != count.unwrap_or (offset)) && (buffer_offset != buffer.len ()) {
				fail! (0xc4161f41);
			}
			(Some (offset), buffer_offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string_until (&mut self, target : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		let (count, offset_increment) = if let Some (buffer) = try! (self.buffer_ref_if_open ()) {
			let buffer = &buffer;
			let (offset, buffer_offset, matched) = try! (unicode_utf8_char_decode_slice_extend_string (buffer, count, Some (delimiter), target));
			if full && ! matched && (offset != count.unwrap_or (offset)) && (buffer_offset != buffer.len ()) {
				fail! (0xa38d9f65);
			}
			(Some (offset), buffer_offset)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn input_close (&mut self) -> (Outcome<()>) {
		self.source = PortBackendBytesReaderSource::None;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_input_open (&mut self) -> (bool) {
		match self.source {
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			PortBackendBytesReaderSource::BytesImmutable (_) =>
				return true,
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PortBackendBytesReaderSource::BytesMutable (_) =>
				return true,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			PortBackendBytesReaderSource::StringImmutable (_) =>
				return true,
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PortBackendBytesReaderSource::StringMutable (_) =>
				return true,
			PortBackendBytesReaderSource::None =>
				return false,
		}
	}
}


impl PortBackendBytesReader {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_bytes_immutable (bytes : StdRc<StdBox<[u8]>>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::BytesImmutable (bytes), range_start, range_end);
	}
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_bytes_mutable (bytes : StdRc<StdRefCell<BytesMutableInternals>>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::BytesMutable (bytes), range_start, range_end);
	}
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_string_immutable (string : StdRc<StdBox<str>>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::StringImmutable (string), range_start, range_end);
	}
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_string_mutable (string : StdRc<StdRefCell<StringMutableInternals>>, range_start : usize, range_end : Option<usize>) -> (Outcome<PortBackendBytesReader>) {
		return PortBackendBytesReader::new_from_source (PortBackendBytesReaderSource::StringMutable (string), range_start, range_end);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ allow (unreachable_code) ]
	fn buffer_ref_if_open (&mut self) -> (Outcome<Option<BytesSliceRef>>) {
		
		#[ allow (unused_variables) ]
		let buffer : BytesSliceRef = match self.source {
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			PortBackendBytesReaderSource::BytesImmutable (ref source) =>
				source.as_ref () .into (),
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PortBackendBytesReaderSource::BytesMutable (ref source) =>
				try_or_fail! (source.as_ref () .try_borrow (), 0xcc774fa3) .into (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			PortBackendBytesReaderSource::StringImmutable (ref source) =>
				source.as_ref () .into (),
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			PortBackendBytesReaderSource::StringMutable (ref source) =>
				try_or_fail! (source.as_ref () .try_borrow (), 0x37bca183) .into (),
			PortBackendBytesReaderSource::None =>
				succeed! (None),
		};
		
		let buffer = buffer.range (self.range_start + self.offset, self.range_end);
		
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.push (byte);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_slice (&mut self, bytes : &[u8], _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.extend_from_slice (bytes);
		succeed! (bytes.len ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_string (&mut self, string : &str, _full : bool) -> (Outcome<usize>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		buffer.extend_from_slice (string.as_bytes ());
		succeed! (string.len ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		let buffer = try! (self.buffer_ref_mut_check_open ());
		let mut bytes = [0; 4];
		let string = char.encode_utf8 (&mut bytes);
		buffer.extend_from_slice (string.as_bytes ());
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_flush (&mut self) -> (Outcome<()>) {
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_close (&mut self) -> (Outcome<()>) {
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_output_open (&mut self) -> (bool) {
		return self.buffer.is_some ();
	}
}


impl PortBackendBytesWriter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (buffer : Option<usize>) -> (Outcome<PortBackendBytesWriter>) {
		let buffer = buffer.unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
		let buffer = StdVec::with_capacity (buffer);
		let backend = PortBackendBytesWriter {
				buffer : Some (buffer),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn finalize (&mut self) -> (Outcome<StdVec<u8>>) {
		if let Some (buffer) = self.buffer.take () {
			succeed! (buffer);
		} else {
			fail! (0x461ed3a2);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_ref_mut_check_open (&mut self) -> (Outcome<&mut StdVec<u8>>) {
		if let Some (ref mut buffer) = self.buffer {
			succeed! (buffer);
		} else {
			fail! (0xd507a546);
		}
	}
}

