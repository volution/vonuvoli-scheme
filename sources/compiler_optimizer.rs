

use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::lambdas::exports::*;
use super::native_procedures::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::optimize;
	pub use super::optimize_script;
}




pub fn optimize (expression : Expression) -> (Outcome<Expression>) {
	return Optimizer::new () .optimize (expression);
}

pub fn optimize_script (expressions : ExpressionVec) -> (Outcome<ExpressionVec>) {
	return Optimizer::new () .optimize_vec (expressions);
}




#[ derive (Debug) ]
pub(crate) struct Optimizer {}


impl Optimizer {
	
	
	
	
	pub(crate) fn new () -> (Optimizer) {
		return Optimizer {};
	}
	
	
	
	
	pub(crate) fn optimize (&self, expression : Expression) -> (Outcome<Expression>) {
		let optimization = OptimizerContext::new ();
		let (_optimization, expression) = try! (self.optimize_0 (optimization, expression));
		succeed! (expression);
	}
	
	pub(crate) fn optimize_vec (&self, expressions : ExpressionVec) -> (Outcome<ExpressionVec>) {
		let optimization = OptimizerContext::new ();
		let (_optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		succeed! (expressions);
	}
	
	
	
	
	fn optimize_0 (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		
		if OPTIMIZER_TRACE_INPUT || OPTIMIZER_TRACE_OUTPUT || OPTIMIZER_TRACE_ERROR {
			
			let expression_input = expression.clone ();
			
			if OPTIMIZER_TRACE_INPUT {
				eprint! ("[dd]  optimizing: {:?}\n", &expression_input);
			}
			
			let outcome = self.optimize_00 (optimization, expression);
			
			match outcome {
				Ok (ref expression_optimized) if OPTIMIZER_TRACE_OUTPUT =>
					eprint! ("[dd]  optimizing succeeded:\n[  ]      {:?}\n[  ]      {:?}\n", &expression_input, expression_optimized),
				Ok (_) =>
					(),
				Err (ref error) if (OPTIMIZER_TRACE_OUTPUT || OPTIMIZER_TRACE_ERROR) && error.is_traceable () =>
					eprint! ("[dd]  optimizing failed:\n[  ]      {:?}\n[  ]      {:?}\n", &expression_input, error),
				Err (_) =>
					(),
			}
			
			return outcome;
			
		} else {
			
			return self.optimize_00 (optimization, expression);
		}
	}
	
	
	fn optimize_00 (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			Expression::Void =>
				succeed! ((optimization, Expression::Void)),
			Expression::Value (value) =>
				return self.optimize_value (optimization, value),
			
			Expression::Sequence (operator, expressions) =>
				return self.optimize_sequence (optimization, operator, expressions),
			Expression::ConditionalIf (clauses) =>
				return self.optimize_conditional_if (optimization, clauses),
			Expression::ConditionalMatch (actual, clauses) =>
				return self.optimize_conditional_match (optimization, *actual, clauses),
			Expression::Loop (initialize, update, body, clauses) =>
				return self.optimize_loop (optimization, option_box_into_owned (initialize), option_box_into_owned (update), option_box_into_owned (body), clauses),
			
			Expression::Contexts (expression) =>
				return self.optimize_for_contexts (optimization, expression),
			
			Expression::ProcedureGenericCall (expression) =>
				return self.optimize_for_procedure_generic_call (optimization, expression),
			Expression::ProcedurePrimitiveCall (expression) =>
				return self.optimize_for_procedure_primitive_call (optimization, expression),
			Expression::ProcedureExtendedCall (expression) =>
				return self.optimize_for_procedure_extended_call (optimization, expression),
			Expression::ProcedureLambdaCall (expression) =>
				return self.optimize_for_procedure_lambda_call (optimization, expression),
			Expression::ProcedureNativeCall (expression) =>
				return self.optimize_for_procedure_native_call (optimization, expression),
			
			Expression::Lambda (lambda, expression, registers_closure, registers_local) =>
				return self.optimize_lambda_create (optimization, lambda, expression, registers_closure, registers_local),
			
			Expression::ErrorReturn (expression) =>
				return self.optimize_error_return (optimization, *expression),
			Expression::ErrorCatch (expression, error_consumer, error_expression) =>
				return self.optimize_error_catch (optimization, *expression, error_consumer, *error_expression),
			Expression::ErrorThrow (expression) =>
				return self.optimize_error_throw (optimization, *expression),
			
		}
	}
	
	
	
	
	fn optimize_for_contexts (&self, optimization : OptimizerContext, expression : ExpressionForContexts) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForContexts::ContextDefine (identifier, expression) =>
				return self.optimize_context_define (optimization, identifier, *expression),
			ExpressionForContexts::ContextUpdate (identifier, expression) =>
				return self.optimize_context_update (optimization, identifier, *expression),
			ExpressionForContexts::ContextSelect (identifier) =>
				return self.optimize_context_select (optimization, identifier),
			
			ExpressionForContexts::BindingInitialize1 (binding, expression) =>
				return self.optimize_binding_initialize_1 (optimization, binding, *expression),
			ExpressionForContexts::BindingInitializeN (initializers, parallel) =>
				return self.optimize_binding_initialize_n (optimization, initializers, parallel),
			ExpressionForContexts::BindingInitializeValues (bindings, expression) =>
				return self.optimize_binding_initialize_values (optimization, bindings, *expression),
			ExpressionForContexts::BindingSet1 (binding, expression) =>
				return self.optimize_binding_set_1 (optimization, binding, *expression),
			ExpressionForContexts::BindingSetN (initializers, parallel) =>
				return self.optimize_binding_set_n (optimization, initializers, parallel),
			ExpressionForContexts::BindingSetValues (bindings, expression) =>
				return self.optimize_binding_set_values (optimization, bindings, *expression),
			ExpressionForContexts::BindingGet1 (binding) =>
				return self.optimize_binding_get_1 (optimization, binding),
			
			ExpressionForContexts::RegisterClosure (expression, borrows) =>
				return self.optimize_register_closure (optimization, *expression, borrows),
			ExpressionForContexts::RegisterInitialize1 (index, expression) =>
				return self.optimize_register_initialize_1 (optimization, index, *expression),
			ExpressionForContexts::RegisterInitializeN (initializers, parallel) =>
				return self.optimize_register_initialize_n (optimization, initializers, parallel),
			ExpressionForContexts::RegisterInitializeValues (indices, expression) =>
				return self.optimize_register_initialize_values (optimization, indices, *expression),
			ExpressionForContexts::RegisterSet1 (index, expression) =>
				return self.optimize_register_set_1 (optimization, index, *expression),
			ExpressionForContexts::RegisterSetN (initializers, parallel) =>
				return self.optimize_register_set_n (optimization, initializers, parallel),
			ExpressionForContexts::RegisterSetValues (indices, expression) =>
				return self.optimize_register_set_values (optimization, indices, *expression),
			ExpressionForContexts::RegisterGet1 (index) =>
				return self.optimize_register_get_1 (optimization, index),
			
		}
	}
	
	
	fn optimize_for_procedure_generic_call (&self, optimization : OptimizerContext, expression : ExpressionForProcedureGenericCall) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForProcedureGenericCall::ProcedureCall (callable, inputs) =>
				return self.optimize_procedure_call (optimization, *callable, inputs),
			ExpressionForProcedureGenericCall::ProcedureCall0 (callable) =>
				return self.optimize_procedure_call_0 (optimization, *callable),
			ExpressionForProcedureGenericCall::ProcedureCall1 (callable, input_1) =>
				return self.optimize_procedure_call_1 (optimization, *callable, *input_1),
			ExpressionForProcedureGenericCall::ProcedureCall2 (callable, input_1, input_2) =>
				return self.optimize_procedure_call_2 (optimization, *callable, *input_1, *input_2),
			ExpressionForProcedureGenericCall::ProcedureCall3 (callable, input_1, input_2, input_3) =>
				return self.optimize_procedure_call_3 (optimization, *callable, *input_1, *input_2, *input_3),
			ExpressionForProcedureGenericCall::ProcedureCall4 (callable, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_call_4 (optimization, *callable, *input_1, *input_2, *input_3, *input_4),
			ExpressionForProcedureGenericCall::ProcedureCall5 (callable, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_call_5 (optimization, *callable, *input_1, *input_2, *input_3, *input_4, *input_5),
			ExpressionForProcedureGenericCall::ProcedureCallN (callable, inputs) =>
				return self.optimize_procedure_call_n (optimization, *callable, inputs),
			
			
		}
	}
	
	
	fn optimize_for_procedure_primitive_call (&self, optimization : OptimizerContext, expression : ExpressionForProcedurePrimitiveCall) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall (primitive, inputs) =>
				return self.optimize_procedure_primitive (optimization, primitive, inputs),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (primitive) =>
				return self.optimize_procedure_primitive_0 (optimization, primitive),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (primitive, input_1) =>
				return self.optimize_procedure_primitive_1 (optimization, primitive, *input_1),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (primitive, input_1, input_2) =>
				return self.optimize_procedure_primitive_2 (optimization, primitive, *input_1, *input_2),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (primitive, input_1, input_2, input_3) =>
				return self.optimize_procedure_primitive_3 (optimization, primitive, *input_1, *input_2, *input_3),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (primitive, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_primitive_4 (optimization, primitive, *input_1, *input_2, *input_3, *input_4),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (primitive, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_primitive_5 (optimization, primitive, *input_1, *input_2, *input_3, *input_4, *input_5),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (primitive, inputs) =>
				return self.optimize_procedure_primitive_n (optimization, primitive, inputs),
			ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallV (primitive, inputs) =>
				return self.optimize_procedure_primitive_v (optimization, primitive, inputs),
			
		}
	}
	
	
	fn optimize_for_procedure_extended_call (&self, optimization : OptimizerContext, expression : ExpressionForProcedureExtendedCall) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall (extended, inputs) =>
				return self.optimize_procedure_extended (optimization, extended, inputs),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (extended) =>
				return self.optimize_procedure_extended_0 (optimization, extended),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (extended, input_1) =>
				return self.optimize_procedure_extended_1 (optimization, extended, *input_1),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (extended, input_1, input_2) =>
				return self.optimize_procedure_extended_2 (optimization, extended, *input_1, *input_2),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (extended, input_1, input_2, input_3) =>
				return self.optimize_procedure_extended_3 (optimization, extended, *input_1, *input_2, *input_3),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (extended, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_extended_4 (optimization, extended, *input_1, *input_2, *input_3, *input_4),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (extended, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_extended_5 (optimization, extended, *input_1, *input_2, *input_3, *input_4, *input_5),
			ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (extended, inputs) =>
				return self.optimize_procedure_extended_n (optimization, extended, inputs),
			
		}
	}
	
	
	fn optimize_for_procedure_lambda_call (&self, optimization : OptimizerContext, expression : ExpressionForProcedureLambdaCall) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall (lambda, inputs) =>
				return self.optimize_procedure_lambda (optimization, lambda, inputs),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (lambda) =>
				return self.optimize_procedure_lambda_0 (optimization, lambda),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (lambda, input_1) =>
				return self.optimize_procedure_lambda_1 (optimization, lambda, *input_1),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (lambda, input_1, input_2) =>
				return self.optimize_procedure_lambda_2 (optimization, lambda, *input_1, *input_2),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (lambda, input_1, input_2, input_3) =>
				return self.optimize_procedure_lambda_3 (optimization, lambda, *input_1, *input_2, *input_3),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (lambda, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_lambda_4 (optimization, lambda, *input_1, *input_2, *input_3, *input_4),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (lambda, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_lambda_5 (optimization, lambda, *input_1, *input_2, *input_3, *input_4, *input_5),
			ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (lambda, inputs) =>
				return self.optimize_procedure_lambda_n (optimization, lambda, inputs),
			
		}
	}
	
	
	fn optimize_for_procedure_native_call (&self, optimization : OptimizerContext, expression : ExpressionForProcedureNativeCall) -> (Outcome<(OptimizerContext, Expression)>) {
		match expression {
			
			ExpressionForProcedureNativeCall::ProcedureNativeCall (native, inputs) =>
				return self.optimize_procedure_native (optimization, native, inputs),
			ExpressionForProcedureNativeCall::ProcedureNativeCall0 (native) =>
				return self.optimize_procedure_native_0 (optimization, native),
			ExpressionForProcedureNativeCall::ProcedureNativeCall1 (native, input_1) =>
				return self.optimize_procedure_native_1 (optimization, native, *input_1),
			ExpressionForProcedureNativeCall::ProcedureNativeCall2 (native, input_1, input_2) =>
				return self.optimize_procedure_native_2 (optimization, native, *input_1, *input_2),
			ExpressionForProcedureNativeCall::ProcedureNativeCall3 (native, input_1, input_2, input_3) =>
				return self.optimize_procedure_native_3 (optimization, native, *input_1, *input_2, *input_3),
			ExpressionForProcedureNativeCall::ProcedureNativeCall4 (native, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_native_4 (optimization, native, *input_1, *input_2, *input_3, *input_4),
			ExpressionForProcedureNativeCall::ProcedureNativeCall5 (native, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_native_5 (optimization, native, *input_1, *input_2, *input_3, *input_4, *input_5),
			ExpressionForProcedureNativeCall::ProcedureNativeCallN (native, inputs) =>
				return self.optimize_procedure_native_n (optimization, native, inputs),
			
		}
	}
	
	
	
	
	#[ allow (dead_code) ]
	fn optimize_0_box (&self, optimization : OptimizerContext, expression : StdBox<Expression>) -> (Outcome<(OptimizerContext, StdBox<Expression>)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, *expression));
		let expression = StdBox::new (expression);
		succeed! ((optimization, expression));
	}
	
	#[ allow (dead_code) ]
	fn optimize_0_box_to_owned (&self, optimization : OptimizerContext, expression : StdBox<Expression>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, *expression));
		succeed! ((optimization, expression));
	}
	
	fn optimize_0_slice (&self, optimization : OptimizerContext, expressions : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, StdBox<[Expression]>)>) {
		let expressions = StdVec::from (expressions);
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let expressions = expressions.into_boxed_slice ();
		succeed! ((optimization, expressions));
	}
	
	fn optimize_0_slice_to_owned (&self, optimization : OptimizerContext, expressions : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, StdVec<Expression>)>) {
		let expressions = StdVec::from (expressions);
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		succeed! ((optimization, expressions));
	}
	
	fn optimize_0_option (&self, optimization : OptimizerContext, expression : Option<Expression>) -> (Outcome<(OptimizerContext, Option<Expression>)>) {
		if let Some (expression) = expression {
			let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
			succeed! ((optimization, Some (expression)));
		} else {
			succeed! ((optimization, None));
		}
	}
	
	fn optimize_0_vec (&self, optimization : OptimizerContext, expressions : ExpressionVec) -> (Outcome<(OptimizerContext, ExpressionVec)>) {
		let mut optimization = optimization;
		let mut expressions_1 = ExpressionVec::with_capacity (expressions.len ());
		for expression in expressions.into_iter () {
			let (optimization_1, expression) = try! (self.optimize_0 (optimization, expression));
			optimization = optimization_1;
			expressions_1.push (expression);
		}
		succeed! ((optimization, expressions_1));
	}
	
	fn optimize_0_vec_transform <Input, Output, Transformer> (&self, optimization : OptimizerContext, inputs : StdVec<Input>, transformer : Transformer) -> (Outcome<(OptimizerContext, StdVec<Output>)>)
			where Transformer : Fn (OptimizerContext, Input) -> (Outcome<(OptimizerContext, Output)>)
	{
		let mut optimization = optimization;
		let mut outputs = StdVec::with_capacity (inputs.len ());
		for input in inputs.into_iter () {
			let (optimization_1, output) = try! (transformer (optimization, input));
			optimization = optimization_1;
			outputs.push (output);
		}
		succeed! ((optimization, outputs));
	}
	
	
	
	
	fn optimize_value (&self, optimization : OptimizerContext, value : Value) -> (Outcome<(OptimizerContext, Expression)>) {
		if value.is_class (ValueClass::Void) {
			succeed! ((optimization, Expression::Void));
		} else {
			succeed! ((optimization, Expression::Value (value)));
		}
	}
	
	
	
	
	fn optimize_sequence (&self, optimization : OptimizerContext, operator : ExpressionSequenceOperator, expressions : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		
		let (optimization, expressions) = try! (self.optimize_0_slice_to_owned (optimization, expressions));
		
		match operator {
			
			ExpressionSequenceOperator::ReturnLast => {
				let mut expressions = expressions;
				let expression = if let Some (last) = expressions.pop () {
					let mut expressions = self.expressions_retain_if_is_not (&optimization, expressions, ExpressionClass::Constant);
					if ! expressions.is_empty () {
						expressions.push (last);
						Expression::Sequence (operator, expressions.into_boxed_slice ())
					} else {
						last
					}
				} else {
					Expression::Void
				};
				succeed! ((optimization, expression));
			},
			
			ExpressionSequenceOperator::ReturnFirst => {
				let mut expressions = expressions.into_iter ();
				let expression = if let Some (first) = expressions.next () {
					let mut expressions = self.expressions_collect_if_is_not (&optimization, expressions, ExpressionClass::Constant);
					if ! expressions.is_empty () {
						expressions.insert (0, first);
						Expression::Sequence (operator, expressions.into_boxed_slice ())
					} else {
						first
					}
				} else {
					Expression::Void
				};
				succeed! ((optimization, expression));
			},
			
			ExpressionSequenceOperator::And => {
				let mut expressions = expressions;
				let expression = if let Some (last) = expressions.pop () {
					let mut expressions = self.expressions_retain_if_is_not (&optimization, expressions, ExpressionClass::Type (TypePrimitive1::IsFalseNot));
					if ! expressions.is_empty () {
						expressions.push (last);
						if self.expressions_are_any (&optimization, expressions.iter (), ExpressionClass::Type (TypePrimitive1::IsFalse)) {
							Expression::Value (FALSE_VALUE)
						} else {
							Expression::Sequence (operator, expressions.into_boxed_slice ())
						}
					} else {
						last
					}
				} else {
					Expression::Value (TRUE_VALUE)
				};
				succeed! ((optimization, expression));
			},
			
			ExpressionSequenceOperator::Or => {
				let mut expressions = expressions;
				let expression = if let Some (last) = expressions.pop () {
					expressions.push (last);
					let mut expressions = self.expressions_retain_if_is_not (&optimization, expressions, ExpressionClass::Type (TypePrimitive1::IsFalse));
					if ! expressions.is_empty () {
						let expressions = if self.expressions_are_any (&optimization, expressions.iter (), ExpressionClass::Type (TypePrimitive1::IsFalseNot)) {
							let expressions_0 = expressions;
							let mut expressions = StdVec::new ();
							for expression in expressions_0.into_iter () {
								if self.expression_is (&optimization, &expression, ExpressionClass::Type (TypePrimitive1::IsFalseNot)) {
									expressions.push (expression);
									break;
								} else {
									expressions.push (expression);
								}
							}
							expressions
						} else {
							expressions
						};
						if expressions.len () == 1 {
							let mut expressions = expressions;
							expressions.pop () .unwrap ()
						} else {
							Expression::Sequence (operator, expressions.into_boxed_slice ())
						}
					} else {
						Expression::Value (FALSE_VALUE)
					}
				} else {
					Expression::Value (FALSE_VALUE)
				};
				succeed! ((optimization, expression));
			}
		}
	}
	
	
	
	
	fn optimize_conditional_if (&self, optimization : OptimizerContext, clauses : ExpressionConditionalIfClauses) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, clauses) = try! (self.optimize_conditional_if_clauses (optimization, clauses));
		let expression = Expression::ConditionalIf (clauses);
		succeed! ((optimization, expression));
	}
	
	fn optimize_conditional_if_clauses (&self, optimization : OptimizerContext, clauses : ExpressionConditionalIfClauses) -> (Outcome<(OptimizerContext, ExpressionConditionalIfClauses)>) {
		match clauses {
			ExpressionConditionalIfClauses::Void =>
				succeed! ((optimization, clauses)),
			ExpressionConditionalIfClauses::Single (clause) => {
				let (optimization, clause) = try! (self.optimize_conditional_if_clause (optimization, *clause));
				let clauses = match clause {
					ExpressionConditionalIfClause::Void =>
						ExpressionConditionalIfClauses::Void,
					_ =>
						ExpressionConditionalIfClauses::Single (StdBox::new (clause)),
				};
				succeed! ((optimization, clauses));
			},
			ExpressionConditionalIfClauses::Multiple (clauses) => {
				let (optimization, clauses) = try! (self.optimize_0_vec_transform (optimization, StdVec::from (clauses),
						|optimization, clause| self.optimize_conditional_if_clause (optimization, clause)));
				let clauses = vec_filter_into! (clauses, clause,
						match *clause {
							ExpressionConditionalIfClause::Void =>
								false,
							_ =>
								true,
						});
				let clauses = match clauses.len () {
					0 =>
						ExpressionConditionalIfClauses::Void,
					1 =>
						ExpressionConditionalIfClauses::Single (StdBox::new (try! (vec_explode_1 (clauses)))),
					_ =>
						ExpressionConditionalIfClauses::Multiple (clauses.into_boxed_slice ()),
				};
				succeed! ((optimization, clauses));
			}
		}
	}
	
	fn optimize_conditional_if_clause (&self, optimization : OptimizerContext, clause : ExpressionConditionalIfClause) -> (Outcome<(OptimizerContext, ExpressionConditionalIfClause)>) {
		match clause {
			ExpressionConditionalIfClause::Void =>
				succeed! ((optimization, clause)),
			ExpressionConditionalIfClause::GuardOnly (guard, guard_consumer) => {
				let (optimization, guard) = try! (self.optimize_conditional_if_guard (optimization, guard));
				let (optimization, guard_consumer) = try! (self.optimize_value_consumer (optimization, guard_consumer));
				let clause = match guard {
					ExpressionConditionalIfGuard::False =>
						match guard_consumer {
							ExpressionValueConsumer::Return =>
								ExpressionConditionalIfClause::Void,
							_ =>
								ExpressionConditionalIfClause::GuardOnly (guard, guard_consumer),
						},
					_ =>
						ExpressionConditionalIfClause::GuardOnly (guard, guard_consumer),
				};
				succeed! ((optimization, clause));
			},
			ExpressionConditionalIfClause::GuardAndExpression (guard, guard_consumer, output) => {
				let (optimization, guard) = try! (self.optimize_conditional_if_guard (optimization, guard));
				let (optimization, guard_consumer) = try! (self.optimize_value_consumer (optimization, guard_consumer));
				let (optimization, output) = try! (self.optimize_0 (optimization, output));
				let clause = match guard {
					ExpressionConditionalIfGuard::False =>
						match guard_consumer {
							ExpressionValueConsumer::Return =>
								ExpressionConditionalIfClause::Void,
							_ =>
								ExpressionConditionalIfClause::GuardOnly (guard, guard_consumer),
						},
					_ =>
						ExpressionConditionalIfClause::GuardAndExpression (guard, guard_consumer, output),
				};
				succeed! ((optimization, clause));
			},
		}
	}
	
	fn optimize_conditional_if_guard (&self, optimization : OptimizerContext, guard : ExpressionConditionalIfGuard) -> (Outcome<(OptimizerContext, ExpressionConditionalIfGuard)>) {
		match guard {
			ExpressionConditionalIfGuard::True =>
				succeed! ((optimization, guard)),
			ExpressionConditionalIfGuard::False =>
				succeed! ((optimization, guard)),
			ExpressionConditionalIfGuard::Expression (expression, negated) => {
				let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
				let guard = ExpressionConditionalIfGuard::Expression (expression, negated);
				succeed! ((optimization, guard));
			},
		}
	}
	
	
	
	
	fn optimize_conditional_match (&self, optimization : OptimizerContext, actual : Expression, clauses : ExpressionConditionalMatchClauses) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, actual) = try! (self.optimize_0 (optimization, actual));
		let (optimization, clauses) = try! (self.optimize_conditional_match_clauses (optimization, clauses));
		let expression = Expression::ConditionalMatch (actual.into (), clauses);
		succeed! ((optimization, expression));
	}
	
	fn optimize_conditional_match_clauses (&self, optimization : OptimizerContext, clauses : ExpressionConditionalMatchClauses) -> (Outcome<(OptimizerContext, ExpressionConditionalMatchClauses)>) {
		match clauses {
			ExpressionConditionalMatchClauses::Void =>
				succeed! ((optimization, clauses)),
			ExpressionConditionalMatchClauses::Single (clause) => {
				let (optimization, clause) = try! (self.optimize_conditional_match_clause (optimization, *clause));
				let clauses = match clause {
					ExpressionConditionalMatchClause::Void =>
						ExpressionConditionalMatchClauses::Void,
					_ =>
						ExpressionConditionalMatchClauses::Single (StdBox::new (clause)),
				};
				succeed! ((optimization, clauses));
			},
			ExpressionConditionalMatchClauses::Multiple (clauses) => {
				let (optimization, clauses) = try! (self.optimize_0_vec_transform (optimization, StdVec::from (clauses),
						|optimization, clause| self.optimize_conditional_match_clause (optimization, clause)));
				let clauses = vec_filter_into! (clauses, clause,
						match *clause {
							ExpressionConditionalMatchClause::Void =>
								false,
							_ =>
								true,
						});
				let clauses = match clauses.len () {
					0 =>
						ExpressionConditionalMatchClauses::Void,
					1 =>
						ExpressionConditionalMatchClauses::Single (StdBox::new (try! (vec_explode_1 (clauses)))),
					_ =>
						ExpressionConditionalMatchClauses::Multiple (clauses.into_boxed_slice ()),
				};
				succeed! ((optimization, clauses));
			}
		}
	}
	
	fn optimize_conditional_match_clause (&self, optimization : OptimizerContext, clause : ExpressionConditionalMatchClause) -> (Outcome<(OptimizerContext, ExpressionConditionalMatchClause)>) {
		match clause {
			ExpressionConditionalMatchClause::Void =>
				succeed! ((optimization, clause)),
			ExpressionConditionalMatchClause::GuardOnly (guard, guard_consumer) => {
				let (optimization, guard) = try! (self.optimize_conditional_match_guard (optimization, guard));
				let (optimization, guard_consumer) = try! (self.optimize_value_consumer (optimization, guard_consumer));
				let clause = match guard {
					ExpressionConditionalMatchGuard::False =>
						match guard_consumer {
							ExpressionValueConsumer::Return =>
								ExpressionConditionalMatchClause::Void,
							_ =>
								ExpressionConditionalMatchClause::GuardOnly (guard, guard_consumer),
						},
					_ =>
						ExpressionConditionalMatchClause::GuardOnly (guard, guard_consumer),
				};
				succeed! ((optimization, clause));
			},
			ExpressionConditionalMatchClause::GuardAndExpression (guard, guard_consumer, output) => {
				let (optimization, guard) = try! (self.optimize_conditional_match_guard (optimization, guard));
				let (optimization, guard_consumer) = try! (self.optimize_value_consumer (optimization, guard_consumer));
				let (optimization, output) = try! (self.optimize_0 (optimization, output));
				let clause = match guard {
					ExpressionConditionalMatchGuard::False =>
						match guard_consumer {
							ExpressionValueConsumer::Return =>
								ExpressionConditionalMatchClause::Void,
							_ =>
								ExpressionConditionalMatchClause::GuardOnly (guard, guard_consumer),
						},
					_ =>
						ExpressionConditionalMatchClause::GuardAndExpression (guard, guard_consumer, output),
				};
				succeed! ((optimization, clause));
			},
		}
	}
	
	fn optimize_conditional_match_guard (&self, optimization : OptimizerContext, guard : ExpressionConditionalMatchGuard) -> (Outcome<(OptimizerContext, ExpressionConditionalMatchGuard)>) {
		match guard {
			ExpressionConditionalMatchGuard::True =>
				succeed! ((optimization, guard)),
			ExpressionConditionalMatchGuard::False =>
				succeed! ((optimization, guard)),
			ExpressionConditionalMatchGuard::Value (_, _) =>
				succeed! ((optimization, guard)),
			ExpressionConditionalMatchGuard::Values (values, negated) => {
				let mut values = StdVec::from (values);
				values.sort ();
				values.dedup ();
				match values.len () {
					0 =>
						succeed! ((optimization, ExpressionConditionalMatchGuard::False)),
					1 =>
						succeed! ((optimization, ExpressionConditionalMatchGuard::Value (try! (vec_explode_1 (values)), negated))),
					_ =>
						succeed! ((optimization, ExpressionConditionalMatchGuard::Values (values.into_boxed_slice (), negated))),
				}
			},
		}
	}
	
	
	
	
	fn optimize_value_consumer (&self, optimization : OptimizerContext, consumer : ExpressionValueConsumer) -> (Outcome<(OptimizerContext, ExpressionValueConsumer)>) {
		succeed! ((optimization, consumer));
	}
	
	
	
	
	fn optimize_loop (&self, optimization : OptimizerContext, initialize : Option<Expression>, update : Option<Expression>, body : Option<Expression>, clauses : ExpressionConditionalIfClauses) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, initialize) = try! (self.optimize_0_option (optimization, initialize));
		let (optimization, update) = try! (self.optimize_0_option (optimization, update));
		let (optimization, body) = try! (self.optimize_0_option (optimization, body));
		let (optimization, clauses) = try! (self.optimize_conditional_if_clauses (optimization, clauses));
		let expression = Expression::Loop (option_box_new (initialize), option_box_new (update), option_box_new (body), clauses);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_context_define (&self, optimization : OptimizerContext, identifier : Symbol, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::ContextDefine (identifier, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_context_update (&self, optimization : OptimizerContext, identifier : Symbol, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::ContextUpdate (identifier, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_context_select (&self, optimization : OptimizerContext, identifier : Symbol) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForContexts::ContextSelect (identifier) .into ();
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_binding_initialize_1 (&self, optimization : OptimizerContext, binding : Binding, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::BindingInitialize1 (binding, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_initialize_n (&self, optimization : OptimizerContext, initializers : StdBox<[(Binding, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (bindings, expressions) = vec_unzip_2 (StdVec::from (initializers));
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (bindings, expressions) .into_boxed_slice ();
		let expression = ExpressionForContexts::BindingInitializeN (initializers, parallel) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_initialize_values (&self, optimization : OptimizerContext, bindings : StdBox<[Binding]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::BindingInitializeValues (bindings, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_1 (&self, optimization : OptimizerContext, binding : Binding, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::BindingSet1 (binding, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_n (&self, optimization : OptimizerContext, initializers : StdBox<[(Binding, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (bindings, expressions) = vec_unzip_2 (StdVec::from (initializers));
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (bindings, expressions) .into_boxed_slice ();
		let expression = ExpressionForContexts::BindingSetN (initializers, parallel) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_values (&self, optimization : OptimizerContext, bindings : StdBox<[Binding]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::BindingSetValues (bindings, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_get_1 (&self, optimization : OptimizerContext, binding : Binding) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = if binding.is_immutable () {
			let value = try! (binding.get ());
			Expression::Value (value)
		} else {
			ExpressionForContexts::BindingGet1 (binding) .into ()
		};
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_register_closure (&self, optimization : OptimizerContext, expression : Expression, borrows : StdBox<[RegisterTemplate]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::RegisterClosure (expression.into (), borrows) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_1 (&self, optimization : OptimizerContext, index : usize, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::RegisterInitialize1 (index, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_n (&self, optimization : OptimizerContext, initializers : StdBox<[(usize, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (indices, expressions) = vec_unzip_2 (StdVec::from (initializers));
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (indices, expressions) .into_boxed_slice ();
		let expression = ExpressionForContexts::RegisterInitializeN (initializers, parallel) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_values (&self, optimization : OptimizerContext, indices : StdBox<[usize]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::RegisterInitializeValues (indices, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_1 (&self, optimization : OptimizerContext, index : usize, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::RegisterSet1 (index, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_n (&self, optimization : OptimizerContext, initializers : StdBox<[(usize, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (indices, expressions) = vec_unzip_2 (StdVec::from (initializers));
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (indices, expressions);
		let initializers = vec_filter_into! (
				initializers,
				&(target_index, ref expression),
				match *expression {
					Expression::Contexts (ExpressionForContexts::RegisterGet1 (source_index)) =>
						target_index != source_index,
					_ =>
						true,
				});
		let expression = if initializers.len () == 0 {
			Expression::Void
		} else if initializers.len () == 1 {
			let (index, expression) = try! (vec_explode_1 (initializers));
			ExpressionForContexts::RegisterSet1 (index, expression.into ()) .into ()
		} else {
			let initializers = initializers.into_boxed_slice ();
			ExpressionForContexts::RegisterSetN (initializers, parallel) .into ()
		};
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_values (&self, optimization : OptimizerContext, indices : StdBox<[usize]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = ExpressionForContexts::RegisterSetValues (indices, expression.into ()) .into ();
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_get_1 (&self, optimization : OptimizerContext, index : usize) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForContexts::RegisterGet1 (index) .into ();
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_error_return (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::ErrorReturn (expression.into ());
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_error_catch (&self, optimization : OptimizerContext, expression : Expression, error_consumer : ExpressionValueConsumer, error_expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let (optimization, error_consumer) = try! (self.optimize_value_consumer (optimization, error_consumer));
		let (optimization, error_expression) = try! (self.optimize_0 (optimization, error_expression));
		let expression = Expression::ErrorCatch (expression.into (), error_consumer, error_expression.into ());
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_error_throw (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::ErrorThrow (expression.into ());
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_lambda_create (&self, optimization : OptimizerContext, template : StdRc<LambdaTemplate>, expression : StdRc<Expression>, registers_closure : StdBox<[RegisterTemplate]>, registers_local : StdRc<[RegisterTemplate]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = match StdRc::try_unwrap (expression) {
			Ok (expression) =>
				expression,
			Err (expression) =>
				// NOTE:  This happens only when the expression was cloned...
				StdRc::as_ref (&expression) .clone (),
		};
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = StdRc::new (expression);
		let expression = Expression::Lambda (template, expression, registers_closure, registers_local);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_procedure_call (&self, optimization : OptimizerContext, callable : Expression, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					return self.optimize_procedure_primitive (optimization, callable.expect_into_0 (), inputs),
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended (optimization, callable.expect_into_0 (), inputs),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), inputs),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), inputs),
				_ =>
					(),
			}
		}
		match inputs.len () {
			0 =>
				return self.optimize_procedure_call_0 (optimization, callable),
			1 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				return self.optimize_procedure_call_1 (optimization, callable, input_1);
			},
			2 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				return self.optimize_procedure_call_2 (optimization, callable, input_1, input_2);
			},
			3 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				return self.optimize_procedure_call_3 (optimization, callable, input_1, input_2, input_3);
			},
			4 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				return self.optimize_procedure_call_4 (optimization, callable, input_1, input_2, input_3, input_4);
			},
			5 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				let input_5 = inputs.next () .unwrap ();
				return self.optimize_procedure_call_5 (optimization, callable, input_1, input_2, input_3, input_4, input_5);
			},
			_ =>
				(),
		}
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall (callable.into (), inputs) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_0 (&self, optimization : OptimizerContext, callable : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive0 (primitive) =>
							return self.optimize_procedure_primitive_0 (optimization, primitive),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
								return self.optimize_procedure_primitive_0 (optimization, primitive);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([]));
							} else {
								fail! (0x92236b30);
							},
						_ =>
							fail! (0xc262ceb2),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_0 (optimization, callable.expect_into_0 ()),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_0 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into ()),
				_ =>
					(),
			}
		}
		let expression = ExpressionForProcedureGenericCall::ProcedureCall0 (callable.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_1 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive1 (primitive) =>
							return self.optimize_procedure_primitive_1 (optimization, primitive, input_1),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
								return self.optimize_procedure_primitive_1 (optimization, primitive, input_1);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1]));
							} else {
								fail! (0x82fe9854);
							},
						_ =>
							fail! (0xa8613b5a),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_1 (optimization, callable.expect_into_0 (), input_1),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([input_1])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_1 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), input_1),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall1 (callable.into (), input_1.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_2 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive2 (primitive) =>
							return self.optimize_procedure_primitive_2 (optimization, primitive, input_1, input_2),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
								return self.optimize_procedure_primitive_2 (optimization, primitive, input_1, input_2);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2]));
							} else {
								fail! (0xc6d7637c);
							},
						_ =>
							fail! (0x246ed89d),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_2 (optimization, callable.expect_into_0 (), input_1, input_2),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([input_1, input_2])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_2 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), input_1, input_2),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall2 (callable.into (), input_1.into (), input_2.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_3 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive3 (primitive) =>
							return self.optimize_procedure_primitive_3 (optimization, primitive, input_1, input_2, input_3),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
								return self.optimize_procedure_primitive_3 (optimization, primitive, input_1, input_2, input_3);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3]));
							} else {
								fail! (0x1f1758e5);
							},
						_ =>
							fail! (0x2b3e2453),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_3 (optimization, callable.expect_into_0 (), input_1, input_2, input_3),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([input_1, input_2, input_3])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_3 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), input_1, input_2, input_3),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall3 (callable.into (), input_1.into (), input_2.into (), input_3.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_4 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive4 (primitive) =>
							return self.optimize_procedure_primitive_4 (optimization, primitive, input_1, input_2, input_3, input_4),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3, input_4])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
								return self.optimize_procedure_primitive_4 (optimization, primitive, input_1, input_2, input_3, input_4);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3, input_4]));
							} else {
								fail! (0xce03eff7);
							},
						_ =>
							fail! (0x47994db4),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_4 (optimization, callable.expect_into_0 (), input_1, input_2, input_3, input_4),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([input_1, input_2, input_3, input_4])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_4 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), input_1, input_2, input_3, input_4),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall4 (callable.into (), input_1.into (), input_2.into (), input_3.into (), input_4.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_5 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::Primitive5 (primitive) =>
							return self.optimize_procedure_primitive_5 (optimization, primitive, input_1, input_2, input_3, input_4, input_5),
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
								return self.optimize_procedure_primitive_5 (optimization, primitive, input_1, input_2, input_3, input_4, input_5);
							} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, StdBox::new ([input_1, input_2, input_3, input_4, input_5]));
							} else {
								fail! (0xad4af318);
							},
						_ =>
							fail! (0x71fe0ce5),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_5 (optimization, callable.expect_into_0 (), input_1, input_2, input_3, input_4, input_5),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_5 (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), input_1, input_2, input_3, input_4, input_5),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = ExpressionForProcedureGenericCall::ProcedureCall5 (callable.into (), input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_call_n (&self, optimization : OptimizerContext, callable : Expression, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_kind (&callable) {
			match class {
				ValueKind::ProcedurePrimitive =>
					match callable.expect_into_0 () {
						ProcedurePrimitive::PrimitiveN (primitive) =>
							return self.optimize_procedure_primitive_n (optimization, primitive, inputs),
						ProcedurePrimitive::PrimitiveV (primitive) =>
							if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
								return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
							} else {
								fail! (0x5b192e69);
							},
						_ =>
							fail! (0xd891d200),
					},
				ValueKind::ProcedureExtended =>
					return self.optimize_procedure_extended_n (optimization, callable.expect_into_0 (), inputs),
				ValueKind::ProcedureNative =>
					return self.optimize_procedure_native (optimization, callable.expect_into_0 (), inputs),
				ValueKind::ProcedureLambda =>
					return self.optimize_procedure_lambda_n (optimization, StdExpectInto0::<ProcedureLambda>::expect_into_0 (callable) .internals_rc_into (), inputs),
				_ =>
					(),
			}
		}
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = ExpressionForProcedureGenericCall::ProcedureCallN (callable.into (), inputs) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	
	
	fn optimize_procedure_call_with_attributes (&self, optimization : OptimizerContext, expression : Expression, attributes : Option<ProcedureAttributes>) -> (Outcome<(OptimizerContext, Expression)>) {
		if let Some (attributes) = attributes {
			if attributes.deterministic {
				match attributes.output {
					ProcedureOutputAttributes::Constant => {
						let evaluate = {
							let inputs = self.expression_procedure_call_inputs_ref (&optimization, &expression) .unwrap ();
							self.expressions_are_all (&optimization, inputs.iter (), ExpressionClass::Constant)
						};
						if evaluate {
							return self.evaluate_to_expression (optimization, expression);
						}
					},
					_ =>
						(),
				}
			}
		}
		match self.expression_procedure_call_callable_ref (&optimization, &expression) .unwrap () {
			ExpressionProcedureCallCallableRef::Expression (_) =>
				(),
			ExpressionProcedureCallCallableRef::Primitive (_) =>
				(),
			ExpressionProcedureCallCallableRef::Extended (_) =>
				(),
			ExpressionProcedureCallCallableRef::Native (_) =>
				(),
			ExpressionProcedureCallCallableRef::Lambda (_) =>
				(),
		}
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_procedure_primitive (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let inputs_count = inputs.len ();
		match primitive {
			ProcedurePrimitive::Primitive0 (primitive) =>
				if inputs_count == 0 {
					return self.optimize_procedure_primitive_0 (optimization, primitive);
				} else {
					fail! (0x4b6f9524);
				},
			ProcedurePrimitive::Primitive1 (primitive) =>
				if inputs_count == 1 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_1 (optimization, primitive, input_1);
				} else {
					fail! (0xe2499a4e);
				},
			ProcedurePrimitive::Primitive2 (primitive) =>
				if inputs_count == 2 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_2 (optimization, primitive, input_1, input_2);
				} else {
					fail! (0x36d805da);
				},
			ProcedurePrimitive::Primitive3 (primitive) =>
				if inputs_count == 3 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_3 (optimization, primitive, input_1, input_2, input_3);
				} else {
					fail! (0x4dda0ef0);
				},
			ProcedurePrimitive::Primitive4 (primitive) =>
				if inputs_count == 4 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_4 (optimization, primitive, input_1, input_2, input_3, input_4);
				} else {
					fail! (0xc75c61bb);
				},
			ProcedurePrimitive::Primitive5 (primitive) =>
				if inputs_count == 5 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					let input_5 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_5 (optimization, primitive, input_1, input_2, input_3, input_4, input_5);
				} else {
					fail! (0x4c9891b8);
				},
			ProcedurePrimitive::PrimitiveN (primitive) =>
				return self.optimize_procedure_primitive_n (optimization, primitive, inputs),
			ProcedurePrimitive::PrimitiveV (primitive) =>
				return self.optimize_procedure_primitive_v (optimization, primitive, inputs),
			ProcedurePrimitive::Unimplemented =>
				fail_unimplemented! (0x6befd021), // OK
			ProcedurePrimitive::Unsupported =>
				fail_unimplemented! (0x6b673d0f), // OK
			ProcedurePrimitive::Reserved =>
				fail_unimplemented! (0x15fb6cc0), // OK
		}
	}
	
	
	fn optimize_procedure_primitive_0 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive0) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (primitive) .into ();
		let attributes = procedure_primitive_0_attributes (primitive);
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_primitive_1 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive1, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitive1::Functions (FunctionsPrimitive1::Call) =>
				return self.optimize_procedure_call_0 (optimization, input_1),
			ProcedurePrimitive1::Runtime (RuntimePrimitive1::ValueRaise) => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let expression = Expression::ErrorThrow (input_1.into ()) .into ();
				succeed! ((optimization, expression));
			},
			ProcedurePrimitive1::Runtime (RuntimePrimitive1::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_1 (optimization, RuntimePrimitive1::ErrorBuild.into (), input_1));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (primitive, input_1.into ()) .into ();
				let attributes = procedure_primitive_1_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			},
		};
	}
	
	
	fn optimize_procedure_primitive_2 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive2, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitive2::Functions (FunctionsPrimitive2::Call) =>
				return self.optimize_procedure_call_1 (optimization, input_1, input_2),
			ProcedurePrimitive2::Runtime (RuntimePrimitive2::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_2 (optimization, RuntimePrimitive2::ErrorBuild.into (), input_1, input_2));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (primitive, input_1.into (), input_2.into ()) .into ();
				let attributes = procedure_primitive_2_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			},
		}
	}
	
	
	fn optimize_procedure_primitive_3 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive3, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitive3::Functions (FunctionsPrimitive3::Call) =>
				return self.optimize_procedure_call_2 (optimization, input_1, input_2, input_3),
			ProcedurePrimitive3::Runtime (RuntimePrimitive3::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_3 (optimization, RuntimePrimitive3::ErrorBuild.into (), input_1, input_2, input_3));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
				let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (primitive, input_1.into (), input_2.into (), input_3.into ()) .into ();
				let attributes = procedure_primitive_3_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			},
		}
	}
	
	
	fn optimize_procedure_primitive_4 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive4, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitive4::Functions (FunctionsPrimitive4::Call) =>
				return self.optimize_procedure_call_3 (optimization, input_1, input_2, input_3, input_4),
			ProcedurePrimitive4::Runtime (RuntimePrimitive4::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_4 (optimization, RuntimePrimitive4::ErrorBuild.into (), input_1, input_2, input_3, input_4));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
				let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
				let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (primitive, input_1.into (), input_2.into (), input_3.into (), input_4.into ()) .into ();
				let attributes = procedure_primitive_4_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			}
		}
	}
	
	
	fn optimize_procedure_primitive_5 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive5, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitive5::Functions (FunctionsPrimitive5::Call) =>
				return self.optimize_procedure_call_4 (optimization, input_1, input_2, input_3, input_4, input_5),
			ProcedurePrimitive5::Runtime (RuntimePrimitive5::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_5 (optimization, RuntimePrimitive5::ErrorBuild.into (), input_1, input_2, input_3, input_4, input_5));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
				let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
				let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
				let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
				let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (primitive, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ()) .into ();
				let attributes = procedure_primitive_5_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			},
		}
	}
	
	
	fn optimize_procedure_primitive_n (&self, optimization : OptimizerContext, primitive : ProcedurePrimitiveN, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		match primitive {
			ProcedurePrimitiveN::Functions (FunctionsPrimitiveN::Call) => {
				let inputs = StdVec::from (inputs);
				let (callable, inputs) = try! (vec_explode_1n (inputs));
				return self.optimize_procedure_call (optimization, callable, inputs.into_boxed_slice ());
			},
			ProcedurePrimitiveN::Runtime (RuntimePrimitiveN::ErrorRaise) => {
				let (optimization, expression) = try! (self.optimize_procedure_primitive_n (optimization, RuntimePrimitiveN::ErrorBuild.into (), inputs));
				let expression = Expression::ErrorThrow (expression.into ()) .into ();
				succeed! ((optimization, expression));
			},
			_ => {
				let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
				let expression = ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (primitive, inputs) .into ();
				let attributes = procedure_primitive_n_attributes (primitive);
				return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
			},
		}
	}
	
	
	fn optimize_procedure_primitive_v (&self, optimization : OptimizerContext, primitive : ProcedurePrimitiveV, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let input_count = inputs.len ();
		match input_count {
			0 =>
				if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive) {
					return self.optimize_procedure_primitive_0 (optimization, primitive);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x0edccbf8);
				},
			1 =>
				if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive) {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_1 (optimization, primitive, input_1);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x47a42bf8);
				},
			2 =>
				if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_2 (optimization, primitive, input_1, input_2);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x6501bcbc);
				},
			3 =>
				if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive) {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_3 (optimization, primitive, input_1, input_2, input_3);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x397d4a45);
				},
			4 =>
				if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive) {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_4 (optimization, primitive, input_1, input_2, input_3, input_4);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x04d5e197);
				},
			5 =>
				if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive) {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					let input_5 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_5 (optimization, primitive, input_1, input_2, input_3, input_4, input_5);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0xcda2ffa1);
				},
			_ =>
				if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x852c3eb8);
				},
		}
	}
	
	
	
	
	fn optimize_procedure_extended (&self, optimization : OptimizerContext, extended : ProcedureExtended, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let input_count = inputs.len ();
		match input_count {
			0 =>
				return self.optimize_procedure_extended_0 (optimization, extended),
			1 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_1 (optimization, extended, input_1);
			},
			2 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_2 (optimization, extended, input_1, input_2);
			},
			3 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_3 (optimization, extended, input_1, input_2, input_3);
			},
			4 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_4 (optimization, extended, input_1, input_2, input_3, input_4);
			},
			5 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				let input_5 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_5 (optimization, extended, input_1, input_2, input_3, input_4, input_5);
			},
			_ =>
				return self.optimize_procedure_extended_n (optimization, extended, inputs),
		}
	}
	
	
	fn optimize_procedure_extended_0 (&self, optimization : OptimizerContext, extended : ProcedureExtended) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (extended) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_1 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (extended, input_1.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_2 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (extended, input_1.into (), input_2.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_3 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (extended, input_1.into (), input_2.into (), input_3.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_4 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (extended, input_1.into (), input_2.into (), input_3.into (), input_4.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_5 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (extended, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_extended_n (&self, optimization : OptimizerContext, extended : ProcedureExtended, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (extended, inputs) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	
	
	fn optimize_procedure_native (&self, optimization : OptimizerContext, native : ProcedureNative, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let inputs_count = inputs.len ();
		let native = native.internals_into ();
		match native {
			ProcedureNativeInternals::Native0 (native) =>
				if inputs_count == 0 {
					return self.optimize_procedure_native_0 (optimization, native);
				} else {
					fail! (0x0664f4d0);
				},
			ProcedureNativeInternals::Native1 (native) =>
				if inputs_count == 1 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					return self.optimize_procedure_native_1 (optimization, native, input_1);
				} else {
					fail! (0xce8a1f83);
				},
			ProcedureNativeInternals::Native2 (native) =>
				if inputs_count == 2 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					return self.optimize_procedure_native_2 (optimization, native, input_1, input_2);
				} else {
					fail! (0x98c15092);
				},
			ProcedureNativeInternals::Native3 (native) =>
				if inputs_count == 3 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					return self.optimize_procedure_native_3 (optimization, native, input_1, input_2, input_3);
				} else {
					fail! (0x6d40d94d);
				},
			ProcedureNativeInternals::Native4 (native) =>
				if inputs_count == 4 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					return self.optimize_procedure_native_4 (optimization, native, input_1, input_2, input_3, input_4);
				} else {
					fail! (0x12b4fab7);
				},
			ProcedureNativeInternals::Native5 (native) =>
				if inputs_count == 5 {
					let mut inputs = StdVec::from (inputs) .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					let input_4 = inputs.next () .unwrap ();
					let input_5 = inputs.next () .unwrap ();
					return self.optimize_procedure_native_5 (optimization, native, input_1, input_2, input_3, input_4, input_5);
				} else {
					fail! (0x7b45d7b6);
				},
			ProcedureNativeInternals::NativeN (native) =>
				return self.optimize_procedure_native_n (optimization, native, inputs),
		}
	}
	
	
	fn optimize_procedure_native_0 (&self, optimization : OptimizerContext, native : ProcedureNative0) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall0 (native) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_1 (&self, optimization : OptimizerContext, native : ProcedureNative1, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall1 (native, input_1.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_2 (&self, optimization : OptimizerContext, native : ProcedureNative2, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall2 (native, input_1.into (), input_2.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_3 (&self, optimization : OptimizerContext, native : ProcedureNative3, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall3 (native, input_1.into (), input_2.into (), input_3.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_4 (&self, optimization : OptimizerContext, native : ProcedureNative4, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall4 (native, input_1.into (), input_2.into (), input_3.into (), input_4.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_5 (&self, optimization : OptimizerContext, native : ProcedureNative5, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCall5 (native, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	fn optimize_procedure_native_n (&self, optimization : OptimizerContext, native : ProcedureNativeN, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = ExpressionForProcedureNativeCall::ProcedureNativeCallN (native, inputs) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	
	
	fn optimize_procedure_lambda (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let input_count = inputs.len ();
		match input_count {
			0 =>
				return self.optimize_procedure_lambda_0 (optimization, lambda),
			1 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_1 (optimization, lambda, input_1);
			},
			2 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_2 (optimization, lambda, input_1, input_2);
			},
			3 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_3 (optimization, lambda, input_1, input_2, input_3);
			},
			4 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_4 (optimization, lambda, input_1, input_2, input_3, input_4);
			},
			5 => {
				let mut inputs = StdVec::from (inputs) .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				let input_5 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_5 (optimization, lambda, input_1, input_2, input_3, input_4, input_5);
			},
			_ =>
				return self.optimize_procedure_lambda_n (optimization, lambda, inputs),
		}
	}
	
	fn optimize_procedure_lambda_0 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (lambda) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_1 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (lambda, input_1.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_2 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (lambda, input_1.into (), input_2.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_3 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (lambda, input_1.into (), input_2.into (), input_3.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_4 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (lambda, input_1.into (), input_2.into (), input_3.into (), input_4.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_5 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (lambda, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ()) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	fn optimize_procedure_lambda_n (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (lambda, inputs) .into ();
		let attributes = None;
		return self.optimize_procedure_call_with_attributes (optimization, expression, attributes);
	}
	
	
	
	
	fn expression_is_not <ExpressionRef : StdAsRef<Expression>> (&self, optimization : &OptimizerContext, expression : ExpressionRef, class : ExpressionClass) -> (bool) {
		return ! self.expression_is (optimization, expression, class);
	}
	
	
	fn expression_is <ExpressionRef : StdAsRef<Expression>> (&self, _optimization : &OptimizerContext, expression : ExpressionRef, class : ExpressionClass) -> (bool) {
		let expression = expression.as_ref ();
		match *expression {
			
			Expression::Void =>
				match class {
					ExpressionClass::Constant =>
						true,
					ExpressionClass::Value (class) =>
						VOID_VALUE.is_class (class),
					ExpressionClass::Type (primitive) =>
						type_primitive_1_evaluate_0 (primitive, &VOID_VALUE) .unwrap (),
				},
			
			Expression::Value (ref value) =>
				match class {
					ExpressionClass::Constant =>
						true,
					ExpressionClass::Value (class) =>
						value.is_class (class),
					ExpressionClass::Type (primitive) =>
						type_primitive_1_evaluate_0 (primitive, value) .unwrap (),
				},
			
			Expression::Sequence (_, _) =>
				false,
			Expression::ConditionalIf (_) =>
				false,
			Expression::ConditionalMatch (_, _) =>
				false,
			Expression::Loop (_, _, _, _) =>
				false,
			
			Expression::Lambda (_, _, _, _) =>
				false,
			
			Expression::ErrorReturn (_) =>
				false,
			Expression::ErrorCatch (_, _, _) =>
				false,
			Expression::ErrorThrow (_) =>
				false,
			
			Expression::Contexts (ref expression) =>
				match *expression {
					
					ExpressionForContexts::ContextDefine (_, _) =>
						false,
					ExpressionForContexts::ContextUpdate (_, _) =>
						false,
					ExpressionForContexts::ContextSelect (_) =>
						false,
					
					ExpressionForContexts::BindingInitialize1 (_, _) =>
						false,
					ExpressionForContexts::BindingInitializeN (_, _) =>
						false,
					ExpressionForContexts::BindingInitializeValues (_, _) =>
						false,
					ExpressionForContexts::BindingSet1 (_, _) =>
						false,
					ExpressionForContexts::BindingSetN (_, _) =>
						false,
					ExpressionForContexts::BindingSetValues (_, _) =>
						false,
					ExpressionForContexts::BindingGet1 (_) =>
						false,
					
					ExpressionForContexts::RegisterClosure (_, _) =>
						false,
					ExpressionForContexts::RegisterInitialize1 (_, _) =>
						false,
					ExpressionForContexts::RegisterInitializeN (_, _) =>
						false,
					ExpressionForContexts::RegisterInitializeValues (_, _) =>
						false,
					ExpressionForContexts::RegisterSet1 (_, _) =>
						false,
					ExpressionForContexts::RegisterSetN (_, _) =>
						false,
					ExpressionForContexts::RegisterSetValues (_, _) =>
						false,
					ExpressionForContexts::RegisterGet1 (_) =>
						false,
					
				},
			
			Expression::ProcedureGenericCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureGenericCall::ProcedureCall (_, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall0 (_) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall1 (_, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall2 (_, _, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall3 (_, _, _, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall4 (_, _, _, _, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCall5 (_, _, _, _, _, _) =>
						false,
					ExpressionForProcedureGenericCall::ProcedureCallN (_, _) =>
						false,
					
				},
			
			Expression::ProcedurePrimitiveCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall (_, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (_) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (_, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (_, _, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (_, _, _, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (_, _, _, _, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (_, _, _, _, _, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (_, _) =>
						false,
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallV (_, _) =>
						false,
					
				},
			
			Expression::ProcedureExtendedCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall (_, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (_) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (_, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (_, _, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (_, _, _, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (_, _, _, _, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (_, _, _, _, _, _) =>
						false,
					ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (_, _) =>
						false,
					
				},
			
			Expression::ProcedureLambdaCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall (_, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (_) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (_, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (_, _, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (_, _, _, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (_, _, _, _, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (_, _, _, _, _, _) =>
						false,
					ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (_, _) =>
						false,
					
				},
			
			Expression::ProcedureNativeCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureNativeCall::ProcedureNativeCall (_, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall0 (_) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall1 (_, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall2 (_, _, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall3 (_, _, _, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall4 (_, _, _, _, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCall5 (_, _, _, _, _, _) =>
						false,
					ExpressionForProcedureNativeCall::ProcedureNativeCallN (_, _) =>
						false,
					
				},
			
		}
	}
	
	
	
	
	fn expression_value_kind <ExpressionRef : StdAsRef<Expression>> (&self, expression : ExpressionRef) -> (Option<ValueKind>) {
		let expression = expression.as_ref ();
		match *expression {
			Expression::Void =>
				Some (ValueKind::Void),
			Expression::Value (ref value) =>
				Some (value.kind ()),
			_ =>
				None,
		}
	}
	
	#[ allow (dead_code) ]
	fn expression_value_class <ExpressionRef : StdAsRef<Expression>> (&self, expression : ExpressionRef) -> (Option<ValueClass>) {
		let expression = expression.as_ref ();
		match *expression {
			Expression::Void =>
				Some (ValueClass::Void),
			Expression::Value (ref value) =>
				Some (value.class ()),
			_ =>
				None,
		}
	}
	
	
	
	
	fn expressions_are_any <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (bool)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.any (|expression| self.expression_is (optimization, expression, class));
	}
	
	#[ allow (dead_code) ]
	fn expressions_are_any_not <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (bool)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.any (|expression| self.expression_is_not (optimization, expression, class));
	}
	
	#[ allow (dead_code) ]
	fn expressions_are_all <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (bool)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.all (|expression| self.expression_is (optimization, expression, class));
	}
	
	#[ allow (dead_code) ]
	fn expressions_are_all_not <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (bool)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.all (|expression| self.expression_is_not (optimization, expression, class));
	}
	
	
	#[ allow (dead_code) ]
	fn expressions_first_that <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (Option<ExpressionRef>)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.find (|expression| self.expression_is (optimization, expression, class));
	}
	
	#[ allow (dead_code) ]
	fn expressions_first_that_not <Iterator, ExpressionRef> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (Option<ExpressionRef>)
			where Iterator : iter::Iterator<Item = ExpressionRef>, ExpressionRef : StdAsRef<Expression>
	{
		let mut expressions = expressions;
		return expressions.find (|expression| self.expression_is_not (optimization, expression, class));
	}
	
	
	
	
	#[ allow (dead_code) ]
	fn expressions_retain_if_is (&self, optimization : &OptimizerContext, expressions : StdVec<Expression>, class : ExpressionClass) -> (StdVec<Expression>) {
		let mut expressions = expressions;
		expressions.retain (|expression| self.expression_is (optimization, expression, class));
		return expressions;
	}
	
	fn expressions_retain_if_is_not (&self, optimization : &OptimizerContext, expressions : StdVec<Expression>, class : ExpressionClass) -> (StdVec<Expression>) {
		let mut expressions = expressions;
		expressions.retain (|expression| self.expression_is_not (optimization, expression, class));
		return expressions;
	}
	
	
	#[ allow (dead_code) ]
	fn expressions_collect_if_is <Iterator> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (StdVec<Expression>)
			where Iterator : iter::Iterator<Item = Expression>
	{
		let expressions = expressions.filter (|expression| self.expression_is (optimization, expression, class));
		let expressions = expressions.collect ();
		return expressions;
	}
	
	fn expressions_collect_if_is_not <Iterator> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (StdVec<Expression>)
			where Iterator : iter::Iterator<Item = Expression>
	{
		let expressions = expressions.filter (|expression| self.expression_is_not (optimization, expression, class));
		let expressions = expressions.collect ();
		return expressions;
	}
	
	
	
	
	fn expression_procedure_call_callable_ref <'a, ExpressionRef : StdAsRef<Expression> + 'a> (&self, _optimization : &OptimizerContext, expression : &'a ExpressionRef) -> (Option<ExpressionProcedureCallCallableRef<'a>>) {
		let expression = expression.as_ref ();
		match *expression {
			
			Expression::Void =>
				None,
			Expression::Value (_) =>
				None,
			
			Expression::Sequence (_, _) =>
				None,
			Expression::ConditionalIf (_) =>
				None,
			Expression::ConditionalMatch (_, _) =>
				None,
			Expression::Loop (_, _, _, _) =>
				None,
			
			Expression::Lambda (_, _, _, _) =>
				None,
			
			Expression::ErrorReturn (_) =>
				None,
			Expression::ErrorCatch (_, _, _) =>
				None,
			Expression::ErrorThrow (_) =>
				None,
			
			Expression::Contexts (ref expression) =>
				match *expression {
					
					ExpressionForContexts::ContextDefine (_, _) =>
						None,
					ExpressionForContexts::ContextUpdate (_, _) =>
						None,
					ExpressionForContexts::ContextSelect (_) =>
						None,
					
					ExpressionForContexts::BindingInitialize1 (_, _) =>
						None,
					ExpressionForContexts::BindingInitializeN (_, _) =>
						None,
					ExpressionForContexts::BindingInitializeValues (_, _) =>
						None,
					ExpressionForContexts::BindingSet1 (_, _) =>
						None,
					ExpressionForContexts::BindingSetN (_, _) =>
						None,
					ExpressionForContexts::BindingSetValues (_, _) =>
						None,
					ExpressionForContexts::BindingGet1 (_) =>
						None,
					
					ExpressionForContexts::RegisterClosure (_, _) =>
						None,
					ExpressionForContexts::RegisterInitialize1 (_, _) =>
						None,
					ExpressionForContexts::RegisterInitializeN (_, _) =>
						None,
					ExpressionForContexts::RegisterInitializeValues (_, _) =>
						None,
					ExpressionForContexts::RegisterSet1 (_, _) =>
						None,
					ExpressionForContexts::RegisterSetN (_, _) =>
						None,
					ExpressionForContexts::RegisterSetValues (_, _) =>
						None,
					ExpressionForContexts::RegisterGet1 (_) =>
						None,
					
				},
			
			Expression::ProcedureGenericCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureGenericCall::ProcedureCall (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall0 (ref callable) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall1 (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall2 (ref callable, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall3 (ref callable, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall4 (ref callable, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCall5 (ref callable, _, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					ExpressionForProcedureGenericCall::ProcedureCallN (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Expression (callable)),
					
				},
			
			Expression::ProcedurePrimitiveCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable)),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (callable) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (callable, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (callable, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (callable, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (callable, _, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallV (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Primitive (callable.into ())),
					
				},
			
			Expression::ProcedureExtendedCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (ref callable) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (ref callable, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (ref callable, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (ref callable, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (ref callable, _, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Extended (callable.internals_ref ())),
					
				},
			
			Expression::ProcedureLambdaCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (ref callable) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (ref callable, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (ref callable, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (ref callable, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (ref callable, _, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Lambda (callable)),
					
				},
			
			Expression::ProcedureNativeCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureNativeCall::ProcedureNativeCall (ref callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.internals_ref () .clone ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall0 (callable) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall1 (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall2 (callable, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall3 (callable, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall4 (callable, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCall5 (callable, _, _, _, _, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					ExpressionForProcedureNativeCall::ProcedureNativeCallN (callable, _) =>
						Some (ExpressionProcedureCallCallableRef::Native (callable.into ())),
					
				},
			
		}
	}
	
	
	
	
	fn expression_procedure_call_inputs_ref <'a, ExpressionRef : StdAsRef<Expression> + 'a> (&self, _optimization : &OptimizerContext, expression : &'a ExpressionRef) -> (Option<StdBox<[&'a Expression]>>) {
		let expression = expression.as_ref ();
		match *expression {
			
			Expression::Void =>
				None,
			Expression::Value (_) =>
				None,
			
			Expression::Sequence (_, _) =>
				None,
			Expression::ConditionalIf (_) =>
				None,
			Expression::ConditionalMatch (_, _) =>
				None,
			Expression::Loop (_, _, _, _) =>
				None,
			
			Expression::Lambda (_, _, _, _) =>
				None,
			
			Expression::ErrorReturn (_) =>
				None,
			Expression::ErrorCatch (_, _, _) =>
				None,
			Expression::ErrorThrow (_) =>
				None,
			
			Expression::Contexts (ref expression) =>
				match *expression {
					
					ExpressionForContexts::ContextDefine (_, _) =>
						None,
					ExpressionForContexts::ContextUpdate (_, _) =>
						None,
					ExpressionForContexts::ContextSelect (_) =>
						None,
					
					ExpressionForContexts::BindingInitialize1 (_, _) =>
						None,
					ExpressionForContexts::BindingInitializeN (_, _) =>
						None,
					ExpressionForContexts::BindingInitializeValues (_, _) =>
						None,
					ExpressionForContexts::BindingSet1 (_, _) =>
						None,
					ExpressionForContexts::BindingSetN (_, _) =>
						None,
					ExpressionForContexts::BindingSetValues (_, _) =>
						None,
					ExpressionForContexts::BindingGet1 (_) =>
						None,
					
					ExpressionForContexts::RegisterClosure (_, _) =>
						None,
					ExpressionForContexts::RegisterInitialize1 (_, _) =>
						None,
					ExpressionForContexts::RegisterInitializeN (_, _) =>
						None,
					ExpressionForContexts::RegisterInitializeValues (_, _) =>
						None,
					ExpressionForContexts::RegisterSet1 (_, _) =>
						None,
					ExpressionForContexts::RegisterSetN (_, _) =>
						None,
					ExpressionForContexts::RegisterSetValues (_, _) =>
						None,
					ExpressionForContexts::RegisterGet1 (_) =>
						None,
					
				},
			
			Expression::ProcedureGenericCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureGenericCall::ProcedureCall (ref _callable, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedureGenericCall::ProcedureCall0 (ref _callable) =>
						Some (StdBox::new ([])),
					ExpressionForProcedureGenericCall::ProcedureCall1 (ref _callable, ref input_1) =>
						Some (StdBox::new ([input_1])),
					ExpressionForProcedureGenericCall::ProcedureCall2 (ref _callable, ref input_1, ref input_2) =>
						Some (StdBox::new ([input_1, input_2])),
					ExpressionForProcedureGenericCall::ProcedureCall3 (ref _callable, ref input_1, ref input_2, ref input_3) =>
						Some (StdBox::new ([input_1, input_2, input_3])),
					ExpressionForProcedureGenericCall::ProcedureCall4 (ref _callable, ref input_1, ref input_2, ref input_3, ref input_4) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4])),
					ExpressionForProcedureGenericCall::ProcedureCall5 (ref _callable, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
					ExpressionForProcedureGenericCall::ProcedureCallN (ref _callable, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					
				},
			
			Expression::ProcedurePrimitiveCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall (_primitive, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall0 (_primitive) =>
						Some (StdBox::new ([])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall1 (_primitive, ref input_1) =>
						Some (StdBox::new ([input_1])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall2 (_primitive, ref input_1, ref input_2) =>
						Some (StdBox::new ([input_1, input_2])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall3 (_primitive, ref input_1, ref input_2, ref input_3) =>
						Some (StdBox::new ([input_1, input_2, input_3])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall4 (_primitive, ref input_1, ref input_2, ref input_3, ref input_4) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCall5 (_primitive, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallN (_primitive, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedurePrimitiveCall::ProcedurePrimitiveCallV (_primitive, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					
				},
			
			Expression::ProcedureExtendedCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall (ref _extended, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall0 (ref _extended) =>
						Some (StdBox::new ([])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall1 (ref _extended, ref input_1) =>
						Some (StdBox::new ([input_1])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall2 (ref _extended, ref input_1, ref input_2) =>
						Some (StdBox::new ([input_1, input_2])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall3 (ref _extended, ref input_1, ref input_2, ref input_3) =>
						Some (StdBox::new ([input_1, input_2, input_3])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall4 (ref _extended, ref input_1, ref input_2, ref input_3, ref input_4) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCall5 (ref _extended, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
					ExpressionForProcedureExtendedCall::ProcedureExtendedCallN (ref _extended, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					
				},
			
			Expression::ProcedureLambdaCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall (ref _lambda, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall0 (ref _lambda) =>
						Some (StdBox::new ([])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall1 (ref _lambda, ref input_1) =>
						Some (StdBox::new ([input_1])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall2 (ref _lambda, ref input_1, ref input_2) =>
						Some (StdBox::new ([input_1, input_2])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall3 (ref _lambda, ref input_1, ref input_2, ref input_3) =>
						Some (StdBox::new ([input_1, input_2, input_3])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall4 (ref _lambda, ref input_1, ref input_2, ref input_3, ref input_4) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCall5 (ref _lambda, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
					ExpressionForProcedureLambdaCall::ProcedureLambdaCallN (ref _lambda, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					
				},
			
			Expression::ProcedureNativeCall (ref expression) =>
				match *expression {
					
					ExpressionForProcedureNativeCall::ProcedureNativeCall (ref _native, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					ExpressionForProcedureNativeCall::ProcedureNativeCall0 (ref _native) =>
						Some (StdBox::new ([])),
					ExpressionForProcedureNativeCall::ProcedureNativeCall1 (ref _native, ref input_1) =>
						Some (StdBox::new ([input_1])),
					ExpressionForProcedureNativeCall::ProcedureNativeCall2 (ref _native, ref input_1, ref input_2) =>
						Some (StdBox::new ([input_1, input_2])),
					ExpressionForProcedureNativeCall::ProcedureNativeCall3 (ref _native, ref input_1, ref input_2, ref input_3) =>
						Some (StdBox::new ([input_1, input_2, input_3])),
					ExpressionForProcedureNativeCall::ProcedureNativeCall4 (ref _native, ref input_1, ref input_2, ref input_3, ref input_4) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4])),
					ExpressionForProcedureNativeCall::ProcedureNativeCall5 (ref _native, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
						Some (StdBox::new ([input_1, input_2, input_3, input_4, input_5])),
					ExpressionForProcedureNativeCall::ProcedureNativeCallN (ref _native, ref inputs) =>
						Some (boxed_slice_to_ref (inputs)),
					
				},
			
		}
	}
	
	
	
	
	#[ allow (dead_code) ]
	fn expression_value_ref <'a, ExpressionRef : StdAsRef<Expression> + 'a> (&self, _optimization : &OptimizerContext, expression : &'a ExpressionRef) -> (Option<&'a Value>) {
		let expression = expression.as_ref ();
		match *expression {
			Expression::Void =>
				Some (&VOID_VALUE),
			Expression::Value (ref value) =>
				Some (value),
			_ =>
				None,
		}
	}
	
	#[ allow (dead_code) ]
	fn expressions_values_ref <'a, ExpressionRef : StdAsRef<Expression> + 'a> (&self, optimization : &OptimizerContext, expressions : &'a [ExpressionRef]) -> (StdBox<[Option<&'a Value>]>) {
		return vec_map! (expressions.iter (), expression, self.expression_value_ref (optimization, expression)) .into_boxed_slice ();
	}
	
	
	
	
	fn evaluate_to_value (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Value)>) {
		let output = {
			let mut evaluation = optimization.evaluator.fork_0 ();
			try! (evaluation.evaluate (&expression))
		};
		succeed! ((optimization, output));
	}
	
	fn evaluate_to_expression (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, output) = try! (self.evaluate_to_value (optimization, expression));
		return self.optimize_value (optimization, output);
	}
	
	
	
	
}




#[ derive (Debug) ]
struct OptimizerContext {
	evaluator : Evaluator,
}


impl OptimizerContext {
	
	fn new () -> (OptimizerContext) {
		return OptimizerContext {
				evaluator : Evaluator::new (),
			};
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub(crate) enum ExpressionClass {
	
	Constant,
	Value (ValueClass),
	Type (TypePrimitive1),
	
}




#[ derive (Clone) ]
pub(crate) enum ExpressionProcedureCallCallableRef <'a> {
	
	Expression (&'a Expression),
	Primitive (ProcedurePrimitive),
	Extended (&'a ProcedureExtendedInternals),
	Native (ProcedureNativeInternals),
	Lambda (&'a LambdaInternals),
	
}

