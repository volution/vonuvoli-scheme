

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	
	
	
	assert_eq! (TRUE, boolean (true));
	assert_eq! (FALSE, boolean (false));
	
	assert_eq! (FALSE, Boolean::from (false));
	assert_eq! (FALSE, false.into ());
	assert_eq! (false, FALSE.into ());
	
	
	
	
	assert_eq! (ZERO, number_i64 (0));
	assert_eq! (ONE, number_i64 (1));
	
	assert_eq! (ZERO, 0.into ());
	assert_eq! (ZERO, 0i8.into ());
	assert_eq! (ZERO, 0u8.into ());
	assert_eq! (ZERO, 0i16.into ());
	assert_eq! (ZERO, 0u16.into ());
	assert_eq! (ZERO, 0i32.into ());
	assert_eq! (ZERO, 0u32.into ());
	assert_eq! (ZERO, 0i64.into ());
	
	assert_eq! (ZERO, NumberInteger::from (0));
	assert_eq! (ZERO, 0.into ());
	assert_eq! (0i64, ZERO.into ());
	
	
	
	
	assert_eq! (ZERO_REAL_POSITIVE, number_f64 (0.0));
	assert_eq! (ONE_REAL_POSITIVE, number_f64 (1.0));
	
	assert_eq! (ZERO_REAL_POSITIVE, 0.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0f32.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0f64.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0i8.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0u8.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0i16.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0u16.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0i32.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0u32.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0i64.into ());
	assert_eq! (ZERO_REAL_POSITIVE, 0u64.into ());
	
	assert_eq! (ZERO_REAL_POSITIVE, NumberReal::from (0.0));
	assert_eq! (ZERO_REAL_POSITIVE, 0.0.into ());
	assert_eq! (0.0, ZERO_REAL_POSITIVE.into ());
	
	assert_eq! (ZERO_REAL_POSITIVE, NumberReal::from (ZERO));
	assert_eq! (ZERO_REAL_POSITIVE, ZERO.into ());
	
	
	
	
	assert_eq! (symbol ("".into ()), Symbol::from (""));
	assert_eq! (symbol ("".into ()), "".into ());
	
	assert_eq! (string ("".into ()), String::from (""));
	assert_eq! (string ("".into ()), "".into ());
	
	
	
	
	assert_eq! (ZERO, (- ZERO).unwrap ());
	assert_eq! (ZERO, (ZERO + ZERO).unwrap ());
	assert_eq! (ZERO, (ZERO + 0).unwrap ());
	assert_eq! (ZERO, (ZERO - ZERO).unwrap ());
	assert_eq! (ZERO, (ZERO - 0).unwrap ());
	
	assert_eq! (ONE, (ONE * ONE).unwrap ());
	assert_eq! (ONE, (ONE * 1).unwrap ());
	assert_eq! (ONE, (ONE / ONE).unwrap ());
	assert_eq! (ONE, (ONE / 1).unwrap ());
	assert_eq! (ZERO, (ONE % ONE).unwrap ());
	assert_eq! (ZERO, (ONE % 1).unwrap ());
	
	assert_eq! (number_i64 (-1), !ZERO);
	assert_eq! (ZERO, ZERO & ZERO);
	assert_eq! (ZERO, ZERO | ZERO);
	assert_eq! (ZERO, ZERO ^ ZERO);
	assert_eq! (ZERO, ZERO ^ 0);
	assert_eq! (ONE, ONE & ONE);
	assert_eq! (ONE, ONE | ONE);
	assert_eq! (ZERO, ONE ^ ONE);
	assert_eq! (ZERO, ONE ^ 1);
	assert_eq! (number_i64 (1 << 1), (ONE << ONE).unwrap ());
	assert_eq! (number_i64 (1 << 1), (ONE << 1).unwrap ());
	assert_eq! (number_i64 (1 >> 1), (ONE >> ONE).unwrap ());
	assert_eq! (number_i64 (1 >> 1), (ONE >> 1).unwrap ());
	
	
	
	
	assert_eq! (ZERO_REAL_POSITIVE, -ZERO_REAL_NEGATIVE);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE + ZERO_REAL_POSITIVE);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE + ZERO);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE + 0);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE - ZERO_REAL_POSITIVE);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE - ZERO);
	assert_eq! (ZERO_REAL_POSITIVE, ZERO_REAL_POSITIVE - 0);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE * ONE_REAL_POSITIVE);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE * ONE);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE * 1);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE / ONE_REAL_POSITIVE);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE / ONE);
	assert_eq! (ONE_REAL_POSITIVE, ONE_REAL_POSITIVE / 1);
	assert_eq! (ZERO_REAL_POSITIVE, ONE_REAL_POSITIVE % ONE_REAL_POSITIVE);
	assert_eq! (ZERO_REAL_POSITIVE, ONE_REAL_POSITIVE % ONE);
	assert_eq! (ZERO_REAL_POSITIVE, ONE_REAL_POSITIVE % 1);
	// assert_eq! (INF_POSITIVE, ONE_REAL_POSITIVE / ZERO_REAL_POSITIVE);
	// assert_eq! (NAN_POSITIVE, ONE_REAL_POSITIVE % ZERO_REAL_POSITIVE);
	
	
	
	
}

