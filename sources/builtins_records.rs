

use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			record_kind_is,
			
			record_build_0, record_build_1, record_build_2, record_build_3, record_build_4, record_build_n,
			
			record_get, record_set,
			
		};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_is (kind : &RecordKind, value : &Value) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::RecordImmutable (value) =>
			return RecordKind::is_self (value.kind (), kind),
		ValueKindMatchAsRef::RecordMutable (value) =>
			return RecordKind::is_self (value.kind (), kind),
		_ =>
			return false,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_0 (kind : &RecordKind, fields : Option<&[usize]>, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 0 {
			fail! (0x6007924a);
		}
		vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ())
	} else {
		vec! []
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_1 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 1 {
			fail! (0x03ce30be);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		values
	} else {
		vec! [value_1.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_2 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 2 {
			fail! (0x34c1df40);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		values
	} else {
		vec! [value_1.clone (), value_2.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_3 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 3 {
			fail! (0xa678b9a9);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		try! (vec_set (&mut values, fields[2], value_3));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_4 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 4 {
			fail! (0xef4b85b7);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		try! (vec_set (&mut values, fields[2], value_3));
		try! (vec_set (&mut values, fields[3], value_4));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone (), value_4.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_n <ValueRef : StdAsRef<Value>> (kind : &RecordKind, fields : Option<&[usize]>, values : &[ValueRef], immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != values.len () {
			fail! (0x9c52a9f1);
		}
		let mut values_ = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		for index in 0 .. fields.len () {
			try! (vec_set_ref (&mut values_, fields[index], &values[index]));
		}
		values_
	} else {
		vec_clone_slice_ref (values)
	};
	return record_new (kind, values, immutable);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_get (kind : &RecordKind, field : usize, record : &Value) -> (Outcome<Value>) {
	let record = try_as_record_ref! (record);
	if ! record.is_kind (kind) {
		fail! (0xe5012bde);
	}
	let record = record.values_as_slice ();
	let value = try_some! (record.get (field), 0xcce25bab);
	let value = value.clone ();
	succeed! (value);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_set (kind : &RecordKind, field : usize, record : &Value, value : &Value) -> (Outcome<Value>) {
	let record = try_as_record_mutable_ref! (record);
	if ! record.is_kind (kind) {
		fail! (0x64c0a2cd);
	}
	let mut record = try! (record.values_ref_mut ());
	let value_ref = try_some! (record.get_mut (field), 0x51cf23d4);
	let mut value_swap = value.clone ();
	mem::swap (&mut value_swap, value_ref);
	succeed! (value_swap);
}

