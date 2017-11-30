

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::SyntaxPrimitive0;
	pub use super::SyntaxPrimitive1;
	pub use super::SyntaxPrimitive2;
	pub use super::SyntaxPrimitive3;
	pub use super::SyntaxPrimitive4;
	pub use super::SyntaxPrimitive5;
	pub use super::SyntaxPrimitiveN;
	pub use super::SyntaxPrimitive;
	
	pub use super::syntax_primitive_0_evaluate;
	pub use super::syntax_primitive_1_evaluate;
	pub use super::syntax_primitive_2_evaluate;
	pub use super::syntax_primitive_3_evaluate;
	pub use super::syntax_primitive_4_evaluate;
	pub use super::syntax_primitive_5_evaluate;
	pub use super::syntax_primitive_n_evaluate;
	pub use super::syntax_primitive_evaluate;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive {
	
	Primitive0 ( SyntaxPrimitive0 ),
	Primitive1 ( SyntaxPrimitive1 ),
	Primitive2 ( SyntaxPrimitive2 ),
	Primitive3 ( SyntaxPrimitive3 ),
	Primitive4 ( SyntaxPrimitive4 ),
	Primitive5 ( SyntaxPrimitive5 ),
	PrimitiveN ( SyntaxPrimitiveN ),
	
	Auxiliary,
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive1 {
	
	Quote,
	QuasiQuote,
	UnQuote,
	UnQuoteSplicing,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive2 {
	
	If,
	
	Define,
	DefineValues,
	
	Set,
	SetValues,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive3 {
	
	If,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxPrimitiveN {
	
	And,
	Or,
	
	Begin,
	
	If,
	
	When,
	Unless,
	
	Cond,
	Case,
	
	Do,
	
	Locals,
	
	LetParallel,
	LetSequential,
	LetRecursiveParallel,
	LetRecursiveSequential,
	
	LetValuesParallel,
	LetValuesSequential,
	
	Define,
	
	Lambda,
	
}




pub fn syntax_primitive_0_evaluate (primitive : SyntaxPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn syntax_primitive_1_evaluate (primitive : SyntaxPrimitive1, _input_1 : &Expression, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		SyntaxPrimitive1::Quote | SyntaxPrimitive1::QuasiQuote | SyntaxPrimitive1::UnQuote | SyntaxPrimitive1::UnQuoteSplicing =>
			fail! (0xe7fceaf5),
		
	}
}




pub fn syntax_primitive_2_evaluate (primitive : SyntaxPrimitive2, _input_1 : &Expression, _input_2 : &Expression, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		SyntaxPrimitive2::If =>
			fail! (0x9581f453),
		
		SyntaxPrimitive2::Define | SyntaxPrimitive2::DefineValues =>
			fail! (0xf72ef0ed),
		
		SyntaxPrimitive2::Set | SyntaxPrimitive2::SetValues =>
			fail! (0xe6843905),
		
	}
}




pub fn syntax_primitive_3_evaluate (primitive : SyntaxPrimitive3, _input_1 : &Expression, _input_2 : &Expression, _input_3 : &Expression, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		SyntaxPrimitive3::If =>
			fail! (0x9eb5f5a1),
		
	}
}




pub fn syntax_primitive_4_evaluate (primitive : SyntaxPrimitive4, _input_1 : &Expression, _input_2 : &Expression, _input_3 : &Expression, _input_4 : &Expression, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn syntax_primitive_5_evaluate (primitive : SyntaxPrimitive5, _input_1 : &Expression, _input_2 : &Expression, _input_3 : &Expression, _input_4 : &Expression, _input_5 : &Expression, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn syntax_primitive_n_evaluate (primitive : SyntaxPrimitiveN, inputs : &[Expression], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		SyntaxPrimitiveN::And => {
			let mut output = TRUE.into ();
			for input in inputs {
				output = try! (evaluator.evaluate (input));
				if is_false (&output) {
					succeed! (output);
				}
			}
			succeed! (output);
		},
		
		SyntaxPrimitiveN::Or => {
			let mut output = FALSE.into ();
			for input in inputs {
				output = try! (evaluator.evaluate (input));
				if is_not_false (&output) {
					succeed! (output);
				}
			}
			succeed! (output);
		},
		
		SyntaxPrimitiveN::Begin =>
			fail! (0x5d19e13b),
		
		SyntaxPrimitiveN::If =>
			fail! (0x6fb1857a),
		
		SyntaxPrimitiveN::When | SyntaxPrimitiveN::Unless =>
			fail! (0x169ec95d),
		
		SyntaxPrimitiveN::Cond | SyntaxPrimitiveN::Case =>
			fail! (0x39b925db),
		
		SyntaxPrimitiveN::Do =>
			fail! (0xf5bd287f),
		
		SyntaxPrimitiveN::Locals =>
			fail! (0xc956c743),
		
		SyntaxPrimitiveN::LetParallel | SyntaxPrimitiveN::LetSequential | SyntaxPrimitiveN::LetRecursiveParallel | SyntaxPrimitiveN::LetRecursiveSequential =>
			fail! (0x136922d0),
		
		SyntaxPrimitiveN::LetValuesParallel | SyntaxPrimitiveN::LetValuesSequential =>
			fail! (0x7b3a21ac),
		
		SyntaxPrimitiveN::Define =>
			fail! (0x33ec681e),
		
		SyntaxPrimitiveN::Lambda =>
			fail! (0xd45f4e3b),
		
	}
}




pub fn syntax_primitive_evaluate (primitive : SyntaxPrimitive, inputs : &[Expression], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		SyntaxPrimitive::Primitive0 (primitive) =>
			if inputs_count == 0 {
				return syntax_primitive_0_evaluate (primitive, evaluator);
			} else {
				fail! (0x79d14403);
			},
		
		SyntaxPrimitive::Primitive1 (primitive) =>
			if inputs_count == 1 {
				return syntax_primitive_1_evaluate (primitive, &inputs[0], evaluator);
			} else {
				fail! (0xc7837cc4);
			},
		
		SyntaxPrimitive::Primitive2 (primitive) =>
			if inputs_count == 2 {
				return syntax_primitive_2_evaluate (primitive, &inputs[0], &inputs[1], evaluator);
			} else {
				fail! (0xb92232f2);
			},
		
		SyntaxPrimitive::Primitive3 (primitive) =>
			if inputs_count == 3 {
				return syntax_primitive_3_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], evaluator);
			} else {
				fail! (0x18d7a5f8);
			},
		
		SyntaxPrimitive::Primitive4 (primitive) =>
			if inputs_count == 4 {
				return syntax_primitive_4_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], evaluator);
			} else {
				fail! (0xef34a67c);
			},
		
		SyntaxPrimitive::Primitive5 (primitive) =>
			if inputs_count == 5 {
				return syntax_primitive_5_evaluate (primitive, &inputs[0], &inputs[1], &inputs[2], &inputs[3], &inputs[4], evaluator);
			} else {
				fail! (0xe095946f);
			},
		
		SyntaxPrimitive::PrimitiveN (primitive) =>
			return syntax_primitive_n_evaluate (primitive, inputs, evaluator),
		
		SyntaxPrimitive::Auxiliary =>
			fail! (0x050a390b),
		
		SyntaxPrimitive::Unimplemented =>
			fail_unimplemented! (0x303dde78),
		
		SyntaxPrimitive::Unsupported =>
			fail_unimplemented! (0x70ac4438),
		
		SyntaxPrimitive::Reserved =>
			fail! (0x20a9c095),
		
	}
}

