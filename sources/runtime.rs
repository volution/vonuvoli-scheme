

use super::errors::exports::*;
use super::runtime_configurations::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{StdInto0, StdTryInto0, StdExpectInto0};
	pub use super::{StdAsRef0, StdTryAsRef0, StdExpectAsRef0};
	
	pub use super::Handle;
	
	pub use super::{vec_into};
	pub use super::{vec_append_2};
	pub use super::{vec_explode_1, vec_explode_1n, vec_explode_2, vec_explode_2n, vec_explode_3, vec_explode_3n};
	pub use super::{vec_zip_2, vec_unzip_2};
	pub use super::{vec_clone_fill};
	pub use super::{vec_clone_vec, vec_clone_slice};
	pub use super::{vec_clone_vec_ref, vec_clone_slice_ref, vec_clone_iter_ref};
	pub use super::{vec_vec_to_ref, vec_slice_to_ref, vec_iter_to_ref};
	pub use super::{vec_set};
	pub use super::{vec_set_ref};
	
	pub use super::{boxed_slice_to_ref};
	
	pub use super::{libc_getrusage_for_thread};
	pub use super::{libc_getpid};
	pub use super::{libc_kill};
	pub use super::{libc_memchr};
	pub use super::{libc_geteuid};
	pub use super::{libc_getegid};
	pub use super::{libc_getgroups};
	pub use super::{libc_close};
	pub use super::{libc_dup};
	pub use super::{libc_fcntl_flags_get, libc_fcntl_flags_set};
	
	pub use super::{execute_main};
	pub use super::{panic_with_error};
	
	pub use super::{parse_os_arguments, parse_os_environment};
	
	pub use super::super::runtime_backtrace::exports::*;
	pub use super::super::runtime_configurations::exports::*;
	pub use super::super::runtime_iterators::exports::*;
	pub use super::super::runtime_unicode::exports::*;
	
}




pub enum Alternative2 <T1, T2> {
	Variant1 (T1),
	Variant2 (T2),
}


impl <T1, T2> Alternative2<T1, T2> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	pub fn is_variant_1 (&self) -> (bool) {
		match *self {
			Alternative2::Variant1 (_) =>
				true,
			_ =>
				false,
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	pub fn is_variant_2 (&self) -> (bool) {
		match *self {
			Alternative2::Variant2 (_) =>
				true,
			_ =>
				false,
		}
	}
}




pub trait StdInto0 <T> : Sized {
	fn into_0 (self) -> (T);
}

pub trait StdTryInto0 <T> : Sized {
	type Error;
	fn try_into_0 (self) -> (Result<T, Self::Error>);
}

pub trait StdExpectInto0 <T> : Sized {
	fn expect_into_0 (self) -> (T);
}




/*
impl <T, U> StdInto0<U> for T where T : StdInto<U> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn into_0 (self) -> (U) {
		T::into (self)
	}
}
*/


impl <T, U> StdTryInto0<U> for T where T : StdTryInto<U> {
	
	type Error = T::Error;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_into_0 (self) -> (Result<U, Self::Error>) {
		T::try_into (self)
	}
}


impl <T, U> StdExpectInto0<U> for T where T : StdTryInto0<U> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn expect_into_0 (self) -> (U) {
		match T::try_into_0 (self) {
			Ok (value) =>
				value,
			Err (_) =>
				panic_0! (0x073cc689, github_issue_new),
		}
	}
}




pub trait StdAsRef0 <T> {
	fn as_ref_0 (&self) -> (&T);
}

pub trait StdTryAsRef0 <T> {
	type Error;
	fn try_as_ref_0 (&self) -> (Result<&T, Self::Error>);
}

pub trait StdExpectAsRef0 <T> {
	fn expect_as_ref_0 (&self) -> (&T);
}




impl <T, U> StdAsRef0<U> for T where T : StdAsRef<U> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn as_ref_0 (&self) -> (&U) {
		T::as_ref (self)
	}
}


impl <T, U> StdTryAsRef0<U> for T where T : StdAsRef0<U> {
	
	type Error = !;
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn try_as_ref_0 (&self) -> (Result<&U, Self::Error>) {
		Ok (T::as_ref_0 (self))
	}
}


impl <T, U> StdExpectAsRef0<U> for T where T : StdTryAsRef0<U> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn expect_as_ref_0 (&self) -> (&U) {
		match T::try_as_ref_0 (self) {
			Ok (value) =>
				value,
			Err (_) =>
				panic_0! (0xdd0868fb, github_issue_new),
		}
	}
}




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub struct Handle ( u64 );


impl Handle {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	pub fn new (handle : u64) -> (Handle) {
		return Handle ( handle );
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cast_lossless) ) ]
	pub const fn for_builtin (handle : u32) -> (Handle) {
		return Handle ( handle as u64 );
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	pub fn value (self) -> (u64) {
		return self.0;
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_into <Element, To : StdFrom<Element>> (from : StdVec<Element>) -> (StdVec<To>) {
	return vec_map_into! (from, value, value.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_append_2 <Element> (vector_1 : StdVec<Element>, vector_2 : StdVec<Element>) -> (StdVec<Element>) {
	let mut vector = StdVec::with_capacity (vector_1.len () + vector_2.len ());
	vector.extend (vector_1.into_iter ());
	vector.extend (vector_2.into_iter ());
	return vector;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_1 <Element> (vector : StdVec<Element>) -> (Outcome<Element>) {
	if vector.len () != 1 {
		fail! (0x0828936d);
	}
	let mut iterator = vector.into_iter ();
	succeed! (iterator.next () .expect ("a02552aa"));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_1n <Element> (vector : StdVec<Element>) -> (Outcome<(Element, StdVec<Element>)>) {
	if vector.is_empty () {
		fail! (0x2b9bdaf2);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
				iterator.next () .expect ("44d3f371"),
				iterator.collect (),
		));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_2 <Element> (vector : StdVec<Element>) -> (Outcome<(Element, Element)>) {
	if vector.len () != 2 {
		fail! (0x6865c09d);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
			iterator.next () .expect ("39cac0bc"),
			iterator.next () .expect ("f48578e8"),
		));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_2n <Element> (vector : StdVec<Element>) -> (Outcome<(Element, Element, StdVec<Element>)>) {
	if vector.len () < 2 {
		fail! (0x3dde9cf1);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
				iterator.next () .expect ("18112f60"),
				iterator.next () .expect ("ca645e46"),
				iterator.collect (),
		));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_3 <Element> (vector : StdVec<Element>) -> (Outcome<(Element, Element, Element)>) {
	if vector.len () != 3 {
		fail! (0xb6510cf5);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
			iterator.next () .expect ("f54cf984"),
			iterator.next () .expect ("d535aa19"),
			iterator.next () .expect ("5331af52"),
		));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_explode_3n <Element> (vector : StdVec<Element>) -> (Outcome<(Element, Element, Element, StdVec<Element>)>) {
	if vector.len () < 3 {
		fail! (0x2d2644c7);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
				iterator.next () .expect ("f7012d8a"),
				iterator.next () .expect ("a4f1d7ae"),
				iterator.next () .expect ("8d161b2e"),
				iterator.collect (),
		));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_zip_2 <Element1, Element2> (vector_1 : StdVec<Element1>, vector_2 : StdVec<Element2>) -> (StdVec<(Element1, Element2)>) {
	if vector_1.len () != vector_2.len () {
		panic_0! (0xa8f6ee9e, github_issue_new);
	}
	let mut vector = StdVec::with_capacity (vector_1.len ());
	let mut vector_1 = vector_1.into_iter ();
	let mut vector_2 = vector_2.into_iter ();
	loop {
		match (vector_1.next (), vector_2.next ()) {
			(Some (element_1), Some (element_2)) =>
				vector.push ((element_1, element_2)),
			(None, None) =>
				return vector,
			(Some (_), None) =>
				unreachable_0! (0x7c360c22, github_issue_new),
			(None, Some (_)) =>
				unreachable_0! (0xaac907db, github_issue_new),
		}
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_unzip_2 <Element1, Element2> (vector : StdVec<(Element1, Element2)>) -> ((StdVec<Element1>, StdVec<Element2>)) {
	let mut vector_1 = StdVec::with_capacity (vector.len ());
	let mut vector_2 = StdVec::with_capacity (vector.len ());
	for (element_1, element_2) in vector {
		vector_1.push (element_1);
		vector_2.push (element_2);
	}
	return (vector_1, vector_2);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_clone_fill <Element : Clone, ElementRef : StdAsRef<Element>> (value : ElementRef, capacity : usize) -> (StdVec<Element>) {
	let mut vector = StdVec::with_capacity (capacity);
	let value = value.as_ref ();
	for _ in 0 .. capacity {
		let value = value.clone ();
		vector.push (value);
	}
	return vector;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (ptr_arg) ) ]
pub fn vec_clone_vec <Element : Clone> (vector : &StdVec<Element>) -> (StdVec<Element>) {
	return vector.clone ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_clone_slice <Element : Clone> (slice : &[Element]) -> (StdVec<Element>) {
	return vec_map! (slice.iter (), value, (*value).clone ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (ptr_arg) ) ]
pub fn vec_clone_vec_ref <Element : Clone, ElementRef : StdAsRef<Element>> (vector : &StdVec<ElementRef>) -> (StdVec<Element>) {
	return vec_map! (vector.iter (), value, value.as_ref () .clone ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_clone_slice_ref <Element : Clone, ElementRef : StdAsRef<Element>> (slice : &[ElementRef]) -> (StdVec<Element>) {
	return vec_map! (slice.iter (), value, value.as_ref () .clone ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_clone_iter_ref <Element : Clone, ElementRef : StdAsRef<Element>, Iterator : iter::Iterator<Item = ElementRef>> (iterator : Iterator) -> (StdVec<Element>) {
	return vec_map! (iterator, value, value.as_ref () .clone ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (ptr_arg) ) ]
pub fn vec_vec_to_ref <Element, ElementRef : StdAsRef<Element>> (vector : &StdVec<ElementRef>) -> (StdVec<&Element>) {
	return vec_map! (vector.iter (), value, value.as_ref ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_slice_to_ref <Element, ElementRef : StdAsRef<Element>> (slice : &[ElementRef]) -> (StdVec<&Element>) {
	return vec_map! (slice.iter (), value, value.as_ref ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_iter_to_ref <'a, Element : 'a, ElementRef : StdAsRef<Element> + 'a, Iterator : iter::Iterator<Item = &'a ElementRef>> (iterator : Iterator) -> (StdVec<&'a Element>) {
	return vec_map! (iterator, value, value.as_ref ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_set <Element : Clone> (vector : &mut StdVec<Element>, index : usize, value : &Element) -> (Outcome<()>) {
	if let Some (target) = vector.get_mut (index) {
		let value = value.clone ();
		*target = value;
		succeed! (());
	} else {
		fail! (0x3a93081f);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn vec_set_ref <Element : Clone, ElementRef : StdAsRef<Element>> (vector : &mut StdVec<Element>, index : usize, value : ElementRef) -> (Outcome<()>) {
	if let Some (target) = vector.get_mut (index) {
		let value = value.as_ref ();
		let value = value.clone ();
		*target = value;
		succeed! (());
	} else {
		fail! (0x117d74b3);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (borrowed_box) ) ]
pub fn boxed_slice_to_ref <'a, Element : 'a, ElementRef : StdAsRef<Element> + 'a> (slice : &'a StdBox<[ElementRef]>) -> (StdBox<[&'a Element]>) {
	return vec_map! (slice.iter (), value, value.as_ref ()) .into_boxed_slice ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_getrusage_for_thread () -> (ext::libc::rusage) {
	unsafe {
		let mut resources = mem::zeroed ();
		if ext::libc::getrusage (ext::libc::RUSAGE_THREAD, &mut resources) == 0 {
			resources
		} else {
			panic_0! (0xfc7fa1cb, github_issue_new);
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_getpid () -> (Outcome<ext::libc::pid_t>) {
	unsafe {
		let outcome = ext::libc::getpid ();
		if outcome <= 0 {
			fail! (0x7b1f7fb4);
		}
		succeed! (outcome);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_kill (process : ext::libc::pid_t, signal : ext::libc::c_int) -> (Outcome<()>) {
	unsafe {
		if ext::libc::kill (process, signal) == 0 {
			succeed! (());
		} else {
			fail! (0x41a2990d);
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_memchr (search : u8, buffer : &[u8]) -> (Option<usize>) {
	unsafe {
		let buffer_pointer = buffer.as_ptr () as * const ext::libc::c_void;
		let found_pointer = ext::libc::memchr (
				buffer_pointer,
				ext::libc::c_int::from (search),
				buffer.len ()
			);
		if found_pointer.is_null () {
			return None;
		} else {
			return Some ((found_pointer as usize) - (buffer_pointer as usize));
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_geteuid () -> (u32) {
	unsafe {
		ext::libc::geteuid ()
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_getegid () -> (u32) {
	unsafe {
		ext::libc::getegid ()
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_getgroups () -> (StdBox<[u32]>) {
	let groups_count = unsafe {
		let outcome = ext::libc::getgroups (0, ptr::null_mut ());
		if outcome < 0 {
			panic_0! (0xa42932ca, github_issue_new);
		}
		outcome as usize
	};
	let mut groups = StdVec::with_capacity (groups_count);
	unsafe {
		let outcome = ext::libc::getgroups (groups_count as i32, groups.as_mut_ptr ());
		if outcome < 0 {
			panic_0! (0xcb1afa65, github_issue_new);
		}
		if outcome as usize != groups_count {
			panic_0! (0x78984a31, github_issue_new);
		}
		groups.set_len (groups_count);
	}
	let groups = groups.into_boxed_slice ();
	return groups;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_close (descriptor : unix_io::RawFd) -> (Outcome<()>) {
	unsafe {
		let outcome = ext::libc::close (descriptor);
		if outcome < 0 {
			fail! (0x0377b93a);
		}
		succeed! (());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_dup (descriptor : unix_io::RawFd) -> (Outcome<unix_io::RawFd>) {
	unsafe {
		let outcome = ext::libc::dup (descriptor);
		if outcome < 0 {
			fail! (0xbb2aa207);
		}
		succeed! (outcome);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_fcntl_flags_get (descriptor : unix_io::RawFd) -> (Outcome<u16>) {
	unsafe {
		let outcome = ext::libc::fcntl (descriptor, ext::libc::F_GETFD);
		if outcome < 0 {
			fail! (0x3e0aedac);
		}
		succeed! (outcome as u16);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn libc_fcntl_flags_set (descriptor : unix_io::RawFd, flags : u16) -> (Outcome<()>) {
	unsafe {
		let outcome = ext::libc::fcntl (descriptor, ext::libc::F_SETFD, ext::libc::c_int::from (flags));
		if outcome < 0 {
			fail! (0xf0a6e89f);
		}
		succeed! (());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn execute_main <Main, Tracer> (main : Main, transcript : &Tracer) -> !
		where
			Main : Fn () -> (Outcome<u32>) + panic::UnwindSafe,
			Tracer : Transcript + ?Sized,
{
	panic::set_hook (StdBox::new (|_| {
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			let backtrace = super::runtime_backtrace::Backtrace::new ();
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			backtrace.report (tracer_error! (super::TRANSCRIPT, 0x7f5eeb6e));
			if ABORT_ON_PANIC_GENERIC {
				process::exit (1);
			}
		}));
	match panic::catch_unwind (main) {
		Ok (outcome) =>
			match outcome {
				Ok (code) => {
					let code = if code <= 255 {
						code
					} else {
						#[ cfg ( feature = "vonuvoli_transcript" ) ]
						trace_warning! (transcript, 0x2daa4ba6 => "exit code (`{}`) out of range;  using `255`!" => (code));
						255
					};
					process::exit (code as i32);
				},
				Err (error) => {
					#[ cfg ( feature = "vonuvoli_transcript" ) ]
					trace_critical! (transcript, 0x4354c758 => "unexpected error encountered;  aborting!" => (), error = &error);
					#[ cfg ( feature = "vonuvoli_transcript" ) ]
					error.backtrace_report (tracer_error! (transcript, 0x6ec79d16));
					process::exit (1);
				},
			},
		Err (error) => {
			let error = match error.downcast::<Error> () {
				Ok (error) => {
					let error = error.deref ();
					#[ cfg ( feature = "vonuvoli_transcript" ) ]
					trace_critical! (transcript, 0x8c0fc747 => "unexpected panic encountered;  aborting!" => (), error = error);
					#[ cfg ( feature = "vonuvoli_transcript" ) ]
					error.backtrace_report (tracer_error! (transcript, 0x29b62906));
					process::exit (1);
				},
				Err (error) =>
					error,
			};
			let error = match error.downcast::<StdString> () {
				Ok (error) => {
					let error = error.deref ();
					#[ cfg ( feature = "vonuvoli_transcript" ) ]
					trace_critical! (transcript, 0x4981dad6 => "unexpected panic encountered;  aborting!\u{1d}" => (), error = error);
					process::exit (1);
				},
				Err (error) =>
					error,
			};
			let _error = error;
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_critical! (transcript, 0x5006e6bf => "unexpected panic encountered;  aborting!" => ());
			process::exit (1);
		},
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
pub fn panic_with_error (error : Error, code : u32, source : &(&'static str, u32, u32), _message : Option<&'static str>) -> ! {
	//  TODO:  use message if provided!
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	trace_critical! (super::TRANSCRIPT, 0x6be7d1b0 => "unexpected panic encountered [{:08x}];  aborting!" => (code), error = &error);
	if ABORT_ON_PANIC_WITH_ERROR {
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		error.backtrace_report (tracer_error! (super::TRANSCRIPT, 0xf0766ceb));
		process::exit (1);
	} else {
		let error = format! ("[{:08x}]  {}", code, error);
		::std::rt::begin_panic (error, source);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn parse_os_arguments (os_arguments : StdVec<ffi::OsString>) -> (Outcome<(StdVec<ffi::OsString>, StdVec<ffi::OsString>)>) {
	
	let mut interpreter_arguments = StdVec::new ();
	let mut process_arguments = StdVec::new ();
	
	let mut interpreter_expand = true;
	for argument in os_arguments {
		if interpreter_expand {
			if let Some (argument) = argument.to_str () {
				if argument == "--" {
					interpreter_expand = false;
					continue;
				}
			}
		}
		if interpreter_expand {
			interpreter_arguments.push (argument);
		} else {
			process_arguments.push (argument);
		}
	}
	
	succeed! ((interpreter_arguments, process_arguments));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (type_complexity) ) ]
pub fn parse_os_environment (os_environment : StdVec<(ffi::OsString, ffi::OsString)>) -> (Outcome<(StdVec<(ffi::OsString, ffi::OsString)>, StdVec<(ffi::OsString, ffi::OsString)>)>) {
	
	let mut interpreter_environment = StdVec::new ();
	let mut process_environment = StdVec::new ();
	
	for (name, value) in os_environment {
		let interpreter_extend = if let Some (name) = name.to_str () {
			name.starts_with ("VONUVOLI_SCHEME_")
		} else {
			false
		};
		if interpreter_extend {
			interpreter_environment.push ((name, value));
		} else {
			process_environment.push ((name, value));
		}
	}
	
	succeed! ((interpreter_environment, process_environment));
}




#[ cfg ( not ( feature = "vonuvoli_transcript" ) ) ]
pub trait Transcript : StdAny {}

