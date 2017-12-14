

#![ allow (unused_imports, unused_variables, dead_code) ]




use super::constants::exports::*;
use super::contexts::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::expressions::exports::*;
use super::extended_procedures::exports::*;
use super::lambdas::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use std::iter;




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




#[ derive (Clone, Debug) ]
pub struct Optimizer {}


impl Optimizer {
	
	
	
	
	pub fn new () -> (Optimizer) {
		return Optimizer {};
	}
	
	
	
	
	pub fn optimize (&self, expression : Expression) -> (Outcome<Expression>) {
		let optimization = OptimizerContext::new ();
		let (_optimization, expression) = try! (self.optimize_0 (optimization, expression));
		succeed! (expression);
	}
	
	pub fn optimize_vec (&self, expressions : ExpressionVec) -> (Outcome<ExpressionVec>) {
		let optimization = OptimizerContext::new ();
		let (_optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		succeed! (expressions);
	}
	
	
	
	/*
	fn optimize_0 (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		match self.optimize_0_0 (optimization.clone (), expression.clone ()) {
			outcome @ Ok (_) =>
				return outcome,
			Err (error) =>
				if error.code == 0x417a888c {
					succeed! ((optimization, expression));
				} else {
					return Err (error);
				},
		}
	}
	*/
	
	fn optimize_0 (&self, optimization : OptimizerContext, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
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
			
			Expression::ContextDefine (identifier, expression) =>
				return self.optimize_context_define (optimization, identifier, *expression),
			Expression::ContextUpdate (identifier, expression) =>
				return self.optimize_context_update (optimization, identifier, *expression),
			Expression::ContextSelect (identifier) =>
				return self.optimize_context_select (optimization, identifier),
			
			Expression::BindingInitialize1 (binding, expression) =>
				return self.optimize_binding_initialize_1 (optimization, binding, *expression),
			Expression::BindingInitializeN (initializers, parallel) =>
				return self.optimize_binding_initialize_n (optimization, initializers, parallel),
			Expression::BindingInitializeValues (bindings, expression) =>
				return self.optimize_binding_initialize_values (optimization, bindings, *expression),
			Expression::BindingSet1 (binding, expression) =>
				return self.optimize_binding_set_1 (optimization, binding, *expression),
			Expression::BindingSetN (initializers, parallel) =>
				return self.optimize_binding_set_n (optimization, initializers, parallel),
			Expression::BindingSetValues (bindings, expression) =>
				return self.optimize_binding_set_values (optimization, bindings, *expression),
			Expression::BindingGet1 (binding) =>
				return self.optimize_binding_get_1 (optimization, binding),
			
			Expression::RegisterClosure (expression, borrows) =>
				return self.optimize_register_closure (optimization, *expression, borrows),
			Expression::RegisterInitialize1 (index, expression) =>
				return self.optimize_register_initialize_1 (optimization, index, *expression),
			Expression::RegisterInitializeN (initializers, parallel) =>
				return self.optimize_register_initialize_n (optimization, initializers, parallel),
			Expression::RegisterInitializeValues (indices, expression) =>
				return self.optimize_register_initialize_values (optimization, indices, *expression),
			Expression::RegisterSet1 (index, expression) =>
				return self.optimize_register_set_1 (optimization, index, *expression),
			Expression::RegisterSetN (initializers, parallel) =>
				return self.optimize_register_set_n (optimization, initializers, parallel),
			Expression::RegisterSetValues (indices, expression) =>
				return self.optimize_register_set_values (optimization, indices, *expression),
			Expression::RegisterGet1 (index) =>
				return self.optimize_register_get_1 (optimization, index),
			
			Expression::Lambda (lambda, expression, registers_closure, registers_local) =>
				return self.optimize_lambda_create (optimization, lambda, *expression, registers_closure, registers_local),
			
			Expression::ProcedureCall (callable, inputs) =>
				return self.optimize_procedure_call (optimization, *callable, inputs),
			Expression::ProcedureCall0 (callable) =>
				return self.optimize_procedure_call_0 (optimization, *callable),
			Expression::ProcedureCall1 (callable, input_1) =>
				return self.optimize_procedure_call_1 (optimization, *callable, *input_1),
			Expression::ProcedureCall2 (callable, input_1, input_2) =>
				return self.optimize_procedure_call_2 (optimization, *callable, *input_1, *input_2),
			Expression::ProcedureCall3 (callable, input_1, input_2, input_3) =>
				return self.optimize_procedure_call_3 (optimization, *callable, *input_1, *input_2, *input_3),
			Expression::ProcedureCall4 (callable, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_call_4 (optimization, *callable, *input_1, *input_2, *input_3, *input_4),
			Expression::ProcedureCall5 (callable, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_call_5 (optimization, *callable, *input_1, *input_2, *input_3, *input_4, *input_5),
			Expression::ProcedureCallN (callable, inputs) =>
				return self.optimize_procedure_call_n (optimization, *callable, inputs),
			
			Expression::ProcedurePrimitiveCall (primitive, inputs) =>
				return self.optimize_procedure_primitive (optimization, primitive, inputs),
			Expression::ProcedurePrimitiveCall0 (primitive) =>
				return self.optimize_procedure_primitive_0 (optimization, primitive),
			Expression::ProcedurePrimitiveCall1 (primitive, input_1) =>
				return self.optimize_procedure_primitive_1 (optimization, primitive, *input_1),
			Expression::ProcedurePrimitiveCall2 (primitive, input_1, input_2) =>
				return self.optimize_procedure_primitive_2 (optimization, primitive, *input_1, *input_2),
			Expression::ProcedurePrimitiveCall3 (primitive, input_1, input_2, input_3) =>
				return self.optimize_procedure_primitive_3 (optimization, primitive, *input_1, *input_2, *input_3),
			Expression::ProcedurePrimitiveCall4 (primitive, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_primitive_4 (optimization, primitive, *input_1, *input_2, *input_3, *input_4),
			Expression::ProcedurePrimitiveCall5 (primitive, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_primitive_5 (optimization, primitive, *input_1, *input_2, *input_3, *input_4, *input_5),
			Expression::ProcedurePrimitiveCallN (primitive, inputs) =>
				return self.optimize_procedure_primitive_n (optimization, primitive, inputs),
			Expression::ProcedurePrimitiveCallV (primitive, inputs) =>
				return self.optimize_procedure_primitive_v (optimization, primitive, inputs),
			
			Expression::ProcedureExtendedCall (extended, inputs) =>
				return self.optimize_procedure_extended (optimization, extended, inputs),
			Expression::ProcedureExtendedCall0 (extended) =>
				return self.optimize_procedure_extended_0 (optimization, extended),
			Expression::ProcedureExtendedCall1 (extended, input_1) =>
				return self.optimize_procedure_extended_1 (optimization, extended, *input_1),
			Expression::ProcedureExtendedCall2 (extended, input_1, input_2) =>
				return self.optimize_procedure_extended_2 (optimization, extended, *input_1, *input_2),
			Expression::ProcedureExtendedCall3 (extended, input_1, input_2, input_3) =>
				return self.optimize_procedure_extended_3 (optimization, extended, *input_1, *input_2, *input_3),
			Expression::ProcedureExtendedCall4 (extended, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_extended_4 (optimization, extended, *input_1, *input_2, *input_3, *input_4),
			Expression::ProcedureExtendedCall5 (extended, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_extended_5 (optimization, extended, *input_1, *input_2, *input_3, *input_4, *input_5),
			Expression::ProcedureExtendedCallN (extended, inputs) =>
				return self.optimize_procedure_extended_n (optimization, extended, inputs),
			
			Expression::ProcedureLambdaCall (lambda, inputs) =>
				return self.optimize_procedure_lambda (optimization, lambda, inputs),
			Expression::ProcedureLambdaCall0 (lambda) =>
				return self.optimize_procedure_lambda_0 (optimization, lambda),
			Expression::ProcedureLambdaCall1 (lambda, input_1) =>
				return self.optimize_procedure_lambda_1 (optimization, lambda, *input_1),
			Expression::ProcedureLambdaCall2 (lambda, input_1, input_2) =>
				return self.optimize_procedure_lambda_2 (optimization, lambda, *input_1, *input_2),
			Expression::ProcedureLambdaCall3 (lambda, input_1, input_2, input_3) =>
				return self.optimize_procedure_lambda_3 (optimization, lambda, *input_1, *input_2, *input_3),
			Expression::ProcedureLambdaCall4 (lambda, input_1, input_2, input_3, input_4) =>
				return self.optimize_procedure_lambda_4 (optimization, lambda, *input_1, *input_2, *input_3, *input_4),
			Expression::ProcedureLambdaCall5 (lambda, input_1, input_2, input_3, input_4, input_5) =>
				return self.optimize_procedure_lambda_5 (optimization, lambda, *input_1, *input_2, *input_3, *input_4, *input_5),
			Expression::ProcedureLambdaCallN (lambda, inputs) =>
				return self.optimize_procedure_lambda_n (optimization, lambda, inputs),
			
		};
	}
	
	
	
	
	fn optimize_0_box (&self, optimization : OptimizerContext, expression : StdBox<Expression>) -> (Outcome<(OptimizerContext, StdBox<Expression>)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, *expression));
		let expression = StdBox::new (expression);
		succeed! ((optimization, expression));
	}
	
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
		if value.is (ValueClass::Void) {
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
					let mut expressions = self.expressions_retain_if_is_not (&optimization, expressions, ExpressionClass::Type (TypePrimitive1::IsTrue));
					if ! expressions.is_empty () {
						expressions.push (last);
						if self.expressions_are_any (&optimization, &expressions, ExpressionClass::Type (TypePrimitive1::IsFalse)) {
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
					let mut expressions = self.expressions_retain_if_is_not (&optimization, expressions, ExpressionClass::Type (TypePrimitive1::IsFalse));
					if ! expressions.is_empty () {
						expressions.push (last);
						if self.expressions_are_any (&optimization, &expressions, ExpressionClass::Type (TypePrimitive1::IsTrue)) {
							Expression::Value (TRUE_VALUE)
						} else {
							Expression::Sequence (operator, expressions.into_boxed_slice ())
						}
					} else {
						last
					}
				} else {
					Expression::Value (FALSE_VALUE)
				};
				succeed! ((optimization, expression));
			}
		}
	}
	
	
	
	
	fn optimize_conditional_if (&self, optimization : OptimizerContext, clauses : StdBox<[(Option<(Expression, bool)>, Option<Expression>)]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, clauses) = try! (self.optimize_conditional_if_clauses (optimization, clauses));
		let expression = Expression::ConditionalIf (clauses);
		succeed! ((optimization, expression));
	}
	
	fn optimize_conditional_if_clauses (&self, optimization : OptimizerContext, clauses : StdBox<[(Option<(Expression, bool)>, Option<Expression>)]>) -> (Outcome<(OptimizerContext, StdBox<[(Option<(Expression, bool)>, Option<Expression>)]>)>) {
		let (guards, expressions) = vec_unzip_2 (clauses.to_vec ());
		let (optimization, guards) = try! (self.optimize_0_vec_transform (optimization, guards,
				|optimization, clause| {
					if let Some ((guard, negated)) = clause {
						let (optimization, guard) = try! (self.optimize_0 (optimization, guard));
						let clause = Some ((guard, negated));
						succeed! ((optimization, clause));
					} else {
						succeed! ((optimization, None));
					}
				}));
		let (optimization, expressions) = try! (self.optimize_0_vec_transform (optimization, expressions,
				|optimization, expression| {
					return self.optimize_0_option (optimization, expression)
				}));
		let clauses = vec_zip_2 (guards, expressions);
		succeed! ((optimization, clauses.into_boxed_slice ()));
	}
	
	
	fn optimize_conditional_match (&self, optimization : OptimizerContext, actual : Expression, clauses : StdBox<[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, actual, clauses) = try! (self.optimize_conditional_match_clauses (optimization, actual, clauses));
		let expression = Expression::ConditionalMatch (actual.into (), clauses);
		succeed! ((optimization, expression));
	}
	
	fn optimize_conditional_match_clauses (&self, optimization : OptimizerContext, actual : Expression, clauses : StdBox<[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]>) -> (Outcome<(OptimizerContext, Expression, StdBox<[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]>)>) {
		let (optimization, actual) = try! (self.optimize_0 (optimization, actual));
		let (guards, expressions) = vec_unzip_2 (clauses.to_vec ());
		let (optimization, expressions) = try! (self.optimize_0_vec_transform (optimization, expressions,
				|optimization, expression| {
					return self.optimize_0_option (optimization, expression)
				}));
		let clauses = vec_zip_2 (guards, expressions);
		succeed! ((optimization, actual, clauses.into_boxed_slice ()));
	}
	
	
	
	
	fn optimize_loop (&self, optimization : OptimizerContext, initialize : Option<Expression>, update : Option<Expression>, body : Option<Expression>, clauses : StdBox<[(Option<(Expression, bool)>, Option<Expression>)]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, initialize) = try! (self.optimize_0_option (optimization, initialize));
		let (optimization, update) = try! (self.optimize_0_option (optimization, update));
		let (optimization, body) = try! (self.optimize_0_option (optimization, body));
		let (optimization, clauses) = try! (self.optimize_conditional_if_clauses (optimization, clauses));
		let expression = Expression::Loop (option_box_new (initialize), option_box_new (update), option_box_new (body), clauses);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_context_define (&self, optimization : OptimizerContext, identifier : Symbol, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::ContextDefine (identifier, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_context_update (&self, optimization : OptimizerContext, identifier : Symbol, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::ContextUpdate (identifier, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_context_select (&self, optimization : OptimizerContext, identifier : Symbol) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = Expression::ContextSelect (identifier);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_binding_initialize_1 (&self, optimization : OptimizerContext, binding : Binding, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::BindingInitialize1 (binding, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_initialize_n (&self, optimization : OptimizerContext, initializers : StdBox<[(Binding, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (bindings, expressions) = vec_unzip_2 (initializers.to_vec ());
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (bindings, expressions) .into_boxed_slice ();
		let expression = Expression::BindingInitializeN (initializers, parallel);
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_initialize_values (&self, optimization : OptimizerContext, bindings : StdBox<[Binding]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::BindingInitializeValues (bindings, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_1 (&self, optimization : OptimizerContext, binding : Binding, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::BindingSet1 (binding, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_n (&self, optimization : OptimizerContext, initializers : StdBox<[(Binding, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (bindings, expressions) = vec_unzip_2 (initializers.to_vec ());
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (bindings, expressions) .into_boxed_slice ();
		let expression = Expression::BindingSetN (initializers, parallel);
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_set_values (&self, optimization : OptimizerContext, bindings : StdBox<[Binding]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::BindingSetValues (bindings, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_binding_get_1 (&self, optimization : OptimizerContext, binding : Binding) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = if binding.is_immutable () {
			let value = try! (binding.get ());
			Expression::Value (value)
		} else {
			Expression::BindingGet1 (binding)
		};
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_register_closure (&self, optimization : OptimizerContext, expression : Expression, borrows : StdBox<[RegistersBindingTemplate]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::RegisterClosure (expression.into (), borrows);
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_1 (&self, optimization : OptimizerContext, index : usize, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::RegisterInitialize1 (index, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_n (&self, optimization : OptimizerContext, initializers : StdBox<[(usize, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (indices, expressions) = vec_unzip_2 (initializers.to_vec ());
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (indices, expressions) .into_boxed_slice ();
		let expression = Expression::RegisterInitializeN (initializers, parallel);
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_initialize_values (&self, optimization : OptimizerContext, indices : StdBox<[usize]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::RegisterInitializeValues (indices, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_1 (&self, optimization : OptimizerContext, index : usize, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::RegisterSet1 (index, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_n (&self, optimization : OptimizerContext, initializers : StdBox<[(usize, Expression)]>, parallel : bool) -> (Outcome<(OptimizerContext, Expression)>) {
		let (indices, expressions) = vec_unzip_2 (initializers.to_vec ());
		let (optimization, expressions) = try! (self.optimize_0_vec (optimization, expressions));
		let initializers = vec_zip_2 (indices, expressions) .into_boxed_slice ();
		let expression = Expression::RegisterSetN (initializers, parallel);
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_set_values (&self, optimization : OptimizerContext, indices : StdBox<[usize]>, expression : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::RegisterSetValues (indices, expression.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_register_get_1 (&self, optimization : OptimizerContext, index : usize) -> (Outcome<(OptimizerContext, Expression)>) {
		let expression = Expression::RegisterGet1 (index);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_lambda_create (&self, optimization : OptimizerContext, template : LambdaTemplate, expression : Expression, registers_closure : StdBox<[RegistersBindingTemplate]>, registers_local : StdBox<[RegistersBindingTemplate]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, expression) = try! (self.optimize_0 (optimization, expression));
		let expression = Expression::Lambda (template, expression.into (), registers_closure, registers_local);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_procedure_call (&self, optimization : OptimizerContext, callable : Expression, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					return self.optimize_procedure_primitive (optimization, callable.into (), inputs),
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended (optimization, callable.into (), inputs),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), inputs),
				_ =>
					(),
			}
		}
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = Expression::ProcedureCall (callable.into (), inputs);
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_0 (&self, optimization : OptimizerContext, callable : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_0 (optimization, callable.into ()),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_0 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into ()),
				_ =>
					(),
			}
		}
		let expression = Expression::ProcedureCall0 (callable.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_1 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_1 (optimization, callable.into (), input_1),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_1 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), input_1),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = Expression::ProcedureCall1 (callable.into (), input_1.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_2 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_2 (optimization, callable.into (), input_1, input_2),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_2 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), input_1, input_2),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = Expression::ProcedureCall2 (callable.into (), input_1.into (), input_2.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_3 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_3 (optimization, callable.into (), input_1, input_2, input_3),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_3 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), input_1, input_2, input_3),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = Expression::ProcedureCall3 (callable.into (), input_1.into (), input_2.into (), input_3.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_4 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_4 (optimization, callable.into (), input_1, input_2, input_3, input_4),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_4 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), input_1, input_2, input_3, input_4),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = Expression::ProcedureCall4 (callable.into (), input_1.into (), input_2.into (), input_3.into (), input_4.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_5 (&self, optimization : OptimizerContext, callable : Expression, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_5 (optimization, callable.into (), input_1, input_2, input_3, input_4, input_5),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_5 (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), input_1, input_2, input_3, input_4, input_5),
				_ =>
					(),
			}
		}
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = Expression::ProcedureCall5 (callable.into (), input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_call_n (&self, optimization : OptimizerContext, callable : Expression, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, callable) = try! (self.optimize_0 (optimization, callable));
		if let Some (class) = self.expression_value_class (&callable) {
			match class {
				ValueClass::ProcedurePrimitive =>
					match callable.into () {
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
				ValueClass::ProcedureExtended =>
					return self.optimize_procedure_extended_n (optimization, callable.into (), inputs),
				ValueClass::ProcedureLambda =>
					return self.optimize_procedure_lambda_n (optimization, StdInto::<ProcedureLambda>::into (callable) .internals_rc_into (), inputs),
				_ =>
					(),
			}
		}
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = Expression::ProcedureCallN (callable.into (), inputs);
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
					let mut inputs = inputs.to_vec () .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_1 (optimization, primitive, input_1);
				} else {
					fail! (0xe2499a4e);
				},
			ProcedurePrimitive::Primitive2 (primitive) =>
				if inputs_count == 2 {
					let mut inputs = inputs.to_vec () .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_2 (optimization, primitive, input_1, input_2);
				} else {
					fail! (0x36d805da);
				},
			ProcedurePrimitive::Primitive3 (primitive) =>
				if inputs_count == 3 {
					let mut inputs = inputs.to_vec () .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					let input_2 = inputs.next () .unwrap ();
					let input_3 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_3 (optimization, primitive, input_1, input_2, input_3);
				} else {
					fail! (0x4dda0ef0);
				},
			ProcedurePrimitive::Primitive4 (primitive) =>
				if inputs_count == 4 {
					let mut inputs = inputs.to_vec () .into_iter ();
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
					let mut inputs = inputs.to_vec () .into_iter ();
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
		let expression = Expression::ProcedurePrimitiveCall0 (primitive);
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_1 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive1, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = Expression::ProcedurePrimitiveCall1 (primitive, input_1.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_2 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive2, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = Expression::ProcedurePrimitiveCall2 (primitive, input_1.into (), input_2.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_3 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive3, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = Expression::ProcedurePrimitiveCall3 (primitive, input_1.into (), input_2.into (), input_3.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_4 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive4, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = Expression::ProcedurePrimitiveCall4 (primitive, input_1.into (), input_2.into (), input_3.into (), input_4.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_5 (&self, optimization : OptimizerContext, primitive : ProcedurePrimitive5, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = Expression::ProcedurePrimitiveCall5 (primitive, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_primitive_n (&self, optimization : OptimizerContext, primitive : ProcedurePrimitiveN, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = Expression::ProcedurePrimitiveCallN (primitive, inputs);
		succeed! ((optimization, expression));
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
					let mut inputs = inputs.to_vec () .into_iter ();
					let input_1 = inputs.next () .unwrap ();
					return self.optimize_procedure_primitive_1 (optimization, primitive, input_1);
				} else if let Some (primitive) = procedure_primitive_v_alternative_n (primitive) {
					return self.optimize_procedure_primitive_n (optimization, primitive, inputs);
				} else {
					fail! (0x47a42bf8);
				},
			2 =>
				if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive) {
					let mut inputs = inputs.to_vec () .into_iter ();
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
					let mut inputs = inputs.to_vec () .into_iter ();
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
					let mut inputs = inputs.to_vec () .into_iter ();
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
					let mut inputs = inputs.to_vec () .into_iter ();
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
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_1 (optimization, extended, input_1);
			},
			2 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_2 (optimization, extended, input_1, input_2);
			},
			3 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_3 (optimization, extended, input_1, input_2, input_3);
			},
			4 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				return self.optimize_procedure_extended_4 (optimization, extended, input_1, input_2, input_3, input_4);
			},
			5 => {
				let mut inputs = inputs.to_vec () .into_iter ();
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
		let expression = Expression::ProcedureExtendedCall0 (extended);
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_1 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = Expression::ProcedureExtendedCall1 (extended, input_1.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_2 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = Expression::ProcedureExtendedCall2 (extended, input_1.into (), input_2.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_3 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = Expression::ProcedureExtendedCall3 (extended, input_1.into (), input_2.into (), input_3.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_4 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = Expression::ProcedureExtendedCall4 (extended, input_1.into (), input_2.into (), input_3.into (), input_4.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_5 (&self, optimization : OptimizerContext, extended : ProcedureExtended, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = Expression::ProcedureExtendedCall5 (extended, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ());
		succeed! ((optimization, expression));
	}
	
	
	fn optimize_procedure_extended_n (&self, optimization : OptimizerContext, extended : ProcedureExtended, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = Expression::ProcedureExtendedCallN (extended, inputs);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn optimize_procedure_lambda (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let input_count = inputs.len ();
		match input_count {
			0 =>
				return self.optimize_procedure_lambda_0 (optimization, lambda),
			1 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_1 (optimization, lambda, input_1);
			},
			2 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_2 (optimization, lambda, input_1, input_2);
			},
			3 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_3 (optimization, lambda, input_1, input_2, input_3);
			},
			4 => {
				let mut inputs = inputs.to_vec () .into_iter ();
				let input_1 = inputs.next () .unwrap ();
				let input_2 = inputs.next () .unwrap ();
				let input_3 = inputs.next () .unwrap ();
				let input_4 = inputs.next () .unwrap ();
				return self.optimize_procedure_lambda_4 (optimization, lambda, input_1, input_2, input_3, input_4);
			},
			5 => {
				let mut inputs = inputs.to_vec () .into_iter ();
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
		let expression = Expression::ProcedureLambdaCall0 (lambda);
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_1 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let expression = Expression::ProcedureLambdaCall1 (lambda, input_1.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_2 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let expression = Expression::ProcedureLambdaCall2 (lambda, input_1.into (), input_2.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_3 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let expression = Expression::ProcedureLambdaCall3 (lambda, input_1.into (), input_2.into (), input_3.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_4 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let expression = Expression::ProcedureLambdaCall4 (lambda, input_1.into (), input_2.into (), input_3.into (), input_4.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_5 (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, input_1 : Expression, input_2 : Expression, input_3 : Expression, input_4 : Expression, input_5 : Expression) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, input_1) = try! (self.optimize_0 (optimization, input_1));
		let (optimization, input_2) = try! (self.optimize_0 (optimization, input_2));
		let (optimization, input_3) = try! (self.optimize_0 (optimization, input_3));
		let (optimization, input_4) = try! (self.optimize_0 (optimization, input_4));
		let (optimization, input_5) = try! (self.optimize_0 (optimization, input_5));
		let expression = Expression::ProcedureLambdaCall5 (lambda, input_1.into (), input_2.into (), input_3.into (), input_4.into (), input_5.into ());
		succeed! ((optimization, expression));
	}
	
	fn optimize_procedure_lambda_n (&self, optimization : OptimizerContext, lambda : StdRc<LambdaInternals>, inputs : StdBox<[Expression]>) -> (Outcome<(OptimizerContext, Expression)>) {
		let (optimization, inputs) = try! (self.optimize_0_slice (optimization, inputs));
		let expression = Expression::ProcedureLambdaCallN (lambda, inputs);
		succeed! ((optimization, expression));
	}
	
	
	
	
	fn expression_is_not (&self, optimization : &OptimizerContext, expression : &Expression, class : ExpressionClass) -> (bool) {
		return ! self.expression_is (optimization, expression, class);
	}
	
	
	fn expression_is (&self, _optimization : &OptimizerContext, expression : &Expression, class : ExpressionClass) -> (bool) {
		match *expression {
			
			Expression::Void =>
				match class {
					ExpressionClass::Constant =>
						true,
					ExpressionClass::Value (class) =>
						VOID_VALUE.is (class),
					ExpressionClass::Type (primitive) =>
						type_primitive_1_evaluate_0 (primitive, &VOID_VALUE) .unwrap (), // PANIC
				},
			
			Expression::Value (ref value) =>
				match class {
					ExpressionClass::Constant =>
						true,
					ExpressionClass::Value (class) =>
						value.is (class),
					ExpressionClass::Type (primitive) =>
						type_primitive_1_evaluate_0 (primitive, value) .unwrap (), // PANIC
				},
			
			Expression::Sequence (_, _) =>
				false,
			Expression::ConditionalIf (_) =>
				false,
			Expression::ConditionalMatch (_, _) =>
				false,
			Expression::Loop (_, _, _, _) =>
				false,
			
			Expression::ContextDefine (_, _) =>
				false,
			Expression::ContextUpdate (_, _) =>
				false,
			Expression::ContextSelect (_) =>
				false,
			
			Expression::BindingInitialize1 (_, _) =>
				false,
			Expression::BindingInitializeN (_, _) =>
				false,
			Expression::BindingInitializeValues (_, _) =>
				false,
			Expression::BindingSet1 (_, _) =>
				false,
			Expression::BindingSetN (_, _) =>
				false,
			Expression::BindingSetValues (_, _) =>
				false,
			Expression::BindingGet1 (_) =>
				false,
			Expression::RegisterClosure (_, _) =>
				false,
			Expression::RegisterInitialize1 (_, _) =>
				false,
			Expression::RegisterInitializeN (_, _) =>
				false,
			Expression::RegisterInitializeValues (_, _) =>
				false,
			Expression::RegisterSet1 (_, _) =>
				false,
			Expression::RegisterSetN (_, _) =>
				false,
			Expression::RegisterSetValues (_, _) =>
				false,
			Expression::RegisterGet1 (_) =>
				false,
			
			Expression::Lambda (_, _, _, _) =>
				false,
			
			Expression::ProcedureCall (_, _) =>
				false,
			Expression::ProcedureCall0 (_) =>
				false,
			Expression::ProcedureCall1 (_, _) =>
				false,
			Expression::ProcedureCall2 (_, _, _) =>
				false,
			Expression::ProcedureCall3 (_, _, _, _) =>
				false,
			Expression::ProcedureCall4 (_, _, _, _, _) =>
				false,
			Expression::ProcedureCall5 (_, _, _, _, _, _) =>
				false,
			Expression::ProcedureCallN (_, _) =>
				false,
			
			Expression::ProcedurePrimitiveCall (_, _) =>
				false,
			Expression::ProcedurePrimitiveCall0 (_) =>
				false,
			Expression::ProcedurePrimitiveCall1 (_, _) =>
				false,
			Expression::ProcedurePrimitiveCall2 (_, _, _) =>
				false,
			Expression::ProcedurePrimitiveCall3 (_, _, _, _) =>
				false,
			Expression::ProcedurePrimitiveCall4 (_, _, _, _, _) =>
				false,
			Expression::ProcedurePrimitiveCall5 (_, _, _, _, _, _) =>
				false,
			Expression::ProcedurePrimitiveCallN (_, _) =>
				false,
			Expression::ProcedurePrimitiveCallV (_, _) =>
				false,
			
			Expression::ProcedureExtendedCall (_, _) =>
				false,
			Expression::ProcedureExtendedCall0 (_) =>
				false,
			Expression::ProcedureExtendedCall1 (_, _) =>
				false,
			Expression::ProcedureExtendedCall2 (_, _, _) =>
				false,
			Expression::ProcedureExtendedCall3 (_, _, _, _) =>
				false,
			Expression::ProcedureExtendedCall4 (_, _, _, _, _) =>
				false,
			Expression::ProcedureExtendedCall5 (_, _, _, _, _, _) =>
				false,
			Expression::ProcedureExtendedCallN (_, _) =>
				false,
			
			Expression::ProcedureLambdaCall (_, _) =>
				false,
			Expression::ProcedureLambdaCall0 (_) =>
				false,
			Expression::ProcedureLambdaCall1 (_, _) =>
				false,
			Expression::ProcedureLambdaCall2 (_, _, _) =>
				false,
			Expression::ProcedureLambdaCall3 (_, _, _, _) =>
				false,
			Expression::ProcedureLambdaCall4 (_, _, _, _, _) =>
				false,
			Expression::ProcedureLambdaCall5 (_, _, _, _, _, _) =>
				false,
			Expression::ProcedureLambdaCallN (_, _) =>
				false,
			
		}
	}
	
	
	
	
	fn expression_value_class (&self, expression : &Expression) -> (Option<ValueClass>) {
		match *expression {
			Expression::Void =>
				Some (ValueClass::Void),
			Expression::Value (ref value) =>
				Some (value.class ()),
			_ =>
				None,
		}
	}
	
	
	
	
	fn expressions_are_any (&self, optimization : &OptimizerContext, expressions : &Vec<Expression>, class : ExpressionClass) -> (bool) {
		return expressions.iter () .any (|expression| self.expression_is (optimization, expression, class));
	}
	
	fn expressions_are_any_not (&self, optimization : &OptimizerContext, expressions : &Vec<Expression>, class : ExpressionClass) -> (bool) {
		return expressions.iter () .any (|expression| self.expression_is_not (optimization, expression, class));
	}
	
	fn expressions_are_all (&self, optimization : &OptimizerContext, expressions : &Vec<Expression>, class : ExpressionClass) -> (bool) {
		return expressions.iter () .all (|expression| self.expression_is (optimization, expression, class));
	}
	
	fn expressions_are_all_not (&self, optimization : &OptimizerContext, expressions : &Vec<Expression>, class : ExpressionClass) -> (bool) {
		return expressions.iter () .all (|expression| self.expression_is_not (optimization, expression, class));
	}
	
	
	
	
	fn expressions_retain_if_is (&self, optimization : &OptimizerContext, expressions : Vec<Expression>, class : ExpressionClass) -> (Vec<Expression>) {
		let mut expressions = expressions;
		expressions.retain (|expression| self.expression_is (optimization, expression, class));
		return expressions;
	}
	
	fn expressions_retain_if_is_not (&self, optimization : &OptimizerContext, expressions : Vec<Expression>, class : ExpressionClass) -> (Vec<Expression>) {
		let mut expressions = expressions;
		expressions.retain (|expression| self.expression_is_not (optimization, expression, class));
		return expressions;
	}
	
	
	fn expressions_collect_if_is <Iterator> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (Vec<Expression>)
			where Iterator : iter::Iterator<Item = Expression>
	{
		let expressions = expressions.filter (|expression| self.expression_is (optimization, expression, class));
		let expressions = expressions.collect ();
		return expressions;
	}
	
	fn expressions_collect_if_is_not <Iterator> (&self, optimization : &OptimizerContext, expressions : Iterator, class : ExpressionClass) -> (Vec<Expression>)
			where Iterator : iter::Iterator<Item = Expression>
	{
		let expressions = expressions.filter (|expression| self.expression_is_not (optimization, expression, class));
		let expressions = expressions.collect ();
		return expressions;
	}
	
	
	
	
}




#[ derive (Clone, Debug) ]
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
pub enum ExpressionClass {
	
	Constant,
	Value (ValueClass),
	Type (TypePrimitive1),
	
}

