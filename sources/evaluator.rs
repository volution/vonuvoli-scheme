

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;


pub mod exports {
	
	pub use super::Evaluator;
	
}




pub struct Evaluator {
}


impl Evaluator {
	
	
	pub fn new () -> (Evaluator) {
		Evaluator {}
	}
	
	
	
	
	pub fn evaluate (&mut self, context : &mut Context, input : &Expression) -> (Outcome<Value>) {
		
		match *input {
			
			Expression::Void => Ok (NULL.into ()),
			Expression::Value (ref value) => Ok (value.clone ()),
			
			Expression::ContextDefine (ref identifier, ref expression) =>
				self.evaluate_context_define (context, identifier, expression),
			Expression::ContextUpdate (ref identifier, ref expression) =>
				self.evaluate_context_update (context, identifier, expression),
			Expression::ContextSelect (ref identifier) =>
				self.evaluate_context_select (context, identifier),
			
			Expression::ProcedurePrimitiveCall0 (primitive) =>
				self.evaluate_procedure_primitive_0 (context, primitive),
			Expression::ProcedurePrimitiveCall1 (primitive, ref input) =>
				self.evaluate_procedure_primitive_1 (context, primitive, input),
			Expression::ProcedurePrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_procedure_primitive_2 (context, primitive, input_1, input_2),
			Expression::ProcedurePrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_procedure_primitive_n (context, primitive, inputs.as_ref ()),
			Expression::ProcedurePrimitiveCall (primitive, ref inputs) =>
				self.evaluate_procedure_primitive (context, primitive, inputs.as_ref ()),
			
			_ =>
				failed_unimplemented! (0xc1942075),
		}
		
	}
	
	#[ inline (always) ]
	pub fn evaluate_slice (&mut self, context : &mut Context, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		let mut outputs = Vec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (context, input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_context_define (&mut self, context : &mut Context, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let mut binding = try! (context.define (identifier));
		let value_new = try! (self.evaluate (context, expression));
		let value_old = binding.set (value_new);
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_context_update (&mut self, context : &mut Context, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let mut binding = try! (context.resolve (identifier));
		let value_new = try! (self.evaluate (context, expression));
		let value_old = binding.set (value_new);
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_context_select (&mut self, context : &mut Context, identifier : &Symbol) -> (Outcome<Value>) {
		let binding = try! (context.resolve (identifier));
		let value = binding.get ();
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_0 (&mut self, context : &mut Context, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		let output = procedure_primitive_0_evaluate (primitive, context);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_1 (&mut self, context : &mut Context, primitive : ProcedurePrimitive1, input : &Expression) -> (Outcome<Value>) {
		let input = try! (self.evaluate (context, input));
		let output = procedure_primitive_1_evaluate (primitive, &input, context);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_2 (&mut self, context : &mut Context, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (context, input_1));
		let input_2 = try! (self.evaluate (context, input_2));
		let output = procedure_primitive_2_evaluate (primitive, &input_1, &input_2, context);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_n (&mut self, context : &mut Context, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (context, inputs));
		let output = procedure_primitive_n_evaluate (primitive, inputs.as_ref (), context);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive (&mut self, context : &mut Context, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (context, inputs));
		let output = procedure_primitive_evaluate (primitive, inputs.as_ref (), context);
		return output;
	}
	
}

