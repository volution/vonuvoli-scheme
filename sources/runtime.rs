

use super::errors::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::Handle;
	
	pub use super::{vec_into};
	pub use super::{vec_append_2};
	pub use super::{vec_explode_1, vec_explode_1n, vec_explode_2, vec_explode_2n, vec_explode_3, vec_explode_3n};
	pub use super::{vec_zip_2, vec_unzip_2};
	pub use super::{vec_clone_vec, vec_clone_slice};
	pub use super::{vec_clone_vec_ref, vec_clone_slice_ref, vec_clone_iter_ref};
	pub use super::{vec_vec_to_ref, vec_slice_to_ref, vec_iter_to_ref};
	
	pub use super::{boxed_slice_to_ref};
	
	pub use super::{libc_getrusage_for_thread};
	
	pub use super::super::runtime_iterators::exports::*;
	pub use super::super::runtime_unicode::exports::*;
	
}




pub trait StdTryFrom <T> : Sized {
	type Error;
	fn try_from (value: T) -> (Result<Self, Self::Error>);
}

pub trait StdTryInto <T> : Sized {
	type Error;
	fn try_into (self) -> (Result<T, Self::Error>);
}

pub trait StdTryAsRef <T> {
	type Error;
	fn try_as_ref (&self) -> (Result<&T, Self::Error>);
}




#[ derive (Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Handle ( u32 );


impl Handle {
	
	#[ inline (always) ]
	pub fn new (handle : u32) -> (Handle) {
		return Handle ( handle );
	}
	
	#[ inline (always) ]
	pub fn value (&self) -> (u32) {
		return self.0;
	}
}


impl fmt::Display for Handle {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "#<handle:{:08x}>", self.0);
	}
}


impl fmt::Debug for Handle {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		return write! (formatter, "Handle({:08x})", self.0);
	}
}




#[ inline (always) ]
pub fn vec_into <Element, To : StdFrom<Element>> (from : StdVec<Element>) -> (StdVec<To>) {
	return vec_map_into! (from, value, value.into ());
}




#[ inline (always) ]
pub fn vec_append_2 <Element> (vector_1 : StdVec<Element>, vector_2 : StdVec<Element>) -> (StdVec<Element>) {
	let mut vector = StdVec::with_capacity (vector_1.len () + vector_2.len ());
	vector.extend (vector_1.into_iter ());
	vector.extend (vector_2.into_iter ());
	return vector;
}




#[ inline (always) ]
pub fn vec_explode_1 <Element> (vector : StdVec<Element>) -> (Outcome<Element>) {
	if vector.len () != 1 {
		fail! (0x0828936d);
	}
	let mut iterator = vector.into_iter ();
	succeed! (iterator.next () .expect ("a02552aa"));
}

#[ inline (always) ]
pub fn vec_explode_1n <Element> (vector : StdVec<Element>) -> (Outcome<(Element, StdVec<Element>)>) {
	if vector.len () < 1 {
		fail! (0x2b9bdaf2);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
				iterator.next () .expect ("44d3f371"),
				iterator.collect (),
		));
}


#[ inline (always) ]
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

#[ inline (always) ]
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


#[ inline (always) ]
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

#[ inline (always) ]
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




#[ inline (always) ]
pub fn vec_zip_2 <Element1, Element2> (vector_1 : StdVec<Element1>, vector_2 : StdVec<Element2>) -> (StdVec<(Element1, Element2)>) {
	if vector_1.len () != vector_2.len () {
		panic! ("a8f6ee9e");
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
				panic! ("7c360c22"),
			(None, Some (_)) =>
				panic! ("aac907db"),
		}
	}
}

#[ inline (always) ]
pub fn vec_unzip_2 <Element1, Element2> (vector : StdVec<(Element1, Element2)>) -> ((StdVec<Element1>, StdVec<Element2>)) {
	let mut vector_1 = StdVec::with_capacity (vector.len ());
	let mut vector_2 = StdVec::with_capacity (vector.len ());
	for (element_1, element_2) in vector.into_iter () {
		vector_1.push (element_1);
		vector_2.push (element_2);
	}
	return (vector_1, vector_2);
}




#[ inline (always) ]
pub fn vec_clone_vec <Element : Clone> (vector : &StdVec<Element>) -> (StdVec<Element>) {
	return vec_map! (vector.iter (), value, value.clone ());
}

#[ inline (always) ]
pub fn vec_clone_slice <Element : Clone> (slice : &[Element]) -> (StdVec<Element>) {
	return vec_map! (slice.iter (), value, (*value).clone ());
}




#[ inline (always) ]
pub fn vec_clone_vec_ref <Element : Clone, ElementRef : StdAsRef<Element>> (vector : &StdVec<ElementRef>) -> (StdVec<Element>) {
	return vec_map! (vector.iter (), value, value.as_ref () .clone ());
}

#[ inline (always) ]
pub fn vec_clone_slice_ref <Element : Clone, ElementRef : StdAsRef<Element>> (slice : &[ElementRef]) -> (StdVec<Element>) {
	return vec_map! (slice.iter (), value, value.as_ref () .clone ());
}

#[ inline (always) ]
pub fn vec_clone_iter_ref <Element : Clone, ElementRef : StdAsRef<Element>, Iterator : iter::Iterator<Item = ElementRef>> (iterator : Iterator) -> (StdVec<Element>) {
	return vec_map! (iterator, value, value.as_ref () .clone ());
}




#[ inline (always) ]
pub fn vec_vec_to_ref <Element, ElementRef : StdAsRef<Element>> (vector : &StdVec<ElementRef>) -> (StdVec<&Element>) {
	return vec_map! (vector.iter (), value, value.as_ref ());
}

#[ inline (always) ]
pub fn vec_slice_to_ref <Element, ElementRef : StdAsRef<Element>> (slice : &[ElementRef]) -> (StdVec<&Element>) {
	return vec_map! (slice.iter (), value, value.as_ref ());
}

#[ inline (always) ]
pub fn vec_iter_to_ref <'a, Element : 'a, ElementRef : StdAsRef<Element> + 'a, Iterator : iter::Iterator<Item = &'a ElementRef>> (iterator : Iterator) -> (StdVec<&'a Element>) {
	return vec_map! (iterator, value, value.as_ref ());
}




#[ inline (always) ]
pub fn boxed_slice_to_ref <'a, Element : 'a, ElementRef : StdAsRef<Element> + 'a> (slice : &'a StdBox<[ElementRef]>) -> (StdBox<[&'a Element]>) {
	return vec_map! (slice.iter (), value, value.as_ref ()) .into_boxed_slice ();
}




#[ inline (always) ]
pub fn libc_getrusage_for_thread () -> (libc::rusage) {
	unsafe {
		let mut resources = mem::zeroed ();
		if libc::getrusage (libc::RUSAGE_THREAD, &mut resources) == 0 {
			resources
		} else {
			panic! ("fc7fa1cb");
		}
	}
}

