

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::evaluate;
	pub use super::Evaluator;
	pub use super::EvaluatorContext;
}




pub fn evaluate (context : &Context, expression : &Expression) -> (Outcome<Value>) {
	return Evaluator::new () .evaluate (context, expression);
}




pub struct Evaluator {}


impl Evaluator {
	
	
	
	
	pub fn new () -> (Evaluator) {
		return Evaluator {};
	}
	
	pub fn evaluate (&self, context : &Context, expression : &Expression) -> (Outcome<Value>) {
		return self.fork (context) .evaluate (expression);
	}
	
	pub fn fork <'a> (&'a self, context : &'a Context) -> EvaluatorContext<'a> {
		return EvaluatorContext::new (self, Some (context), None);
	}
	
	
	
	
	pub fn evaluate_0 (&self, evaluation : &mut EvaluatorContext, input : &Expression) -> (Outcome<Value>) {
		
		match *input {
			
			Expression::Void =>
				Ok (VOID.into ()),
			Expression::Value (ref value) =>
				Ok (value.clone ()),
			
			Expression::Sequence (ref expressions) =>
				self.evaluate_sequence (evaluation, expressions),
			Expression::Conditional (ref clauses) =>
				self.evaluate_conditional (evaluation, clauses),
			
			Expression::ContextDefine (ref identifier, ref expression) =>
				self.evaluate_context_define (evaluation, identifier, expression),
			Expression::ContextUpdate (ref identifier, ref expression) =>
				self.evaluate_context_update (evaluation, identifier, expression),
			Expression::ContextSelect (ref identifier) =>
				self.evaluate_context_select (evaluation, identifier),
			
			Expression::RegisterClosure (ref expression, ref borrows) =>
				self.evaluate_register_closure (evaluation, expression, borrows),
			Expression::RegisterInitialize1 (index, ref expression) =>
				self.evaluate_register_initialize_1 (evaluation, index, expression),
			Expression::RegisterInitializeN (ref initializers, parallel) =>
				self.evaluate_register_initialize_n (evaluation, initializers, parallel),
			Expression::RegisterSet (index, ref expression) =>
				self.evaluate_register_set (evaluation, index, expression),
			Expression::RegisterGet (index) =>
				self.evaluate_register_get (evaluation, index),
			
			Expression::BindingInitialize1 (ref binding, ref expression) =>
				self.evaluate_binding_initialize_1 (evaluation, binding, expression),
			Expression::BindingInitializeN (ref initializers, parallel) =>
				self.evaluate_binding_initialize_n (evaluation, initializers, parallel),
			Expression::BindingSet (ref binding, ref expression) =>
				self.evaluate_binding_set (evaluation, binding, expression),
			Expression::BindingGet (ref binding) =>
				self.evaluate_binding_get (evaluation, binding),
			
			Expression::Lambda (ref lambda, ref expression, ref registers_closure, ref registers_local) =>
				self.evaluate_lambda_create (evaluation, lambda, expression, registers_closure, registers_local),
			
			Expression::ProcedureCall (ref callable, ref inputs) =>
				self.evaluate_procedure_call (evaluation, callable, inputs.as_ref ()),
			
			Expression::ProcedurePrimitiveCall0 (primitive) =>
				self.evaluate_procedure_primitive_0 (evaluation, primitive),
			Expression::ProcedurePrimitiveCall1 (primitive, ref input_1) =>
				self.evaluate_procedure_primitive_1 (evaluation, primitive, input_1),
			Expression::ProcedurePrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_procedure_primitive_2 (evaluation, primitive, input_1, input_2),
			Expression::ProcedurePrimitiveCall3 (primitive, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_primitive_3 (evaluation, primitive, input_1, input_2, input_3),
			Expression::ProcedurePrimitiveCall4 (primitive, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_primitive_4 (evaluation, primitive, input_1, input_2, input_3, input_4),
			Expression::ProcedurePrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_procedure_primitive_n (evaluation, primitive, inputs.as_ref ()),
			Expression::ProcedurePrimitiveCall (primitive, ref inputs) =>
				self.evaluate_procedure_primitive (evaluation, primitive, inputs.as_ref ()),
			
			Expression::SyntaxPrimitiveCall0 (primitive) =>
				self.evaluate_syntax_primitive_0 (evaluation, primitive),
			Expression::SyntaxPrimitiveCall1 (primitive, ref input_1) =>
				self.evaluate_syntax_primitive_1 (evaluation, primitive, input_1),
			Expression::SyntaxPrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_syntax_primitive_2 (evaluation, primitive, input_1, input_2),
			Expression::SyntaxPrimitiveCall3 (primitive, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_syntax_primitive_3 (evaluation, primitive, input_1, input_2, input_3),
			Expression::SyntaxPrimitiveCall4 (primitive, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_syntax_primitive_4 (evaluation, primitive, input_1, input_2, input_3, input_4),
			Expression::SyntaxPrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_syntax_primitive_n (evaluation, primitive, inputs.as_ref ()),
			Expression::SyntaxPrimitiveCall (primitive, ref inputs) =>
				self.evaluate_syntax_primitive (evaluation, primitive, inputs.as_ref ()),
			
		}
		
	}
	
	
	
	
	pub fn evaluate_sequence (&self, evaluation : &mut EvaluatorContext, expressions : &ExpressionVec) -> (Outcome<Value>) {
		let mut output = VOID.into ();
		for expression in expressions {
			output = try! (evaluation.evaluate (expression));
		}
		return Ok (output);
	}
	
	
	
	
	pub fn evaluate_conditional (&self, evaluation : &mut EvaluatorContext, clauses : &StdVec<(bool, Expression, Option<Expression>)>) -> (Outcome<Value>) {
		for &(negated, ref guard, ref expression) in clauses {
			let guard = try! (evaluation.evaluate (guard));
			let matched = if negated {
				is_false (&guard)
			} else {
				is_not_false (&guard)
			};
			if matched {
				if let Some (ref expression) = *expression {
					return evaluation.evaluate (expression);
				} else {
					succeed! (guard);
				}
			}
		}
		return Ok (VOID.into ());
	}
	
	
	
	
	pub fn evaluate_context_define (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xfe053ac6);
		let template = ContextBindingTemplate {
				identifier : identifier.clone (),
				value : None,
				immutable : false,
			};
		let binding = try! (context.define (&template));
		let value_new = try! (evaluation.evaluate (expression));
		let value_new = try! (binding.initialize (value_new));
		return Ok (value_new);
	}
	
	pub fn evaluate_context_update (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0x4be15062);
		let binding = try_some_2! (context.resolve (identifier), 0x8c4717b1);
		let value_new = try! (evaluation.evaluate (expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	pub fn evaluate_context_select (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xdf799bc8);
		let binding = try_some_2! (context.resolve (identifier), 0x8790e81e);
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	pub fn evaluate_register_closure (&self, evaluation : &mut EvaluatorContext, expression : &Expression, borrows : &[RegistersBindingTemplate]) -> (Outcome<Value>) {
		let registers = try! (Registers::new_and_define (borrows, evaluation.registers));
		let mut evaluation = EvaluatorContext::new (self, evaluation.context, Some (&registers));
		return evaluation.evaluate (expression);
	}
	
	pub fn evaluate_register_initialize_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x4f5f5ffc);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_initialize_1 (evaluation, &binding, expression);
	}
	
	pub fn evaluate_register_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &StdVec<(usize, Expression)>, parallel : bool) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x4f5f5ffc);
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		let bindings = try_vec_map! (indices, index, registers.resolve (index));
		let bindings = bindings.iter () .collect ();
		let initializers = vec_zip_2 (bindings, expressions);
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	pub fn evaluate_register_set (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x9e3f943b);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_set (evaluation, &binding, expression);
	}
	
	pub fn evaluate_register_get (&self, evaluation : &mut EvaluatorContext, index : usize) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x89f09b48);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_get (evaluation, &binding);
	}
	
	
	
	
	pub fn evaluate_binding_initialize_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (evaluation.evaluate (expression));
		let value_new = try! (binding.initialize (value_new));
		return Ok (value_new);
	}
	
	pub fn evaluate_binding_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &StdVec<(Binding, Expression)>, parallel : bool) -> (Outcome<Value>) {
		let initializers = initializers.iter () .map (|&(ref binding, ref expression)| (binding, expression)) .collect ();
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	pub fn evaluate_binding_initialize_n_0 (&self, evaluation : &mut EvaluatorContext, initializers : &StdVec<(&Binding, &Expression)>, parallel : bool) -> (Outcome<Value>) {
		
		let expressions = initializers.iter () .map (|&(_, expression)| expression) .collect::<StdVec<_>> ();
		let bindings = initializers.iter () .map (|&(binding, _)| binding) .collect::<StdVec<_>> ();
		
		if parallel {
			let values_new = try_vec_map! (expressions, expression, evaluation.evaluate (expression));
			for (binding, value_new) in vec_zip_2 (bindings, values_new) {
				try! (binding.initialize (value_new));
			}
		} else {
			for (binding, expression) in vec_zip_2 (bindings, expressions) {
				let value_new = try! (evaluation.evaluate (expression));
				try! (binding.initialize (value_new));
			}
		}
		
		return Ok (VOID);
	}
	
	pub fn evaluate_binding_set (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (evaluation.evaluate (expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	pub fn evaluate_binding_get (&self, _evaluation : &mut EvaluatorContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	pub fn evaluate_lambda_create (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaTemplate, expressions : &Expression, registers_closure : &StdVec<RegistersBindingTemplate>, registers_local : &StdVec<RegistersBindingTemplate>) -> (Outcome<Value>) {
		let registers_closure = try! (Registers::new_and_define (registers_closure, evaluation.registers));
		let lambda = Lambda::new (lambda.clone (), expressions.clone (), registers_closure, registers_local.clone ());
		succeed! (lambda.into ());
	}
	
	
	
	
	pub fn evaluate_lambda_call (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &inputs);
	}
	
	pub fn evaluate_lambda_call_with_values (&self, _evaluation : &mut EvaluatorContext, lambda : &Lambda, inputs : &[Value]) -> (Outcome<Value>) {
		
		let expression;
		let mut registers = Registers::new ();
		
		{
			let lambda = lambda.internals ();
			
			expression = lambda.expression.clone ();
			
			let inputs_count = inputs.len ();
			let mut inputs_offset = 0;
			for identifier in &lambda.arguments_positional {
				if inputs_offset >= inputs_count {
					fail! (0x1fbd1c55);
				}
				let register = RegistersBindingTemplate {
						identifier : Some (identifier.clone ()),
						value : Some (inputs[inputs_offset].clone ()),
						borrow : None,
						immutable : false,
					};
				try! (registers.define (&register, None));
				inputs_offset += 1;
			}
			if let Some (ref identifier) = lambda.argument_rest {
				let inputs = list_build_n (&inputs[inputs_offset..]);
				let register = RegistersBindingTemplate {
						identifier : Some (identifier.clone ()),
						value : Some (inputs),
						borrow : None,
						immutable : false,
					};
				try! (registers.define (&register, None));
			} else {
				if inputs_offset < inputs_count {
					fail! (0x6c9a5289);
				}
			}
			
			try! (registers.define_all (&lambda.registers_local, Some (&lambda.registers_closure)));
		}
		
		let mut evaluation = EvaluatorContext::new (self, None, Some (&registers));
		return evaluation.evaluate (&expression);
	}
	
	
	
	
	pub fn evaluate_procedure_call (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return self.evaluate_procedure_call_with_values (evaluation, &callable, &inputs);
	}
	
	pub fn evaluate_procedure_call_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				return self.evaluate_procedure_primitive_with_values (evaluation, *callable.as_ref (), inputs),
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_with_values (evaluation, callable.as_ref (), inputs),
			_ =>
				fail! (0x88be334b),
		}
	}
	
	
	
	
	pub fn evaluate_procedure_primitive_0 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		return procedure_primitive_0_evaluate (primitive, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_1 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (evaluation.evaluate (input_1));
		return procedure_primitive_1_evaluate (primitive, &input_1, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_1_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_2 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		return procedure_primitive_2_evaluate (primitive, &input_1, &input_2, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_2_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_3 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		return procedure_primitive_3_evaluate (primitive, &input_1, &input_2, &input_3, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_3_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_4 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		let input_4 = try! (evaluation.evaluate (input_4));
		return procedure_primitive_4_evaluate (primitive, &input_1, &input_2, &input_3, &input_4, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_4_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return procedure_primitive_n_evaluate (primitive, &inputs, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_n_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return procedure_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	pub fn evaluate_procedure_primitive_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[Value]) -> (Outcome<Value>) {
		return procedure_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	
	
	
	pub fn evaluate_syntax_primitive_0 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive0) -> (Outcome<Value>) {
		return syntax_primitive_0_evaluate (primitive, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_1 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive1, input_1 : &Expression) -> (Outcome<Value>) {
		return syntax_primitive_1_evaluate (primitive, input_1, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_2 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		return syntax_primitive_2_evaluate (primitive, input_1, input_2, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_3 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		return syntax_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_4 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		return syntax_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		return syntax_primitive_n_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	pub fn evaluate_syntax_primitive (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		return syntax_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	
}




pub struct EvaluatorContext <'a> {
	pub evaluator : &'a Evaluator,
	pub context : Option<&'a Context>,
	pub registers : Option<&'a Registers>,
}


impl <'a> EvaluatorContext<'a> {
	
	pub fn new (evaluator : &'a Evaluator, context : Option<&'a Context>, registers : Option<&'a Registers>) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : evaluator,
				context : context,
				registers : registers,
			}
	}
	
	pub fn evaluate (&mut self, input : &Expression) -> (Outcome<Value>) {
		return self.evaluator.evaluate_0 (self, input);
	}
	
	pub fn evaluate_slice (&mut self, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		let mut outputs = Vec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
}

