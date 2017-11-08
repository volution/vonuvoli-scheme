

macro_rules! fail {
	( $code : expr ) => (return ::std::result::Result::Err (::errors::error_generic ($code)));
}




pub mod contexts;
pub mod counters;
pub mod errors;
pub mod evaluator;
pub mod globals;
pub mod operators;
pub mod parser;
pub mod runtime;
pub mod values;

mod parser_peg;




pub mod exports {
	pub use super::contexts::exports::*;
	pub use super::evaluator::exports::*;
	pub use super::parser::exports::*;
	pub use super::values::exports::*;
}

