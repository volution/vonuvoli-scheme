

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::fmt;
use std::ptr;




pub mod exports {
	
	pub use super::Lambda;
	pub use super::LambdaInternals;
	pub use super::LambdaTemplate;
	
	pub use super::ProcedureLambda;
	pub use super::SyntaxLambda;
	
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
	
	#[ inline (always) ]
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
	
	#[ inline (always) ]
	pub fn internals (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ inline (always) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ inline (always) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Lambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<lambda>")
	}
}


impl fmt::Debug for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}




#[ derive (Clone, Hash) ]
pub struct ProcedureLambda ( StdRc<LambdaInternals> );


impl ProcedureLambda {
	
	#[ inline (always) ]
	pub fn new (lambda : Lambda) -> (ProcedureLambda) {
		return ProcedureLambda (lambda.internals_rc_clone ());
	}
	
	#[ inline (always) ]
	pub fn internals (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ inline (always) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ inline (always) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ProcedureLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<procedure-lambda>")
	}
}

impl fmt::Debug for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}




#[ derive (Clone, Hash) ]
pub struct SyntaxLambda ( StdRc<LambdaInternals> );


impl SyntaxLambda {
	
	#[ inline (always) ]
	pub fn new (lambda : Lambda) -> (SyntaxLambda) {
		return SyntaxLambda (lambda.internals_rc_clone ());
	}
	
	#[ inline (always) ]
	pub fn internals (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ inline (always) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ inline (always) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &SyntaxLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		formatter.write_str ("#<syntax-lambda>")
	}
}

impl fmt::Debug for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}

