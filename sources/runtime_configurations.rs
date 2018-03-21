

pub mod exports {
	pub use super::*;
}




pub const DEFAULT_NEW_IMMUTABLE : bool = cfg! (feature = "vonuvoli_new_immutable");
pub const STRING_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
pub const BYTES_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
pub const ARRAY_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
pub const RECORD_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;
pub const PAIR_NEW_IMMUTABLE : bool = DEFAULT_NEW_IMMUTABLE && true;

pub const PARAMETER_NEW_IMMUTABLE : bool = true;




pub const COMPILER_TRACE_INPUT : bool = false;
pub const COMPILER_TRACE_OUTPUT : bool = false;
pub const COMPILER_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_compiler_trace_error");

pub const OPTIMIZER_TRACE_INPUT : bool = false;
pub const OPTIMIZER_TRACE_OUTPUT : bool = false;
pub const OPTIMIZER_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_optimizer_trace_error");

pub const EVALUATOR_TRACE_INPUT : bool = false;
pub const EVALUATOR_TRACE_OUTPUT : bool = false;
pub const EVALUATOR_TRACE_ERROR : bool = cfg! (feature = "vonuvoli_evaluator_trace_error");




pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN : bool = true;
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_display_flush");
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR : char = ' ';

pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN : bool = false;
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_write_flush");
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR : char = ' ';

pub const DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH : bool = cfg! (feature = "vonuvoli_port_output_newline_flush");
pub const DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR : char = '\n';

pub const DEFAULT_PORT_BUFFER_SIZE : usize = 16 * 1024;




pub const PANIC_ON_FAILED : bool = cfg! (feature = "vonuvoli_panic_on_failed");
pub const ABORT_ON_PANIC_WITH_ERROR : bool = cfg! (feature = "vonuvoli_abort_on_panic");
pub const ABORT_ON_PANIC_GENERIC : bool =  cfg! (feature = "vonuvoli_abort_on_panic");
pub const ERRORS_WITH_BACKTRACE : bool = cfg! (feature = "vonuvoli_backtrace");


pub const TRANSCRIPT_OUTPUT_COMPACT : bool = cfg! (feature = "vonuvoli_transcript_compact");

pub const TRANSCRIPT_OUTPUT_MULTILINE_ALIGN_RIGHT : bool = false || TRANSCRIPT_OUTPUT_COMPACT;
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_BEFORE : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_HEADER : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;
pub const TRANSCRIPT_OUTPUT_MULTILINE_BREAK_AFTER : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;

pub const TRANSCRIPT_OUTPUT_SEPARATOR_UNIT_BREAK : bool = true && ! TRANSCRIPT_OUTPUT_COMPACT;

pub const TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ENABLED : bool = cfg! (feature = "vonuvoli_transcript_ansi_enabled");
pub const TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ALWAYS : bool = cfg! (feature = "vonuvoli_transcript_ansi_always") && TRANSCRIPT_OUTPUT_SUPPORTS_ANSI_SEQUENCES_ENABLED;

pub const TRANSCRIPT_BUFFER_SIZE : usize = 1 * 1024;

