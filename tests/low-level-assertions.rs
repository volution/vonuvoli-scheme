

use std::f64;




#[ test ]
fn test () -> () {
	
	assert! (0.0f64 == 0.0f64);
	assert! (-0.0f64 == 0.0f64);
	
	assert! (f64::NAN != f64::NAN);
	
}

