

use super::errors::exports::*;
use super::globals::exports::*;
use super::ports_memory::exports::*;
use super::ports_native::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		Port, PortInternals,
		PortState,
		PortBackend,
		PortDescriptor,
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
	pub state : PortState,
	pub backend : PortBackend,
	pub handle : Handle,
}


pub enum PortState {
	Open,
	Closed,
	Failed (Error),
}


pub enum PortBackend {
	
	BytesReader ( PortBackendBytesReader ),
	BytesWriter ( PortBackendBytesWriter ),
	
	NativeReader ( PortBackendNativeReader ),
	NativeWriter ( PortBackendNativeWriter ),
	
	Descriptor ( PortDescriptor ),
	
}


#[ derive (Clone, Debug) ]
pub enum PortDescriptor {
	RawFd (unix_io::RawFd),
	Stdin,
	Stdout,
	Stderr,
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
	fn byte_read_extend_until (&self, buffer : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_string_until (&self, buffer : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_consume <Consumer> (&self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>);
	
	fn char_ready (&self) -> (Outcome<bool>);
	fn char_peek (&self) -> (Outcome<Option<char>>);
	fn char_read (&self) -> (Outcome<Option<char>>);
	fn char_read_slice (&self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend (&self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend_until (&self, buffer : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string_until (&self, buffer : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn input_close (&self) -> (Outcome<()>);
	
	fn is_input_open (&self) -> (bool);
	
}


pub trait PortBackendReader {
	
	fn byte_ready (&mut self) -> (Outcome<bool>);
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>);
	fn byte_read (&mut self) -> (Outcome<Option<u8>>);
	fn byte_read_slice (&mut self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_extend (&mut self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_extend_until (&mut self, buffer : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_read_string_until (&mut self, buffer : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn byte_consume <Consumer> (&mut self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>);
	
	fn char_ready (&mut self) -> (Outcome<bool>);
	fn char_peek (&mut self) -> (Outcome<Option<char>>);
	fn char_read (&mut self) -> (Outcome<Option<char>>);
	fn char_read_slice (&mut self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend (&mut self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_extend_until (&mut self, buffer : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	fn char_read_string_until (&mut self, buffer : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>);
	
	fn input_close (&mut self) -> (Outcome<()>);
	
	fn is_input_open (&mut self) -> (bool);
	
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
	
	fn is_output_open (&mut self) -> (bool);
	
}




impl Port {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_bytes_reader_from_bytes_immutable (buffer : StdRc<StdBox<[u8]>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_bytes_immutable (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_bytes_reader_from_bytes_mutable (buffer : StdRc<StdRefCell<BytesMutableInternals>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_bytes_mutable (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_bytes_reader_from_string_immutable (buffer : StdRc<StdBox<str>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_string_immutable (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_bytes_reader_from_string_mutable (buffer : StdRc<StdRefCell<StringMutableInternals>>, range_start : usize, range_end : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesReader::new_from_string_mutable (buffer, range_start, range_end));
		let backend = PortBackend::BytesReader (backend);
		return Port::new_from_backend (backend);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_bytes_writer (buffer : Option<usize>) -> (Outcome<Port>) {
		let backend = try! (PortBackendBytesWriter::new (buffer));
		let backend = PortBackend::BytesWriter (backend);
		return Port::new_from_backend (backend);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_native_reader_from_unbuffered (reader : StdBox<io::Read>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Port>) {
		let backend = try! (PortBackendNativeReader::new_from_unbuffered (reader, buffer, descriptor));
		let backend = PortBackend::NativeReader (backend);
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_native_writer_from_unbuffered (writer : StdBox<io::Write>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Port>) {
		let backend = try! (PortBackendNativeWriter::new_from_unbuffered (writer, buffer, descriptor));
		let backend = PortBackend::NativeWriter (backend);
		return Port::new_from_backend (backend);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdin () -> (Outcome<Port>) {
		let backend = try! (PortBackend::new_stdin ());
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdout () -> (Outcome<Port>) {
		let backend = try! (PortBackend::new_stdout ());
		return Port::new_from_backend (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stderr () -> (Outcome<Port>) {
		let backend = try! (PortBackend::new_stderr ());
		return Port::new_from_backend (backend);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_descriptor (descriptor : PortDescriptor) -> (Outcome<Port>) {
		let backend = PortBackend::Descriptor (descriptor);
		return Port::new_from_backend (backend);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_from_backend (backend : PortBackend) -> (Outcome<Port>) {
		let internals = PortInternals {
				state : PortState::Open,
				backend : backend,
				handle : port_handles_next (),
			};
		let port = Port (StdRc::new (StdRefCell::new (internals)));
		succeed! (port);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (Outcome<StdRef<PortInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow (StdRc::as_ref (&self.0)), 0x853d2bfd));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<PortInternals>>) {
		succeed! (try_or_fail! (StdRefCell::try_borrow_mut (StdRc::as_ref (&self.0)), 0xe174d781));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Outcome<Handle>) {
		let self_0 = try! (self.internals_ref ());
		succeed! (self_0.handle);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_if_open (&self) -> (Outcome<Option<StdRef<PortInternals>>>) {
		let self_0 = try! (self.internals_ref ());
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut_if_open (&self) -> (Outcome<Option<StdRefMut<PortInternals>>>) {
		let self_0 = try! (self.internals_ref_mut ());
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_check_open (&self) -> (Outcome<StdRef<PortInternals>>) {
		let self_0 = try! (self.internals_ref ());
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut_check_open (&self) -> (Outcome<StdRefMut<PortInternals>>) {
		let self_0 = try! (self.internals_ref_mut ());
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
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref_if_open (&self) -> (Outcome<Option<StdRef<PortBackend>>>) {
		let internals = try! (self.internals_ref_if_open ());
		if let Some (internals) = internals {
			succeed! (Some (StdRef::map (internals, |internals| &internals.backend)));
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref_mut_if_open (&self) -> (Outcome<Option<StdRefMut<PortBackend>>>) {
		let internals = try! (self.internals_ref_mut_if_open ());
		if let Some (internals) = internals {
			succeed! (Some (StdRefMut::map (internals, |internals| &mut internals.backend)));
		} else {
			succeed! (None);
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref_check_open (&self) -> (Outcome<StdRef<PortBackend>>) {
		let internals = try! (self.internals_ref_check_open ());
		succeed! (StdRef::map (internals, |internals| &internals.backend));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref_mut_check_open (&self) -> (Outcome<StdRefMut<PortBackend>>) {
		let internals = try! (self.internals_ref_mut_check_open ());
		succeed! (StdRefMut::map (internals, |internals| &mut internals.backend));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Port) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}


impl PortInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref (&self) -> (&PortBackend) {
		return &self.backend;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn backend_ref_mut (&mut self) -> (&mut PortBackend) {
		return &mut self.backend;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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




impl Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn descriptor (&self) -> (Outcome<Option<PortDescriptor>>) {
		let self_0 = try! (self.internals_ref_check_open ());
		return self_0.backend.descriptor ();
	}
}




impl PortQueries for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_read_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_read_implemented ();
		} else {
			return false;
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_write_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_write_implemented ();
		} else {
			return false;
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_byte_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_byte_implemented ();
		} else {
			return false;
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_char_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_char_implemented ();
		} else {
			return false;
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_value_implemented (&self) -> (bool) {
		if let Ok (Some (self_0)) = self.internals_ref_if_open () {
			return self_0.backend.is_value_implemented ();
		} else {
			return false;
		}
	}
}




impl PortReader for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_ready (&self) -> (Outcome<bool>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_ready ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_peek (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_peek ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read (&self) -> (Outcome<Option<u8>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_slice (&self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_slice (buffer, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend (&self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_extend (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_string (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend_until (&self, buffer : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_extend_until (buffer, delimiter, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string_until (&self, buffer : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_read_string_until (buffer, delimiter, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_consume <Consumer> (&self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.byte_consume (consumer);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (0);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_ready (&self) -> (Outcome<bool>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_ready ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (true);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_peek (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_peek ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read (&self) -> (Outcome<Option<char>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_slice (&self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_slice (buffer, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend (&self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_extend (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string (&self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_string (buffer, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend_until (&self, buffer : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_extend_until (buffer, delimiter, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string_until (&self, buffer : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.char_read_string_until (buffer, delimiter, count, full);
			return self_0.process_outcome (outcome);
		} else {
			succeed! (None);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn input_close (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.input_close ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_input_open (&self) -> (bool) {
		if let Ok (Some (mut self_0)) = self.internals_ref_mut_if_open () {
			return self_0.backend.is_input_open ();
		} else {
			return false;
		}
	}
}




impl PortWriter for Port {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write (&self, byte : u8) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write (byte);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_slice (&self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write_slice (bytes, full);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_string (&self, string : &str, full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.byte_write_string (string, full);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write (&self, char : char) -> (Outcome<()>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write (char);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_slice (&self, chars : &[char], full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write_slice (chars, full);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_string (&self, string : &str, full : bool) -> (Outcome<usize>) {
		let mut self_0 = try! (self.internals_ref_mut_check_open ());
		let outcome = self_0.backend.char_write_string (string, full);
		return self_0.process_outcome (outcome);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_flush (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.output_flush ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_close (&self) -> (Outcome<()>) {
		if let Some (mut self_0) = try! (self.internals_ref_mut_if_open ()) {
			let outcome = self_0.backend.output_close ();
			return self_0.process_outcome (outcome);
		} else {
			succeed! (());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_output_open (&self) -> (bool) {
		if let Ok (Some (mut self_0)) = self.internals_ref_mut_if_open () {
			return self_0.backend.is_output_open ();
		} else {
			return false;
		}
	}
}




impl PortBackend {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdin () -> (Outcome<PortBackend>) {
		let backend = try! (PortBackendNativeReader::new_stdin ());
		let backend = PortBackend::NativeReader (backend);
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stdout () -> (Outcome<PortBackend>) {
		let backend = try! (PortBackendNativeWriter::new_stdout ());
		let backend = PortBackend::NativeWriter (backend);
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_stderr () -> (Outcome<PortBackend>) {
		let backend = try! (PortBackendNativeWriter::new_stderr ());
		let backend = PortBackend::NativeWriter (backend);
		succeed! (backend);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn close (&mut self) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_close (),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.output_close (),
			
			PortBackend::Descriptor (_) =>
				fail_unimplemented! (0xb7f61dce), // deferred
			
		}
	}
	
	pub fn descriptor (&self) -> (Outcome<Option<PortDescriptor>>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				succeed! (None),
			PortBackend::BytesWriter (_) =>
				succeed! (None),
			
			PortBackend::NativeReader (ref backend) =>
				return backend.descriptor (),
			PortBackend::NativeWriter (ref backend) =>
				return backend.descriptor (),
			
			PortBackend::Descriptor (ref descriptor) =>
				succeed! (Some (descriptor.clone ())),
			
		}
	}
}




impl PortQueries for PortBackend {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_read_implemented (&self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return false,
			
			PortBackend::NativeReader (_) =>
				return true,
			PortBackend::NativeWriter (_) =>
				return false,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_write_implemented (&self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (_) =>
				return true,
			
			PortBackend::NativeReader (_) =>
				return false,
			PortBackend::NativeWriter (_) =>
				return true,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_byte_implemented (&self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return true,
			
			PortBackend::NativeReader (_) =>
				return true,
			PortBackend::NativeWriter (_) =>
				return true,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_char_implemented (&self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return true,
			PortBackend::BytesWriter (_) =>
				return true,
			
			PortBackend::NativeReader (_) =>
				return true,
			PortBackend::NativeWriter (_) =>
				return true,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_value_implemented (&self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (_) =>
				return false,
			
			PortBackend::NativeReader (_) =>
				return false,
			PortBackend::NativeWriter (_) =>
				return false,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
}




impl PortBackendReader for PortBackend {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_ready (&mut self) -> (Outcome<bool>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_ready (),
			PortBackend::BytesWriter (_) =>
				fail! (0x5a27a6cf),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_ready (),
			PortBackend::NativeWriter (_) =>
				fail! (0xf6b9d777),
			
			PortBackend::Descriptor (_) =>
				fail! (0xdfdf7d3c),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_peek (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_peek (),
			PortBackend::BytesWriter (_) =>
				fail! (0xc27a4d90),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_peek (),
			PortBackend::NativeWriter (_) =>
				fail! (0x693a0eb6),
			
			PortBackend::Descriptor (_) =>
				fail! (0xd5309105),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read (&mut self) -> (Outcome<Option<u8>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read (),
			PortBackend::BytesWriter (_) =>
				fail! (0xf667bc10),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read (),
			PortBackend::NativeWriter (_) =>
				fail! (0x39e6cf6d),
			
			PortBackend::Descriptor (_) =>
				fail! (0x1321027b),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_slice (&mut self, buffer : &mut [u8], full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_slice (buffer, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xddc2d8a4),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read_slice (buffer, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x11f30803),
			
			PortBackend::Descriptor (_) =>
				fail! (0x53272b36),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend (&mut self, buffer : &mut StdVec<u8>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_extend (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xc9f43b63),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read_extend (buffer, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0xfadfcd4e),
			
			PortBackend::Descriptor (_) =>
				fail! (0x69dc6eeb),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_string (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xcf79a239),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read_string (buffer, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x00245653),
			
			PortBackend::Descriptor (_) =>
				fail! (0xffd71e4a),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_extend_until (&mut self, buffer : &mut StdVec<u8>, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_extend_until (buffer, delimiter, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xa13de3a3),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read_extend_until (buffer, delimiter, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x971d0c58),
			
			PortBackend::Descriptor (_) =>
				fail! (0x4cf09429),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_read_string_until (&mut self, buffer : &mut StdString, delimiter : u8, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_read_string_until (buffer, delimiter, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xc580867d),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_read_string_until (buffer, delimiter, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x09555e0a),
			
			PortBackend::Descriptor (_) =>
				fail! (0x4b3f6dea),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_consume <Consumer> (&mut self, consumer : &mut Consumer) -> (Outcome<usize>) where Consumer : FnMut (&[u8]) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.byte_consume (consumer),
			PortBackend::BytesWriter (_) =>
				fail! (0xfdb50452),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.byte_consume (consumer),
			PortBackend::NativeWriter (_) =>
				fail! (0x7f62b9fb),
			
			PortBackend::Descriptor (_) =>
				fail! (0x890b8324),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_ready (&mut self) -> (Outcome<bool>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_ready (),
			PortBackend::BytesWriter (_) =>
				fail! (0x53e51828),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_ready (),
			PortBackend::NativeWriter (_) =>
				fail! (0x3256e2ec),
			
			PortBackend::Descriptor (_) =>
				fail! (0xb243b3c7),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_peek (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_peek (),
			PortBackend::BytesWriter (_) =>
				fail! (0x5b9b1a55),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_peek (),
			PortBackend::NativeWriter (_) =>
				fail! (0x4c7f1120),
			
			PortBackend::Descriptor (_) =>
				fail! (0xa91061e5),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read (&mut self) -> (Outcome<Option<char>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read (),
			PortBackend::BytesWriter (_) =>
				fail! (0x4cb1bbf3),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read (),
			PortBackend::NativeWriter (_) =>
				fail! (0x3af9daae),
			
			PortBackend::Descriptor (_) =>
				fail! (0xf433bea0),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_slice (&mut self, buffer : &mut [char], full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_slice (buffer, full),
			PortBackend::BytesWriter (_) =>
				fail! (0xa6dd9cfc),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read_slice (buffer, full),
			PortBackend::NativeWriter (_) =>
				fail! (0xfc132fec),
			
			PortBackend::Descriptor (_) =>
				fail! (0x552b2d02),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend (&mut self, buffer : &mut StdVec<char>, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_extend (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x5d9b848d),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read_extend (buffer, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x1647a5ea),
			
			PortBackend::Descriptor (_) =>
				fail! (0x71753f9a),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string (&mut self, buffer : &mut StdString, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_string (buffer, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x63177075),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read_string (buffer, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x1add1477),
			
			PortBackend::Descriptor (_) =>
				fail! (0x1da6954c),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_extend_until (&mut self, buffer : &mut StdVec<char>, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_extend_until (buffer, delimiter, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x7cace7ff),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read_extend_until (buffer, delimiter, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x621f1695),
			
			PortBackend::Descriptor (_) =>
				fail! (0x5c8c0f0f),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_read_string_until (&mut self, buffer : &mut StdString, delimiter : char, count : Option<usize>, full : bool) -> (Outcome<Option<usize>>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.char_read_string_until (buffer, delimiter, count, full),
			PortBackend::BytesWriter (_) =>
				fail! (0x7a47bd7b),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.char_read_string_until (buffer, delimiter, count, full),
			PortBackend::NativeWriter (_) =>
				fail! (0x44f16931),
			
			PortBackend::Descriptor (_) =>
				fail! (0x530812a2),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn input_close (&mut self) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::BytesWriter (_) =>
				fail! (0x3aef6f3e),
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.input_close (),
			PortBackend::NativeWriter (_) =>
				fail! (0xee88f941),
			
			PortBackend::Descriptor (_) =>
				fail! (0x7e916568),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_input_open (&mut self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (ref mut backend) =>
				return backend.is_input_open (),
			PortBackend::BytesWriter (_) =>
				return false,
			
			PortBackend::NativeReader (ref mut backend) =>
				return backend.is_input_open (),
			PortBackend::NativeWriter (_) =>
				return false,
			
			PortBackend::Descriptor (_) =>
				return false,
			
		}
	}
}




impl PortBackendWriter for PortBackend {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write (&mut self, byte : u8) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0xf4d88c23),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write (byte),
			
			PortBackend::NativeReader (_) =>
				fail! (0xa4bc3df7),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.byte_write (byte),
			
			PortBackend::Descriptor (_) =>
				fail! (0xc2a502e6),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_slice (&mut self, bytes : &[u8], full : bool) -> (Outcome<usize>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0x6b40031c),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write_slice (bytes, full),
			
			PortBackend::NativeReader (_) =>
				fail! (0x11185a54),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.byte_write_slice (bytes, full),
			
			PortBackend::Descriptor (_) =>
				fail! (0x81ac2bba),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn byte_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0x69b8e5e7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.byte_write_string (string, full),
			
			PortBackend::NativeReader (_) =>
				fail! (0x8eb45415),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.byte_write_string (string, full),
			
			PortBackend::Descriptor (_) =>
				fail! (0xe73b58ad),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write (&mut self, char : char) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0x6b9c35d1),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write (char),
			
			PortBackend::NativeReader (_) =>
				fail! (0x1c1ce62f),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.char_write (char),
			
			PortBackend::Descriptor (_) =>
				fail! (0xfe210b9c),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_slice (&mut self, chars : &[char], full : bool) -> (Outcome<usize>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0xd8c4feb7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write_slice (chars, full),
			
			PortBackend::NativeReader (_) =>
				fail! (0x3e37a36e),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.char_write_slice (chars, full),
			
			PortBackend::Descriptor (_) =>
				fail! (0xcd048d90),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn char_write_string (&mut self, string : &str, full : bool) -> (Outcome<usize>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0x773381e6),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.char_write_string (string, full),
			
			PortBackend::NativeReader (_) =>
				fail! (0x91f63752),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.char_write_string (string, full),
			
			PortBackend::Descriptor (_) =>
				fail! (0x2e423ff7),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_flush (&mut self) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0xe4db6cd7),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_flush (),
			
			PortBackend::NativeReader (_) =>
				fail! (0x9440749b),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.output_flush (),
			
			PortBackend::Descriptor (_) =>
				fail! (0xaf2f83c2),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn output_close (&mut self) -> (Outcome<()>) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				fail! (0x61b53f17),
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.output_close (),
			
			PortBackend::NativeReader (_) =>
				fail! (0x992e7687),
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.output_close (),
			
			PortBackend::Descriptor (_) =>
				fail! (0x121e5864),
			
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_output_open (&mut self) -> (bool) {
		match *self {
			
			PortBackend::BytesReader (_) =>
				return false,
			PortBackend::BytesWriter (ref mut backend) =>
				return backend.is_output_open (),
			
			PortBackend::NativeReader (_) =>
				return false,
			PortBackend::NativeWriter (ref mut backend) =>
				return backend.is_output_open (),
			
			PortBackend::Descriptor (_) =>
				return false,
		}
	}
}




impl PortDescriptor {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_file (file : &fs::File) -> (Option<PortDescriptor>) {
		return Some (PortDescriptor::RawFd (unix_io::AsRawFd::as_raw_fd (file)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_child_stdin (stream : &process::ChildStdin) -> (Option<PortDescriptor>) {
		return Some (PortDescriptor::RawFd (unix_io::AsRawFd::as_raw_fd (stream)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_child_stdout (stream : &process::ChildStdout) -> (Option<PortDescriptor>) {
		return Some (PortDescriptor::RawFd (unix_io::AsRawFd::as_raw_fd (stream)));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn for_child_stderr (stream : &process::ChildStderr) -> (Option<PortDescriptor>) {
		return Some (PortDescriptor::RawFd (unix_io::AsRawFd::as_raw_fd (stream)));
	}
}

