

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
			
			Expression::PrimitiveCall0 (primitive) =>
				self.evaluate_primitive_0 (context, primitive),
			Expression::PrimitiveCall1 (primitive, ref input) =>
				self.evaluate_primitive_1 (context, primitive, input),
			Expression::PrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_primitive_2 (context, primitive, input_1, input_2),
			Expression::PrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_primitive_n (context, primitive, inputs.as_ref ()),
			Expression::PrimitiveCall (primitive, ref inputs) =>
				self.evaluate_primitive (context, primitive, inputs.as_ref ()),
			
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
	pub fn evaluate_primitive_0 (&mut self, context : &mut Context, primitive : Primitive0) -> (Outcome<Value>) {
		let output = primitive_0_evaluate (primitive);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_primitive_1 (&mut self, context : &mut Context, primitive : Primitive1, input : &Expression) -> (Outcome<Value>) {
		let input = try! (self.evaluate (context, input));
		let output = primitive_1_evaluate (primitive, &input);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_primitive_2 (&mut self, context : &mut Context, primitive : Primitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (context, input_1));
		let input_2 = try! (self.evaluate (context, input_2));
		let output = primitive_2_evaluate (primitive, &input_1, &input_2);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_primitive_n (&mut self, context : &mut Context, primitive : PrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (context, inputs));
		let output = primitive_n_evaluate (primitive, inputs.as_ref ());
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_primitive (&mut self, context : &mut Context, primitive : Primitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (context, inputs));
		let output = primitive_evaluate (primitive, inputs.as_ref ());
		return output;
	}
	
}

