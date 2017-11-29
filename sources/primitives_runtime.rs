

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::RuntimePrimitive0;
	pub use super::RuntimePrimitive1;
	pub use super::RuntimePrimitive2;
	pub use super::RuntimePrimitive3;
	pub use super::RuntimePrimitive4;
	pub use super::RuntimePrimitive5;
	pub use super::RuntimePrimitiveN;
	
	pub use super::runtime_primitive_0_evaluate;
	pub use super::runtime_primitive_1_evaluate;
	pub use super::runtime_primitive_2_evaluate;
	pub use super::runtime_primitive_3_evaluate;
	pub use super::runtime_primitive_4_evaluate;
	pub use super::runtime_primitive_5_evaluate;
	pub use super::runtime_primitive_n_evaluate;
	
	pub use super::runtime_primitive_n_alternative_0;
	pub use super::runtime_primitive_n_alternative_1;
	pub use super::runtime_primitive_n_alternative_2;
	pub use super::runtime_primitive_n_alternative_3;
	pub use super::runtime_primitive_n_alternative_4;
	pub use super::runtime_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive1 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum RuntimePrimitiveN {}




pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, _input_1 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, _inputs : &[Value], _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn runtime_primitive_n_alternative_0 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive0>) {
	match primitive {}
}


pub fn runtime_primitive_n_alternative_1 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive1>) {
	match primitive {}
}


pub fn runtime_primitive_n_alternative_2 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive2>) {
	match primitive {}
}


pub fn runtime_primitive_n_alternative_3 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive3>) {
	match primitive {}
}


pub fn runtime_primitive_n_alternative_4 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive4>) {
	match primitive {}
}


pub fn runtime_primitive_n_alternative_5 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive5>) {
	match primitive {}
}

