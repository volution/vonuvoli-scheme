

use super::contexts::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::SyntaxPrimitive1;
	pub use super::SyntaxPrimitive2;
	pub use super::SyntaxPrimitive3;
	pub use super::SyntaxPrimitiveN;
	pub use super::SyntaxPrimitive;
	
	pub use super::syntax_primitive_1_evaluate;
	pub use super::syntax_primitive_2_evaluate;
	pub use super::syntax_primitive_3_evaluate;
	pub use super::syntax_primitive_n_evaluate;
	pub use super::syntax_primitive_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxPrimitive {
	Primitive1 ( SyntaxPrimitive1 ),
	Primitive2 ( SyntaxPrimitive2 ),
	Primitive3 ( SyntaxPrimitive3 ),
	PrimitiveN ( SyntaxPrimitiveN ),
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxPrimitive1 {
	
	Quote,
	QuasiQuote,
	UnQuote,
	UnQuoteSplicing,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxPrimitive2 {
	
	Define,
	DefineValues,
	
	Set,
	SetValues,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxPrimitive3 {
	
	If,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxPrimitiveN {
	
	Lambda,
	
	Begin,
	
	Cond,
	Case,
	When,
	Unless,
	
	Let,
	LetValues,
	
	Do,
	
	And,
	Or,
	
}




#[ inline (always) ]
pub fn syntax_primitive_1_evaluate (_primitive : SyntaxPrimitive1, _input : &Expression, _evaluator : &mut Evaluator, _context : &mut Context) -> (Outcome<Value>) {
	return failed_unimplemented! (0xc0c18893);
}


#[ inline (always) ]
pub fn syntax_primitive_2_evaluate (_primitive : SyntaxPrimitive2, _input_1 : &Expression, _input_2 : &Expression, _evaluator : &mut Evaluator, _context : &mut Context) -> (Outcome<Value>) {
	return failed_unimplemented! (0xc0c18893);
}


#[ inline (always) ]
pub fn syntax_primitive_3_evaluate (_primitive : SyntaxPrimitive3, _input_1 : &Expression, _input_2 : &Expression, _input_3 : &Expression, _evaluator : &mut Evaluator, _context : &mut Context) -> (Outcome<Value>) {
	return failed_unimplemented! (0xc0c18893);
}


#[ inline (always) ]
pub fn syntax_primitive_n_evaluate (_primitive : SyntaxPrimitiveN, _input : &[Expression], _evaluator : &mut Evaluator, _context : &mut Context) -> (Outcome<Value>) {
	return failed_unimplemented! (0xc0c18893);
}




#[ inline (always) ]
pub fn syntax_primitive_evaluate (primitive : SyntaxPrimitive, inputs : &[Expression], evaluator : &mut Evaluator, context : &mut Context) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		SyntaxPrimitive::Primitive1 (primitive) =>
			if inputs_count != 1 {
				return failed! (0xc7837cc4);
			} else {
				return syntax_primitive_1_evaluate (primitive, &inputs[0], evaluator, context);
			},
		SyntaxPrimitive::Primitive2 (primitive) =>
			if inputs_count != 2 {
				return failed! (0xb92232f2);
			} else {
				return syntax_primitive_2_evaluate (primitive, &inputs[0], &inputs[1], evaluator, context);
			},
		SyntaxPrimitive::Primitive3 (primitive) =>
			if inputs_count != 3 {
				return failed! (0x18d7a5f8);
			} else {
				return syntax_primitive_3_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], evaluator, context);
			},
		SyntaxPrimitive::PrimitiveN (primitive) =>
			return syntax_primitive_n_evaluate (primitive, inputs, evaluator, context),
	}
}

