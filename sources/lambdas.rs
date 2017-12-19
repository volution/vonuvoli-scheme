

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::cmp;
use std::fmt;
use std::hash;
use std::ptr;




pub mod exports {
	
	pub use super::Lambda;
	pub use super::LambdaInternals;
	pub use super::LambdaTemplate;
	
	pub use super::ProcedureLambda;
	pub use super::SyntaxLambda;
	
}




#[ derive (Debug) ]
pub struct LambdaInternals {
	pub handle_1 : Handle,
	pub handle_2 : Handle,
	pub arguments_positional : usize,
	pub argument_rest : bool,
	pub expression : StdRc<Expression>,
	pub registers_closure : Registers,
	pub registers_local : StdRc<[RegisterTemplate]>,
}


impl cmp::Eq for LambdaInternals {}

impl cmp::PartialEq for LambdaInternals {
	fn eq (&self, other : &LambdaInternals) -> (bool) {
		Handle::eq (&self.handle_2, &other.handle_2)
	}
}


impl cmp::Ord for LambdaInternals {
	fn cmp (&self, other : &LambdaInternals) -> (cmp::Ordering) {
		Handle::cmp (&self.handle_2, &other.handle_2)
	}
}

impl cmp::PartialOrd for LambdaInternals {
	fn partial_cmp (&self, other : &LambdaInternals) -> (Option<cmp::Ordering>) {
		Handle::partial_cmp (&self.handle_2, &other.handle_2)
	}
}


impl hash::Hash for LambdaInternals {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		self.handle_2.hash (hasher);
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct LambdaTemplate {
	pub identifier : Option<Symbol>,
	pub arguments_positional : StdBox<[Symbol]>,
	pub argument_rest : Option<Symbol>,
	pub handle : Handle,
}


impl LambdaTemplate {
	
	#[ inline (always) ]
	pub fn new (identifier : Option<Symbol>, arguments_positional : StdBox<[Symbol]>, argument_rest : Option<Symbol>) -> (LambdaTemplate) {
		return LambdaTemplate {
				identifier : identifier,
				arguments_positional : arguments_positional,
				argument_rest : argument_rest,
				handle : lambdas_handles_next (),
			};
	}
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Lambda ( StdRc<LambdaInternals> );

impl Lambda {
	
	#[ inline (always) ]
	pub fn new (template : StdRc<LambdaTemplate>, expression : StdRc<Expression>, registers_closure : Registers, registers_local : StdRc<[RegisterTemplate]>) -> (Lambda) {
		let internals = LambdaInternals {
				handle_1 : template.handle,
				handle_2 : lambdas_handles_next (),
				arguments_positional : template.arguments_positional.len (),
				argument_rest : template.argument_rest.is_some (),
				expression : expression,
				registers_closure : registers_closure,
				registers_local : registers_local,
			};
		return Lambda (StdRc::new (internals));
	}
	
	#[ inline (always) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
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
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &Lambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ());
	}
}


impl fmt::Debug for Lambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ProcedureLambda ( StdRc<LambdaInternals> );


impl ProcedureLambda {
	
	#[ inline (always) ]
	pub fn new (lambda : Lambda) -> (ProcedureLambda) {
		return ProcedureLambda (lambda.internals_rc_clone ());
	}
	
	#[ inline (always) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
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
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &ProcedureLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ());
	}
}

impl fmt::Debug for ProcedureLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct SyntaxLambda ( StdRc<LambdaInternals> );


impl SyntaxLambda {
	
	#[ inline (always) ]
	pub fn new (lambda : Lambda) -> (SyntaxLambda) {
		return SyntaxLambda (lambda.internals_rc_clone ());
	}
	
	#[ inline (always) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
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
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &SyntaxLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}


impl fmt::Display for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		let self_0 = self.internals_ref ();
		return write! (formatter, "#<lambda:{:08x}:{:08x}>", self_0.handle_1.value (), self_0.handle_2.value ());
	}
}

impl fmt::Debug for SyntaxLambda {
	
	#[ inline (never) ]
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		self.0.fmt (formatter)
	}
}

