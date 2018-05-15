

use super::errors::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

#[ cfg ( feature = "vonuvoli_tests" ) ]
use super::tests::exports::*;

use super::prelude::*;

use super::parser_peg as peg;

def_transcript! (transcript);




pub mod exports {
	
	pub use super::parse_value;
	pub use super::parse_script;
	
	#[ cfg ( feature = "vonuvoli_tests" ) ]
	pub use super::{parse_tests, parse_test};
	
}




#[ inline (never) ]
pub fn parse_value (input : &str) -> (Outcome<Value>) {
	match peg::value (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			trace_error! (transcript, 0x3ab38ddb => "parsing failed!" => (), error = &error);
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
			trace_error! (transcript, 0x1712eae3 => "parsing failed!" => (), error = &error);
			fail! (0xb13e446a);
		},
	}
}




#[ cfg ( feature = "vonuvoli_tests" ) ]
#[ inline (never) ]
pub fn parse_tests (input : &str) -> (Outcome<StdVec<TestCase>>) {
	match peg::tests (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			trace_error! (transcript, 0x4b9cc676 => "parsing failed!" => (), error = &error);
			fail! (0x86ee143a);
		},
	}
}

#[ cfg ( feature = "vonuvoli_tests" ) ]
#[ inline (never) ]
pub fn parse_test (input : &str) -> (Outcome<TestCase>) {
	match peg::test (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			TODO! ("wrap and return this error instead of printing");
			trace_error! (transcript, 0xd1255912 => "parsing failed!" => (), error = &error);
			fail! (0x46eb5847);
		},
	}
}




impl TranscriptError for peg::ParseError {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn message (&self) -> (Option<borrow::Cow<str>>) {
		Some (borrow::Cow::Owned (format! ("{}", self)))
	}
}

