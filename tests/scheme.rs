

#[ macro_use ]
extern crate rust_scheme;




def_tests_from_file! (
		values => "scheme/values.sst",
		types => "scheme/types.sst",
		pairs => "scheme/pairs.sst",
		lists => "scheme/lists.sst",
		quotation => "scheme/quotation.sst",
		booleans => "scheme/booleans.sst",
		control => "scheme/control.sst",
		contexts => "scheme/contexts.sst",
		lambdas => "scheme/lambdas.sst",
		functions => "scheme/functions.sst",
		functions_lists => "scheme/functions-lists.sst",
		functions_arrays => "scheme/functions-arrays.sst",
		functions_bytes => "scheme/functions-bytes.sst",
		functions_strings => "scheme/functions-strings.sst",
		arrays => "scheme/arrays.sst",
		bytes => "scheme/bytes.sst",
		strings => "scheme/strings.sst",
	);

