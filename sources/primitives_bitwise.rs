

use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::BitwisePrimitive0;
	pub use super::BitwisePrimitive1;
	pub use super::BitwisePrimitive2;
	pub use super::BitwisePrimitive3;
	pub use super::BitwisePrimitive4;
	pub use super::BitwisePrimitive5;
	pub use super::BitwisePrimitiveN;
	pub use super::BitwisePrimitiveV;
	
	pub use super::bitwise_primitive_0_evaluate;
	pub use super::bitwise_primitive_1_evaluate;
	pub use super::bitwise_primitive_2_evaluate;
	pub use super::bitwise_primitive_3_evaluate;
	pub use super::bitwise_primitive_4_evaluate;
	pub use super::bitwise_primitive_5_evaluate;
	pub use super::bitwise_primitive_n_evaluate;
	
	pub use super::bitwise_primitive_v_alternative_0;
	pub use super::bitwise_primitive_v_alternative_1;
	pub use super::bitwise_primitive_v_alternative_2;
	pub use super::bitwise_primitive_v_alternative_3;
	pub use super::bitwise_primitive_v_alternative_4;
	pub use super::bitwise_primitive_v_alternative_5;
	pub use super::bitwise_primitive_v_alternative_n;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::bitwise_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (BitwisePrimitive0, (procedure, 0), {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
});


def_primitives_enum! (BitwisePrimitive1, (procedure, 1), {
	
	Complement,
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
});


def_primitives_enum! (BitwisePrimitive2, (procedure, 2), {
	
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
	
});


def_primitives_enum! (BitwisePrimitive3, (procedure, 3), {});


def_primitives_enum! (BitwisePrimitive4, (procedure, 4), {});


def_primitives_enum! (BitwisePrimitive5, (procedure, 5), {});


def_primitives_enum! (BitwisePrimitiveN, (procedure, n), {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
});


def_primitives_enum! (BitwisePrimitiveV, (procedure, v), {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
});




impl_procedure_primitive_enum_matrix! (
		(BitwisePrimitive0, bitwise_primitive_0_evaluate, bitwise_primitive_0_attributes, bitwise_primitive_v_alternative_0),
		(BitwisePrimitive1, bitwise_primitive_1_evaluate, bitwise_primitive_1_attributes, bitwise_primitive_v_alternative_1),
		(BitwisePrimitive2, bitwise_primitive_2_evaluate, bitwise_primitive_2_attributes, bitwise_primitive_v_alternative_2),
		(BitwisePrimitive3, bitwise_primitive_3_evaluate, bitwise_primitive_3_attributes, bitwise_primitive_v_alternative_3),
		(BitwisePrimitive4, bitwise_primitive_4_evaluate, bitwise_primitive_4_attributes, bitwise_primitive_v_alternative_4),
		(BitwisePrimitive5, bitwise_primitive_5_evaluate, bitwise_primitive_5_attributes, bitwise_primitive_v_alternative_5),
		(BitwisePrimitiveN, bitwise_primitive_n_evaluate, bitwise_primitive_n_attributes, bitwise_primitive_v_alternative_n),
		(BitwisePrimitiveV, bitwise_primitive_v_evaluate, bitwise_primitive_v_attributes),
	);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_0_evaluate (primitive : BitwisePrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		BitwisePrimitive0::And =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Or =>
			ZERO,
		
		BitwisePrimitive0::Xor =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Nand =>
			ZERO,
		
		BitwisePrimitive0::Nor =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Nxor =>
			ZERO,
		
	};
	
	succeed! (output.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_1_evaluate (primitive : BitwisePrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let input_1 = try_as_number_integer_ref! (input_1);
	
	let output = match primitive {
		
		BitwisePrimitive1::Complement =>
			input_1.bitnot (),
		
		BitwisePrimitive1::And =>
			input_1.clone (),
		
		BitwisePrimitive1::Or =>
			input_1.clone (),
		
		BitwisePrimitive1::Xor =>
			ZERO,
		
		BitwisePrimitive1::Nand =>
			input_1.bitnot (),
		
		BitwisePrimitive1::Nor =>
			input_1.bitnot (),
		
		BitwisePrimitive1::Nxor =>
			ZERO.bitnot (),
		
	};
	
	succeed! (output.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_2_evaluate (primitive : BitwisePrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_3_evaluate (primitive : BitwisePrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_4_evaluate (primitive : BitwisePrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_5_evaluate (primitive : BitwisePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_n_evaluate (primitive : BitwisePrimitiveN, inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let mut output = match primitive {
		
		BitwisePrimitiveN::And | BitwisePrimitiveN::Nand =>
			(<u64>::max_value () as i64).into (),
		
		BitwisePrimitiveN::Or | BitwisePrimitiveN::Nor =>
			ZERO,
		
		BitwisePrimitiveN::Xor | BitwisePrimitiveN::Nxor =>
			ZERO,
		
	};
	
	for input in inputs {
		let input = input.as_ref ();
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




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_0 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive0>) {
	match primitive {
		BitwisePrimitiveV::And =>
			Some (BitwisePrimitive0::And),
		BitwisePrimitiveV::Or =>
			Some (BitwisePrimitive0::Or),
		BitwisePrimitiveV::Xor =>
			Some (BitwisePrimitive0::Xor),
		BitwisePrimitiveV::Nand =>
			Some (BitwisePrimitive0::Nand),
		BitwisePrimitiveV::Nor =>
			Some (BitwisePrimitive0::Nor),
		BitwisePrimitiveV::Nxor =>
			Some (BitwisePrimitive0::Nxor),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_1 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive1>) {
	match primitive {
		BitwisePrimitiveV::And =>
			Some (BitwisePrimitive1::And),
		BitwisePrimitiveV::Or =>
			Some (BitwisePrimitive1::Or),
		BitwisePrimitiveV::Xor =>
			Some (BitwisePrimitive1::Xor),
		BitwisePrimitiveV::Nand =>
			Some (BitwisePrimitive1::Nand),
		BitwisePrimitiveV::Nor =>
			Some (BitwisePrimitive1::Nor),
		BitwisePrimitiveV::Nxor =>
			Some (BitwisePrimitive1::Nxor),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_2 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive2>) {
	match primitive {
		BitwisePrimitiveV::And =>
			Some (BitwisePrimitive2::And),
		BitwisePrimitiveV::Or =>
			Some (BitwisePrimitive2::Or),
		BitwisePrimitiveV::Xor =>
			Some (BitwisePrimitive2::Xor),
		BitwisePrimitiveV::Nand =>
			Some (BitwisePrimitive2::Nand),
		BitwisePrimitiveV::Nor =>
			Some (BitwisePrimitive2::Nor),
		BitwisePrimitiveV::Nxor =>
			Some (BitwisePrimitive2::Nxor),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_3 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive3>) {
	match primitive {
		BitwisePrimitiveV::And =>
			None,
		BitwisePrimitiveV::Or =>
			None,
		BitwisePrimitiveV::Xor =>
			None,
		BitwisePrimitiveV::Nand =>
			None,
		BitwisePrimitiveV::Nor =>
			None,
		BitwisePrimitiveV::Nxor =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_4 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive4>) {
	match primitive {
		BitwisePrimitiveV::And =>
			None,
		BitwisePrimitiveV::Or =>
			None,
		BitwisePrimitiveV::Xor =>
			None,
		BitwisePrimitiveV::Nand =>
			None,
		BitwisePrimitiveV::Nor =>
			None,
		BitwisePrimitiveV::Nxor =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_5 (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitive5>) {
	match primitive {
		BitwisePrimitiveV::And =>
			None,
		BitwisePrimitiveV::Or =>
			None,
		BitwisePrimitiveV::Xor =>
			None,
		BitwisePrimitiveV::Nand =>
			None,
		BitwisePrimitiveV::Nor =>
			None,
		BitwisePrimitiveV::Nxor =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_v_alternative_n (primitive : BitwisePrimitiveV) -> (Option<BitwisePrimitiveN>) {
	match primitive {
		BitwisePrimitiveV::And =>
			Some (BitwisePrimitiveN::And),
		BitwisePrimitiveV::Or =>
			Some (BitwisePrimitiveN::Or),
		BitwisePrimitiveV::Xor =>
			Some (BitwisePrimitiveN::Xor),
		BitwisePrimitiveV::Nand =>
			Some (BitwisePrimitiveN::Nand),
		BitwisePrimitiveV::Nor =>
			Some (BitwisePrimitiveN::Nor),
		BitwisePrimitiveV::Nxor =>
			Some (BitwisePrimitiveN::Nxor),
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_0_attributes (_primitive : BitwisePrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_1_attributes (_primitive : BitwisePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_2_attributes (_primitive : BitwisePrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_3_attributes (_primitive : BitwisePrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_3);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_4_attributes (_primitive : BitwisePrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_5_attributes (_primitive : BitwisePrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn bitwise_primitive_n_attributes (_primitive : BitwisePrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

