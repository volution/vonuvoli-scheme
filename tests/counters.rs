

extern crate rust_scheme;

use rust_scheme::counters::*;




#[ test ]
fn test () -> () {
	
	let mut counter = PermutationCounter::new ();
	
	for index in 0..1000 {
		println! ("{:08} -> {:08x}", index, counter.next ());
	}
	
}

