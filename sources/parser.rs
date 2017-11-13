

use super::errors::exports::*;
use super::values::exports::*;

use super::parser_peg as peg;




pub mod exports {
	pub use super::parse;
}




#[ inline (always) ]
pub fn parse (input : &str) -> Outcome<Value> {
	if let Ok (output) = peg::value_full (input) {
		Ok (output)
	} else {
		failed! (0x2af5f184)
	}
}

