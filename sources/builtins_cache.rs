

use super::conversions::exports::*;
use super::errors::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::evaluator::exports::*;

#[ allow (unused_imports) ]
use super::hashes::exports::*;

#[ allow (unused_imports) ]
use super::constants::exports::*;

#[ allow (unused_imports) ]
use super::builtins::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			Cache,
			CacheInternals,
			CacheConfiguration,
		};
	
	pub use super::{
			cache_is,
			cache_open,
			cache_close,
		};
	
	pub use super::{
			cache_exclude_all,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	pub use super::{
			cache_select_serde,
			cache_include_serde,
			cache_exclude_serde,
			cache_resolve_serde,
		};
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::{
			cache_select_bytes,
			cache_include_bytes,
			cache_exclude_bytes,
			cache_resolve_bytes,
		};
	
}




pub struct Cache (StdRefCell<Option<CacheInternals>>, CacheConfiguration);


pub struct CacheInternals {
	environment : StdArc<ext::lmdb::Environment>,
	databases : StdMap<StdString, StdRc<ext::lmdb::Database<'static>>>,
}


pub struct CacheConfiguration {
	time_to_live : u64,
}




impl Cache {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref_mut (&self) -> (Outcome<StdRefMut<CacheInternals>>) {
		let self_0 = try_or_fail! (StdRefCell::try_borrow_mut (&self.0), 0x8e5e8d2b);
		if self_0.is_some () {
			let self_0 = StdRefMut::map (self_0, |self_0| try_some_or_panic! (self_0.as_mut (), 0xaa8792d7, github_issue_new));
			succeed! (self_0);
		} else {
			fail! (0xdf7af7bb);
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn configuration_ref (&self) -> (&CacheConfiguration) {
		&self.1
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn opaque_internals_ref_mut (cache : &Value) -> (Outcome<StdRefMut<CacheInternals>>) {
		let cache = try! (Cache::opaque_cast (cache));
		return cache.internals_ref_mut ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn opaque_configuration_ref (cache : &Value) -> (Outcome<&CacheConfiguration>) {
		let cache = try! (Cache::opaque_cast (cache));
		succeed! (&cache.1);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn opaque_internals_ref_mut_and_configuration_ref (cache : &Value) -> (Outcome<(StdRefMut<CacheInternals>, &CacheConfiguration)>) {
		let cache = try! (Cache::opaque_cast (cache));
		succeed! ((try! (cache.internals_ref_mut ()), &cache.1));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn opaque_cast (cache : &Value) -> (Outcome<&Cache>) {
		let cache = try_as_opaque_ref! (cache);
		let cache : &Cache = try_some! (cache.downcast (), 0x54e4fb93);
		succeed! (cache);
	}
}


impl CacheInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_database (&mut self, namespace : &str, create : bool) -> (Outcome<StdRc<ext::lmdb::Database<'static>>>) {
		
		FIXME! ("cache database for namespace");
		
		let environment = StdArc::clone (&self.environment);
		
		let mut flags = ext::lmdb::db::Flags::empty ();
		if create {
			flags.insert (ext::lmdb::db::CREATE);
		}
		let options = ext::lmdb::DatabaseOptions::new (flags);
		
		let database = try_or_fail! (ext::lmdb::Database::open (environment, Some (namespace), &options), 0x34052f19);
		let database = StdRc::new (database);
		
		succeed! (database);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn resolve_databases_all (&mut self) -> (Outcome<StdVec<StdRc<ext::lmdb::Database<'static>>>>) {
		
		FIXME! ("cache database for namespace");
		
		let flags = ext::lmdb::db::Flags::empty ();
		let options = ext::lmdb::DatabaseOptions::new (flags);
		
		let environment = StdArc::clone (&self.environment);
		let database_root = try_or_fail! (ext::lmdb::Database::open (environment, None, &options), 0x17873892);
		
		let transaction = try_or_fail! (ext::lmdb::ReadTransaction::new (database_root.env ()), 0x164658d2);
		let accessor = transaction.access ();
		let mut cursor = try_or_fail! (transaction.cursor (&database_root), 0x5b66867a);
		
		let mut databases = StdVec::new ();
		
		loop {
			match
					if databases.is_empty () {
						cursor.first::<str, [u8]> (&accessor)
					} else {
						cursor.next::<str, [u8]> (&accessor)
					}
			{
				Ok ((namespace, _)) => {
					let environment = StdArc::clone (&self.environment);
					let database = try_or_fail! (ext::lmdb::Database::open (environment, Some (namespace), &options), 0x4dca6249);
					let database = StdRc::new (database);
					databases.push (database);
				},
				Err (error) =>
					match error {
						ext::lmdb::error::Error::Code (ext::lmdb::error::NOTFOUND) =>
							break,
						_ =>
							fail! (0xb3d47279),
					},
			}
		}
		
		succeed! (databases);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_open (path : &Value, size : Option<&Value>, time_to_live : Option<&Value>, namespaces : Option<&Value>, accessors : Option<&Value>) -> (Outcome<Value>) {
	
	let path = try! (path_slice_coerce (path));
	let path = path.deref ();
	
	// NOTE:  The LMDB API expects `&str`, although any `&Path` should be accepted!
	let path = try_some! (path.to_str (), 0x1b90433e);
	
	let size = try! (count_coerce_option (size));
	let size = size.unwrap_or (CACHE_SIZE_DEFAULT);
	if size > CACHE_SIZE_MAXIMUM {
		fail! (0xa531096d);
	}
	let size = size * 1024 * 1024;
	
	let time_to_live = try! (count_coerce_option (time_to_live));
	let time_to_live = time_to_live.unwrap_or (CACHE_TIME_TO_LIVE_DEFAULT);
	if time_to_live > CACHE_TIME_TO_LIVE_MAXIMUM {
		fail! (0x82b32421);
	}
	
	let namespaces = try! (count_coerce_option (namespaces));
	let namespaces = namespaces.unwrap_or (CACHE_NAMESPACES_DEFAULT);
	if namespaces > CACHE_NAMESPACES_MAXIMUM {
		fail! (0x4b605dee);
	}
	
	let accessors = try! (count_coerce_option (accessors));
	let accessors = accessors.unwrap_or (CACHE_ACCESSORS_DEFAULT);
	if accessors > CACHE_ACCESSORS_MAXIMUM {
		fail! (0xd38fe877);
	}
	
	match fs::metadata (path) {
		Ok (metadata) =>
			if ! metadata.is_dir () {
				fail! (0xab4e523c);
			},
		Err (error) =>
			match error.raw_os_error () {
				Some (ext::libc::ENOENT) =>
					try_or_fail! (fs::create_dir (path), 0xbf8631d9),
				_ =>
					fail! (0xa646b5b3),
			},
	}
	
	let mut builder = try_or_fail! (ext::lmdb::EnvBuilder::new (), 0x70773c89);
	try_or_fail! (builder.set_mapsize (size), 0x0e3d3e8a);
	try_or_fail! (builder.set_maxdbs (namespaces as u32), 0x9fbfaae8);
	try_or_fail! (builder.set_maxreaders (accessors as u32), 0x1454d1ce);
	
	let mut flags = ext::lmdb::open::Flags::empty ();
	flags.insert (ext::lmdb::open::WRITEMAP);
	flags.insert (ext::lmdb::open::NOMETASYNC);
	flags.insert (ext::lmdb::open::NOTLS);
	
	let mode = CACHE_FILE_MODE;
	
	let environment = try_or_fail! (unsafe { builder.open (path, flags, mode) }, 0x339a13c9);
	let environment = StdArc::new (environment);
	
	let internals = CacheInternals {
			environment : environment,
			databases : StdMap::with_capacity (namespaces),
		};
	
	let configuration = CacheConfiguration {
			time_to_live : time_to_live as u64,
		};
	
	succeed! (opaque_new (Cache (StdRefCell::new (Some (internals)), configuration)) .into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_close (cache : &Value) -> (Outcome<()>) {
	
	let cache = try! (Cache::opaque_cast (cache));
	let mut cache = try_or_fail! (StdRefCell::try_borrow_mut (&cache.0), 0xff8da8cb);
	
	*cache = None;
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_is (value : &Value) -> (bool) {
	return Opaque::value_is::<Cache> (value);
}




#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_select_serde (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>) -> (Outcome<Value>) {
	
	let (_configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (hash_value_with_blake2b (key, CACHE_KEY_SIZE, None, HashMode::ValuesCoerceMutable));
	let key = key.deref ();
	
	let value = try! (cache_backend_select (database, key, |value| serde_deserialize_from_buffer (value)));
	let value = value.unwrap_or (FALSE_VALUE);
	
	succeed! (value);
}


#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_include_serde (cache : &Value, namespace : Option<&Value>, key : &Value, value : &Value, namespace_create : Option<bool>) -> (Outcome<()>) {
	
	let (configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (hash_value_with_blake2b (key, CACHE_KEY_SIZE, None, HashMode::ValuesCoerceMutable));
	let key = key.deref ();
	
	let value = try! (serde_serialize_into_buffer (value));
	let value = value.deref ();
	
	try! (cache_backend_include (database, key, value, configuration.time_to_live));
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_exclude_serde (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>) -> (Outcome<()>) {
	
	let (_configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (hash_value_with_blake2b (key, CACHE_KEY_SIZE, None, HashMode::ValuesCoerceMutable));
	let key = key.deref ();
	
	try! (cache_backend_exclude (database, key));
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_resolve_serde (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>, generator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let (configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key_value = key;
	let key = try! (hash_value_with_blake2b (key_value, CACHE_KEY_SIZE, None, HashMode::ValuesCoerceMutable));
	let key = key.deref ();
	
	{
		let value = try! (cache_backend_select (database, key, |value| serde_deserialize_from_buffer (value)));
		if let Some (value) = value {
			succeed! (value);
		}
	}
	
	let value_value = try! (evaluator.evaluate_procedure_call_1 (generator, key_value));
	
	{
		let value = try! (serde_serialize_into_buffer (&value_value));
		let value = value.deref ();
		
		try! (cache_backend_include (database, key, value, configuration.time_to_live));
	}
	
	succeed! (value_value);
}




#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_select_bytes (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>) -> (Outcome<Value>) {
	
	let (_configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (bytes_slice_coerce_1a (key));
	let key = key.deref ();
	let key = ext::blake2_rfc::blake2b::blake2b (CACHE_KEY_SIZE, &[], key);
	let key = key.as_bytes ();
	
	let value = try! (cache_backend_select (database, key, |value| succeed! (bytes_clone_slice (value))));
	let value = value.unwrap_or (FALSE_VALUE);
	
	succeed! (value);
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_include_bytes (cache : &Value, namespace : Option<&Value>, key : &Value, value : &Value, namespace_create : Option<bool>) -> (Outcome<()>) {
	
	let (configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (bytes_slice_coerce_1a (key));
	let key = key.deref ();
	let key = ext::blake2_rfc::blake2b::blake2b (CACHE_KEY_SIZE, &[], key);
	let key = key.as_bytes ();
	
	let value = try! (bytes_slice_coerce_1a (value));
	let value = value.deref ();
	
	try! (cache_backend_include (database, key, value, configuration.time_to_live));
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_exclude_bytes (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>) -> (Outcome<()>) {
	
	let (_configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key = try! (bytes_slice_coerce_1a (key));
	let key = key.deref ();
	let key = ext::blake2_rfc::blake2b::blake2b (CACHE_KEY_SIZE, &[], key);
	let key = key.as_bytes ();
	
	try! (cache_backend_exclude (database, key));
	
	succeed! (());
}


#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_resolve_bytes (cache : &Value, namespace : Option<&Value>, key : &Value, namespace_create : Option<bool>, generator : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let (configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
	let database = database.deref ();
	
	let key_value = key;
	let key = try! (bytes_slice_coerce_1a (key));
	let key = key.deref ();
	let key = ext::blake2_rfc::blake2b::blake2b (CACHE_KEY_SIZE, &[], key);
	let key = key.as_bytes ();
	
	{
		let value = try! (cache_backend_select (database, key, |value| succeed! (bytes_clone_slice (value))));
		if let Some (value) = value {
			succeed! (value);
		}
	}
	
	let value_value = try! (evaluator.evaluate_procedure_call_1 (generator, key_value));
	
	{
		let value = try_as_bytes_ref! (&value_value);
		let value = value.bytes_as_slice ();
		
		try! (cache_backend_include (database, key, value, configuration.time_to_live));
	}
	
	succeed! (value_value);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn cache_exclude_all (cache : &Value, namespace : Option<&Value>, namespace_create : Option<bool>) -> (Outcome<()>) {
	
	FIXME! ("if no namespace is specified clear all namespaces");
	
	if namespace.is_none () {
		
		let (_configuration, databases) = try! (cache_backend_resolve_databases_all (cache));
		for database in databases {
			let database = database.deref ();
			
			try! (cache_backend_exclude_all (database));
		}
		
	} else {
		
		let (_configuration, database) = try! (cache_backend_resolve_database (cache, namespace, namespace_create));
		let database = database.deref ();
		
		try! (cache_backend_exclude_all (database));
		
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_resolve_database <'a> (cache : &'a Value, namespace : Option<&Value>, namespace_create : Option<bool>) -> (Outcome<(&'a CacheConfiguration, StdRc<ext::lmdb::Database<'static>>)>) {
	
	let (mut cache, configuration) = try! (Cache::opaque_internals_ref_mut_and_configuration_ref (cache));
	
	let namespace = if let Some (namespace) = namespace {
		match namespace.kind_match_as_ref () {
			ValueKindMatchAsRef::Symbol (value) =>
				value.string_as_str (),
			ValueKindMatchAsRef::Boolean (value) =>
				if value.value () {
					CACHE_NAMESPACE_NAME_DEFAULT
				} else {
					fail! (0x1275e5e5);
				},
			_ =>
				fail! (0xf9933376),
		}
	} else {
		CACHE_NAMESPACE_NAME_DEFAULT
	};
	
	let namespace_create = namespace_create.unwrap_or (CACHE_NAMESPACE_CREATE_DEFAULT);
	
	let database = try! (cache.resolve_database (namespace, namespace_create));
	
	succeed! ((configuration, database));
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_resolve_databases_all (cache : &Value) -> (Outcome<(&CacheConfiguration, StdVec<StdRc<ext::lmdb::Database<'static>>>)>) {
	
	let (mut cache, configuration) = try! (Cache::opaque_internals_ref_mut_and_configuration_ref (cache));
	
	let databases = try! (cache.resolve_databases_all ());
	
	succeed! ((configuration, databases));
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_select <Decoder, Value> (database : &ext::lmdb::Database, key : &[u8], decoder : Decoder) -> (Outcome<Option<Value>>)
		where
			Decoder : FnOnce (&[u8]) -> (Outcome<Value>),
{
	
	let environment = database.env ();
	let transaction = try_or_fail! (ext::lmdb::ReadTransaction::new (environment), 0x5136c223);
	let accessor = transaction.access ();
	
	match accessor.get (database, key) {
		Ok (value) =>
			if let Some ((header, value)) = try! (cache_backend_record_unwrap (value, key, None)) {
				if header.is_fresh () {
					let value = try! (decoder (value));
					succeed! (Some (value));
				} else {
					succeed! (None);
				}
			} else {
				succeed! (None);
			},
		Err (error) =>
			match error {
				ext::lmdb::error::Error::Code (ext::lmdb::error::NOTFOUND) =>
					succeed! (None),
				_ =>
					fail! (0x18b3fffe),
			},
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_include (database : &ext::lmdb::Database, key : &[u8], value : &[u8], time_to_live : u64) -> (Outcome<()>) {
	
	let environment = database.env ();
	let transaction = try_or_fail! (ext::lmdb::WriteTransaction::new (environment), 0x30e3e16e);
	
	{
		let mut accessor = transaction.access ();
		let flags = ext::lmdb::put::Flags::empty ();
		
		let record_size = CACHE_HASH_SIZE + CACHE_HEADER_SIZE + value.len ();
		let record_data = try_or_fail! (unsafe { accessor.put_reserve_unsized (database, key, record_size, flags) }, 0xdaed26ce);
		
		let mut header = CacheRecordHeader::new (time_to_live);
		
		try! (cache_backend_record_wrap (&mut header, value, record_data, key, None));
	}
	
	try_or_fail! (transaction.commit (), 0xadfd7f01);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_exclude (database : &ext::lmdb::Database, key : &[u8]) -> (Outcome<()>) {
	
	let environment = database.env ();
	let transaction = try_or_fail! (ext::lmdb::WriteTransaction::new (environment), 0x6600ae94);
	
	{
		let mut accessor = transaction.access ();
		
		match accessor.del_key (database, key) {
			Ok (()) =>
				(),
			Err (error) =>
				match error {
					ext::lmdb::error::Error::Code (ext::lmdb::error::NOTFOUND) =>
						(),
					_ =>
						fail! (0xc7090246),
				},
		}
	}
	
	try_or_fail! (transaction.commit (), 0x365ce83f);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_exclude_all (database : &ext::lmdb::Database) -> (Outcome<()>) {
	
	let environment = database.env ();
	let transaction = try_or_fail! (ext::lmdb::WriteTransaction::new (environment), 0xc8e40f4d);
	
	{
		let mut accessor = transaction.access ();
		
		try_or_fail! (accessor.clear_db (database), 0x649382c6);
	}
	
	try_or_fail! (transaction.commit (), 0x518da901);
	
	succeed! (());
}




#[ derive ( Copy, Clone ) ]
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
// #[ repr (packed) ]
struct CacheRecordHeader {
	timestamp_created : u64,
	time_to_live : u64,
}


impl CacheRecordHeader {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn new (time_to_live : u64) -> (CacheRecordHeader) {
		let now = try_or_panic_0! (time::UNIX_EPOCH.elapsed (), 0xffe35099) .as_secs ();
		CacheRecordHeader {
				timestamp_created : now,
				time_to_live : time_to_live,
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_fresh (&self) -> (bool) {
		let now = try_or_panic_0! (time::UNIX_EPOCH.elapsed (), 0xffe35099) .as_secs ();
		(self.timestamp_created <= now) && ((self.timestamp_created + self.time_to_live) >= now)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn is_stale (&self) -> (bool) {
		return ! self.is_fresh ();
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_record_unwrap <'a> (record_data : &'a [u8], record_key : &[u8], hash_key : Option<&[u8]>) -> (Outcome<Option<(CacheRecordHeader, &'a [u8])>>) {
	
	if record_data.len () < (CACHE_HASH_SIZE + CACHE_HEADER_SIZE) {
		fail! (0xc4bab5d0);
	}
	
	let (record_hash, record_data) = record_data.split_at (CACHE_HASH_SIZE);
	
	{
		let hash_key = hash_key.unwrap_or (CACHE_HASH_KEY_DEFAULT);
		let hash_key = ext::blake2_rfc::blake2b::blake2b (CACHE_HASH_SIZE, hash_key, record_key);
		let hash_key = hash_key.as_bytes ();
		
		let hash = ext::blake2_rfc::blake2b::blake2b (CACHE_HASH_SIZE, hash_key, &record_data);
		let hash = hash.as_bytes ();
		
		if <[u8]>::ne (hash, record_hash) {
			succeed! (None);
		}
	}
	
	{
		let (record_header_data, record_value_data) = record_data.split_at (CACHE_HEADER_SIZE);
		let header = unsafe { mem::transmute::<_, &CacheRecordHeader> (record_header_data.as_ptr ()) } .clone ();
		
		succeed! (Some ((header, record_value_data)));
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn cache_backend_record_wrap <'a> (header : &CacheRecordHeader, value_data : &[u8], record_data : &mut [u8], record_key : &[u8], hash_key : Option<&[u8]>) -> (Outcome<()>) {
	
	if record_data.len () != (CACHE_HASH_SIZE + CACHE_HEADER_SIZE + value_data.len ()) {
		fail! (0x458ea65d);
	}
	
	let (record_hash, record_data) = record_data.split_at_mut (CACHE_HASH_SIZE);
	
	{
		let (record_header_data, record_value_data) = record_data.split_at_mut (CACHE_HEADER_SIZE);
		
		unsafe {
			let header : *const CacheRecordHeader = header;
			let header_data = slice::from_raw_parts (header as *const u8, CACHE_HEADER_SIZE);
			<[u8]>::copy_from_slice (record_header_data, header_data);
		}
		
		<[u8]>::copy_from_slice (record_value_data, value_data);
	}
	
	{
		let hash_key = hash_key.unwrap_or (CACHE_HASH_KEY_DEFAULT);
		let hash_key = ext::blake2_rfc::blake2b::blake2b (CACHE_HASH_SIZE, hash_key, record_key);
		let hash_key = hash_key.as_bytes ();
		
		let hash = ext::blake2_rfc::blake2b::blake2b (CACHE_HASH_SIZE, hash_key, record_data);
		let hash = hash.as_bytes ();
		
		<[u8]>::copy_from_slice (record_hash, hash);
		
		succeed! (());
	}
}




const CACHE_SIZE_DEFAULT : usize = 128;
const CACHE_SIZE_MAXIMUM : usize = 64 * 1024;

const CACHE_TIME_TO_LIVE_DEFAULT : usize = 6 * 60;
const CACHE_TIME_TO_LIVE_MAXIMUM : usize = 28 * 24 * 60 * 60;

const CACHE_NAMESPACES_DEFAULT : usize = 128;
const CACHE_NAMESPACES_MAXIMUM : usize = 1024;

const CACHE_NAMESPACE_NAME_DEFAULT : &'static str = "default";
const CACHE_NAMESPACE_CREATE_DEFAULT : bool = true;

const CACHE_ACCESSORS_DEFAULT : usize = 128;
const CACHE_ACCESSORS_MAXIMUM : usize = 1024;

const CACHE_FILE_MODE : ext::lmdb::FileMode = 0o600;

const CACHE_KEY_SIZE : usize = 256 / 8;

const CACHE_HASH_SIZE : usize = 256 / 8;
const CACHE_HASH_KEY_DEFAULT : &'static [u8] = super::runtime::exports::BUILD_KEY;
const CACHE_HEADER_SIZE : usize = mem::size_of::<CacheRecordHeader> ();

