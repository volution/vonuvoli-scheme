

#![ feature (test) ]

#[ macro_use ]
extern crate rust_scheme;

extern crate test;




def_tests_from_file! (
		
		values => "scheme/values.sst",
		quotation => "scheme/quotation.sst",
		
		types => "scheme/types.sst",
		booleans => "scheme/booleans.sst",
		
		pairs => "scheme/pairs.sst",
		lists => "scheme/lists.sst",
		arrays => "scheme/arrays.sst",
		bytes => "scheme/bytes.sst",
		strings => "scheme/strings.sst",
		
		contexts => "scheme/contexts.sst",
		control => "scheme/control.sst",
		loops => "scheme/loops.sst",
		
		lambdas => "scheme/lambdas.sst",
		functions => "scheme/functions.sst",
		functions_lists => "scheme/functions-lists.sst",
		functions_arrays => "scheme/functions-arrays.sst",
		functions_bytes => "scheme/functions-bytes.sst",
		functions_strings => "scheme/functions-strings.sst",
		
		comparisons_equivalent_objects => "scheme/comparisons-equivalent-objects.sst",
		comparisons_equivalent_by_identity => "scheme/comparisons-equivalent-by-identity.sst",
		comparisons_equivalent_by_value_strict => "scheme/comparisons-equivalent-by-value-strict.sst",
		comparisons_equivalent_by_value_strict_recursive => "scheme/comparisons-equivalent-by-value-strict-recursive.sst",
		comparisons_equivalent_by_value_coerced => "scheme/comparisons-equivalent-by-value-coerced.sst",
		comparisons_equivalent_by_value_coerced_recursive => "scheme/comparisons-equivalent-by-value-coerced-recursive.sst",
		
		comparisons_ordering_numbers => "scheme/comparisons-ordering-numbers.sst",
		comparisons_ordering_objects => "scheme/comparisons-ordering-objects.sst",
		
		ports => "scheme/ports.sst",
		ports_memory => "scheme/ports-memory.sst",
		
		runtime => "scheme/runtime.sst",
		
		optimizations => "scheme/optimizations.sst",
		
	);




def_benchmarks_from_file! (
		
		values_benchmark => "scheme/values.sst",
		
	);

