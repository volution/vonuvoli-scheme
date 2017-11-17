

use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::{pair};
	pub use super::{list_build_1, list_build_2, list_build_3, list_build_4, list_build_n};
	pub use super::{list_append_2, list_append_3, list_append_4, list_append_n};
	pub use super::{vec_clone_list, vec_drain_list, vec_drain_list_dotted};
	pub use super::{is_true, is_false, is_not_false, is_true_or_equivalent, is_false_or_equivalent};
}




#[ inline (always) ]
pub fn pair (left : &Value, right : &Value) -> (Value) {
	pair_new (left.clone (), right.clone ()) .into ()
}




#[ inline (always) ]
pub fn list_build_1 (value_1 : &Value) -> (Value) {
	pair_new (value_1.clone (), NULL) .into ()
}

#[ inline (always) ]
pub fn list_build_2 (value_1 : &Value, value_2 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), NULL) .into ()) .into ()
}

#[ inline (always) ]
pub fn list_build_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), NULL) .into ()) .into ()) .into ()
}

#[ inline (always) ]
pub fn list_build_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (Value) {
	pair_new (value_1.clone (), pair_new (value_2.clone (), pair_new (value_3.clone (), pair_new (value_4.clone (), NULL) .into ()) .into ()) .into ()) .into ()
}

#[ inline (always) ]
pub fn list_build_n (values : &[Value]) -> (Value) {
	if values.is_empty () {
		NULL
	} else {
		values.iter () .rev () .fold (NULL, |last, value| Value::Pair (pair_new (value.clone (), last)))
	}
}




#[ inline (always) ]
pub fn list_append_2 (list_1 : &Value, list_2 : &Value) -> (Outcome<Value>) {
	if (*list_1 == NULL) && (*list_2 == NULL) {
		succeed! (NULL);
	}
	let mut values = ValueVec::new ();
	try! (vec_drain_list (&mut values, &list_1));
	match try! (vec_drain_list_dotted (&mut values, &list_2)) {
		Some (last) =>
			succeed! (list_dotted_new (values, last)),
		None =>
			succeed! (list_new (values)),
	}
}

#[ inline (always) ]
pub fn list_append_3 (list_1 : &Value, list_2 : &Value, list_3 : &Value) -> (Outcome<Value>) {
	if (*list_1 == NULL) && (*list_2 == NULL) && (*list_3 == NULL) {
		succeed! (NULL);
	}
	let mut values = ValueVec::new ();
	try! (vec_drain_list (&mut values, &list_1));
	try! (vec_drain_list (&mut values, &list_2));
	match try! (vec_drain_list_dotted (&mut values, &list_3)) {
		Some (last) =>
			succeed! (list_dotted_new (values, last)),
		None =>
			succeed! (list_new (values)),
	}
}

#[ inline (always) ]
pub fn list_append_4 (list_1 : &Value, list_2 : &Value, list_3 : &Value, list_4 : &Value) -> (Outcome<Value>) {
	if (*list_1 == NULL) && (*list_2 == NULL) && (*list_3 == NULL) && (*list_4 == NULL) {
		succeed! (NULL);
	}
	let mut values = ValueVec::new ();
	try! (vec_drain_list (&mut values, &list_1));
	try! (vec_drain_list (&mut values, &list_2));
	try! (vec_drain_list (&mut values, &list_3));
	match try! (vec_drain_list_dotted (&mut values, &list_4)) {
		Some (last) =>
			succeed! (list_dotted_new (values, last)),
		None =>
			succeed! (list_new (values)),
	}
}

#[ inline (always) ]
pub fn list_append_n (lists : &[Value]) -> (Outcome<Value>) {
	match lists.split_last () {
		Some ((list_last, lists_first)) =>
			if lists_first.is_empty () {
				succeed! ((*list_last).clone ());
			} else {
				let mut values = ValueVec::new ();
				for list in lists_first {
					try! (vec_drain_list (&mut values, &list));
				}
				match try! (vec_drain_list_dotted (&mut values, &list_last)) {
					Some (last) =>
						succeed! (list_dotted_new (values, last)),
					None =>
						succeed! (list_new (values)),
				}
			},
		None =>
			succeed! (NULL),
	}
}




#[ inline (always) ]
pub fn vec_clone_list (list : &Value) -> (Outcome<ValueVec>) {
	let mut vector = ValueVec::new ();
	try! (vec_drain_list (&mut vector, list));
	succeed! (vector);
}


#[ inline (always) ]
pub fn vec_drain_list (vector : &mut ValueVec, list : &Value) -> (Outcome<()>) {
	match try! (vec_drain_list_dotted (vector, list)) {
		Some (_value) =>
			fail! (0x57ebb8de),
		None =>
			succeed! (()),
	}
}


#[ inline (always) ]
pub fn vec_drain_list_dotted (vector : &mut ValueVec, list : &Value) -> (Outcome<Option<Value>>) {
	let mut cursor = list;
	loop {
		match cursor {
			&Value::Pair (ref pair) => {
				vector.push (pair.left () .clone ());
				cursor = pair.right ();
			},
			&Value::Null =>
				succeed! (None),
			ref value =>
				succeed! (Some ((*value).clone ())),
		}
	}
}




#[ inline (always) ]
pub fn is_true (value : &Value) -> (bool) {
	*value == TRUE.into ()
}

#[ inline (always) ]
pub fn is_false (value : &Value) -> (bool) {
	*value == FALSE.into ()
}

#[ inline (always) ]
pub fn is_not_false (value : &Value) -> (bool) {
	*value != FALSE.into ()
}

#[ inline (always) ]
pub fn is_true_or_equivalent (value : &Value) -> (bool) {
	!is_false_or_equivalent (value)
}

#[ inline (always) ]
pub fn is_false_or_equivalent (value : &Value) -> (bool) {
	match *value {
		Value::Null | Value::Void | Value::Undefined =>
			true,
		Value::Boolean (FALSE) =>
			true,
		Value::Error (_) =>
			true,
		_ =>
			false,
	}
}

