

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::lambdas::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;




pub mod exports {
	pub use super::evaluate;
	pub use super::evaluate_script;
	pub use super::Evaluator;
	pub use super::EvaluatorContext;
}




pub fn evaluate (context : &Context, expression : &Expression) -> (Outcome<Value>) {
	let evaluator = Evaluator::new ();
	let mut evaluation = evaluator.fork (context);
	return evaluation.evaluate (expression);
}

pub fn evaluate_script <Iterator, ExpressionRef> (context : &Context, expressions : Iterator) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : AsRef<Expression>
{
	let evaluator = Evaluator::new ();
	let mut evaluation = evaluator.fork (context);
	return evaluation.evaluate_script (expressions);
}




#[ derive (Clone, Debug) ]
pub struct Evaluator {}


impl Evaluator {
	
	
	
	
	pub fn new () -> (Evaluator) {
		return Evaluator {};
	}
	
	pub fn fork <'a> (&'a self, context : &'a Context) -> (EvaluatorContext<'a>) {
		return EvaluatorContext::new (self, Some (context), None);
	}
	
	pub fn fork_0 <'a> (&'a self) -> (EvaluatorContext<'a>) {
		return EvaluatorContext::new (self, None, None);
	}
	
	
	
	
	fn evaluate (&self, evaluation : &mut EvaluatorContext, input : &Expression) -> (Outcome<Value>) {
		
		match *input {
			
			Expression::Void =>
				Ok (VOID.into ()),
			Expression::Value (ref value) =>
				Ok (value.clone ()),
			
			Expression::Sequence (operator, ref expressions) =>
				self.evaluate_sequence (evaluation, operator, expressions),
			Expression::ConditionalIf (ref clauses) =>
				self.evaluate_conditional_if (evaluation, clauses),
			Expression::ConditionalMatch (ref actual, ref clauses) =>
				self.evaluate_conditional_match (evaluation, actual, clauses),
			Expression::Loop (ref initialize, ref update, ref body, ref clauses) =>
				self.evaluate_loop (evaluation, option_box_as_ref (initialize), option_box_as_ref (update), option_box_as_ref (body), clauses),
			
			Expression::Contexts (ref expression) =>
				self.evaluate_for_contexts (evaluation, expression),
			
			Expression::ProcedureGenericCall (ref expression) =>
				self.evaluate_for_procedure_generic_call (evaluation, expression),
			Expression::ProcedurePrimitiveCall (ref expression) =>
				self.evaluate_for_procedure_primitive_call (evaluation, expression),
			Expression::ProcedureExtendedCall (ref expression) =>
				self.evaluate_for_procedure_extended_call (evaluation, expression),
			Expression::ProcedureLambdaCall (ref expression) =>
				self.evaluate_for_procedure_lambda_call (evaluation, expression),
			
			Expression::Lambda (ref lambda, ref expression, ref registers_closure, ref registers_local) =>
				self.evaluate_lambda_create (evaluation, lambda, expression, registers_closure, registers_local),
			
		}
		
	}
	
	
	fn evaluate_for_contexts (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForContexts) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForContexts::ContextDefine (ref identifier, ref expression) =>
				self.evaluate_context_define (evaluation, identifier, expression),
			ExpressionForContexts::ContextUpdate (ref identifier, ref expression) =>
				self.evaluate_context_update (evaluation, identifier, expression),
			ExpressionForContexts::ContextSelect (ref identifier) =>
				self.evaluate_context_select (evaluation, identifier),
			
			ExpressionForContexts::BindingInitialize1 (ref binding, ref expression) =>
				self.evaluate_binding_initialize_1 (evaluation, binding, expression),
			ExpressionForContexts::BindingInitializeN (ref initializers, parallel) =>
				self.evaluate_binding_initialize_n (evaluation, initializers, parallel),
			ExpressionForContexts::BindingInitializeValues (ref bindings, ref expression) =>
				self.evaluate_binding_initialize_values (evaluation, bindings, expression),
			ExpressionForContexts::BindingSet1 (ref binding, ref expression) =>
				self.evaluate_binding_set_1 (evaluation, binding, expression),
			ExpressionForContexts::BindingSetN (ref initializers, parallel) =>
				self.evaluate_binding_set_n (evaluation, initializers, parallel),
			ExpressionForContexts::BindingSetValues (ref bindings, ref expression) =>
				self.evaluate_binding_set_values (evaluation, bindings, expression),
			ExpressionForContexts::BindingGet1 (ref binding) =>
				self.evaluate_binding_get_1 (evaluation, binding),
			
			ExpressionForContexts::RegisterClosure (ref expression, ref borrows) =>
				self.evaluate_register_closure (evaluation, expression, borrows),
			ExpressionForContexts::RegisterInitialize1 (index, ref expression) =>
				self.evaluate_register_initialize_1 (evaluation, index, expression),
			ExpressionForContexts::RegisterInitializeN (ref initializers, parallel) =>
				self.evaluate_register_initialize_n (evaluation, initializers, parallel),
			ExpressionForContexts::RegisterInitializeValues (ref indices, ref expression) =>
				self.evaluate_register_initialize_values (evaluation, indices, expression),
			ExpressionForContexts::RegisterSet1 (index, ref expression) =>
				self.evaluate_register_set_1 (evaluation, index, expression),
			ExpressionForContexts::RegisterSetN (ref initializers, parallel) =>
				self.evaluate_register_set_n (evaluation, initializers, parallel),
			ExpressionForContexts::RegisterSetValues (ref indices, ref expression) =>
				self.evaluate_register_set_values (evaluation, indices, expression),
			ExpressionForContexts::RegisterGet1 (index) =>
				self.evaluate_register_get_1 (evaluation, index),
			
		}
	}
	
	
	fn evaluate_for_procedure_generic_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureGenericCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureGenericCall::ProcedureCall (ref callable, ref inputs) =>
				self.evaluate_procedure_call (evaluation, callable, inputs),
			ExpressionForProcedureGenericCall::ProcedureCall0 (ref callable) =>
				self.evaluate_procedure_call_0 (evaluation, callable),
			ExpressionForProcedureGenericCall::ProcedureCall1 (ref callable, ref input_1) =>
				self.evaluate_procedure_call_1 (evaluation, callable, input_1),
			ExpressionForProcedureGenericCall::ProcedureCall2 (ref callable, ref input_1, ref input_2) =>
				self.evaluate_procedure_call_2 (evaluation, callable, input_1, input_2),
			ExpressionForProcedureGenericCall::ProcedureCall3 (ref callable, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_call_3 (evaluation, callable, input_1, input_2, input_3),
			ExpressionForProcedureGenericCall::ProcedureCall4 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_call_4 (evaluation, callable, input_1, input_2, input_3, input_4),
			ExpressionForProcedureGenericCall::ProcedureCall5 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_call_5 (evaluation, callable, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedureGenericCall::ProcedureCallN (ref callable, ref inputs) =>
				self.evaluate_procedure_call_n (evaluation, callable, inputs),
			
		}
	}
	
	
	fn evaluate_for_procedure_primitive_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedurePrimitiveCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall (primitive, ref inputs) =>
				self.evaluate_procedure_primitive (evaluation, primitive, inputs),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (primitive) =>
				self.evaluate_procedure_primitive_0 (evaluation, primitive),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (primitive, ref input_1) =>
				self.evaluate_procedure_primitive_1 (evaluation, primitive, input_1),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (primitive, ref input_1, ref input_2) =>
				self.evaluate_procedure_primitive_2 (evaluation, primitive, input_1, input_2),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (primitive, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_primitive_3 (evaluation, primitive, input_1, input_2, input_3),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (primitive, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_primitive_4 (evaluation, primitive, input_1, input_2, input_3, input_4),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (primitive, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_primitive_5 (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_procedure_primitive_n (evaluation, primitive, inputs),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallV (primitive, ref inputs) =>
				self.evaluate_procedure_primitive_v (evaluation, primitive, inputs),
			
		}
	}
	
	
	fn evaluate_for_procedure_extended_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureExtendedCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall (ref procedure, ref inputs) =>
				self.evaluate_procedure_extended (evaluation, procedure, inputs),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (ref procedure) =>
				self.evaluate_procedure_extended_0 (evaluation, procedure),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (ref procedure, ref input_1) =>
				self.evaluate_procedure_extended_1 (evaluation, procedure, input_1),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (ref procedure, ref input_1, ref input_2) =>
				self.evaluate_procedure_extended_2 (evaluation, procedure, input_1, input_2),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (ref procedure, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_extended_3 (evaluation, procedure, input_1, input_2, input_3),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_extended_4 (evaluation, procedure, input_1, input_2, input_3, input_4),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_extended_5 (evaluation, procedure, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (ref procedure, ref inputs) =>
				self.evaluate_procedure_extended_n (evaluation, procedure, inputs),
			
		}
	}
	
	
	fn evaluate_for_procedure_lambda_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureLambdaCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall (ref lambda, ref inputs) =>
				self.evaluate_procedure_lambda (evaluation, lambda, inputs),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (ref lambda) =>
				self.evaluate_procedure_lambda_0 (evaluation, lambda),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (ref lambda, ref input_1) =>
				self.evaluate_procedure_lambda_1 (evaluation, lambda, input_1),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (ref lambda, ref input_1, ref input_2) =>
				self.evaluate_procedure_lambda_2 (evaluation, lambda, input_1, input_2),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (ref lambda, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_lambda_3 (evaluation, lambda, input_1, input_2, input_3),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (ref lambda, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_lambda_4 (evaluation, lambda, input_1, input_2, input_3, input_4),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (ref lambda, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_lambda_5 (evaluation, lambda, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (ref lambda, ref inputs) =>
				self.evaluate_procedure_lambda_n (evaluation, lambda, inputs),
			
		}
	}
	
	
	fn evaluate_slice (&self, evaluation : &mut EvaluatorContext, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		let mut outputs = Vec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (evaluation, input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
	
	
	
	
	fn evaluate_sequence (&self, evaluation : &mut EvaluatorContext, operator : ExpressionSequenceOperator, expressions : &[Expression]) -> (Outcome<Value>) {
		match operator {
			
			ExpressionSequenceOperator::ReturnLast => {
				let mut output = None;
				for expression in expressions {
					let output_1 = try! (self.evaluate (evaluation, expression));
					output = Some (output_1);
				}
				if let Some (output) = output {
					succeed! (output);
				} else {
					succeed! (VOID.into ());
				}
			},
			
			ExpressionSequenceOperator::ReturnFirst => {
				let mut output = None;
				for expression in expressions {
					let output_1 = try! (self.evaluate (evaluation, expression));
					if output.is_none () {
						output = Some (output_1);
					}
				}
				if let Some (output) = output {
					succeed! (output);
				} else {
					succeed! (VOID.into ());
				}
			},
			
			ExpressionSequenceOperator::And => {
				let mut output = None;
				for expression in expressions {
					let output_1 = try! (self.evaluate (evaluation, expression));
					if is_false (&output_1) {
						succeed! (output_1);
					}
					output = Some (output_1);
				}
				if let Some (output) = output {
					succeed! (output);
				} else {
					succeed! (TRUE.into ());
				}
			},
			
			ExpressionSequenceOperator::Or => {
				let mut output = None;
				for expression in expressions {
					let output_1 = try! (self.evaluate (evaluation, expression));
					if is_not_false (&output_1) {
						succeed! (output_1);
					}
					output = Some (output_1);
				}
				if let Some (output) = output {
					succeed! (output);
				} else {
					succeed! (FALSE.into ());
				}
			}
		}
	}
	
	
	
	
	fn evaluate_conditional_if (&self, evaluation : &mut EvaluatorContext, clauses : &[(Option<(Expression, bool)>, Option<Expression>)]) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_if_clauses (evaluation, clauses)) {
			succeed! (output);
		} else {
			succeed! (VOID.into ());
		}
	}
	
	fn evaluate_conditional_if_clauses (&self, evaluation : &mut EvaluatorContext, clauses : &[(Option<(Expression, bool)>, Option<Expression>)]) -> (Outcome<Option<Value>>) {
		for &(ref guard, ref expression) in clauses {
			let (matched, guard) = match *guard {
				Some ((ref guard, negated)) => {
					let guard = try! (self.evaluate (evaluation, guard));
					let matched = if negated {
						is_false (&guard)
					} else {
						is_not_false (&guard)
					};
					(matched, guard)
				},
				None =>
					(true, TRUE.into ()),
			};
			if matched {
				if let Some (ref expression) = *expression {
					let output = try! (self.evaluate (evaluation, expression));
					succeed! (Some (output));
				} else {
					succeed! (Some (guard));
				}
			}
		}
		succeed! (None);
	}
	
	
	fn evaluate_conditional_match (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_match_clauses (evaluation, actual, clauses)) {
			succeed! (output);
		} else {
			succeed! (VOID.into ());
		}
	}
	
	fn evaluate_conditional_match_clauses (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]) -> (Outcome<Option<Value>>) {
		let actual = try! (self.evaluate (evaluation, actual));
		for &(ref expected, ref expression) in clauses {
			let matched = match *expected {
				Some ((ref expected, negated)) => {
					let mut matched = false;
					for expected in expected.iter () {
						if try! (equivalent_by_value_strict_2 (&actual, expected)) {
							if !negated {
								matched = true;
							}
							break;
						}
					}
					matched
				},
				None =>
					true,
			};
			if matched {
				if let Some (ref expression) = *expression {
					let output = try! (self.evaluate (evaluation, expression));
					succeed! (Some (output));
				} else {
					succeed! (Some (actual));
				}
			}
		}
		succeed! (None);
	}
	
	
	
	
	fn evaluate_loop (&self, evaluation : &mut EvaluatorContext, initialize : Option<&Expression>, update : Option<&Expression>, body : Option<&Expression>, clauses : &[(Option<(Expression, bool)>, Option<Expression>)]) -> (Outcome<Value>) {
		
		if let Some (initialize) = initialize {
			try! (self.evaluate (evaluation, initialize));
		}
		
		loop {
			
			if let Some (output) = try! (self.evaluate_conditional_if_clauses (evaluation, clauses)) {
				succeed! (output);
			}
			
			if let Some (body) = body {
				try! (self.evaluate (evaluation, body));
			}
			
			if let Some (update) = update {
				try! (self.evaluate (evaluation, update));
			}
		}
	}
	
	
	
	
	fn evaluate_context_define (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xfe053ac6);
		let template = ContextBindingTemplate {
				identifier : identifier.clone (),
				value : None,
				immutable : false,
			};
		let binding = try! (context.define (&template));
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_new = try! (binding.initialize (value_new));
		return Ok (value_new);
	}
	
	fn evaluate_context_update (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0x4be15062);
		let binding = try_some_2! (context.resolve (identifier), 0x8c4717b1);
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	fn evaluate_context_select (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xdf799bc8);
		let binding = try_some_2! (context.resolve (identifier), 0x8790e81e);
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	fn evaluate_binding_initialize_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_new = try! (binding.initialize (value_new));
		return Ok (value_new);
	}
	
	fn evaluate_binding_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(Binding, Expression)], parallel : bool) -> (Outcome<Value>) {
		let initializers = initializers.iter () .map (|&(ref binding, ref expression)| (binding, expression)) .collect::<StdVec<_>> ();
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	fn evaluate_binding_initialize_n_0 (&self, evaluation : &mut EvaluatorContext, initializers : &[(&Binding, &Expression)], parallel : bool) -> (Outcome<Value>) {
		
		let expressions = initializers.iter () .map (|&(_, expression)| expression) .collect::<StdVec<_>> ();
		let bindings = initializers.iter () .map (|&(binding, _)| binding) .collect::<StdVec<_>> ();
		
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (binding, value_new) in vec_zip_2 (bindings, values_new) {
				try! (binding.initialize (value_new));
			}
		} else {
			for (binding, expression) in vec_zip_2 (bindings, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				try! (binding.initialize (value_new));
			}
		}
		
		return Ok (VOID.into ());
	}
	
	fn evaluate_binding_initialize_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != bindings.len () {
			fail! (0x34cd5a9a);
		}
		for (binding, value_new) in bindings.iter () .zip (values_new.values_ref () .iter ()) {
			try! (binding.initialize (value_new.clone ()));
		}
		return Ok (values_new.into ());
	}
	
	fn evaluate_binding_set_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	
	fn evaluate_binding_set_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(Binding, Expression)], parallel : bool) -> (Outcome<Value>) {
		let initializers = initializers.iter () .map (|&(ref binding, ref expression)| (binding, expression)) .collect::<StdVec<_>> ();
		return self.evaluate_binding_set_n_0 (evaluation, &initializers, parallel);
	}
	
	fn evaluate_binding_set_n_0 (&self, evaluation : &mut EvaluatorContext, initializers : &[(&Binding, &Expression)], parallel : bool) -> (Outcome<Value>) {
		
		let expressions = initializers.iter () .map (|&(_, expression)| expression) .collect::<StdVec<_>> ();
		let bindings = initializers.iter () .map (|&(binding, _)| binding) .collect::<StdVec<_>> ();
		
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (binding, value_new) in vec_zip_2 (bindings, values_new) {
				try! (binding.set (value_new));
			}
		} else {
			for (binding, expression) in vec_zip_2 (bindings, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				try! (binding.set (value_new));
			}
		}
		
		return Ok (VOID.into ());
	}
	
	
	fn evaluate_binding_set_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new_ = try! (self.evaluate (evaluation, expression));
		let values_new_ = try_into_values! (values_new_);
		if values_new_.values_length () != bindings.len () {
			fail! (0xd47ae677);
		}
		let mut values_old = StdVec::with_capacity (values_new_.values_length ());
		for (binding, value_new) in bindings.iter () .zip (values_new_.values_ref () .iter ()) {
			let value_old = try! (binding.set (value_new.clone ()));
			values_old.push (value_old);
		}
		return Ok (values_new (values_old.into_boxed_slice ()) .into ());
	}
	
	fn evaluate_binding_get_1 (&self, _evaluation : &mut EvaluatorContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	fn evaluate_register_closure (&self, evaluation : &mut EvaluatorContext, expression : &Expression, borrows : &[RegistersBindingTemplate]) -> (Outcome<Value>) {
		let registers = try! (Registers::new_and_define (borrows, evaluation.registers));
		let mut evaluation = EvaluatorContext::new (self, evaluation.context, Some (&registers));
		return self.evaluate (&mut evaluation, expression);
	}
	
	fn evaluate_register_initialize_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x4f5f5ffc);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_initialize_1 (evaluation, &binding, expression);
	}
	
	fn evaluate_register_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x4f5f5ffc);
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		let bindings = try_vec_map_into! (indices, index, registers.resolve (index));
		let bindings = bindings.iter () .collect ();
		let initializers = vec_zip_2 (bindings, expressions);
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	fn evaluate_register_initialize_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x64f31042);
		let bindings = try_vec_map_into! (indices, index, registers.resolve (*index));
		return self.evaluate_binding_initialize_values (evaluation, &bindings, expression);
	}
	
	fn evaluate_register_set_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x9e3f943b);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_set_1 (evaluation, &binding, expression);
	}
	
	fn evaluate_register_set_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0xf6492a67);
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		let bindings = try_vec_map_into! (indices, index, registers.resolve (index));
		let bindings = bindings.iter () .collect ();
		let initializers = vec_zip_2 (bindings, expressions);
		return self.evaluate_binding_set_n_0 (evaluation, &initializers, parallel);
	}
	
	fn evaluate_register_set_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x2137dc1e);
		let bindings = try_vec_map_into! (indices, index, registers.resolve (*index));
		return self.evaluate_binding_set_values (evaluation, &bindings, expression);
	}
	
	fn evaluate_register_get_1 (&self, evaluation : &mut EvaluatorContext, index : usize) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x89f09b48);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_get_1 (evaluation, &binding);
	}
	
	
	
	
	fn evaluate_lambda_create (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaTemplate, expressions : &Expression, registers_closure : &[RegistersBindingTemplate], registers_local : &[RegistersBindingTemplate]) -> (Outcome<Value>) {
		let registers_closure = try! (Registers::new_and_define (registers_closure, evaluation.registers));
		let lambda = Lambda::new (lambda.clone (), expressions.clone (), registers_closure, registers_local.to_vec ());
		succeed! (ProcedureLambda::new (lambda) .into ());
	}
	
	
	
	
	fn evaluate_procedure_lambda (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &inputs);
	}
	
	fn evaluate_procedure_lambda_with_values (&self, _evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[&Value]) -> (Outcome<Value>) {
		
		let expression;
		let mut registers = Registers::new ();
		
		{
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
		return self.evaluate (&mut evaluation, &expression);
	}
	
	
	fn evaluate_procedure_lambda_0 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[]);
	}
	
	
	fn evaluate_procedure_lambda_1 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_lambda_1_with_values (evaluation, lambda, &input_1);
	}
	
	fn evaluate_procedure_lambda_1_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1]);
	}
	
	
	fn evaluate_procedure_lambda_2 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_lambda_2_with_values (evaluation, lambda, &input_1, &input_2);
	}
	
	fn evaluate_procedure_lambda_2_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2]);
	}
	
	
	fn evaluate_procedure_lambda_3 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_lambda_3_with_values (evaluation, lambda, &input_1, &input_2, &input_3);
	}
	
	fn evaluate_procedure_lambda_3_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3]);
	}
	
	
	fn evaluate_procedure_lambda_4 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_lambda_4_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4);
	}
	
	fn evaluate_procedure_lambda_4_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4]);
	}
	
	
	fn evaluate_procedure_lambda_5 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_lambda_5_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	fn evaluate_procedure_lambda_5_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4, input_5]);
	}
	
	
	fn evaluate_procedure_lambda_n (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_n_with_values (evaluation, lambda, &inputs);
	}
	
	fn evaluate_procedure_lambda_n_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[&Value]) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, inputs);
	}
	
	
	
	
	fn evaluate_procedure_call (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_with_values (evaluation, &callable, &inputs);
	}
	
	fn evaluate_procedure_call_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				return self.evaluate_procedure_primitive_with_values (evaluation, *callable.as_ref (), inputs),
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_with_values (evaluation, callable.as_ref (), inputs),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), inputs),
			_ =>
				fail! (0x88be334b),
		}
	}
	
	
	fn evaluate_procedure_call_0 (&self, evaluation : &mut EvaluatorContext, callable : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		return self.evaluate_procedure_call_0_with_values (evaluation, &callable);
	}
	
	fn evaluate_procedure_call_0_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive0 (primitive) =>
						return self.evaluate_procedure_primitive_0 (evaluation, primitive),
					primitive =>
						return procedure_primitive_g_evaluate_0 (primitive, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_0 (evaluation, callable.as_ref ()),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_0 (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals ()),
			_ =>
				fail! (0xc58e190e),
		}
	}
	
	
	fn evaluate_procedure_call_1 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_call_1_with_values (evaluation, &callable, &input_1);
	}
	
	fn evaluate_procedure_call_1_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive1 (primitive) =>
						return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, input_1),
					primitive =>
						return procedure_primitive_g_evaluate_1 (primitive, input_1, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_1_with_values (evaluation, callable.as_ref (), input_1),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_1_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), input_1),
			_ =>
				fail! (0xe7f6dbfc),
		}
	}
	
	
	fn evaluate_procedure_call_2 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_call_2_with_values (evaluation, &callable, &input_1, &input_2);
	}
	
	fn evaluate_procedure_call_2_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive2 (primitive) =>
						return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, input_1, input_2),
					primitive =>
						return procedure_primitive_g_evaluate_2 (primitive, input_1, input_2, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_2_with_values (evaluation, callable.as_ref (), input_1, input_2),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_2_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), input_1, input_2),
			_ =>
				fail! (0x856bf44d),
		}
	}
	
	
	fn evaluate_procedure_call_3 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_call_3_with_values (evaluation, &callable, &input_1, &input_2, &input_3);
	}
	
	fn evaluate_procedure_call_3_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive3 (primitive) =>
						return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, input_1, input_2, input_3),
					primitive =>
						return procedure_primitive_g_evaluate_3 (primitive, input_1, input_2, input_3, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_3_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_3_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), input_1, input_2, input_3),
			_ =>
				fail! (0xdb6b9bbc),
		}
	}
	
	
	fn evaluate_procedure_call_4 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_call_4_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4);
	}
	
	fn evaluate_procedure_call_4_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive4 (primitive) =>
						return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, input_1, input_2, input_3, input_4),
					primitive =>
						return procedure_primitive_g_evaluate_4 (primitive, input_1, input_2, input_3, input_4, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_4_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3, input_4),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_4_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), input_1, input_2, input_3, input_4),
			_ =>
				fail! (0xf0969d74),
		}
	}
	
	
	fn evaluate_procedure_call_5 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_call_5_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	fn evaluate_procedure_call_5_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive5 (primitive) =>
						return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
					primitive =>
						return procedure_primitive_g_evaluate_5 (primitive, input_1, input_2, input_3, input_4, input_5, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_5_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3, input_4, input_5),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_5_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), input_1, input_2, input_3, input_4, input_5),
			_ =>
				fail! (0x62e8aef5),
		}
	}
	
	
	fn evaluate_procedure_call_n (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_n_with_values (evaluation, &callable, &inputs);
	}
	
	fn evaluate_procedure_call_n_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::PrimitiveN (primitive) =>
						return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, inputs),
					primitive =>
						return procedure_primitive_g_evaluate_n (primitive, inputs, evaluation),
				},
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_n_with_values (evaluation, callable.as_ref (), inputs),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_n_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals (), inputs),
			_ =>
				fail! (0x906ebf03),
		}
	}
	
	
	
	
	fn evaluate_procedure_primitive (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_with_values (evaluation, primitive, &inputs);
	}
	
	fn evaluate_procedure_primitive_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_0 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		return procedure_primitive_0_evaluate (primitive, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_1 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, &input_1);
	}
	
	fn evaluate_procedure_primitive_1_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_2 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, &input_1, &input_2);
	}
	
	fn evaluate_procedure_primitive_2_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_3 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, &input_1, &input_2, &input_3);
	}
	
	fn evaluate_procedure_primitive_3_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_4 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4);
	}
	
	fn evaluate_procedure_primitive_4_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_5 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	fn evaluate_procedure_primitive_5_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, &inputs);
	}
	
	fn evaluate_procedure_primitive_n_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluation);
	}
	
	
	fn evaluate_procedure_primitive_v (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_v_with_values (evaluation, primitive, &inputs);
	}
	
	fn evaluate_procedure_primitive_v_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_v_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	
	
	fn evaluate_procedure_extended (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_with_values (evaluation, extended, &inputs);
	}
	
	fn evaluate_procedure_extended_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	fn evaluate_procedure_extended_0 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended) -> (Outcome<Value>) {
		return procedure_extended_evaluate_0 (extended, evaluation);
	}
	
	
	fn evaluate_procedure_extended_1 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_extended_1_with_values (evaluation, extended, &input_1);
	}
	
	fn evaluate_procedure_extended_1_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_1 (extended, input_1, evaluation);
	}
	
	
	fn evaluate_procedure_extended_2 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_extended_2_with_values (evaluation, extended, &input_1, &input_2);
	}
	
	fn evaluate_procedure_extended_2_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_2 (extended, input_1, input_2, evaluation);
	}
	
	
	fn evaluate_procedure_extended_3 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_extended_3_with_values (evaluation, extended, &input_1, &input_2, &input_3);
	}
	
	fn evaluate_procedure_extended_3_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_3 (extended, input_1, input_2, input_3, evaluation);
	}
	
	
	fn evaluate_procedure_extended_4 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_extended_4_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4);
	}
	
	fn evaluate_procedure_extended_4_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_4 (extended, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	fn evaluate_procedure_extended_5 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_extended_5_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	fn evaluate_procedure_extended_5_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_5 (extended, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	fn evaluate_procedure_extended_n (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_n_with_values (evaluation, extended, &inputs);
	}
	
	fn evaluate_procedure_extended_n_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	
	
}




#[ derive (Clone, Debug) ]
pub struct EvaluatorContext <'a> {
	evaluator : &'a Evaluator,
	context : Option<&'a Context>,
	registers : Option<&'a Registers>,
}


impl <'a> EvaluatorContext<'a> {
	
	
	fn new (evaluator : &'a Evaluator, context : Option<&'a Context>, registers : Option<&'a Registers>) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : evaluator,
				context : context,
				registers : registers,
			}
	}
	
	
	pub fn evaluate (&mut self, input : &Expression) -> (Outcome<Value>) {
		return self.evaluator.evaluate (self, input);
	}
	
	pub fn evaluate_slice (&mut self, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		return self.evaluator.evaluate_slice (self, inputs);
	}
	
	pub fn evaluate_script <Iterator, ExpressionRef> (&mut self, inputs : Iterator) -> (Outcome<()>)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : AsRef<Expression>
	{
		for input in inputs {
			try! (self.evaluate (input.as_ref ()));
		}
		return Ok (());
	}
	
	
	pub fn evaluate_procedure_call_0 (&mut self, callable : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_0_with_values (self, callable);
	}
	
	pub fn evaluate_procedure_call_1 (&mut self, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_1_with_values (self, callable, input_1);
	}
	
	pub fn evaluate_procedure_call_2 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_2_with_values (self, callable, input_1, input_2);
	}
	
	pub fn evaluate_procedure_call_3 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_3_with_values (self, callable, input_1, input_2, input_3);
	}
	
	pub fn evaluate_procedure_call_4 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_4_with_values (self, callable, input_1, input_2, input_3, input_4);
	}
	
	pub fn evaluate_procedure_call_5 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_5_with_values (self, callable, input_1, input_2, input_3, input_4, input_5);
	}
	
	pub fn evaluate_procedure_call_n (&mut self, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_n_with_values (self, callable, inputs);
	}
}

