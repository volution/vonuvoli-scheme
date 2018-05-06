

use super::values::exports::*;

#[ allow (unused_imports) ]
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::*;
}




pub const NULL : ValueSingleton = ValueSingleton::Null;
pub const VOID : ValueSingleton = ValueSingleton::Void;
pub const UNDEFINED : ValueSingleton = ValueSingleton::Undefined;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const PORT_EOF : ValueSingleton = ValueSingleton::PortEof;

pub const TRUE : Boolean = Boolean (true);
pub const FALSE : Boolean = Boolean (false);


pub const NULL_VALUE : Value = Value::Singleton (VALUE_META_1, NULL, VALUE_META_2);
pub const VOID_VALUE : Value = Value::Singleton (VALUE_META_1, VOID, VALUE_META_2);
pub const UNDEFINED_VALUE : Value = Value::Singleton (VALUE_META_1, UNDEFINED, VALUE_META_2);
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const PORT_EOF_VALUE : Value = Value::Singleton (VALUE_META_1, PORT_EOF, VALUE_META_2);

pub const TRUE_VALUE : Value = Value::Boolean (VALUE_META_1, TRUE, VALUE_META_2);
pub const FALSE_VALUE : Value = Value::Boolean (VALUE_META_1, FALSE, VALUE_META_2);




pub const ZERO : NumberInteger = NumberInteger (0);
pub const ONE : NumberInteger = NumberInteger (1);

pub const ZERO_VALUE : Value = Value::NumberInteger (VALUE_META_1, ZERO, VALUE_META_2);
pub const ONE_VALUE : Value = Value::NumberInteger (VALUE_META_1, ONE, VALUE_META_2);

pub const ZERO_REAL_POSITIVE : NumberReal = NumberReal (0.0);
pub const ZERO_REAL_NEGATIVE : NumberReal = NumberReal (-0.0);
pub const ONE_REAL_POSITIVE : NumberReal = NumberReal (1.0);
pub const ONE_REAL_NEGATIVE : NumberReal = NumberReal (-1.0);

pub const INF_POSITIVE : NumberReal = NumberReal (f64::INFINITY);
pub const INF_NEGATIVE : NumberReal = NumberReal (f64::NEG_INFINITY);
pub const NAN_POSITIVE : NumberReal = NumberReal (f64::NAN);
pub const NAN_NEGATIVE : NumberReal = NumberReal (f64::NAN);

pub const EPSILON_POSITIVE : NumberReal = NumberReal (0f64 + f64::EPSILON);
pub const EPSILON_NEGATIVE : NumberReal = NumberReal (0f64 - f64::EPSILON);




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_0 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (0),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_1 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (1),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_2 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (2),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_3 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (3),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_4 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (4),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_5 : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Exact (5),
				output : ProcedureOutputAttributes::Constant,
			};

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
pub const CONSTANT_PROCEDURE_ATTRIBUTES_N : ProcedureAttributes =
		ProcedureAttributes {
				deterministic : true,
				arity : ProcedureArity::Unbounded,
				output : ProcedureOutputAttributes::Constant,
			};




#[ derive ( Copy, Clone, PartialEq, PartialOrd ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct Constant <Value> ( Value );

impl <Value : Copy> Constant<Value> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn value (&self) -> (Value) {
		self.0
	}
}


impl cmp::Eq for Constant<i16> {}

impl cmp::Ord for Constant<i16> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		i16::cmp (&self.0, &other.0)
	}
}

impl hash::Hash for Constant<i16> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_i16 (self.0);
	}
}


impl cmp::Eq for Constant<i32> {}

impl cmp::Ord for Constant<i32> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		i32::cmp (&self.0, &other.0)
	}
}

impl hash::Hash for Constant<i32> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_i32 (self.0);
	}
}


impl cmp::Eq for Constant<f32> {}

impl cmp::Ord for Constant<f32> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn cmp (&self, other : &Self) -> (cmp::Ordering) {
		if let Some (cmp) = f32::partial_cmp (&self.0, &other.0) {
			cmp
		} else {
			u32::cmp (&(self.0).to_bits (), &(other.0).to_bits ())
		}
	}
}

impl hash::Hash for Constant<f32> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u32 (self.0.to_bits ());
	}
}

