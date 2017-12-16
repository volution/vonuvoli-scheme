

use std::f64;




#[ test ]
#[ allow (non_snake_case) ]
fn test__0 () -> () {
	
	assert! (0.0f64 == 0.0f64);
	assert! (-0.0f64 == 0.0f64);
	
	assert! (f64::NAN != f64::NAN);
	
}

