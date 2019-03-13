

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
use super::lambdas::exports::*;

#[ cfg ( feature = "vonuvoli_values_extended" ) ]
use super::extended_procedures::exports::*;

#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
use super::parameters::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::evaluate;
	pub use super::evaluate_script;
	
	pub use super::Evaluator;
	pub use super::EvaluatorConfiguration;
	pub use super::EvaluatorContext;
	
}




#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




#[ cfg ( not ( feature = "vonuvoli_builtins_parameters" ) ) ]
type Parameters = !;




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn evaluate (expression : &Expression, context : Option<&Context>, parameters : Option<&Parameters>, configuration : Option<&EvaluatorConfiguration>) -> (Outcome<Value>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	let evaluator = Evaluator::new (configuration);
	let context = context.cloned ();
	let parameters = parameters.cloned ();
	let mut evaluation = evaluator.fork (context, parameters);
	return evaluation.evaluate (expression);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn evaluate_script <Iterator, ExpressionRef> (expressions : Iterator, context : Option<&Context>, parameters : Option<&Parameters>, configuration : Option<&EvaluatorConfiguration>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
{
	let configuration = configuration.cloned () .unwrap_or_default ();
	let evaluator = Evaluator::new (configuration);
	let context = context.cloned ();
	let parameters = parameters.cloned ();
	let mut evaluation = evaluator.fork (context, parameters);
	return evaluation.evaluate_script (expressions);
}




pub struct Evaluator {
	configuration : EvaluatorConfiguration,
}


impl Evaluator {
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (configuration : EvaluatorConfiguration) -> (Evaluator) {
		return Evaluator {
				configuration : configuration,
			};
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork (&self, context : Option<Context>, parameters : Option<Parameters>) -> (EvaluatorContext) {
		return EvaluatorContext::new (self, context, parameters);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork_0 (&self) -> (EvaluatorContext) {
		return EvaluatorContext::new (self, None, None);
	}
	
	
	
	
	#[ inline (never) ]
	fn evaluate (&self, evaluation : &mut EvaluatorContext, expression : &Expression) -> (Outcome<Value>) {
		
		#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		{ if self.configuration.is_trace_enabled () {
			
			if self.configuration.should_trace_input () {
				trace_debugging! (transcript, 0xc9ab7675 => "evaluating:\u{1e}{:#?}" => (expression));
			}
			
			let outcome = self.evaluate_00 (evaluation, expression);
			
			match outcome {
				Ok (ref output) if self.configuration.should_trace_output () =>
					trace_debugging! (transcript, 0x3a69ec68 => "evaluating succeeded:\u{1e}{:#?}\u{1e}{:#?}" => (expression, output)),
				Ok (_) =>
					(),
				Err (ref error) if (self.configuration.should_trace_output () || (self.configuration.should_trace_error () && evaluation.should_trace_error ())) && error.is_traceable () && !error.was_reported () =>
					trace_error! (transcript, 0xde839a96 => "evaluating failed:\u{1e}{:#?}" => (expression), error = error),
				Err (_) =>
					(),
			}
			
			return outcome;
			
		} else {
			
			return self.evaluate_00 (evaluation, expression);
		} }
		
		#[ cfg ( not ( all ( feature = "vonuvoli_evaluator_trace_enabled", feature = "vonuvoli_transcript" ) ) ) ]
		return self.evaluate_00 (evaluation, expression);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline (always) ) ]
	fn evaluate_00 (&self, evaluation : &mut EvaluatorContext, input : &Expression) -> (Outcome<Value>) {
		match *input {
			
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			Expression::Void =>
				Ok (VOID.into ()),
			Expression::Value (ref value) =>
				Ok (value.clone ()),
			
			Expression::Sequence (operator, ref expressions) =>
				self.evaluate_sequence (evaluation, operator, expressions),
			Expression::ConditionalIf (ref clauses) =>
				self.evaluate_conditional_if (evaluation, clauses),
			#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
			Expression::ConditionalMatch (ref actual, ref clauses) =>
				self.evaluate_conditional_match (evaluation, actual, clauses),
			Expression::Loop (ref initialize, ref update, ref body, ref clauses) =>
				self.evaluate_loop (evaluation, option_box_as_ref (initialize), option_box_as_ref (update), option_box_as_ref (body), clauses.as_ref ()),
			
			Expression::Contexts (ref expression) =>
				self.evaluate_for_contexts (evaluation, expression),
			
			Expression::ProcedureGenericCall (ref expression) =>
				self.evaluate_for_procedure_generic_call (evaluation, expression),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			Expression::ProcedurePrimitiveCall (ref expression) =>
				self.evaluate_for_procedure_primitive_call (evaluation, expression),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			Expression::ProcedureExtendedCall (ref expression) =>
				self.evaluate_for_procedure_extended_call (evaluation, expression),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			Expression::ProcedureNativeCall (ref expression) =>
				self.evaluate_for_procedure_native_call (evaluation, expression),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Expression::ProcedureLambdaCall (ref expression) =>
				self.evaluate_for_procedure_lambda_call (evaluation, expression),
			
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			Expression::Lambda (ref lambda, ref expression, ref registers_closure, ref registers_local) =>
				self.evaluate_lambda_create (evaluation, lambda, expression, registers_closure, registers_local),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Expression::ErrorReturn (ref expression) =>
				self.evaluate_error_return (evaluation, expression),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Expression::ErrorCatch (ref expression, ref error_consumer, ref error_expression) =>
				self.evaluate_error_catch (evaluation, expression, error_consumer, error_expression),
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			Expression::ErrorThrow (ref expression) =>
				self.evaluate_error_throw (evaluation, expression),
			
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ExpressionForContexts::BindingInitializeValues (ref bindings, ref expression) =>
				self.evaluate_binding_initialize_values (evaluation, bindings, expression),
			ExpressionForContexts::BindingSet1 (ref binding, ref expression) =>
				self.evaluate_binding_set_1 (evaluation, binding, expression),
			ExpressionForContexts::BindingSetN (ref initializers, parallel) =>
				self.evaluate_binding_set_n (evaluation, initializers, parallel),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
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
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ExpressionForContexts::RegisterInitializeValues (ref indices, ref expression) =>
				self.evaluate_register_initialize_values (evaluation, indices, expression),
			ExpressionForContexts::RegisterSet1 (index, ref expression) =>
				self.evaluate_register_set_1 (evaluation, index, expression),
			ExpressionForContexts::RegisterSetN (ref initializers, parallel) =>
				self.evaluate_register_set_n (evaluation, initializers, parallel),
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ExpressionForContexts::RegisterSetValues (ref indices, ref expression) =>
				self.evaluate_register_set_values (evaluation, indices, expression),
			ExpressionForContexts::RegisterGet1 (index) =>
				self.evaluate_register_get_1 (evaluation, index),
			
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ExpressionForContexts::ParameterClosure (ref expression) =>
				self.evaluate_parameter_closure (evaluation, expression),
			
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_for_procedure_generic_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureGenericCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureGenericCall::ProcedureCall (ref callable, ref inputs) =>
				self.evaluate_procedure_call (evaluation, callable, inputs),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall0 (ref callable) =>
				self.evaluate_procedure_call_0 (evaluation, callable),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall1 (ref callable, ref input_1) =>
				self.evaluate_procedure_call_1 (evaluation, callable, input_1),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall2 (ref callable, ref input_1, ref input_2) =>
				self.evaluate_procedure_call_2 (evaluation, callable, input_1, input_2),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall3 (ref callable, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_call_3 (evaluation, callable, input_1, input_2, input_3),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall4 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_call_4 (evaluation, callable, input_1, input_2, input_3, input_4),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCall5 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_call_5 (evaluation, callable, input_1, input_2, input_3, input_4, input_5),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionForProcedureGenericCall::ProcedureCallN (ref callable, ref inputs) =>
				self.evaluate_procedure_call_n (evaluation, callable, inputs),
			
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	
	#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_for_procedure_native_call (&self, evaluation : &mut EvaluatorContext, input : &ExpressionForProcedureNativeCall) -> (Outcome<Value>) {
		match *input {
			
			ExpressionForProcedureNativeCall::ProcedureNativeCall (ref procedure, ref inputs) =>
				self.evaluate_procedure_native (evaluation, procedure.internals_ref (), inputs),
			
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
			
			ExpressionForProcedureNativeCall::ProcedureNativeCall0E (ref procedure) =>
				self.evaluate_procedure_native_0e (evaluation, procedure),
			ExpressionForProcedureNativeCall::ProcedureNativeCall1E (ref procedure, ref input_1) =>
				self.evaluate_procedure_native_1e (evaluation, procedure, input_1),
			ExpressionForProcedureNativeCall::ProcedureNativeCall2E (ref procedure, ref input_1, ref input_2) =>
				self.evaluate_procedure_native_2e (evaluation, procedure, input_1, input_2),
			ExpressionForProcedureNativeCall::ProcedureNativeCall3E (ref procedure, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_native_3e (evaluation, procedure, input_1, input_2, input_3),
			ExpressionForProcedureNativeCall::ProcedureNativeCall4E (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_native_4e (evaluation, procedure, input_1, input_2, input_3, input_4),
			ExpressionForProcedureNativeCall::ProcedureNativeCall5E (ref procedure, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_native_5e (evaluation, procedure, input_1, input_2, input_3, input_4, input_5),
			ExpressionForProcedureNativeCall::ProcedureNativeCallNE (ref procedure, ref inputs) =>
				self.evaluate_procedure_native_ne (evaluation, procedure, inputs),
			
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_slice (&self, evaluation : &mut EvaluatorContext, inputs : &[Expression]) -> (Outcome<StdVec<Value>>) {
		let mut outputs = StdVec::with_capacity (inputs.len ());
		for input in inputs {
			let output = try! (self.evaluate (evaluation, input));
			outputs.push (output);
		}
		return Ok (outputs);
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_conditional_if (&self, evaluation : &mut EvaluatorContext, clauses : &ExpressionConditionalIfClauses) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_if_clauses (evaluation, clauses)) {
			if let Some (output) = output {
				succeed! (output);
			} else {
				succeed! (VOID.into ());
			}
		} else {
			succeed! (VOID.into ());
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
	fn evaluate_conditional_if_clauses (&self, evaluation : &mut EvaluatorContext, clauses : &ExpressionConditionalIfClauses) -> (Outcome<Option<Option<Value>>>) {
		match *clauses {
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClauses::Void =>
				succeed! (None),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClauses::TrueReturn =>
				succeed! (Some (Some (TRUE_VALUE))),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClauses::ExpressionOnly (ref expression) => {
				let value = try! (self.evaluate (evaluation, expression));
				succeed! (Some (Some (value)));
			},
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
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
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
	fn evaluate_conditional_if_clause (&self, evaluation : &mut EvaluatorContext, clause : &ExpressionConditionalIfClause) -> (Outcome<Option<Option<Value>>>) {
		match *clause {
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClause::Void =>
				succeed! (None),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClause::TrueReturn =>
				succeed! (Some (Some (TRUE_VALUE))),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfClause::ExpressionOnly (ref expression) => {
				let value = try! (self.evaluate (evaluation, expression));
				succeed! (Some (Some (value)));
			},
			ExpressionConditionalIfClause::GuardOnly (ref guard, ref guard_consumer) =>
				return self.evaluate_conditional_if_guard (evaluation, guard, guard_consumer),
			ExpressionConditionalIfClause::GuardAndExpression (ref guard, ref guard_consumer, ref output) =>
				if try! (self.evaluate_conditional_if_guard (evaluation, guard, guard_consumer)) .is_some () {
					let output = try! (self.evaluate (evaluation, output));
					succeed! (Some (Some (output)));
				} else {
					succeed! (None);
				},
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
	fn evaluate_conditional_if_guard (&self, evaluation : &mut EvaluatorContext, guard : &ExpressionConditionalIfGuard, guard_consumer : &ExpressionValueConsumer) -> (Outcome<Option<Option<Value>>>) {
		match *guard {
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfGuard::True =>
				succeed! (Some (try! (self.evaluate_value_consumer (evaluation, TRUE.into (), guard_consumer)))),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfGuard::False =>
				succeed! (None),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalIfGuard::Value (ref value, negated) => {
				let output = value.clone ();
				let (matched, output) = if ! negated {
					(is_not_false (&output), output)
				} else {
					(is_false (&output), TRUE.into ())
				};
				if matched {
					succeed! (Some (try! (self.evaluate_value_consumer (evaluation, output, guard_consumer))));
				} else {
					succeed! (None);
				}
			},
			ExpressionConditionalIfGuard::Expression (ref expression, negated) => {
				let output = try! (self.evaluate (evaluation, expression));
				let (matched, output) = if ! negated {
					(is_not_false (&output), output)
				} else {
					(is_false (&output), TRUE.into ())
				};
				if matched {
					succeed! (Some (try! (self.evaluate_value_consumer (evaluation, output, guard_consumer))));
				} else {
					succeed! (None);
				}
			},
		}
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_conditional_match (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &ExpressionConditionalMatchClauses) -> (Outcome<Value>) {
		if let Some (output) = try! (self.evaluate_conditional_match_clauses (evaluation, actual, clauses)) {
			if let Some (output) = output {
				succeed! (output);
			} else {
				succeed! (VOID.into ());
			}
		} else {
			succeed! (VOID.into ());
		}
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::option_option) ) ]
	fn evaluate_conditional_match_clauses (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &ExpressionConditionalMatchClauses) -> (Outcome<Option<Option<Value>>>) {
		match *clauses {
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClauses::Void =>
				succeed! (None),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClauses::TrueReturn => {
				let actual = try! (self.evaluate (evaluation, actual));
				succeed! (Some (Some (actual)));
			},
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClauses::ExpressionOnly (ref expression) => {
				let value = try! (self.evaluate (evaluation, expression));
				succeed! (Some (Some (value)));
			},
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClauses::Single (ref clause) => {
				let actual = try! (self.evaluate (evaluation, actual));
				match try! (self.evaluate_conditional_match_clause (evaluation, actual, clause)) {
					Alternative2::Variant1 (output) =>
						succeed! (Some (output)),
					Alternative2::Variant2 (_) =>
						succeed! (None),
				}
			},
			ExpressionConditionalMatchClauses::Multiple (ref clauses) => {
				let mut actual = try! (self.evaluate (evaluation, actual));
				for clause in clauses.iter () {
					match try! (self.evaluate_conditional_match_clause (evaluation, actual, clause)) {
						Alternative2::Variant1 (output) =>
							succeed! (Some (output)),
						Alternative2::Variant2 (actual_1) =>
							actual = actual_1,
					}
				}
				succeed! (None);
			},
		}
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_conditional_match_clause (&self, evaluation : &mut EvaluatorContext, actual : Value, clause : &ExpressionConditionalMatchClause) -> (Outcome<Alternative2<Option<Value>, Value>>) {
		match *clause {
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClause::Void =>
				succeed! (Alternative2::Variant2 (actual)),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClause::TrueReturn =>
				succeed! (Alternative2::Variant1 (Some (actual))),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchClause::ExpressionOnly (ref expression) => {
				let value = try! (self.evaluate (evaluation, expression));
				succeed! (Alternative2::Variant1 (Some (value)));
			},
			ExpressionConditionalMatchClause::GuardOnly (ref guard, ref guard_consumer) =>
				return self.evaluate_conditional_match_guard (evaluation, actual, guard, guard_consumer),
			ExpressionConditionalMatchClause::GuardAndExpression (ref guard, ref guard_consumer, ref output) =>
				match try! (self.evaluate_conditional_match_guard (evaluation, actual, guard, guard_consumer)) {
					Alternative2::Variant1 (_) => {
						let output = try! (self.evaluate (evaluation, output));
						succeed! (Alternative2::Variant1 (Some (output)));
					},
					outcome @ Alternative2::Variant2 (_) =>
						succeed! (outcome),
				},
		}
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_conditional_match_guard (&self, evaluation : &mut EvaluatorContext, actual : Value, guard : &ExpressionConditionalMatchGuard, guard_consumer : &ExpressionValueConsumer) -> (Outcome<Alternative2<Option<Value>, Value>>) {
		let (matched, negated) = match *guard {
			ExpressionConditionalMatchGuard::True =>
				(true, false),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchGuard::False =>
				succeed! (Alternative2::Variant2 (actual)),
			#[ cfg ( feature = "vonuvoli_expressions_optimizer" ) ]
			ExpressionConditionalMatchGuard::Value (ref expected, negated) => {
				let matched = try! (equivalent_by_value_strict_2 (&actual, expected, false));
				(matched, negated)
			},
			ExpressionConditionalMatchGuard::Values (ref expected, negated) => {
				let mut matched = false;
				for expected in expected.iter () {
					matched = try! (equivalent_by_value_strict_2 (&actual, expected, false));
					if matched {
						break;
					}
				}
				(matched, negated)
			},
		};
		if matched ^ negated {
			succeed! (Alternative2::Variant1 (try! (self.evaluate_value_consumer (evaluation, actual, guard_consumer))));
		} else {
			succeed! (Alternative2::Variant2 (actual));
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_value_consumer (&self, evaluation : &mut EvaluatorContext, value : Value, consumer : &ExpressionValueConsumer) -> (Outcome<Option<Value>>) {
		match *consumer {
			ExpressionValueConsumer::Ignore =>
				succeed! (None),
			ExpressionValueConsumer::Return =>
				succeed! (Some (value)),
			ExpressionValueConsumer::BindingInitialize (ref binding) => {
				try! (binding.initialize (value));
				succeed! (None);
			},
			ExpressionValueConsumer::BindingSet (ref binding) => {
				try! (binding.set (value));
				succeed! (None);
			},
			ExpressionValueConsumer::RegisterInitialize (index) => {
				let registers = try_some_ref! (evaluation.registers, 0xa6038927);
				try! (registers.initialize_value (index, value));
				succeed! (None);
			},
			ExpressionValueConsumer::RegisterSet (index) => {
				let registers = try_some_ref! (evaluation.registers, 0xa147d2fc);
				try! (registers.update_value (index, value));
				succeed! (None);
			},
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_loop (&self, evaluation : &mut EvaluatorContext, initialize : Option<&Expression>, update : Option<&Expression>, body : Option<&Expression>, clauses : Option<&ExpressionConditionalIfClauses>) -> (Outcome<Value>) {
		
		if let Some (initialize) = initialize {
			try! (self.evaluate (evaluation, initialize));
		}
		
		loop {
			
			if let Some (clauses) = clauses {
				if let Some (output) = try! (self.evaluate_conditional_if_clauses (evaluation, clauses)) {
					if let Some (output) = output {
						succeed! (output);
					} else {
						succeed! (VOID.into ());
					}
				}
			}
			
			if let Some (body) = body {
				try! (self.evaluate (evaluation, body));
			}
			
			if let Some (update) = update {
				try! (self.evaluate (evaluation, update));
			}
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_context_define (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let context = try_some_ref! (evaluation.context, 0xfe053ac6);
		let template = BindingTemplate {
				identifier : Some (identifier.clone ()),
				value : None,
				immutable : false,
			};
		let binding = try! (context.define (&template));
		try! (binding.initialize (value_new.clone ()));
		return Ok (value_new);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_context_update (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let context = try_some_ref! (evaluation.context, 0x4be15062);
		let binding = try_some_2! (context.resolve (identifier), 0x8c4717b1);
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_context_select (&self, evaluation : &mut EvaluatorContext, identifier : &Symbol) -> (Outcome<Value>) {
		let context = try_some_ref! (evaluation.context, 0xdf799bc8);
		let binding = try_some_2! (context.resolve (identifier), 0x8790e81e);
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_binding_initialize_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		try! (binding.initialize (value_new.clone ()));
		return Ok (value_new);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_binding_initialize_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != bindings.len () {
			fail! (0x34cd5a9a);
		}
		for (binding, value_new) in bindings.iter () .zip (values_new.values_ref () .iter ()) {
			try! (binding.initialize (value_new.clone ()));
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_binding_set_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
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
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_binding_set_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != bindings.len () {
			fail! (0xd47ae677);
		}
		for (binding, value_new) in bindings.iter () .zip (values_new.values_ref () .iter ()) {
			try! (binding.set (value_new.clone ()));
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_binding_get_1 (&self, _evaluation : &mut EvaluatorContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_closure (&self, evaluation : &mut EvaluatorContext, expression : &Expression, borrows : &[RegisterTemplate]) -> (Outcome<Value>) {
		let registers = try! (Registers::new_and_define (borrows, evaluation.registers.as_ref ()));
		let mut evaluation = evaluation.fork_with_registers (registers);
		return self.evaluate (&mut evaluation, expression);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_initialize_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let registers = try_some_ref! (evaluation.registers, 0x2ed416ec);
		try! (registers.initialize_value (index, value_new.clone ()));
		succeed! (value_new);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (index, value_new) in vec_zip_2 (indices, values_new) {
				let registers = try_some_ref! (evaluation.registers, 0xa488be50);
				try! (registers.initialize_value (index, value_new));
			}
		} else {
			for (index, expression) in vec_zip_2 (indices, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				let registers = try_some_ref! (evaluation.registers, 0x1ba75f00);
				try! (registers.initialize_value (index, value_new));
			}
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_initialize_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != indices.len () {
			fail! (0xb1dce1a7);
		}
		let registers = try_some_ref! (evaluation.registers, 0x018c6632);
		for (index, value_new) in indices.iter () .zip (values_new.values_ref () .iter ()) {
			try! (registers.initialize_value (*index, value_new.clone ()));
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_set_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (self.evaluate (evaluation, expression));
		let registers = try_some_ref! (evaluation.registers, 0x01a2c7be);
		let value_old = try! (registers.update_value (index, value_new));
		return Ok (value_old);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_set_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		if parallel {
			let values_new = try_vec_map_into! (expressions, expression, self.evaluate (evaluation, expression));
			for (index, value_new) in vec_zip_2 (indices, values_new) {
				let registers = try_some_ref! (evaluation.registers, 0x4467b069);
				try! (registers.update_value (index, value_new));
			}
		} else {
			for (index, expression) in vec_zip_2 (indices, expressions) {
				let value_new = try! (self.evaluate (evaluation, expression));
				let registers = try_some_ref! (evaluation.registers, 0x3d46b523);
				try! (registers.update_value (index, value_new));
			}
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_set_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (self.evaluate (evaluation, expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != indices.len () {
			fail! (0x7257e042);
		}
		let registers = try_some_ref! (evaluation.registers, 0x159bc8d2);
		for (index, value_new) in indices.iter () .zip (values_new.values_ref () .iter ()) {
			try! (registers.update_value (*index, value_new.clone ()));
		}
		return Ok (VOID.into ());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_register_get_1 (&self, evaluation : &mut EvaluatorContext, index : usize) -> (Outcome<Value>) {
		let registers = try_some_ref! (evaluation.registers, 0x153a6512);
		let value = try! (registers.resolve_value (index));
		return Ok (value);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_parameter_closure (&self, evaluation : &mut EvaluatorContext, expression : &Expression) -> (Outcome<Value>) {
		let mut evaluation = try! (evaluation.fork_parameters ());
		return self.evaluate (&mut evaluation, expression);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_error_return (&self, evaluation : &mut EvaluatorContext, expression : &Expression) -> (Outcome<Value>) {
		#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		evaluation.disable_trace_error ();
		let outcome = self.evaluate (evaluation, expression);
		#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		evaluation.enable_trace_error ();
		match outcome {
			Ok (value) =>
				succeed! (value),
			Err (error) =>
				succeed! (error.into_value ()),
		}
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_error_catch (&self, evaluation : &mut EvaluatorContext, expression : &Expression, error_consumer : &ExpressionValueConsumer, error_expression : &Expression) -> (Outcome<Value>) {
		#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		evaluation.disable_trace_error ();
		let outcome = self.evaluate (evaluation, expression);
		#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		evaluation.enable_trace_error ();
		match outcome {
			Ok (value) =>
				succeed! (value),
			Err (error) =>
				if error.is_interceptable () {
					let error = error.into_value ();
					try! (self.evaluate_value_consumer (evaluation, error, error_consumer));
					return self.evaluate (evaluation, error_expression);
				} else {
					return Err (error);
				},
		}
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_error_throw (&self, evaluation : &mut EvaluatorContext, expression : &Expression) -> (Outcome<Value>) {
		let value = try! (self.evaluate (evaluation, expression));
		let error = error_coerce_from (None, value);
		return Err (error);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_lambda_create (&self, evaluation : &mut EvaluatorContext, template : &LambdaTemplate, expression : &StdRc<Expression>, registers_closure : &[RegisterTemplate], registers_local : &StdRc<[RegisterTemplate]>) -> (Outcome<Value>) {
		let expression = StdRc::clone (expression);
		let registers_closure = try! (Registers::new_and_define (registers_closure, evaluation.registers.as_ref ()));
		let registers_local = StdRc::clone (registers_local);
		let lambda = Lambda::new (template, expression, registers_closure, registers_local);
		succeed! (ProcedureLambda::new (lambda) .into ());
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ inline (never) ]
	fn evaluate_procedure_lambda_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		
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
		
		let registers = try! (Registers::new_and_define (lambda_registers_local, Some (lambda_registers_closure)));
		
		let mut inputs_offset = 0;
		for _ in 0..lambda_arguments_positional {
			let input = inputs[inputs_offset].as_ref () .clone ();
			try! (registers.initialize_value (inputs_offset, input));
			inputs_offset += 1;
		}
		if lambda_argument_rest {
			let inputs = if inputs_offset < inputs_count {
				list_build_n (&inputs[inputs_offset..], None, None)
			} else {
				NULL.into ()
			};
			try! (registers.initialize_value (inputs_offset, inputs));
		}
		
		let mut evaluation = evaluation.fork_with_registers (registers);
		return self.evaluate (&mut evaluation, lambda_expression);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_0 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals) -> (Outcome<Value>) {
		const INPUTS_EMPTY : &[&Value] = &[];
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, INPUTS_EMPTY);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_1 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_lambda_1_with_values (evaluation, lambda, &input_1);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_1_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1]);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_2 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_lambda_2_with_values (evaluation, lambda, &input_1, &input_2);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_2_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2]);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_3 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_lambda_3_with_values (evaluation, lambda, &input_1, &input_2, &input_3);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_3_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3]);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_4 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_lambda_4_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_4_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4]);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_5 (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_lambda_5_with_values (evaluation, lambda, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_5_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, &[input_1, input_2, input_3, input_4, input_5]);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_n (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_lambda_n_with_values (evaluation, lambda, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_lambda_n_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaInternals, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return self.evaluate_procedure_lambda_with_values (evaluation, lambda, inputs);
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_with_values (evaluation, &callable, &inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				return self.evaluate_procedure_primitive_with_values (evaluation, *callable, inputs),
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_with_values (evaluation, callable, inputs),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_with_values (evaluation, callable.internals_ref (), inputs),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_with_values (evaluation, callable.internals_ref (), inputs),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (parameter) =>
				if inputs.is_empty () {
					return evaluation.parameter_resolve (parameter, None);
				} else {
					fail! (0xf57f4952);
				},
			_ =>
				fail! (0x88be334b),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_0 (&self, evaluation : &mut EvaluatorContext, callable : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		return self.evaluate_procedure_call_0_with_values (evaluation, &callable);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_0_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive0 (primitive) =>
						return self.evaluate_procedure_primitive_0 (evaluation, primitive),
					primitive =>
						return self.evaluate_procedure_primitive_0_g (evaluation, primitive),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_0 (evaluation, callable),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_0_g (evaluation, callable.internals_ref ()),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_0 (evaluation, callable.internals_ref ()),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (parameter) =>
				return evaluation.parameter_resolve (parameter, None),
			_ =>
				fail! (0xc58e190e),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_1 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_call_1_with_values (evaluation, &callable, &input_1);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_1_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive1 (primitive) =>
						return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, input_1),
					primitive =>
						return self.evaluate_procedure_primitive_1_g_with_values (evaluation, primitive, input_1),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_1_with_values (evaluation, callable, input_1),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_1_g_with_values (evaluation, callable.internals_ref (), input_1),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_1_with_values (evaluation, callable.internals_ref (), input_1),
			_ =>
				fail! (0xe7f6dbfc),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_2 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_call_2_with_values (evaluation, &callable, &input_1, &input_2);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_2_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive2 (primitive) =>
						return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, input_1, input_2),
					primitive =>
						return self.evaluate_procedure_primitive_2_g_with_values (evaluation, primitive, input_1, input_2),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_2_with_values (evaluation, callable, input_1, input_2),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_2_g_with_values (evaluation, callable.internals_ref (), input_1, input_2),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_2_with_values (evaluation, callable.internals_ref (), input_1, input_2),
			_ =>
				fail! (0x856bf44d),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_3 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_call_3_with_values (evaluation, &callable, &input_1, &input_2, &input_3);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_3_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive3 (primitive) =>
						return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, input_1, input_2, input_3),
					primitive =>
						return self.evaluate_procedure_primitive_3_g_with_values (evaluation, primitive, input_1, input_2, input_3),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_3_with_values (evaluation, callable, input_1, input_2, input_3),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_3_g_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_3_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3),
			_ =>
				fail! (0xdb6b9bbc),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_4 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_call_4_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_4_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive4 (primitive) =>
						return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, input_1, input_2, input_3, input_4),
					primitive =>
						return self.evaluate_procedure_primitive_4_g_with_values (evaluation, primitive, input_1, input_2, input_3, input_4),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_4_with_values (evaluation, callable, input_1, input_2, input_3, input_4),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_4_g_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3, input_4),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_4_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3, input_4),
			_ =>
				fail! (0xf0969d74),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_5 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_call_5_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_5_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::Primitive5 (primitive) =>
						return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
					primitive =>
						return self.evaluate_procedure_primitive_5_g_with_values (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_5_with_values (evaluation, callable, input_1, input_2, input_3, input_4, input_5),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_5_g_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3, input_4, input_5),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_5_with_values (evaluation, callable.internals_ref (), input_1, input_2, input_3, input_4, input_5),
			_ =>
				fail! (0x62e8aef5),
		}
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_n (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (self.evaluate (evaluation, callable));
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_call_n_with_values (evaluation, &callable, &inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_call_n_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		match callable.kind_match_as_ref () {
			ValueKindMatchAsRef::ProcedurePrimitive (callable) =>
				match *callable {
					ProcedurePrimitive::PrimitiveN (primitive) =>
						return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, inputs),
					primitive =>
						return self.evaluate_procedure_primitive_n_g_with_values (evaluation, primitive, inputs),
				},
			#[ cfg ( feature = "vonuvoli_values_extended" ) ]
			ValueKindMatchAsRef::ProcedureExtended (callable) =>
				return self.evaluate_procedure_extended_n_with_values (evaluation, callable, inputs),
			#[ cfg ( feature = "vonuvoli_values_native" ) ]
			ValueKindMatchAsRef::ProcedureNative (callable) =>
				return self.evaluate_procedure_native_n_g_with_values (evaluation, callable.internals_ref (), inputs),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchAsRef::ProcedureLambda (callable) =>
				return self.evaluate_procedure_lambda_n_with_values (evaluation, callable.internals_ref (), inputs),
			#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
			ValueKindMatchAsRef::Parameter (parameter) =>
				if inputs.is_empty () {
					return evaluation.parameter_resolve (parameter, None);
				} else {
					fail! (0xe74157e3);
				},
			_ =>
				fail! (0x906ebf03),
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_with_values (evaluation, primitive, &inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_0 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive0) -> (Outcome<Value>) {
		return procedure_primitive_0_evaluate (primitive, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_0_g (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_0 (primitive, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_1 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, &input_1);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_1_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_1_evaluate (primitive, input_1, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_1_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_1 (primitive, input_1, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_2 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, &input_1, &input_2);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_2_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_2_evaluate (primitive, input_1, input_2, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_2_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_2 (primitive, input_1, input_2, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_3 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, &input_1, &input_2, &input_3);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_3_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_3_evaluate (primitive, input_1, input_2, input_3, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_3_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_3 (primitive, input_1, input_2, input_3, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_4 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_4_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_4_evaluate (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_4_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_4 (primitive, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_5 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_5_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_5_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_5 (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, &inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_n_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluation);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_n_g_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_primitive_g_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_v (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_primitive_v_with_values (evaluation, primitive, &inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_primitive_v_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveV, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_primitive_v_evaluate_n (primitive, inputs, evaluation);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_with_values (evaluation, extended, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_0 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended) -> (Outcome<Value>) {
		return procedure_extended_evaluate_0 (extended, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_1 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_extended_1_with_values (evaluation, extended, &input_1);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_1_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_1 (extended, input_1, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_2 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_extended_2_with_values (evaluation, extended, &input_1, &input_2);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_2_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_2 (extended, input_1, input_2, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_3 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_extended_3_with_values (evaluation, extended, &input_1, &input_2, &input_3);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_3_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_3 (extended, input_1, input_2, input_3, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_4 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_extended_4_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_4_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_4 (extended, input_1, input_2, input_3, input_4, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_5 (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_extended_5_with_values (evaluation, extended, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_5_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_extended_evaluate_5 (extended, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_n (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_extended_n_with_values (evaluation, extended, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_extended" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_extended_n_with_values (&self, evaluation : &mut EvaluatorContext, extended : &ProcedureExtended, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return procedure_extended_evaluate_n (extended, inputs, evaluation);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_native_with_values (evaluation, native, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		let inputs_count = inputs.len ();
		match *native {
			
			ProcedureNativeInternals::Native0 (ref native) =>
				if inputs_count == 0 {
					return native.0 ();
				} else {
					fail! (0xd77d88db);
				},
			ProcedureNativeInternals::Native0E (ref native) =>
				if inputs_count == 0 {
					return native.0 (evaluation);
				} else {
					fail! (0xd3c0db37);
				},
			
			ProcedureNativeInternals::Native1 (ref native) =>
				if inputs_count == 1 {
					return native.0 (inputs[0].as_ref ());
				} else {
					fail! (0xdba52c0f);
				},
			ProcedureNativeInternals::Native1E (ref native) =>
				if inputs_count == 1 {
					return native.0 (inputs[0].as_ref (), evaluation);
				} else {
					fail! (0x05b52d20);
				},
			
			ProcedureNativeInternals::Native2 (ref native) =>
				if inputs_count == 2 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref ());
				} else {
					fail! (0x07864964);
				},
			ProcedureNativeInternals::Native2E (ref native) =>
				if inputs_count == 2 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), evaluation);
				} else {
					fail! (0x5c7dcbe3);
				},
			
			ProcedureNativeInternals::Native3 (ref native) =>
				if inputs_count == 3 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref ());
				} else {
					fail! (0x0e221162);
				},
			ProcedureNativeInternals::Native3E (ref native) =>
				if inputs_count == 3 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), evaluation);
				} else {
					fail! (0x6a6c5c6b);
				},
			
			ProcedureNativeInternals::Native4 (ref native) =>
				if inputs_count == 4 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref ());
				} else {
					fail! (0x7ec98b38);
				},
			ProcedureNativeInternals::Native4E (ref native) =>
				if inputs_count == 4 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), evaluation);
				} else {
					fail! (0xefff8476);
				},
			
			ProcedureNativeInternals::Native5 (ref native) =>
				if inputs_count == 5 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), inputs[4].as_ref ());
				} else {
					fail! (0xa365d03d);
				},
			ProcedureNativeInternals::Native5E (ref native) =>
				if inputs_count == 5 {
					return native.0 (inputs[0].as_ref (), inputs[1].as_ref (), inputs[2].as_ref (), inputs[3].as_ref (), inputs[4].as_ref (), evaluation);
				} else {
					fail! (0x39ad3d33);
				},
			
			ProcedureNativeInternals::NativeN (ref native) => {
				FIXME! ("find a way to elude this");
				let inputs = vec_slice_to_ref (inputs);
				return native.0 (&inputs);
			},
			ProcedureNativeInternals::NativeNE (ref native) => {
				FIXME! ("find a way to elude this");
				let inputs = vec_slice_to_ref (inputs);
				return native.0 (&inputs, evaluation);
			},
			
			ProcedureNativeInternals::NativeV (ref native) => {
				let native = try_some! (native.0 (inputs_count), 0x669a2cac);
				return self.evaluate_procedure_native_with_values (evaluation, &native, inputs);
			},
			
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_0 (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative0) -> (Outcome<Value>) {
		return native.0 ();
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_0e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative0E) -> (Outcome<Value>) {
		return native.0 (evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_0_g (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native0 (ref native) =>
				return native.0 (),
			ProcedureNativeInternals::Native0E (ref native) =>
				return native.0 (evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[], evaluation),
			_ =>
				fail! (0x3b2b8840),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_1 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative1, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_native_1_with_values (evaluation, native, &input_1);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_1_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative1, input_1 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_1e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative1E, input_1 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		return self.evaluate_procedure_native_1e_with_values (evaluation, native, &input_1);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_1e_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative1E, input_1 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_1_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, input_1 : &Value) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native1 (ref native) =>
				return native.0 (input_1),
			ProcedureNativeInternals::Native1E (ref native) =>
				return native.0 (input_1, evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[input_1]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[input_1], evaluation),
			_ =>
				fail! (0xee275ee6),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_2 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative2, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_native_2_with_values (evaluation, native, &input_1, &input_2);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_2_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_2e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative2E, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		return self.evaluate_procedure_native_2e_with_values (evaluation, native, &input_1, &input_2);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_2e_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative2E, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_2_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native2 (ref native) =>
				return native.0 (input_1, input_2),
			ProcedureNativeInternals::Native2E (ref native) =>
				return native.0 (input_1, input_2, evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[input_1, input_2]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[input_1, input_2], evaluation),
			_ =>
				fail! (0x45146253),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_3 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative3, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_native_3_with_values (evaluation, native, &input_1, &input_2, &input_3);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_3_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_3e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative3E, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		return self.evaluate_procedure_native_3e_with_values (evaluation, native, &input_1, &input_2, &input_3);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_3e_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative3E, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_3_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native3 (ref native) =>
				return native.0 (input_1, input_2, input_3),
			ProcedureNativeInternals::Native3E (ref native) =>
				return native.0 (input_1, input_2, input_3, evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[input_1, input_2, input_3]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[input_1, input_2, input_3], evaluation),
			_ =>
				fail! (0xb6b70b62),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_4 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative4, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_native_4_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_4_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3, input_4);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_4e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative4E, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		return self.evaluate_procedure_native_4e_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_4e_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative4E, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3, input_4, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_4_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native4 (ref native) =>
				return native.0 (input_1, input_2, input_3, input_4),
			ProcedureNativeInternals::Native4E (ref native) =>
				return native.0 (input_1, input_2, input_3, input_4, evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[input_1, input_2, input_3, input_4]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[input_1, input_2, input_3, input_4], evaluation),
			_ =>
				fail! (0x28930473),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_5 (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_native_5_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_5_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNative5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3, input_4, input_5);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_5e (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative5E, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (self.evaluate (evaluation, input_1));
		let input_2 = try! (self.evaluate (evaluation, input_2));
		let input_3 = try! (self.evaluate (evaluation, input_3));
		let input_4 = try! (self.evaluate (evaluation, input_4));
		let input_5 = try! (self.evaluate (evaluation, input_5));
		return self.evaluate_procedure_native_5e_with_values (evaluation, native, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_5e_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNative5E, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return native.0 (input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_5_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		match *native {
			ProcedureNativeInternals::Native5 (ref native) =>
				return native.0 (input_1, input_2, input_3, input_4, input_5),
			ProcedureNativeInternals::Native5E (ref native) =>
				return native.0 (input_1, input_2, input_3, input_4, input_5, evaluation),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.0 (&[input_1, input_2, input_3, input_4, input_5]),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.0 (&[input_1, input_2, input_3, input_4, input_5], evaluation),
			_ =>
				fail! (0xb1b016a9),
		}
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_n (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_native_n_with_values (evaluation, native, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_n_with_values (&self, _evaluation : &mut EvaluatorContext, native : &ProcedureNativeN, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		FIXME! ("find a way to elude this");
		let inputs = vec_slice_to_ref (inputs);
		return native.0 (&inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_ne (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeNE, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (self.evaluate_slice (evaluation, inputs));
		FIXME! ("find a way to elude this");
		let inputs = vec_vec_to_ref (&inputs);
		return self.evaluate_procedure_native_ne_with_values (evaluation, native, &inputs);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_ne_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeNE, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		FIXME! ("find a way to elude this");
		let inputs = vec_slice_to_ref (inputs);
		return native.0 (&inputs, evaluation);
	}
	
	#[ cfg ( feature = "vonuvoli_values_native" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn evaluate_procedure_native_n_g_with_values (&self, evaluation : &mut EvaluatorContext, native : &ProcedureNativeInternals, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return self.evaluate_procedure_native_with_values (evaluation, native, inputs);
	}
	
	
	
	
}




pub struct EvaluatorContext <'a> {
	evaluator : &'a Evaluator,
	context : Option<Context>,
	registers : Option<Registers>,
	parameters : Option<Parameters>,
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	trace_error_disabled : usize,
}


impl <'a> EvaluatorContext<'a> {
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn new (evaluator : &'a Evaluator, context : Option<Context>, parameters : Option<Parameters>) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : evaluator,
				context : context,
				registers : None,
				parameters : parameters,
				#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
				#[ cfg ( feature = "vonuvoli_transcript" ) ]
				trace_error_disabled : 0,
			}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn fork_with_registers (&mut self, registers : Registers) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : self.evaluator,
				context : self.context.clone (),
				registers : Some (registers),
				parameters : self.parameters.clone (),
				#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
				#[ cfg ( feature = "vonuvoli_transcript" ) ]
				trace_error_disabled : self.trace_error_disabled,
			}
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork_with_parameters (&mut self, parameters : Parameters) -> (EvaluatorContext<'a>) {
		return EvaluatorContext {
				evaluator : self.evaluator,
				context : self.context.clone (),
				registers : self.registers.clone (),
				parameters : Some (parameters),
				#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
				#[ cfg ( feature = "vonuvoli_transcript" ) ]
				trace_error_disabled : self.trace_error_disabled,
			}
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn fork_parameters (&mut self) -> (Outcome<EvaluatorContext<'a>>) {
		let parameters = if let Some (ref parameters) = self.parameters {
			try! (parameters.fork ())
		} else {
			try! (Parameters::new_empty ())
		};
		succeed! (self.fork_with_parameters (parameters));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn evaluate (&mut self, input : &Expression) -> (Outcome<Value>) {
		return self.evaluator.evaluate (self, input);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn evaluate_slice (&mut self, inputs : &[Expression]) -> (Outcome<StdVec<Value>>) {
		return self.evaluator.evaluate_slice (self, inputs);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn evaluate_script <Iterator, ExpressionRef> (&mut self, inputs : Iterator) -> (Outcome<()>)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
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
	pub fn evaluate_procedure_call_n (&mut self, callable : &Value, inputs : &[impl StdAsRef<Value>]) -> (Outcome<Value>) {
		return self.evaluator.evaluate_procedure_call_n_with_values (self, callable, inputs);
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parameters (&mut self) -> (Outcome<&Parameters>) {
		succeed! (try_some_ref! (self.parameters, 0x5c1eb919));
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parameter_resolve (&mut self, parameter : &Parameter, default : Option<&Value>) -> (Outcome<Value>) {
		// NOTE:  The following `transmute` should be safe!
		let parameters : &Parameters = unsafe { mem::transmute (try! (self.parameters ())) };
		return parameters.resolve (parameter, default, self);
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parameter_resolve_for_builtin (&mut self, parameter : &UniqueData) -> (Outcome<Option<Value>>) {
		// NOTE:  The following `transmute` should be safe!
		let parameters : &Parameters = unsafe { mem::transmute (try! (self.parameters ())) };
		return parameters.resolve_for_builtin (parameter, self);
	}
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn parameter_configure (&mut self, parameter : &Parameter, value : &Value) -> (Outcome<()>) {
		// NOTE:  The following `transmute` should be safe!
		let parameters : &Parameters = unsafe { mem::transmute (try! (self.parameters ())) };
		return parameters.configure (parameter, value, self);
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn disable_trace_error (&mut self) -> () {
		self.trace_error_disabled += 1;
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn enable_trace_error (&mut self) -> () {
		if self.trace_error_disabled == 0 {
			panic_0! (0xbe205870, github_issue_new);
		}
		self.trace_error_disabled -= 1;
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn should_trace_error (&self) -> (bool) {
		return self.trace_error_disabled == 0;
	}
}




#[ derive ( Clone, Default ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub struct EvaluatorConfiguration {
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_input : Option<bool>,
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_output : Option<bool>,
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_error : Option<bool>,
}


impl EvaluatorConfiguration {
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_input (&self) -> (bool) {
		self.trace_input.unwrap_or (EVALUATOR_TRACE_INPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_output (&self) -> (bool) {
		self.trace_output.unwrap_or (EVALUATOR_TRACE_OUTPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_error (&self) -> (bool) {
		self.trace_error.unwrap_or (EVALUATOR_TRACE_ERROR)
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn should_trace_output_or_error (&self) -> (bool) {
		self.should_trace_output () || self.should_trace_error ()
	}
	
	#[ cfg ( feature = "vonuvoli_evaluator_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_trace_enabled (&self) -> (bool) {
		self.should_trace_input () || self.should_trace_output () || self.should_trace_error ()
	}
}

