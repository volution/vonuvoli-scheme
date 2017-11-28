

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::BooleanPrimitive0;
	pub use super::BooleanPrimitive1;
	pub use super::BooleanPrimitive2;
	pub use super::BooleanPrimitiveN;
	
	pub use super::boolean_primitive_0_evaluate;
	pub use super::boolean_primitive_1_evaluate;
	pub use super::boolean_primitive_2_evaluate;
	pub use super::boolean_primitive_n_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive0 {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive1 {
	
	Not,
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive2 {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
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




pub fn boolean_primitive_0_evaluate (primitive : BooleanPrimitive0) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		BooleanPrimitive0::And =>
			TRUE,
		
		BooleanPrimitive0::Or =>
			FALSE,
		
		BooleanPrimitive0::Xor =>
			TRUE,
		
		BooleanPrimitive0::Nand =>
			FALSE,
		
		BooleanPrimitive0::Nor =>
			TRUE,
		
		BooleanPrimitive0::Nxor =>
			FALSE,
		
	};
	
	succeed! (output.into ());
}




pub fn boolean_primitive_1_evaluate (primitive : BooleanPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	
	let input_1 = try_as_boolean_ref! (input_1);
	
	let output = match primitive {
		
		BooleanPrimitive1::Not =>
			input_1.not (),
		
		BooleanPrimitive1::And =>
			input_1.clone (),
		
		BooleanPrimitive1::Or =>
			input_1.clone (),
		
		BooleanPrimitive1::Xor =>
			FALSE.into (),
		
		BooleanPrimitive1::Nand =>
			input_1.not (),
		
		BooleanPrimitive1::Nor =>
			input_1.not (),
		
		BooleanPrimitive1::Nxor =>
			TRUE.into (),
		
	};
	
	succeed! (output.into ());
}




pub fn boolean_primitive_2_evaluate (primitive : BooleanPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	
	let input_1 = try_as_boolean_ref! (input_1);
	let input_2 = try_as_boolean_ref! (input_2);
	
	let output = match primitive {
		
		BooleanPrimitive2::And =>
			input_1.and (input_2),
		
		BooleanPrimitive2::Or =>
			input_1.or (input_2),
		
		BooleanPrimitive2::Xor =>
			input_1.xor (input_2),
		
		BooleanPrimitive2::Nand =>
			input_1.nand (input_2),
		
		BooleanPrimitive2::Nor =>
			input_1.nor (input_2),
		
		BooleanPrimitive2::Nxor =>
			input_1.nxor (input_2),
		
	};
	
	succeed! (output.into ());
}




pub fn boolean_primitive_n_evaluate (primitive : BooleanPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	let mut output = match primitive {
		
		BooleanPrimitiveN::And | BooleanPrimitiveN::Nand =>
			TRUE,
		
		BooleanPrimitiveN::Or | BooleanPrimitiveN::Nor =>
			FALSE,
		
		BooleanPrimitiveN::Xor | BooleanPrimitiveN::Nxor =>
			FALSE,
		
	};
	
	for input in inputs {
		let input = try_as_boolean_ref! (input);
		output = match primitive {
			
			BooleanPrimitiveN::And | BooleanPrimitiveN::Nand =>
				output.and (input),
			
			BooleanPrimitiveN::Or | BooleanPrimitiveN::Nor =>
				output.or (input),
			
			BooleanPrimitiveN::Xor | BooleanPrimitiveN::Nxor =>
				output.xor (input),
			
		}
	}
	
	output = match primitive {
		
		BooleanPrimitiveN::And | BooleanPrimitiveN::Or =>
			output,
		
		BooleanPrimitiveN::Nand | BooleanPrimitiveN::Nor =>
			output.not (),
		
		BooleanPrimitiveN::Xor =>
			output.xor (&FALSE),
		
		BooleanPrimitiveN::Nxor =>
			output.xor (&FALSE) .not (),
		
	};
	
	succeed! (output.into ());
}

