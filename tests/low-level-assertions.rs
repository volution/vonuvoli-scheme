

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_test! (test__0, {
	
	assert! (0.0f64 == 0.0f64);
	assert! (-0.0f64 == 0.0f64);
	
	assert! (f64::NAN != f64::NAN);
	
});

