

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::BooleanPrimitive1;
	pub use super::BooleanPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive1 {
	Negate,
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitiveN {
	And,
	Or,
	Xor,
	Nand,
	Nor,
	Nxor,
}




#[ inline (always) ]
pub fn boolean_primitive_1_evaluate (primitive : BooleanPrimitive1, input : &Value) -> (Outcome<Value>) {
	let input = enforce_boolean_unwrap! (*input);
	let output = match primitive {
		BooleanPrimitive1::Negate => input.not (),
	};
	return Ok (output.into ());
}


#[ inline (always) ]
pub fn boolean_primitive_n_evaluate (primitive : BooleanPrimitiveN, input : &[Value]) -> (Outcome<Value>) {
	
	let mut output = match primitive {
		BooleanPrimitiveN::And | BooleanPrimitiveN::Nand => TRUE,
		BooleanPrimitiveN::Or | BooleanPrimitiveN::Nor => FALSE,
		BooleanPrimitiveN::Xor | BooleanPrimitiveN::Nxor => FALSE,
	};
	
	for input in input {
		let input = enforce_boolean_unwrap! (*input);
		match primitive {
			BooleanPrimitiveN::And | BooleanPrimitiveN::Nand => output = output.and (input),
			BooleanPrimitiveN::Or | BooleanPrimitiveN::Nor => output = output.or (input),
			BooleanPrimitiveN::Xor | BooleanPrimitiveN::Nxor => output = output.xor (input),
		}
	}
	
	match primitive {
		BooleanPrimitiveN::And | BooleanPrimitiveN::Or => {},
		BooleanPrimitiveN::Nand | BooleanPrimitiveN::Nor => output = output.not (),
		BooleanPrimitiveN::Xor => output = output.xor (&FALSE),
		BooleanPrimitiveN::Nxor => output = output.xor (&FALSE) .not (),
	};
	
	return Ok (output.into ());
}

