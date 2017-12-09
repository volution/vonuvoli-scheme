

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::PortPrimitive0;
	pub use super::PortPrimitive1;
	pub use super::PortPrimitive2;
	pub use super::PortPrimitive3;
	pub use super::PortPrimitive4;
	pub use super::PortPrimitive5;
	pub use super::PortPrimitiveN;
	
	pub use super::port_primitive_0_evaluate;
	pub use super::port_primitive_1_evaluate;
	pub use super::port_primitive_2_evaluate;
	pub use super::port_primitive_3_evaluate;
	pub use super::port_primitive_4_evaluate;
	pub use super::port_primitive_5_evaluate;
	pub use super::port_primitive_n_evaluate;
	
	pub use super::port_primitive_n_alternative_0;
	pub use super::port_primitive_n_alternative_1;
	pub use super::port_primitive_n_alternative_2;
	pub use super::port_primitive_n_alternative_3;
	pub use super::port_primitive_n_alternative_4;
	pub use super::port_primitive_n_alternative_5;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive0 {
	
	RsNewLine,
	
	OutputToBytes,
	OutputToString,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive1 {
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	FlushOutput,
	
	ByteReady,
	BytePeek,
	ByteRead,
	
	CharReady,
	CharPeek,
	CharRead,
	
	CharsReadLine,
	
	ValueRead,
	
	RsDisplay,
	
	InputFromBytes,
	InputFromString,
	
	OutputToBytesCollect,
	OutputToStringCollect,
	
	OpenBinaryInput,
	OpenBinaryOutput,
	
	OpenTextualInput,
	OpenTextualOutput,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive2 {
	
	BytesReadCollect,
	
	CharsReadCollect,
	
	ByteWrite,
	BytesWrite,
	
	CharWrite,
	CharsWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveN {}




pub fn port_primitive_0_evaluate (primitive : PortPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive0::RsNewLine => {
			// FIXME:  Replace this stub implementation!
			use std::io;
			use std::io::Write;
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
			fail_unimplemented! (0x502eda3e),
		
		PortPrimitive0::OutputToString =>
			fail_unimplemented! (0x646d0bda),
		
	}
}




pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive1::RsDisplay => {
			// FIXME:  Replace this stub implementation!
			use std::io;
			use std::io::Write;
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
			try! (port_close_input (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::CloseOutput => {
			try! (port_close_output (input_1));
			succeed! (VOID.into ());
		},
		
		PortPrimitive1::FlushOutput =>
			fail_unimplemented! (0xa9890df6),
		
		PortPrimitive1::ByteReady =>
			fail_unimplemented! (0x2cbf294d),
		
		PortPrimitive1::BytePeek =>
			fail_unimplemented! (0x848317e1),
		
		PortPrimitive1::ByteRead =>
			fail_unimplemented! (0x85777bd4),
		
		PortPrimitive1::CharReady =>
			fail_unimplemented! (0x31980370),
		
		PortPrimitive1::CharPeek =>
			fail_unimplemented! (0xace61b21),
		
		PortPrimitive1::CharRead =>
			fail_unimplemented! (0x345ad587),
		
		PortPrimitive1::CharsReadLine =>
			fail_unimplemented! (0x83c98559),
		
		PortPrimitive1::ValueRead =>
			fail_unimplemented! (0xae3d8a9f),
		
		PortPrimitive1::InputFromBytes =>
			fail_unimplemented! (0xd650991d),
		
		PortPrimitive1::InputFromString =>
			fail_unimplemented! (0x992f783d),
		
		PortPrimitive1::OutputToBytesCollect =>
			fail_unimplemented! (0x481b595e),
		
		PortPrimitive1::OutputToStringCollect =>
			fail_unimplemented! (0xdac73f5c),
		
		PortPrimitive1::OpenBinaryInput =>
			fail_unimplemented! (0xa3e9af81),
		
		PortPrimitive1::OpenBinaryOutput =>
			fail_unimplemented! (0xa9fd190e),
		
		PortPrimitive1::OpenTextualInput =>
			fail_unimplemented! (0x2166fd23),
		
		PortPrimitive1::OpenTextualOutput =>
			fail_unimplemented! (0x677c2ccf),
		
	}
}




pub fn port_primitive_2_evaluate (primitive : PortPrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive2::BytesReadCollect =>
			fail_unimplemented! (0x544399ba),
		
		PortPrimitive2::CharsReadCollect =>
			fail_unimplemented! (0x3df7a0bf),
		
		PortPrimitive2::ByteWrite =>
			fail_unimplemented! (0x55a95369),
		
		PortPrimitive2::BytesWrite =>
			fail_unimplemented! (0x718eacd2),
		
		PortPrimitive2::CharWrite =>
			fail_unimplemented! (0x05ec1858),
		
		PortPrimitive2::CharsWrite =>
			fail_unimplemented! (0x3745d7e6),
		
		PortPrimitive2::ValueWrite =>
			fail_unimplemented! (0x696cb627),
		
		PortPrimitive2::ValueWriteShared =>
			fail_unimplemented! (0xd82b6e11),
		
		PortPrimitive2::ValueWriteSimple =>
			fail_unimplemented! (0x71e1d1d0),
		
	}
}




pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




pub fn port_primitive_n_alternative_0 (primitive : PortPrimitiveN) -> (Option<PortPrimitive0>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_1 (primitive : PortPrimitiveN) -> (Option<PortPrimitive1>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_2 (primitive : PortPrimitiveN) -> (Option<PortPrimitive2>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_3 (primitive : PortPrimitiveN) -> (Option<PortPrimitive3>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_4 (primitive : PortPrimitiveN) -> (Option<PortPrimitive4>) {
	match primitive {}
}


pub fn port_primitive_n_alternative_5 (primitive : PortPrimitiveN) -> (Option<PortPrimitive5>) {
	match primitive {}
}

