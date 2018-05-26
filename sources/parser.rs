

use super::errors::exports::*;
use super::values::exports::*;
use super::values_tests::exports::*;
use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;

use super::parser_peg as peg;




pub mod exports {
	
	pub use super::parse_value;
	pub use super::parse_script;
	
	pub use super::{parse_tests, parse_test};
	
}




#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




#[ inline (never) ]
pub fn parse_value (input : &str) -> (Outcome<Value>) {
	return parse_0 (input, peg::value);
}




#[ inline (never) ]
pub fn parse_script (input : &str) -> (Outcome<ValueVec>) {
	return parse_0 (input, peg::script);
}




#[ inline (never) ]
pub fn parse_tests (input : &str) -> (Outcome<StdVec<TestCase>>) {
	return parse_0 (input, peg::tests);
}

#[ inline (never) ]
pub fn parse_test (input : &str) -> (Outcome<TestCase>) {
	return parse_0 (input, peg::test);
}




#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_0 <Parser, ParserOutput, ParserError> (input : &str, parser : Parser) -> (Outcome<ParserOutput>)
		where
			Parser : Fn (&str) -> (Result<ParserOutput, ParserError>),
			ParserOutput : fmt::Debug,
			ParserError : TranscriptError,
{
	let outcome = if PARSER_TRACE_INPUT || PARSER_TRACE_OUTPUT || PARSER_TRACE_ERROR {
		
		if PARSER_TRACE_INPUT {
			trace_debugging! (transcript, 0x77979f89 => "parsing:\u{1e}{}" => (&input));
		}
		
		let outcome = parser (input);
		
		match outcome {
			Ok (ref output) if PARSER_TRACE_OUTPUT =>
				trace_debugging! (transcript, 0x1770409f => "parsing succeeded:\u{1e}{}\u{1e}{:#?}" => (input, output)),
			Ok (_) =>
				(),
			Err (ref error) if (PARSER_TRACE_OUTPUT || PARSER_TRACE_ERROR) =>
				trace_error! (transcript, 0xee841c6f => "parsing failed:\u{1e}{}" => (input), error = error),
			Err (_) =>
				(),
		}
		
		outcome
		
	} else {
		
		parser (input)
		
	};
	
	return parse_0_outcome (outcome);
}


#[ cfg ( not ( all ( feature = "vonuvoli_parser_trace_enabled", feature = "vonuvoli_transcript" ) ) ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_0 <Parser, ParserOutput, ParserError> (input : &str, parser : Parser) -> (Outcome<ParserOutput>)
		where Parser : Fn (&str) -> (Result<ParserOutput, ParserError>)
{
	let outcome = parser (input);
	return parse_0_outcome (outcome);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_0_outcome <ParserOutput, ParserError> (outcome : Result<ParserOutput, ParserError>) -> (Outcome<ParserOutput>) {
 	match outcome {
		Ok (output) =>
			succeed! (output),
		Err (_error) => {
			TODO! ("wrap and return this error");
			fail! (0x2af5f184);
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

