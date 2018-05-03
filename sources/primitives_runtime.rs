

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
use super::transcript::exports::*;

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




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive0 {
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	
	ProcessArgumentsAsList,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ProcessArgumentsAsArray,
	ProcessArgumentsCount,
	ProcessEnvironmentVariablesAsList,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ProcessEnvironmentVariablesAsArray,
	
	ProcessExit,
	ProcessExitEmergency,
	
	PosixTimestamp,
	
	JiffiesTimestamp,
	JiffiesPerSecond,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive1 {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ValueRaise,
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorMessage,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorArgumentsAsList,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ErrorArgumentsAsArray,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	ErrorArgumentsAsValues,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	ProcessArgument,
	ProcessEnvironmentVariable,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitPoll,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitTry,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitCheck,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStdinGet,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStdoutGet,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStderrGet,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheOpen,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheClose,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheExcludeAll,
	
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	SerdeSerializeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	SerdeDeserializeBytes,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive2 {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterConfigure,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheOpen,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheSelectBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheExcludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheExcludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheExcludeAll,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive3 {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheOpen,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheSelectBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheIncludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheExcludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheExcludeSerde,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive4 {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheOpen,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheIncludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitive5 {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitiveN {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawn,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunTry,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunCheck,
	
}


#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
pub enum RuntimePrimitiveV {
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorRaise,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterConfigure,
	
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceCritical,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceError,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceWarning,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceNotice,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInformation,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceInternal,
	#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
	TranscriptTraceDebugging,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheOpen,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheSelectBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheIncludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheExcludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheExcludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheExcludeAll,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive0::ParameterBuild =>
			return parameter_build (None, None, None, None, evaluator) .into_0 (),
		
		RuntimePrimitive0::ProcessArgumentsAsList =>
			return process_arguments (evaluator, false),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RuntimePrimitive0::ProcessArgumentsAsArray =>
			return process_arguments (evaluator, true),
		
		RuntimePrimitive0::ProcessArgumentsCount =>
			return process_arguments_count (evaluator),
		
		RuntimePrimitive0::ProcessEnvironmentVariablesAsList =>
			return process_environment_variables (evaluator, false),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RuntimePrimitive0::ProcessEnvironmentVariablesAsArray =>
			return process_environment_variables (evaluator, true),
		
		RuntimePrimitive0::ProcessExit =>
			return Err (try! (error_exit (None, false)) .into ()),
		
		RuntimePrimitive0::ProcessExitEmergency =>
			return Err (try! (error_exit (None, true)) .into ()),
		
		RuntimePrimitive0::PosixTimestamp =>
			return posix_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesTimestamp =>
			return jiffies_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesPerSecond =>
			return jiffies_per_second () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ValueRaise =>
			return Err (error_coerce (None, input_1)),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ErrorRaise =>
			return Err (try! (error_build_0 (None, input_1))),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ErrorBuild =>
			return error_build_0 (None, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ErrorMessage =>
			return error_message (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ErrorArgumentsAsList =>
			return error_arguments_as_list (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RuntimePrimitive1::ErrorArgumentsAsArray =>
			return error_arguments_as_array (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		#[ cfg ( feature = "vonuvoli_values_values" ) ]
		RuntimePrimitive1::ErrorArgumentsAsValues =>
			return error_arguments_as_values (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive1::ParameterBuild =>
			return parameter_build (None, Some (input_1), None, None, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive1::ParameterResolve =>
			return parameter_resolve (input_1, None, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive1::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::ProcessArgument =>
			return process_argument (input_1, evaluator),
		
		RuntimePrimitive1::ProcessEnvironmentVariable =>
			return process_environment_variable (input_1, evaluator),
		
		RuntimePrimitive1::ProcessExit =>
			return Err (try! (error_exit (Some (input_1), false)) .into ()),
		
		RuntimePrimitive1::ProcessExitEmergency =>
			return Err (try! (error_exit (Some (input_1), true)) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, None, None, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitPoll =>
			return process_wait (input_1, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitTry =>
			return process_wait (input_1, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitCheck =>
			return process_wait_check (input_1, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStdinGet =>
			return process_stdin_get (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStdoutGet =>
			return process_stdout_get (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStderrGet =>
			return process_stderr_get (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CacheOpen =>
			return cache_open (input_1, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CacheClose =>
			return cache_close (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CacheExcludeAll =>
			return cache_exclude_all (input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive1::SerdeSerializeBytes =>
			return serde_serialize_into_bytes (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive1::SerdeDeserializeBytes =>
			return serde_deserialize_from_bytes (input_1),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive2::ErrorRaise =>
			return Err (try! (error_build_1 (None, input_1, input_2))),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive2::ErrorBuild =>
			return error_build_1 (None, input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterBuild =>
			return parameter_build (None, Some (input_1), Some (input_2), None, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterResolve =>
			return parameter_resolve (input_1, Some (input_2), evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterConfigure =>
			return parameter_configure (input_1, input_2, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive2::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive2::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), None, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive2::CacheOpen =>
			return cache_open (input_1, Some (input_2), None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive2::CacheSelectBytes =>
			return cache_select_bytes (input_1, None, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive2::CacheExcludeBytes =>
			return cache_exclude_bytes (input_1, None, input_2, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive2::CacheSelectSerde =>
			return cache_select_serde (input_1, None, input_2, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive2::CacheExcludeSerde =>
			return cache_exclude_serde (input_1, None, input_2, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive2::CacheExcludeAll =>
			return cache_exclude_all (input_1, Some (input_2), None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive3::ErrorRaise =>
			return Err (try! (error_build_2 (None, input_1, input_2, input_3))),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive3::ErrorBuild =>
			return error_build_2 (None, input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive3::ParameterBuild =>
			return parameter_build (None, Some (input_1), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ()), evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive3::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive3::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), Some (input_3), &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive3::CacheOpen =>
			return cache_open (input_1, Some (input_2), Some (input_3), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheSelectBytes =>
			return cache_select_bytes (input_1, Some (input_2), input_3, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheIncludeBytes =>
			return cache_include_bytes (input_1, None, input_2, input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheExcludeBytes =>
			return cache_exclude_bytes (input_1, Some (input_2), input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheSelectSerde =>
			return cache_select_serde (input_1, Some (input_2), input_3, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheIncludeSerde =>
			return cache_include_serde (input_1, None, input_2, input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheExcludeSerde =>
			return cache_exclude_serde (input_1, Some (input_2), input_3, None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive4::ErrorRaise =>
			return Err (try! (error_build_3 (None, input_1, input_2, input_3, input_4))),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive4::ErrorBuild =>
			return error_build_3 (None, input_1, input_2, input_3, input_4) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive4::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive4::CacheOpen =>
			return cache_open (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive4::CacheIncludeBytes =>
			return cache_include_bytes (input_1, Some (input_2), input_3, input_4, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive4::CacheIncludeSerde =>
			return cache_include_serde (input_1, Some (input_2), input_3, input_4, None) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive5::ErrorRaise =>
			return Err (try! (error_build_4 (None, input_1, input_2, input_3, input_4, input_5))),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive5::ErrorBuild =>
			return error_build_4 (None, input_1, input_2, input_3, input_4, input_5) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitive5::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveN::ErrorRaise => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x84aec603);
			return Err (try! (error_build_n (None, message, inputs)));
		},
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveN::ErrorBuild => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x87db450f);
			return error_build_n (None, message, inputs) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveN::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessSpawn =>
			return process_spawn (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunTry =>
			return process_run (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunCheck =>
			return process_run_check (inputs, &mut Some (evaluator)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_0 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive0>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			None,
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive0::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_1 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive1>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive1::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive1::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive1::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			Some (RuntimePrimitive1::ParameterResolve),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive1::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive1::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive1::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive1::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive1::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive1::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive1::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive1::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitive1::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			Some (RuntimePrimitive1::CacheExcludeAll),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_2 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive2>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive2::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive2::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive2::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			Some (RuntimePrimitive2::ParameterResolve),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			Some (RuntimePrimitive2::ParameterConfigure),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive2::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive2::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive2::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive2::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive2::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive2::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive2::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive2::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitive2::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			Some (RuntimePrimitive2::CacheSelectBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			Some (RuntimePrimitive2::CacheExcludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			Some (RuntimePrimitive2::CacheSelectSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			Some (RuntimePrimitive2::CacheExcludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			Some (RuntimePrimitive2::CacheExcludeAll),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_3 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive3>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive3::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive3::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive3::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive3::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive3::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive3::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive3::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive3::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive3::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive3::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive3::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitive3::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			Some (RuntimePrimitive3::CacheSelectBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			Some (RuntimePrimitive3::CacheIncludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			Some (RuntimePrimitive3::CacheExcludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			Some (RuntimePrimitive3::CacheSelectSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			Some (RuntimePrimitive3::CacheIncludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			Some (RuntimePrimitive3::CacheExcludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_4 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive4>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive4::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive4::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive4::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive4::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive4::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive4::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive4::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive4::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive4::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitive4::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			Some (RuntimePrimitive4::CacheIncludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			Some (RuntimePrimitive4::CacheIncludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_5 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive5>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive5::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive5::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive5::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive5::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive5::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive5::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive5::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive5::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive5::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_n (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitiveN>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitiveN::ErrorRaise),
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitiveN::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitiveN::TranscriptTraceCritical),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitiveN::TranscriptTraceError),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitiveN::TranscriptTraceWarning),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitiveN::TranscriptTraceNotice),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitiveN::TranscriptTraceInformation),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitiveN::TranscriptTraceInternal),
		#[ cfg ( feature = "vonuvoli_builtins_transcript" ) ]
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitiveN::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_attributes (_primitive : RuntimePrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_attributes (_primitive : RuntimePrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
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

