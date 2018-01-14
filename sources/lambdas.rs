

use super::contexts::exports::*;
use super::expressions::exports::*;
use super::globals::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




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




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct LambdaTemplate {
	pub identifier : Option<Symbol>,
	pub arguments_positional : StdBox<[Symbol]>,
	pub argument_rest : Option<Symbol>,
	pub handle : Handle,
}


impl LambdaTemplate {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Lambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ProcedureLambda ( StdRc<LambdaInternals> );


impl ProcedureLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lambda : Lambda) -> (ProcedureLambda) {
		return ProcedureLambda (lambda.internals_rc_clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ProcedureLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct SyntaxLambda ( StdRc<LambdaInternals> );


impl SyntaxLambda {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (lambda : Lambda) -> (SyntaxLambda) {
		return SyntaxLambda (lambda.internals_rc_clone ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return self.0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &SyntaxLambda) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}

