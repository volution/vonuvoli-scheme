

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
	
	ValueRaise,
	
	ErrorRaise,
	ErrorBuild,
	ErrorMessage,
	ErrorArgumentsAsList,
	ErrorArgumentsAsArray,
	ErrorArgumentsAsValues,
	
	RecordKindGet,
	RecordBuild,
	RecordToImmutable,
	RecordToMutable,
	RecordToArray,
	RecordFromArray,
	RecordToValues,
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
	ProcessEnvironment,
	
	ProcessExit,
	ProcessExitEmergency,
	
	ProcessSpawnExtended,
	
	ProcessWaitPoll,
	ProcessWaitTry,
	ProcessWaitCheck,
	
	ProcessStdinGet,
	ProcessStdoutGet,
	ProcessStderrGet,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive2 {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordKindIs,
	RecordGet,
	RecordBuild,
	RecordToArray,
	RecordFromArray,
	RecordToValues,
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
	ProcessSpawnExtended,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive3 {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordGet,
	RecordSet,
	RecordBuild,
	
	ProcessSpawnExtended,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive4 {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordSet,
	RecordBuild,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive5 {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordBuild,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveN {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordBuild,
	
	ProcessSpawn,
	ProcessRunTry,
	ProcessRunCheck,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveV {
	
	ErrorRaise,
	ErrorBuild,
	
	RecordSet,
	RecordGet,
	RecordBuild,
	RecordToArray,
	RecordFromArray,
	RecordToValues,
	RecordFromValues,
	RecordToList,
	RecordFromList,
	
	ProcessExit,
	ProcessExitEmergency,
	
	ProcessSpawnExtended,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive0::ProcessArguments =>
			fail_unimplemented! (0x1a7fa84a), // deferred
		
		RuntimePrimitive0::ProcessEnvironment =>
			fail_unimplemented! (0x3d8c06db), // deferred
		
		RuntimePrimitive0::ProcessExit =>
			fail_unimplemented! (0x6b6e5604), // deferred
		
		RuntimePrimitive0::ProcessExitEmergency =>
			fail_unimplemented! (0xe1a2c04e), // deferred
		
		RuntimePrimitive0::PosixTimestamp =>
			return posix_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesTimestamp =>
			return jiffies_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesPerSecond =>
			return jiffies_per_second () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, input_1 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive1::ValueRaise =>
			return Err (error_coerce (None, input_1)),
		
		RuntimePrimitive1::ErrorRaise =>
			return Err (try! (error_build_0 (None, input_1))),
		
		RuntimePrimitive1::ErrorBuild =>
			return error_build_0 (None, input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorMessage =>
			return error_message (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsList =>
			return error_arguments_as_list (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsArray =>
			return error_arguments_as_array (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsValues =>
			return error_arguments_as_values (input_1) .into_0 (),
		
		RuntimePrimitive1::RecordKindGet =>
			return record_kind_get (input_1) .into_0 (),
		
		RuntimePrimitive1::RecordBuild =>
			return record_build_0 (try_as_record_kind_ref! (input_1), None, None),
		
		RuntimePrimitive1::RecordToImmutable =>
			return try_as_record_as_ref! (input_1) .to_immutable () .into_0 (),
		
		RuntimePrimitive1::RecordToMutable =>
			return try_as_record_as_ref! (input_1) .to_mutable () .into_0 (),
		
		RuntimePrimitive1::RecordToArray =>
			return record_to_array (None, input_1, None),
		
		RuntimePrimitive1::RecordFromArray =>
			return record_from_array (None, input_1, None),
		
		RuntimePrimitive1::RecordToValues =>
			return record_to_values (None, input_1, None),
		
		RuntimePrimitive1::RecordFromValues =>
			return record_from_values (None, input_1, None),
		
		RuntimePrimitive1::RecordToList =>
			return record_to_list (None, input_1, None),
		
		RuntimePrimitive1::RecordFromList =>
			return record_from_list (None, input_1, None),
		
		RuntimePrimitive1::ProcessEnvironment =>
			fail_unimplemented! (0x8f801b52), // deferred
		
		RuntimePrimitive1::ProcessExit =>
			fail_unimplemented! (0xf2f39391), // deferred
		
		RuntimePrimitive1::ProcessExitEmergency =>
			fail_unimplemented! (0x7a0fae27), // deferred
		
		RuntimePrimitive1::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, None, None) .into_0 (),
		
		RuntimePrimitive1::ProcessWaitPoll =>
			return process_wait (input_1, false) .into_0 (),
		
		RuntimePrimitive1::ProcessWaitTry =>
			return process_wait (input_1, true) .into_0 (),
		
		RuntimePrimitive1::ProcessWaitCheck =>
			return process_wait_check (input_1, true) .into_0 (),
		
		RuntimePrimitive1::ProcessStdinGet =>
			return process_stdin_get (input_1),
		
		RuntimePrimitive1::ProcessStdoutGet =>
			return process_stdout_get (input_1),
		
		RuntimePrimitive1::ProcessStderrGet =>
			return process_stderr_get (input_1),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive2::ErrorRaise =>
			return Err (try! (error_build_1 (None, input_1, input_2))),
		
		RuntimePrimitive2::ErrorBuild =>
			return error_build_1 (None, input_1, input_2) .into_0 (),
		
		RuntimePrimitive2::RecordKindIs =>
			return record_kind_is (try_as_record_kind_ref! (input_1), input_2) .into_0 (),
		
		RuntimePrimitive2::RecordGet =>
			return record_get_x (None, input_1, input_2),
		
		RuntimePrimitive2::RecordBuild =>
			return record_build_1 (try_as_record_kind_ref! (input_1), None, input_2, None),
		
		RuntimePrimitive2::RecordToArray =>
			return record_to_array (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::RecordFromArray =>
			return record_from_array (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::RecordToValues =>
			return record_to_values (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::RecordFromValues =>
			return record_from_values (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::RecordToList =>
			return record_to_list (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::RecordFromList =>
			return record_from_list (Some (try_as_record_kind_ref! (input_1)), input_2, None),
		
		RuntimePrimitive2::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive3::ErrorRaise =>
			return Err (try! (error_build_2 (None, input_1, input_2, input_3))),
		
		RuntimePrimitive3::ErrorBuild =>
			return error_build_2 (None, input_1, input_2, input_3) .into_0 (),
		
		RuntimePrimitive3::RecordGet =>
			return record_get_x (Some (try_as_record_kind_ref! (input_1)), input_2, input_3),
		
		RuntimePrimitive3::RecordSet =>
			return record_set_x (None, input_1, input_2, input_3),
		
		RuntimePrimitive3::RecordBuild =>
			return record_build_2 (try_as_record_kind_ref! (input_1), None, input_2, input_3, None),
		
		RuntimePrimitive3::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), Some (input_3)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive4::ErrorRaise =>
			return Err (try! (error_build_3 (None, input_1, input_2, input_3, input_4))),
		
		RuntimePrimitive4::ErrorBuild =>
			return error_build_3 (None, input_1, input_2, input_3, input_4) .into_0 (),
		
		RuntimePrimitive4::RecordSet =>
			return record_set_x (Some (try_as_record_kind_ref! (input_1)), input_2, input_3, input_4),
		
		RuntimePrimitive4::RecordBuild =>
			return record_build_3 (try_as_record_kind_ref! (input_1), None, input_2, input_3, input_4, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive5::ErrorRaise =>
			return Err (try! (error_build_4 (None, input_1, input_2, input_3, input_4, input_5))),
		
		RuntimePrimitive5::ErrorBuild =>
			return error_build_4 (None, input_1, input_2, input_3, input_4, input_5) .into_0 (),
		
		RuntimePrimitive5::RecordBuild =>
			return record_build_4 (try_as_record_kind_ref! (input_1), None, input_2, input_3, input_4, input_5, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, inputs : &[&Value], _evaluator : &EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitiveN::ErrorRaise => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x84aec603);
			return Err (try! (error_build_n (None, message, inputs)));
		},
		
		RuntimePrimitiveN::ErrorBuild => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x87db450f);
			return error_build_n (None, message, inputs) .into_0 ();
		},
		
		RuntimePrimitiveN::RecordBuild => {
			let (kind, inputs) = try_some! (inputs.split_first (), 0xa34efcb8);
			return record_build_n (try_as_record_kind_ref! (*kind), None, inputs, None);
		},
		
		RuntimePrimitiveN::ProcessSpawn =>
			return process_spawn (inputs) .into_0 (),
		
		RuntimePrimitiveN::ProcessRunTry =>
			return process_run (inputs) .into_0 (),
		
		RuntimePrimitiveN::ProcessRunCheck =>
			return process_run_check (inputs) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_0 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive0>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			None,
		RuntimePrimitiveV::ErrorBuild =>
			None,
		RuntimePrimitiveV::RecordGet =>
			None,
		RuntimePrimitiveV::RecordSet =>
			None,
		RuntimePrimitiveV::RecordBuild =>
			None,
		RuntimePrimitiveV::RecordToArray =>
			None,
		RuntimePrimitiveV::RecordFromArray =>
			None,
		RuntimePrimitiveV::RecordToValues =>
			None,
		RuntimePrimitiveV::RecordFromValues =>
			None,
		RuntimePrimitiveV::RecordToList =>
			None,
		RuntimePrimitiveV::RecordFromList =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_1 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive1>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive1::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive1::ErrorBuild),
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveV::RecordGet =>
			None,
		RuntimePrimitiveV::RecordSet =>
			None,
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitive1::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			Some (RuntimePrimitive1::RecordToArray),
		RuntimePrimitiveV::RecordFromArray =>
			Some (RuntimePrimitive1::RecordFromArray),
		RuntimePrimitiveV::RecordToValues =>
			Some (RuntimePrimitive1::RecordToValues),
		RuntimePrimitiveV::RecordFromValues =>
			Some (RuntimePrimitive1::RecordFromValues),
		RuntimePrimitiveV::RecordToList =>
			Some (RuntimePrimitive1::RecordToList),
		RuntimePrimitiveV::RecordFromList =>
			Some (RuntimePrimitive1::RecordFromList),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive1::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_2 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive2>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive2::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive2::ErrorBuild),
		RuntimePrimitiveV::RecordGet =>
			Some (RuntimePrimitive2::RecordGet),
		RuntimePrimitiveV::RecordSet =>
			None,
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitive2::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			Some (RuntimePrimitive2::RecordToArray),
		RuntimePrimitiveV::RecordFromArray =>
			Some (RuntimePrimitive2::RecordFromArray),
		RuntimePrimitiveV::RecordToValues =>
			Some (RuntimePrimitive2::RecordToValues),
		RuntimePrimitiveV::RecordFromValues =>
			Some (RuntimePrimitive2::RecordFromValues),
		RuntimePrimitiveV::RecordToList =>
			Some (RuntimePrimitive2::RecordToList),
		RuntimePrimitiveV::RecordFromList =>
			Some (RuntimePrimitive2::RecordFromList),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive2::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_3 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive3>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive3::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive3::ErrorBuild),
		RuntimePrimitiveV::RecordGet =>
			Some (RuntimePrimitive3::RecordGet),
		RuntimePrimitiveV::RecordSet =>
			Some (RuntimePrimitive3::RecordSet),
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitive3::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			None,
		RuntimePrimitiveV::RecordFromArray =>
			None,
		RuntimePrimitiveV::RecordToValues =>
			None,
		RuntimePrimitiveV::RecordFromValues =>
			None,
		RuntimePrimitiveV::RecordToList =>
			None,
		RuntimePrimitiveV::RecordFromList =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive3::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_4 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive4>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive4::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive4::ErrorBuild),
		RuntimePrimitiveV::RecordGet =>
			None,
		RuntimePrimitiveV::RecordSet =>
			Some (RuntimePrimitive4::RecordSet),
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitive4::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			None,
		RuntimePrimitiveV::RecordFromArray =>
			None,
		RuntimePrimitiveV::RecordToValues =>
			None,
		RuntimePrimitiveV::RecordFromValues =>
			None,
		RuntimePrimitiveV::RecordToList =>
			None,
		RuntimePrimitiveV::RecordFromList =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_5 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive5>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive5::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive5::ErrorBuild),
		RuntimePrimitiveV::RecordGet =>
			None,
		RuntimePrimitiveV::RecordSet =>
			None,
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitive5::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			None,
		RuntimePrimitiveV::RecordFromArray =>
			None,
		RuntimePrimitiveV::RecordToValues =>
			None,
		RuntimePrimitiveV::RecordFromValues =>
			None,
		RuntimePrimitiveV::RecordToList =>
			None,
		RuntimePrimitiveV::RecordFromList =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_n (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitiveN>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitiveN::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitiveN::ErrorBuild),
		RuntimePrimitiveV::RecordGet =>
			None,
		RuntimePrimitiveV::RecordSet =>
			None,
		RuntimePrimitiveV::RecordBuild =>
			Some (RuntimePrimitiveN::RecordBuild),
		RuntimePrimitiveV::RecordToArray =>
			None,
		RuntimePrimitiveV::RecordFromArray =>
			None,
		RuntimePrimitiveV::RecordToValues =>
			None,
		RuntimePrimitiveV::RecordFromValues =>
			None,
		RuntimePrimitiveV::RecordToList =>
			None,
		RuntimePrimitiveV::RecordFromList =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_attributes (primitive : RuntimePrimitive1) -> (Option<ProcedureAttributes>) {
	match primitive {
		RuntimePrimitive1::ProcessEnvironment =>
			Some (CONSTANT_PROCEDURE_ATTRIBUTES_1),
		_ =>
			None,
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_attributes (_primitive : RuntimePrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_attributes (_primitive : RuntimePrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_attributes (_primitive : RuntimePrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_attributes (_primitive : RuntimePrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_attributes (_primitive : RuntimePrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

