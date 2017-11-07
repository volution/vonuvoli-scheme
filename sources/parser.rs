
use super::parser_peg;
use super::values;

pub fn parse_value (input : &str) -> values::Value
{
	return parser_peg::value_full (input) .expect ("391e2457");
}

