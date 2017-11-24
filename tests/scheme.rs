

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
		arrays => "scheme/arrays.sst",
	);

