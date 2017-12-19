

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
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




#[ inline (always) ]
pub fn evaluate (context : &Context, expression : &Expression) -> (Outcome<Value>) {
	let evaluator = Evaluator::new ();
	let mut evaluation = evaluator.fork (context);
	return evaluation.evaluate (expression);
}

#[ inline (always) ]
pub fn evaluate_script <Iterator, ExpressionRef> (context : &Context, expressions : Iterator) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : AsRef<Expression>
{
	let evaluator = Evaluator::new ();
	let mut evaluation = evaluator.fork (context);
	return evaluation.evaluate_script (expressions);
}




#[ derive (Debug) ]
pub struct Evaluator {}


impl Evaluator {
	
	
	
	
	#[ inline (always) ]
	pub fn new () -> (Evaluator) {
		return Evaluator {};
	}
	
	#[ inline (always) ]
	pub fn fork <'a> (&'a self, context : &'a Context) -> (EvaluatorContext<'a>) {
		return EvaluatorContext::new (self, Some (context), Registers::new ());
	}
	
	#[ inline (always) ]
	pub fn fork_0 <'a> (&'a self) -> (EvaluatorContext<'a>) {
		return EvaluatorContext::new (self, None, Registers::new ());
	}
	
	
	
	
	#[ inline (never) ]
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
			Expression::ProcedureNativeCall (ref expression) =>
				self.evaluate_for_procedure_native_call (evaluation, expression),
			
			Expression::Lambda (ref lambda, ref expression, ref registers_closure, ref registers_local) =>
				self.evaluate_lambda_create (evaluation, lambda, expression, registers_closure, registers_local),
			
		}
		
	}
	
	
	#[ inline (always) ]
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
	
	
	#[ inline (always) ]
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
	
	
	#[ inline (always) ]
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
	
	
	#[ inline (always) ]
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
	
	
	#[ inline (always) ]
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
	
	
	#[ inline (always) ]
	fn evaluate_for_procedure_native_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureNativeCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureNativeCall::ProcedureNativeCall (ref _procedure, ref _inputs) =>
				fail_unimplemented! (0xe933a5ac),
			ExpressionForProcedureNativeCall::ProcedureNativeCall0 (ref procedure) =>
				self.evaluate_procedure_native_0 (evaluation, procedure),
			ExpressionForProcedureNativeCall::ProcedureNativeCall1 (ref procedure, ref input_1) =>
				self.evaluate_procedure_native_1 (evaluation, procedure, input_1),
			ExpressionForProcedureNativeCall::ProcedureNativeCall2 (ref procedure, ref input_1, ref input_2) =>
				self.evaluate_procedure_native_2 (evaluation, procedure, input_1, input_2),
			ExpressionForProcedureNativeCall::ProcedureNativeCall3 (ref procedure, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_native_3 (evaluation, procedure, input_1, input_2, input_3),
			ExpressionForProcedureNativeCall::ProcedureNativeCall4 (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_native_4 (evaluation, procedure, input_1, input_2, input_3, input_4),
			ExpressionForProcedureNativeCall::ProcedureNativeCall5 (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_native_5 (evaluation, procedure, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedureNativeCall::ProcedureNativeCallN (ref procedure, ref inputs) =>
				self.evaluate_procedure_native_n (evaluation, procedure, inputs),
			
		}
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_slice (&self, evaluation : &mut EvaluatorContext, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		let mut outputs = Vec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (evaluation, input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
	
	
	
	
	#[ inline (always) ]
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
	
	
	
	
	#[ inline (always) ]
	fn evaluate_conditional_if (&self, evaluation : &mut EvaluatorContext, clauses : &ExpressionConditionalIfClauses) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_if_clauses (evaluation, clauses)) {
			succeed! (output);
		} else {
			succeed! (VOID.into ());
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_if_clauses (&self, evaluation : &mut EvaluatorContext, clauses : &ExpressionConditionalIfClauses) -> (Outcome<Option<Value>>) {
		match *clauses {
			ExpressionConditionalIfClauses::Void =>
				succeed! (None),
			ExpressionConditionalIfClauses::Single (ref clause) =>
				return self.evaluate_conditional_if_clause (evaluation, clause),
			ExpressionConditionalIfClauses::Multiple (ref clauses) => {
				for clause in clauses.iter () {
					let output = try! (self.evaluate_conditional_if_clause (evaluation, clause));
					if output.is_some () {
						succeed! (output);
					}
				}
				succeed! (None);
			},
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_if_clause (&self, evaluation : &mut EvaluatorContext, clause : &ExpressionConditionalIfClause) -> (Outcome<Option<Value>>) {
		match *clause {
			ExpressionConditionalIfClause::Void =>
				succeed! (None),
			ExpressionConditionalIfClause::GuardOnly (ref guard) =>
				return self.evaluate_conditional_if_guard (evaluation, guard),
			ExpressionConditionalIfClause::GuardAndOutput (ref guard, ref output) =>
				if try! (self.evaluate_conditional_if_guard (evaluation, guard)) .is_some () {
					let output = try! (self.evaluate (evaluation, output));
					succeed! (Some (output));
				} else {
					succeed! (None);
				},
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_if_guard (&self, evaluation : &mut EvaluatorContext, guard : &ExpressionConditionalIfGuard) -> (Outcome<Option<Value>>) {
		match *guard {
			ExpressionConditionalIfGuard::True =>
				succeed! (Some (TRUE.into ())),
			ExpressionConditionalIfGuard::False =>
				succeed! (None),
			ExpressionConditionalIfGuard::Expression (ref expression, negated) => {
				let output = try! (self.evaluate (evaluation, expression));
				let (matched, output) = if ! negated {
					(is_not_false (&output), output)
				} else {
					(is_false (&output), TRUE.into ())
				};
				if matched {
					succeed! (Some (output));
				} else {
					succeed! (None);
				}
			},
		}
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_conditional_match (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &ExpressionConditionalMatchClauses) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_match_clauses (evaluation, actual, clauses)) {
			succeed! (output);
		} else {
			succeed! (VOID.into ());
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_match_clauses (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &ExpressionConditionalMatchClauses) -> (Outcome<Option<Value>>) {
		let (actual, matched, output) = match *clauses {
			ExpressionConditionalMatchClauses::Void =>
				succeed! (None),
			ExpressionConditionalMatchClauses::Single (ref clause) => {
				let actual = try! (self.evaluate (evaluation, actual));
				let (matched, output) = try! (self.evaluate_conditional_match_clause (evaluation, &actual, clause));
				(actual, matched, output)
			},
			ExpressionConditionalMatchClauses::Multiple (ref clauses) => {
				let actual = try! (self.evaluate (evaluation, actual));
				let mut matched = false;
				let mut output = None;
				for clause in clauses.iter () {
					let (matched_1, output_1) = try! (self.evaluate_conditional_match_clause (evaluation, &actual, clause));
					matched = matched_1;
					output = output_1;
					if matched {
						break;
					}
				}
				(actual, matched, output)
			},
		};
		if matched {
			if let Some (output) = output {
				succeed! (Some (output));
			} else {
				succeed! (Some (actual));
			}
		} else {
			succeed! (None);
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_match_clause (&self, evaluation : &mut EvaluatorContext, actual : &Value, clause : &ExpressionConditionalMatchClause) -> (Outcome<(bool, Option<Value>)>) {
		match *clause {
			ExpressionConditionalMatchClause::Void =>
				succeed! ((false, None)),
			ExpressionConditionalMatchClause::GuardOnly (ref guard) =>
				if try! (self.evaluate_conditional_match_guard (evaluation, actual, guard)) {
					succeed! ((true, None));
				} else {
					succeed! ((false, None));
				},
			ExpressionConditionalMatchClause::GuardAndOutput (ref guard, ref output) =>
				if try! (self.evaluate_conditional_match_guard (evaluation, actual, guard)) {
					let output = try! (self.evaluate (evaluation, output));
					succeed! ((true, Some (output)));
				} else {
					succeed! ((false, None));
				},
		}
	}
	
	#[ inline (always) ]
	fn evaluate_conditional_match_guard (&self, _evaluation : &mut EvaluatorContext, actual : &Value, guard : &ExpressionConditionalMatchGuard) -> (Outcome<bool>) {
		let (matched, negated) = match *guard {
			ExpressionConditionalMatchGuard::True =>
				succeed! (true),
			ExpressionConditionalMatchGuard::False =>
				succeed! (false),
			ExpressionConditionalMatchGuard::Value (ref expected, negated) => {
				let matched = try! (equivalent_by_value_strict_2 (actual, expected));
				(matched, negated)
			},
			ExpressionConditionalMatchGuard::Values (ref expected, negated) => {
				let mut matched = false;
				for expected in expected.iter () {
					matched = try! (equivalent_by_value_strict_2 (actual, expected));
					if matched {
						break;
					}
				}
				(matched, negated)
			},
		};
		succeed! (matched ^ negated);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_loop (&self, evaluation : &mut EvaluatorContext, initialize : Option<&Expression>, update : Option<&Expression>, body : Option<&Expression>, clauses : &ExpressionConditionalIfClauses) -> (Outcome<Value>) {
		
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
	
	
	
	
	#[ inline (always) ]
	fn evaluate_context_define (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xfe053ac6);
		let template = BindingTemplate {
				identifier : Some (identifier.clone ()),
				value : None,
				immutable : false,
			};
		let binding = try! (context.define (&template));
		let value_new = try! (self.evaluate (evaluation, expression));
		try! (binding.initialize (value_new.clone ()));
		return Ok (value_new);
	}
	
	#[ inline (always) ]
	fn evaluate_context_update (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0x4be15062);
		let binding = try_some_2! (context.resolve (identifier), 0x8c4717b1);
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	fn evaluate_context_select (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol) -> (Outcome<Value>) {
		let context = try_some! (evaluation.context, 0xdf799bc8);
		let binding = try_some_2! (context.resolve (identifier), 0x8790e81e);
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_binding_initialize_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		try! (binding.initialize (value_new.clone ()));
		return Ok (value_new);
	}
	
	#[ inline (always) ]
	fn evaluate_binding_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(Binding, Expression)], parallel : bool) -> (Outcome<Value>) {
		let bindings = initializers.iter () .map (|&(ref binding, _)| binding) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
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
	
	#[ inline (always) ]
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
	
	#[ inline (always) ]
	fn evaluate_binding_set_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	
	#[ inline (always) ]
	fn evaluate_binding_set_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(Binding, Expression)], parallel : bool) -> (Outcome<Value>) {
		let bindings = initializers.iter () .map (|&(ref binding, _)| binding) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
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
	
	
	#[ inline (always) ]
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
	
	#[ inline (always) ]
	fn evaluate_binding_get_1 (&self, _evaluation : &mut EvaluatorContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_register_closure (&self, evaluation : &mut EvaluatorContext, expression : &Expression, borrows : &[RegisterTemplate]) -> (Outcome<Value>) {
		let registers = try! (Registers::new_and_define (borrows, &evaluation.registers));
		let mut evaluation = EvaluatorContext::new (self, evaluation.context, registers);
		return self.evaluate (&mut evaluation, expression);
	}
	
	#[ inline (always) ]
	fn evaluate_register_initialize_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		try! (evaluation.registers.initialize_value (index, value_new.clone ()));
		succeed! (value_new);
	}
	
	#[ inline (always) ]
	fn evaluate_register_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (index, value_new) in vec_zip_2 (indices, values_new) {
				try! (evaluation.registers.initialize_value (index, value_new));
			}
		} else {
			for (index, expression) in vec_zip_2 (indices, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				try! (evaluation.registers.initialize_value (index, value_new));
			}
		}
		return Ok (VOID.into ());
	}
	
	#[ inline (always) ]
	fn evaluate_register_initialize_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != indices.len () {
			fail! (0xb1dce1a7);
		}
		for (index, value_new) in indices.iter () .zip (values_new.values_ref () .iter ()) {
			try! (evaluation.registers.initialize_value (*index, value_new.clone ()));
		}
		return Ok (values_new.into ());
	}
	
	#[ inline (always) ]
	fn evaluate_register_set_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (evaluation.registers.update_value (index, value_new));
		return Ok (value_old);
	}
	
	#[ inline (always) ]
	fn evaluate_register_set_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (index, value_new) in vec_zip_2 (indices, values_new) {
				try! (evaluation.registers.update_value (index, value_new));
			}
		} else {
			for (index, expression) in vec_zip_2 (indices, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				try! (evaluation.registers.update_value (index, value_new));
			}
		}
		return Ok (VOID.into ());
	}
	
	#[ inline (always) ]
	fn evaluate_register_set_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let values_new_ = try! (self.evaluate (evaluation, expression));
		let values_new_ = try_into_values! (values_new_);
		if values_new_.values_length () != indices.len () {
			fail! (0x7257e042);
		}
		let mut values_old = StdVec::with_capacity (values_new_.values_length ());
		for (index, value_new) in indices.iter () .zip (values_new_.values_ref () .iter ()) {
			let value_old = try! (evaluation.registers.update_value (*index, value_new.clone ()));
			values_old.push (value_old);
		}
		return Ok (values_new (values_old.into_boxed_slice ()) .into ());
	}
	
	#[ inline (always) ]
	fn evaluate_register_get_1 (&self, evaluation : &mut EvaluatorContext, index : usize) -> (Outcome<Value>) {
		let value = try! (evaluation.registers.resolve_value (index));
		return Ok (value);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_lambda_create (&self, evaluation : &mut EvaluatorContext, template : &StdRc<LambdaTemplate>, expression : &StdRc<Expression>, registers_closure : &[RegisterTemplate], registers_local : &StdRc<[RegisterTemplate]>) -> (Outcome<Value>) {
		let expression = StdRc::clone (expression);
		let registers_closure = try! (Registers::new_and_define (registers_closure, &evaluation.registers));
		let registers_local = StdRc::clone (registers_local);
		let template = StdRc::clone (template);
		let lambda = Lambda::new (template, expression, registers_closure, registers_local);
		succeed! (ProcedureLambda::new (lambda) .into ());
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &inputs);
	}
	
	#[ inline (never) ]
	fn evaluate_procedure_lambda_with_values (&self, _evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[&Value]) -> (Outcome<Value>) {
		
		let lambda_arguments_positional = lambda.arguments_positional;
		let lambda_argument_rest = lambda.argument_rest;
		let lambda_registers_local = lambda.registers_local.as_ref ();
		let lambda_registers_closure = &lambda.registers_closure;
		let lambda_expression = &lambda.expression;
		
		let inputs_count = inputs.len ();
		if ! lambda_argument_rest {
			if inputs_count != lambda_arguments_positional {
				fail! (0x6c9a5289);
			}
		} else {
			if inputs_count < lambda_arguments_positional {
				fail! (0xdbd70de8);
			}
		}
		
		let mut registers = try! (Registers::new_and_define (&lambda_registers_local, lambda_registers_closure));
		
		let mut inputs_offset = 0;
		for _ in 0..lambda_arguments_positional {
			let input = inputs[inputs_offset].clone ();
			try! (registers.initialize_value (inputs_offset, input));
			inputs_offset += 1;
		}
		if lambda_argument_rest {
			let inputs = if inputs_offset < inputs_count {
				list_build_n (&inputs[inputs_offset..])
			} else {
				NULL.into ()
			};
			try! (registers.initialize_value (inputs_offset, inputs));
		}
		
		let mut evaluation = EvaluatorContext::new (self, None, registers);
		return self.evaluate (&mut evaluation, lambda_expression);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_0 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_1 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_lambda_1_with_values (evaluation, lambda, &input_1);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_1_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_2 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_lambda_2_with_values (evaluation, lambda, &input_1, &input_2);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_2_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_3 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_lambda_3_with_values (evaluation, lambda, &input_1, &input_2, &input_3);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_3_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_4 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_lambda_4_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_4_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_5 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_lambda_5_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_5_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4, input_5]);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_n (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_n_with_values (evaluation, lambda, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_lambda_n_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[&Value]) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, inputs);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_with_values (evaluation, &callable, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_call_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				return self.evaluate_procedure_primitive_with_values (evaluation, *callable.as_ref (), inputs),
			ValueClass::ProcedureExtended =>
				return self.evaluate_procedure_extended_with_values (evaluation, callable.as_ref (), inputs),
			ValueClass::ProcedureLambda =>
				return self.evaluate_procedure_lambda_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), inputs),
			_ =>
				fail! (0x88be334b),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_0 (&self, evaluation : &mut EvaluatorContext, callable : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		return self.evaluate_procedure_call_0_with_values (evaluation, &callable);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_0 (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref ()),
			_ =>
				fail! (0xc58e190e),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_1 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_call_1_with_values (evaluation, &callable, &input_1);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_1_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), input_1),
			_ =>
				fail! (0xe7f6dbfc),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_2 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_call_2_with_values (evaluation, &callable, &input_1, &input_2);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_2_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), input_1, input_2),
			_ =>
				fail! (0x856bf44d),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_3 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_call_3_with_values (evaluation, &callable, &input_1, &input_2, &input_3);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_3_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), input_1, input_2, input_3),
			_ =>
				fail! (0xdb6b9bbc),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_4 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_call_4_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_4_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), input_1, input_2, input_3, input_4),
			_ =>
				fail! (0xf0969d74),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_5 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_call_5_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_5_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), input_1, input_2, input_3, input_4, input_5),
			_ =>
				fail! (0x62e8aef5),
		}
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_call_n (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_n_with_values (evaluation, &callable, &inputs);
	}
	
	#[ inline (always) ]
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
				return self.evaluate_procedure_lambda_n_with_values (evaluation, StdAsRef::<ProcedureLambda>::as_ref (callable) .internals_ref (), inputs),
			_ =>
				fail! (0x906ebf03),
		}
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_with_values (evaluation, primitive, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_0 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		return procedure_primitive_0_evaluate (primitive, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_1 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, &input_1);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_1_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_2 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, &input_1, &input_2);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_2_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_3 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, &input_1, &input_2, &input_3);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_3_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_4 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_4_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_5 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_5_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_n_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_v (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_v_with_values (evaluation, primitive, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_primitive_v_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_primitive_v_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_with_values (evaluation, extended, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_0 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended) -> (Outcome<Value>) {
		return procedure_extended_evaluate_0 (extended, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_1 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_extended_1_with_values (evaluation, extended, &input_1);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_1_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_1 (extended, input_1, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_2 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_extended_2_with_values (evaluation, extended, &input_1, &input_2);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_2_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_2 (extended, input_1, input_2, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_3 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_extended_3_with_values (evaluation, extended, &input_1, &input_2, &input_3);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_3_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_3 (extended, input_1, input_2, input_3, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_4 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_extended_4_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_4_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_4 (extended, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_5 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_extended_5_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_5_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_5 (extended, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_n (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_n_with_values (evaluation, extended, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_extended_n_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[&Value]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_0 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative0) -> (Outcome<Value>) {
		return native (evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_1 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_native_1_with_values (evaluation, native, &input_1);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_1_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative1, input_1 : &Value) -> (Outcome<Value>) {
		return native (input_1, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_2 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_native_2_with_values (evaluation, native, &input_1, &input_2);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_2_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return native (input_1, input_2, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_3 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_native_3_with_values (evaluation, native, &input_1, &input_2, &input_3);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_3_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return native (input_1, input_2, input_3, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_4 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_native_4_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_4_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return native (input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_5 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_native_5_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_5_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return native (input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	#[ inline (always) ]
	fn evaluate_procedure_native_n (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_native_n_with_values (evaluation, native, &inputs);
	}
	
	#[ inline (always) ]
	fn evaluate_procedure_native_n_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeN, inputs : &[&Value]) -> (Outcome<Value>) {
		return native (inputs, evaluation);
	}
	
	
	
	
}




#[ derive (Debug) ]
pub struct EvaluatorContext <'a> {
	evaluator : &'a Evaluator,
	context : Option<&'a Context>,
	registers : Registers,
}


impl <'a> EvaluatorContext<'a> {
	
	
	#[ inline (always) ]
	fn new (evaluator : &'a Evaluator, context : Option<&'a Context>, registers : Registers) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : evaluator,
				context : context,
				registers : registers,
			}
	}
	
	
	#[ inline (always) ]
	pub fn evaluate (&mut self, input : &Expression) -> (Outcome<Value>) {
		return self.evaluator.evaluate (self, input);
	}
	
	#[ inline (always) ]
	pub fn evaluate_slice (&mut self, inputs : &[Expression]) -> (Outcome<Vec<Value>>) {
		return self.evaluator.evaluate_slice (self, inputs);
	}
	
	#[ inline (always) ]
	pub fn evaluate_script <Iterator, ExpressionRef> (&mut self, inputs : Iterator) -> (Outcome<()>)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : AsRef<Expression>
	{
		for input in inputs {
			try! (self.evaluate (input.as_ref ()));
		}
		return Ok (());
	}
	
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_0 (&mut self, callable : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_0_with_values (self, callable);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_1 (&mut self, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_1_with_values (self, callable, input_1);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_2 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_2_with_values (self, callable, input_1, input_2);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_3 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_3_with_values (self, callable, input_1, input_2, input_3);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_4 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_4_with_values (self, callable, input_1, input_2, input_3, input_4);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_5 (&mut self, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_5_with_values (self, callable, input_1, input_2, input_3, input_4, input_5);
	}
	
	#[ inline (never) ]
	pub fn evaluate_procedure_call_n (&mut self, callable : &Value, inputs : &[&Value]) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_n_with_values (self, callable, inputs);
	}
}

