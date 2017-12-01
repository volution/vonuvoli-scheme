

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::fmt;
use std::ptr;




pub mod exports {
	pub use super::Lambda;
	pub use super::LambdaTemplate;
}




#[ derive (Clone, Hash) ]
pub struct Lambda ( StdRc<LambdaInternals> );


#[ derive (Debug, Hash) ]
pub struct LambdaInternals {
	pub identifier : Option<Symbol>,
	pub arguments_positional : StdVec<Symbol>,
	pub argument_rest : Option<Symbol>,
	pub expression : Expression,
	// FIXME:  Recursive functions might leak;  investigate!
	pub registers_closure : Registers,
	pub registers_local : StdVec<RegistersBindingTemplate>,
}


#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct LambdaTemplate {
	pub identifier : Option<Symbol>,
	pub arguments_positional : StdVec<Symbol>,
	pub argument_rest : Option<Symbol>,
}


impl Lambda {
	
	pub fn new (template : LambdaTemplate, expression : Expression, registers_closure : Registers, registers_local : StdVec<RegistersBindingTemplate>) -> (Lambda) {
		let internals = LambdaInternals {
				identifier : template.identifier,
				arguments_positional : template.arguments_positional,
				argument_rest : template.argument_rest,
				expression : expression,
				registers_closure : registers_closure,
				registers_local : registers_local,
			};
		return Lambda (StdRc::new (internals));
	}
	
	pub fn internals (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn is_self (&self, other : &Lambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
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

