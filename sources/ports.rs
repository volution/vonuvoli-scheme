

use super::errors::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
// use super::values::exports::*;

use std::fmt;
use std::hash;
// use std::io;
use std::ptr;




pub mod exports {
	
	pub use super::{ Port, PortInternals, PortState, PortBackend };
	pub use super::{ PortQueries, PortReader, PortBackendReader };
	
}




#[ derive (Clone) ]
pub struct Port ( StdRc<StdRefCell<PortInternals>> );


pub struct PortInternals {
	state : PortState,
	backend : PortBackend,
	handle : u32,
}


pub enum PortState {
	Open,
	Closed,
	Failed (Error),
}


pub enum PortBackend {
	BytesReader ( PortBackendBytesReader ),
}




pub trait PortQueries {
	
	fn is_read_implemented (&self) -> (bool);
	fn is_write_implemented (&self) -> (bool);
	
	fn is_byte_implemented (&self) -> (bool);
	fn is_char_implemented (&self) -> (bool);
	fn is_value_implemented (&self) -> (bool);
	
	fn is_read_open (&self) -> (bool);
	fn is_write_open (&self) -> (bool);
	
}




pub trait PortReader {
	
	fn peek_byte (&self) -> (Outcome<Option<u8>>);
	fn read_byte (&self) -> (Outcome<Option<u8>>);
	
	fn peek_char (&self) -> (Outcome<Option<char>>);
	fn read_char (&self) -> (Outcome<Option<char>>);
	
	fn close_input (&self) -> (Outcome<()>);
}


pub trait PortBackendReader {
	
	fn peek_byte (&mut self) -> (Outcome<Option<u8>>);
	fn read_byte (&mut self) -> (Outcome<Option<u8>>);
	
	fn peek_char (&mut self) -> (Outcome<Option<char>>);
	fn read_char (&mut self) -> (Outcome<Option<char>>);
	
	fn close_input (&mut self) -> (Outcome<()>);
}




impl Port {
	
	
	pub fn new_bytes_reader (buffer : StdRc<StdVec<u8>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		if let Some (range_end) = range_end {
			if range_end < range_start {
				fail! (0xc8068e78);
			}
		}
		let backend = PortBackendBytesReader {
				buffer : Some (buffer),
				range_start : range_start,
				range_end : range_end,
				offset : 0,
			};
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	
	pub fn new_from_backend (backend : PortBackend) -> (Outcome<Port>) {
		let internals = PortInternals {
				state : PortState::Open,
				backend : backend,
				handle : ports_handles_next (),
			};
		let port = Port (StdRc::new (StdRefCell::new (internals)));
		succeed! (port);
	}
	
	
	fn internals_ref (&self) -> (StdRef<PortInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	fn internals_ref_mut (&self) -> (StdRefMut<PortInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	
	fn internals_ref_if_open (&self) -> (Outcome<Option<StdRef<PortInternals>>>) {
		let self_0 = self.internals_ref ();
		match self_0.state {
			PortState::Open =>
				(),
			PortState::Closed =>
				succeed! (None),
			PortState::Failed (ref error) =>
				return Err (error.clone ()),
		}
		succeed! (Some (self_0));
	}
	
	fn internals_ref_mut_if_open (&self) -> (Outcome<Option<StdRefMut<PortInternals>>>) {
		let self_0 = self.internals_ref_mut ();
		match self_0.state {
			PortState::Open =>
				(),
			PortState::Closed =>
				succeed! (None),
			PortState::Failed (ref error) =>
				return Err (error.clone ()),
		}
		succeed! (Some (self_0));
	}
	
	
	pub fn is_self (&self, other : &Port) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl hash::Hash for Port {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		let self_0 = self.internals_ref ();
		hasher.write_u32 (self_0.handle);
	}
}


impl fmt::Display for Port {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<port>")
	}
}


impl fmt::Debug for Port {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<port>")
	}
}




impl PortQueries for Port {
	
	fn is_read_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_read_implemented ();
		} else {
			return false;
		}
	}
	
	fn is_write_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_write_implemented ();
		} else {
			return false;
		}
	}
	
	fn is_byte_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_byte_implemented ();
		} else {
			return false;
		}
	}
	
	fn is_char_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_char_implemented ();
		} else {
			return false;
		}
	}
	
	fn is_value_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_value_implemented ();
		} else {
			return false;
		}
	}
	
	fn is_read_open (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_read_open ();
		} else {
			return false;
		}
	}
	
	fn is_write_open (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_write_open ();
		} else {
			return false;
		}
	}
}




impl PortReader for Port {
	
	fn peek_byte (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.peek_byte () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (None);
		}
	}
	
	fn read_byte (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.read_byte () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (None);
		}
	}
	
	fn peek_char (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.peek_char () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (None);
		}
	}
	
	fn read_char (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.read_char () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (None);
		}
	}
	
	fn close_input (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.close_input () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (());
		}
	}
}




impl Port {
	
	pub fn close (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			match self_0.backend.close () {
				outcome @ Ok (_) =>
					return outcome,
				Err (error) => {
					self_0.state = PortState::Failed (error.clone ());
					return Err (error);
				},
			}
		} else {
			succeed! (());
		}
	}
}




impl PortQueries for PortBackend {
	
	fn is_read_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
		}
	}
	
	fn is_write_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
		}
	}
	
	fn is_byte_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
		}
	}
	
	fn is_char_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
		}
	}
	
	fn is_value_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
		}
	}
	
	fn is_read_open (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (ref backend) =>
				return backend.is_open (),
		}
	}
	
	fn is_write_open (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
		}
	}
	
}




impl PortBackendReader for PortBackend {
	
	fn peek_byte (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.peek_byte (),
		}
	}
	
	fn read_byte (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.read_byte (),
		}
	}
	
	fn peek_char (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.peek_char (),
		}
	}
	
	fn read_char (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.read_char (),
		}
	}
	
	fn close_input (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.close_input (),
		}
	}
}




impl PortBackend {
	
	pub fn close (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.close_input (),
		}
	}
}




pub struct PortBackendBytesReader {
	buffer : Option<StdRc<StdVec<u8>>>,
	range_start : usize,
	range_end : Option<usize>,
	offset : usize,
}


impl PortBackendReader for PortBackendBytesReader {
	
	fn peek_byte (&mut self) -> (Outcome<Option<u8>>) {
		if let Some (buffer) = try! (self.buffer_ref ()) {
			succeed! (Some (buffer[0]));
		} else {
			succeed! (None);
		}
	}
	
	fn read_byte (&mut self) -> (Outcome<Option<u8>>) {
		let (byte, offset_increment) = if let Some (buffer) = try! (self.buffer_ref ()) {
			(Some (buffer[0]), 1)
		} else {
			(None, 0)
		};
		self.offset += offset_increment;
		succeed! (byte);
	}
	
	fn peek_char (&mut self) -> (Outcome<Option<char>>) {
		if let Some ((char, _)) = try! (self.decode_char ()) {
			succeed! (Some (char));
		} else {
			succeed! (None);
		}
	}
	
	fn read_char (&mut self) -> (Outcome<Option<char>>) {
		if let Some ((char, offset_increment)) = try! (self.decode_char ()) {
			self.offset += offset_increment;
			succeed! (Some (char));
		} else {
			succeed! (None);
		}
	}
	
	fn close_input (&mut self) -> (Outcome<()>) {
		self.buffer = None;
		succeed! (());
	}
}


impl PortBackendBytesReader {
	
	fn is_open (&self) -> (bool) {
		return self.buffer.is_some ();
	}
	
	fn buffer_ref (&self) -> (Outcome<Option<&[u8]>>) {
		if let Some (ref buffer) = self.buffer {
			let buffer = buffer.as_ref ();
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
		} else {
			succeed! (None);
		}
	}
	
	fn decode_char (&self) -> (Outcome<Option<(char, usize)>>) {
		unimplemented! ();
	}
}

