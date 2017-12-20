

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_scheme_tests_from_file! (
		
		test__values => "scheme/values.sst",
		test__contexts => "scheme/contexts.sst",
		test__quotation => "scheme/quotation.sst",
		
		test__types => "scheme/types.sst",
		test__booleans => "scheme/booleans.sst",
		test__arithmetic => "scheme/arithmetic.sst",
		
		test__pairs => "scheme/pairs.sst",
		test__lists => "scheme/lists.sst",
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
		
		test__comparisons_equivalent_objects => "scheme/comparisons-equivalent-objects.sst",
		test__comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		test__comparisons_equivalent_by_value_strict => "scheme/comparisons-equivalent-by-value-strict.sst",
		test__comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		test__comparisons_equivalent_by_value_coerced => "scheme/comparisons-equivalent-by-value-coerced.sst",
		test__comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		
		test__comparisons_ordering_numbers => "scheme/comparisons-ordering-numbers.sst",
		test__comparisons_ordering_objects => "scheme/comparisons-ordering-objects.sst",
		
		test__ports => "scheme/ports.sst",
		test__ports_memory => "scheme/ports-memory.sst",
		test__ports_native => "scheme/ports-native.sst",
		
		test__runtime => "scheme/runtime.sst",
		
		test__scratchpad => "scheme/scratchpad.sst",
		test__issues => "scheme/issues.sst",
		
	);




def_scheme_benchmarks_from_file! (
		
		benchmark__types => "scheme/types.sst",
		benchmark__booleans => "scheme/booleans.sst",
		benchmark__arithmetic => "scheme/arithmetic.sst",
		
		benchmark__pairs => "scheme/pairs.sst",
		benchmark__lists => "scheme/lists.sst",
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
		
		benchmark__comparisons_equivalent_objects => "scheme/comparisons-equivalent-objects.sst",
		benchmark__comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		benchmark__comparisons_equivalent_by_value_strict => "scheme/comparisons-equivalent-by-value-strict.sst",
		benchmark__comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		benchmark__comparisons_equivalent_by_value_coerced => "scheme/comparisons-equivalent-by-value-coerced.sst",
		benchmark__comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		
		benchmark__comparisons_ordering_numbers => "scheme/comparisons-ordering-numbers.sst",
		benchmark__comparisons_ordering_objects => "scheme/comparisons-ordering-objects.sst",
		
		benchmark__ports => "scheme/ports.sst",
		benchmark__ports_memory => "scheme/ports-memory.sst",
		benchmark__ports_native => "scheme/ports-native.sst",
		
		benchmark__runtime => "scheme/runtime.sst",
		
		benchmark__scratchpad => "scheme/scratchpad.sst",
		benchmark__issues => "scheme/issues.sst",
		
	);

