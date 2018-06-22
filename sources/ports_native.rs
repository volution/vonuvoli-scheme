

use super::errors::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{PortBackendNativeReader, PortBackendNativeReaderTarget, PortBackendNativeReaderTargetRef};
	pub use super::{PortBackendNativeWriter, PortBackendNativeWriterTarget, PortBackendNativeWriterTargetRef};
	
}




#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct PortBackendNativeReader {
	reader : PortBackendNativeReaderTarget,
	descriptor : Option<PortDescriptor>,
}

pub enum PortBackendNativeReaderTarget {
	Buffered (io::BufReader<StdBox<dyn io::Read>>),
	Stdin,
	Closed,
}

pub enum PortBackendNativeReaderTargetRef<'a> {
	Buffered (&'a mut io::BufReader<StdBox<dyn io::Read>>),
	Stdin (io::Stdin, io::StdinLock<'a>),
}


impl PortBackendReader for PortBackendNativeReader {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_ready (&mut self) -> (Outcome<bool>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			if let Some (_buffer) = try! (reader.buffer_ref ()) {
				succeed! (true);
			} else {
				succeed! (true);
			}
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			if let Some (buffer) = try! (reader.buffer_ref ()) {
				succeed! (Some (buffer[0]));
			} else {
				succeed! (None);
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let (byte, buffer_increment) = if let Some (buffer) = try! (reader.buffer_ref ()) {
				(Some (buffer[0]), 1)
			} else {
				(None, 0)
			};
			if buffer_increment > 0 {
				try! (reader.buffer_consume (buffer_increment));
			}
			succeed! (byte);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_slice (&mut self, target : &mut [u8], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = target.len ();
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let limit = usize::min (buffer.len (), count_remaining);
					<[u8]>::copy_from_slice (&mut target[count_accumulated..(count_accumulated + limit)], &buffer[..limit]);
					Some ((limit, limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend (&mut self, target : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let limit = usize::min (buffer.len (), count_remaining);
					target.extend (&buffer[..limit]);
					Some ((limit, limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string (&mut self, target : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let limit = usize::min (buffer.len (), count_remaining);
					let (_, limit, _) = try! (unicode_utf8_char_decode_slice_extend_string (&buffer[..limit], None, None, target));
					Some ((limit, limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend_until (&mut self, target : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let (matched, increments) = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (matched, limit) = match libc_memchr (delimiter, buffer) {
						Some (offset) =>
							(true, usize::min (offset + 1, count_remaining)),
						None =>
							(false, usize::min (buffer.len (), count_remaining)),
					};
					target.extend (&buffer[..limit]);
					(matched, Some ((limit, limit)))
				} else {
					(false, None)
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if matched || (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string_until (&mut self, target : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let (matched, increments) = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (matched, limit) = match libc_memchr (delimiter, buffer) {
						Some (offset) =>
							(true, usize::min (offset + 1, count_remaining)),
						None =>
							(false, usize::min (buffer.len (), count_remaining)),
					};
					let (_, limit, _) = try! (unicode_utf8_char_decode_slice_extend_string (&buffer[..limit], None, None, target));
					(matched, Some ((limit, limit)))
				} else {
					(false, None)
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if matched || (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_consume <Consumer> (&mut self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let limit = buffer.len ();
					TODO! ("if the `consumer` failed we should still consume the entire buffer");
					try! (consumer (buffer));
					Some ((limit, limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
				}
				if increments.is_none () {
					succeed! (count_accumulated);
				}
			}
		} else {
			succeed! (0);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_ready (&mut self) -> (Outcome<bool>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			if let Some (buffer) = try! (reader.buffer_ref ()) {
				let char_width = unicode_utf8_char_width (buffer[0]);
				if char_width <= buffer.len () {
					succeed! (true);
				} else {
					succeed! (false);
				}
			} else {
				succeed! (true);
			}
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			if let Some (buffer) = try! (reader.buffer_ref ()) {
				let (char, _) = try! (unicode_utf8_char_decode_and_width (buffer));
				succeed! (Some (char));
			} else {
				succeed! (None);
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let (char, buffer_increment) = if let Some (buffer) = try! (reader.buffer_ref ()) {
				let (char, char_width) = try! (unicode_utf8_char_decode_and_width (buffer));
				(Some (char), char_width)
			} else {
				(None, 0)
			};
			if buffer_increment > 0 {
				try! (reader.buffer_consume (buffer_increment));
			}
			succeed! (char);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_slice (&mut self, target : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = target.len ();
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (target_limit, buffer_limit, _) = try! (unicode_utf8_char_decode_slice_copy_slice (buffer, Some (count_remaining), None, &mut target[count_accumulated..]));
					Some ((target_limit, buffer_limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend (&mut self, target : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (target_limit, buffer_limit, _) = try! (unicode_utf8_char_decode_slice_extend_vector (buffer, Some (count_remaining), None, target));
					Some ((target_limit, buffer_limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string (&mut self, target : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let increments = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (target_limit, buffer_limit, _) = try! (unicode_utf8_char_decode_slice_extend_string (buffer, Some (count_remaining), None, target));
					Some ((target_limit, buffer_limit))
				} else {
					None
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend_until (&mut self, target : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let (matched, increments) = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (target_limit, buffer_limit, matched) = try! (unicode_utf8_char_decode_slice_extend_vector (buffer, Some (count_remaining), Some (delimiter), target));
					(matched, Some ((target_limit, buffer_limit)))
				} else {
					(false, None)
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if matched || (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string_until (&mut self, target : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut reader) = try! (self.reader_ref_mut_if_open ()) {
			let mut count_accumulated = 0;
			let mut count_remaining = count.unwrap_or (usize::max_value ());
			if count_remaining == 0 {
				succeed! (Some (0));
			}
			loop {
				let (matched, increments) = if let Some (buffer) = try! (reader.buffer_ref ()) {
					let (target_limit, buffer_limit, matched) = try! (unicode_utf8_char_decode_slice_extend_string (buffer, Some (count_remaining), Some (delimiter), target));
					(matched, Some ((target_limit, buffer_limit)))
				} else {
					(false, None)
				};
				if let Some ((count_increment, buffer_increment)) = increments {
					try! (reader.buffer_consume (buffer_increment));
					count_accumulated += count_increment;
					count_remaining -= count_increment;
				}
				if matched || (count_remaining == 0) || increments.is_none () || ! full {
					if count_accumulated > 0 {
						succeed! (Some (count_accumulated));
					} else {
						succeed! (None);
					}
				}
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn input_close (&mut self) -> (Outcome<()>) {
		self.reader = PortBackendNativeReaderTarget::Closed;
		self.descriptor = None;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_input_open (&mut self) -> (bool) {
		match self.reader {
			PortBackendNativeReaderTarget::Closed =>
				return false,
			_ =>
				return true,
		}
	}
}


impl PortBackendNativeReader {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_unbuffered (reader : StdBox<dyn io::Read>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<PortBackendNativeReader>) {
		let buffer = buffer.unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
		let reader = io::BufReader::with_capacity (buffer, reader);
		return PortBackendNativeReader::new_from_buffered (reader, descriptor);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_buffered (reader : io::BufReader<StdBox<dyn io::Read>>, descriptor : Option<PortDescriptor>) -> (Outcome<PortBackendNativeReader>) {
		let backend = PortBackendNativeReader {
				reader : PortBackendNativeReaderTarget::Buffered (reader),
				descriptor : descriptor,
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdin () -> (Outcome<PortBackendNativeReader>) {
		let backend = PortBackendNativeReader {
				reader : PortBackendNativeReaderTarget::Stdin,
				descriptor : Some (PortDescriptor::Stdin),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn descriptor (&self) -> (Outcome<Option<PortDescriptor>>) {
		succeed! (self.descriptor.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn reader_ref_mut_if_open (&mut self) -> (Outcome<Option<PortBackendNativeReaderTargetRef>>) {
		match self.reader {
			PortBackendNativeReaderTarget::Closed =>
				succeed! (None),
			_ =>
				succeed! (Some (try! (self.reader_ref_mut_check_open ()))),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn reader_ref_mut_check_open (&mut self) -> (Outcome<PortBackendNativeReaderTargetRef>) {
		match self.reader {
			PortBackendNativeReaderTarget::Buffered (ref mut reader) =>
				succeed! (PortBackendNativeReaderTargetRef::Buffered (reader)),
			PortBackendNativeReaderTarget::Stdin => {
				let stdin = io::stdin ();
				let stdin_locked = unsafe { mem::transmute (stdin.lock ()) };
				succeed! (PortBackendNativeReaderTargetRef::Stdin (stdin, stdin_locked));
			},
			PortBackendNativeReaderTarget::Closed =>
				fail! (0xe84fab94),
		}
	}
}


impl <'a> PortBackendNativeReaderTargetRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn as_ref (&self) -> (&dyn io::BufRead) {
		match *self {
			PortBackendNativeReaderTargetRef::Buffered (ref reader) =>
				*reader,
			PortBackendNativeReaderTargetRef::Stdin (_, ref reader) =>
				reader,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn as_ref_mut (&mut self) -> (&mut dyn io::BufRead) {
		match *self {
			PortBackendNativeReaderTargetRef::Buffered (ref mut reader) =>
				*reader,
			PortBackendNativeReaderTargetRef::Stdin (_, ref mut reader) =>
				reader,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn buffer_ref (&mut self) -> (Outcome<Option<&[u8]>>) {
		let reader = self.as_ref_mut ();
		if let Ok (buffer) = reader.fill_buf () {
			if ! buffer.is_empty () {
				succeed! (Some (buffer));
			} else {
				succeed! (None);
			}
		} else {
			fail! (0x5dcf2c12);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn buffer_consume (&mut self, size : usize) -> (Outcome<()>) {
		let reader = self.as_ref_mut ();
		reader.consume (size);
		succeed! (());
	}
}




#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct PortBackendNativeWriter {
	writer : PortBackendNativeWriterTarget,
	descriptor : Option<PortDescriptor>,
}

pub enum PortBackendNativeWriterTarget {
	Buffered (io::BufWriter<StdBox<dyn io::Write>>),
	Stdout,
	Stderr,
	Closed,
}

pub enum PortBackendNativeWriterTargetRef<'a> {
	Buffered (&'a mut io::BufWriter<StdBox<dyn io::Write>>),
	Stdout (io::Stdout, io::StdoutLock<'a>),
	Stderr (io::Stderr, io::StderrLock<'a>),
}


impl PortBackendWriter for PortBackendNativeWriter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		let bytes = [byte];
		succeed_or_fail! (writer.write_all (&bytes), 0x1ebd7525);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_slice (&mut self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		if full {
			try_or_fail! (writer.write_all (bytes), 0x30691aa9);
			succeed! (bytes.len ());
		} else {
			succeed_or_fail! (writer.write (bytes), 0x4a7ae9ae);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		return self.byte_write_slice (string.as_bytes (), full);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		let mut bytes = [0; 4];
		let string = char.encode_utf8 (&mut bytes);
		succeed_or_fail! (writer.write_all (string.as_bytes ()), 0xaca4d20e);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_slice (&mut self, chars : &[char], full : bool) -> (Outcome<usize>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in chars {
			let string = char.encode_utf8 (&mut bytes);
			let should_continue = try! (Self::char_write_perhaps_full (writer, string, full));
			if ! should_continue {
				count += 1;
				break;
			} else {
				count += 1;
			}
		}
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		let mut bytes = [0; 4];
		let mut count = 0;
		for char in string.chars () {
			let string = char.encode_utf8 (&mut bytes);
			let should_continue = try! (Self::char_write_perhaps_full (writer, string, full));
			if ! should_continue {
				count += 1;
				break;
			} else {
				count += 1;
			}
		}
		succeed! (count);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_flush (&mut self) -> (Outcome<()>) {
		let mut writer = try! (self.writer_ref_mut_check_open ());
		let writer = writer.as_ref_mut ();
		succeed_or_fail! (writer.flush (), 0xf10df25a);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_close (&mut self) -> (Outcome<()>) {
		self.writer = PortBackendNativeWriterTarget::Closed;
		self.descriptor = None;
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_output_open (&mut self) -> (bool) {
		match self.writer {
			PortBackendNativeWriterTarget::Closed =>
				return false,
			_ =>
				return true,
		}
	}
}


impl PortBackendNativeWriter {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_unbuffered (writer : StdBox<dyn io::Write>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<PortBackendNativeWriter>) {
		let buffer = buffer.unwrap_or (DEFAULT_PORT_BUFFER_SIZE);
		let writer = io::BufWriter::with_capacity (buffer, writer);
		return PortBackendNativeWriter::new_from_buffered (writer, descriptor);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_buffered (writer : io::BufWriter<StdBox<dyn io::Write>>, descriptor : Option<PortDescriptor>) -> (Outcome<PortBackendNativeWriter>) {
		let backend = PortBackendNativeWriter {
				writer : PortBackendNativeWriterTarget::Buffered (writer),
				descriptor : descriptor,
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdout () -> (Outcome<PortBackendNativeWriter>) {
		let backend = PortBackendNativeWriter {
				writer : PortBackendNativeWriterTarget::Stdout,
				descriptor : Some (PortDescriptor::Stdout),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stderr () -> (Outcome<PortBackendNativeWriter>) {
		let backend = PortBackendNativeWriter {
				writer : PortBackendNativeWriterTarget::Stderr,
				descriptor : Some (PortDescriptor::Stderr),
			};
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn descriptor (&self) -> (Outcome<Option<PortDescriptor>>) {
		succeed! (self.descriptor.clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn writer_ref_mut_if_open (&mut self) -> (Outcome<Option<PortBackendNativeWriterTargetRef>>) {
		match self.writer {
			PortBackendNativeWriterTarget::Closed =>
				succeed! (None),
			_ =>
				succeed! (Some (try! (self.writer_ref_mut_check_open ()))),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn writer_ref_mut_check_open (&mut self) -> (Outcome<PortBackendNativeWriterTargetRef>) {
		match self.writer {
			PortBackendNativeWriterTarget::Buffered (ref mut writer) =>
				succeed! (PortBackendNativeWriterTargetRef::Buffered (writer)),
			PortBackendNativeWriterTarget::Stdout => {
				let stdout = io::stdout ();
				let stdout_locked = unsafe { mem::transmute (stdout.lock ()) };
				succeed! (PortBackendNativeWriterTargetRef::Stdout (stdout, stdout_locked));
			},
			PortBackendNativeWriterTarget::Stderr => {
				let stderr = io::stderr ();
				let stderr_locked = unsafe { mem::transmute (stderr.lock ()) };
				succeed! (PortBackendNativeWriterTargetRef::Stderr (stderr, stderr_locked));
			},
			PortBackendNativeWriterTarget::Closed =>
				fail! (0x6f55fd9c),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn char_write_perhaps_full (writer : &mut dyn io::Write, string : &str, full : bool) -> (Outcome<bool>) {
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


impl <'a> PortBackendNativeWriterTargetRef<'a> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn as_ref (&self) -> (&dyn io::Write) {
		match *self {
			PortBackendNativeWriterTargetRef::Buffered (ref writer) =>
				*writer,
			PortBackendNativeWriterTargetRef::Stdout (_, ref writer) =>
				writer,
			PortBackendNativeWriterTargetRef::Stderr (_, ref writer) =>
				writer,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn as_ref_mut (&mut self) -> (&mut dyn io::Write) {
		match *self {
			PortBackendNativeWriterTargetRef::Buffered (ref mut writer) =>
				*writer,
			PortBackendNativeWriterTargetRef::Stdout (_, ref mut writer) =>
				writer,
			PortBackendNativeWriterTargetRef::Stderr (_, ref mut writer) =>
				writer,
		}
	}
}




#[ cfg ( feature = "vonuvoli_fmt_debug" ) ] // OK ~~
impl fmt::Debug for PortBackendNativeReaderTarget {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		TODO! ("imlement this");
		formatter.debug_tuple ("PortBackendNativeReaderTarget") .finish ()
	}
}


#[ cfg ( feature = "vonuvoli_fmt_debug" ) ] // OK ~~
impl fmt::Debug for PortBackendNativeWriterTarget {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		TODO! ("imlement this");
		formatter.debug_tuple ("PortBackendNativeWriterTarget") .finish ()
	}
}

