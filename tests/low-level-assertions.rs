

#![ feature (test) ]
#![ no_implicit_prelude ]
#![ allow (deprecated) ] // FIXME
#![ allow (bare_trait_objects) ] // FIXME
include! ("prelude.in");




def_test! (test__0, {
	
	assert! (0.0f64 == 0.0f64);
	assert! (-0.0f64 == 0.0f64);
	
	assert! (f64::NAN != f64::NAN);
	
});

