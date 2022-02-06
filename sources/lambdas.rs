

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




#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct LambdaInternals {
	pub handle_1 : Handle,
	pub handle_2 : Handle,
	pub arguments_positional : usize,
	pub argument_rest : bool,
	pub expression : StdRc<Expression>,
	pub registers_closure : Registers,
	pub registers_local : StdRc<[RegisterTemplate]>,
}




#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ~~
pub struct LambdaTemplate {
	pub identifier : Option<Symbol>,
	pub arguments_positional : StdBox<[Symbol]>,
	pub argument_rest : Option<Symbol>,
	pub handle : Handle,
}


impl LambdaTemplate {
	
	pub fn new (identifier : Option<Symbol>, arguments_positional : StdBox<[Symbol]>, argument_rest : Option<Symbol>) -> (LambdaTemplate) {
		return LambdaTemplate {
				identifier : identifier,
				arguments_positional : arguments_positional,
				argument_rest : argument_rest,
				handle : lambda_handles_next (),
			};
	}
}




#[ derive ( Clone ) ] // OK
pub struct Lambda ( StdRc<LambdaInternals> );

impl Lambda {
	
	pub fn new (template : &LambdaTemplate, expression : StdRc<Expression>, registers_closure : Registers, registers_local : StdRc<[RegisterTemplate]>) -> (Lambda) {
		let internals = LambdaInternals {
				handle_1 : template.handle,
				handle_2 : lambda_handles_next (),
				arguments_positional : template.arguments_positional.len (),
				argument_rest : template.argument_rest.is_some (),
				expression : expression,
				registers_closure : registers_closure,
				registers_local : registers_local,
			};
		return Lambda (StdRc::new (internals));
	}
	
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return StdRc::clone (&self.0);
	}
	
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	pub fn is_self (&self, other : &Lambda) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}




#[ derive ( Clone ) ] // OK
pub struct ProcedureLambda ( StdRc<LambdaInternals> );


impl ProcedureLambda {
	
	pub fn new (lambda : Lambda) -> (ProcedureLambda) {
		return ProcedureLambda (lambda.0);
	}
	
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return StdRc::clone (&self.0);
	}
	
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	pub fn is_self (&self, other : &ProcedureLambda) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}




#[ derive ( Clone ) ] // OK
pub struct SyntaxLambda ( StdRc<LambdaInternals> );


impl SyntaxLambda {
	
	pub fn new (lambda : Lambda) -> (SyntaxLambda) {
		return SyntaxLambda (lambda.0);
	}
	
	pub fn internals_ref (&self) -> (&LambdaInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	pub fn internals_rc_clone (&self) -> (StdRc<LambdaInternals>) {
		return StdRc::clone (&self.0);
	}
	
	pub fn internals_rc_into (self) -> (StdRc<LambdaInternals>) {
		return self.0;
	}
	
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle_2;
	}
	
	pub fn is_self (&self, other : &SyntaxLambda) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0)
	}
}

