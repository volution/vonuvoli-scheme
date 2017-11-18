

#[ macro_use ]
extern crate rust_scheme;




def_tests_from_file! (
		values => "scheme/values.sst",
		booleans => "scheme/booleans.sst",
		control => "scheme/control.sst",
	);

