

use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{posix_timestamp, jiffies_timestamp, jiffies_per_second};
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn posix_timestamp () -> (NumberReal) {
	let elapsed = match time::UNIX_EPOCH.elapsed () {
		Ok (elapsed) =>
			elapsed,
		Err (_) =>
			// NOTE:  It is impossible for the clock to be before the epoch!
			panic! ("09bcf425"),
	};
	let elapsed =
			(elapsed.as_secs () as f64)
			+ ((elapsed.subsec_nanos () as f64) / 1_000_000_000f64);
	return elapsed.into ();
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn jiffies_timestamp () -> (NumberInteger) {
	unsafe {
		match JIFFIES_INSTANT {
			Some (instant) => {
				let elapsed = instant.elapsed ();
				let elapsed_seconds = elapsed.as_secs ();
				// NOTE:  If this process runs for more than 100 years we shall panic!
				if elapsed_seconds < (25 * 1461 * 24 * 3600) {
					let elapsed =
							(elapsed_seconds * 1_000_000_000)
							+ (elapsed.subsec_nanos () as u64);
					return elapsed.into ();
				} else {
					panic! ("70f11280");
				}
			},
			None => {
				JIFFIES_INSTANT = Some (time::Instant::now ());
				return jiffies_timestamp ();
			}
		}
	}
}


#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn jiffies_per_second () -> (NumberInteger) {
	return 1_000_000_000.into ();
}


static mut JIFFIES_INSTANT : Option<time::Instant> = None;

