

use super::errors::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::EvaluatorContext;
}




pub trait EvaluatorContext {
	
	fn evaluate_procedure_call_0 (&mut self, callable : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_1 (&mut self, callable : &Value, input_1 : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_2 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_3 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_4 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_5 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>);
	fn evaluate_procedure_call_n (&mut self, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>);
	
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	fn parameters (&mut self) -> (Outcome<&Parameters>);
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	fn parameter_resolve (&mut self, parameter : &Parameter, default : Option<&Value>) -> (Outcome<Value>);
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	fn parameter_resolve_for_builtin (&mut self, parameter : &UniqueData) -> (Outcome<Option<Value>>);
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	fn parameter_configure (&mut self, parameter : &Parameter, value : &Value) -> (Outcome<()>);
	
}

