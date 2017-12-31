

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

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
	
	RsNewLine,
	
	OutputToBytes,
	OutputToString,
	
	Eof,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive1 {
	
	RsDisplay,
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	FlushOutput,
	
	ByteReady,
	BytePeek,
	ByteRead,
	
	CharacterReady,
	CharacterPeek,
	CharacterRead,
	
	BytesReadCollect,
	
	StringReadCollect,
	
	StringReadLine,
	
	ValueRead,
	
	InputFromBytes,
	InputFromString,
	
	OutputToBytesFinalize,
	OutputToStringFinalize,
	
	OpenBinaryInput,
	OpenBinaryOutput,
	
	OpenTextualInput,
	OpenTextualOutput,
	
	FileExists,
	FileDelete,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive2 {
	
	BytesReadCollect,
	BytesReadExtend,
	BytesReadCopy,
	
	StringReadCollect,
	StringReadExtend,
	
	ByteWrite,
	BytesWrite,
	
	CharacterWrite,
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	
	CallAndClose,
	OpenBinaryInputThenCallAndClose,
	OpenBinaryOutputThenCallAndClose,
	OpenTextualInputThenCallAndClose,
	OpenTextualOutputThenCallAndClose,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveN {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveV {}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_0_evaluate (primitive : PortPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive0::RsNewLine => {
			// FIXME:  Replace this stub implementation!
			let mut stream = io::stdout ();
			let mut stream = stream.lock ();
			match write! (stream, "\n") {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0xe2f91118),
			}
			match stream.flush () {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0x35130507),
			}
			succeed! (VOID.into ());
		},
		
		PortPrimitive0::OutputToBytes =>
			return port_bytes_writer_new (),
		
		PortPrimitive0::OutputToString =>
			return port_string_writer_new (),
		
		PortPrimitive0::Eof =>
			succeed! (PORT_EOF.into ()),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive1::RsDisplay => {
			// FIXME:  Replace this stub implementation!
			let mut stream = io::stdout ();
			let mut stream = stream.lock ();
			match write! (stream, "{}", input_1) {
				Ok (()) =>
					(),
				Err (_) =>
					fail! (0x7aab6cc6),
			}
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::IsInputOpen =>
			succeed! (try! (is_port_input_open (input_1)) .into ()),
		
		PortPrimitive1::IsOutputOpen =>
			succeed! (try! (is_port_output_open (input_1)) .into ()),
		
		PortPrimitive1::Close => {
			try! (port_close (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::CloseInput => {
			try! (port_input_close (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::CloseOutput => {
			try! (port_output_close (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::FlushOutput => {
			try! (port_output_flush (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::ByteReady =>
			succeed! (try! (port_input_byte_ready (input_1)) .into ()),
		
		PortPrimitive1::BytePeek =>
			return port_input_byte_peek (input_1),
		
		PortPrimitive1::ByteRead =>
			return port_input_byte_read (input_1),
		
		PortPrimitive1::CharacterReady =>
			succeed! (try! (port_input_character_ready (input_1)) .into ()),
		
		PortPrimitive1::CharacterPeek =>
			return port_input_character_peek (input_1),
		
		PortPrimitive1::CharacterRead =>
			return port_input_character_read (input_1),
		
		PortPrimitive1::BytesReadCollect =>
			return port_input_bytes_read_collect (input_1, None),
		
		PortPrimitive1::StringReadCollect =>
			return port_input_string_read_collect (input_1, None),
		
		PortPrimitive1::StringReadLine =>
			fail_unimplemented! (0x83c98559),
		
		PortPrimitive1::ValueRead =>
			fail_unimplemented! (0xae3d8a9f),
		
		PortPrimitive1::InputFromBytes =>
			return port_bytes_reader_new (input_1),
		
		PortPrimitive1::InputFromString =>
			return port_string_reader_new (input_1),
		
		PortPrimitive1::OutputToBytesFinalize =>
			return port_bytes_writer_finalize (input_1),
		
		PortPrimitive1::OutputToStringFinalize =>
			return port_string_writer_finalize (input_1),
		
		PortPrimitive1::OpenBinaryInput =>
			return port_file_reader_open (input_1),
		
		PortPrimitive1::OpenBinaryOutput =>
			return port_file_writer_open (input_1),
		
		PortPrimitive1::OpenTextualInput =>
			return port_file_reader_open (input_1),
		
		PortPrimitive1::OpenTextualOutput =>
			return port_file_writer_open (input_1),
		
		PortPrimitive1::FileExists =>
			succeed! (try! (port_file_exists (input_1)) .into ()),
		
		PortPrimitive1::FileDelete =>
			succeed! (try! (port_file_delete (input_1)) .into ()),
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_2_evaluate (primitive : PortPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive2::BytesReadCollect =>
			return port_input_bytes_read_collect (input_2, Some (input_1)),
		
		PortPrimitive2::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, None),
		
		PortPrimitive2::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, None, None),
		
		PortPrimitive2::StringReadCollect =>
			return port_input_string_read_collect (input_2, Some (input_1)),
		
		PortPrimitive2::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, None),
		
		PortPrimitive2::ByteWrite => {
			try! (port_output_byte_write (input_2, input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive2::BytesWrite => {
			try! (port_output_bytes_write (input_2, input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive2::CharacterWrite => {
			try! (port_output_character_write (input_2, input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive2::StringWrite => {
			try! (port_output_string_write (input_2, input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive2::ValueWrite =>
			fail_unimplemented! (0x696cb627),
		
		PortPrimitive2::ValueWriteShared =>
			fail_unimplemented! (0xd82b6e11),
		
		PortPrimitive2::ValueWriteSimple =>
			fail_unimplemented! (0x71e1d1d0),
		
		PortPrimitive2::CallAndClose =>
			return port_call_and_close (input_1, input_2, evaluator),
		
		PortPrimitive2::OpenBinaryInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryInput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenBinaryOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryOutput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualInput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualOutput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_0 (primitive : PortPrimitiveV) -> (Option<PortPrimitive0>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_1 (primitive : PortPrimitiveV) -> (Option<PortPrimitive1>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_2 (primitive : PortPrimitiveV) -> (Option<PortPrimitive2>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_3 (primitive : PortPrimitiveV) -> (Option<PortPrimitive3>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_4 (primitive : PortPrimitiveV) -> (Option<PortPrimitive4>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_5 (primitive : PortPrimitiveV) -> (Option<PortPrimitive5>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_v_alternative_n (primitive : PortPrimitiveV) -> (Option<PortPrimitiveN>) {
	match primitive {}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_0_attributes (_primitive : PortPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_1_attributes (_primitive : PortPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_2_attributes (_primitive : PortPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_3_attributes (_primitive : PortPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_4_attributes (_primitive : PortPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_5_attributes (_primitive : PortPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn port_primitive_n_attributes (_primitive : PortPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

