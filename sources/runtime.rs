

use super::errors::exports::*;




pub mod exports {
	
	pub use std::borrow::Borrow as StdBorrow;
	pub use std::boxed::Box as StdBox;
	pub use std::cell::Cell as StdCell;
	pub use std::cell::Ref as StdRef;
	pub use std::cell::RefCell as StdRefCell;
	pub use std::cell::RefMut as StdRefMut;
	pub use std::collections::HashMap as StdMap;
	pub use std::convert::AsRef as StdAsRef;
	pub use std::convert::From as StdFrom;
	pub use std::convert::Into as StdInto;
	pub use std::rc::Rc as StdRc;
	pub use std::result::Result as StdResult;
	pub use std::string::String as StdString;
	pub use std::vec::Vec as StdVec;
	
	pub use super::StdTryFrom;
	pub use super::StdTryInto;
	pub use super::StdTryAsRef;
	
	pub use super::{vec_into, vec_clone_slice};
	pub use super::{vec_append_2};
	pub use super::{vec_explode_1, vec_explode_1n, vec_explode_2, vec_explode_2n, vec_explode_3, vec_explode_3n};
	pub use super::{vec_zip_2};
	
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




pub fn vec_into <Element, To : From<Element>> (from : Vec<Element>) -> (Vec<To>) {
	vec_map! (from, value, value.into ())
}

pub fn vec_clone_slice <Element : Clone, To : From<Element>> (from : &[Element]) -> (Vec<To>) {
	vec_map! (from.to_vec (), value, value.into ())
}




pub fn vec_append_2 <Element> (vector_1 : Vec<Element>, vector_2 : Vec<Element>) -> (Vec<Element>) {
	let mut vector = Vec::with_capacity (vector_1.len () + vector_2.len ());
	vector.extend (vector_1.into_iter ());
	vector.extend (vector_2.into_iter ());
	return vector;
}




pub fn vec_explode_1 <Element> (vector : Vec<Element>) -> (Outcome<Element>) {
	if vector.len () != 1 {
		fail! (0x0828936d);
	}
	let mut iterator = vector.into_iter ();
	succeed! (iterator.next () .expect ("a116f5d2"));
}

pub fn vec_explode_1n <Element> (vector : Vec<Element>) -> (Outcome<(Element, Vec<Element>)>) {
	if vector.len () < 1 {
		fail! (0x2b9bdaf2);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
				iterator.next () .expect ("a116f5d2"),
				iterator.collect (),
		));
}


pub fn vec_explode_2 <Element> (vector : Vec<Element>) -> (Outcome<(Element, Element)>) {
	if vector.len () != 2 {
		fail! (0x6865c09d);
	}
	let mut iterator = vector.into_iter ();
	succeed! ((
			iterator.next () .expect ("39cac0bc"),
			iterator.next () .expect ("f48578e8"),
		));
}

pub fn vec_explode_2n <Element> (vector : Vec<Element>) -> (Outcome<(Element, Element, Vec<Element>)>) {
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


pub fn vec_explode_3 <Element> (vector : Vec<Element>) -> (Outcome<(Element, Element, Element)>) {
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

pub fn vec_explode_3n <Element> (vector : Vec<Element>) -> (Outcome<(Element, Element, Element, Vec<Element>)>) {
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




pub fn vec_zip_2 <Element1, Element2> (vector_1 : Vec<Element1>, vector_2 : Vec<Element2>) -> (Vec<(Element1, Element2)>) {
	return vector_1.into_iter () .zip (vector_2.into_iter ()) .collect ();
}

