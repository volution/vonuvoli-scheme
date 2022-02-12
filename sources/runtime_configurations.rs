

pub mod exports {
	pub use super::*;
}




#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const DEFAULT_NEW_IMMUTABLE : bool = cfg! (feature = "vonuvoli_new_immutable");

#[ cfg ( feature = "vonuvoli_values_string" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const STRING_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const BYTES_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
#[ cfg ( feature = "vonuvoli_values_array" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const ARRAY_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const RECORD_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;

#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
pub const PAIR_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
pub const PARAMETER_NEW_IMMUTABLE : bool = true;




#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const COMPILER_TRACE_INPUT : bool = false;
#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const COMPILER_TRACE_OUTPUT : bool = false;
#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const COMPILER_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_compiler_trace_error");

#[ cfg ( feature = "vonuvoli_optimizer_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const OPTIMIZER_TRACE_INPUT : bool = false;
#[ cfg ( feature = "vonuvoli_optimizer_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const OPTIMIZER_TRACE_OUTPUT : bool = false;
#[ cfg ( feature = "vonuvoli_optimizer_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const OPTIMIZER_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_optimizer_trace_error");

#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const EVALUATOR_TRACE_INPUT : bool = false;
#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const EVALUATOR_TRACE_OUTPUT : bool = false;
#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const EVALUATOR_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_evaluator_trace_error");

#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const PARSER_TRACE_INPUT : bool = false;
#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const PARSER_TRACE_OUTPUT : bool = false;
#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const PARSER_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_parser_trace_error");

#[ cfg ( feature = "vonuvoli_tests" ) ]
pub const TESTS_FAIL_ON_FIRST_ERROR : bool = true;




#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN : bool = true;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_display_flush");
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR : char = ' ';

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN : bool = false;
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_write_flush");
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR : char = ' ';

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_newline_flush");
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR : char = '\n';
#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_OUTPUT_ZERO_SEPARATOR : char = '\0';

#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
pub const DEFAULT_PORT_BUFFER_SIZE : usize = 16 * 1024;




pub const PANIC_ON_FAILED : bool = cfg! (feature = "vonuvoli_panic_on_failed");
pub const ABORT_ON_PANIC_WITH_ERROR : bool = cfg! (feature = "vonuvoli_abort_on_panic");
pub const ABORT_ON_PANIC_GENERIC : bool =  cfg! (feature = "vonuvoli_abort_on_panic");

#[ cfg ( feature = "vonuvoli_backtrace" ) ]
pub const ERRORS_WITH_BACKTRACE : bool = cfg! (feature = "vonuvoli_backtrace");


#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_COMPACT : bool = cfg! (feature = "vonuvoli_transcript_compact");

#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_MULTILINE_ALIGN_RIGHT : bool = false || TRANSCRIPT_OUTPUT_COMPACT;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_BEFORE : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_HEADER : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_AFTER : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_SEPARATOR_UNIT_BREAK : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ENABLED : bool = cfg! (feature = "vonuvoli_transcript_ansi_enabled");
#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ALWAYS : bool = cfg! (feature = "vonuvoli_transcript_ansi_always") && TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ENABLED;


#[ cfg ( feature = "vonuvoli_transcript" ) ]
pub const TRANSCRIPT_BUFFER_SIZE : usize = 1024;




// FIXME:  Replace token with build-related token!
#[ cfg ( feature = "vonuvoli_release" ) ]
pub const BUILD_KEY : [u8; 32] = *b"479b80b437dba9e68c0c024b2cf9c6fc";

// FIXME:  Replace token with random token (different for each build)!
#[ cfg ( not ( feature = "vonuvoli_release" ) ) ]
pub const BUILD_KEY : [u8; 32] = *b"0a8ac11b09fd60b9c4553a3aeaff239c";

