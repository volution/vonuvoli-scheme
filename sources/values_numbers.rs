

use super::errors::*;

use super::prelude::*;




pub mod exports {
	pub use super::{NumberInteger, NumberReal};
	pub use super::{number_i64, number_f64};
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct NumberInteger ( pub i64 );


impl NumberInteger {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn value (&self) -> (i64) {
		self.0
	}
}


macro_rules! NumberInteger_fn_try_to_signed_integer {
	($export : ident, $type : ty) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if mem::size_of::<i64> () <= mem::size_of::<$type> () {
				succeed! (value as $type);
			}
			let min = <$type>::min_value () as i64;
			if value < min {
				fail! (0xe1deffc5);
			}
			let max = <$type>::max_value () as i64;
			if value > max {
				fail! (0x3d9c7881);
			}
			succeed! (value as $type);
		}
	);
}

macro_rules! NumberInteger_fn_try_to_unsigned_integer {
	($export : ident, $type : ty) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if value < 0 {
				fail! (0xe4d76587);
			}
			if mem::size_of::<i64> () <= mem::size_of::<$type> () {
				succeed! (value as $type);
			}
			let max = <$type>::max_value () as i64;
			if value > max {
				fail! (0x4212bbb7);
			}
			succeed! (value as $type);
		}
	);
}


macro_rules! NumberInteger_fn_predicate {
	($delegate : ident) => (
		NumberInteger_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (bool) {
			<i64>::$delegate (self.0)
		}
	);
}


macro_rules! NumberInteger_fn_delegate_1 {
	($delegate : ident) => (
		NumberInteger_fn_delegate_1! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (NumberInteger) {
			<i64>::$delegate (self.0) .into ()
		}
	);
}

macro_rules! NumberInteger_fn_delegate_2 {
	($delegate : ident) => (
		NumberInteger_fn_delegate_2! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self, other : &NumberInteger) -> (NumberInteger) {
			<i64>::$delegate (self.0, other.0) .into ()
		}
	);
}


macro_rules! NumberInteger_fn_delegate_1_real {
	($delegate : ident) => (
		NumberInteger_fn_delegate_1_real! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (NumberReal) {
			<f64>::$delegate (self.0 as f64) .into ()
		}
	);
}

macro_rules! NumberInteger_fn_delegate_2_real {
	($delegate : ident) => (
		NumberInteger_fn_delegate_2_real! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0 as f64, other.0) .into ()
		}
	);
}


impl NumberInteger {
	
	
	NumberInteger_fn_try_to_signed_integer! (try_to_i8, i8);
	NumberInteger_fn_try_to_signed_integer! (try_to_i16, i16);
	NumberInteger_fn_try_to_signed_integer! (try_to_i32, i32);
	NumberInteger_fn_try_to_signed_integer! (try_to_i64, i64);
	NumberInteger_fn_try_to_signed_integer! (try_to_isize, isize);
	
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u8, u8);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u16, u16);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u32, u32);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u64, u64);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_usize, usize);
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try_to_char (&self) -> (Outcome<char>) {
		let value = try! (self.try_to_u32 ());
		if let Some (value) = char::from_u32 (value) {
			succeed! (value);
		} else {
			fail! (0x36d5ef86);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try_to_real (&self) -> (Outcome<NumberReal>) {
		succeed! ((self.0 as f64) .into ());
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn neg (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_neg (self.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xd93d04db);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn abs (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_abs (self.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x997daa2a);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn add (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_add (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xd61736b6);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn sub (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_sub (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x1e036be9);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn mul (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_mul (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x32c5b516);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn div (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_div (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x39216440);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn rem (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_rem (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xce26bc76);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn pow (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xdcca20dd);
		}
		succeed! (<i64>::pow (self.0, other as u32) .into ());
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_zero (&self) -> (bool) {
		self.0 == 0
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_even (&self) -> (bool) {
		(self.0 & 1) == 0
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_odd (&self) -> (bool) {
		(self.0 & 1) != 0
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitnot (&self) -> (NumberInteger) {
		(!self.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitand (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 & other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 | other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitxor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 ^ other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitnand (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitand (other) .bitnot ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitnor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitor (other) .bitnot ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn bitnxor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitxor (other) .bitnot ()
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn shl (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xb84272a0);
		}
		if let Some (outcome) = <i64>::checked_shl (self.0, other as u32) {
			succeed! (outcome.into ());
		} else {
			fail! (0x734e69d8);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn shr (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x26d90f55);
		}
		if let Some (outcome) = <i64>::checked_shr (self.0, other as u32) {
			succeed! (outcome.into ());
		} else {
			fail! (0xc3bb81a9);
		}
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn rotate_left (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xe2038e82);
		}
		succeed! ((<i64>::rotate_left (self.0, other as u32)) .into ());
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn rotate_right (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x1d868231);
		}
		succeed! ((<i64>::rotate_right (self.0, other as u32)) .into ());
	}
	
	
	NumberInteger_fn_delegate_1! (wrapping_neg);
	NumberInteger_fn_delegate_1! (wrapping_abs);
	NumberInteger_fn_delegate_2! (wrapping_add);
	NumberInteger_fn_delegate_2! (wrapping_sub);
	NumberInteger_fn_delegate_2! (wrapping_mul);
	NumberInteger_fn_delegate_2! (wrapping_div);
	NumberInteger_fn_delegate_2! (wrapping_rem);
	
	NumberInteger_fn_delegate_2! (saturating_add);
	NumberInteger_fn_delegate_2! (saturating_sub);
	NumberInteger_fn_delegate_2! (saturating_mul);
	
	NumberInteger_fn_delegate_1! (signum);
	
	NumberInteger_fn_predicate! (is_positive);
	NumberInteger_fn_predicate! (is_negative);
	
	NumberInteger_fn_delegate_2! (min);
	NumberInteger_fn_delegate_2! (max);
	
	NumberInteger_fn_delegate_1! (count_ones);
	NumberInteger_fn_delegate_1! (count_zeros);
	NumberInteger_fn_delegate_1! (leading_zeros);
	NumberInteger_fn_delegate_1! (trailing_zeros);
	NumberInteger_fn_delegate_1! (swap_bytes);
	NumberInteger_fn_delegate_1! (from_be);
	NumberInteger_fn_delegate_1! (from_le);
	NumberInteger_fn_delegate_1! (to_be);
	NumberInteger_fn_delegate_1! (to_le);
	
	NumberInteger_fn_delegate_1_real! (recip);
	NumberInteger_fn_delegate_1_real! (sqrt);
	NumberInteger_fn_delegate_1_real! (cbrt);
	
	NumberInteger_fn_delegate_2_real! (power, powf);
	NumberInteger_fn_delegate_2_real! (log);
	
	NumberInteger_fn_delegate_1_real! (exp);
	NumberInteger_fn_delegate_1_real! (exp2);
	NumberInteger_fn_delegate_1_real! (exp_m1);
	NumberInteger_fn_delegate_1_real! (ln);
	NumberInteger_fn_delegate_1_real! (log2);
	NumberInteger_fn_delegate_1_real! (log10);
	NumberInteger_fn_delegate_1_real! (ln_1p);
	
	NumberInteger_fn_delegate_1_real! (sin);
	NumberInteger_fn_delegate_1_real! (cos);
	NumberInteger_fn_delegate_1_real! (tan);
	NumberInteger_fn_delegate_1_real! (asin);
	NumberInteger_fn_delegate_1_real! (acos);
	NumberInteger_fn_delegate_1_real! (atan);
	
	NumberInteger_fn_delegate_1_real! (sinh);
	NumberInteger_fn_delegate_1_real! (cosh);
	NumberInteger_fn_delegate_1_real! (tanh);
	NumberInteger_fn_delegate_1_real! (asinh);
	NumberInteger_fn_delegate_1_real! (acosh);
	NumberInteger_fn_delegate_1_real! (atanh);
	
	NumberInteger_fn_delegate_2_real! (hypot);
	NumberInteger_fn_delegate_2_real! (atan2);
	
	NumberInteger_fn_delegate_1_real! (to_degrees);
	NumberInteger_fn_delegate_1_real! (to_radians);
	
}


impl ops::Neg for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn neg (self) -> (Outcome<NumberInteger>) {
		NumberInteger::neg (&self)
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Add<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn add (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::add (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Sub<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn sub (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::sub (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Mul<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn mul (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::mul (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Div<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn div (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::div (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Rem<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn rem (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::rem (&self, &other.into ())
	}
}


impl ops::Not for NumberInteger {
	
	type Output = NumberInteger;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn not (self) -> (NumberInteger) {
		NumberInteger::bitnot (&self)
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitAnd<NumberIntegerInto> for NumberInteger {
	
	type Output = NumberInteger;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bitand (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitand (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitOr<NumberIntegerInto> for NumberInteger {
	
	type Output = NumberInteger;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bitor (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitor (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitXor<NumberIntegerInto> for NumberInteger {
	
	type Output = NumberInteger;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn bitxor (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitxor (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Shl<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn shl (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::shl (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Shr<NumberIntegerInto> for NumberInteger {
	
	type Output = Outcome<NumberInteger>;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn shr (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::shr (&self, &other.into ())
	}
}




#[ derive (Clone, Debug) ]
pub struct NumberReal ( pub f64 );


impl NumberReal {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn value (&self) -> (f64) {
		self.0
	}
}


macro_rules! NumberReal_fn_try_to_signed_integer {
	($export : ident, $type : ty) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if ! value.is_finite () {
				fail! (0xd0fb8a28);
			}
			if value != value.trunc () {
				fail! (0xdbe9740c);
			}
			let min = <$type>::min_value () as f64;
			if value < min {
				fail! (0x00d89f0a);
			}
			let max = <$type>::max_value () as f64;
			if value > max {
				fail! (0x0cafe950);
			}
			succeed! (value as $type);
		}
	);
}

macro_rules! NumberReal_fn_try_to_unsigned_integer {
	($export : ident, $type : ty) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if ! value.is_finite () {
				fail! (0xb6d5d018);
			}
			if value != value.trunc () {
				fail! (0xb1015ff1);
			}
			if value < 0.0 {
				fail! (0x5a4b35a7);
			}
			let max = <$type>::max_value () as f64;
			if value > max {
				fail! (0x7d87bb56);
			}
			succeed! (value as $type);
		}
	);
}


macro_rules! NumberReal_fn_predicate {
	($delegate : ident) => (
		NumberReal_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (bool) {
			<f64>::$delegate (self.0)
		}
	);
}


macro_rules! NumberReal_fn_delegate_1 {
	($delegate : ident) => (
		NumberReal_fn_delegate_1! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self) -> (NumberReal) {
			<f64>::$delegate (self.0) .into ()
		}
	);
}

macro_rules! NumberReal_fn_delegate_2 {
	($delegate : ident) => (
		NumberReal_fn_delegate_2! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0, other.0) .into ()
		}
	);
}


impl NumberReal {
	
	
	NumberReal_fn_try_to_signed_integer! (try_to_i8, i8);
	NumberReal_fn_try_to_signed_integer! (try_to_i16, i16);
	NumberReal_fn_try_to_signed_integer! (try_to_i32, i32);
	NumberReal_fn_try_to_signed_integer! (try_to_i64, i64);
	NumberReal_fn_try_to_signed_integer! (try_to_isize, isize);
	
	NumberReal_fn_try_to_unsigned_integer! (try_to_u8, u8);
	NumberReal_fn_try_to_unsigned_integer! (try_to_u16, u16);
	NumberReal_fn_try_to_unsigned_integer! (try_to_u32, u32);
	NumberReal_fn_try_to_unsigned_integer! (try_to_u64, u64);
	NumberReal_fn_try_to_unsigned_integer! (try_to_usize, usize);
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try_to_f32 (&self) -> (Outcome<f32>) {
		let value = self.0;
		if value.is_finite () {
			let min = f32::MIN as f64;
			if value < min {
				fail! (0x65eca18f);
			}
			let max = f32::MAX as f64;
			if value > max {
				fail! (0x219dba58);
			}
			succeed! (value as f32);
		} else if value == f64::NAN {
			succeed! (f32::NAN);
		} else if value == f64::INFINITY {
			succeed! (f32::INFINITY);
		} else  if value == f64::NEG_INFINITY {
			succeed! (f32::NEG_INFINITY);
		} else {
			fail_panic! (0xa371a722);
		}
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn try_to_integer (&self) -> (Outcome<NumberInteger>) {
		succeed! (try! (self.try_to_i64 ()) .into ());
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn neg (&self) -> (NumberReal) {
		(-self.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn add (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 + other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn sub (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 - other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn mul (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 * other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn div (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 / other.0) .into ()
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn rem (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 % other.0) .into ()
	}
	
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_zero (&self) -> (bool) {
		self.0 == 0.0
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_even (&self) -> (bool) {
		(self.0 % 2.0) == 0.0
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	pub fn is_odd (&self) -> (bool) {
		(self.0 % 2.0) != 0.0
	}
	
	
	NumberReal_fn_delegate_1! (abs);
	NumberReal_fn_delegate_1! (signum);
	
	NumberReal_fn_predicate! (is_finite);
	NumberReal_fn_predicate! (is_infinite);
	NumberReal_fn_predicate! (is_nan);
	NumberReal_fn_predicate! (is_positive, is_sign_positive);
	NumberReal_fn_predicate! (is_negative, is_sign_negative);
	
	NumberReal_fn_delegate_2! (min);
	NumberReal_fn_delegate_2! (max);
	
	NumberReal_fn_delegate_1! (floor);
	NumberReal_fn_delegate_1! (ceil);
	NumberReal_fn_delegate_1! (round);
	NumberReal_fn_delegate_1! (trunc);
	NumberReal_fn_delegate_1! (fract);
	
	NumberReal_fn_delegate_1! (recip);
	NumberReal_fn_delegate_1! (sqrt);
	NumberReal_fn_delegate_1! (cbrt);
	
	NumberReal_fn_delegate_2! (power, powf);
	NumberReal_fn_delegate_2! (log);
	
	NumberReal_fn_delegate_1! (exp);
	NumberReal_fn_delegate_1! (exp2);
	NumberReal_fn_delegate_1! (exp_m1);
	NumberReal_fn_delegate_1! (ln);
	NumberReal_fn_delegate_1! (log2);
	NumberReal_fn_delegate_1! (log10);
	NumberReal_fn_delegate_1! (ln_1p);
	
	NumberReal_fn_delegate_1! (sin);
	NumberReal_fn_delegate_1! (cos);
	NumberReal_fn_delegate_1! (tan);
	NumberReal_fn_delegate_1! (asin);
	NumberReal_fn_delegate_1! (acos);
	NumberReal_fn_delegate_1! (atan);
	
	NumberReal_fn_delegate_1! (sinh);
	NumberReal_fn_delegate_1! (cosh);
	NumberReal_fn_delegate_1! (tanh);
	NumberReal_fn_delegate_1! (asinh);
	NumberReal_fn_delegate_1! (acosh);
	NumberReal_fn_delegate_1! (atanh);
	
	NumberReal_fn_delegate_2! (hypot);
	NumberReal_fn_delegate_2! (atan2);
	
	NumberReal_fn_delegate_1! (to_degrees);
	NumberReal_fn_delegate_1! (to_radians);
	
}


impl ops::Neg for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn neg (self) -> (NumberReal) {
		NumberReal::neg (&self)
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Add<NumberRealInto> for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn add (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::add (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Sub<NumberRealInto> for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn sub (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::sub (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Mul<NumberRealInto> for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn mul (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::mul (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Div<NumberRealInto> for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn div (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::div (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Rem<NumberRealInto> for NumberReal {
	
	type Output = NumberReal;
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
	fn rem (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::rem (&self, &other.into ())
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn number_i64 (value : i64) -> (NumberInteger) {
	NumberInteger (value)
}

#[ cfg_attr ( feature = "scheme_inline_always", inline (always) ) ]
pub fn number_f64 (value : f64) -> (NumberReal) {
	NumberReal (value)
}

