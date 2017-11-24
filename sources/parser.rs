

use super::errors::exports::*;
use super::runtime::exports::*;
use super::tests::exports::*;
use super::values::exports::*;

use super::parser_peg as peg;




pub mod exports {
	pub use super::parse_value;
	pub use super::{parse_tests, parse_test};
}




pub fn parse_value (input : &str) -> (Outcome<Value>) {
	match peg::value_full (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			eprintln! ("[ee]  {:?}", error);
			fail! (0x2af5f184);
		},
	}
}




pub fn parse_tests (input : &str) -> (Outcome<StdVec<TestCase>>) {
	match peg::tests (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			// FIXME:  Wrap and return this error instead of printing!
			eprintln! ("[ee]  {:?}", error);
			fail! (0x86ee143a);
		},
	}
}

pub fn parse_test (input : &str) -> (Outcome<TestCase>) {
	match peg::test (input) {
		Ok (output) =>
			succeed! (output),
		Err (error) => {
			// FIXME:  Wrap and return this error instead of printing!
			eprintln! ("[ee]  {:?}", error);
			fail! (0x46eb5847);
		},
	}
}

