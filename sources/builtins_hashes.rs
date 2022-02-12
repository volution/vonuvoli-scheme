

use super::errors::exports::*;
use super::hashes::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			hash_value_with_default,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	pub use super::{
			hash_value_with_siphash_seeded,
			hash_value_with_siphash_unseeded,
			coerce_siphash_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_highwayhash" ) ]
	pub use super::{
			hash_value_with_highwayhash_seeded,
			hash_value_with_highwayhash_unseeded,
			coerce_highwayhash_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_xxh3" ) ]
	pub use super::{
			hash_value_with_xxh3_seeded,
			hash_value_with_xxh3_unseeded,
			coerce_xxh3_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	pub use super::{
			hash_value_with_seahash_seeded,
			hash_value_with_seahash_unseeded,
			coerce_seahash_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	pub use super::{
			hash_value_with_blake2b_seeded,
			hash_value_with_blake2b_unseeded,
			coerce_blake2b_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	pub use super::{
			hash_value_with_blake2s_seeded,
			hash_value_with_blake2s_unseeded,
			coerce_blake2s_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake3" ) ]
	pub use super::{
			hash_value_with_blake3_seeded,
			hash_value_with_blake3_unseeded,
			coerce_blake3_seed,
		};
	
	pub use super::super::hashes::{
			HashMode,
		};
	
}




pub fn hash_value_with_default <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	#[ cfg ( feature = "lazy_static" ) ]
	let hasher = hash::BuildHasher::build_hasher (RANDOM_STATE.deref ());
	#[ cfg ( not ( feature = "lazy_static" ) ) ]
	let hasher = hash_map::DefaultHasher::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "lazy_static" ) ]
lazy_static! {
	static ref RANDOM_STATE : hash_map::RandomState = hash_map::RandomState::new ();
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
pub fn hash_value_with_siphash_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&(u64, u64)>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = if let Some (seed) = seed {
		let &(seed_1, seed_2) = if let Some (seed) = seed {
			seed
		} else {
			SIPHASH_DEFAULT_SEED.deref ()
		};
		ext::siphasher::sip::SipHasher24::new_with_keys (seed_1, seed_2)
	} else {
		ext::siphasher::sip::SipHasher24::new ()
	};
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
pub fn hash_value_with_siphash_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = ext::siphasher::sip::SipHasher24::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
pub fn coerce_siphash_seed (value : &Value) -> (Outcome<Option<Option<(u64, u64)>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = r#try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0x63e3aa97),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac))));
				},
				16 => {
					let mut seed : [u8; 16] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : (u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some (seed)));
				},
				_ =>
					fail! (0xf3ec9555),
			}
		},
		_ =>
			fail! (0x05b4c4e3),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
lazy_static! {
	static ref SIPHASH_DEFAULT_SEED : (u64, u64) = {
			use super::externals::rand::RngCore;
			let mut generator = ext::rand::rngs::OsRng {};
			(generator.next_u64 (), generator.next_u64 ())
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_highwayhash" ) ]
pub fn hash_value_with_highwayhash_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&[u64; 4]>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let key = if let Some (seed) = seed {
		let seed = if let Some (seed) = seed {
			seed
		} else {
			HIGHWAYHASH_DEFAULT_SEED.deref ()
		};
		ext::highway::Key (*seed)
	} else {
		ext::highway::Key::default ()
	};
	let hasher = ext::highway::HighwayBuilder::new (key);
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_highwayhash" ) ]
pub fn hash_value_with_highwayhash_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let key = ext::highway::Key::default ();
	let hasher = ext::highway::HighwayBuilder::new (key);
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_highwayhash" ) ]
pub fn coerce_highwayhash_seed (value : &Value) -> (Outcome<Option<Option<[u64; 4]>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some ([seed, seed ^ 0x618ef69d0f85f1c1, seed ^ 0xea7a16cc4b1645d6, seed ^ 0x92bfb60bdeb589df])));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = r#try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0xaf0397f4),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ([seed, seed ^ 0x618ef69d0f85f1c1, seed ^ 0xea7a16cc4b1645d6, seed ^ 0x92bfb60bdeb589df])));
				},
				16 => {
					let mut seed : [u8; 16] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let (seed_1, seed_2) : (u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ([seed_1, seed_2, seed_1 ^ 0xd5d902be30c5c6f9, seed_2 ^ 0x8d5ecda2dde8637e])));
				},
				32 => {
					let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let (seed_1, seed_2, seed_3, seed_4) : (u64, u64, u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ([seed_1, seed_2, seed_3, seed_4])));
				},
				_ =>
					fail! (0xd9b7deb6),
			}
		},
		_ =>
			fail! (0x3bf4bdc7),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_highwayhash" ) ]
lazy_static! {
	static ref HIGHWAYHASH_DEFAULT_SEED : [u64; 4] = {
			use super::externals::rand::RngCore;
			let mut generator = ext::rand::rngs::OsRng {};
			[generator.next_u64 (), generator.next_u64 (), generator.next_u64 (), generator.next_u64 ()]
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_xxh3" ) ]
pub fn hash_value_with_xxh3_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&u64>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		let seed = if let Some (seed) = seed {
			seed
		} else {
			XXH3_DEFAULT_SEED.deref ()
		};
		*seed
	} else {
		0
	};
	let hasher = ext::twox_hash::xxh3::Hash64::with_seed (seed);
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_xxh3" ) ]
pub fn hash_value_with_xxh3_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = 0;
	let hasher = ext::twox_hash::xxh3::Hash64::with_seed (seed);
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_xxh3" ) ]
pub fn coerce_xxh3_seed (value : &Value) -> (Outcome<Option<Option<u64>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some (seed)));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = r#try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0x8a3bd16c),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some (seed)));
				},
				_ =>
					fail! (0x98e4f4b2),
			}
		},
		_ =>
			fail! (0x95989988),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_xxh3" ) ]
lazy_static! {
	static ref XXH3_DEFAULT_SEED : u64 = {
			use super::externals::rand::RngCore;
			let mut generator = ext::rand::rngs::OsRng {};
			generator.next_u64 ()
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
pub fn hash_value_with_seahash_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&(u64, u64, u64, u64)>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = if let Some (seed) = seed {
		let &(seed_1, seed_2, seed_3, seed_4) = if let Some (seed) = seed {
			seed
		} else {
			SEAHASH_DEFAULT_SEED.deref ()
		};
		ext::seahash::SeaHasher::with_seeds (seed_1, seed_2, seed_3, seed_4)
	} else {
		ext::seahash::SeaHasher::new ()
	};
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
pub fn hash_value_with_seahash_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = ext::seahash::SeaHasher::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
pub fn coerce_seahash_seed (value : &Value) -> (Outcome<Option<Option<(u64, u64, u64, u64)>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac, seed ^ 0x65e2b15736aae9df, seed ^ 0x9bc88f9627d32f76))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = r#try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0x4d9bb47d),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac, seed ^ 0x65e2b15736aae9df, seed ^ 0x9bc88f9627d32f76))));
				},
				32 => {
					let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : (u64, u64, u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some (seed)));
				},
				_ =>
					fail! (0xdb8aabed),
			}
		},
		_ =>
			fail! (0x3ce9d745),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
lazy_static! {
	static ref SEAHASH_DEFAULT_SEED : (u64, u64, u64, u64) = {
			use super::externals::rand::RngCore;
			let mut generator = ext::rand::rngs::OsRng {};
			(generator.next_u64 (), generator.next_u64 (), generator.next_u64 (), generator.next_u64 ())
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn hash_value_with_blake2b_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, seed : Option<Option<&[u8]>>, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		if let Some (seed) = seed {
			Some (seed)
		} else {
			let seed = BLAKE2B_DEFAULT_SEED.deref ();
			Some (&seed[..])
		}
	} else {
		None
	};
	return hash_value_with_blake2b (value, bits, seed, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn hash_value_with_blake2b_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	return hash_value_with_blake2b (value, bits, None, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn coerce_blake2b_seed (value : &Value) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	return coerce_blake_seed (value, 64);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
lazy_static! {
	static ref BLAKE2B_DEFAULT_SEED : [u8; 64] = {
			use super::externals::rand::RngCore;
			let mut seed : [u8; 64] = unsafe { mem::uninitialized () };
			let mut generator = ext::rand::rngs::OsRng {};
			generator.fill_bytes (&mut seed);
			seed
		};
}


#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn hash_value_with_blake2s_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, seed : Option<Option<&[u8]>>, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		if let Some (seed) = seed {
			Some (seed)
		} else {
			let seed = BLAKE2S_DEFAULT_SEED.deref ();
			Some (&seed[..])
		}
	} else {
		None
	};
	return hash_value_with_blake2s (value, bits, seed, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn hash_value_with_blake2s_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	return hash_value_with_blake2s (value, bits, None, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
pub fn coerce_blake2s_seed (value : &Value) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	return coerce_blake_seed (value, 32);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
lazy_static! {
	static ref BLAKE2S_DEFAULT_SEED : [u8; 32] = {
			use super::externals::rand::RngCore;
			let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
			let mut generator = ext::rand::rngs::OsRng {};
			generator.fill_bytes (&mut seed);
			seed
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_blake3" ) ]
pub fn hash_value_with_blake3_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, seed : Option<Option<&[u8]>>, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		if let Some (seed) = seed {
			Some (seed)
		} else {
			let seed = BLAKE3_DEFAULT_SEED.deref ();
			Some (&seed[..])
		}
	} else {
		None
	};
	return hash_value_with_blake3 (value, bits, seed, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake3" ) ]
pub fn hash_value_with_blake3_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	return hash_value_with_blake3 (value, bits, None, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake3" ) ]
pub fn coerce_blake3_seed (value : &Value) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	return coerce_blake_seed (value, 32);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake3" ) ]
lazy_static! {
	static ref BLAKE3_DEFAULT_SEED : [u8; 32] = {
			use super::externals::rand::RngCore;
			let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
			let mut generator = ext::rand::rngs::OsRng {};
			generator.fill_bytes (&mut seed);
			seed
		};
}




#[ cfg ( any ( feature = "vonuvoli_builtins_hashes_blake2", feature = "vonuvoli_builtins_hashes_blake3" ) ) ]
fn coerce_blake_seed (value : &Value, max_size : usize) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			if 8 > max_size {
				fail! (0x387b3d00);
			}
			let seed : &[u8; 8] = unsafe { mem::transmute (value) };
			succeed! (Some (Some (GenericRef::Immutable (seed))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = r#try! (value.bytes_ref ());
			{
				let bytes = bytes.bytes_as_slice ();
				if bytes.is_empty () {
					succeed! (None);
				}
				if bytes.len () > max_size {
					fail! (0xe1446d5a);
				}
			}
			succeed! (Some (Some (bytes.into_generic_ref ())));
		},
		_ =>
			fail! (0x1f1571ee),
	}
}




const DEFAULT_HASH_MODE : HashMode = HashMode::ValuesCoerceMutable;

