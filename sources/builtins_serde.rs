

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;
use super::externals::serde_bytes;

use super::prelude::*;




pub mod exports {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
			serde_serialize_into_bytes,
			serde_deserialize_from_bytes,
		};
	
	pub use super::{
			serde_serialize_into_buffer,
			serde_deserialize_from_buffer,
		};
	
	pub use super::{
			
			serde_value_to_ast,
			serde_ast_to_value,
			ValueSerde,
			
		};
	
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_serialize_into_bytes (value : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let buffer = try! (serde_serialize_into_buffer (value));
	let buffer = StdVec::from (buffer);
	succeed! (bytes_new (buffer, immutable));
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_deserialize_from_bytes (bytes : &Value) -> (Outcome<Value>) {
	let buffer = try_as_bytes_ref! (bytes);
	let buffer = buffer.bytes_as_slice ();
	let value = try! (serde_deserialize_from_buffer (buffer));
	succeed! (value);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_serialize_into_buffer (value : &Value) -> (Outcome<StdBox<[u8]>>) {
	let config = serde_bincode_config ();
	let value = try! (serde_value_to_ast (value));
	let buffer = try_or_fail! (config.serialize (&value), 0xb1d0c20c);
	succeed! (buffer.into_boxed_slice ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_deserialize_from_buffer (buffer : &[u8]) -> (Outcome<Value>) {
	let config = serde_bincode_config ();
	let value = try_or_fail! (config.deserialize (buffer), 0x9664c785);
	let value = try! (serde_ast_to_value (value));
	succeed! (value);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn serde_bincode_config () -> (ext::bincode::Config) {
	let mut config = ext::bincode::config ();
	config.no_limit ();
	config.native_endian ();
	return config;
}




FIXME! ("explicitly implement the required methods to serialize and deserialize values, without resorting to building a new value tree");




#[ derive ( Serialize, Deserialize ) ] // OK
pub enum ValueSerde {
	
	Null,
	Void,
	Undefined,
	
	Boolean (bool),
	NumberInteger (i64),
	NumberReal (f64),
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	Character (char),
	
	Symbol (StdString),
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	Keyword (StdString),
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	String (StdString),
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ serde ( with = "serde_bytes" ) ]
	Bytes (StdVec<u8>),
	
	Pair (StdBox<ValueSerde>, StdBox<ValueSerde>),
	List (StdVec<ValueSerde>, Option<StdBox<ValueSerde>>),
	
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	Array (StdVec<ValueSerde>),
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	Values (StdVec<ValueSerde>),
	
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	#[ serde ( with = "serde_bytes" ) ]
	Path (StdVec<u8>),
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_value_to_ast (value : &Value) -> (Outcome<ValueSerde>) {
	match value.class_match_as_ref () {
		
		ValueClassMatchAsRef::Null =>
			succeed! (ValueSerde::Null),
		ValueClassMatchAsRef::Void =>
			succeed! (ValueSerde::Void),
		ValueClassMatchAsRef::Undefined =>
			succeed! (ValueSerde::Undefined),
		ValueClassMatchAsRef::Singleton (_) =>
			fail! (0xab1da9f4),
		
		ValueClassMatchAsRef::Boolean (value) =>
			succeed! (ValueSerde::Boolean (value.value ())),
		ValueClassMatchAsRef::Number (class) =>
			match class {
				NumberMatchAsRef::Integer (value) =>
					succeed! (ValueSerde::NumberInteger (value.value ())),
				NumberMatchAsRef::Real (value) =>
					succeed! (ValueSerde::NumberReal (value.value ())),
			},
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::Character (value) =>
			succeed! (ValueSerde::Character (value.value ())),
		
		ValueClassMatchAsRef::Symbol (value) =>
			succeed! (ValueSerde::Symbol (StdString::from (value.string_as_str ()))),
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueClassMatchAsRef::Keyword (value) =>
			succeed! (ValueSerde::Keyword (StdString::from (value.string_as_str ()))),
		#[ cfg ( feature = "vonuvoli_values_unique" ) ]
		ValueClassMatchAsRef::Unique (_) =>
			fail! (0x4ffcf723),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::String (class) => {
			let value = try! (class.string_ref ());
			succeed! (ValueSerde::String (value.string_clone ()));
		},
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (class) => {
			let value = try! (class.bytes_ref ());
			succeed! (ValueSerde::Bytes (value.bytes_clone ()));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueClassMatchAsRef::StringRegex (_) =>
			fail! (0xfd407187),
		#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::BytesRegex (_) =>
			fail! (0xeddee53b),
		
		ValueClassMatchAsRef::Pair (class) => {
			{
				let values = try! (class.pair_ref ());
				let (left, right) = values.left_and_right ();
				if ! right.is_class (ValueClass::Pair) {
					let left = try! (serde_value_to_ast (left));
					let right = try! (serde_value_to_ast (right));
					succeed! (ValueSerde::Pair (StdBox::new (left), StdBox::new (right)));
				}
			}
			{
				let (values, dotted) = try! (vec_list_ref_clone_dotted (value));
				let values = try_vec_map! (values.iter (), value, serde_value_to_ast (value.deref ()));
				let dotted = try_option_map! (dotted, serde_value_to_ast (dotted.deref ()));
				let dotted = option_map! (dotted, StdBox::new (dotted));
				succeed! (ValueSerde::List (values, dotted));
			}
		},
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueClassMatchAsRef::Array (class) => {
			let values = try! (class.array_ref ());
			let values = values.values_as_slice ();
			let values = try_vec_map! (values.iter (), value, serde_value_to_ast (value));
			succeed! (ValueSerde::Array (values));
		},
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueClassMatchAsRef::Values (values) => {
			let values = values.values_as_slice ();
			let values = try_vec_map! (values.iter (), value, serde_value_to_ast (value));
			succeed! (ValueSerde::Values (values));
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::RecordKind (_) =>
			fail! (0x26b93d27),
		#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
		ValueClassMatchAsRef::Record (_) =>
			fail! (0x807f1386),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		ValueClassMatchAsRef::Error (_) =>
			fail! (0xf88d3f27),
		
		ValueClassMatchAsRef::Procedure (_) =>
			fail! (0xfa909bd0),
		ValueClassMatchAsRef::Syntax (_) =>
			fail! (0xf4a1a452),
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueClassMatchAsRef::Path (value) => {
			let value = value.path_ref ();
			let value = value.to_path_buf () .into_os_string () .into_vec ();
			succeed! (ValueSerde::Path (value));
		},
		#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
		ValueClassMatchAsRef::Port (_) =>
			fail! (0xa3e18d57),
		ValueClassMatchAsRef::Resource (_) =>
			fail! (0x30d2e523),
		
		#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
		ValueClassMatchAsRef::Opaque (_) =>
			fail! (0x6f8a3ef1),
		
		ValueClassMatchAsRef::Internal (_) =>
			fail! (0x7aedddfa),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn serde_ast_to_value (value : ValueSerde) -> (Outcome<Value>) {
	match value {
		
		ValueSerde::Null =>
			succeed! (NULL_VALUE),
		ValueSerde::Void =>
			succeed! (VOID_VALUE),
		ValueSerde::Undefined =>
			succeed! (UNDEFINED_VALUE),
		
		ValueSerde::Boolean (value) =>
			succeed! (value.into ()),
		ValueSerde::NumberInteger (value) =>
			succeed! (value.into ()),
		ValueSerde::NumberReal (value) =>
			succeed! (value.into ()),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueSerde::Character (value) =>
			succeed! (value.into ()),
		
		ValueSerde::Symbol (value) =>
			succeed! (symbol_new (value) .into ()),
		#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
		ValueSerde::Keyword (value) =>
			succeed! (keyword_new (value) .into ()),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		ValueSerde::String (value) =>
			succeed! (string_new (value) .into ()),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueSerde::Bytes (value) =>
			succeed! (bytes_new (value, None) .into ()),
		
		ValueSerde::Pair (left, right) => {
			let left = try! (serde_ast_to_value (*left));
			let right = try! (serde_ast_to_value (*right));
			succeed! (pair_new (left, right, None) .into ());
		},
		ValueSerde::List (values, dotted) => {
			let values = try_vec_map_into! (values, value, serde_ast_to_value (value));
			let dotted = try_option_map! (dotted, serde_ast_to_value (*dotted));
			let list = list_collect_dotted (values, dotted, None);
			succeed! (list);
		},
		
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		ValueSerde::Array (values) => {
			let values = try_vec_map_into! (values, value, serde_ast_to_value (value));
			succeed! (array_new (values, None) .into ());
		},
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		ValueSerde::Values (values) => {
			let values = try_vec_map_into! (values, value, serde_ast_to_value (value));
			succeed! (values_new (values.into_boxed_slice ()) .into ());
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		ValueSerde::Path (value) => {
			let value = ffi::OsString::from_vec (value) .into ();
			succeed! (Path::new_from_raw (value, false) .into ());
		},
		
	}
}

