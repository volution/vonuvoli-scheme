

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;


pub mod exports {
	
	pub use super::Evaluator;
	pub use super::EvaluationContext;
	
}




pub struct Evaluator {
}

pub struct EvaluationContext <'a> {
	pub evaluator : &'a Evaluator,
	pub context : &'a Context,
	pub registers : &'a Registers,
}




impl Evaluator {
	
	
	#[ inline (always) ]
	pub fn new () -> (Evaluator) {
		Evaluator {}
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_top (&self, context : &Context, input : &Expression) -> (Outcome<Value>) {
		let registers = Registers::new (0);
		let mut evaluation = EvaluationContext {
				evaluator : self,
				context : context,
				registers : &registers,
			};
		return self.evaluate (&mut evaluation, input);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate (&self, evaluation : &mut EvaluationContext, input : &Expression) -> (Outcome<Value>) {
		
		match *input {
			
			Expression::Void => Ok (NULL.into ()),
			Expression::Value (ref value) => Ok (value.clone ()),
			
			Expression::ContextDefine (ref identifier, ref expression) =>
				self.evaluate_context_define (evaluation, identifier, expression),
			Expression::ContextUpdate (ref identifier, ref expression) =>
				self.evaluate_context_update (evaluation, identifier, expression),
			Expression::ContextSelect (ref identifier) =>
				self.evaluate_context_select (evaluation, identifier),
			
			Expression::RegisterGet (index) =>
				self.evaluate_register_get (evaluation, index),
			Expression::RegisterSet (index, ref expression) =>
				self.evaluate_register_set (evaluation, index, expression),
			
			Expression::BindingGet (ref binding) =>
				self.evaluate_binding_get (evaluation, binding),
			Expression::BindingSet (ref binding, ref expression) =>
				self.evaluate_binding_set (evaluation, binding, expression),
			
			Expression::ProcedureCall (ref callable, ref inputs) =>
				self.evaluate_procedure_call (evaluation, callable, inputs.as_ref ()),
			
			Expression::ProcedurePrimitiveCall0 (primitive) =>
				self.evaluate_procedure_primitive_0 (evaluation, primitive),
			Expression::ProcedurePrimitiveCall1 (primitive, ref input) =>
				self.evaluate_procedure_primitive_1 (evaluation, primitive, input),
			Expression::ProcedurePrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_procedure_primitive_2 (evaluation, primitive, input_1, input_2),
			Expression::ProcedurePrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_procedure_primitive_n (evaluation, primitive, inputs.as_ref ()),
			Expression::ProcedurePrimitiveCall (primitive, ref inputs) =>
				self.evaluate_procedure_primitive (evaluation, primitive, inputs.as_ref ()),
			
			Expression::SyntaxPrimitiveCall1 (primitive, ref input) =>
				self.evaluate_syntax_primitive_1 (evaluation, primitive, input),
			Expression::SyntaxPrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_syntax_primitive_2 (evaluation, primitive, input_1, input_2),
			Expression::SyntaxPrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_syntax_primitive_n (evaluation, primitive, inputs.as_ref ()),
			Expression::SyntaxPrimitiveCall (primitive, ref inputs) =>
				self.evaluate_syntax_primitive (evaluation, primitive, inputs.as_ref ()),
			
		}
		
	}
	
	#[ inline (always) ]
	pub fn evaluate_slice (&self, evaluation : &mut EvaluationContext, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		let mut outputs = Vec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (evaluation, input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_context_define (&self, evaluation : &mut EvaluationContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let binding = try! (evaluation.context.define (identifier));
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_context_update (&self, evaluation : &mut EvaluationContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let binding = try! (evaluation.context.resolve (identifier));
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_context_select (&self, evaluation : &mut EvaluationContext, identifier : &Symbol) -> (Outcome<Value>) {
		let binding = try! (evaluation.context.resolve (identifier));
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_register_set (&self, evaluation : &mut EvaluationContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let binding = try! (evaluation.registers.resolve (index));
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_register_get (&self, evaluation : &mut EvaluationContext, index : usize) -> (Outcome<Value>) {
		let binding = try! (evaluation.registers.resolve (index));
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_binding_set (&self, evaluation : &mut EvaluationContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	pub fn evaluate_binding_get (&self, _evaluation : &mut EvaluationContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_procedure_call (&self, evaluation : &mut EvaluationContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		match callable {
			Value::ProcedurePrimitive (primitive) =>
				return self.evaluate_procedure_primitive (evaluation, primitive, inputs),
			_ =>
				return failed! (0xe5b2fe88),
		}
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_0 (&self, evaluation : &mut EvaluationContext, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		let output = procedure_primitive_0_evaluate (primitive, evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_1 (&self, evaluation : &mut EvaluationContext, primitive : ProcedurePrimitive1, input : &Expression) -> (Outcome<Value>) {
		let input = try! (self.evaluate (evaluation, input));
		let output = procedure_primitive_1_evaluate (primitive, &input, evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_2 (&self, evaluation : &mut EvaluationContext, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let output = procedure_primitive_2_evaluate (primitive, &input_1, &input_2, evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluationContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let output = procedure_primitive_n_evaluate (primitive, inputs.as_ref (), evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_procedure_primitive (&self, evaluation : &mut EvaluationContext, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let output = procedure_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
		return output;
	}
	
	
	
	
	#[ inline (always) ]
	pub fn evaluate_syntax_primitive_1 (&self, evaluation : &mut EvaluationContext, primitive : SyntaxPrimitive1, input : &Expression) -> (Outcome<Value>) {
		let output = syntax_primitive_1_evaluate (primitive, &input, evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_syntax_primitive_2 (&self, evaluation : &mut EvaluationContext, primitive : SyntaxPrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let output = syntax_primitive_2_evaluate (primitive, &input_1, &input_2, evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_syntax_primitive_n (&self, evaluation : &mut EvaluationContext, primitive : SyntaxPrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let output = syntax_primitive_n_evaluate (primitive, inputs.as_ref (), evaluation);
		return output;
	}
	
	#[ inline (always) ]
	pub fn evaluate_syntax_primitive (&self, evaluation : &mut EvaluationContext, primitive : SyntaxPrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let output = syntax_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
		return output;
	}
	
}

