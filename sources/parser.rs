

use super::errors::exports::*;
use super::values::exports::*;
use super::values_tests::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;

use super::parser_peg as peg;




pub mod exports {
	
	pub use super::parse_value;
	pub use super::parse_script;
	
	pub use super::{parse_tests, parse_test};
	
}




#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




#[ inline (never) ]
pub fn parse_value (input : &str) -> (Outcome<Value>) {
	match peg::value (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0x3ab38ddb => "parsing failed!" => (), error = &error);
			#[ cfg ( not ( feature = "vonuvoli_transcript" ) ) ]
			mem::drop (error);
			fail! (0x2af5f184);
		},
	}
}




#[ inline (never) ]
pub fn parse_script (input : &str) -> (Outcome<ValueVec>) {
	match peg::script (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0x1712eae3 => "parsing failed!" => (), error = &error);
			#[ cfg ( not ( feature = "vonuvoli_transcript" ) ) ]
			mem::drop (error);
			fail! (0xb13e446a);
		},
	}
}




#[ inline (never) ]
pub fn parse_tests (input : &str) -> (Outcome<StdVec<TestCase>>) {
	match peg::tests (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0x4b9cc676 => "parsing failed!" => (), error = &error);
			#[ cfg ( not ( feature = "vonuvoli_transcript" ) ) ]
			mem::drop (error);
			fail! (0x86ee143a);
		},
	}
}

#[ inline (never) ]
pub fn parse_test (input : &str) -> (Outcome<TestCase>) {
	match peg::test (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			#[ cfg ( feature = "vonuvoli_transcript" ) ]
			trace_error! (transcript, 0xd1255912 => "parsing failed!" => (), error = &error);
			#[ cfg ( not ( feature = "vonuvoli_transcript" ) ) ]
			mem::drop (error);
			fail! (0x46eb5847);
		},
	}
}




#[ cfg ( feature = "vonuvoli_transcript" ) ]
impl TranscriptError for peg::ParseError {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn message (&self) -> (Option<borrow::Cow<str>>) {
		Some (borrow::Cow::Owned (format! ("{}", self)))
	}
}

