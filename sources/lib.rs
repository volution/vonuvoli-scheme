

#![ feature (core) ]
#![ feature (libc) ]
#![ feature (test) ]

#![ feature (i128_type) ]
#![ feature (try_from) ]
#![ feature (ascii_ctype) ]
#![ feature (str_internals) ]
#![ feature (char_from_unchecked) ]
#![ feature (slice_get_slice) ]
#![ feature (const_fn) ]
#![ feature (stmt_expr_attributes) ]
#![ feature (type_ascription) ]
#![ feature (vec_resize_default) ]

#![ no_implicit_prelude ]




// NOTE:  Treat all warnings as errors!
//        (Except those explicitly `allow`-ed.)
#![ deny (warnings) ]

// NOTE:  Temporarely `allow` these lints!
#![ allow (dead_code) ]
#![ allow (unused_macros) ]

// NOTE:  These lints are `allow`-ed by default.
//        Toggle `warn` for some of them!
#![ warn (anonymous_parameters) ]
#![ allow (box_pointers) ]
#![ allow (missing_copy_implementations) ]
#![ allow (missing_debug_implementations) ]
#![ warn (trivial_casts) ]
#![ warn (trivial_numeric_casts) ]
#![ warn (unreachable_pub) ]
#![ warn (unused_extern_crates) ]
#![ allow (unused_import_braces) ]
#![ warn (unused_qualifications) ]
#![ allow (variant_size_differences) ]

// NOTE:  These lints are `allow`-ed by default.
#![ allow (missing_docs) ]
#![ allow (unsafe_code) ]




pub extern crate core;
pub extern crate libc;
pub extern crate test;




#[ cfg ( feature = "vonuvoli_backtrace" ) ]
pub extern crate backtrace;
#[ cfg ( feature = "vonuvoli_backtrace" ) ]
pub extern crate rustc_demangle;

#[ cfg ( feature = "vonuvoli_terminal" ) ]
pub extern crate ansi_term;
#[ cfg ( feature = "vonuvoli_terminal" ) ]
pub extern crate atty;

#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate ring;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate digest;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate sha1;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate sha2;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate sha3;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate md5;
#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub extern crate blake2;

#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
pub extern crate rand;

#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
pub extern crate data_encoding;




pub mod externals {
	
	pub use core;
	pub use libc;
	pub use test;
	
	#[ cfg ( feature = "vonuvoli_backtrace" ) ]
	pub use {
		backtrace,
		rustc_demangle,
	};
	
	#[ cfg ( feature = "vonuvoli_terminal" ) ]
	pub use {
		ansi_term,
		atty,
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
	pub use {
		ring,
		digest,
		sha1,
		sha2,
		sha3,
		md5,
		blake2,
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
	pub use {
		rand,
	};
	
	#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
	pub use {
		data_encoding,
	};
	
}




include! ("macros.in");




pub(crate) mod builtins;
pub(crate) mod builtins_arrays;
pub(crate) mod builtins_bytes;
pub(crate) mod builtins_comparisons;
pub(crate) mod builtins_functions;
pub(crate) mod builtins_lists;
pub(crate) mod builtins_ports;
pub(crate) mod builtins_processes;
pub(crate) mod builtins_records;
pub(crate) mod builtins_runtime;
pub(crate) mod builtins_strings;
pub(crate) mod builtins_types;
pub(crate) mod compiler;
pub(crate) mod compiler_optimizer;
pub(crate) mod constants;
pub(crate) mod contexts;
pub(crate) mod conversions;
pub(crate) mod counters;
pub(crate) mod display;
pub(crate) mod errors;
pub(crate) mod evaluator;
pub(crate) mod expressions;
pub(crate) mod extended_procedures;
pub(crate) mod extended_syntaxes;
pub(crate) mod globals;
pub(crate) mod hashes;
pub(crate) mod lambdas;
pub(crate) mod languages;
pub(crate) mod languages_builtins;
pub(crate) mod languages_r7rs;
pub(crate) mod native_procedures;
pub(crate) mod native_syntaxes;
pub(crate) mod ordering;
pub(crate) mod parameters;
pub(crate) mod parser;
pub(crate) mod ports;
pub(crate) mod ports_memory;
pub(crate) mod ports_native;
pub(crate) mod primitives;
pub(crate) mod primitives_arithmetic;
pub(crate) mod primitives_arrays;
pub(crate) mod primitives_bitwise;
pub(crate) mod primitives_boolean;
pub(crate) mod primitives_bytes;
pub(crate) mod primitives_comparisons;
pub(crate) mod primitives_functions;
pub(crate) mod primitives_lists;
pub(crate) mod primitives_ports;
pub(crate) mod primitives_procedures;
pub(crate) mod primitives_records;
pub(crate) mod primitives_runtime;
pub(crate) mod primitives_strings;
pub(crate) mod primitives_syntaxes;
pub(crate) mod primitives_types;
pub(crate) mod processes;
pub(crate) mod runtime;
pub(crate) mod runtime_backtrace;
pub(crate) mod runtime_configurations;
pub(crate) mod runtime_iterators;
pub(crate) mod runtime_unicode;
pub(crate) mod tests;
pub(crate) mod values;
pub(crate) mod values_arrays;
pub(crate) mod values_booleans;
pub(crate) mod values_bytes;
pub(crate) mod values_characters;
pub(crate) mod values_keywords;
pub(crate) mod values_numbers;
pub(crate) mod values_pairs;
pub(crate) mod values_records;
pub(crate) mod values_strings;
pub(crate) mod values_symbols;
pub(crate) mod values_unique;
pub(crate) mod values_value;
pub(crate) mod values_values;


// NOTE:  This module is generated thus we can't easily change its members visibility...
#[ allow (unreachable_pub) ]
pub(crate) mod parser_peg;


#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub(crate) mod builtins_crypto;

#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
pub(crate) mod builtins_random;

#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
pub(crate) mod builtins_encoding;




pub mod prelude;




pub mod internals {
	
	pub use super::builtins::exports as builtins;
	pub use super::builtins_arrays::exports as builtins_arrays;
	pub use super::builtins_bytes::exports as builtins_bytes;
	pub use super::builtins_comparisons::exports as builtins_comparisons;
	pub use super::builtins_functions::exports as builtins_functions;
	pub use super::builtins_lists::exports as builtins_lists;
	pub use super::builtins_ports::exports as builtins_ports;
	pub use super::builtins_processes::exports as builtins_processes;
	pub use super::builtins_records::exports as builtins_records;
	pub use super::builtins_runtime::exports as builtins_runtime;
	pub use super::builtins_strings::exports as builtins_strings;
	pub use super::builtins_types::exports as builtins_types;
	pub use super::compiler::exports as compiler;
	pub use super::compiler_optimizer::exports as compiler_optimizer;
	pub use super::constants::exports as constants;
	pub use super::contexts::exports as contexts;
	pub use super::conversions::exports as conversions;
	pub use super::errors::exports as errors;
	pub use super::evaluator::exports as evaluator;
	pub use super::expressions::exports as expressions;
	pub use super::extended_procedures::exports as extended_procedures;
	pub use super::extended_syntaxes::exports as extended_syntaxes;
	pub use super::lambdas::exports as lambdas;
	pub use super::languages::exports as languages;
	pub use super::languages_builtins::exports as languages_builtins;
	pub use super::languages_r7rs::exports as languages_r7rs;
	pub use super::native_procedures::exports as native_procedures;
	pub use super::native_syntaxes::exports as native_syntaxes;
	pub use super::parameters::exports as parameters;
	pub use super::parser::exports as parser;
	pub use super::ports::exports as ports;
	pub use super::ports_memory::exports as ports_memory;
	pub use super::ports_native::exports as ports_native;
	pub use super::primitives::exports as primitives;
	pub use super::primitives_arithmetic::exports as primitives_arithmetic;
	pub use super::primitives_arrays::exports as primitives_arrays;
	pub use super::primitives_bitwise::exports as primitives_bitwise;
	pub use super::primitives_boolean::exports as primitives_boolean;
	pub use super::primitives_bytes::exports as primitives_bytes;
	pub use super::primitives_comparisons::exports as primitives_comparisons;
	pub use super::primitives_functions::exports as primitives_functions;
	pub use super::primitives_lists::exports as primitives_lists;
	pub use super::primitives_ports::exports as primitives_ports;
	pub use super::primitives_procedures::exports as primitives_procedures;
	pub use super::primitives_records::exports as primitives_records;
	pub use super::primitives_runtime::exports as primitives_runtime;
	pub use super::primitives_strings::exports as primitives_strings;
	pub use super::primitives_syntaxes::exports as primitives_syntaxes;
	pub use super::primitives_types::exports as primitives_types;
	pub use super::processes::exports as processes;
	pub use super::runtime::exports as runtime;
	pub use super::runtime_backtrace::exports as runtime_backtrace;
	pub use super::runtime_configurations::exports as runtime_configurations;
	pub use super::runtime_iterators::exports as runtime_iterators;
	pub use super::runtime_unicode::exports as runtime_unicode;
	pub use super::tests::exports as tests;
	pub use super::values::exports as values;
	pub use super::values_arrays::exports as values_arrays;
	pub use super::values_booleans::exports as values_booleans;
	pub use super::values_bytes::exports as values_bytes;
	pub use super::values_characters::exports as values_characters;
	pub use super::values_numbers::exports as values_numbers;
	pub use super::values_keywords::exports as values_keywords;
	pub use super::values_pairs::exports as values_pairs;
	pub use super::values_records::exports as values_records;
	pub use super::values_strings::exports as values_strings;
	pub use super::values_symbols::exports as values_symbols;
	pub use super::values_unique::exports as values_unique;
	pub use super::values_value::exports as values_value;
	pub use super::values_values::exports as values_values;
	
	#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
	pub use super::builtins_crypto::exports as builtins_crypto;
	
	#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
	pub use super::builtins_random::exports as builtins_random;
	
	#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
	pub use super::builtins_encoding::exports as builtins_encoding;
	
}




pub mod exports {
	
	pub use super::builtins::exports::*;
	pub use super::compiler::exports::*;
	pub use super::compiler_optimizer::exports::*;
	pub use super::constants::exports::*;
	pub use super::contexts::exports::*;
	pub use super::conversions::exports::*;
	pub use super::errors::exports::*;
	pub use super::evaluator::exports::*;
	pub use super::expressions::exports::*;
	pub use super::extended_procedures::exports::*;
	pub use super::extended_syntaxes::exports::*;
	pub use super::lambdas::exports::*;
	pub use super::languages::exports::*;
	pub use super::native_procedures::exports::*;
	pub use super::native_syntaxes::exports::*;
	pub use super::parameters::exports::*;
	pub use super::parser::exports::*;
	pub use super::ports::exports::*;
	pub use super::primitives::exports::*;
	pub use super::processes::exports::*;
	pub use super::runtime::exports::*;
	pub use super::tests::exports::*;
	pub use super::values::exports::*;
	
}

