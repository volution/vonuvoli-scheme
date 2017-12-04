

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




#[ derive (Clone, Debug) ]
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
			Expression::ConditionalIf (ref clauses) =>
				self.evaluate_conditional_if (evaluation, clauses),
			Expression::ConditionalMatch (ref actual, ref clauses) =>
				self.evaluate_conditional_match (evaluation, actual, clauses),
			
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
			Expression::RegisterInitializeValues (ref indices, ref expression) =>
				self.evaluate_register_initialize_values (evaluation, indices, expression),
			Expression::RegisterSet1 (index, ref expression) =>
				self.evaluate_register_set_1 (evaluation, index, expression),
			Expression::RegisterSetValues (ref indices, ref expression) =>
				self.evaluate_register_set_values (evaluation, indices, expression),
			Expression::RegisterGet1 (index) =>
				self.evaluate_register_get_1 (evaluation, index),
			
			Expression::BindingInitialize1 (ref binding, ref expression) =>
				self.evaluate_binding_initialize_1 (evaluation, binding, expression),
			Expression::BindingInitializeN (ref initializers, parallel) =>
				self.evaluate_binding_initialize_n (evaluation, initializers, parallel),
			Expression::BindingInitializeValues (ref bindings, ref expression) =>
				self.evaluate_binding_initialize_values (evaluation, bindings, expression),
			Expression::BindingSet1 (ref binding, ref expression) =>
				self.evaluate_binding_set_1 (evaluation, binding, expression),
			Expression::BindingSetValues (ref bindings, ref expression) =>
				self.evaluate_binding_set_values (evaluation, bindings, expression),
			Expression::BindingGet1 (ref binding) =>
				self.evaluate_binding_get_1 (evaluation, binding),
			
			Expression::Lambda (ref lambda, ref expression, ref registers_closure, ref registers_local) =>
				self.evaluate_lambda_create (evaluation, lambda, expression, registers_closure, registers_local),
			
			Expression::ProcedureCall0 (ref callable) =>
				self.evaluate_procedure_call_0 (evaluation, callable),
			Expression::ProcedureCall1 (ref callable, ref input_1) =>
				self.evaluate_procedure_call_1 (evaluation, callable, input_1),
			Expression::ProcedureCall2 (ref callable, ref input_1, ref input_2) =>
				self.evaluate_procedure_call_2 (evaluation, callable, input_1, input_2),
			Expression::ProcedureCall3 (ref callable, ref input_1, ref input_2, ref input_3) =>
				self.evaluate_procedure_call_3 (evaluation, callable, input_1, input_2, input_3),
			Expression::ProcedureCall4 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4) =>
				self.evaluate_procedure_call_4 (evaluation, callable, input_1, input_2, input_3, input_4),
			Expression::ProcedureCall5 (ref callable, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_call_5 (evaluation, callable, input_1, input_2, input_3, input_4, input_5),
			Expression::ProcedureCallN (ref callable, ref inputs) =>
				self.evaluate_procedure_call_n (evaluation, callable, inputs),
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
			Expression::ProcedurePrimitiveCall5 (primitive, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_procedure_primitive_5 (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
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
			Expression::SyntaxPrimitiveCall5 (primitive, ref input_1, ref input_2, ref input_3, ref input_4, ref input_5) =>
				self.evaluate_syntax_primitive_5 (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
			Expression::SyntaxPrimitiveCallN (primitive, ref inputs) =>
				self.evaluate_syntax_primitive_n (evaluation, primitive, inputs.as_ref ()),
			Expression::SyntaxPrimitiveCall (primitive, ref inputs) =>
				self.evaluate_syntax_primitive (evaluation, primitive, inputs.as_ref ()),
			
		}
		
	}
	
	
	
	
	pub fn evaluate_sequence (&self, evaluation : &mut EvaluatorContext, expressions : &[Expression]) -> (Outcome<Value>) {
		let mut output = VOID.into ();
		for expression in expressions {
			output = try! (evaluation.evaluate (expression));
		}
		return Ok (output);
	}
	
	
	
	
	pub fn evaluate_conditional_if (&self, evaluation : &mut EvaluatorContext, clauses : &[(Option<(Expression, bool)>, Option<Expression>)]) -> (Outcome<Value>) {
		for &(ref guard, ref expression) in clauses {
			let (matched, guard) = match *guard {
				Some ((ref guard, negated)) => {
					let guard = try! (evaluation.evaluate (guard));
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
					return evaluation.evaluate (expression);
				} else {
					succeed! (guard);
				}
			}
		}
		return Ok (VOID.into ());
	}
	
	
	pub fn evaluate_conditional_match (&self, evaluation : &mut EvaluatorContext, actual : &Expression, clauses : &[(Option<(StdBox<[Value]>, bool)>, Option<Expression>)]) -> (Outcome<Value>) {
		let actual = try! (evaluation.evaluate (actual));
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
					return evaluation.evaluate (expression);
				} else {
					succeed! (actual);
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
	
	pub fn evaluate_register_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(usize, Expression)], parallel : bool) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x4f5f5ffc);
		let indices = initializers.iter () .map (|&(index, _)| index) .collect::<StdVec<_>> ();
		let expressions = initializers.iter () .map (|&(_, ref expression)| expression) .collect::<StdVec<_>> ();
		let bindings = try_vec_map! (indices, index, registers.resolve (index));
		let bindings = bindings.iter () .collect ();
		let initializers = vec_zip_2 (bindings, expressions);
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	pub fn evaluate_register_initialize_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x64f31042);
		let bindings = try_vec_map! (indices, index, registers.resolve (*index));
		return self.evaluate_binding_initialize_values (evaluation, &bindings, expression);
	}
	
	pub fn evaluate_register_set_1 (&self, evaluation : &mut EvaluatorContext, index : usize, expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x9e3f943b);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_set_1 (evaluation, &binding, expression);
	}
	
	pub fn evaluate_register_set_values (&self, evaluation : &mut EvaluatorContext, indices : &[usize], expression : &Expression) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x2137dc1e);
		let bindings = try_vec_map! (indices, index, registers.resolve (*index));
		return self.evaluate_binding_set_values (evaluation, &bindings, expression);
	}
	
	pub fn evaluate_register_get_1 (&self, evaluation : &mut EvaluatorContext, index : usize) -> (Outcome<Value>) {
		let registers = try_some! (evaluation.registers, 0x89f09b48);
		let binding = try! (registers.resolve (index));
		return self.evaluate_binding_get_1 (evaluation, &binding);
	}
	
	
	
	
	pub fn evaluate_binding_initialize_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (evaluation.evaluate (expression));
		let value_new = try! (binding.initialize (value_new));
		return Ok (value_new);
	}
	
	pub fn evaluate_binding_initialize_n (&self, evaluation : &mut EvaluatorContext, initializers : &[(Binding, Expression)], parallel : bool) -> (Outcome<Value>) {
		let initializers = initializers.iter () .map (|&(ref binding, ref expression)| (binding, expression)) .collect::<StdVec<_>> ();
		return self.evaluate_binding_initialize_n_0 (evaluation, &initializers, parallel);
	}
	
	pub fn evaluate_binding_initialize_n_0 (&self, evaluation : &mut EvaluatorContext, initializers : &[(&Binding, &Expression)], parallel : bool) -> (Outcome<Value>) {
		
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
	
	pub fn evaluate_binding_initialize_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new = try! (evaluation.evaluate (expression));
		let values_new = try_into_values! (values_new);
		if values_new.values_length () != bindings.len () {
			fail! (0x34cd5a9a);
		}
		for (binding, value_new) in bindings.iter () .zip (values_new.values_ref () .iter ()) {
			try! (binding.initialize (value_new.clone ()));
		}
		return Ok (values_new.into ());
	}
	
	pub fn evaluate_binding_set_1 (&self, evaluation : &mut EvaluatorContext, binding : &Binding, expression : &Expression) -> (Outcome<Value>) {
		let value_new = try! (evaluation.evaluate (expression));
		let value_old = try! (binding.set (value_new));
		return Ok (value_old);
	}
	
	
	pub fn evaluate_binding_set_values (&self, evaluation : &mut EvaluatorContext, bindings : &[Binding], expression : &Expression) -> (Outcome<Value>) {
		let values_new_ = try! (evaluation.evaluate (expression));
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
	
	pub fn evaluate_binding_get_1 (&self, _evaluation : &mut EvaluatorContext, binding : &Binding) -> (Outcome<Value>) {
		let value = try! (binding.get ());
		return Ok (value);
	}
	
	
	
	
	pub fn evaluate_lambda_create (&self, evaluation : &mut EvaluatorContext, lambda : &LambdaTemplate, expressions : &Expression, registers_closure : &[RegistersBindingTemplate], registers_local : &[RegistersBindingTemplate]) -> (Outcome<Value>) {
		let registers_closure = try! (Registers::new_and_define (registers_closure, evaluation.registers));
		let lambda = Lambda::new (lambda.clone (), expressions.clone (), registers_closure, registers_local.to_vec ());
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
	
	pub fn evaluate_lambda_call_0_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[][..]);
	}
	
	pub fn evaluate_lambda_call_1_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, input_1 : &Value) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[input_1.clone ()][..]);
	}
	
	pub fn evaluate_lambda_call_2_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[input_1.clone (), input_2.clone ()][..]);
	}
	
	pub fn evaluate_lambda_call_3_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[input_1.clone (), input_2.clone (), input_3.clone ()][..]);
	}
	
	pub fn evaluate_lambda_call_4_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[input_1.clone (), input_2.clone (), input_3.clone (), input_4.clone ()][..]);
	}
	
	pub fn evaluate_lambda_call_5_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, &[input_1.clone (), input_2.clone (), input_3.clone (), input_4.clone (), input_5.clone ()][..]);
	}
	
	pub fn evaluate_lambda_call_n_with_values (&self, evaluation : &mut EvaluatorContext, lambda : &Lambda, inputs : &[Value]) -> (Outcome<Value>) {
		return self.evaluate_lambda_call_with_values (evaluation, lambda, inputs);
	}
	
	
	
	
	pub fn evaluate_procedure_call_0 (&self, evaluation : &mut EvaluatorContext, callable : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		return self.evaluate_procedure_call_0_with_values (evaluation, &callable);
	}
	
	pub fn evaluate_procedure_call_0_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive0 (primitive) =>
						return self.evaluate_procedure_primitive_0 (evaluation, primitive),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_0 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_0 (evaluation, primitive),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[][..]),
						},
					_ =>
						fail! (0xc773cca0),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_0_with_values (evaluation, callable.as_ref ()),
			_ =>
				fail! (0xc58e190e),
		}
	}
	
	
	pub fn evaluate_procedure_call_1 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let input_1 = try! (evaluation.evaluate (input_1));
		return self.evaluate_procedure_call_1_with_values (evaluation, &callable, &input_1);
	}
	
	pub fn evaluate_procedure_call_1_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive1 (primitive) =>
						return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, input_1),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_1 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_1_with_values (evaluation, primitive, input_1),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[input_1.clone ()][..]),
						},
					_ =>
						fail! (0x55ce48e1),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_1_with_values (evaluation, callable.as_ref (), input_1),
			_ =>
				fail! (0xe7f6dbfc),
		}
	}
	
	
	pub fn evaluate_procedure_call_2 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		return self.evaluate_procedure_call_2_with_values (evaluation, &callable, &input_1, &input_2);
	}
	
	pub fn evaluate_procedure_call_2_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive2 (primitive) =>
						return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, input_1, input_2),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_2 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_2_with_values (evaluation, primitive, input_1, input_2),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[input_1.clone (), input_2.clone ()][..]),
						},
					_ =>
						fail! (0xc8018868),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_2_with_values (evaluation, callable.as_ref (), input_1, input_2),
			_ =>
				fail! (0x856bf44d),
		}
	}
	
	
	pub fn evaluate_procedure_call_3 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		return self.evaluate_procedure_call_3_with_values (evaluation, &callable, &input_1, &input_2, &input_3);
	}
	
	pub fn evaluate_procedure_call_3_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive3 (primitive) =>
						return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, input_1, input_2, input_3),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_3 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_3_with_values (evaluation, primitive, input_1, input_2, input_3),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[input_1.clone (), input_2.clone (), input_3.clone ()][..]),
						},
					_ =>
						fail! (0xade03d0b),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_3_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3),
			_ =>
				fail! (0xdb6b9bbc),
		}
	}
	
	
	pub fn evaluate_procedure_call_4 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		let input_4 = try! (evaluation.evaluate (input_4));
		return self.evaluate_procedure_call_4_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4);
	}
	
	pub fn evaluate_procedure_call_4_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive4 (primitive) =>
						return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, input_1, input_2, input_3, input_4),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_4 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_4_with_values (evaluation, primitive, input_1, input_2, input_3, input_4),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[input_1.clone (), input_2.clone (), input_3.clone (), input_4.clone ()][..]),
						},
					_ =>
						fail! (0x2cbf2824),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_4_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3, input_4),
			_ =>
				fail! (0xf0969d74),
		}
	}
	
	
	pub fn evaluate_procedure_call_5 (&self, evaluation : &mut EvaluatorContext, callable : &Expression, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		let input_4 = try! (evaluation.evaluate (input_4));
		let input_5 = try! (evaluation.evaluate (input_5));
		return self.evaluate_procedure_call_5_with_values (evaluation, &callable, &input_1, &input_2, &input_3, &input_4, &input_5);
	}
	
	pub fn evaluate_procedure_call_5_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::Primitive5 (primitive) =>
						return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
					ProcedurePrimitive::PrimitiveN (primitive) =>
						match procedure_primitive_n_alternative_5 (primitive) {
							Some (primitive) =>
								return self.evaluate_procedure_primitive_5_with_values (evaluation, primitive, input_1, input_2, input_3, input_4, input_5),
							None =>
								return self.evaluate_procedure_primitive_n_with_values_without_alternatives (evaluation, primitive, &[input_1.clone (), input_2.clone (), input_3.clone (), input_4.clone (), input_5.clone ()][..]),
						},
					_ =>
						fail! (0xdf89aaf6),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_5_with_values (evaluation, callable.as_ref (), input_1, input_2, input_3, input_4, input_5),
			_ =>
				fail! (0x62e8aef5),
		}
	}
	
	
	pub fn evaluate_procedure_call_n (&self, evaluation : &mut EvaluatorContext, callable : &Expression, inputs : &[Expression]) -> (Outcome<Value>) {
		let callable = try! (evaluation.evaluate (callable));
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return self.evaluate_procedure_call_n_with_values (evaluation, &callable, &inputs);
	}
	
	pub fn evaluate_procedure_call_n_with_values (&self, evaluation : &mut EvaluatorContext, callable : &Value, inputs : &[Value]) -> (Outcome<Value>) {
		match callable.class () {
			ValueClass::ProcedurePrimitive =>
				match *callable.as_ref () {
					ProcedurePrimitive::PrimitiveN (primitive) =>
						return self.evaluate_procedure_primitive_n_with_values (evaluation, primitive, inputs),
					_ =>
						fail! (0x77f3e789),
				},
			ValueClass::Lambda =>
				return self.evaluate_lambda_call_n_with_values (evaluation, callable.as_ref (), inputs),
			_ =>
				fail! (0x906ebf03),
		}
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
	
	
	pub fn evaluate_procedure_primitive_5 (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		let input_1 = try! (evaluation.evaluate (input_1));
		let input_2 = try! (evaluation.evaluate (input_2));
		let input_3 = try! (evaluation.evaluate (input_3));
		let input_4 = try! (evaluation.evaluate (input_4));
		let input_5 = try! (evaluation.evaluate (input_5));
		return procedure_primitive_5_evaluate (primitive, &input_1, &input_2, &input_3, &input_4, &input_5, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_5_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value) -> (Outcome<Value>) {
		return procedure_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	
	pub fn evaluate_procedure_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		let inputs = try! (evaluation.evaluate_slice (inputs));
		return procedure_primitive_n_evaluate (primitive, &inputs, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_n_with_values (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate (primitive, inputs, evaluation);
	}
	
	pub fn evaluate_procedure_primitive_n_with_values_without_alternatives (&self, evaluation : &mut EvaluatorContext, primitive : ProcedurePrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
		return procedure_primitive_n_evaluate_without_alternatives (primitive, inputs, evaluation);
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
	
	pub fn evaluate_syntax_primitive_5 (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive5, input_1 : &Expression, input_2 : &Expression, input_3 : &Expression, input_4 : &Expression, input_5 : &Expression) -> (Outcome<Value>) {
		return syntax_primitive_5_evaluate (primitive, input_1, input_2, input_3, input_4, input_5, evaluation);
	}
	
	pub fn evaluate_syntax_primitive_n (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitiveN, inputs : &[Expression]) -> (Outcome<Value>) {
		return syntax_primitive_n_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	pub fn evaluate_syntax_primitive (&self, evaluation : &mut EvaluatorContext, primitive : SyntaxPrimitive, inputs : &[Expression]) -> (Outcome<Value>) {
		return syntax_primitive_evaluate (primitive, inputs.as_ref (), evaluation);
	}
	
	
}




#[ derive (Clone, Debug) ]
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

