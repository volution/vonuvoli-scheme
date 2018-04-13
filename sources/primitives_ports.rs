

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

#[ allow (unused_imports) ]
use super::conversions::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::PortPrimitive0;
	pub use super::PortPrimitive1;
	pub use super::PortPrimitive2;
	pub use super::PortPrimitive3;
	pub use super::PortPrimitive4;
	pub use super::PortPrimitive5;
	pub use super::PortPrimitiveN;
	pub use super::PortPrimitiveV;
	
	pub use super::port_primitive_0_evaluate;
	pub use super::port_primitive_1_evaluate;
	pub use super::port_primitive_2_evaluate;
	pub use super::port_primitive_3_evaluate;
	pub use super::port_primitive_4_evaluate;
	pub use super::port_primitive_5_evaluate;
	pub use super::port_primitive_n_evaluate;
	
	pub use super::port_primitive_v_alternative_0;
	pub use super::port_primitive_v_alternative_1;
	pub use super::port_primitive_v_alternative_2;
	pub use super::port_primitive_v_alternative_3;
	pub use super::port_primitive_v_alternative_4;
	pub use super::port_primitive_v_alternative_5;
	pub use super::port_primitive_v_alternative_n;
	
	pub use super::port_primitive_0_attributes;
	pub use super::port_primitive_1_attributes;
	pub use super::port_primitive_2_attributes;
	pub use super::port_primitive_3_attributes;
	pub use super::port_primitive_4_attributes;
	pub use super::port_primitive_5_attributes;
	pub use super::port_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive0 {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	OutputToBytes,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	OutputToString,
	
	CurrentInput,
	CurrentOutput,
	CurrentError,
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	Eof,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteReady,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytePeek,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteRead,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterReady,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterPeek,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	
	ValueRead,
	
	NewLine,
	
	FlushOutput,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive1 {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	OutputToBytes,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	OutputToString,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	InputFromBytes,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	InputFromString,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	OutputToBytesFinalize,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	OutputToStringFinalize,
	
	OpenBinaryInput,
	OpenBinaryOutput,
	
	OpenTextualInput,
	OpenTextualOutput,
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteReady,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytePeek,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteRead,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterReady,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterPeek,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	
	ValueRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
	ValueWriteAndNewLine,
	ValueDisplayAndNewLine,
	
	NewLine,
	FlushOutput,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive2 {
	
	CallAndClose,
	
	OpenBinaryInputThenCallAndClose,
	OpenBinaryOutputThenCallAndClose,
	OpenTextualInputThenCallAndClose,
	OpenTextualOutputThenCallAndClose,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	WithOpenBinaryInputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	WithOpenBinaryOutputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	WithOpenTextualInputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	WithOpenTextualOutputThenCallAndClose,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
	ValueWriteAndNewLine,
	ValueDisplayAndNewLine,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive3 {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive4 {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive5 {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveN {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveV {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	OutputToBytes,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	OutputToString,
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteReady,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytePeek,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteRead,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterReady,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterPeek,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	
	ValueRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
	ValueWriteAndNewLine,
	ValueDisplayAndNewLine,
	
	NewLine,
	FlushOutput,
	
}




#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
macro_rules! stdin_val {
	( $evaluator : expr ) => (
		try! (try! ($evaluator .parameters ()) .resolve_stdin_value ())
	);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
macro_rules! stdout_val {
	( $evaluator : expr ) => (
		try! (try! ($evaluator .parameters ()) .resolve_stdout_value ())
	);
}

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
macro_rules! stderr_val {
	( $evaluator : expr ) => (
		try! (try! ($evaluator .parameters ()) .resolve_stderr_value ())
	);
}




#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
macro_rules! stdin_val {
	( $evaluator : expr ) => (
		Value::from (try! (Port::new_stdin ()))
	);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
macro_rules! stdout_val {
	( $evaluator : expr ) => (
		Value::from (try! (Port::new_stdout ()))
	);
}

#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
macro_rules! stderr_val {
	( $evaluator : expr ) => (
		Value::from (try! (Port::new_stderr ()))
	);
}




macro_rules! stdin_ref {
	( $evaluator : expr ) => (
		& stdin_val! ($evaluator)
	);
}

macro_rules! stdout_ref {
	( $evaluator : expr ) => (
		& stdout_val! ($evaluator)
	);
}

macro_rules! stderr_ref {
	( $evaluator : expr ) => (
		& stderr_val! ($evaluator)
	);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_0_evaluate (primitive : PortPrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::OutputToBytes =>
			return port_bytes_writer_new (None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::OutputToString =>
			return port_string_writer_new (None),
		
		PortPrimitive0::CurrentInput =>
			succeed! (stdin_val! (evaluator)),
		
		PortPrimitive0::CurrentOutput =>
			succeed! (stdout_val! (evaluator)),
		
		PortPrimitive0::CurrentError =>
			succeed! (stderr_val! (evaluator)),
		
		PortPrimitive0::IsInputOpen =>
			return is_port_input_open (stdin_ref! (evaluator)) .into_0 (),
		
		PortPrimitive0::IsOutputOpen =>
			return is_port_output_open (stdout_ref! (evaluator)) .into_0 (),
		
		PortPrimitive0::Close => {
			try! (port_input_close (stdin_ref! (evaluator)));
			try! (port_output_close (stdout_ref! (evaluator)));
			succeed! (VOID_VALUE);
		},
		
		PortPrimitive0::CloseInput =>
			return port_input_close (stdin_ref! (evaluator)) .into_0 (),
		
		PortPrimitive0::CloseOutput =>
			return port_output_close (stdout_ref! (evaluator)) .into_0 (),
		
		PortPrimitive0::Eof =>
			return PORT_EOF.into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::ByteReady =>
			return port_input_byte_ready (stdin_ref! (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytePeek =>
			return port_input_byte_peek (stdin_ref! (evaluator)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::ByteRead =>
			return port_input_byte_read (stdin_ref! (evaluator)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::CharacterReady =>
			return port_input_character_ready (stdin_ref! (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::CharacterPeek =>
			return port_input_character_peek (stdin_ref! (evaluator)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::CharacterRead =>
			return port_input_character_read (stdin_ref! (evaluator)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadCollect =>
			return port_input_bytes_read_collect (stdin_ref! (evaluator), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadChunk =>
			return port_input_bytes_read_collect (stdin_ref! (evaluator), None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadLine =>
			return port_input_bytes_read_line (stdin_ref! (evaluator), Some (false), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadCollect =>
			return port_input_string_read_collect (stdin_ref! (evaluator), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadChunk =>
			return port_input_string_read_collect (stdin_ref! (evaluator), None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadLine =>
			return port_input_string_read_line (stdin_ref! (evaluator), Some (false), None, Some (true)),
		
		PortPrimitive0::ValueRead =>
			fail_unimplemented! (0x75ffa1de), // deferred
		
		PortPrimitive0::NewLine =>
			return port_output_newline (stdout_ref! (evaluator), None, Some (true)) .into_0 (),
		
		PortPrimitive0::FlushOutput =>
			return port_output_flush (stdout_ref! (evaluator)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::OutputToBytes =>
			fail_unimplemented! (0xf2c90a09), // deferred
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::OutputToString =>
			fail_unimplemented! (0xe31eb4dd), // deferred
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::InputFromBytes =>
			return port_bytes_reader_new (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::InputFromString =>
			return port_string_reader_new (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::OutputToBytesFinalize =>
			return port_bytes_writer_finalize (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::OutputToStringFinalize =>
			return port_string_writer_finalize (input_1),
		
		PortPrimitive1::OpenBinaryInput =>
			return port_file_reader_open (input_1, None),
		
		PortPrimitive1::OpenBinaryOutput =>
			return port_file_writer_open (input_1, None),
		
		PortPrimitive1::OpenTextualInput =>
			return port_file_reader_open (input_1, None),
		
		PortPrimitive1::OpenTextualOutput =>
			return port_file_writer_open (input_1, None),
		
		PortPrimitive1::IsInputOpen =>
			return is_port_input_open (input_1) .into_0 (),
		
		PortPrimitive1::IsOutputOpen =>
			return is_port_output_open (input_1) .into_0 (),
		
		PortPrimitive1::Close =>
			return port_close (input_1) .into_0 (),
		
		PortPrimitive1::CloseInput =>
			return port_input_close (input_1) .into_0 (),
		
		PortPrimitive1::CloseOutput =>
			return port_output_close (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::ByteReady =>
			return port_input_byte_ready (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytePeek =>
			return port_input_byte_peek (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::ByteRead =>
			return port_input_byte_read (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::CharacterReady =>
			return port_input_character_ready (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::CharacterPeek =>
			return port_input_character_peek (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::CharacterRead =>
			return port_input_character_read (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadCopy =>
			return port_input_bytes_read_copy_range (stdin_ref! (evaluator), input_1, None, None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadExtend =>
			return port_input_bytes_read_extend (stdin_ref! (evaluator), input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadCollect =>
			return port_input_bytes_read_collect (stdin_ref! (evaluator), Some (input_1), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadChunk =>
			return port_input_bytes_read_collect (input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadLine =>
			return port_input_bytes_read_line (input_1, Some (false), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadExtend =>
			return port_input_string_read_extend (stdin_ref! (evaluator), input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadCollect =>
			return port_input_string_read_collect (stdin_ref! (evaluator), Some (input_1), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadChunk =>
			return port_input_string_read_collect (input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadLine =>
			return port_input_string_read_line (input_1, Some (false), None, Some (true)),
		
		PortPrimitive1::ValueRead =>
			fail_unimplemented! (0xae3d8a9f), // deferred
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::ByteWrite =>
			return port_output_byte_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesWrite =>
			return port_output_bytes_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::CharacterWrite =>
			return port_output_character_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringWrite =>
			return port_output_string_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		PortPrimitive1::ValueWrite =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive1::ValueWriteShared =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive1::ValueWriteSimple =>
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive1::ValueDisplay =>
			return port_output_value_display (stdout_ref! (evaluator), input_1, None, None, None, Some (true)) .into_0 (),
		
		PortPrimitive1::ValueWriteAndNewLine =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), None) .into_0 (),
		
		PortPrimitive1::ValueDisplayAndNewLine =>
			return port_output_value_display (stdout_ref! (evaluator), input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), Some (true)) .into_0 (),
		
		PortPrimitive1::NewLine =>
			return port_output_newline (input_1, None, Some (true)) .into_0 (),
		
		PortPrimitive1::FlushOutput =>
			return port_output_flush (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_2_evaluate (primitive : PortPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive2::CallAndClose =>
			return port_call_and_close_1 (input_1, input_2, evaluator),
		
		PortPrimitive2::OpenBinaryInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryInput, input_1, evaluator));
			return port_call_and_close_1 (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenBinaryOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryOutput, input_1, evaluator));
			return port_call_and_close_1 (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualInput, input_1, evaluator));
			return port_call_and_close_1 (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualOutput, input_1, evaluator));
			return port_call_and_close_1 (&port, input_2, evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		PortPrimitive2::WithOpenBinaryInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryInput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		PortPrimitive2::WithOpenBinaryOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryOutput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		PortPrimitive2::WithOpenTextualInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualInput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		PortPrimitive2::WithOpenTextualOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualOutput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, None, None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadCollect =>
			return port_input_bytes_read_collect (input_2, Some (input_1), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadChunk =>
			return port_input_bytes_read_collect (input_1, Some (input_2), Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadLine =>
			return port_input_bytes_read_line (input_1, Some (false), Some (input_2), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadCollect =>
			return port_input_string_read_collect (input_2, Some (input_1), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadChunk =>
			return port_input_string_read_collect (input_1, Some (input_2), Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadLine =>
			return port_input_string_read_line (input_1, Some (false), Some (input_2), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::ByteWrite =>
			return port_output_byte_write (input_2, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesWrite =>
			return port_output_bytes_write (input_2, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::CharacterWrite =>
			return port_output_character_write (input_2, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringWrite =>
			return port_output_string_write (input_2, input_1) .into_0 (),
		
		PortPrimitive2::ValueWrite =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueWriteShared =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueWriteSimple =>
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueDisplay =>
			return port_output_value_display (input_2, input_1, None, None, None, Some (true)) .into_0 (),
		
		PortPrimitive2::ValueWriteAndNewLine =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (input_2, input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), None) .into_0 (),
		
		PortPrimitive2::ValueDisplayAndNewLine =>
			return port_output_value_display (input_2, input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), Some (true)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, Some (input_3), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, Some (input_3), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesWrite =>
			fail_unimplemented! (0xe9bfad62),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringWrite =>
			fail_unimplemented! (0x0145ea8e),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), Some (input_4), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, Some (input_3), Some (try! (boolean_coerce (input_4)))),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, Some (input_3), Some (try! (boolean_coerce (input_4)))),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesWrite =>
			fail_unimplemented! (0x2e16ec86),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringWrite =>
			fail_unimplemented! (0xa5f90fe1),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ allow (unused_variables) ]
pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive5::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), Some (input_4), Some (try! (boolean_coerce (input_5)))),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_0 (primitive : PortPrimitiveV) -> (Option<PortPrimitive0>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			Some (PortPrimitive0::OutputToBytes),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			Some (PortPrimitive0::OutputToString),
		PortPrimitiveV::IsInputOpen =>
			Some (PortPrimitive0::IsInputOpen),
		PortPrimitiveV::IsOutputOpen =>
			Some (PortPrimitive0::IsOutputOpen),
		PortPrimitiveV::Close =>
			Some (PortPrimitive0::Close),
		PortPrimitiveV::CloseInput =>
			Some (PortPrimitive0::CloseInput),
		PortPrimitiveV::CloseOutput =>
			Some (PortPrimitive0::CloseOutput),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			Some (PortPrimitive0::ByteReady),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			Some (PortPrimitive0::BytePeek),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			Some (PortPrimitive0::ByteRead),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			Some (PortPrimitive0::CharacterReady),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			Some (PortPrimitive0::CharacterPeek),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			Some (PortPrimitive0::CharacterRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			Some (PortPrimitive0::BytesReadCollect),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			Some (PortPrimitive0::BytesReadChunk),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			Some (PortPrimitive0::BytesReadLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			Some (PortPrimitive0::StringReadCollect),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			Some (PortPrimitive0::StringReadChunk),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			Some (PortPrimitive0::StringReadLine),
		PortPrimitiveV::ValueRead =>
			Some (PortPrimitive0::ValueRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		PortPrimitiveV::ValueWrite =>
			None,
		PortPrimitiveV::ValueWriteShared =>
			None,
		PortPrimitiveV::ValueWriteSimple =>
			None,
		PortPrimitiveV::ValueDisplay =>
			None,
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			Some (PortPrimitive0::NewLine),
		PortPrimitiveV::FlushOutput =>
			Some (PortPrimitive0::FlushOutput),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_1 (primitive : PortPrimitiveV) -> (Option<PortPrimitive1>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			Some (PortPrimitive1::OutputToBytes),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			Some (PortPrimitive1::OutputToString),
		PortPrimitiveV::IsInputOpen =>
			Some (PortPrimitive1::IsInputOpen),
		PortPrimitiveV::IsOutputOpen =>
			Some (PortPrimitive1::IsOutputOpen),
		PortPrimitiveV::Close =>
			Some (PortPrimitive1::Close),
		PortPrimitiveV::CloseInput =>
			Some (PortPrimitive1::CloseInput),
		PortPrimitiveV::CloseOutput =>
			Some (PortPrimitive1::CloseOutput),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			Some (PortPrimitive1::ByteReady),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			Some (PortPrimitive1::BytePeek),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			Some (PortPrimitive1::ByteRead),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			Some (PortPrimitive1::CharacterReady),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			Some (PortPrimitive1::CharacterPeek),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			Some (PortPrimitive1::CharacterRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive1::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			Some (PortPrimitive1::BytesReadExtend),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			Some (PortPrimitive1::BytesReadCollect),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			Some (PortPrimitive1::BytesReadChunk),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			Some (PortPrimitive1::BytesReadLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			Some (PortPrimitive1::StringReadExtend),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			Some (PortPrimitive1::StringReadCollect),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			Some (PortPrimitive1::StringReadChunk),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			Some (PortPrimitive1::StringReadLine),
		PortPrimitiveV::ValueRead =>
			Some (PortPrimitive1::ValueRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			Some (PortPrimitive1::ByteWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive1::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			Some (PortPrimitive1::CharacterWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive1::StringWrite),
		PortPrimitiveV::ValueWrite =>
			Some (PortPrimitive1::ValueWrite),
		PortPrimitiveV::ValueWriteShared =>
			Some (PortPrimitive1::ValueWriteShared),
		PortPrimitiveV::ValueWriteSimple =>
			Some (PortPrimitive1::ValueWriteSimple),
		PortPrimitiveV::ValueDisplay =>
			Some (PortPrimitive1::ValueDisplay),
		PortPrimitiveV::ValueWriteAndNewLine =>
			Some (PortPrimitive1::ValueWriteAndNewLine),
		PortPrimitiveV::ValueDisplayAndNewLine =>
			Some (PortPrimitive1::ValueDisplayAndNewLine),
		PortPrimitiveV::NewLine =>
			Some (PortPrimitive1::NewLine),
		PortPrimitiveV::FlushOutput =>
			Some (PortPrimitive1::FlushOutput),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_2 (primitive : PortPrimitiveV) -> (Option<PortPrimitive2>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			None,
		PortPrimitiveV::IsInputOpen =>
			None,
		PortPrimitiveV::IsOutputOpen =>
			None,
		PortPrimitiveV::Close =>
			None,
		PortPrimitiveV::CloseInput =>
			None,
		PortPrimitiveV::CloseOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive2::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			Some (PortPrimitive2::BytesReadExtend),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			Some (PortPrimitive2::BytesReadCollect),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			Some (PortPrimitive2::BytesReadChunk),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			Some (PortPrimitive2::BytesReadLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			Some (PortPrimitive2::StringReadExtend),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			Some (PortPrimitive2::StringReadCollect),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			Some (PortPrimitive2::StringReadChunk),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			Some (PortPrimitive2::StringReadLine),
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			Some (PortPrimitive2::ByteWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive2::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			Some (PortPrimitive2::CharacterWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive2::StringWrite),
		PortPrimitiveV::ValueWrite =>
			Some (PortPrimitive2::ValueWrite),
		PortPrimitiveV::ValueWriteShared =>
			Some (PortPrimitive2::ValueWriteShared),
		PortPrimitiveV::ValueWriteSimple =>
			Some (PortPrimitive2::ValueWriteSimple),
		PortPrimitiveV::ValueDisplay =>
			Some (PortPrimitive2::ValueDisplay),
		PortPrimitiveV::ValueWriteAndNewLine =>
			Some (PortPrimitive2::ValueWriteAndNewLine),
		PortPrimitiveV::ValueDisplayAndNewLine =>
			Some (PortPrimitive2::ValueDisplayAndNewLine),
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_3 (primitive : PortPrimitiveV) -> (Option<PortPrimitive3>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			None,
		PortPrimitiveV::IsInputOpen =>
			None,
		PortPrimitiveV::IsOutputOpen =>
			None,
		PortPrimitiveV::Close =>
			None,
		PortPrimitiveV::CloseInput =>
			None,
		PortPrimitiveV::CloseOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive3::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			Some (PortPrimitive3::BytesReadExtend),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			Some (PortPrimitive3::StringReadExtend),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			None,
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive3::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive3::StringWrite),
		PortPrimitiveV::ValueWrite =>
			None,
		PortPrimitiveV::ValueWriteShared =>
			None,
		PortPrimitiveV::ValueWriteSimple =>
			None,
		PortPrimitiveV::ValueDisplay =>
			None,
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_4 (primitive : PortPrimitiveV) -> (Option<PortPrimitive4>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			None,
		PortPrimitiveV::IsInputOpen =>
			None,
		PortPrimitiveV::IsOutputOpen =>
			None,
		PortPrimitiveV::Close =>
			None,
		PortPrimitiveV::CloseInput =>
			None,
		PortPrimitiveV::CloseOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive4::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			Some (PortPrimitive4::BytesReadExtend),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			Some (PortPrimitive4::StringReadExtend),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			None,
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive4::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive4::StringWrite),
		PortPrimitiveV::ValueWrite =>
			None,
		PortPrimitiveV::ValueWriteShared =>
			None,
		PortPrimitiveV::ValueWriteSimple =>
			None,
		PortPrimitiveV::ValueDisplay =>
			None,
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_5 (primitive : PortPrimitiveV) -> (Option<PortPrimitive5>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			None,
		PortPrimitiveV::IsInputOpen =>
			None,
		PortPrimitiveV::IsOutputOpen =>
			None,
		PortPrimitiveV::Close =>
			None,
		PortPrimitiveV::CloseInput =>
			None,
		PortPrimitiveV::CloseOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive5::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			None,
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		PortPrimitiveV::ValueWrite =>
			None,
		PortPrimitiveV::ValueWriteShared =>
			None,
		PortPrimitiveV::ValueWriteSimple =>
			None,
		PortPrimitiveV::ValueDisplay =>
			None,
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_n (primitive : PortPrimitiveV) -> (Option<PortPrimitiveN>) {
	match primitive {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::OutputToBytes =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::OutputToString =>
			None,
		PortPrimitiveV::IsInputOpen =>
			None,
		PortPrimitiveV::IsOutputOpen =>
			None,
		PortPrimitiveV::Close =>
			None,
		PortPrimitiveV::CloseInput =>
			None,
		PortPrimitiveV::CloseOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytePeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterReady =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterPeek =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadExtend =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollect =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunk =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLine =>
			None,
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		PortPrimitiveV::ValueWrite =>
			None,
		PortPrimitiveV::ValueWriteShared =>
			None,
		PortPrimitiveV::ValueWriteSimple =>
			None,
		PortPrimitiveV::ValueDisplay =>
			None,
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_0_attributes (_primitive : PortPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_1_attributes (_primitive : PortPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_2_attributes (_primitive : PortPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_3_attributes (_primitive : PortPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_4_attributes (_primitive : PortPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_5_attributes (_primitive : PortPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_attributes (_primitive : PortPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

