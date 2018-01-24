

#![ feature (core) ]
#![ feature (libc) ]
#![ feature (test) ]

#![ feature (try_from) ]
#![ feature (ascii_ctype) ]
#![ feature (str_internals) ]
#![ feature (char_from_unchecked) ]
#![ feature (slice_get_slice) ]

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




include! ("macros.in");




pub mod builtins;
pub mod builtins_arrays;
pub mod builtins_bytes;
pub mod builtins_comparisons;
pub mod builtins_functions;
pub mod builtins_lists;
pub mod builtins_ports;
pub mod builtins_processes;
pub mod builtins_runtime;
pub mod builtins_strings;
pub mod builtins_types;
pub mod compiler;
pub mod compiler_optimizer;
pub mod constants;
pub mod contexts;
pub mod conversions;
pub mod counters;
pub mod display;
pub mod errors;
pub mod evaluator;
pub mod expressions;
pub mod extended_procedures;
pub mod extended_syntaxes;
pub mod globals;
pub mod hashes;
pub mod lambdas;
pub mod languages;
pub mod languages_builtins;
pub mod languages_r7rs;
pub mod native_procedures;
pub mod native_syntaxes;
pub mod ordering;
pub mod parser;
pub mod parser_peg;
pub mod ports;
pub mod ports_memory;
pub mod ports_native;
pub mod prelude;
pub mod primitives;
pub mod primitives_arithmetic;
pub mod primitives_arrays;
pub mod primitives_bitwise;
pub mod primitives_boolean;
pub mod primitives_bytes;
pub mod primitives_comparisons;
pub mod primitives_functions;
pub mod primitives_lists;
pub mod primitives_ports;
pub mod primitives_procedures;
pub mod primitives_runtime;
pub mod primitives_strings;
pub mod primitives_syntaxes;
pub mod primitives_types;
pub mod processes;
pub mod runtime;
pub mod runtime_configurations;
pub mod runtime_iterators;
pub mod runtime_unicode;
pub mod tests;
pub mod values;
pub mod values_arrays;
pub mod values_booleans;
pub mod values_bytes;
pub mod values_characters;
pub mod values_numbers;
pub mod values_pairs;
pub mod values_strings;
pub mod values_symbols;
pub mod values_value;
pub mod values_values;




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
	pub use super::parser::exports::*;
	pub use super::ports::exports::*;
	pub use super::primitives::exports::*;
	pub use super::tests::exports::*;
	pub use super::values::exports::*;
	
}

