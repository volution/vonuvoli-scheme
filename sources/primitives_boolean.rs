

use super::constants::exports::*;
use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::BooleanPrimitive0;
	pub use super::BooleanPrimitive1;
	pub use super::BooleanPrimitive2;
	pub use super::BooleanPrimitive3;
	pub use super::BooleanPrimitive4;
	pub use super::BooleanPrimitive5;
	pub use super::BooleanPrimitiveN;
	
	pub use super::boolean_primitive_0_evaluate;
	pub use super::boolean_primitive_1_evaluate;
	pub use super::boolean_primitive_2_evaluate;
	pub use super::boolean_primitive_3_evaluate;
	pub use super::boolean_primitive_4_evaluate;
	pub use super::boolean_primitive_5_evaluate;
	pub use super::boolean_primitive_n_evaluate;
	
	pub use super::boolean_primitive_n_alternative_0;
	pub use super::boolean_primitive_n_alternative_1;
	pub use super::boolean_primitive_n_alternative_2;
	pub use super::boolean_primitive_n_alternative_3;
	pub use super::boolean_primitive_n_alternative_4;
	pub use super::boolean_primitive_n_alternative_5;
	
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
pub enum BooleanPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BooleanPrimitive5 {}


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




pub fn boolean_primitive_3_evaluate (primitive : BooleanPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn boolean_primitive_4_evaluate (primitive : BooleanPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn boolean_primitive_5_evaluate (primitive : BooleanPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {}
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




pub fn boolean_primitive_n_alternative_0 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive0>) {
	match primitive {
		BooleanPrimitiveN::And =>
			Some (BooleanPrimitive0::And),
		BooleanPrimitiveN::Or =>
			Some (BooleanPrimitive0::Or),
		BooleanPrimitiveN::Xor =>
			Some (BooleanPrimitive0::Xor),
		BooleanPrimitiveN::Nand =>
			Some (BooleanPrimitive0::Nand),
		BooleanPrimitiveN::Nor =>
			Some (BooleanPrimitive0::Nor),
		BooleanPrimitiveN::Nxor =>
			Some (BooleanPrimitive0::Nxor),
	}
}


pub fn boolean_primitive_n_alternative_1 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive1>) {
	match primitive {
		BooleanPrimitiveN::And =>
			Some (BooleanPrimitive1::And),
		BooleanPrimitiveN::Or =>
			Some (BooleanPrimitive1::Or),
		BooleanPrimitiveN::Xor =>
			Some (BooleanPrimitive1::Xor),
		BooleanPrimitiveN::Nand =>
			Some (BooleanPrimitive1::Nand),
		BooleanPrimitiveN::Nor =>
			Some (BooleanPrimitive1::Nor),
		BooleanPrimitiveN::Nxor =>
			Some (BooleanPrimitive1::Nxor),
	}
}


pub fn boolean_primitive_n_alternative_2 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive2>) {
	match primitive {
		BooleanPrimitiveN::And =>
			Some (BooleanPrimitive2::And),
		BooleanPrimitiveN::Or =>
			Some (BooleanPrimitive2::Or),
		BooleanPrimitiveN::Xor =>
			Some (BooleanPrimitive2::Xor),
		BooleanPrimitiveN::Nand =>
			Some (BooleanPrimitive2::Nand),
		BooleanPrimitiveN::Nor =>
			Some (BooleanPrimitive2::Nor),
		BooleanPrimitiveN::Nxor =>
			Some (BooleanPrimitive2::Nxor),
	}
}


pub fn boolean_primitive_n_alternative_3 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive3>) {
	match primitive {
		_ =>
			None,
	}
}


pub fn boolean_primitive_n_alternative_4 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive4>) {
	match primitive {
		_ =>
			None,
	}
}


pub fn boolean_primitive_n_alternative_5 (primitive : BooleanPrimitiveN) -> (Option<BooleanPrimitive5>) {
	match primitive {
		_ =>
			None,
	}
}

