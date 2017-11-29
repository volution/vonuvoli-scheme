

use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::ComparisonPrimitive0;
	pub use super::ComparisonPrimitive1;
	pub use super::ComparisonPrimitive2;
	pub use super::ComparisonPrimitive3;
	pub use super::ComparisonPrimitive4;
	pub use super::ComparisonPrimitive5;
	pub use super::ComparisonPrimitiveN;
	
	pub use super::comparison_primitive_0_evaluate;
	pub use super::comparison_primitive_1_evaluate;
	pub use super::comparison_primitive_2_evaluate;
	pub use super::comparison_primitive_3_evaluate;
	pub use super::comparison_primitive_4_evaluate;
	pub use super::comparison_primitive_5_evaluate;
	pub use super::comparison_primitive_n_evaluate;
	
	pub use super::comparison_primitive_n_alternative_0;
	pub use super::comparison_primitive_n_alternative_1;
	pub use super::comparison_primitive_n_alternative_2;
	pub use super::comparison_primitive_n_alternative_3;
	pub use super::comparison_primitive_n_alternative_4;
	pub use super::comparison_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive0 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive1 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ComparisonPrimitiveN {}




pub fn comparison_primitive_0_evaluate (primitive : ComparisonPrimitive0) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_1_evaluate (primitive : ComparisonPrimitive1, _input_1 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_2_evaluate (primitive : ComparisonPrimitive2, _input_1 : &Value, _input_2 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_3_evaluate (primitive : ComparisonPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_4_evaluate (primitive : ComparisonPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_5_evaluate (primitive : ComparisonPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_n_evaluate (primitive : ComparisonPrimitiveN, _inputs : &[Value]) -> (Outcome<Value>) {
	match primitive {}
}




pub fn comparison_primitive_n_alternative_0 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive0>) {
	match primitive {}
}


pub fn comparison_primitive_n_alternative_1 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive1>) {
	match primitive {}
}


pub fn comparison_primitive_n_alternative_2 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive2>) {
	match primitive {}
}


pub fn comparison_primitive_n_alternative_3 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive3>) {
	match primitive {}
}


pub fn comparison_primitive_n_alternative_4 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive4>) {
	match primitive {}
}


pub fn comparison_primitive_n_alternative_5 (primitive : ComparisonPrimitiveN) -> (Option<ComparisonPrimitive5>) {
	match primitive {}
}

