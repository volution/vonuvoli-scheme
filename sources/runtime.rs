

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

