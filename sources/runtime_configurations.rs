

pub mod exports {
	pub use super::*;
}




pub const STRING_NEW_IMMUTABLE : bool = true;
pub const BYTES_NEW_IMMUTABLE : bool = true;
pub const ARRAY_NEW_IMMUTABLE : bool = true;
pub const RECORD_NEW_IMMUTABLE : bool = true;

pub const PAIR_NEW_IMMUTABLE : bool = true;

pub const PARAMETER_NEW_IMMUTABLE : bool = true;




pub const COMPILER_TRACE_INPUT : bool = false;
pub const COMPILER_TRACE_OUTPUT : bool = false;
pub const COMPILER_TRACE_ERROR : bool = true;

pub const OPTIMIZER_TRACE_INPUT : bool = false;
pub const OPTIMIZER_TRACE_OUTPUT : bool = false;
pub const OPTIMIZER_TRACE_ERROR : bool = false;

pub const EVALUATOR_TRACE_INPUT : bool = false;
pub const EVALUATOR_TRACE_OUTPUT : bool = false;
pub const EVALUATOR_TRACE_ERROR : bool = false;




pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN : bool = true;
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLUSH : bool = true;
pub const DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR : char = ' ';

pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN : bool = false;
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH : bool = false;
pub const DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR : char = ' ';

pub const DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH : bool = true;
pub const DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR : char = '\n';

pub const DEFAULT_PORT_BUFFER_SIZE : usize = 16 * 1024;




pub const PANIC_ON_FAILED : bool = false;

