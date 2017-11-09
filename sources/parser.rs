

use super::parser_peg as peg;
use super::values::exports::*;




pub mod exports {
	pub use super::parse_value;
}




#[ inline (always) ]
pub fn parse_value (input : &str) -> Value {
	return peg::value_full (input) .expect ("391e2457");
}

