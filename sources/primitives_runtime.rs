

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;
use super::conversions::exports::*;
use super::primitives_procedures::exports::*;

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
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::runtime_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (RuntimePrimitive0, (procedure, 0), {
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	
	ProcessArgumentsAsList,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ProcessArgumentsAsArray,
	ProcessArgumentsCount,
	ProcessEnvironmentVariablesAsList,
	#[ cfg ( feature = "vonuvoli_values_array" ) ]
	ProcessEnvironmentVariablesAsArray,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "blake2-rfc" ) ]
	ProcessEnvironmentFingerprint,
	
	ProcessExit,
	ProcessExitEmergency,
	
	PosixTimestamp,
	
	JiffiesTimestamp,
	JiffiesPerSecond,
	
});


def_primitives_enum! (RuntimePrimitive1, (procedure, 1), {
	
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
	
	Abort,
	
	Pause,
	
	ProcessArgument,
	ProcessEnvironmentVariable,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessExecExtended,
	
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
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CachePruneAll,
	
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	SerdeSerializeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	SerdeDeserializeBytes,
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
	DefaultHash,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	SipHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	SipHashUnseeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	SeaHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	SeaHashUnseeded,
	
});


def_primitives_enum! (RuntimePrimitive2, (procedure, 2), {
	
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
	
	Abort,
	
	Pause,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessExecExtended,
	
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
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CachePruneAll,
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	SipHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	SeaHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2bHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2bHashUnseeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2sHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2sHashUnseeded,
	
});


def_primitives_enum! (RuntimePrimitive3, (procedure, 3), {
	
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
	
	Abort,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessExecExtended,
	
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
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheResolveBytes,
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
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheResolveSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CachePruneAll,
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2bHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2sHashSeeded,
	
});


def_primitives_enum! (RuntimePrimitive4, (procedure, 4), {
	
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
	
	Abort,
	
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
	CacheResolveBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheResolveSerde,
	
});


def_primitives_enum! (RuntimePrimitive5, (procedure, 5), {
	
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
	
	Abort,
	
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheSelectBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheIncludeBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheResolveBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheResolveSerde,
	
});


def_primitives_enum! (RuntimePrimitiveN, (procedure, n), {
	
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
	
	Abort,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawn,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessExec,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunTry,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunCheck,
	
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
	CacheResolveBytes,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheSelectSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheIncludeSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheResolveSerde,
	
});


def_primitives_enum! (RuntimePrimitiveV, (procedure, v), {
	
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
	
	Abort,
	
	Pause,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessExecExtended,
	
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
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	CacheResolveBytes,
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
	#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
	CacheResolveSerde,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CacheExcludeAll,
	#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
	CachePruneAll,
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	SipHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	SeaHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2bHashSeeded,
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	Blake2sHashSeeded,
	
});




impl_procedure_primitive_enum_matrix! (
		(RuntimePrimitive0, runtime_primitive_0_evaluate, runtime_primitive_0_attributes, runtime_primitive_v_alternative_0),
		(RuntimePrimitive1, runtime_primitive_1_evaluate, runtime_primitive_1_attributes, runtime_primitive_v_alternative_1),
		(RuntimePrimitive2, runtime_primitive_2_evaluate, runtime_primitive_2_attributes, runtime_primitive_v_alternative_2),
		(RuntimePrimitive3, runtime_primitive_3_evaluate, runtime_primitive_3_attributes, runtime_primitive_v_alternative_3),
		(RuntimePrimitive4, runtime_primitive_4_evaluate, runtime_primitive_4_attributes, runtime_primitive_v_alternative_4),
		(RuntimePrimitive5, runtime_primitive_5_evaluate, runtime_primitive_5_attributes, runtime_primitive_v_alternative_5),
		(RuntimePrimitiveN, runtime_primitive_n_evaluate, runtime_primitive_n_attributes, runtime_primitive_v_alternative_n),
		(RuntimePrimitiveV, runtime_primitive_v_evaluate, runtime_primitive_v_attributes),
	);




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive0::ParameterBuild =>
			return parameter_build (None, None, None, None, evaluator) .into_0 (),
		
		RuntimePrimitive0::ProcessArgumentsAsList =>
			return process_arguments (evaluator, false, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RuntimePrimitive0::ProcessArgumentsAsArray =>
			return process_arguments (evaluator, true, None),
		
		RuntimePrimitive0::ProcessArgumentsCount =>
			return process_arguments_count (evaluator),
		
		RuntimePrimitive0::ProcessEnvironmentVariablesAsList =>
			return process_environment_variables (evaluator, false, None),
		
		#[ cfg ( feature = "vonuvoli_values_array" ) ]
		RuntimePrimitive0::ProcessEnvironmentVariablesAsArray =>
			return process_environment_variables (evaluator, true, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "blake2-rfc" ) ]
		RuntimePrimitive0::ProcessEnvironmentFingerprint =>
			return process_environment_fingerprint (evaluator),
		
		RuntimePrimitive0::ProcessExit =>
			return Err (r#try! (error_exit (None, false))),
		
		RuntimePrimitive0::ProcessExitEmergency =>
			return Err (r#try! (error_exit (None, true))),
		
		RuntimePrimitive0::PosixTimestamp =>
			return posix_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesTimestamp =>
			return jiffies_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesPerSecond =>
			return jiffies_per_second () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ValueRaise =>
			return Err (error_coerce (None, input_1)),
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive1::ErrorRaise =>
			return Err (r#try! (error_build_0 (None, input_1))),
		
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
		
		RuntimePrimitive1::Abort =>
			return Err (r#try! (abort_g (&[input_1], evaluator))),
		
		RuntimePrimitive1::Pause =>
			return pause (input_1, None) .into_0 (),
		
		RuntimePrimitive1::ProcessArgument =>
			return process_argument (input_1, evaluator, None),
		
		RuntimePrimitive1::ProcessEnvironmentVariable =>
			return process_environment_variable (input_1, evaluator, None),
		
		RuntimePrimitive1::ProcessExit =>
			return Err (r#try! (error_exit (Some (input_1), false))),
		
		RuntimePrimitive1::ProcessExitEmergency =>
			return Err (r#try! (error_exit (Some (input_1), true))),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, None, None, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessExecExtended =>
			return Err (Error::new_exec (r#try! (process_prepare_extended (input_1, None, None, &mut Some (evaluator))))),
		
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
			return cache_open (input_1, None, None, None, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CacheClose =>
			return cache_close (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CacheExcludeAll =>
			return cache_exclude_all (input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive1::CachePruneAll =>
			return cache_prune_all (input_1, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive1::SerdeSerializeBytes =>
			return serde_serialize_into_bytes (input_1, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive1::SerdeDeserializeBytes =>
			return serde_deserialize_from_bytes (input_1, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes" ) ]
		RuntimePrimitive1::DefaultHash =>
			succeed! ((r#try! (hash_value_with_default (input_1, None)) as i64) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitive1::SipHashSeeded =>
			succeed! ((r#try! (hash_value_with_siphash_seeded (input_1, Some (None), None)) as i64) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitive1::SipHashUnseeded =>
			succeed! ((r#try! (hash_value_with_siphash_unseeded (input_1, None)) as i64) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitive1::SeaHashSeeded =>
			succeed! ((r#try! (hash_value_with_seahash_seeded (input_1, Some (None), None)) as i64) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitive1::SeaHashUnseeded =>
			succeed! ((r#try! (hash_value_with_seahash_unseeded (input_1, None)) as i64) .into ()),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive2::ErrorRaise =>
			return Err (r#try! (error_build_1 (None, input_1, input_2))),
		
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
		
		RuntimePrimitive2::Abort =>
			return Err (r#try! (abort_g (&[input_1, input_2], evaluator))),
		
		RuntimePrimitive2::Pause =>
			return pause (input_1, Some (input_2)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive2::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), None, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive2::ProcessExecExtended =>
			return Err (Error::new_exec (r#try! (process_prepare_extended (input_1, Some (input_2), None, &mut Some (evaluator))))),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive2::CacheOpen =>
			return cache_open (input_1, Some (input_2), None, None, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive2::CacheSelectBytes =>
			return cache_select_bytes (input_1, None, input_2, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive2::CacheExcludeBytes =>
			return cache_exclude_bytes (input_1, None, input_2, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive2::CacheSelectSerde =>
			return cache_select_serde (input_1, None, input_2, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive2::CacheExcludeSerde =>
			return cache_exclude_serde (input_1, None, input_2, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive2::CacheExcludeAll =>
			return cache_exclude_all (input_1, Some (input_2), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive2::CachePruneAll =>
			return cache_prune_all (input_1, Some (input_2), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitive2::SipHashSeeded => {
			let seed = r#try! (coerce_siphash_seed (input_2));
			let seed = option_ref_map! (seed, seed.as_ref ());
			succeed! ((r#try! (hash_value_with_siphash_seeded (input_1, seed, None)) as i64) .into ());
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitive2::SeaHashSeeded => {
			let seed = r#try! (coerce_seahash_seed (input_2));
			let seed = option_ref_map! (seed, seed.as_ref ());
			succeed! ((r#try! (hash_value_with_seahash_seeded (input_1, seed, None)) as i64) .into ());
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive2::Blake2bHashSeeded =>
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2b_seeded (input_1, r#try! (count_coerce (input_2)), Some (None), None))) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive2::Blake2bHashUnseeded =>
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2b_unseeded (input_1, r#try! (count_coerce (input_2)), None))) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive2::Blake2sHashSeeded =>
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2s_seeded (input_1, r#try! (count_coerce (input_2)), Some (None), None))) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive2::Blake2sHashUnseeded =>
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2s_unseeded (input_1, r#try! (count_coerce (input_2)), None))) .into ()),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive3::ErrorRaise =>
			return Err (r#try! (error_build_2 (None, input_1, input_2, input_3))),
		
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
		
		RuntimePrimitive3::Abort =>
			return Err (r#try! (abort_g (&[input_1, input_2, input_3], evaluator))),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive3::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), Some (input_3), &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive3::ProcessExecExtended =>
			return Err (Error::new_exec (r#try! (process_prepare_extended (input_1, Some (input_2), Some (input_3), &mut Some (evaluator))))),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive3::CacheOpen =>
			return cache_open (input_1, Some (input_2), Some (input_3), None, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheSelectBytes =>
			return cache_select_bytes (input_1, Some (input_2), input_3, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheIncludeBytes =>
			return cache_include_bytes (input_1, None, input_2, input_3, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheExcludeBytes =>
			return cache_exclude_bytes (input_1, Some (input_2), input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive3::CacheResolveBytes =>
			return cache_resolve_bytes (input_1, None, input_2, None, None, None, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheSelectSerde =>
			return cache_select_serde (input_1, Some (input_2), input_3, None, None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheIncludeSerde =>
			return cache_include_serde (input_1, None, input_2, input_3, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheExcludeSerde =>
			return cache_exclude_serde (input_1, Some (input_2), input_3, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive3::CacheResolveSerde =>
			return cache_resolve_serde (input_1, None, input_2, None, None, None, input_3, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive3::CachePruneAll =>
			return cache_prune_all (input_1, Some (input_2), Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive3::Blake2bHashSeeded => {
			let seed = r#try! (coerce_blake2b_seed (input_3));
			let seed = option_ref_map! (seed, option_ref_map! (seed, seed.as_ref ()));
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2b_seeded (input_1, r#try! (count_coerce (input_2)), seed, None))) .into ());
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitive3::Blake2sHashSeeded => {
			let seed = r#try! (coerce_blake2s_seed (input_3));
			let seed = option_ref_map! (seed, option_ref_map! (seed, seed.as_ref ()));
			succeed! (bytes_immutable_new_0 (r#try! (hash_value_with_blake2s_seeded (input_1, r#try! (count_coerce (input_2)), seed, None))) .into ());
		},
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive4::ErrorRaise =>
			return Err (r#try! (error_build_3 (None, input_1, input_2, input_3, input_4))),
		
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
		
		RuntimePrimitive4::Abort =>
			return Err (r#try! (abort_g (&[input_1, input_2, input_3, input_4], evaluator))),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitive4::CacheOpen =>
			return cache_open (input_1, Some (input_2), Some (input_3), None, None, Some (input_4), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive4::CacheSelectBytes =>
			return cache_select_bytes (input_1, Some (input_2), input_3, Some (input_4), None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive4::CacheIncludeBytes =>
			return cache_include_bytes (input_1, Some (input_2), input_3, input_4, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive4::CacheResolveBytes =>
			return cache_resolve_bytes (input_1, Some (input_2), input_3, None, None, None, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive4::CacheSelectSerde =>
			return cache_select_serde (input_1, Some (input_2), input_3, Some (input_4), None, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive4::CacheIncludeSerde =>
			return cache_include_serde (input_1, Some (input_2), input_3, input_4, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive4::CacheResolveSerde =>
			return cache_resolve_serde (input_1, Some (input_2), input_3, None, None, None, input_4, evaluator),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitive5::ErrorRaise =>
			return Err (r#try! (error_build_4 (None, input_1, input_2, input_3, input_4, input_5))),
		
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
		
		RuntimePrimitive5::Abort =>
			return Err (r#try! (abort_g (&[input_1, input_2, input_3, input_4, input_5], evaluator))),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive5::CacheSelectBytes =>
			return cache_select_bytes (input_1, Some (input_2), input_3, Some (input_4), Some (input_5), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive5::CacheIncludeBytes =>
			return cache_include_bytes (input_1, Some (input_2), input_3, input_4, Some (input_5), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitive5::CacheResolveBytes =>
			return cache_resolve_bytes (input_1, Some (input_2), input_3, Some (input_5), None, None, input_4, evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive5::CacheSelectSerde =>
			return cache_select_serde (input_1, Some (input_2), input_3, Some (input_4), Some (input_5), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive5::CacheIncludeSerde =>
			return cache_include_serde (input_1, Some (input_2), input_3, input_4, Some (input_5), None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitive5::CacheResolveSerde =>
			return cache_resolve_serde (input_1, Some (input_2), input_3, Some (input_5), None, None, input_4, evaluator),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, inputs : &[impl StdAsRef<Value>], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveN::ErrorRaise => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x84aec603);
			return Err (r#try! (error_build_n (None, message.as_ref (), inputs)));
		},
		
		#[ cfg ( feature = "vonuvoli_values_error" ) ]
		RuntimePrimitiveN::ErrorBuild => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x87db450f);
			return error_build_n (None, message.as_ref (), inputs) .into_0 ();
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
		
		RuntimePrimitiveN::Abort =>
			return Err (r#try! (abort_g (inputs, evaluator))),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessSpawn =>
			return process_spawn (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessExec =>
			return Err (Error::new_exec (r#try! (process_prepare (inputs, &mut Some (evaluator))))),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunTry =>
			return process_run (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunCheck =>
			return process_run_check (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveN::CacheOpen =>
			match inputs {
				[ref input_1] =>
					return cache_open (input_1.as_ref (), None, None, None, None, None, None),
				[ref input_1, ref input_2] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), None, None, None, None, None),
				[ref input_1, ref input_2, ref input_3] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), Some (input_3.as_ref ()), None, None, None, None),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), Some (input_3.as_ref ()), None, None, Some (input_4.as_ref ()), None),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), Some (input_3.as_ref ()), None, None, Some (input_4.as_ref ()), Some (input_5.as_ref ())),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), Some (input_3.as_ref ()), Some (input_6.as_ref ()), None, Some (input_4.as_ref ()), Some (input_5.as_ref ())),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6, ref input_7] =>
					return cache_open (input_1.as_ref (), Some (input_2.as_ref ()), Some (input_3.as_ref ()), Some (input_6.as_ref ()), Some (input_7.as_ref ()), Some (input_4.as_ref ()), Some (input_5.as_ref ())),
				_ =>
					fail! (0x30069f47),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveN::CacheSelectBytes =>
			match inputs {
				[ref input_1, ref input_2] =>
					return cache_select_bytes (input_1.as_ref (), None, input_2.as_ref (), None, None, None),
				[ref input_1, ref input_2, ref input_3] =>
					return cache_select_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), None, None, None),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_select_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_4.as_ref ()), None, None),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_select_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_4.as_ref ()), Some (input_5.as_ref ()), None),
				_ =>
					fail! (0xaa226112),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveN::CacheIncludeBytes =>
			match inputs {
				[ref input_1, ref input_2, ref input_3] =>
					return cache_include_bytes (input_1.as_ref (), None, input_2.as_ref (), input_3.as_ref (), None, None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_include_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), None, None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_include_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), Some (input_5.as_ref ()), None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6] =>
					return cache_include_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), Some (input_5.as_ref ()), Some (input_6.as_ref ()), None) .into_0 (),
				_ =>
					fail! (0x97a121a6),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveN::CacheResolveBytes =>
			match inputs {
				[ref input_1, ref input_2, ref input_3] =>
					return cache_resolve_bytes (input_1.as_ref (), None, input_2.as_ref (), None, None, None, input_3.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_resolve_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), None, None, None, input_4.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_resolve_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_5.as_ref ()), None, None, input_4.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6] =>
					return cache_resolve_bytes (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_5.as_ref ()), Some (input_6.as_ref ()), None, input_4.as_ref (), evaluator),
				_ =>
					fail! (0x8dfc8ed5),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveN::CacheSelectSerde =>
			match inputs {
				[ref input_1, ref input_2] =>
					return cache_select_serde (input_1.as_ref (), None, input_2.as_ref (), None, None, None),
				[ref input_1, ref input_2, ref input_3] =>
					return cache_select_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), None, None, None),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_select_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_4.as_ref ()), None, None),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_select_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_4.as_ref ()), Some (input_5.as_ref ()), None),
				_ =>
					fail! (0x471fac38),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveN::CacheIncludeSerde =>
			match inputs {
				[ref input_1, ref input_2, ref input_3] =>
					return cache_include_serde (input_1.as_ref (), None, input_2.as_ref (), input_3.as_ref (), None, None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_include_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), None, None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_include_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), Some (input_5.as_ref ()), None, None) .into_0 (),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6] =>
					return cache_include_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), input_4.as_ref (), Some (input_5.as_ref ()), Some (input_6.as_ref ()), None) .into_0 (),
				_ =>
					fail! (0x25233bc9),
			},
		
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveN::CacheResolveSerde =>
			match inputs {
				[ref input_1, ref input_2, ref input_3] =>
					return cache_resolve_serde (input_1.as_ref (), None, input_2.as_ref (), None, None, None, input_3.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4] =>
					return cache_resolve_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), None, None, None, input_4.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5] =>
					return cache_resolve_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_5.as_ref ()), None, None, input_4.as_ref (), evaluator),
				[ref input_1, ref input_2, ref input_3, ref input_4, ref input_5, ref input_6] =>
					return cache_resolve_serde (input_1.as_ref (), Some (input_2.as_ref ()), input_3.as_ref (), Some (input_5.as_ref ()), Some (input_6.as_ref ()), None, input_4.as_ref (), evaluator),
				_ =>
					fail! (0x39361efd),
			},
		
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
		RuntimePrimitiveV::Abort =>
			None,
		RuntimePrimitiveV::Pause =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
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
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitive1::Abort),
		RuntimePrimitiveV::Pause =>
			Some (RuntimePrimitive1::Pause),
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive1::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			Some (RuntimePrimitive1::ProcessExecExtended),
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
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
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			Some (RuntimePrimitive1::CacheExcludeAll),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			Some (RuntimePrimitive1::CachePruneAll),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			Some (RuntimePrimitive1::SipHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			Some (RuntimePrimitive1::SeaHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
			None,
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitive2::Abort),
		RuntimePrimitiveV::Pause =>
			Some (RuntimePrimitive2::Pause),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive2::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			Some (RuntimePrimitive2::ProcessExecExtended),
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
			None,
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
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			Some (RuntimePrimitive2::CacheExcludeAll),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			Some (RuntimePrimitive2::CachePruneAll),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			Some (RuntimePrimitive2::SipHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			Some (RuntimePrimitive2::SeaHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			Some (RuntimePrimitive2::Blake2bHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
			Some (RuntimePrimitive2::Blake2sHashSeeded),
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitive3::Abort),
		RuntimePrimitiveV::Pause =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive3::ProcessSpawnExtended),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			Some (RuntimePrimitive3::ProcessExecExtended),
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
			Some (RuntimePrimitive3::CacheResolveBytes),
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
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			Some (RuntimePrimitive3::CacheResolveSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			Some (RuntimePrimitive3::CachePruneAll),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			Some (RuntimePrimitive3::Blake2bHashSeeded),
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
			Some (RuntimePrimitive3::Blake2sHashSeeded),
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitive4::Abort),
		RuntimePrimitiveV::Pause =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitive4::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			Some (RuntimePrimitive4::CacheSelectBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			Some (RuntimePrimitive4::CacheIncludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
			Some (RuntimePrimitive4::CacheResolveBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			Some (RuntimePrimitive4::CacheSelectSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			Some (RuntimePrimitive4::CacheIncludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			Some (RuntimePrimitive4::CacheResolveSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitive5::Abort),
		RuntimePrimitiveV::Pause =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			Some (RuntimePrimitive5::CacheSelectBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			Some (RuntimePrimitive5::CacheIncludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
			Some (RuntimePrimitive5::CacheResolveBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			Some (RuntimePrimitive5::CacheSelectSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			Some (RuntimePrimitive5::CacheIncludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			Some (RuntimePrimitive5::CacheResolveSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
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
		RuntimePrimitiveV::Abort =>
			Some (RuntimePrimitiveN::Abort),
		RuntimePrimitiveV::Pause =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessExecExtended =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheOpen =>
			Some (RuntimePrimitiveN::CacheOpen),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheSelectBytes =>
			Some (RuntimePrimitiveN::CacheSelectBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheIncludeBytes =>
			Some (RuntimePrimitiveN::CacheIncludeBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheExcludeBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		RuntimePrimitiveV::CacheResolveBytes =>
			Some (RuntimePrimitiveN::CacheResolveBytes),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheSelectSerde =>
			Some (RuntimePrimitiveN::CacheSelectSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheIncludeSerde =>
			Some (RuntimePrimitiveN::CacheIncludeSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheExcludeSerde =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_serde" ) ]
		RuntimePrimitiveV::CacheResolveSerde =>
			Some (RuntimePrimitiveN::CacheResolveSerde),
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CacheExcludeAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_cache" ) ]
		RuntimePrimitiveV::CachePruneAll =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
		RuntimePrimitiveV::SipHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
		RuntimePrimitiveV::SeaHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2bHashSeeded =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
		RuntimePrimitiveV::Blake2sHashSeeded =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_attributes (_primitive : RuntimePrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_attributes (_primitive : RuntimePrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_attributes (_primitive : RuntimePrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_attributes (_primitive : RuntimePrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_attributes (_primitive : RuntimePrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_attributes (_primitive : RuntimePrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_attributes (_primitive : RuntimePrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

