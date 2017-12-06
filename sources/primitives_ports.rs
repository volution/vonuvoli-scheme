

use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::PortPrimitive0;
	pub use super::PortPrimitive1;
	pub use super::PortPrimitive2;
	pub use super::PortPrimitive3;
	pub use super::PortPrimitive4;
	pub use super::PortPrimitive5;
	pub use super::PortPrimitiveN;
	
	pub use super::port_primitive_0_evaluate;
	pub use super::port_primitive_1_evaluate;
	pub use super::port_primitive_2_evaluate;
	pub use super::port_primitive_3_evaluate;
	pub use super::port_primitive_4_evaluate;
	pub use super::port_primitive_5_evaluate;
	pub use super::port_primitive_n_evaluate;
	
	pub use super::port_primitive_n_alternative_0;
	pub use super::port_primitive_n_alternative_1;
	pub use super::port_primitive_n_alternative_2;
	pub use super::port_primitive_n_alternative_3;
	pub use super::port_primitive_n_alternative_4;
	pub use super::port_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive0 {
	
	RsNewLine,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive1 {
	
	RsDisplay,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveN {}




pub fn port_primitive_0_evaluate (primitive : PortPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive0::RsNewLine => {
			// FIXME:  Replace this stub implementation!
			use std::io;
			use std::io::Write;
			let mut stream = io::stdout ();
			let mut stream = stream.lock ();
			match write! (stream, "\n") {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0xe2f91118),
			}
			match stream.flush () {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0x35130507),
			}
			succeed! (VOID.into ());
		},
		
	}
}




pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive1::RsDisplay => {
			// FIXME:  Replace this stub implementation!
			use std::io;
			use std::io::Write;
			let mut stream = io::stdout ();
			let mut stream = stream.lock ();
			match write! (stream, "{}", input_1) {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0x7aab6cc6),
			}
			succeed! (VOID.into ());
		},
		
	}
}




pub fn port_primitive_2_evaluate (primitive : PortPrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_n_alternative_0 (primitive : PortPrimitiveN) -> (Option<PortPrimitive0>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_1 (primitive : PortPrimitiveN) -> (Option<PortPrimitive1>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_2 (primitive : PortPrimitiveN) -> (Option<PortPrimitive2>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_3 (primitive : PortPrimitiveN) -> (Option<PortPrimitive3>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_4 (primitive : PortPrimitiveN) -> (Option<PortPrimitive4>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_5 (primitive : PortPrimitiveN) -> (Option<PortPrimitive5>) {
	match primitive {}
}

