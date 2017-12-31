

use super::constants::exports::*;
use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::RuntimePrimitive0;
	pub use super::RuntimePrimitive1;
	pub use super::RuntimePrimitive2;
	pub use super::RuntimePrimitive3;
	pub use super::RuntimePrimitive4;
	pub use super::RuntimePrimitive5;
	pub use super::RuntimePrimitiveN;
	pub use super::RuntimePrimitiveV;
	
	pub use super::runtime_primitive_0_evaluate;
	pub use super::runtime_primitive_1_evaluate;
	pub use super::runtime_primitive_2_evaluate;
	pub use super::runtime_primitive_3_evaluate;
	pub use super::runtime_primitive_4_evaluate;
	pub use super::runtime_primitive_5_evaluate;
	pub use super::runtime_primitive_n_evaluate;
	
	pub use super::runtime_primitive_v_alternative_0;
	pub use super::runtime_primitive_v_alternative_1;
	pub use super::runtime_primitive_v_alternative_2;
	pub use super::runtime_primitive_v_alternative_3;
	pub use super::runtime_primitive_v_alternative_4;
	pub use super::runtime_primitive_v_alternative_5;
	pub use super::runtime_primitive_v_alternative_n;
	
	pub use super::runtime_primitive_0_attributes;
	pub use super::runtime_primitive_1_attributes;
	pub use super::runtime_primitive_2_attributes;
	pub use super::runtime_primitive_3_attributes;
	pub use super::runtime_primitive_4_attributes;
	pub use super::runtime_primitive_5_attributes;
	pub use super::runtime_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive0 {
	
	ProcessArguments,
	ProcessEnvironment,
	
	ProcessExit,
	ProcessExitEmergency,
	
	PosixTimestamp,
	
	JiffiesTimestamp,
	JiffiesPerSecond,
	
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
pub enum RuntimePrimitiveN {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveV {
	
	ProcessExit,
	ProcessExitEmergency,
	
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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
			succeed! (posix_timestamp () .into ()),
		
		RuntimePrimitive0::JiffiesTimestamp =>
			succeed! (jiffies_timestamp () .into ()),
		
		RuntimePrimitive0::JiffiesPerSecond =>
			succeed! (jiffies_per_second () .into ()),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
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




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, _inputs : &[&Value], _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_0 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive0>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_1 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive1>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_2 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive2>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_3 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive3>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_4 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive4>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_5 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive5>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_v_alternative_n (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitiveN>) {
	match primitive {
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_0_attributes (primitive : RuntimePrimitive0) -> (Option<ProcedureAttributes>) {
	match primitive {
		RuntimePrimitive0::ProcessArguments |
		RuntimePrimitive0::ProcessEnvironment |
		RuntimePrimitive0::JiffiesPerSecond =>
			Some (CONSTANT_PROCEDURE_ATTRIBUTES_0),
		_ =>
			None,
	}
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_1_attributes (primitive : RuntimePrimitive1) -> (Option<ProcedureAttributes>) {
	match primitive {
		RuntimePrimitive1::ProcessEnvironment =>
			Some (CONSTANT_PROCEDURE_ATTRIBUTES_1),
		_ =>
			None,
	}
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_2_attributes (_primitive : RuntimePrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_3_attributes (_primitive : RuntimePrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_4_attributes (_primitive : RuntimePrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_5_attributes (_primitive : RuntimePrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn runtime_primitive_n_attributes (_primitive : RuntimePrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

