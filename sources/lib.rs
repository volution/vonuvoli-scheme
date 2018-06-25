

#![ feature (core) ]
#![ feature (libc) ]
#![ feature (test) ]

#![ feature (try_from) ]
#![ feature (ascii_ctype) ]
#![ feature (str_internals) ]
#![ feature (char_from_unchecked) ]
#![ feature (slice_get_slice) ]
#![ feature (const_fn) ]
#![ feature (stmt_expr_attributes) ]
#![ feature (type_ascription) ]
#![ feature (vec_resize_default) ]
#![ feature (libstd_sys_internals) ]
#![ feature (never_type) ]
#![ feature (slice_patterns) ]
#![ feature (swap_with_slice) ]

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
#![ warn (bare_trait_objects) ]
#![ allow (box_pointers) ]
#![ allow (elided_lifetimes_in_paths) ]
#![ allow (missing_copy_implementations) ]
#![ allow (missing_debug_implementations) ]
#![ allow (missing_docs) ]
#![ allow (single_use_lifetimes) ]
#![ warn (trivial_casts) ]
#![ warn (trivial_numeric_casts) ]
#![ warn (unreachable_pub) ]
#![ warn (unused_extern_crates) ]
#![ allow (unused_import_braces) ]
#![ warn (unused_qualifications) ]
#![ allow (unused_results) ]
#![ warn (variant_size_differences) ]

// NOTE:  These lints are `warn`-ed by default.
//        Toggle `allow` for some of them!
#![ allow (unsafe_code) ]

// NOTE:  These lints are `allow`-ed when using conditional compilation!
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (unused_imports) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (unused_variables) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (unused_assignments) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (unreachable_code) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (unreachable_patterns) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (variant_size_differences) ) ]
#![ cfg_attr ( not ( feature = "vonuvoli_lints_warnings" ), allow (bare_trait_objects) ) ]

// NOTE:  These warnings are disabled in order to get the documentation built!
#![ allow (intra_doc_link_resolution_failure) ]




pub extern crate core;
pub extern crate libc;
pub extern crate test;




#[ cfg ( feature = "backtrace" ) ]
pub extern crate backtrace;
#[ cfg ( feature = "rustc-demangle" ) ]
pub extern crate rustc_demangle;

#[ cfg ( feature = "ansi_term" ) ]
pub extern crate ansi_term;
#[ cfg ( feature = "atty" ) ]
pub extern crate atty;

#[ cfg ( feature = "regex" ) ]
pub extern crate regex;

#[ cfg ( feature = "ring" ) ]
pub extern crate ring;
#[ cfg ( feature = "digest" ) ]
pub extern crate digest;
#[ cfg ( feature = "sha-1" ) ]
pub extern crate sha1;
#[ cfg ( feature = "sha2" ) ]
pub extern crate sha2;
#[ cfg ( feature = "sha3" ) ]
pub extern crate sha3;
#[ cfg ( feature = "md-5" ) ]
pub extern crate md5;
#[ cfg ( feature = "blake2" ) ]
pub extern crate blake2;

#[ cfg ( feature = "rand" ) ]
pub extern crate rand;

#[ cfg ( feature = "data-encoding" ) ]
pub extern crate data_encoding;

#[ cfg ( feature = "lmdb-zero" ) ]
pub extern crate lmdb_zero;

#[ cfg ( feature = "serde" ) ]
pub extern crate serde;

#[ cfg ( feature = "serde_derive" ) ]
#[ macro_use ]
pub extern crate serde_derive;

#[ cfg ( feature = "serde_bytes" ) ]
pub extern crate serde_bytes;

#[ cfg ( feature = "bincode" ) ]
pub extern crate bincode;

#[ cfg ( feature = "blake2-rfc" ) ]
pub extern crate blake2_rfc;

#[ cfg ( feature = "siphasher" ) ]
pub extern crate siphasher;
#[ cfg ( feature = "seahash" ) ]
pub extern crate seahash;

#[ cfg ( feature = "lazy_static" ) ]
#[ macro_use ]
pub extern crate lazy_static;

#[ cfg ( feature = "tempfile" ) ]
pub extern crate tempfile;




pub mod externals {
	
	pub use core;
	pub use libc;
	pub use test;
	
	#[ cfg ( feature = "backtrace" ) ]
	pub use backtrace;
	#[ cfg ( feature = "rustc-demangle" ) ]
	pub use rustc_demangle;
	
	#[ cfg ( feature = "ansi_term" ) ]
	pub use ansi_term;
	#[ cfg ( feature = "atty" ) ]
	pub use atty;
	
	#[ cfg ( feature = "regex" ) ]
	pub use regex;
	
	#[ cfg ( feature = "ring" ) ]
	pub use ring;
	#[ cfg ( feature = "digest" ) ]
	pub use digest;
	#[ cfg ( feature = "sha-1" ) ]
	pub use sha1;
	#[ cfg ( feature = "sha2" ) ]
	pub use sha2;
	#[ cfg ( feature = "sha3" ) ]
	pub use sha3;
	#[ cfg ( feature = "md-5" ) ]
	pub use md5;
	#[ cfg ( feature = "blake2" ) ]
	pub use blake2;
	
	#[ cfg ( feature = "rand" ) ]
	pub use rand;
	
	#[ cfg ( feature = "data-encoding" ) ]
	pub use data_encoding;
	
	#[ cfg ( feature = "lmdb-zero" ) ]
	pub use lmdb_zero as lmdb;
	
	#[ cfg ( feature = "serde" ) ]
	pub use serde;
	#[ cfg ( feature = "serde_derive" ) ]
	pub use serde_derive;
	#[ cfg ( feature = "serde_bytes" ) ]
	pub use serde_bytes;
	#[ cfg ( feature = "bincode" ) ]
	pub use bincode;
	
	#[ cfg ( feature = "blake2-rfc" ) ]
	pub use blake2_rfc;
	
	#[ cfg ( feature = "siphasher" ) ]
	pub use siphasher;
	#[ cfg ( feature = "seahash" ) ]
	pub use seahash;
	
	#[ cfg ( feature = "lazy_static" ) ]
	pub use lazy_static;
	
	#[ cfg ( feature = "tempfile" ) ]
	pub use tempfile;
	
}




include! ("macros.in");




pub(crate) mod builtins;
#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub(crate) mod builtins_arrays;
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
pub(crate) mod builtins_bytes;
#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
pub(crate) mod builtins_cache;
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
pub(crate) mod builtins_comparisons;
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
pub(crate) mod builtins_filesystem;
pub(crate) mod builtins_functions;
#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
pub(crate) mod builtins_hashes;
pub(crate) mod builtins_lists;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub(crate) mod builtins_ports;
#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
pub(crate) mod builtins_processes;
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
pub(crate) mod builtins_records;
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
pub(crate) mod builtins_regularex;
pub(crate) mod builtins_runtime;
#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
pub(crate) mod builtins_serde;
#[ cfg ( feature = "vonuvoli_values_string" ) ]
pub(crate) mod builtins_strings;
pub(crate) mod builtins_types;
#[ cfg ( feature = "vonuvoli_compiler" ) ]
pub(crate) mod compiler;
#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub(crate) mod compiler_optimizer;
pub(crate) mod constants;
pub(crate) mod contexts;
pub(crate) mod conversions;
pub(crate) mod counters;
pub(crate) mod display;
#[ cfg ( feature = "vonuvoli_documentation" ) ]
pub(crate) mod documentation;
pub(crate) mod errors;
#[ cfg ( feature = "vonuvoli_evaluator" ) ]
pub(crate) mod evaluator;
#[ cfg ( not ( feature = "vonuvoli_evaluator" ) ) ]
pub(crate) mod evaluator_trait;
#[ cfg ( not ( feature = "vonuvoli_evaluator" ) ) ]
pub(crate) use evaluator_trait as evaluator;
#[ cfg ( feature = "vonuvoli_expressions" ) ]
pub(crate) mod expressions;
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
pub(crate) mod extended_procedures;
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_extended" ) ]
pub(crate) mod extended_syntaxes;
pub(crate) mod globals;
#[ cfg ( feature = "vonuvoli_hash" ) ]
pub(crate) mod hashes;
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
pub(crate) mod lambdas;
pub(crate) mod libraries;
pub(crate) mod libraries_builtins;
pub(crate) mod libraries_r7rs;
#[ cfg ( feature = "vonuvoli_values_native" ) ]
pub(crate) mod native_procedures;
#[ cfg ( feature = "vonuvoli_expressions" ) ]
#[ cfg ( feature = "vonuvoli_compiler" ) ]
#[ cfg ( feature = "vonuvoli_values_native" ) ]
pub(crate) mod native_syntaxes;
#[ cfg ( feature = "vonuvoli_eqord" ) ]
pub(crate) mod ordering;
#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub(crate) mod parameters;
#[ cfg ( feature = "vonuvoli_parser" ) ]
pub(crate) mod parser;
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
pub(crate) mod paths;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub(crate) mod ports;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub(crate) mod ports_memory;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub(crate) mod ports_native;
pub(crate) mod primitives;
pub(crate) mod primitives_arithmetic;
#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub(crate) mod primitives_arrays;
pub(crate) mod primitives_bitwise;
pub(crate) mod primitives_boolean;
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
pub(crate) mod primitives_bytes;
#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
pub(crate) mod primitives_comparisons;
#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
pub(crate) mod primitives_filesystem;
pub(crate) mod primitives_functions;
pub(crate) mod primitives_lists;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub(crate) mod primitives_ports;
pub(crate) mod primitives_procedures;
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
pub(crate) mod primitives_records;
pub(crate) mod primitives_runtime;
#[ cfg ( feature = "vonuvoli_values_string" ) ]
pub(crate) mod primitives_strings;
pub(crate) mod primitives_syntaxes;
pub(crate) mod primitives_types;
#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
pub(crate) mod processes;
#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
pub(crate) mod regularex;
pub(crate) mod runtime;
pub(crate) mod runtime_backtrace;
pub(crate) mod runtime_configurations;
pub(crate) mod runtime_iterators;
pub(crate) mod runtime_unicode;
#[ cfg ( feature = "vonuvoli_tests" ) ]
pub(crate) mod tests;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub(crate) mod transcript;
pub(crate) mod values;
#[ cfg ( feature = "vonuvoli_values_array" ) ]
pub(crate) mod values_arrays;
pub(crate) mod values_booleans;
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
pub(crate) mod values_bytes;
#[ cfg ( feature = "vonuvoli_values_string" ) ]
pub(crate) mod values_characters;
#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
pub(crate) mod values_keywords;
pub(crate) mod values_numbers;
#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
pub(crate) mod values_opaque;
pub(crate) mod values_pairs;
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
pub(crate) mod values_records;
#[ cfg ( feature = "vonuvoli_values_string" ) ]
pub(crate) mod values_strings;
pub(crate) mod values_symbols;
#[ cfg ( any ( feature = "vonuvoli_values_unique", feature = "vonuvoli_builtins_parameters" ) ) ]
pub(crate) mod values_unique;
#[ cfg ( any ( feature = "vonuvoli_parser", feature = "vonuvoli_tests" ) ) ]
pub(crate) mod values_tests;
pub(crate) mod values_value;
#[ cfg ( feature = "vonuvoli_values_values" ) ]
pub(crate) mod values_values;


// NOTE:  This module is generated thus we can't easily change its members visibility...
#[ cfg ( feature = "vonuvoli_parser" ) ]
#[ allow (unreachable_pub) ]
pub(crate) mod parser_peg;


#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
pub(crate) mod builtins_crypto;

#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
pub(crate) mod builtins_random;

#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
pub(crate) mod builtins_encoding;


#[ cfg ( feature = "vonuvoli_tools_common" ) ]
pub(crate) mod tools;
#[ cfg ( feature = "vonuvoli_tools_common" ) ]
pub(crate) mod tools_common;
#[ cfg ( feature = "vonuvoli_tools_interpreter" ) ]
pub(crate) mod tools_interpreter;
#[ cfg ( feature = "vonuvoli_tools_compiler" ) ]
pub(crate) mod tools_compiler;
#[ cfg ( feature = "vonuvoli_tools_tester" ) ]
pub(crate) mod tools_tester;
#[ cfg ( feature = "vonuvoli_tools_bencher" ) ]
pub(crate) mod tools_bencher;
#[ cfg ( feature = "vonuvoli_tools_reports" ) ]
pub(crate) mod tools_reports;
#[ cfg ( feature = "vonuvoli_tools_documentation" ) ]
pub(crate) mod tools_documentation;




pub mod prelude;




pub mod internals {
	
	pub use super::builtins::exports as builtins;
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::builtins_arrays::exports as builtins_arrays;
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::builtins_bytes::exports as builtins_bytes;
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	pub use super::builtins_cache::exports as builtins_cache;
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::builtins_comparisons::exports as builtins_comparisons;
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::builtins_filesystem::exports as builtins_filesystem;
	pub use super::builtins_functions::exports as builtins_functions;
	#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
	pub use super::builtins_hashes::exports as builtins_hashes;
	pub use super::builtins_lists::exports as builtins_lists;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::builtins_ports::exports as builtins_ports;
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	pub use super::builtins_processes::exports as builtins_processes;
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	pub use super::builtins_records::exports as builtins_records;
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	pub use super::builtins_regularex::exports as builtins_regularex;
	pub use super::builtins_runtime::exports as builtins_runtime;
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	pub use super::builtins_serde::exports as builtins_serde;
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::builtins_strings::exports as builtins_strings;
	pub use super::builtins_types::exports as builtins_types;
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	pub use super::compiler::exports as compiler;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::compiler_optimizer::exports as compiler_optimizer;
	pub use super::constants::exports as constants;
	pub use super::contexts::exports as contexts;
	pub use super::conversions::exports as conversions;
	#[ cfg ( feature = "vonuvoli_documentation" ) ]
	pub use super::documentation::exports as documentation;
	pub use super::errors::exports as errors;
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	pub use super::evaluator::exports as evaluator;
	#[ cfg ( not ( feature = "vonuvoli_evaluator" ) ) ]
	pub use super::evaluator_trait::exports as evaluator;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	pub use super::expressions::exports as expressions;
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::extended_procedures::exports as extended_procedures;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::extended_syntaxes::exports as extended_syntaxes;
	#[ cfg ( feature = "vonuvoli_hash" ) ]
	pub use super::hashes::exports as hashes;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::lambdas::exports as lambdas;
	pub use super::libraries::exports as libraries;
	pub use super::libraries_builtins::exports as libraries_builtins;
	pub use super::libraries_r7rs::exports as libraries_r7rs;
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::native_procedures::exports as native_procedures;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::native_syntaxes::exports as native_syntaxes;
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	pub use super::parameters::exports as parameters;
	#[ cfg ( feature = "vonuvoli_parser" ) ]
	pub use super::parser::exports as parser;
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::paths::exports as paths;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::ports::exports as ports;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::ports_memory::exports as ports_memory;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::ports_native::exports as ports_native;
	pub use super::primitives::exports as primitives;
	pub use super::primitives_arithmetic::exports as primitives_arithmetic;
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::primitives_arrays::exports as primitives_arrays;
	pub use super::primitives_bitwise::exports as primitives_bitwise;
	pub use super::primitives_boolean::exports as primitives_boolean;
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::primitives_bytes::exports as primitives_bytes;
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	pub use super::primitives_comparisons::exports as primitives_comparisons;
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::primitives_filesystem::exports as primitives_filesystem;
	pub use super::primitives_functions::exports as primitives_functions;
	pub use super::primitives_lists::exports as primitives_lists;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::primitives_ports::exports as primitives_ports;
	pub use super::primitives_procedures::exports as primitives_procedures;
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	pub use super::primitives_records::exports as primitives_records;
	pub use super::primitives_runtime::exports as primitives_runtime;
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::primitives_strings::exports as primitives_strings;
	pub use super::primitives_syntaxes::exports as primitives_syntaxes;
	pub use super::primitives_types::exports as primitives_types;
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	pub use super::processes::exports as processes;
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	pub use super::regularex::exports as regularex;
	pub use super::runtime::exports as runtime;
	pub use super::runtime_backtrace::exports as runtime_backtrace;
	pub use super::runtime_configurations::exports as runtime_configurations;
	pub use super::runtime_iterators::exports as runtime_iterators;
	pub use super::runtime_unicode::exports as runtime_unicode;
	#[ cfg ( feature = "vonuvoli_tests" ) ]
	pub use super::tests::exports as tests;
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub use super::transcript::exports as transcript;
	pub use super::values::exports as values;
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	pub use super::values_arrays::exports as values_arrays;
	pub use super::values_booleans::exports as values_booleans;
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	pub use super::values_bytes::exports as values_bytes;
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::values_characters::exports as values_characters;
	pub use super::values_numbers::exports as values_numbers;
	#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
	pub use super::values_keywords::exports as values_keywords;
	#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
	pub use super::values_opaque::exports as values_opaque;
	pub use super::values_pairs::exports as values_pairs;
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	pub use super::values_records::exports as values_records;
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	pub use super::values_strings::exports as values_strings;
	pub use super::values_symbols::exports as values_symbols;
	#[ cfg ( any ( feature = "vonuvoli_values_unique", feature = "vonuvoli_builtins_parameters" ) ) ]
	pub use super::values_unique::exports as values_unique;
	#[ cfg ( any ( feature = "vonuvoli_parser", feature = "vonuvoli_tests" ) ) ]
	pub use super::values_tests::exports as values_tests;
	pub use super::values_value::exports as values_value;
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	pub use super::values_values::exports as values_values;
	
	#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
	pub use super::builtins_crypto::exports as builtins_crypto;
	
	#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
	pub use super::builtins_random::exports as builtins_random;
	
	#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
	pub use super::builtins_encoding::exports as builtins_encoding;
	
	#[ cfg ( feature = "vonuvoli_tools_common" ) ]
	pub use super::tools::exports as tools;
	#[ cfg ( feature = "vonuvoli_tools_common" ) ]
	pub use super::tools_common::exports as tools_common;
	#[ cfg ( feature = "vonuvoli_tools_interpreter" ) ]
	pub use super::tools_interpreter::exports as tools_interpreter;
	#[ cfg ( feature = "vonuvoli_tools_compiler" ) ]
	pub use super::tools_compiler::exports as tools_compiler;
	#[ cfg ( feature = "vonuvoli_tools_tester" ) ]
	pub use super::tools_tester::exports as tools_tester;
	#[ cfg ( feature = "vonuvoli_tools_bencher" ) ]
	pub use super::tools_bencher::exports as tools_bencher;
	#[ cfg ( feature = "vonuvoli_tools_reports" ) ]
	pub use super::tools_reports::exports as tools_reports;
	#[ cfg ( feature = "vonuvoli_tools_documentation" ) ]
	pub use super::tools_documentation::exports as tools_documentation;
	
}




pub mod exports {
	
	pub use super::builtins::exports::*;
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	pub use super::compiler::exports::*;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::compiler_optimizer::exports::*;
	pub use super::constants::exports::*;
	pub use super::contexts::exports::*;
	pub use super::conversions::exports::*;
	#[ cfg ( feature = "vonuvoli_documentation" ) ]
	pub use super::documentation::exports::*;
	pub use super::errors::exports::*;
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	pub use super::evaluator::exports::*;
	#[ cfg ( not ( feature = "vonuvoli_evaluator" ) ) ]
	pub use super::evaluator_trait::exports::*;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	pub use super::expressions::exports::*;
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::extended_procedures::exports::*;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	pub use super::extended_syntaxes::exports::*;
	#[ cfg ( feature = "vonuvoli_hash" ) ]
	pub use super::hashes::exports::*;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	pub use super::lambdas::exports::*;
	pub use super::libraries::exports::*;
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::native_procedures::exports::*;
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_compiler" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	pub use super::native_syntaxes::exports::*;
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	pub use super::parameters::exports::*;
	#[ cfg ( feature = "vonuvoli_parser" ) ]
	pub use super::parser::exports::*;
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	pub use super::paths::exports::*;
	#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
	pub use super::ports::exports::*;
	pub use super::primitives::exports::*;
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	pub use super::processes::exports::*;
	#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
	pub use super::regularex::exports::*;
	pub use super::runtime::exports::*;
	#[ cfg ( feature = "vonuvoli_tests" ) ]
	pub use super::tests::exports::*;
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub use super::transcript::exports::*;
	pub use super::values::exports::*;
	#[ cfg ( any ( feature = "vonuvoli_parser", feature = "vonuvoli_tests" ) ) ]
	pub use super::values_tests::exports::*;
	#[ cfg ( feature = "vonuvoli_tools_common" ) ]
	pub use super::tools::exports::*;
	
}




#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript_root! (TRANSCRIPT);

