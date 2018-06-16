

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;
use super::primitives_procedures::exports::*;
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
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_0_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_1_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_2_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_3_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_4_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_5_attributes;
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::port_primitive_n_attributes;
	
}




include! ("./macros_primitives.in");




def_primitives_enum! (PortPrimitive0, (procedure, 0), {
	
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
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueRead,
	
	NewLine,
	
	FlushOutput,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});


def_primitives_enum! (PortPrimitive1, (procedure, 1), {
	
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
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWrite,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteShared,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteSimple,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplay,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteAndNewLine,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplayAndNewLine,
	
	NewLine,
	FlushOutput,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	DescriptorGet,
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	DescriptorClone,
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	DescriptorValue,
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	DescriptorPath,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryRelease,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
	TemporaryPath,
	
});


def_primitives_enum! (PortPrimitive2, (procedure, 2), {
	
	CallAndClose,
	
	OpenBinaryInputThenCallAndClose,
	OpenBinaryOutputThenCallAndClose,
	OpenTextualInputThenCallAndClose,
	OpenTextualOutputThenCallAndClose,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	WithOpenBinaryInputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	WithOpenBinaryOutputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	WithOpenTextualInputThenCallAndClose,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg ( feature = "vonuvoli_evaluator" ) ]
	WithOpenTextualOutputThenCallAndClose,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZero,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueReadFold,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWrite,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteShared,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteSimple,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplay,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteAndNewLine,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplayAndNewLine,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	DescriptorFlagGet,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});


def_primitives_enum! (PortPrimitive3, (procedure, 3), {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueReadFold,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
	DescriptorFlagSet,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});


def_primitives_enum! (PortPrimitive4, (procedure, 4), {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReadExtend,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});


def_primitives_enum! (PortPrimitive5, (procedure, 5), {
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});


def_primitives_enum! (PortPrimitiveN, (procedure, n), {});


def_primitives_enum! (PortPrimitiveV, (procedure, v), {
	
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
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadCopy,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	BytesReadExtend,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollect,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunk,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
	StringReadExtend,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollect,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunk,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueRead,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadCollectFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadChunkFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadLineFold,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringReadZeroFold,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
	ValueReadFold,
	
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	ByteWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWrite,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteLine,
	#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
	BytesWriteZero,
	
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	CharacterWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWrite,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteLine,
	#[ cfg ( feature = "vonuvoli_values_string" ) ]
	StringWriteZero,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWrite,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteShared,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteSimple,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplay,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueWriteAndNewLine,
	#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
	ValueDisplayAndNewLine,
	
	NewLine,
	FlushOutput,
	
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryBinaryCreate,
	#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
	TemporaryTextualCreate,
	
});




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
			return port_input_bytes_read_collect (stdin_ref! (evaluator), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadChunk =>
			return port_input_bytes_read_collect (stdin_ref! (evaluator), None, Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadLine =>
			return port_input_bytes_read_collect_line (stdin_ref! (evaluator), Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive0::BytesReadZero =>
			return port_input_bytes_read_collect_zero (stdin_ref! (evaluator), Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadCollect =>
			return port_input_string_read_collect (stdin_ref! (evaluator), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadChunk =>
			return port_input_string_read_collect (stdin_ref! (evaluator), None, Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadLine =>
			return port_input_string_read_collect_line (stdin_ref! (evaluator), Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive0::StringReadZero =>
			return port_input_string_read_collect_zero (stdin_ref! (evaluator), Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitive0::ValueRead =>
			fail_unimplemented! (0x75ffa1de, (github_issue, 19)),
		
		PortPrimitive0::NewLine =>
			return port_output_newline (stdout_ref! (evaluator), None, Some (true)) .into_0 (),
		
		PortPrimitive0::FlushOutput =>
			return port_output_flush (stdout_ref! (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive0::TemporaryBinaryCreate =>
			return port_temporary_create (None, None, None, None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive0::TemporaryTextualCreate =>
			return port_temporary_create (None, None, None, None, None, None) .into_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::OutputToBytes =>
			fail_unimplemented! (0xf2c90a09, (github_issue, 53)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::OutputToString =>
			fail_unimplemented! (0xe31eb4dd, (github_issue, 53)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::InputFromBytes =>
			return port_bytes_reader_new (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::InputFromString =>
			return port_string_reader_new (input_1),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::OutputToBytesFinalize =>
			return port_bytes_writer_finalize (input_1, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::OutputToStringFinalize =>
			return port_string_writer_finalize (input_1, None),
		
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive1::BytesReadCopy =>
			return port_input_bytes_read_copy_range (stdin_ref! (evaluator), input_1, None, None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive1::BytesReadExtend =>
			return port_input_bytes_read_extend (stdin_ref! (evaluator), input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadCollect =>
			return port_input_bytes_read_collect (stdin_ref! (evaluator), Some (input_1), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadChunk =>
			return port_input_bytes_read_collect (input_1, None, Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadLine =>
			return port_input_bytes_read_collect_line (input_1, Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesReadZero =>
			return port_input_bytes_read_collect_zero (input_1, Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive1::StringReadExtend =>
			return port_input_string_read_extend (stdin_ref! (evaluator), input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadCollect =>
			return port_input_string_read_collect (stdin_ref! (evaluator), Some (input_1), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadChunk =>
			return port_input_string_read_collect (input_1, None, Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadLine =>
			return port_input_string_read_collect_line (input_1, Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringReadZero =>
			return port_input_string_read_collect_zero (input_1, Some (false), None, Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitive1::ValueRead =>
			fail_unimplemented! (0xae3d8a9f, (github_issue, 19)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::ByteWrite =>
			return port_output_byte_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesWrite =>
			return port_output_bytes_write (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesWriteLine =>
			return port_output_bytes_write_line (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive1::BytesWriteZero =>
			return port_output_bytes_write_zero (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::CharacterWrite =>
			return port_output_character_write (stdout_ref! (evaluator), input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringWrite =>
			return port_output_string_write (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringWriteLine =>
			return port_output_string_write_line (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive1::StringWriteZero =>
			return port_output_string_write_zero (stdout_ref! (evaluator), input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueWrite => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueWriteShared => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueWriteSimple =>
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueDisplay =>
			return port_output_value_display (stdout_ref! (evaluator), input_1, None, None, None, Some (true)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueWriteAndNewLine => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (stdout_ref! (evaluator), input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive1::ValueDisplayAndNewLine =>
			return port_output_value_display (stdout_ref! (evaluator), input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), Some (true)) .into_0 (),
		
		PortPrimitive1::NewLine =>
			return port_output_newline (input_1, None, Some (true)) .into_0 (),
		
		PortPrimitive1::FlushOutput =>
			return port_output_flush (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		PortPrimitive1::DescriptorGet =>
			port_descriptor_for (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		PortPrimitive1::DescriptorClone =>
			port_descriptor_clone (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		PortPrimitive1::DescriptorValue =>
			port_descriptor_ref (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		PortPrimitive1::DescriptorPath =>
			port_descriptor_path (input_1, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive1::TemporaryBinaryCreate =>
			return port_temporary_create (Some (input_1), None, None, None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive1::TemporaryTextualCreate =>
			return port_temporary_create (Some (input_1), None, None, None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive1::TemporaryRelease =>
			return port_temporary_release (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
		PortPrimitive1::TemporaryPath =>
			return port_temporary_path (input_1),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
		#[ cfg ( feature = "vonuvoli_evaluator" ) ]
		PortPrimitive2::WithOpenBinaryInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryInput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		#[ cfg ( feature = "vonuvoli_evaluator" ) ]
		PortPrimitive2::WithOpenBinaryOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryOutput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		#[ cfg ( feature = "vonuvoli_evaluator" ) ]
		PortPrimitive2::WithOpenTextualInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualInput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdin (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		#[ cfg ( feature = "vonuvoli_evaluator" ) ]
		PortPrimitive2::WithOpenTextualOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualOutput, input_1, evaluator));
			let mut evaluator = try! (evaluator.fork_parameters ());
			try! (try! (evaluator.parameters ()) .configure_stdout (try_as_port_ref! (&port)));
			return port_call_and_close_0 (&port, input_2, &mut evaluator);
		},
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive2::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, None, None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive2::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadCollect =>
			return port_input_bytes_read_collect (input_2, Some (input_1), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadChunk =>
			return port_input_bytes_read_collect (input_1, Some (input_2), Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadLine =>
			return port_input_bytes_read_collect_line (input_1, Some (false), Some (input_2), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadZero =>
			return port_input_bytes_read_collect_zero (input_1, Some (false), Some (input_2), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive2::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, None, Some (false)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadCollect =>
			return port_input_string_read_collect (input_2, Some (input_1), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadChunk =>
			return port_input_string_read_collect (input_1, Some (input_2), Some (false), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadLine =>
			return port_input_string_read_collect_line (input_1, Some (false), Some (input_2), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadZero =>
			return port_input_string_read_collect_zero (input_1, Some (false), Some (input_2), Some (true), None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadChunkFold =>
			return port_input_bytes_read_collect_fold (stdin_ref! (evaluator), None, Some (false), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadLineFold =>
			return port_input_bytes_read_collect_line_fold (stdin_ref! (evaluator), Some (false), None, Some (true), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesReadZeroFold =>
			return port_input_bytes_read_collect_zero_fold (stdin_ref! (evaluator), Some (false), None, Some (true), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadChunkFold =>
			return port_input_string_read_collect_fold (stdin_ref! (evaluator), None, Some (false), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadLineFold =>
			return port_input_string_read_collect_line_fold (stdin_ref! (evaluator), Some (false), None, Some (true), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringReadZeroFold =>
			return port_input_string_read_collect_zero_fold (stdin_ref! (evaluator), Some (false), None, Some (true), input_1, input_2, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitive2::ValueReadFold =>
			fail_unimplemented! (0xf0afe093, (github_issue, 19)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::ByteWrite =>
			return port_output_byte_write (input_2, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesWrite =>
			return port_output_bytes_write (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesWriteLine =>
			return port_output_bytes_write_line (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive2::BytesWriteZero =>
			return port_output_bytes_write_zero (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::CharacterWrite =>
			return port_output_character_write (input_2, input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringWrite =>
			return port_output_string_write (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringWriteLine =>
			return port_output_string_write_line (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive2::StringWriteZero =>
			return port_output_string_write_zero (input_2, input_1, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueWrite => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueWriteShared => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueWriteSimple =>
			return port_output_value_write (input_2, input_1, None, None, None, None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueDisplay =>
			return port_output_value_display (input_2, input_1, None, None, None, Some (true)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueWriteAndNewLine => {
			TODO! ("add support for cyclic objects");
			return port_output_value_write (input_2, input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), None) .into_0 ();
		},
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitive2::ValueDisplayAndNewLine =>
			return port_output_value_display (input_2, input_1, None, None, Some (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR), Some (true)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		PortPrimitive2::DescriptorFlagGet =>
			return port_descriptor_flag_get (input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive2::TemporaryBinaryCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), None, None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive2::TemporaryTextualCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), None, None, None, None) .into_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive3::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), None, Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive3::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, Some (input_3), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive3::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, Some (input_3), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadCollectFold =>
			return port_input_bytes_read_collect_fold (stdin_ref! (evaluator), Some (input_1), Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadChunkFold =>
			return port_input_bytes_read_collect_fold (input_1, None, Some (false), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadLineFold =>
			return port_input_bytes_read_collect_line_fold (input_1, Some (false), None, Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesReadZeroFold =>
			return port_input_bytes_read_collect_zero_fold (input_1, Some (false), None, Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringReadCollectFold =>
			return port_input_string_read_collect_fold (stdin_ref! (evaluator), Some (input_1), Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringReadChunkFold =>
			return port_input_string_read_collect_fold (input_1, None, Some (false), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringReadLineFold =>
			return port_input_string_read_collect_line_fold (input_1, Some (false), None, Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringReadZeroFold =>
			return port_input_string_read_collect_zero_fold (input_1, Some (false), None, Some (true), input_2, input_3, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitive3::ValueReadFold =>
			fail_unimplemented! (0xf9064a28, (github_issue, 19)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesWrite =>
			return port_output_bytes_write (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesWriteLine =>
			return port_output_bytes_write_line (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive3::BytesWriteZero =>
			return port_output_bytes_write_zero (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringWrite =>
			return port_output_string_write (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringWriteLine =>
			return port_output_string_write_line (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive3::StringWriteZero =>
			return port_output_string_write_zero (input_2, input_1, Some (input_3), None) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_descriptors" ) ]
		PortPrimitive3::DescriptorFlagSet =>
			return port_descriptor_flag_set (input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive3::TemporaryBinaryCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), None, None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive3::TemporaryTextualCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), None, None, None) .into_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive4::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), Some (input_4), Some (true)),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive4::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, Some (input_3), Some (try! (boolean_coerce (input_4)))),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive4::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, Some (input_3), Some (try! (boolean_coerce (input_4)))),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadCollectFold =>
			return port_input_bytes_read_collect_fold (input_2, Some (input_1), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadChunkFold =>
			return port_input_bytes_read_collect_fold (input_1, Some (input_2), Some (false), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadLineFold =>
			return port_input_bytes_read_collect_line_fold (input_1, Some (false), Some (input_2), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesReadZeroFold =>
			return port_input_bytes_read_collect_zero_fold (input_1, Some (false), Some (input_2), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringReadCollectFold =>
			return port_input_string_read_collect_fold (input_2, Some (input_1), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringReadChunkFold =>
			return port_input_string_read_collect_fold (input_1, Some (input_2), Some (false), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringReadLineFold =>
			return port_input_string_read_collect_line_fold (input_1, Some (false), Some (input_2), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringReadZeroFold =>
			return port_input_string_read_collect_zero_fold (input_1, Some (false), Some (input_2), Some (true), input_3, input_4, evaluator, None),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesWrite =>
			return port_output_bytes_write (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesWriteLine =>
			return port_output_bytes_write_line (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitive4::BytesWriteZero =>
			return port_output_bytes_write_zero (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringWrite =>
			return port_output_string_write (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringWriteLine =>
			return port_output_string_write_line (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitive4::StringWriteZero =>
			return port_output_string_write_zero (input_2, input_1, Some (input_3), Some (input_4)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive4::TemporaryBinaryCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), Some (try! (boolean_coerce (input_4))), None, None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive4::TemporaryTextualCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), Some (try! (boolean_coerce (input_4))), None, None) .into_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitive5::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, Some (input_3), Some (input_4), Some (try! (boolean_coerce (input_5)))),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive5::TemporaryBinaryCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), Some (try! (boolean_coerce (input_4))), Some (try! (boolean_coerce (input_5))), None) .into_0 () .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitive5::TemporaryTextualCreate =>
			return port_temporary_create (Some (input_1), Some (input_2), Some (input_3), Some (try! (boolean_coerce (input_4))), Some (try! (boolean_coerce (input_5))), None) .into_0 () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[impl StdAsRef<Value>], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			Some (PortPrimitive0::BytesReadZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			Some (PortPrimitive0::StringReadZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			Some (PortPrimitive0::ValueRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			Some (PortPrimitive0::NewLine),
		PortPrimitiveV::FlushOutput =>
			Some (PortPrimitive0::FlushOutput),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive0::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive0::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive1::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			Some (PortPrimitive1::BytesReadZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			Some (PortPrimitive1::StringReadZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			Some (PortPrimitive1::ValueRead),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			Some (PortPrimitive1::ByteWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive1::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			Some (PortPrimitive1::BytesWriteLine),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			Some (PortPrimitive1::BytesWriteZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			Some (PortPrimitive1::CharacterWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive1::StringWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			Some (PortPrimitive1::StringWriteLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			Some (PortPrimitive1::StringWriteZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			Some (PortPrimitive1::ValueWrite),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			Some (PortPrimitive1::ValueWriteShared),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			Some (PortPrimitive1::ValueWriteSimple),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			Some (PortPrimitive1::ValueDisplay),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			Some (PortPrimitive1::ValueWriteAndNewLine),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			Some (PortPrimitive1::ValueDisplayAndNewLine),
		PortPrimitiveV::NewLine =>
			Some (PortPrimitive1::NewLine),
		PortPrimitiveV::FlushOutput =>
			Some (PortPrimitive1::FlushOutput),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive1::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive1::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive2::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			Some (PortPrimitive2::BytesReadZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			Some (PortPrimitive2::StringReadZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			Some (PortPrimitive2::BytesReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			Some (PortPrimitive2::BytesReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			Some (PortPrimitive2::BytesReadZeroFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			Some (PortPrimitive2::StringReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			Some (PortPrimitive2::StringReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			Some (PortPrimitive2::StringReadZeroFold),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			Some (PortPrimitive2::ValueReadFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			Some (PortPrimitive2::ByteWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive2::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			Some (PortPrimitive2::BytesWriteLine),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			Some (PortPrimitive2::BytesWriteZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			Some (PortPrimitive2::CharacterWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive2::StringWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			Some (PortPrimitive2::StringWriteLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			Some (PortPrimitive2::StringWriteZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			Some (PortPrimitive2::ValueWrite),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			Some (PortPrimitive2::ValueWriteShared),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			Some (PortPrimitive2::ValueWriteSimple),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			Some (PortPrimitive2::ValueDisplay),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			Some (PortPrimitive2::ValueWriteAndNewLine),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			Some (PortPrimitive2::ValueDisplayAndNewLine),
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive2::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive2::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive3::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			Some (PortPrimitive3::BytesReadCollectFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			Some (PortPrimitive3::BytesReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			Some (PortPrimitive3::BytesReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			Some (PortPrimitive3::BytesReadZeroFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			Some (PortPrimitive3::StringReadCollectFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			Some (PortPrimitive3::StringReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			Some (PortPrimitive3::StringReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			Some (PortPrimitive3::StringReadZeroFold),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			Some (PortPrimitive3::ValueReadFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive3::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			Some (PortPrimitive3::BytesWriteLine),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			Some (PortPrimitive3::BytesWriteZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive3::StringWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			Some (PortPrimitive3::StringWriteLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			Some (PortPrimitive3::StringWriteZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive3::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive3::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive4::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			Some (PortPrimitive4::BytesReadCollectFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			Some (PortPrimitive4::BytesReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			Some (PortPrimitive4::BytesReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			Some (PortPrimitive4::BytesReadZeroFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			Some (PortPrimitive4::StringReadCollectFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			Some (PortPrimitive4::StringReadChunkFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			Some (PortPrimitive4::StringReadLineFold),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			Some (PortPrimitive4::StringReadZeroFold),
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			Some (PortPrimitive4::BytesWrite),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			Some (PortPrimitive4::BytesWriteLine),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			Some (PortPrimitive4::BytesWriteZero),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			Some (PortPrimitive4::StringWrite),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			Some (PortPrimitive4::StringWriteLine),
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			Some (PortPrimitive4::StringWriteZero),
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive4::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive4::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			Some (PortPrimitive5::BytesReadCopy),
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			Some (PortPrimitive5::TemporaryBinaryCreate),
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			Some (PortPrimitive5::TemporaryTextualCreate),
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
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
		PortPrimitiveV::BytesReadCopy =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
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
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueRead =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadCollectFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadChunkFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadLineFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringReadZeroFold =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_input_value" ) ]
		PortPrimitiveV::ValueReadFold =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::ByteWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		PortPrimitiveV::BytesWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::CharacterWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteLine =>
			None,
		#[ cfg ( feature = "vonuvoli_values_string" ) ]
		PortPrimitiveV::StringWriteZero =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWrite =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteShared =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteSimple =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplay =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueWriteAndNewLine =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_output_value" ) ]
		PortPrimitiveV::ValueDisplayAndNewLine =>
			None,
		PortPrimitiveV::NewLine =>
			None,
		PortPrimitiveV::FlushOutput =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryBinaryCreate =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_ports_temporary" ) ]
		PortPrimitiveV::TemporaryTextualCreate =>
			None,
	}
}




#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_0_attributes (_primitive : PortPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_1_attributes (_primitive : PortPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_2_attributes (_primitive : PortPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_3_attributes (_primitive : PortPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_4_attributes (_primitive : PortPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_5_attributes (_primitive : PortPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg ( feature = "vonuvoli_optimizer" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_attributes (_primitive : PortPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

