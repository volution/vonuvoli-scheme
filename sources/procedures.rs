

use super::expressions::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::fmt;




pub mod exports {
	pub use super::Procedure;
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Procedure {
	Primitive ( Primitive ),
	Lambda ( Lambda ),
}


impl fmt::Display for Procedure {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<procedure>")
	}
}




#[ derive (Clone, Eq, PartialEq, Hash) ]
pub struct Lambda ( StdRc<LambdaInternals> );


#[ derive (Debug, Eq, PartialEq, Hash) ]
pub struct LambdaInternals {
	pub identifier : Symbol,
	pub argument_count : usize,
	pub argument_identifiers : StdVec<Symbol>,
	pub expression : Expression,
}


impl fmt::Display for Lambda {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<lambda>")
	}
}


impl fmt::Debug for Lambda {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		
		self.0.fmt (formatter)
	}
}

