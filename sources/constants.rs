

use super::values::exports::*;

use std::f64;




pub mod exports {
	pub use super::*;
}




pub const NULL : Value = Value::Null;
pub const VOID : Value = Value::Void;
pub const UNDEFINED : Value = Value::Undefined;

pub const TRUE : Boolean = Boolean (true);
pub const FALSE : Boolean = Boolean (false);

pub const ZERO : NumberInteger = NumberInteger (0);
pub const ONE : NumberInteger = NumberInteger (1);

pub const INF_POSITIVE : NumberReal = NumberReal (f64::INFINITY);
pub const INF_NEGATIVE : NumberReal = NumberReal (f64::NEG_INFINITY);
pub const NAN_POSITIVE : NumberReal = NumberReal (f64::NAN);
pub const NAN_NEGATIVE : NumberReal = NumberReal (f64::NAN);

