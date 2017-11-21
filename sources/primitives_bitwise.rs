

use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::BitwisePrimitive1;
	pub use super::BitwisePrimitive2;
	pub use super::BitwisePrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive1 {
	
	Complement,
	
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive2 {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
	ShiftLeft,
	ShiftRight,
	
	RotateLeft,
	RotateRight,
	
}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitiveN {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}




pub fn bitwise_primitive_1_evaluate (primitive : BitwisePrimitive1, input : &Value) -> (Outcome<Value>) {
	
	let input = try_as_number_integer_ref! (input);
	
	let output = match primitive {
		BitwisePrimitive1::Complement => input.bitnot (),
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_2_evaluate (primitive : BitwisePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	
	let input_1 = try_as_number_integer_ref! (input_1);
	let input_2 = try_as_number_integer_ref! (input_2);
	
	let output = match primitive {
		
		BitwisePrimitive2::And =>
			input_1.bitand (input_2),
		
		BitwisePrimitive2::Or =>
			input_1.bitor (input_2),
		
		BitwisePrimitive2::Xor =>
			input_1.bitxor (input_2),
		
		BitwisePrimitive2::Nand =>
			input_1.bitnand (input_2),
		
		BitwisePrimitive2::Nor =>
			input_1.bitnor (input_2),
		
		BitwisePrimitive2::Nxor =>
			input_1.bitnxor (input_2),
		
		BitwisePrimitive2::ShiftLeft =>
			try! (input_1.shl (input_2)),
		
		BitwisePrimitive2::ShiftRight =>
			try! (input_1.shr (input_2)),
		
		BitwisePrimitive2::RotateLeft =>
			try! (input_1.rotate_left (input_2)),
		
		BitwisePrimitive2::RotateRight =>
			try! (input_1.rotate_right (input_2)),
		
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_n_evaluate (primitive : BitwisePrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	let mut output = match primitive {
		
		BitwisePrimitiveN::And | BitwisePrimitiveN::Nand =>
			(<u64>::max_value () as i64).into (),
		
		BitwisePrimitiveN::Or | BitwisePrimitiveN::Nor =>
			ZERO,
		
		BitwisePrimitiveN::Xor | BitwisePrimitiveN::Nxor =>
			ZERO,
		
	};
	
	for input in inputs {
		let input = try_as_number_integer_ref! (input);
		
		output = match primitive {
			
			BitwisePrimitiveN::And | BitwisePrimitiveN::Nand =>
				output.bitand (input),
			
			BitwisePrimitiveN::Or | BitwisePrimitiveN::Nor =>
				output.bitor (input),
			
			BitwisePrimitiveN::Xor | BitwisePrimitiveN::Nxor =>
				output.bitxor (input),
			
		}
	}
	
	output = match primitive {
		
		BitwisePrimitiveN::And | BitwisePrimitiveN::Or =>
			output,
		
		BitwisePrimitiveN::Nand | BitwisePrimitiveN::Nor =>
			output.bitnot (),
		
		BitwisePrimitiveN::Xor =>
			output.bitxor (&ZERO),
		
		BitwisePrimitiveN::Nxor =>
			output.bitxor (&ZERO) .bitnot (),
		
	};
	
	succeed! (output.into ());
}

