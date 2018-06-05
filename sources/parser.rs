

use super::errors::exports::*;
use super::values::exports::*;
use super::values_tests::exports::*;
use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
use super::transcript::exports::*;

use super::prelude::*;

use super::parser_peg as peg;




pub mod exports {
	
	pub use super::parse_value;
	pub use super::parse_script;
	
	pub use super::ParserConfiguration;
	
	pub use super::{parse_tests, parse_test};
	
}




#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




#[ inline (never) ]
pub fn parse_value (input : &str, configuration : Option<&ParserConfiguration>) -> (Outcome<Value>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return parse_0 (input, peg::value, &configuration);
}




#[ inline (never) ]
pub fn parse_script (input : &str, configuration : Option<&ParserConfiguration>) -> (Outcome<ValueVec>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return parse_0 (input, peg::script, &configuration);
}




#[ inline (never) ]
pub fn parse_tests (input : &str, configuration : Option<&ParserConfiguration>) -> (Outcome<StdVec<TestCase>>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return parse_0 (input, peg::tests, &configuration);
}

#[ inline (never) ]
pub fn parse_test (input : &str, configuration : Option<&ParserConfiguration>) -> (Outcome<TestCase>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return parse_0 (input, peg::test, &configuration);
}




#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn parse_0 <Parser, ParserOutput, ParserError> (input : &str, parser : Parser, configuration : &ParserConfiguration) -> (Outcome<ParserOutput>)
		where
			Parser : Fn (&str) -> (Result<ParserOutput, ParserError>),
			ParserOutput : fmt::Debug,
			ParserError : TranscriptError,
{
	let outcome = if configuration.is_trace_enabled () {
		
		if configuration.should_trace_input () {
			trace_debugging! (transcript, 0x77979f89 => "parsing:\u{1e}{}" => (&input));
		}
		
		let outcome = parser (input);
		
		match outcome {
			Ok (ref output) if configuration.should_trace_output () =>
				trace_debugging! (transcript, 0x1770409f => "parsing succeeded:\u{1e}{}\u{1e}{:#?}" => (input, output)),
			Ok (_) =>
				(),
			Err (ref error) if configuration.should_trace_output_or_error () =>
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
fn parse_0 <Parser, ParserOutput, ParserError> (input : &str, parser : Parser, _configuration : &ParserConfiguration) -> (Outcome<ParserOutput>)
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




#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
impl TranscriptError for peg::ParseError {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn message (&self) -> (Option<borrow::Cow<str>>) {
		Some (borrow::Cow::Owned (format! ("{}", self)))
	}
}




#[ derive ( Clone, Default ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub struct ParserConfiguration {
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_input : Option<bool>,
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_output : Option<bool>,
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_error : Option<bool>,
}


impl ParserConfiguration {
	
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_input (&self) -> (bool) {
		self.trace_input.unwrap_or (PARSER_TRACE_INPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_output (&self) -> (bool) {
		self.trace_output.unwrap_or (PARSER_TRACE_OUTPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_error (&self) -> (bool) {
		self.trace_error.unwrap_or (PARSER_TRACE_ERROR)
	}
	
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_output_or_error (&self) -> (bool) {
		self.should_trace_output () || self.should_trace_error ()
	}
	
	#[ cfg ( feature = "vonuvoli_parser_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_trace_enabled (&self) -> (bool) {
		self.should_trace_input () || self.should_trace_output () || self.should_trace_error ()
	}
}

