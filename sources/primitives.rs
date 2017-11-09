

use super::errors::exports::*;
use super::values::exports::*;

use super::primitives_arithmetic::*;
use super::primitives_bitwise::*;
use super::primitives_boolean::*;
use super::primitives_types::*;




pub mod exports {
	
	pub use super::primitive_0_evaluate;
	pub use super::primitive_1_evaluate;
	pub use super::primitive_2_evaluate;
	pub use super::primitive_n_evaluate;
	pub use super::primitive_evaluate;
	
	pub use super::Primitive;
	pub use super::Primitive0;
	pub use super::Primitive1;
	pub use super::Primitive2;
	pub use super::PrimitiveN;
	
	pub use super::super::primitives_arithmetic::*;
	pub use super::super::primitives_bitwise::*;
	pub use super::super::primitives_boolean::*;
	pub use super::super::primitives_types::*;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Primitive {
	Primitive0 ( Primitive0 ),
	Primitive1 ( Primitive1 ),
	Primitive2 ( Primitive2 ),
	PrimitiveN ( PrimitiveN ),
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Primitive0 {
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Primitive1 {
	Type ( TypePrimitive1 ),
	Boolean ( BooleanPrimitive1 ),
	Arithmetic ( ArithmeticPrimitive1 ),
	Bitwise ( BitwisePrimitive1 ),
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Primitive2 {
	Arithmetic ( ArithmeticPrimitive2 ),
	Bitwise ( BitwisePrimitive2 ),
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum PrimitiveN {
	Boolean ( BooleanPrimitiveN ),
	Arithmetic ( ArithmeticPrimitiveN ),
	Bitwise ( BitwisePrimitiveN ),
}




#[ inline (always) ]
pub fn primitive_0_evaluate (primitive : Primitive0) -> (Outcome<Value>) {
	unimplemented! (0xf5c28466)
}

#[ inline (always) ]
pub fn primitive_1_evaluate (primitive : Primitive1, input : &Value) -> (Outcome<Value>) {
	match primitive {
		Primitive1::Boolean (primitive) =>
			return boolean_primitive_1_evaluate (primitive, input),
		_ =>
			return unimplemented! (0x85e495c7)
	}
}

#[ inline (always) ]
pub fn primitive_2_evaluate (primitive : Primitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	unimplemented! (0x9ed223e5)
}

#[ inline (always) ]
pub fn primitive_n_evaluate (primitive : PrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	match primitive {
		PrimitiveN::Boolean (primitive) =>
			return boolean_primitive_n_evaluate (primitive, inputs),
		_ =>
			return unimplemented! (0xa1bd3a06)
	}
}


#[ inline (always) ]
pub fn primitive_evaluate (primitive : Primitive, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		Primitive::Primitive0 (primitive) =>
			if inputs_count != 0 {
				return failed! (0xabfe1f25);
			} else {
				return primitive_0_evaluate (primitive);
			},
		Primitive::Primitive1 (primitive) =>
			if inputs_count != 1 {
				return failed! (0x5bc94cf2);
			} else {
				return primitive_1_evaluate (primitive, &inputs[0]);
			},
		Primitive::Primitive2 (primitive) =>
			if inputs_count != 2 {
				return failed! (0xb1c56ed3);
			} else {
				return primitive_2_evaluate (primitive, &inputs[0], &inputs[1]);
			},
		Primitive::PrimitiveN (primitive) =>
			return primitive_n_evaluate (primitive, inputs),
	}
}

