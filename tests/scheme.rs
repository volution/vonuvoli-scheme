

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_scheme_tests_from_file! (
		
		test__values => "scheme/values.sst",
		test__contexts => "scheme/contexts.sst",
		test__quotation => "scheme/quotation.sst",
		
		test__types => "scheme/types.sst",
		test__types_negated => "scheme/types-negated.sst",
		
		test__booleans => "scheme/booleans.sst",
		test__arithmetic => "scheme/arithmetic.sst",
		
		test__pairs => "scheme/pairs.sst",
		test__lists => "scheme/lists.sst",
		test__lists_mutations => "scheme/lists-mutations.sst",
		test__arrays => "scheme/arrays.sst",
		test__bytes => "scheme/bytes.sst",
		test__strings => "scheme/strings.sst",
		
		test__control => "scheme/control.sst",
		test__loops => "scheme/loops.sst",
		
		test__lambdas => "scheme/lambdas.sst",
		test__functions => "scheme/functions.sst",
		test__functions_lists => "scheme/functions-lists.sst",
		test__functions_arrays => "scheme/functions-arrays.sst",
		test__functions_bytes => "scheme/functions-bytes.sst",
		test__functions_strings => "scheme/functions-strings.sst",
		test__functions_higher => "scheme/functions-higher.sst",
		
		test__parameters => "scheme/parameters.sst",
		
		test__comparisons_equivalent_objects => "scheme/comparisons-equivalent-objects.sst",
		test__comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		test__comparisons_equivalent_by_value_strict_shallow => "scheme/comparisons-equivalent-by-value-strict-shallow.sst",
		test__comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		test__comparisons_equivalent_by_value_coerced_shallow => "scheme/comparisons-equivalent-by-value-coerced-shallow.sst",
		test__comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		
		test__comparisons_equivalent_objects_negated => "scheme/comparisons-equivalent-objects-negated.sst",
		test__comparisons_equivalent_by_identity_negated => "scheme/comparisons-equivalent-by-identity-negated.sst",
		test__comparisons_equivalent_by_value_strict_shallow_negated => "scheme/comparisons-equivalent-by-value-strict-shallow-negated.sst",
		test__comparisons_equivalent_by_value_strict_recursive_negated => "scheme/comparisons-equivalent-by-value-strict-recursive-negated.sst",
		test__comparisons_equivalent_by_value_coerced_shallow_negated => "scheme/comparisons-equivalent-by-value-coerced-shallow-negated.sst",
		test__comparisons_equivalent_by_value_coerced_recursive_negated => "scheme/comparisons-equivalent-by-value-coerced-recursive-negated.sst",
		
		test__comparisons_ordering_numbers => "scheme/comparisons-ordering-numbers.sst",
		test__comparisons_ordering_objects => "scheme/comparisons-ordering-objects.sst",
		
		test__ports => "scheme/ports.sst",
		test__ports_memory => "scheme/ports-memory.sst",
		test__ports_native => "scheme/ports-native.sst",
		test__ports_outputs => "scheme/ports-outputs.sst",
		test__ports_temporary => "scheme/ports-temporary.sst",
		test__ports_parameters => "scheme/ports-parameters.sst",
		
		test__paths => "scheme/paths.sst",
		test__filesystem => "scheme/filesystem.sst",
		
		test__records_procedures => "scheme/records-procedures.sst",
		test__records_syntaxes => "scheme/records-syntaxes.sst",
		
		test__errors => "scheme/errors.sst",
		test__runtime => "scheme/runtime.sst",
		
		test__processes => "scheme/processes.sst",
		
		test__scratchpad => "scheme/scratchpad.sst",
		test__issues => "scheme/issues.sst",
		
		test__srfi_1 => "scheme/srfi-1.sst",
		
	);




def_scheme_benchmarks_from_file! (
		
		benchmark__values => "scheme/values.sst",
		benchmark__contexts => "scheme/contexts.sst",
		benchmark__quotation => "scheme/quotation.sst",
		
		benchmark__types => "scheme/types.sst",
		benchmark__types_negated => "scheme/types-negated.sst",
		
		benchmark__booleans => "scheme/booleans.sst",
		benchmark__arithmetic => "scheme/arithmetic.sst",
		
		benchmark__pairs => "scheme/pairs.sst",
		benchmark__lists => "scheme/lists.sst",
		benchmark__lists_mutations => "scheme/lists-mutations.sst",
		benchmark__arrays => "scheme/arrays.sst",
		benchmark__bytes => "scheme/bytes.sst",
		benchmark__strings => "scheme/strings.sst",
		
		benchmark__control => "scheme/control.sst",
		benchmark__loops => "scheme/loops.sst",
		
		benchmark__lambdas => "scheme/lambdas.sst",
		benchmark__functions => "scheme/functions.sst",
		benchmark__functions_lists => "scheme/functions-lists.sst",
		benchmark__functions_arrays => "scheme/functions-arrays.sst",
		benchmark__functions_bytes => "scheme/functions-bytes.sst",
		benchmark__functions_strings => "scheme/functions-strings.sst",
		benchmark__functions_higher => "scheme/functions-higher.sst",
		
		benchmark__parameters => "scheme/parameters.sst",
		
		benchmark__comparisons_equivalent_objects => "scheme/comparisons-equivalent-objects.sst",
		benchmark__comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		benchmark__comparisons_equivalent_by_value_strict_shallow => "scheme/comparisons-equivalent-by-value-strict-shallow.sst",
		benchmark__comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		benchmark__comparisons_equivalent_by_value_coerced_shallow => "scheme/comparisons-equivalent-by-value-coerced-shallow.sst",
		benchmark__comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		
		benchmark__comparisons_equivalent_objects_negated => "scheme/comparisons-equivalent-objects-negated.sst",
		benchmark__comparisons_equivalent_by_identity_negated => "scheme/comparisons-equivalent-by-identity-negated.sst",
		benchmark__comparisons_equivalent_by_value_strict_shallow_negated => "scheme/comparisons-equivalent-by-value-strict-shallow-negated.sst",
		benchmark__comparisons_equivalent_by_value_strict_recursive_negated => "scheme/comparisons-equivalent-by-value-strict-recursive-negated.sst",
		benchmark__comparisons_equivalent_by_value_coerced_shallow_negated => "scheme/comparisons-equivalent-by-value-coerced-shallow-negated.sst",
		benchmark__comparisons_equivalent_by_value_coerced_recursive_negated => "scheme/comparisons-equivalent-by-value-coerced-recursive-negated.sst",
		
		benchmark__comparisons_ordering_numbers => "scheme/comparisons-ordering-numbers.sst",
		benchmark__comparisons_ordering_objects => "scheme/comparisons-ordering-objects.sst",
		
		benchmark__ports => "scheme/ports.sst",
		benchmark__ports_memory => "scheme/ports-memory.sst",
		benchmark__ports_native => "scheme/ports-native.sst",
		benchmark__ports_outputs => "scheme/ports-outputs.sst",
		benchmark__ports_temporary => "scheme/ports-temporary.sst",
		benchmark__ports_parameters => "scheme/ports-parameters.sst",
		
		benchmark__paths => "scheme/paths.sst",
		benchmark__filesystem => "scheme/filesystem.sst",
		
		benchmark__records_procedures => "scheme/records-procedures.sst",
		benchmark__records_syntaxes => "scheme/records-syntaxes.sst",
		
		benchmark__errors => "scheme/errors.sst",
		benchmark__runtime => "scheme/runtime.sst",
		
		benchmark__processes => "scheme/processes.sst",
		
		benchmark__scratchpad => "scheme/scratchpad.sst",
		benchmark__issues => "scheme/issues.sst",
		
		benchmark__srfi_1 => "scheme/srfi-1.sst",
		
	);




#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
def_scheme_tests_from_file! (
		
		test__regex_strings => "scheme/regex-strings.sst",
		test__regex_bytes => "scheme/regex-bytes.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_crypto" ) ]
def_scheme_tests_from_file! (
		
		test__crypto => "scheme/crypto.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_random" ) ]
def_scheme_tests_from_file! (
		
		test__random => "scheme/random.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_encoding" ) ]
def_scheme_tests_from_file! (
		
		test__encoding => "scheme/encoding.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
def_scheme_tests_from_file! (
		
		test__cache_bytes_default => "scheme/cache-bytes-default.sst",
		test__cache_bytes_namespace => "scheme/cache-bytes-namespace.sst",
		
		test__cache_objects_default => "scheme/cache-objects-default.sst",
		test__cache_objects_namespace => "scheme/cache-objects-namespace.sst",
		
		test__cache_objects_partitions => "scheme/cache-objects-partitions.sst",
		test__cache_objects_busting => "scheme/cache-objects-busting.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
def_scheme_tests_from_file! (
		
		test__hashes_default => "scheme/hashes-default.sst",
		test__hashes_siphash => "scheme/hashes-siphash.sst",
		test__hashes_seahash => "scheme/hashes-seahash.sst",
		test__hashes_blake2b => "scheme/hashes-blake2b.sst",
		test__hashes_blake2s => "scheme/hashes-blake2s.sst",
		
	);


#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
def_scheme_tests_from_file! (
		
		test__serde => "scheme/serde.sst",
		
	);


