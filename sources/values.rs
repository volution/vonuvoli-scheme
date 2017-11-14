

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::ops;




pub mod exports {
	pub use super::{ValueClass};
	pub use super::{Value, ValueBox, ValueVec};
	pub use super::{Boolean, NumberInteger, NumberReal, Character, Symbol, String, Bytes, Pair, Array};
	pub use super::{boolean, number_i64, number_f64, character};
	pub use super::{symbol, symbol_from_slice, symbol_from_characters};
	pub use super::{string, string_from_slice, string_from_characters};
	pub use super::{bytes_from_slice, array_from_slice};
	pub use super::{list_from_slice, list_from_slice_2};
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	
	Boolean,
	NumberInteger,
	NumberReal,
	Character,
	
	Symbol,
	String,
	Bytes,
	
	Pair,
	Array,
	
	Error,
	
	Lambda,
	ProcedurePrimitive,
	SyntaxPrimitive,
	
	Context,
	Binding,
	
	Number,
	Procedure,
	Syntax,
	
	List,
	ListProper,
	ListDotted,
	
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Value {
	
	Null,
	Void,
	Undefined,
	
	Boolean ( Boolean ),
	NumberInteger ( NumberInteger ),
	NumberReal ( NumberReal ),
	Character ( Character ),
	
	Symbol ( Symbol ),
	String ( String ),
	Bytes ( Bytes ),
	
	Pair ( Pair ),
	Array ( Array ),
	
	Error ( Error ),
	
	Lambda ( Lambda ),
	ProcedurePrimitive ( ProcedurePrimitive ),
	SyntaxPrimitive ( SyntaxPrimitive ),
	
	Context ( Context ),
	Binding ( Binding ),
	
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


impl Value {
	
	#[ inline (always) ]
	pub fn class (&self) -> (ValueClass) {
		match *self {
			
			Value::Null => ValueClass::Null,
			Value::Void => ValueClass::Void,
			Value::Undefined => ValueClass::Undefined,
			
			Value::Boolean (_) => ValueClass::Boolean,
			Value::NumberInteger (_) => ValueClass::NumberInteger,
			Value::NumberReal (_) => ValueClass::NumberReal,
			Value::Character (_) => ValueClass::Character,
			
			Value::Symbol (_) => ValueClass::Symbol,
			Value::String (_) => ValueClass::String,
			Value::Bytes (_) => ValueClass::Bytes,
			
			Value::Pair (_) => ValueClass::Pair,
			Value::Array (_) => ValueClass::Array,
			
			Value::Error (_) => ValueClass::Error,
			
			Value::Lambda (_) => ValueClass::Lambda,
			Value::ProcedurePrimitive (_) => ValueClass::ProcedurePrimitive,
			Value::SyntaxPrimitive (_) => ValueClass::SyntaxPrimitive,
			
			Value::Context (_) => ValueClass::Context,
			Value::Binding (_) => ValueClass::Binding,
			
		}
	}
	
	#[ inline (always) ]
	pub fn is (&self, class : ValueClass) -> (bool) {
		let class_actual = self.class ();
		if class_actual == class {
			return true;
		} else {
			match class {
				ValueClass::Number =>
					return (class_actual == ValueClass::NumberInteger) || (class_actual == ValueClass::NumberReal),
				ValueClass::List =>
					return (class_actual == ValueClass::Null) || (class_actual == ValueClass::Pair),
				ValueClass::ListProper | ValueClass::ListDotted =>
					return (class_actual == ValueClass::Null) || ((class_actual == ValueClass::Pair) && StdAsRef::<Pair>::as_ref (self) .right () .is (class)),
				ValueClass::Procedure =>
					return (class_actual == ValueClass::ProcedurePrimitive) || (class_actual == ValueClass::Lambda),
				ValueClass::Syntax =>
					return (class_actual == ValueClass::SyntaxPrimitive) || false,
				_ =>
					return false,
			}
		}
	}
	
}




impl fmt::Display for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			Value::Null => formatter.write_str ("#null"),
			Value::Void => formatter.write_str ("#void"),
			Value::Undefined => formatter.write_str ("#undefined"),
			Value::Boolean (ref value) => value.fmt (formatter),
			Value::NumberInteger (ref value) => value.fmt (formatter),
			Value::NumberReal (ref value) => value.fmt (formatter),
			Value::Character (ref value) => value.fmt (formatter),
			Value::Symbol (ref value) => value.fmt (formatter),
			Value::String (ref value) => value.fmt (formatter),
			Value::Bytes (ref value) => value.fmt (formatter),
			Value::Pair (ref value) => value.fmt (formatter),
			Value::Array (ref value) => value.fmt (formatter),
			Value::Error (ref value) => value.fmt (formatter),
			Value::Lambda (ref value) => value.fmt (formatter),
			Value::ProcedurePrimitive (ref value) => write! (formatter, "#<procedure:{:?}>", value),
			Value::SyntaxPrimitive (ref value) => write! (formatter, "#<syntax:{:?}>", value),
			Value::Context (ref value) => value.fmt (formatter),
			Value::Binding (ref value) => value.fmt (formatter),
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Boolean ( pub bool );


impl Boolean {
	
	#[ inline (always) ]
	pub fn not (&self) -> (Boolean) {
		(!self.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn and (&self, other : &Boolean) -> (Boolean) {
		(self.0 && other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn or (&self, other : &Boolean) -> (Boolean) {
		(self.0 || other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn xor (&self, other : &Boolean) -> (Boolean) {
		(self.0 ^ other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn nand (&self, other : &Boolean) -> (Boolean) {
		self.and (other) .not ()
	}
	
	#[ inline (always) ]
	pub fn nor (&self, other : &Boolean) -> (Boolean) {
		self.or (other) .not ()
	}
	
	#[ inline (always) ]
	pub fn nxor (&self, other : &Boolean) -> (Boolean) {
		self.xor (other) .not ()
	}
}


impl ops::Not for Boolean {
	type Output = Boolean;
	#[ inline (always) ]
	fn not (self) -> (Boolean) {
		Boolean::not (&self)
	}
}


impl fmt::Display for Boolean {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.0 {
			true => formatter.write_str ("#true"),
			false => formatter.write_str ("#false"),
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct NumberInteger ( pub i64 );


macro_rules! NumberInteger_fn_predicate {
	($delegate : ident) => (
		NumberInteger_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ inline (always) ]
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
		#[ inline (always) ]
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
		#[ inline (always) ]
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
		#[ inline (always) ]
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
		#[ inline (always) ]
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0 as f64, other.0) .into ()
		}
	);
}


impl NumberInteger {
	
	
	#[ inline (always) ]
	pub fn neg (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_neg (self.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0xd93d04db);
		}
	}
	
	#[ inline (always) ]
	pub fn abs (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_abs (self.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0x997daa2a);
		}
	}
	
	#[ inline (always) ]
	pub fn add (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_add (self.0, other.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0xd61736b6);
		}
	}
	
	#[ inline (always) ]
	pub fn sub (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_sub (self.0, other.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0x1e036be9);
		}
	}
	
	#[ inline (always) ]
	pub fn mul (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_mul (self.0, other.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0x32c5b516);
		}
	}
	
	#[ inline (always) ]
	pub fn div (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_div (self.0, other.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0xce26bc76);
		}
	}
	
	#[ inline (always) ]
	pub fn rem (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_rem (self.0, other.0) {
			return Ok (outcome.into ());
		} else {
			fail! (0xce26bc76);
		}
	}
	
	#[ inline (always) ]
	pub fn pow (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xdcca20dd);
		}
		return Ok (<i64>::pow (self.0, other as u32) .into ());
	}
	
	
	#[ inline (always) ]
	pub fn is_zero (&self) -> (bool) {
		(self.0 == 0)
	}
	
	#[ inline (always) ]
	pub fn is_even (&self) -> (bool) {
		((self.0 & 1) == 0)
	}
	
	#[ inline (always) ]
	pub fn is_odd (&self) -> (bool) {
		((self.0 & 1) != 0)
	}
	
	
	#[ inline (always) ]
	pub fn bitnot (&self) -> (NumberInteger) {
		(!self.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn bitand (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 & other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn bitor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 | other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn bitxor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 ^ other.0) .into ()
	}
	
	#[ inline (always) ]
	pub fn bitnand (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitand (other) .bitnot ()
	}
	
	#[ inline (always) ]
	pub fn bitnor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitor (other) .bitnot ()
	}
	
	#[ inline (always) ]
	pub fn bitnxor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitxor (other) .bitnot ()
	}
	
	
	#[ inline (always) ]
	pub fn shl (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xb84272a0);
		}
		if let Some (outcome) = <i64>::checked_shl (self.0, other as u32) {
			return Ok (outcome.into ());
		} else {
			fail! (0x734e69d8);
		}
	}
	
	#[ inline (always) ]
	pub fn shr (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x26d90f55);
		}
		if let Some (outcome) = <i64>::checked_shr (self.0, other as u32) {
			return Ok (outcome.into ());
		} else {
			fail! (0xc3bb81a9);
		}
	}
	
	#[ inline (always) ]
	pub fn rotate_left (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xe2038e82);
		}
		return Ok ((<i64>::rotate_left (self.0, other as u32)) .into ());
	}
	
	#[ inline (always) ]
	pub fn rotate_right (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x1d868231);
		}
		return Ok ((<i64>::rotate_right (self.0, other as u32)) .into ());
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
	#[ inline (always) ]
	fn neg (self) -> (Outcome<NumberInteger>) {
		NumberInteger::neg (&self)
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Add<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn add (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::add (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Sub<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn sub (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::sub (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Mul<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn mul (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::mul (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Div<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn div (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::div (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Rem<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn rem (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::rem (&self, &other.into ())
	}
}


impl ops::Not for NumberInteger {
	type Output = NumberInteger;
	#[ inline (always) ]
	fn not (self) -> (NumberInteger) {
		NumberInteger::bitnot (&self)
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::BitAnd<IntoNumberInteger> for NumberInteger {
	type Output = NumberInteger;
	#[ inline (always) ]
	fn bitand (self, other : IntoNumberInteger) -> (NumberInteger) {
		NumberInteger::bitand (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::BitOr<IntoNumberInteger> for NumberInteger {
	type Output = NumberInteger;
	#[ inline (always) ]
	fn bitor (self, other : IntoNumberInteger) -> (NumberInteger) {
		NumberInteger::bitor (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::BitXor<IntoNumberInteger> for NumberInteger {
	type Output = NumberInteger;
	#[ inline (always) ]
	fn bitxor (self, other : IntoNumberInteger) -> (NumberInteger) {
		NumberInteger::bitxor (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Shl<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn shl (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::shl (&self, &other.into ())
	}
}

impl <IntoNumberInteger : StdInto<NumberInteger>> ops::Shr<IntoNumberInteger> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	#[ inline (always) ]
	fn shr (self, other : IntoNumberInteger) -> (Outcome<NumberInteger>) {
		NumberInteger::shr (&self, &other.into ())
	}
}


impl fmt::Display for NumberInteger {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug) ]
pub struct NumberReal ( pub f64 );


macro_rules! NumberReal_fn_predicate {
	($delegate : ident) => (
		NumberReal_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		#[ inline (always) ]
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
		#[ inline (always) ]
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
		#[ inline (always) ]
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0, other.0) .into ()
		}
	);
}


impl NumberReal {
	
	
	#[ inline (always) ]
	pub fn neg (&self) -> (NumberReal) {
		(-self.0).into ()
	}
	
	#[ inline (always) ]
	pub fn add (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 + other.0).into ()
	}
	
	#[ inline (always) ]
	pub fn sub (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 - other.0).into ()
	}
	
	#[ inline (always) ]
	pub fn mul (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 * other.0).into ()
	}
	
	#[ inline (always) ]
	pub fn div (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 / other.0).into ()
	}
	
	#[ inline (always) ]
	pub fn rem (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 % other.0).into ()
	}
	
	
	#[ inline (always) ]
	pub fn is_zero (&self) -> (bool) {
		(self.0 == 0.0)
	}
	
	#[ inline (always) ]
	pub fn is_even (&self) -> (bool) {
		((self.0 % 2.0) == 0.0)
	}
	
	#[ inline (always) ]
	pub fn is_odd (&self) -> (bool) {
		((self.0 % 2.0) != 0.0)
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
	#[ inline (always) ]
	fn neg (self) -> (NumberReal) {
		NumberReal::neg (&self)
	}
}

impl <IntoNumberReal : StdInto<NumberReal>> ops::Add<IntoNumberReal> for NumberReal {
	type Output = NumberReal;
	#[ inline (always) ]
	fn add (self, other : IntoNumberReal) -> (NumberReal) {
		NumberReal::add (&self, &other.into ())
	}
}

impl <IntoNumberReal : StdInto<NumberReal>> ops::Sub<IntoNumberReal> for NumberReal {
	type Output = NumberReal;
	#[ inline (always) ]
	fn sub (self, other : IntoNumberReal) -> (NumberReal) {
		NumberReal::sub (&self, &other.into ())
	}
}

impl <IntoNumberReal : StdInto<NumberReal>> ops::Mul<IntoNumberReal> for NumberReal {
	type Output = NumberReal;
	#[ inline (always) ]
	fn mul (self, other : IntoNumberReal) -> (NumberReal) {
		NumberReal::mul (&self, &other.into ())
	}
}

impl <IntoNumberReal : StdInto<NumberReal>> ops::Div<IntoNumberReal> for NumberReal {
	type Output = NumberReal;
	#[ inline (always) ]
	fn div (self, other : IntoNumberReal) -> (NumberReal) {
		NumberReal::div (&self, &other.into ())
	}
}

impl <IntoNumberReal : StdInto<NumberReal>> ops::Rem<IntoNumberReal> for NumberReal {
	type Output = NumberReal;
	#[ inline (always) ]
	fn rem (self, other : IntoNumberReal) -> (NumberReal) {
		NumberReal::rem (&self, &other.into ())
	}
}


impl cmp::Eq for NumberReal {}

impl cmp::PartialEq for NumberReal {
	#[ inline (always) ]
	fn eq (&self, other : &NumberReal) -> (bool) {
		self.0.to_bits () == other.0.to_bits ()
	}
}

impl hash::Hash for NumberReal {
	#[ inline (always) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u64 (self.0.to_bits ());
	}
}

impl fmt::Display for NumberReal {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Character ( pub char );


impl fmt::Display for Character {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		let character = self.0;
		match character {
			'!' ... '~' => { try! (formatter.write_str ("#\\")); try! (formatter.write_char (character)); },
			_ => try! (write! (formatter, "#\\x{:02x}", character as u32)),
		}
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct Symbol ( StdRc<StdString> );


impl fmt::Display for Symbol {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		if self.0.is_empty () {
			return formatter.write_str ("||");
		} else {
			// return formatter.write_str (self.0.as_str ());
			use std::fmt::Write;
			try! (formatter.write_char ('|'));
			for character in self.0.chars () {
				match character {
					'|' | '\\' => { try! (formatter.write_char ('\\')); try! (formatter.write_char (character)); },
					' ' ... '~' => try! (formatter.write_char (character)),
					_ => try! (write! (formatter, "#\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
			return Ok(());
		}
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct String ( StdRc<StdString> );


impl fmt::Display for String {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('"'));
		for character in self.0.chars () {
			match character {
				'"' | '\\' => { try! (formatter.write_char ('\\')); try! (formatter.write_char (character)); },
				' ' ... '~' => try! (formatter.write_char (character)),
				_ => try! (write! (formatter, "#\\x{:02x};", character as u32)),
			}
		}
		try! (formatter.write_char ('"'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct Bytes ( StdRc<StdVec<u8>> );


impl fmt::Display for Bytes {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#u8("));
		let mut is_first = true;
		for byte in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (write! (formatter, "{}", byte));
		}
		try! (formatter.write_char (')'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct Pair ( StdRc<(Value, Value)> );


impl Pair {
	
	#[ inline (always) ]
	pub fn left (&self) -> (&Value) {
		return &(self.0).0;
	}
	
	#[ inline (always) ]
	pub fn right (&self) -> (&Value) {
		return &(self.0).1;
	}
}


impl fmt::Display for Pair {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('('));
		let mut head = self;
		loop {
			try! ((head.0).0.fmt (formatter));
			match (head.0).1 {
				Value::Null => break,
				Value::Pair (ref tail) => {
					try! (formatter.write_char (' '));
					head = tail;
				},
				_ => {
					try! (formatter.write_char (' '));
					try! (formatter.write_char ('.'));
					try! (formatter.write_char (' '));
					try! ((head.0).1.fmt (formatter));
					break;
				},
			}
		}
		try! (formatter.write_char (')'));
		return Ok(());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct Array ( StdRc<StdVec<Value>> );


impl fmt::Display for Array {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#("));
		let mut is_first = true;
		for element in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (element.fmt (formatter));
		}
		try! (formatter.write_char (')'));
		return Ok(());
	}
}




#[ inline (always) ]
pub fn boolean (value : bool) -> (Boolean) {
	Boolean (value)
}

#[ inline (always) ]
pub fn number_i64 (value : i64) -> (NumberInteger) {
	NumberInteger (value)
}

#[ inline (always) ]
pub fn number_f64 (value : f64) -> (NumberReal) {
	NumberReal (value)
}

#[ inline (always) ]
pub fn character (value : char) -> (Character) {
	Character (value)
}




#[ inline (always) ]
pub fn symbol (value : StdString) -> (Symbol) {
	Symbol (StdRc::new (value))
}

#[ inline (always) ]
pub fn string (value : StdString) -> (String) {
	String (StdRc::new (value))
}




#[ inline (always) ]
pub fn symbol_from_slice (value : &str) -> (Symbol) {
	symbol (StdString::from (value))
}

#[ inline (always) ]
pub fn string_from_slice (value : &str) -> (String) {
	string (StdString::from (value))
}




#[ inline (always) ]
pub fn symbol_from_characters (slice : &[char]) -> (Symbol) {
	let mut value = StdString::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return symbol (StdString::from (value));
}

#[ inline (always) ]
pub fn string_from_characters (slice : &[char]) -> (String) {
	let mut value = StdString::with_capacity (slice.len ());
	for character in slice {
		value.push (*character);
	}
	return string (StdString::from (value));
}




#[ inline (always) ]
pub fn bytes_from_slice (slice : &[u8]) -> (Bytes) {
	Bytes (StdRc::new (slice.to_vec ()))
}

#[ inline (always) ]
pub fn array_from_slice (slice : &[Value]) -> (Array) {
	Array (StdRc::new (slice.to_vec ()))
}




#[ inline (always) ]
pub fn list_from_slice (slice : &[Value]) -> (Value) {
	list_from_slice_2 (slice, Value::Null)
}

#[ inline (always) ]
pub fn list_from_slice_2 (slice : &[Value], continuation : Value) -> (Value) {
	let mut head = continuation;
	for value in slice.iter () .rev () {
		head = Value::Pair (Pair (StdRc::new ((value.clone (), head))));
	}
	return head;
}

