

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::lambdas::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::compile;
	pub use super::compile_script;
	pub use super::Compiler;
	pub use super::CompilerContext;
	pub use super::CompilerBindings;
	pub use super::CompilerBinding;
	pub use super::super::compiler_optimizer::exports::*;
}




pub fn compile (context : &Context, token : &Value) -> (Outcome<Expression>) {
	return Compiler::new () .compile (Some (context), token);
}

pub fn compile_script (context : &Context, tokens : &[Value]) -> (Outcome<ExpressionVec>) {
	return Compiler::new () .compile_slice (Some (context), tokens);
}




#[ derive (Debug) ]
pub struct Compiler {}


impl Compiler {
	
	
	
	
	pub fn new () -> (Compiler) {
		return Compiler {};
	}
	
	
	
	
	pub fn compile (&self, context : Option<&Context>, token : &Value) -> (Outcome<Expression>) {
		let token = token.clone ();
		let compilation = CompilerContext::new_with_context (context);
		let (_compilation, expression) = try! (self.compile_0 (compilation, token));
		succeed! (expression);
	}
	
	pub fn compile_slice (&self, context : Option<&Context>, tokens : &[Value]) -> (Outcome<ExpressionVec>) {
		let tokens = vec_clone_slice (tokens);
		let compilation = CompilerContext::new_with_context (context);
		let (_compilation, expressions) = try! (self.compile_0_vec (compilation, tokens));
		succeed! (expressions);
	}
	
	
	
	
	fn compile_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		if COMPILER_TRACE_INPUT || COMPILER_TRACE_OUTPUT || COMPILER_TRACE_ERROR {
			
			let token_input = token.clone ();
			
			if COMPILER_TRACE_INPUT {
				eprint! ("[dd]  compiling: {}\n", &token_input);
			}
			
			let outcome = self.compile_00 (compilation, token);
			
			match outcome {
				Ok (ref expression) if COMPILER_TRACE_OUTPUT =>
					eprint! ("[dd]  compiling succeeded:\n[  ]      {}\n[  ]      {:?}\n", &token_input, expression),
				Ok (_) =>
					(),
				Err (ref error) if COMPILER_TRACE_OUTPUT || COMPILER_TRACE_ERROR =>
					eprint! ("[dd]  compiling failed:\n[  ]      {}\n[  ]      {:?}\n", &token_input, error),
				Err (_) =>
					(),
			}
			
			return outcome;
			
		} else {
			
			return self.compile_00 (compilation, token);
		}
	}
	
	
	fn compile_00 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		match token.class_match_into () {
			
			ValueClassMatchInto::Null =>
				return self.compile_syntax_quote_0 (compilation, NULL_VALUE),
			
			ValueClassMatchInto::Void =>
				return self.compile_syntax_quote_0 (compilation, VOID_VALUE),
			
			ValueClassMatchInto::Undefined =>
				return self.compile_syntax_quote_0 (compilation, UNDEFINED_VALUE),
			
			ValueClassMatchInto::Singleton (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Boolean (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Number (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Character (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::String (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Symbol (value) =>
				return self.compile_symbol (compilation, value),
			
			ValueClassMatchInto::Pair (class) =>
				return self.compile_form (compilation, class.into_immutable ()),
			
			ValueClassMatchInto::Bytes (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Array (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Values (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Error (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Procedure (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Syntax (_class) =>
				fail_unimplemented! (0xc617f3c7), // deferred
			
			ValueClassMatchInto::Port (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Resource (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Internal (_value) =>
				fail_unimplemented! (0x45661ea2), // deferred
			
			ValueClassMatchInto::Opaque (value) =>
				return self.compile_syntax_quote_0 (compilation, value),
			
		}
	}
	
	
	
	
	fn compile_0_vec (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, ExpressionVec)>) {
		let mut expressions = ExpressionVec::with_capacity (tokens.len ());
		let mut compilation = compilation;
		for token in tokens.into_iter () {
			let (compilation_1, expression) = try! (self.compile_0 (compilation, token));
			compilation = compilation_1;
			expressions.push (expression);
		}
		succeed! ((compilation, expressions));
	}
	
	
	
	
	fn compile_symbol (&self, compilation : CompilerContext, identifier : Symbol) -> (Outcome<(CompilerContext, Expression)>) {
		let (compilation, binding) = try! (compilation.resolve (identifier));
		match binding {
			CompilerBinding::Binding (_, binding, _) =>
				succeed! ((compilation, ExpressionForContexts::BindingGet1 (binding) .into ())),
			CompilerBinding::Register (_, index, _) =>
				succeed! ((compilation, ExpressionForContexts::RegisterGet1 (index) .into ())),
			CompilerBinding::Undefined =>
				fail! (0xc6825cfd),
		}
	}
	
	
	
	
	fn compile_form (&self, compilation : CompilerContext, form : PairImmutable) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (callable, arguments) = form.left_and_right_into ();
		
		match try! (self.compile_form_match_callable (compilation, callable)) {
			
			(compilation, Alternative2::Variant1 (syntax)) =>
				return self.compile_syntax_call (compilation, syntax, arguments),
			
			(compilation, Alternative2::Variant2 (callable)) =>
				return self.compile_procedure_call (compilation, callable, arguments),
			
		}
	}
	
	
	fn compile_form_match_callable (&self, compilation : CompilerContext, callable : Value) -> (Outcome<(CompilerContext, Alternative2<SyntaxPrimitive, Value>)>) {
		
		match callable.class_match_into () {
			
			ValueClassMatchInto::Symbol (symbol) => {
				let (compilation, callable) = try! (compilation.resolve_value (symbol.clone ()));
				if let Some (callable) = callable {
					let class = callable.class_match_into ();
					return self.compile_form_match_class (compilation, class);
				} else {
					succeed! ((compilation, Alternative2::Variant2 (symbol.into ())));
				}
			},
			
			class =>
				return self.compile_form_match_class (compilation, class),
			
		}
	}
	
	
	fn compile_form_match_class (&self, compilation : CompilerContext, class : ValueClassMatchInto) -> (Outcome<(CompilerContext, Alternative2<SyntaxPrimitive, Value>)>) {
		
		match class {
			
			ValueClassMatchInto::Syntax (class) =>
				match class {
					SyntaxMatchInto::Primitive (syntax) =>
						succeed! ((compilation, Alternative2::Variant1 (syntax))),
					SyntaxMatchInto::Extended (_syntax) =>
						fail_unimplemented! (0xb9915ee0), // deferred
					SyntaxMatchInto::Native (_syntax) =>
						fail_unimplemented! (0x09bdd94b), // deferred
					SyntaxMatchInto::Lambda (_syntax) =>
						fail_unimplemented! (0xd3517f42), // deferred
				},
			
			class =>
				succeed! ((compilation, Alternative2::Variant2 (class.value ()))),
			
		}
	}
	
	
	
	
	fn compile_procedure_call (&self, compilation : CompilerContext, procedure : Value, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		let arguments = try! (vec_list_clone (&arguments));
		return self.compile_procedure_call_0 (compilation, procedure, arguments);
	}
	
	
	fn compile_procedure_call_0 (&self, compilation : CompilerContext, procedure : Value, arguments : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, procedure) = try! (self.compile_0 (compilation, procedure));
		let (compilation, arguments) = try! (self.compile_0_vec (compilation, arguments));
		
		let expression = ExpressionForProcedureGenericCall::ProcedureCall (procedure.into (), arguments.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_call (&self, compilation : CompilerContext, syntax : SyntaxPrimitive, tokens : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens = try! (vec_list_clone (&tokens));
		
		match syntax {
			
			SyntaxPrimitive::PrimitiveV (syntax) =>
				match syntax {
					
					SyntaxPrimitiveV::Quote =>
						return self.compile_syntax_quote (compilation, tokens),
					
					SyntaxPrimitiveV::QuasiQuote =>
						return self.compile_syntax_quasi_quote (compilation, tokens),
					
					SyntaxPrimitiveV::UnQuote |
					SyntaxPrimitiveV::UnQuoteSplicing =>
						fail! (0x99b4857b),
					
					SyntaxPrimitiveV::Begin =>
						return self.compile_syntax_sequence (compilation, ExpressionSequenceOperator::ReturnLast, tokens),
					
					SyntaxPrimitiveV::And =>
						return self.compile_syntax_sequence (compilation, ExpressionSequenceOperator::And, tokens),
					
					SyntaxPrimitiveV::Or =>
						return self.compile_syntax_sequence (compilation, ExpressionSequenceOperator::Or, tokens),
					
					SyntaxPrimitiveV::If =>
						return self.compile_syntax_if (compilation, tokens),
					
					SyntaxPrimitiveV::When |
					SyntaxPrimitiveV::Unless =>
						return self.compile_syntax_when_unless (compilation, syntax, tokens),
					
					SyntaxPrimitiveV::Cond =>
						return self.compile_syntax_cond (compilation, tokens),
					
					SyntaxPrimitiveV::Case =>
						return self.compile_syntax_case (compilation, tokens),
					
					SyntaxPrimitiveV::Do =>
						return self.compile_syntax_do (compilation, tokens),
					
					SyntaxPrimitiveV::DoCond =>
						fail_unimplemented! (0x2e2b0079), // deferred
					
					SyntaxPrimitiveV::While |
					SyntaxPrimitiveV::Until =>
						fail_unimplemented! (0xdae6d716), // deferred
					
					SyntaxPrimitiveV::WhileCond |
					SyntaxPrimitiveV::UntilCond =>
						fail_unimplemented! (0x9e9861c0), // deferred
					
					SyntaxPrimitiveV::Guard =>
						return self.compile_syntax_guard (compilation, tokens),
					
					SyntaxPrimitiveV::GuardCond =>
						return self.compile_syntax_guard_cond (compilation, tokens),
					
					SyntaxPrimitiveV::Locals =>
						return self.compile_syntax_locals (compilation, tokens),
					
					SyntaxPrimitiveV::LetParallel |
					SyntaxPrimitiveV::LetSequential |
					SyntaxPrimitiveV::LetRecursiveParallel |
					SyntaxPrimitiveV::LetRecursiveSequential =>
						return self.compile_syntax_let (compilation, syntax, tokens),
					
					SyntaxPrimitiveV::LetValuesParallel |
					SyntaxPrimitiveV::LetValuesSequential =>
						return self.compile_syntax_let_values (compilation, syntax, tokens),
					
					SyntaxPrimitiveV::Define =>
						return self.compile_syntax_define (compilation, tokens),
					
					SyntaxPrimitiveV::DefineValues =>
						return self.compile_syntax_define_values (compilation, tokens),
					
					SyntaxPrimitiveV::Set =>
						return self.compile_syntax_set (compilation, tokens),
					
					SyntaxPrimitiveV::SetValues =>
						return self.compile_syntax_set_values (compilation, tokens),
					
					SyntaxPrimitiveV::Lambda =>
						return self.compile_syntax_lambda (compilation, None, tokens),
					
				},
			
			SyntaxPrimitive::Auxiliary =>
				fail! (0xc64adbb8),
			
			SyntaxPrimitive::Unimplemented =>
				fail_unimplemented! (0xa4e41f62), // OK
			
			SyntaxPrimitive::Unsupported =>
				fail_unimplemented! (0x175a7f9e), // OK
			
			SyntaxPrimitive::Reserved =>
				fail! (0x1aed14f3),
			
		}
	}
	
	
	
	
	fn compile_syntax_sequence (&self, compilation : CompilerContext, operator : ExpressionSequenceOperator, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, tokens));
		
		let expression = Expression::Sequence (operator, statements.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_if (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if (tokens_count != 2) && (tokens_count != 3) {
			fail! (0xe34389a7);
		}
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, tokens));
		
		let clauses = if tokens_count == 3 {
			let (guard, if_true, if_false) = try! (vec_explode_3 (statements));
			vec! [
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::Expression (guard, false),
							ExpressionValueConsumer::Ignore,
							if_true),
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::True,
							ExpressionValueConsumer::Ignore,
							if_false),
				]
		} else if tokens_count == 2 {
			let (guard, if_true) = try! (vec_explode_2 (statements));
			vec! [
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::Expression (guard, false),
							ExpressionValueConsumer::Ignore,
							if_true),
				]
		} else {
			fail_unreachable! (0xbc801c5d);
		};
		
		let clauses = ExpressionConditionalIfClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalIf (clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_when_unless (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if tokens_count < 2 {
			fail! (0x3c364a9f);
		}
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, tokens));
		let (guard, statements) = try! (vec_explode_1n (statements));
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		
		let negated = match syntax {
			SyntaxPrimitiveV::When =>
				false,
			SyntaxPrimitiveV::Unless =>
				true,
			_ =>
				fail_unreachable! (0x500d298f),
		};
		
		let clauses = vec! [
				ExpressionConditionalIfClause::GuardAndExpression (
						ExpressionConditionalIfGuard::Expression (guard, negated),
						ExpressionValueConsumer::Ignore,
						statements),
			];
		
		let clauses = ExpressionConditionalIfClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalIf (clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_cond (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let mut clauses = StdVec::new ();
		let mut compilation = compilation;
		
		for tokens in tokens.into_iter () {
			
			let tokens = try! (vec_list_clone (&tokens));
			if tokens.is_empty () {
				fail! (0x86331f4b);
			}
			let (guard, statements) = try! (vec_explode_1n (tokens));
			
			let (compilation_1, guard) = if ! is_symbol_eq ("else", &guard) {
				let (compilation_1, guard) = try! (self.compile_0 (compilation, guard));
				let guard = ExpressionConditionalIfGuard::Expression (guard, false);
				(compilation_1, guard)
			} else {
				let guard = ExpressionConditionalIfGuard::True;
				(compilation, guard)
			};
			
			let (compilation_1, guard_consumer) = if ! statements.is_empty () && is_symbol_eq ("=>", &statements[0]) {
				fail_unimplemented! (0xfa332991); // deferred
			} else {
				(compilation_1, ExpressionValueConsumer::Ignore)
			};
			
			let (compilation_1, statements) = try! (self.compile_0_vec (compilation_1, statements));
			
			let clause = if ! statements.is_empty () {
				let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
				ExpressionConditionalIfClause::GuardAndExpression (guard, guard_consumer, statements)
			} else {
				ExpressionConditionalIfClause::GuardOnly (guard, ExpressionValueConsumer::Return)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let clauses = ExpressionConditionalIfClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalIf (clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_case (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 1 {
			fail! (0xeb8569a2);
		}
		
		let (actual, tokens) = try! (vec_explode_1n (tokens));
		
		let (compilation, actual) = try! (self.compile_0 (compilation, actual));
		
		let mut clauses = StdVec::new ();
		let mut compilation = compilation;
		
		for tokens in tokens.into_iter () {
			
			let tokens = try! (vec_list_clone (&tokens));
			if tokens.is_empty () {
				fail! (0x17388f6a);
			}
			let (expected, statements) = try! (vec_explode_1n (tokens));
			
			let guard = if ! is_symbol_eq ("else", &expected) {
				let expected = try! (vec_list_clone (&expected));
				ExpressionConditionalMatchGuard::Values (expected.into_boxed_slice (), false)
			} else {
				ExpressionConditionalMatchGuard::True
			};
			
			let (compilation_1, guard_consumer) = if ! statements.is_empty () && is_symbol_eq ("=>", &statements[0]) {
				fail_unimplemented! (0xef5d468c); // deferred
			} else {
				(compilation, ExpressionValueConsumer::Ignore)
			};
			
			let (compilation_1, statements) = try! (self.compile_0_vec (compilation_1, statements));
			
			let clause = if ! statements.is_empty () {
				let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
				ExpressionConditionalMatchClause::GuardAndExpression (guard, guard_consumer, statements)
			} else {
				ExpressionConditionalMatchClause::GuardOnly (guard, ExpressionValueConsumer::Return)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let clauses = ExpressionConditionalMatchClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalMatch (actual.into (), clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_do (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (definitions, break_statements, loop_statements) = try! (vec_explode_2n (tokens));
		
		let definitions = try! (vec_list_clone (&definitions));
		
		let mut identifiers = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		let mut updaters = StdVec::with_capacity (definitions.len ());
		for definition in definitions.into_iter () {
			let definition = try! (vec_list_clone (&definition));
			let (identifier, initializer, updater) = match definition.len () {
				2 => {
					let (identifier, initializer) = try! (vec_explode_2 (definition));
					(identifier, initializer, None)
				},
				3 => {
					let (identifier, initializer, updater) = try! (vec_explode_3 (definition));
					(identifier, initializer, Some (updater))
				},
				_ =>
					fail! (0x0735df7b),
			};
			let identifier = try_into_symbol! (identifier);
			let updater = if let Some (updater) = updater {
				updater
			} else {
				identifier.clone () .into ()
			};
			identifiers.push (identifier);
			initializers.push (initializer);
			updaters.push (updater);
		}
		
		let (compilation, has_definitions) = if identifiers.is_empty () {
			(compilation, false)
		} else {
			let compilation = try! (compilation.fork_locals (true));
			(compilation, true)
		};
		
		let (compilation, binding_initializers, binding_updaters) = if has_definitions {
			
			let mut binding_templates = StdVec::new ();
			let mut binding_initializers = StdVec::new ();
			let mut binding_updaters = StdVec::new ();
			let mut compilation = compilation;
			
			for initializer in initializers.into_iter () {
				let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
				compilation = compilation_1;
				binding_initializers.push (initializer);
			}
			
			for identifier in identifiers.into_iter () {
				let (compilation_1, binding) = try! (compilation.define (identifier));
				compilation = compilation_1;
				binding_templates.push (binding);
			}
			
			for updater in updaters.into_iter () {
				let (compilation_1, updater) = try! (self.compile_0 (compilation, updater));
				compilation = compilation_1;
				binding_updaters.push (updater);
			}
			
			let binding_initializers = try! (self.compile_syntax_binding_set_n (binding_templates.clone (), binding_initializers, true, true));
			let binding_updaters = try! (self.compile_syntax_binding_set_n (binding_templates.clone (), binding_updaters, true, false));
			
			(compilation, Some (binding_initializers.into ()), Some (binding_updaters.into ()))
			
		} else {
			
			(compilation, None, None)
		};
		
		
		let break_statements = try! (vec_list_clone (&break_statements));
		let (break_guard, break_statements) = try! (vec_explode_1n (break_statements));
		
		let (compilation, break_guard) = try! (self.compile_0 (compilation, break_guard));
		let break_guard = ExpressionConditionalIfGuard::Expression (break_guard, false);
		
		let (compilation, break_clause) = if break_statements.is_empty () {
			let clause = ExpressionConditionalIfClause::GuardOnly (break_guard, ExpressionValueConsumer::Return);
			(compilation, clause)
		} else {
			// FIXME:  Add support for `(guard => expression)` just like for `cond`!
			let (compilation, break_statements) = try! (self.compile_0_vec (compilation, break_statements));
			let break_statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, break_statements.into_boxed_slice ());
			let clause = ExpressionConditionalIfClause::GuardAndExpression (break_guard, ExpressionValueConsumer::Ignore, break_statements);
			(compilation, clause)
		};
		
		let break_clauses = ExpressionConditionalIfClauses::Multiple (StdBox::new ([break_clause]));
		
		let (compilation, loop_statement) = if loop_statements.is_empty () {
			(compilation, None)
		} else {
			let (compilation, loop_statements) = try! (self.compile_0_vec (compilation, loop_statements));
			let loop_statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, loop_statements.into_boxed_slice ());
			(compilation, Some (loop_statements.into ()))
		};
		
		let expression = Expression::Loop (binding_initializers, binding_updaters, loop_statement, break_clauses);
		
		let (compilation, expression) = if has_definitions {
			let (compilation, registers) = try! (compilation.unfork_locals ());
			let expression = ExpressionForContexts::RegisterClosure (expression.into (), registers.into_boxed_slice ()) .into ();
			(compilation, expression)
		} else {
			(compilation, expression)
		};
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_guard (&self, _compilation : CompilerContext, _tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0x4bc8b360);
	}
	
	
	
	
	fn compile_syntax_guard_cond (&self, _compilation : CompilerContext, _tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0xc89144f5);
	}
	
	
	
	
	fn compile_syntax_locals (&self, compilation : CompilerContext, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = try! (compilation.fork_locals (true));
		let (compilation, statements) = try! (self.compile_0_vec (compilation, statements));
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_let (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x633b3ed8);
		}
		let (definitions, statements) = try! (vec_explode_1n (tokens));
		
		let definitions = match definitions.kind_match_into () {
			ValueKindMatchInto::Null =>
				return self.compile_syntax_locals (compilation, statements),
			ValueKindMatchInto::Symbol (symbol) => {
				let (definitions, statements) = try! (vec_explode_1n (statements));
				return self.compile_syntax_lambda_let (compilation, symbol, definitions, statements);
			},
			ValueKindMatchInto::PairImmutable (pair) =>
				try! (vec_list_clone (&pair.into ())),
			ValueKindMatchInto::PairMutable (pair) =>
				try! (vec_list_clone (&pair.into ())),
			_ =>
				fail! (0x825cb457),
		};
		
		let mut identifiers = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions.into_iter () {
			let definition = try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x190d57f8);
			}
			let (identifier, initializer) = try! (vec_explode_2 (definition));
			let identifier = try_into_symbol! (identifier);
			identifiers.push (identifier);
			initializers.push (initializer);
		}
		
		let mut compilation = try! (compilation.fork_locals (true));
		let mut binding_templates = StdVec::new ();
		let mut binding_initializers = StdVec::new ();
		
		match syntax {
			
			SyntaxPrimitiveV::LetParallel => {
				for initializer in initializers.into_iter () {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					compilation = compilation_1;
					binding_initializers.push (initializer);
				}
				for identifier in identifiers.into_iter () {
					let (compilation_1, binding) = try! (compilation.define (identifier));
					compilation = compilation_1;
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetSequential => {
				for (initializer, identifier) in initializers.into_iter ().zip (identifiers.into_iter ()) {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					let (compilation_1, binding) = try! (compilation_1.define (identifier));
					compilation = compilation_1;
					binding_initializers.push (initializer);
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetRecursiveParallel | SyntaxPrimitiveV::LetRecursiveSequential => {
				for identifier in identifiers.into_iter () {
					let (compilation_1, binding) = try! (compilation.define (identifier));
					compilation = compilation_1;
					binding_templates.push (binding);
				}
				for initializer in initializers.into_iter () {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					compilation = compilation_1;
					binding_initializers.push (initializer);
				}
			},
			
			_ =>
				fail_unreachable! (0xa1c3e4ac),
			
		}
		
		let parallel = match syntax {
			SyntaxPrimitiveV::LetParallel =>
				true,
			SyntaxPrimitiveV::LetSequential =>
				false,
			SyntaxPrimitiveV::LetRecursiveParallel =>
				true,
			SyntaxPrimitiveV::LetRecursiveSequential =>
				false,
			_ =>
				fail_unreachable! (0xa615e40c),
		};
		
		let binding_initializers = try! (self.compile_syntax_binding_set_n (binding_templates, binding_initializers, parallel, true));
		let binding_initializers = vec! [ binding_initializers ];
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_let_values (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x10672a0d);
		}
		let (definitions, statements) = try! (vec_explode_1n (tokens));
		
		let definitions = match definitions.kind_match_into () {
			ValueKindMatchInto::Null =>
				return self.compile_syntax_locals (compilation, statements),
			ValueKindMatchInto::PairImmutable (pair) =>
				try! (vec_list_clone (&pair.into ())),
			ValueKindMatchInto::PairMutable (pair) =>
				try! (vec_list_clone (&pair.into ())),
			_ =>
				fail! (0x60cfd87a),
		};
		
		let mut identifiers_n = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions.into_iter () {
			let definition = try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x6cbd574f);
			}
			let (identifiers, initializer) = try! (vec_explode_2 (definition));
			let identifiers = try! (vec_list_clone (&identifiers));
			let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
			identifiers_n.push (identifiers);
			initializers.push (initializer);
		}
		
		let mut compilation = try! (compilation.fork_locals (true));
		let mut binding_templates_n = StdVec::new ();
		let mut binding_initializers = StdVec::new ();
		
		match syntax {
			
			SyntaxPrimitiveV::LetValuesParallel => {
				for initializer in initializers.into_iter () {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					compilation = compilation_1;
					binding_initializers.push (initializer);
				}
				for identifiers in identifiers_n.into_iter () {
					let mut binding_templates = StdVec::new ();
					for identifier in identifiers.into_iter () {
						let (compilation_1, binding) = try! (compilation.define (identifier));
						compilation = compilation_1;
						binding_templates.push (binding);
					}
					binding_templates_n.push (binding_templates);
				}
			},
			
			SyntaxPrimitiveV::LetValuesSequential => {
				for (initializer, identifiers) in initializers.into_iter ().zip (identifiers_n.into_iter ()) {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					compilation = compilation_1;
					let mut binding_templates = StdVec::new ();
					for identifier in identifiers.into_iter () {
						let (compilation_1, binding) = try! (compilation.define (identifier));
						compilation = compilation_1;
						binding_templates.push (binding);
					}
					binding_templates_n.push (binding_templates);
					binding_initializers.push (initializer);
				}
			},
			
			_ =>
				fail_unreachable! (0x7498ded2),
			
		}
		
		let binding_initializers = try! (self.compile_syntax_binding_set_values_n (binding_templates_n, binding_initializers, true));
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_define (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x4481879c);
		}
		let (signature, statements) = try! (vec_explode_1n (tokens));
		
		let (compilation, binding, expression) = match signature.class_match_into () {
			
			ValueClassMatchInto::Symbol (identifier) => {
				
				if statements.len () != 1 {
					fail! (0xc364edf8);
				}
				
				let statement = try! (vec_explode_1 (statements));
				
				let (compilation, binding) = try! (compilation.define (identifier));
				let (compilation, expression) = try! (self.compile_0 (compilation, statement));
				
				(compilation, binding, expression)
			},
			
			ValueClassMatchInto::Pair (signature) => {
				
				if statements.len () < 1 {
					fail! (0x48c70de5);
				}
				
				let (signature, argument_rest) = try! (vec_list_clone_dotted (&signature.value ()));
				let (identifier, arguments_positional) = try! (vec_explode_1n (signature));
				
				let identifier = try_into_symbol! (identifier);
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				
				let (compilation, binding) = try! (compilation.define (identifier.clone ()));
				let (compilation, expression) = try! (self.compile_syntax_lambda_0 (compilation, Some (identifier), arguments_positional, argument_rest, statements));
				
				(compilation, binding, expression)
			},
			
			_ =>
				fail! (0x0f0edc26),
		};
		
		let expression = try! (self.compile_syntax_binding_set_1 (binding, expression, true));
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_define_values (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0x5d801f2e);
		}
		
		let (identifiers, initializer) = try! (vec_explode_2 (tokens));
		let identifiers = try! (vec_list_clone (&identifiers));
		let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
		
		let mut compilation = compilation;
		let mut binding_templates = StdVec::new ();
		for identifier in identifiers.into_iter () {
			let (compilation_1, binding) = try! (compilation.define (identifier));
			compilation = compilation_1;
			binding_templates.push (binding);
		}
		
		let (compilation, binding_initializer) = try! (self.compile_0 (compilation, initializer));
		
		let expression = try! (self.compile_syntax_binding_set_values_1 (binding_templates, binding_initializer, true));
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_set (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0x2573a064);
		}
		
		let (identifier, initializer) = try! (vec_explode_2 (tokens));
		let identifier = try_into_symbol! (identifier);
		
		let (compilation, binding) = try! (compilation.resolve (identifier));
		
		let (compilation, initializer) = try! (self.compile_0 (compilation, initializer));
		
		let initializer = try! (self.compile_syntax_binding_set_1 (binding, initializer, false));
		
		succeed! ((compilation, initializer));
	}
	
	
	
	
	fn compile_syntax_set_values (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0xecf87cfa);
		}
		
		let (identifiers, initializer) = try! (vec_explode_2 (tokens));
		let identifiers = try! (vec_list_clone (&identifiers));
		let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
		
		let mut compilation = compilation;
		let mut bindings = StdVec::new ();
		for identifier in identifiers.into_iter () {
			let (compilation_1, binding) = try! (compilation.resolve (identifier));
			compilation = compilation_1;
			bindings.push (binding);
		}
		
		let (compilation, initializer) = try! (self.compile_0 (compilation, initializer));
		
		let initializer = try! (self.compile_syntax_binding_set_values_1 (bindings, initializer, false));
		
		succeed! ((compilation, initializer));
	}
	
	
	
	
	fn compile_syntax_binding_set_1 (&self, binding : CompilerBinding, expression : Expression, initialize : bool) -> (Outcome<Expression>) {
		match binding {
			
			CompilerBinding::Binding (_, binding, _) => {
				let expression = if initialize {
					ExpressionForContexts::BindingInitialize1 (binding, expression.into ()) .into ()
				} else {
					ExpressionForContexts::BindingSet1 (binding, expression.into ()) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (_, index, _) => {
				let expression = if initialize {
					ExpressionForContexts::RegisterInitialize1 (index, expression.into ()) .into ()
				} else {
					ExpressionForContexts::RegisterSet1 (index, expression.into ()) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Undefined =>
				fail! (0x42370d15),
			
		}
	}
	
	
	fn compile_syntax_binding_set_n (&self, bindings : StdVec<CompilerBinding>, expressions : StdVec<Expression>, parallel : bool, initialize : bool) -> (Outcome<Expression>) {
		
		if bindings.len () == 0 {
			fail! (0xf99d15e7);
		}
		if bindings.len () != expressions.len () {
			fail! (0x4a2cb09a);
		}
		
		let initializers = vec_zip_2 (bindings, expressions);
		
		match initializers[0].0 {
			
			CompilerBinding::Binding (_, _, _) => {
				let initializers = try_vec_map_into! (
						initializers,
						(binding, expression),
						match binding {
							CompilerBinding::Binding (_, binding, _) =>
								succeed! ((binding, expression)),
							_ =>
								fail! (0x31f5b387),
						});
				let expression = if initialize {
					ExpressionForContexts::BindingInitializeN (initializers.into_boxed_slice (), parallel) .into ()
				} else {
					ExpressionForContexts::BindingSetN (initializers.into_boxed_slice (), parallel) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (_, _, _) => {
				let initializers = try_vec_map_into! (
						initializers,
						(binding, expression),
						match binding {
							CompilerBinding::Register (_, index, _) =>
								succeed! ((index, expression)),
							_ =>
								fail! (0x9d0610d6),
						});
				let expression = if initialize {
					ExpressionForContexts::RegisterInitializeN (initializers.into_boxed_slice (), parallel) .into ()
				} else {
					ExpressionForContexts::RegisterSetN (initializers.into_boxed_slice (), parallel) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Undefined =>
				fail! (0xac48836a),
			
		}
	}
	
	
	
	
	fn compile_syntax_binding_set_values_1 (&self, bindings : StdVec<CompilerBinding>, expression : Expression, initialize : bool) -> (Outcome<Expression>) {
		
		if bindings.len () == 0 {
			fail! (0xe6c10f17);
		}
		
		match bindings[0] {
			
			CompilerBinding::Binding (_, _, _) => {
				let bindings = try_vec_map_into! (
						bindings,
						binding,
						match binding {
							CompilerBinding::Binding (_, binding, _) =>
								succeed! (binding),
							_ =>
								fail! (0xe59c62c6),
						});
				let expression = if initialize {
					ExpressionForContexts::BindingInitializeValues (bindings.into_boxed_slice (), expression.into ()) .into ()
				} else {
					ExpressionForContexts::BindingSetValues (bindings.into_boxed_slice (), expression.into ()) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (_, _, _) => {
				let bindings = try_vec_map_into! (
						bindings,
						binding,
						match binding {
							CompilerBinding::Register (_, index, _) =>
								succeed! (index),
							_ =>
								fail! (0x5627731f),
						});
				let expression = if initialize {
					ExpressionForContexts::RegisterInitializeValues (bindings.into_boxed_slice (), expression.into ()) .into ()
				} else {
					ExpressionForContexts::RegisterSetValues (bindings.into_boxed_slice (), expression.into ()) .into ()
				};
				succeed! (expression);
			},
			
			CompilerBinding::Undefined =>
				fail! (0x2d8c07dd),
			
		}
	}
	
	
	fn compile_syntax_binding_set_values_n (&self, bindings : StdVec<StdVec<CompilerBinding>>, expressions : StdVec<Expression>, initialize : bool) -> (Outcome<StdVec<Expression>>) {
		
		if bindings.len () == 0 {
			fail! (0x28e6a67b);
		}
		if bindings.len () != expressions.len () {
			fail! (0x3ab734fa);
		}
		
		let mut initializers = StdVec::with_capacity (bindings.len ());
		
		for (bindings, expression) in vec_zip_2 (bindings, expressions) {
			let initializer = try! (self.compile_syntax_binding_set_values_1 (bindings, expression, initialize));
			initializers.push (initializer);
		}
		
		succeed! (initializers);
	}
	
	
	
	
	fn compile_syntax_lambda (&self, compilation : CompilerContext, identifier : Option<Symbol>, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x2dfd91d1);
		}
		
		let (arguments, statements) = try! (vec_explode_1n (tokens));
		let (arguments_positional, argument_rest) = match arguments.class_match_into () {
			ValueClassMatchInto::Null =>
				(StdVec::new (), None),
			ValueClassMatchInto::Symbol (argument) =>
				(StdVec::new (), Some (argument)),
			ValueClassMatchInto::Pair (arguments) => {
				let (arguments_positional, argument_rest) = try! (vec_list_clone_dotted (&arguments.value ()));
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				(arguments_positional, argument_rest)
			},
			_ =>
				fail! (0x70773cab),
		};
		
		return self.compile_syntax_lambda_0 (compilation, identifier, arguments_positional, argument_rest, statements);
	}
	
	
	fn compile_syntax_lambda_let (&self, compilation : CompilerContext, identifier : Symbol, definitions : Value, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let definitions = try! (vec_list_clone (&definitions));
		
		let mut compilation = try! (compilation.fork_locals (true));
		
		let mut argument_identifiers = StdVec::with_capacity (definitions.len ());
		let mut argument_initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions.into_iter () {
			let definition = try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x4ad3c4b8);
			}
			let (identifier, initializer) = try! (vec_explode_2 (definition));
			let identifier = try_into_symbol! (identifier);
			let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
			argument_identifiers.push (identifier);
			argument_initializers.push (initializer);
			compilation = compilation_1;
		}
		
		let (compilation, lambda_binding) = try! (compilation.define (identifier.clone ()));
		let (compilation, lambda_value) = try! (self.compile_syntax_lambda_0 (compilation, Some (identifier.clone ()), argument_identifiers, None, statements));
		let (compilation, lambda_reference) = try! (self.compile_symbol (compilation, identifier.clone ()));
		
		let lambda_initializer = try! (self.compile_syntax_binding_set_1 (lambda_binding, lambda_value, true));
		let lambda_call = ExpressionForProcedureGenericCall::ProcedureCall (lambda_reference.into (), argument_initializers.into_boxed_slice ()) .into ();
		let statements = vec! [ lambda_initializer, lambda_call ];
		
		let (compilation, registers) = try! (compilation.unfork_locals ());
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	fn compile_syntax_lambda_0 (&self, compilation : CompilerContext, identifier : Option<Symbol>, arguments_positional : StdVec<Symbol>, argument_rest : Option<Symbol>, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = try! (compilation.fork_locals_with_bindings ());
		
		let mut compilation = try! (compilation.fork_locals (true));
		for identifier in &arguments_positional {
			let (compilation_1, _) = try! (compilation.define (identifier.clone ()));
			compilation = compilation_1;
		}
		if let Some (ref identifier) = argument_rest {
			let (compilation_1, _) = try! (compilation.define (identifier.clone ()));
			compilation = compilation_1;
		}
		
		let (compilation, statements) = try! (self.compile_0_vec (compilation, statements));
		
		let (compilation, registers_local) = try! (compilation.unfork_locals ());
		let (compilation, registers_closure) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		
		let template = LambdaTemplate::new (identifier, arguments_positional.into_boxed_slice (), argument_rest);
		
		let template = StdRc::new (template);
		let statements = StdRc::new (statements);
		let registers_closure = StdBox::from (registers_closure);
		let registers_local = StdRc::from (registers_local);
		let expression = Expression::Lambda (template, statements, registers_closure, registers_local);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_quote (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		let token = try! (vec_explode_1 (tokens));
		return self.compile_syntax_quote_0 (compilation, token);
	}
	
	fn compile_syntax_quote_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		succeed! ((compilation, Expression::Value (token)));
	}
	
	
	
	
	fn compile_syntax_quasi_quote (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		let token = try! (vec_explode_1 (tokens));
		return self.compile_syntax_quasi_quote_0 (compilation, token, true, false, 0, 0);
	}
	
	
	fn compile_syntax_quasi_quote_0 (&self, compilation : CompilerContext, token : Value, top : bool, spliceable : bool, quote_depth : usize, unquote_depth : usize) -> (Outcome<(CompilerContext, Expression)>) {
		
		fn splice <ExpressionInto : StdInto<Expression>> (expression : ExpressionInto, spliceable : bool) -> (Expression) {
			let expression = expression.into ();
			if spliceable {
				ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([expression])) .into ()
			} else {
				expression
			}
		}
		
		match token.class_match_into () {
			
			ValueClassMatchInto::Null =>
				succeed! ((compilation, splice (NULL, spliceable))),
			
			ValueClassMatchInto::Void =>
				succeed! ((compilation, splice (VOID, spliceable))),
			
			ValueClassMatchInto::Undefined =>
				succeed! ((compilation, splice (UNDEFINED, spliceable))),
			
			ValueClassMatchInto::Singleton (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Boolean (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Number (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Character (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::String (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Bytes (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Symbol (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Array (class) =>
				// FIXME:  Add support for quasi-quotation in arrays!
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Values (value) =>
				// FIXME:  Add support for quasi-quotation in values!
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Error (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Procedure (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Syntax (class) =>
				match class {
					SyntaxMatchInto::Primitive (syntax) =>
						match syntax {
							SyntaxPrimitive::PrimitiveV (syntax) =>
								match syntax {
									SyntaxPrimitiveV::Quote =>
										succeed! ((compilation, splice (symbol_clone_str ("quote"), spliceable))),
									SyntaxPrimitiveV::QuasiQuote =>
										succeed! ((compilation, splice (symbol_clone_str ("quasiquote"), spliceable))),
									SyntaxPrimitiveV::UnQuote =>
										succeed! ((compilation, splice (symbol_clone_str ("unquote"), spliceable))),
									SyntaxPrimitiveV::UnQuoteSplicing =>
										succeed! ((compilation, splice (symbol_clone_str ("unquote-splicing"), spliceable))),
									_ =>
										succeed! ((compilation, splice (syntax, spliceable))),
								},
							_ =>
								succeed! ((compilation, splice (syntax, spliceable))),
						},
					_ =>
						succeed! ((compilation, splice (class.value (), spliceable))),
				},
			
			ValueClassMatchInto::Port (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Resource (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Internal (_value) =>
				fail_unimplemented! (0x0bc54733), // deferred
			
			ValueClassMatchInto::Opaque (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Pair (class) => {
				
				let (callable, arguments) = class.left_and_right_into ();
				
				let (compilation, callable) = match try! (self.compile_form_match_callable (compilation, callable)) {
					
					(compilation, Alternative2::Variant1 (syntax)) => {
						let tokens = try! (vec_list_clone (&arguments));
						let tokens_count = tokens.len ();
						match syntax {
							
							SyntaxPrimitive::PrimitiveV (SyntaxPrimitiveV::UnQuote) =>
								if tokens_count == 1 {
									let token = try! (vec_explode_1 (tokens));
									let (compilation, element) = if quote_depth == unquote_depth {
										try! (self.compile_0 (compilation, token))
									} else {
										let (compilation, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth, unquote_depth + 1));
										// FIXME:  Eliminate dynamic creation of symbol!
										let element = ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("unquote") .into ()), element])) .into ();
										(compilation, element)
									};
									succeed! ((compilation, splice (element, spliceable)));
								} else {
									fail! (0x9dc44267);
								},
							
							SyntaxPrimitive::PrimitiveV (SyntaxPrimitiveV::UnQuoteSplicing) =>
								if tokens_count == 1 {
									if spliceable {
										let token = try! (vec_explode_1 (tokens));
										let (compilation, element) = if quote_depth == unquote_depth {
											try! (self.compile_0 (compilation, token))
										} else {
											let (compilation, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth, unquote_depth + 1));
											// FIXME:  Eliminate dynamic creation of symbol!
											let element = ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("unquote-splicing") .into ()), element])) .into ();
											(compilation, element)
										};
										succeed! ((compilation, element));
									} else {
										fail! (0x47356961);
									}
								} else {
									fail! (0xe0c45124);
								},
							
							SyntaxPrimitive::PrimitiveV (SyntaxPrimitiveV::QuasiQuote) =>
								if tokens_count == 1 {
									let token = try! (vec_explode_1 (tokens));
									let (compilation, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth + 1, unquote_depth));
									// FIXME:  Eliminate dynamic creation of symbol!
									let element : Expression = ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("quasiquote") .into ()), element])) .into ();
									succeed! ((compilation, splice (element, spliceable)));
								} else {
									fail! (0x95565615);
								},
							
							_ =>
								(compilation, syntax.into ()),
						}
					},
					
					(compilation, Alternative2::Variant2 (callable)) =>
						(compilation, callable),
					
				};
				
				let token : Value = pair_immutable_new (callable, arguments) .into ();
				
				let mut compilation = compilation;
				let mut elements = ExpressionVec::new ();
				let mut cursor = &token;
				loop {
					// FIXME:  Use `ListIterator`!
					match cursor.class () {
						
						ValueClass::Pair => {
							let pair = try_as_pair_immutable_ref! (cursor);
							let (compilation_1, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, pair.left () .clone (), false, true, quote_depth, unquote_depth));
							compilation = compilation_1;
							elements.push (element);
							cursor = pair.right ();
							if token.is_self (cursor) {
								fail! (0x150ac5b5);
							}
						},
						
						ValueClass::Null =>
							break,
						
						_ => {
							let (compilation_1, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, cursor.clone (), false, true, quote_depth, unquote_depth));
							compilation = compilation_1;
							elements.push (element);
							break;
						},
						
					}
				}
				
				let expression = ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitiveV::ListAppend.into (), elements.into_boxed_slice ()) .into ();
				
				let expression = if top {
					expression
				} else {
					ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitive2::Pair.into (), StdBox::new ([expression, NULL.into ()])) .into ()
				};
				
				succeed! ((compilation, expression));
			},
			
		}
	}
	
	
	
	
}




#[ derive (Debug) ]
pub struct CompilerContext {
	bindings : CompilerBindings,
}


impl CompilerContext {
	
	
	fn new_with_context (context : Option<&Context>) -> (CompilerContext) {
		if let Some (context) = context {
			return CompilerContext::new_with_bindings (CompilerBindings::Globals1 (context.clone (), true));
		} else {
			return CompilerContext::new_with_bindings (CompilerBindings::None);
		}
	}
	
	fn new_with_bindings (bindings : CompilerBindings) -> (CompilerContext) {
		return CompilerContext {
				bindings : bindings,
			};
	}
	
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerContext>) {
		let bindings = try! (self.bindings.fork_locals (force));
		succeed! (CompilerContext::new_with_bindings (bindings));
	}
	
	fn fork_locals_with_bindings (self) -> (Outcome<CompilerContext>) {
		let bindings = try! (self.bindings.fork_locals_with_bindings ());
		succeed! (CompilerContext::new_with_bindings (bindings));
	}
	
	fn unfork_locals (self) -> (Outcome<(CompilerContext, StdVec<RegisterTemplate>)>) {
		let (bindings, registers) = try! (self.bindings.unfork_locals ());
		succeed! ((CompilerContext::new_with_bindings (bindings), registers));
	}
	
	
	fn resolve (self, identifier : Symbol) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = try! (this.bindings.resolve (identifier));
		succeed! ((this, binding));
	}
	
	fn resolve_value (self, identifier : Symbol) -> (Outcome<(CompilerContext, Option<Value>)>) {
		let mut this = self;
		let value = try! (this.bindings.resolve_value (identifier));
		succeed! ((this, value));
	}
	
	fn define (self, identifier : Symbol) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = try! (this.bindings.define (identifier));
		succeed! ((this, binding));
	}
	
	fn define_enable (self) -> (CompilerContext) {
		let mut this = self;
		this.bindings.define_enable ();
		return this;
	}
	
	fn define_disable (self) -> (CompilerContext) {
		let mut this = self;
		this.bindings.define_disable ();
		return this;
	}
}




#[ derive (Debug) ]
pub enum CompilerBindings {
	None,
	Globals1 (Context, bool),
	Globals2 (StdBox<CompilerBindings>, Context, bool),
	Locals (StdBox<CompilerBindings>, StdMap<Symbol, CompilerBinding>, StdVec<RegisterTemplate>, bool, bool),
}


#[ derive (Clone, Debug) ]
pub enum CompilerBinding {
	Undefined,
	Binding (Symbol, Binding, Option<BindingTemplate>),
	Register (Symbol, usize, RegisterTemplate),
}


impl CompilerBindings {
	
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerBindings>) {
		if force {
			succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), false, true));
		} else {
			match self {
				CompilerBindings::None =>
					fail! (0xad3e033b),
				CompilerBindings::Globals1 (_, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None), true)),
				CompilerBindings::Globals2 (_, _, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None), true)),
				CompilerBindings::Locals (_, _, _, _, _) =>
					succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), false, true)),
			}
		}
	}
	
	fn fork_locals_with_bindings (self) -> (Outcome<CompilerBindings>) {
		succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), true, true));
	}
	
	
	fn unfork_locals (self) -> (Outcome<(CompilerBindings, StdVec<RegisterTemplate>)>) {
		match self {
			CompilerBindings::None =>
				fail! (0x98657e5a),
			CompilerBindings::Globals1 (_, _) =>
				fail! (0xdd470d36),
			CompilerBindings::Globals2 (parent, _, _) =>
				succeed! ((*parent, StdVec::new ())),
			CompilerBindings::Locals (parent, _, registers, _, _) =>
				succeed! ((*parent, registers)),
		}
	}
	
	
	fn resolve (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		return self.resolve_0 (identifier, false);
	}
	
	fn resolve_0 (&mut self, identifier : Symbol, force_binding : bool) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				succeed! (CompilerBinding::Undefined),
			CompilerBindings::Globals1 (ref context, _) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (identifier, binding, None));
				} else {
					succeed! (CompilerBinding::Undefined);
				},
			CompilerBindings::Globals2 (ref mut parent, ref context, _) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (identifier, binding, None));
				} else {
					return parent.resolve_0 (identifier, force_binding);
				},
			CompilerBindings::Locals (ref mut parent, ref mut cached, ref mut registers, _, _) => {
				let transform_into_binding = if let Some (binding) = cached.get (&identifier) {
					match *binding {
						ref binding @ CompilerBinding::Undefined |
						ref binding @ CompilerBinding::Binding (_, _, _) |
						ref binding @ CompilerBinding::Register (_, _, RegisterTemplate::Borrow (_)) |
						ref binding @ CompilerBinding::Register (_, _, RegisterTemplate::LocalBinding (_)) =>
							succeed! (binding.clone ()),
						ref binding @ CompilerBinding::Register (_, _, RegisterTemplate::LocalValue (_, _)) if ! force_binding =>
							succeed! (binding.clone ()),
						CompilerBinding::Register (_, index, RegisterTemplate::LocalValue (ref value, immutable)) =>
							Some ((index, BindingTemplate { identifier : None, value : value.clone (), immutable : immutable })),
					}
				} else {
					None
				};
				if let Some ((index, template)) = transform_into_binding {
					let template = RegisterTemplate::LocalBinding (template);
					let binding = CompilerBinding::Register (identifier.clone (), index, template.clone ());
					registers[index] = template;
					cached.insert (identifier, binding.clone ());
					succeed! (binding);
				}
				{
					match try! (parent.resolve_0 (identifier.clone (), true)) {
						CompilerBinding::Undefined =>
							succeed! (CompilerBinding::Undefined),
						binding @ CompilerBinding::Binding (_, _, _) => {
							cached.insert (identifier, binding.clone ());
							succeed! (binding);
						},
						CompilerBinding::Register (_, parent_index, _) => {
							let self_index = registers.len ();
							let template = RegisterTemplate::Borrow (parent_index);
							let self_binding = CompilerBinding::Register (identifier.clone (), self_index, template.clone ());
							registers.push (template);
							cached.insert (identifier, self_binding.clone ());
							succeed! (self_binding);
						}
					}
				}
			},
		}
	}
	
	
	fn resolve_value (&mut self, identifier : Symbol) -> (Outcome<Option<Value>>) {
		match try! (self.resolve (identifier)) {
			CompilerBinding::Binding (_, binding, _) =>
				succeed! (try! (binding.get_option ())),
			CompilerBinding::Register (_, _, _) =>
				succeed! (None),
			CompilerBinding::Undefined =>
				succeed! (None),
		}
	}
	
	
	fn define (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				fail! (0xd943456d),
			CompilerBindings::Globals1 (ref context, define_allowed) | CompilerBindings::Globals2 (_, ref context, define_allowed) => {
				if ! define_allowed {
					fail! (0x0d9133ab);
				}
				let template = BindingTemplate {
						identifier : Some (identifier.clone ()),
						value : None,
						immutable : false,
					};
				let binding = try! (context.define (&template));
				succeed! (CompilerBinding::Binding (identifier, binding, Some (template)));
			},
			CompilerBindings::Locals (_, ref mut cached, ref mut registers, force_binding, define_allowed) => {
				if ! define_allowed {
					fail! (0xaf5074cb);
				}
				let index = registers.len ();
				let template = if force_binding {
					let template = BindingTemplate { identifier : None, value : None, immutable : false };
					RegisterTemplate::LocalBinding (template)
				} else {
					RegisterTemplate::LocalValue (None, false)
				};
				let binding = CompilerBinding::Register (identifier.clone (), index, template.clone ());
				registers.push (template);
				cached.insert (identifier, binding.clone ());
				succeed! (binding);
			},
		}
	}
	
	
	fn undefine (&mut self, binding : &CompilerBinding) -> (Outcome<()>) {
		match *binding {
			CompilerBinding::Undefined =>
				fail! (0x1670a9f8),
			CompilerBinding::Binding (_, _, _) =>
				fail! (0x06fec793),
			CompilerBinding::Register (ref identifier, _, _) =>
				match *self {
					CompilerBindings::None =>
						fail! (0x3a1f3306),
					CompilerBindings::Globals1 (_, _) =>
						fail! (0xcdf142d8),
					CompilerBindings::Globals2 (_, _, _) =>
						fail! (0x382ba35e),
					CompilerBindings::Locals (_, ref mut cached, _, _, _) => {
						cached.remove (identifier);
						succeed! (());
					},
				},
		}
	}
	
	
	fn define_disable (&mut self) -> () {
		match *self {
			CompilerBindings::None =>
				(),
			CompilerBindings::Globals1 (_, ref mut define_allowed) =>
				*define_allowed = false,
			CompilerBindings::Globals2 (_, _, ref mut define_allowed) =>
				*define_allowed = false,
			CompilerBindings::Locals (_, _, _, _, ref mut define_allowed) =>
				*define_allowed = false,
		}
	}
	
	fn define_enable (&mut self) -> () {
		match *self {
			CompilerBindings::None =>
				(),
			CompilerBindings::Globals1 (_, ref mut define_allowed) =>
				*define_allowed = true,
			CompilerBindings::Globals2 (_, _, ref mut define_allowed) =>
				*define_allowed = true,
			CompilerBindings::Locals (_, _, _, _, ref mut define_allowed) =>
				*define_allowed = true,
		}
	}
}

