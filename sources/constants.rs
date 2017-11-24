

use super::values::exports::*;

use std::f64;




pub mod exports {
	pub use super::*;
}




pub const NULL : Value = Value::Null (VALUE_META_1, VALUE_META_2);
pub const VOID : Value = Value::Void (VALUE_META_1, VALUE_META_2);
pub const UNDEFINED : Value = Value::Undefined (VALUE_META_1, VALUE_META_2);

pub const TRUE : Boolean = Boolean (true);
pub const FALSE : Boolean = Boolean (false);

pub const ZERO : NumberInteger = NumberInteger (0);
pub const ONE : NumberInteger = NumberInteger (1);

pub const ZERO_REAL_POSITIVE : NumberReal = NumberReal (0.0);
pub const ZERO_REAL_NEGATIVE : NumberReal = NumberReal (-0.0);
pub const ONE_REAL_POSITIVE : NumberReal = NumberReal (1.0);
pub const ONE_REAL_NEGATIVE : NumberReal = NumberReal (-1.0);

pub const INF_POSITIVE : NumberReal = NumberReal (f64::INFINITY);
pub const INF_NEGATIVE : NumberReal = NumberReal (f64::NEG_INFINITY);
pub const NAN_POSITIVE : NumberReal = NumberReal (f64::NAN);
pub const NAN_NEGATIVE : NumberReal = NumberReal (f64::NAN);

