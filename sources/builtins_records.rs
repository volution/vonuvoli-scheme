

use super::builtins::exports::*;
use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			record_kind_build,
			
			record_kind_identifier,
			record_kind_size,
			
			record_kind_resolve_field,
			record_kind_resolve_field_identifier,
			record_kind_resolve_field_index,
			record_kind_resolve_field_indices,
			
		};
	
	pub use super::{
			
			record_kind_get, record_kind_is,
			
			record_build,
			record_build_0, record_build_1, record_build_2, record_build_3, record_build_4, record_build_n,
			
			record_resolve_field_index,
			
			record_get,
			record_get_x,
			
			record_ref,
			record_as_ref,
			record_immutable_as_ref,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			record_set,
			record_set_x,
			
			record_mutable_as_ref,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::{
			
			record_to_array, record_from_array,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::{
			
			record_to_values, record_from_values,
			
		};
	
	pub use super::{
			
			record_to_list, record_from_list,
			record_to_assoc, record_from_assoc,
			
		};
	
	pub use super::{
			
			record_kind_is_fn,
			
			record_build_fn_n,
			record_build_fn_c,
			
		};
	
	pub use super::{
			
			record_get_fn,
			record_get_x_fn,
			
		};
	
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	pub use super::{
			
			record_set_fn,
			record_set_x_fn,
			
		};
	
}




pub fn record_kind_build (identifier : Option<&Value>, fields : &Value) -> (Outcome<RecordKind>) {
	let (fields, size) = match fields.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (fields) =>
			(None, r#try! (fields.try_to_usize ())),
		_ => {
			let fields = r#try! (list_or_array_coerce_clone (fields));
			let fields = try_vec_map_into! (fields, field,
					{
						let (field, mutable) : (Option<Value>, Option<Value>) = match field.class_match_as_ref () {
							ValueClassMatchAsRef::Symbol (field) =>
								(Some (field.clone () .into ()), Some (TRUE_VALUE)),
							ValueClassMatchAsRef::Boolean (field) =>
								(None, Some (field.clone () .into ())),
							ValueClassMatchAsRef::Pair (field) => {
								let field = r#try! (field.pair_ref ());
								let field_identifier = field.left () .clone ();
								let field_mutable = field.right () .clone ();
								(Some (field_identifier), Some (field_mutable))
							},
							_ =>
								fail! (0x25152a15),
						};
						let field = if let Some (field) = field {
							match field.kind_match_as_ref () {
								ValueKindMatchAsRef::Symbol (field) =>
									Some (field.string_rc_clone ()),
								ValueKindMatchAsRef::Boolean (field) =>
									if ! field.value () {
										None
									} else {
										fail! (0x1b9efccb)
									},
								_ =>
									fail! (0xa1c091f1),
							}
						} else {
							None
						};
						let mutable = if let Some (mutable) = mutable {
							let mutable = try_as_boolean_ref! (&mutable);
							mutable.value ()
						} else {
							true
						};
						succeed! ((field, mutable));
					});
			let size = fields.len ();
			let fields = fields.into_boxed_slice ();
			(Some (fields), size)
		},
	};
	let identifier = if let Some (identifier) = identifier {
		match identifier.kind_match_as_ref () {
			ValueKindMatchAsRef::Boolean (identifier) =>
				if ! identifier.value () {
					None
				} else {
					fail! (0xddeb44fd);
				},
			ValueKindMatchAsRef::Symbol (identifier) =>
				Some (identifier.string_rc_clone ()),
			_ =>
				fail! (0xbd53861b),
		}
	} else {
		None
	};
	return RecordKind::new (identifier, fields, size);
}




pub fn record_kind_identifier (kind : &Value) -> (Outcome<Value>) {
	let kind = try_as_record_kind_ref! (kind);
	if let Some (identifier) = kind.identifier_rc_clone () {
		succeed! (Symbol::from_rc (identifier) .into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}

pub fn record_kind_size (kind : &Value) -> (Outcome<usize>) {
	let kind = try_as_record_kind_ref! (kind);
	succeed! (kind.values_count ());
}




pub fn record_kind_resolve_field <'a> (kind : &'a RecordKind, field : &Value) -> (Outcome<&'a RecordKindField>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) => {
			let field = r#try! (field.try_to_usize ());
			let field = try_some! (kind.field_by_index (field), 0x3e6492c1);
			succeed! (field);
		},
		ValueKindMatchAsRef::Symbol (field) => {
			let field = field.string_as_str ();
			let field = try_some! (kind.field_by_identifier (field), 0x4b1ac298);
			succeed! (field);
		},
		_ =>
			fail! (0xe0e0c34b),
	}
}

pub fn record_kind_resolve_field_identifier (kind : &RecordKind, field : &Value) -> (Outcome<Value>) {
	let field = r#try! (record_kind_resolve_field (kind, field));
	if let Some (ref identifier) = field.identifier {
		succeed! (Symbol::clone_rc (identifier) .into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}

pub fn record_kind_resolve_field_index (kind : &RecordKind, field : &Value) -> (Outcome<usize>) {
	let field = r#try! (record_kind_resolve_field (kind, field));
	succeed! (field.index);
}

pub fn record_kind_resolve_field_indices (kind : &RecordKind, fields : &Value) -> (Outcome<Option<StdBox<[usize]>>>) {
	match fields.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (field) =>
			if ! field.value () {
				succeed! (None);
			} else {
				let fields = (0 .. kind.values_count ()) .collect::<StdVec<usize>> ();
				succeed! (Some (fields.into_boxed_slice ()));
			},
		_ => {
			let fields = r#try! (list_or_array_coerce_clone (fields));
			let fields = try_vec_map_into! (fields, field, record_kind_resolve_field_index (kind, &field));
			succeed! (Some (fields.into_boxed_slice ()));
		},
	}
}




pub fn record_kind_get (value : &Value) -> (Outcome<RecordKind>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::RecordImmutable (value) =>
			succeed! (value.kind () .clone ()),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::RecordMutable (value) =>
			succeed! (value.kind () .clone ()),
		_ =>
			fail! (0xc4f39aeb),
	}
}

pub fn record_kind_is (kind : &RecordKind, value : &Value, immutable : Option<bool>) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::RecordImmutable (value) =>
			return RecordKind::is_self (value.kind (), kind) && immutable.unwrap_or (true),
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		ValueKindMatchAsRef::RecordMutable (value) =>
			return RecordKind::is_self (value.kind (), kind) && ! immutable.unwrap_or (false),
		_ =>
			return false,
	}
}




pub fn record_build (kind : &RecordKind, fields : Option<&[usize]>, values : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = r#try! (sequence_coerce_clone (values));
	let values = if let Some (fields) = fields {
		if fields.len () != values.len () {
			fail! (0x27fd4ee2);
		}
		let mut values_ = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		for index in 0 .. fields.len () {
			r#try! (vec_set_ref (&mut values_, fields[index], &values[index]));
		}
		values_
	} else {
		values
	};
	return record_new (kind, values, immutable);
}


pub fn record_build_0 (kind : &RecordKind, fields : Option<&[usize]>, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if ! fields.is_empty () {
			fail! (0x6007924a);
		}
		vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ())
	} else {
		vec! []
	};
	return record_new (kind, values, immutable);
}

pub fn record_build_1 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 1 {
			fail! (0x03ce30be);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		r#try! (vec_set (&mut values, fields[0], value_1));
		values
	} else {
		vec! [value_1.clone ()]
	};
	return record_new (kind, values, immutable);
}

pub fn record_build_2 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 2 {
			fail! (0x34c1df40);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		r#try! (vec_set (&mut values, fields[0], value_1));
		r#try! (vec_set (&mut values, fields[1], value_2));
		values
	} else {
		vec! [value_1.clone (), value_2.clone ()]
	};
	return record_new (kind, values, immutable);
}

pub fn record_build_3 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 3 {
			fail! (0xa678b9a9);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		r#try! (vec_set (&mut values, fields[0], value_1));
		r#try! (vec_set (&mut values, fields[1], value_2));
		r#try! (vec_set (&mut values, fields[2], value_3));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone ()]
	};
	return record_new (kind, values, immutable);
}

pub fn record_build_4 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 4 {
			fail! (0xef4b85b7);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		r#try! (vec_set (&mut values, fields[0], value_1));
		r#try! (vec_set (&mut values, fields[1], value_2));
		r#try! (vec_set (&mut values, fields[2], value_3));
		r#try! (vec_set (&mut values, fields[3], value_4));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone (), value_4.clone ()]
	};
	return record_new (kind, values, immutable);
}

pub fn record_build_n <ValueRef : StdAsRef<Value>> (kind : &RecordKind, fields : Option<&[usize]>, values : &[ValueRef], immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != values.len () {
			fail! (0x9c52a9f1);
		}
		let mut values_ = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		for index in 0 .. fields.len () {
			r#try! (vec_set_ref (&mut values_, fields[index], &values[index]));
		}
		values_
	} else {
		vec_clone_slice_ref (values)
	};
	return record_new (kind, values, immutable);
}




pub fn record_resolve_field_index (kind : Option<&RecordKind>, field : &Value, record : &Value) -> (Outcome<usize>) {
	let (kind, _record) = r#try! (record_as_ref (kind, record));
	return record_kind_resolve_field_index (kind, field);
}




pub fn record_get (kind : Option<&RecordKind>, field : usize, record : &Value) -> (Outcome<Value>) {
	let (kind, record) = r#try! (record_ref (kind, record));
	let field = try_some! (kind.field_by_index (field), 0x68689806);
	let record = record.values_as_slice ();
	let value = try_some_or_panic! (record.get (field.index), 0xcce25bab);
	let value = value.clone ();
	succeed! (value);
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn record_set (kind : Option<&RecordKind>, field : usize, record : &Value, value : &Value) -> (Outcome<Value>) {
	let (kind, record) = r#try! (record_mutable_as_ref (kind, record));
	let field = try_some! (kind.field_by_index (field), 0x42baf564);
	if ! field.mutable {
		fail! (0xbe7a850f);
	}
	let mut record = r#try! (record.values_ref_mut ());
	let value_ref = try_some_or_panic! (record.get_mut (field.index), 0x8b20ee6e);
	let mut value_swap = value.clone ();
	mem::swap (&mut value_swap, value_ref);
	succeed! (value_swap);
}




pub fn record_get_x (kind : Option<&RecordKind>, field : &Value, record : &Value) -> (Outcome<Value>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_get (kind, r#try! (field.try_to_usize ()), record),
		_ => {
			let field = r#try! (record_resolve_field_index (kind, field, record));
			return record_get (None, field, record);
		},
	}
}


#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn record_set_x (kind : Option<&RecordKind>, field : &Value, record : &Value, value : &Value) -> (Outcome<Value>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_set (kind, r#try! (field.try_to_usize ()), record, value),
		_ => {
			let field = r#try! (record_resolve_field_index (kind, field, record));
			return record_set (None, field, record, value);
		},
	}
}




#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub fn record_to_array (kind : Option<&RecordKind>, record : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let (_kind, record) = r#try! (record_as_ref (kind, record));
	let values = r#try! (record.values_rc_clone ());
	succeed! (array_from_rc (values, immutable));
}

#[ cfg ( feature = "vonuvoli_values_values" ) ]
pub fn record_to_values (kind : Option<&RecordKind>, record : &Value) -> (Outcome<Value>) {
	let (_kind, record) = r#try! (record_as_ref (kind, record));
	let values = r#try! (record.values_rc_clone ());
	succeed! (values_from_rc (values) .into ());
}

pub fn record_to_list (kind : Option<&RecordKind>, record : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let (_kind, record) = r#try! (record_ref (kind, record));
	let values = record.values_as_slice ();
	let values = list_build_n (values, None, immutable);
	succeed! (values);
}

pub fn record_to_assoc (_kind : Option<&RecordKind>, _record : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0xd7b46e66, (github_issue, 41));
}


#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub fn record_from_array (kind : Option<&RecordKind>, values : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let kind = try_some! (kind, 0x6bf5ff36);
	let values = try_as_array_as_ref! (values);
	let values = r#try! (values.values_rc_clone ());
	let record = r#try! (record_from_rc (kind, values, immutable));
	succeed! (record);
}

#[ cfg ( feature = "vonuvoli_values_values" ) ]
pub fn record_from_values (kind : Option<&RecordKind>, values : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let kind = try_some! (kind, 0x2555c3b4);
	let values = try_as_values_ref! (values);
	let values = values.values_rc_clone ();
	let record = r#try! (record_from_rc (kind, values, immutable));
	succeed! (record);
}

pub fn record_from_list (kind : Option<&RecordKind>, values : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let kind = try_some! (kind, 0xe3499059);
	let values = r#try! (vec_list_clone (values));
	let record = r#try! (record_new (kind, values, immutable));
	succeed! (record);
}

pub fn record_from_assoc (_kind : Option<&RecordKind>, _values : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x8ccef2c5, (github_issue, 41));
}




pub fn record_kind_is_fn (kind : &RecordKind, immutable : Option<bool>) -> (ProcedureExtended) {
	return ProcedureExtendedInternals::RecordKindIs (kind.clone (), immutable) .into ();
}




pub fn record_build_fn_n (kind : &RecordKind, fields : Option<&Value>, immutable : Option<bool>) -> (Outcome<ProcedureExtended>) {
	return record_build_fn (kind, fields, immutable, true);
}

pub fn record_build_fn_c (kind : &RecordKind, fields : Option<&Value>, immutable : Option<bool>) -> (Outcome<ProcedureExtended>) {
	return record_build_fn (kind, fields, immutable, false);
}

fn record_build_fn (kind : &RecordKind, fields : Option<&Value>, immutable : Option<bool>, variadric : bool) -> (Outcome<ProcedureExtended>) {
	let fields = if let Some (fields) = fields {
		let fields = r#try! (record_kind_resolve_field_indices (kind, fields));
		if fields.is_some () {
			fields
		} else {
			Some (StdVec::new () .into_boxed_slice ())
		}
	} else {
		None
	};
	let kind = kind.clone ();
	if variadric {
		succeed! (ProcedureExtendedInternals::RecordBuildN (kind, fields, immutable) .into ());
	} else {
		succeed! (ProcedureExtendedInternals::RecordBuildC (kind, fields, immutable) .into ());
	}
}




pub fn record_get_fn (kind : Option<&RecordKind>, field : usize) -> (Outcome<ProcedureExtended>) {
	if let Some (kind) = kind {
		if field >= kind.values_count () {
			fail! (0x56ee989d);
		}
	}
	let kind = option_map! (kind, kind.clone ());
	succeed! (ProcedureExtendedInternals::RecordGet (kind, field) .into ());
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn record_set_fn (kind : Option<&RecordKind>, field : usize) -> (Outcome<ProcedureExtended>) {
	if let Some (kind) = kind {
		if field >= kind.values_count () {
			fail! (0x4747b115);
		}
	}
	let kind = option_map! (kind, kind.clone ());
	succeed! (ProcedureExtendedInternals::RecordSet (kind, field) .into ());
}


pub fn record_get_x_fn (kind : Option<&RecordKind>, field : &Value) -> (Outcome<ProcedureExtended>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_get_fn (kind, r#try! (field.try_to_usize ())),
		_ => {
			let kind = option_map! (kind, kind.clone ());
			succeed! (ProcedureExtendedInternals::RecordGetX (kind, field.clone ()) .into ());
		},
	}
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn record_set_x_fn (kind : Option<&RecordKind>, field : &Value) -> (Outcome<ProcedureExtended>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_set_fn (kind, r#try! (field.try_to_usize ())),
		_ => {
			let kind = option_map! (kind, kind.clone ());
			succeed! (ProcedureExtendedInternals::RecordSetX (kind, field.clone ()) .into ());
		},
	}
}




pub fn record_ref <'a> (kind : Option<&'a RecordKind>, record : &'a Value) -> (Outcome<(&'a RecordKind, RecordRef<'a>)>) {
	let record = try_as_record_ref! (record);
	let kind = if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0xe5012bde);
		}
		kind
	} else {
		unsafe { mem::transmute (record.kind ()) }
	};
	succeed! ((kind, record));
}

pub fn record_as_ref <'a> (kind : Option<&'a RecordKind>, record : &'a Value) -> (Outcome<(&'a RecordKind, RecordAsRef<'a>)>) {
	let record = try_as_record_as_ref! (record);
	let kind = if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0x9d886165);
		}
		kind
	} else {
		unsafe { mem::transmute (record.kind ()) }
	};
	succeed! ((kind, record));
}

pub fn record_immutable_as_ref <'a> (kind : Option<&'a RecordKind>, record : &'a Value) -> (Outcome<(&'a RecordKind, &'a RecordImmutable)>) {
	let record = try_as_record_immutable_ref! (record);
	let kind = if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0x03f257e3);
		}
		kind
	} else {
		record.kind ()
	};
	succeed! ((kind, record));
}

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub fn record_mutable_as_ref <'a> (kind : Option<&'a RecordKind>, record : &'a Value) -> (Outcome<(&'a RecordKind, &'a RecordMutable)>) {
	let record = try_as_record_mutable_ref! (record);
	let kind = if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0x6459a0fd);
		}
		kind
	} else {
		record.kind ()
	};
	succeed! ((kind, record));
}

