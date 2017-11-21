

use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;
use super::tests::exports::*;

use super::parser_peg as peg;




pub mod exports {
	pub use super::parse_value;
	pub use super::{parse_tests, parse_test};
}




pub fn parse_value (input : &str) -> (Outcome<Value>) {
	if let Ok (output) = peg::value_full (input) {
		succeed! (output);
	} else {
		fail! (0x2af5f184);
	}
}




pub fn parse_tests (input : &str) -> (Outcome<StdVec<TestCase>>) {
	if let Ok (output) = peg::tests (input) {
		succeed! (output);
	} else {
		fail! (0x86ee143a);
	}
}

pub fn parse_test (input : &str) -> (Outcome<TestCase>) {
	if let Ok (output) = peg::test (input) {
		succeed! (output);
	} else {
		fail! (0x46eb5847);
	}
}

