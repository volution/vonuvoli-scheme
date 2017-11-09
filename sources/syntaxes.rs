

use std::fmt;




pub mod exports {
	pub use super::Syntax;
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub enum Syntax {
}


impl fmt::Display for Syntax {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<syntax>")
	}
}

