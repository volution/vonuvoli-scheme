

use super::errors::exports::*;
use super::globals::exports::*;
use super::ports_memory::exports::*;
use super::runtime::exports::*;

use std::fmt;
use std::hash;
use std::ptr;




pub mod exports {
	
	pub use super::{
		Port, PortInternals,
		PortState,
		PortBackend
	};
	
	pub use super::{
		PortQueries,
		PortReader, PortBackendReader,
		PortWriter, PortBackendWriter,
	};
	
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
	BytesWriter ( PortBackendBytesWriter ),
}




pub trait PortQueries {
	
	fn is_read_implemented (&self) -> (bool);
	fn is_write_implemented (&self) -> (bool);
	
	fn is_byte_implemented (&self) -> (bool);
	fn is_char_implemented (&self) -> (bool);
	fn is_value_implemented (&self) -> (bool);
	
}




pub trait PortReader {
	
	fn byte_ready (&self) -> (Outcome<bool>);
	fn byte_peek (&self) -> (Outcome<Option<u8>>);
	fn byte_read (&self) -> (Outcome<Option<u8>>);
	fn byte_read_slice (&self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_extend (&self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn char_ready (&self) -> (Outcome<bool>);
	fn char_peek (&self) -> (Outcome<Option<char>>);
	fn char_read (&self) -> (Outcome<Option<char>>);
	fn char_read_slice (&self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend (&self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn input_close (&self) -> (Outcome<()>);
	
	fn is_input_open (&self) -> (bool);
	
}


pub trait PortBackendReader {
	
	fn byte_ready (&self) -> (Outcome<bool>);
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>);
	fn byte_read (&mut self) -> (Outcome<Option<u8>>);
	fn byte_read_slice (&mut self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_extend (&mut self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn char_ready (&self) -> (Outcome<bool>);
	fn char_peek (&mut self) -> (Outcome<Option<char>>);
	fn char_read (&mut self) -> (Outcome<Option<char>>);
	fn char_read_slice (&mut self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend (&mut self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn input_close (&mut self) -> (Outcome<()>);
	
	fn is_input_open (&self) -> (bool);
	
}




pub trait PortWriter {
	
	fn byte_write (&self, byte : u8) -> (Outcome<()>);
	fn byte_write_slice (&self, bytes : &[u8], full : bool) -> (Outcome<usize>);
	fn byte_write_string (&self, string : &str, full : bool) -> (Outcome<usize>);
	
	fn char_write (&self, char : char) -> (Outcome<()>);
	fn char_write_slice (&self, chars : &[char], full : bool) -> (Outcome<usize>);
	fn char_write_string (&self, string : &str, full : bool) -> (Outcome<usize>);
	
	fn output_flush (&self) -> (Outcome<()>);
	fn output_close (&self) -> (Outcome<()>);
	
	fn is_output_open (&self) -> (bool);
	
}


pub trait PortBackendWriter {
	
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>);
	fn byte_write_slice (&mut self, bytes : &[u8], full : bool) -> (Outcome<usize>);
	fn byte_write_string (&mut self, bytes : &str, full : bool) -> (Outcome<usize>);
	
	fn char_write (&mut self, char : char) -> (Outcome<()>);
	fn char_write_slice (&mut self, chars : &[char], full : bool) -> (Outcome<usize>);
	fn char_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>);
	
	fn output_flush (&mut self) -> (Outcome<()>);
	fn output_close (&mut self) -> (Outcome<()>);
	
	fn is_output_open (&self) -> (bool);
	
}




impl Port {
	
	
	pub fn new_bytes_reader_from_bytes (buffer : StdRc<StdVec<u8>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_bytes (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	pub fn new_bytes_reader_from_string (buffer : StdRc<StdString>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_string (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	pub fn new_bytes_writer () -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesWriter::new ());
		let backend = PortBackend::BytesWriter (backend);
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
	
	
	pub fn internals_ref (&self) -> (StdRef<PortInternals>) {
		return StdRefCell::borrow (StdRc::as_ref (&self.0));
	}
	
	pub fn internals_ref_mut (&self) -> (StdRefMut<PortInternals>) {
		return StdRefCell::borrow_mut (StdRc::as_ref (&self.0));
	}
	
	
	pub fn internals_ref_if_open (&self) -> (Outcome<Option<StdRef<PortInternals>>>) {
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
	
	pub fn internals_ref_mut_if_open (&self) -> (Outcome<Option<StdRefMut<PortInternals>>>) {
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
	
	
	pub fn internals_ref_check_open (&self) -> (Outcome<StdRef<PortInternals>>) {
		let self_0 = self.internals_ref ();
		match self_0.state {
			PortState::Open =>
				(),
			PortState::Closed =>
				fail! (0x1a089b59),
			PortState::Failed (ref error) =>
				return Err (error.clone ()),
		}
		succeed! (self_0);
	}
	
	pub fn internals_ref_mut_check_open (&self) -> (Outcome<StdRefMut<PortInternals>>) {
		let self_0 = self.internals_ref_mut ();
		match self_0.state {
			PortState::Open =>
				(),
			PortState::Closed =>
				fail! (0x42e63ca7),
			PortState::Failed (ref error) =>
				return Err (error.clone ()),
		}
		succeed! (self_0);
	}
	
	
	pub fn is_self (&self, other : &Port) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl PortInternals {
	
	pub fn backend_ref (&self) -> (&PortBackend) {
		return &self.backend;
	}
	
	pub fn backend_ref_mut (&mut self) -> (&mut PortBackend) {
		return &mut self.backend;
	}
	
	fn process_outcome <T> (&mut self, outcome : Outcome<T>) -> (Outcome<T>) {
		match outcome {
			outcome @ Ok (_) =>
				return outcome,
			Err (error) => {
				self.state = PortState::Failed (error.clone ());
				return Err (error);
			},
		}
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
}




impl PortReader for Port {
	
	fn byte_ready (&self) -> (Outcome<bool>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_ready ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (true);
		}
	}
	
	fn byte_peek (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_peek ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn byte_read (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn byte_read_slice (&self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_slice (buffer, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn byte_read_extend (&self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_extend (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn byte_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_string (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn char_ready (&self) -> (Outcome<bool>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_ready ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (true);
		}
	}
	
	fn char_peek (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_peek ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn char_read (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn char_read_slice (&self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_slice (buffer, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn char_read_extend (&self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_extend (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn char_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_string (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	fn input_close (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.input_close ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	fn is_input_open (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_input_open ();
		} else {
			return false;
		}
	}
}




impl PortWriter for Port {
	
	fn byte_write (&self, byte : u8) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write (byte);
		return self_0.process_outcome (outcome);
	}
	
	fn byte_write_slice (&self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write_slice (bytes, full);
		return self_0.process_outcome (outcome);
	}
	
	fn byte_write_string (&self, string : &str, full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write_string (string, full);
		return self_0.process_outcome (outcome);
	}
	
	fn char_write (&self, char : char) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write (char);
		return self_0.process_outcome (outcome);
	}
	
	fn char_write_slice (&self, chars : &[char], full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write_slice (chars, full);
		return self_0.process_outcome (outcome);
	}
	
	fn char_write_string (&self, string : &str, full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write_string (string, full);
		return self_0.process_outcome (outcome);
	}
	
	fn output_flush (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.output_flush ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	fn output_close (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.output_close ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	fn is_output_open (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_output_open ();
		} else {
			return false;
		}
	}
}




impl PortBackend {
	
	pub fn close (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_close (),
		}
	}
}




impl PortQueries for PortBackend {
	
	fn is_read_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return false,
		}
	}
	
	fn is_write_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (_) =>
				return true,
		}
	}
	
	fn is_byte_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return true,
		}
	}
	
	fn is_char_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return true,
		}
	}
	
	fn is_value_implemented (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (_) =>
				return false,
		}
	}
}




impl PortBackendReader for PortBackend {
	
	fn byte_ready (&self) -> (Outcome<bool>) {
		match *self {
			PortBackend::BytesReader (ref backend) =>
				return backend.byte_ready (),
			PortBackend::BytesWriter (_) =>
				fail! (0xfeae1f6f),
		}
	}
	
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_peek (),
			PortBackend::BytesWriter (_) =>
				fail! (0xc27a4d90),
		}
	}
	
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read (),
			PortBackend::BytesWriter (_) =>
				fail! (0xf667bc10),
		}
	}
	
	fn byte_read_slice (&mut self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_slice (buffer, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x11f30803),
		}
	}
	
	fn byte_read_extend (&mut self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_extend (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xc9f43b63),
		}
	}
	
	fn byte_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_string (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xcf79a239),
		}
	}
	
	fn char_ready (&self) -> (Outcome<bool>) {
		match *self {
			PortBackend::BytesReader (ref backend) =>
				return backend.char_ready (),
			PortBackend::BytesWriter (_) =>
				fail! (0x53e51828),
		}
	}
	
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_peek (),
			PortBackend::BytesWriter (_) =>
				fail! (0x5b9b1a55),
		}
	}
	
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read (),
			PortBackend::BytesWriter (_) =>
				fail! (0x3af9daae),
		}
	}
	
	fn char_read_slice (&mut self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_slice (buffer, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xfc132fec),
		}
	}
	
	fn char_read_extend (&mut self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_extend (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x1647a5ea),
		}
	}
	
	fn char_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_string (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x1add1477),
		}
	}
	
	fn input_close (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::BytesWriter (_) =>
				fail! (0x3aef6f3e),
		}
	}
	
	fn is_input_open (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (ref backend) =>
				return backend.is_input_open (),
			PortBackend::BytesWriter (_) =>
				return false,
		}
	}
}




impl PortBackendWriter for PortBackend {
	
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0xf4d88c23),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write (byte),
		}
	}
	
	fn byte_write_slice (&mut self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0x6b40031c),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write_slice (bytes, full),
		}
	}
	
	fn byte_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0x69b8e5e7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write_string (string, full),
		}
	}
	
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0x6b9c35d1),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write (char),
		}
	}
	
	fn char_write_slice (&mut self, chars : &[char], full : bool) -> (Outcome<usize>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0xd8c4feb7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write_slice (chars, full),
		}
	}
	
	fn char_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0x773381e6),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write_string (string, full),
		}
	}
	
	fn output_flush (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0xe4db6cd7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_flush (),
		}
	}
	
	fn output_close (&mut self) -> (Outcome<()>) {
		match *self {
			PortBackend::BytesReader (_) =>
				fail! (0x61b53f17),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_close (),
		}
	}
	
	fn is_output_open (&self) -> (bool) {
		match *self {
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (ref backend) =>
				return backend.is_output_open (),
		}
	}
}

