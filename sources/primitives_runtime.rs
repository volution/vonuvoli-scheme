

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




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive0 {
	
	ProcessArguments,
	ProcessEnvironment,
	
	ProcessExit,
	ProcessExitEmergency,
	
	PosixTimestamp,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive1 {
	
	ProcessEnvironment,
	
	ProcessExit,
	ProcessExitEmergency,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive2 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveN {
	
	ProcessExit,
	ProcessExitEmergency,
	
}




pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive0::ProcessArguments =>
			fail_unimplemented! (0x1a7fa84a),
		
		RuntimePrimitive0::ProcessEnvironment =>
			fail_unimplemented! (0x3d8c06db),
		
		RuntimePrimitive0::ProcessExit =>
			fail_unimplemented! (0x6b6e5604),
		
		RuntimePrimitive0::ProcessExitEmergency =>
			fail_unimplemented! (0xe1a2c04e),
		
		RuntimePrimitive0::PosixTimestamp =>
			fail_unimplemented! (0x3b85005d),
		
	}
}




pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, _input_1 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive1::ProcessEnvironment =>
			fail_unimplemented! (0x8f801b52),
		
		RuntimePrimitive1::ProcessExit =>
			fail_unimplemented! (0xf2f39391),
		
		RuntimePrimitive1::ProcessExitEmergency =>
			fail_unimplemented! (0x7a0fae27),
	}
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




pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, inputs : &[Value], evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		RuntimePrimitiveN::ProcessExit =>
			match inputs_count {
				0 =>
					return runtime_primitive_0_evaluate (RuntimePrimitive0::ProcessExit, evaluator),
				1 =>
					return runtime_primitive_1_evaluate (RuntimePrimitive1::ProcessExit, &inputs[0], evaluator),
				_ =>
					fail! (0xcb935dec),
			},
		
		RuntimePrimitiveN::ProcessExitEmergency =>
			match inputs_count {
				0 =>
					return runtime_primitive_0_evaluate (RuntimePrimitive0::ProcessExitEmergency, evaluator),
				1 =>
					return runtime_primitive_1_evaluate (RuntimePrimitive1::ProcessExitEmergency, &inputs[0], evaluator),
				_ =>
					fail! (0xbde2b2cf),
			},
		
	}
}




pub fn runtime_primitive_n_alternative_0 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive0>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveN::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
	}
}


pub fn runtime_primitive_n_alternative_1 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive1>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveN::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
	}
}


pub fn runtime_primitive_n_alternative_2 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive2>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			None,
		RuntimePrimitiveN::ProcessExitEmergency =>
			None,
	}
}


pub fn runtime_primitive_n_alternative_3 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive3>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			None,
		RuntimePrimitiveN::ProcessExitEmergency =>
			None,
	}
}


pub fn runtime_primitive_n_alternative_4 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive4>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			None,
		RuntimePrimitiveN::ProcessExitEmergency =>
			None,
	}
}


pub fn runtime_primitive_n_alternative_5 (primitive : RuntimePrimitiveN) -> (Option<RuntimePrimitive5>) {
	match primitive {
		RuntimePrimitiveN::ProcessExit =>
			None,
		RuntimePrimitiveN::ProcessExitEmergency =>
			None,
	}
}

