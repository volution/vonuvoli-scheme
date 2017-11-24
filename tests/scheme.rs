

#[ macro_use ]
extern crate rust_scheme;




def_tests_from_file! (
		values => "scheme/values.sst",
		types => "scheme/types.sst",
		lists => "scheme/lists.sst",
		quotation => "scheme/quotation.sst",
		booleans => "scheme/booleans.sst",
		control => "scheme/control.sst",
		contexts => "scheme/contexts.sst",
		lambdas => "scheme/lambdas.sst",
		functions => "scheme/functions.sst",
	);

