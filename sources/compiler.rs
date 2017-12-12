

use super::builtins::exports::*;
use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::lambdas::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::compile;
	pub use super::compile_script;
	pub use super::Compiler;
}




pub fn compile (context : &Context, token : &Value) -> (Outcome<Expression>) {
	return Compiler::new () .compile (context, token);
}

pub fn compile_script (context : &Context, tokens : &[Value]) -> (Outcome<ExpressionVec>) {
	let compiler = Compiler::new ();
	let mut expressions = StdVec::with_capacity (tokens.len ());
	for token in tokens {
		let expression = try! (compiler.compile (context, token));
		expressions.push (expression);
	}
	succeed! (expressions);
}




#[ derive (Clone, Debug) ]
pub struct Compiler {}


impl Compiler {
	
	
	
	
	pub fn new () -> (Compiler) {
		return Compiler {};
	}
	
	pub fn compile (&self, context : &Context, token : &Value) -> (Outcome<Expression>) {
		let compilation = CompilerContext::new (CompilerBindings::Globals1 (context.clone ()));
		let (_compilation, expression) = try! (self.compile_0 (compilation, token.clone ()));
		succeed! (expression);
	}
	
	
	
	
	fn compile_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		match token.class () {
			
			ValueClass::Null |
			ValueClass::Void |
			ValueClass::Undefined |
			ValueClass::Singleton =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::Boolean |
			ValueClass::NumberInteger |
			ValueClass::NumberReal |
			ValueClass::Character =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::String =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::Symbol =>
				return self.compile_symbol (compilation, token.into ()),
			
			ValueClass::Pair =>
				return self.compile_form (compilation, token.into ()),
			
			ValueClass::Bytes =>
				return self.compile_syntax_quote (compilation, token),
			ValueClass::Array =>
				return self.compile_syntax_quote (compilation, token),
			ValueClass::Values =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::Error =>
				fail_panic! (0x2aa7bc60),
			
			ValueClass::ProcedurePrimitive =>
				fail_panic! (0xa9e5d4ca),
			ValueClass::ProcedureExtended =>
				fail_panic! (0xaf6f1288),
			ValueClass::ProcedureLambda =>
				fail_panic! (0x2c4bb21a),
			ValueClass::SyntaxPrimitive =>
				fail_panic! (0x09e47c84),
			ValueClass::SyntaxExtended =>
				fail_panic! (0xe781b659),
			ValueClass::SyntaxLambda =>
				fail_panic! (0x7f9c4bb4),
			
			ValueClass::Port =>
				fail_panic! (0x05fe5b73),
			
			ValueClass::Binding =>
				fail_panic! (0x7172b055),
			ValueClass::Context =>
				fail_panic! (0x5f0d7003),
			
		}
	}
	
	
	
	
	fn compile_vec_0 (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, ExpressionVec)>) {
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
		let mut compilation = compilation;
		match try! (compilation.bindings.resolve (identifier)) {
			CompilerBinding::Undefined =>
				fail! (0xc6825cfd),
			CompilerBinding::Binding (binding) =>
				succeed! ((compilation, Expression::BindingGet1 (binding))),
			CompilerBinding::Register (index) =>
				succeed! ((compilation, Expression::RegisterGet1 (index))),
		}
	}
	
	
	
	
	fn compile_form (&self, compilation : CompilerContext, form : Pair) -> (Outcome<(CompilerContext, Expression)>) {
		
		match try! (self.compile_form_0 (compilation, form.clone ())) {
			
			(compilation, Some ((syntax, tokens))) =>
				return self.compile_syntax_call (compilation, syntax, tokens),
			
			(compilation, None) =>
				return self.compile_procedure_call (compilation, form.left () .clone (), form.right () .clone ()),
		}
	}
	
	
	fn compile_form_0 (&self, compilation : CompilerContext, token : Pair) -> (Outcome<(CompilerContext, Option<(SyntaxPrimitive, Value)>)>) {
		
		let mut compilation = compilation;
		let callable = token.left () .clone ();
		let arguments = token.right () .clone ();
		
		match callable.class () {
			
			ValueClass::Symbol => {
				if let Some (callable) = try! (compilation.bindings.resolve_value (callable.into ())) {
					match callable.class () {
						ValueClass::SyntaxPrimitive =>
							succeed! ((compilation, Some ((callable.into (), arguments)))),
						_ =>
							succeed! ((compilation, None)),
					}
				} else {
					succeed! ((compilation, None));
				}
			},
			
			ValueClass::SyntaxPrimitive =>
				succeed! ((compilation, Some ((callable.into (), arguments)))),
			
			_ =>
				succeed! ((compilation, None)),
			
		}
	}
	
	
	
	
	fn compile_procedure_call (&self, compilation : CompilerContext, procedure : Value, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, procedure) = try! (self.compile_0 (compilation, procedure));
		let (compilation, arguments) = try! (self.compile_vec_0 (compilation, try! (vec_list_clone (&arguments))));
		
		let expression = Expression::ProcedureCall (procedure.into (), arguments.into_boxed_slice ());
		
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
						fail_unimplemented! (0x2e2b0079),
					
					SyntaxPrimitiveV::While |
					SyntaxPrimitiveV::Until =>
						fail_unimplemented! (0xdae6d716),
					
					SyntaxPrimitiveV::WhileCond |
					SyntaxPrimitiveV::UntilCond =>
						fail_unimplemented! (0x9e9861c0),
					
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
				fail! (0x1aed14f3),
			
			SyntaxPrimitive::Unimplemented =>
				fail_unimplemented! (0xa4e41f62), // OK
			
			SyntaxPrimitive::Unsupported =>
				fail_unimplemented! (0x175a7f9e), // OK
			
			SyntaxPrimitive::Reserved =>
				fail! (0x1aed14f3),
			
		}
	}
	
	
	
	
	fn compile_syntax_and_or (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveN, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		
		let expression = Expression::SyntaxPrimitiveCall (syntax.into (), statements.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_begin (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		
		let expression = Expression::Sequence (statements.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_if (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if (tokens_count != 2) && (tokens_count != 3) {
			fail! (0xe34389a7);
		}
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		
		let clauses = if tokens_count == 3 {
			let (guard, if_true, if_false) = try! (vec_explode_3 (statements));
			vec! [
					(Some ((guard, false)), Some (if_true)),
					(None, Some (if_false)),
				]
		} else if tokens_count == 2 {
			let (guard, if_true) = try! (vec_explode_2 (statements));
			vec! [
					(Some ((guard, false)), Some (if_true)),
				]
		} else {
			fail_unreachable! (0xbc801c5d);
		};
		
		let expression = Expression::ConditionalIf (clauses.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_when_unless (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveN, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if tokens_count < 2 {
			fail! (0x3c364a9f);
		}
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		let (guard, statements) = try! (vec_explode_1n (statements));
		let statements = Expression::Sequence (statements.into_boxed_slice ());
		
		let negated = match syntax {
			SyntaxPrimitiveV::When =>
				false,
			SyntaxPrimitiveV::Unless =>
				true,
			_ =>
				fail_unreachable! (0x500d298f),
		};
		
		let clauses = vec! [
				(Some ((guard, negated)), Some (statements)),
			];
		let expression = Expression::ConditionalIf (clauses.into_boxed_slice ());
		
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
			
			let (compilation_1, guard) = if ! (guard.is (ValueClass::Symbol) && Symbol::as_ref (&guard) .string_eq ("else")) {
				let (compilation_1, guard) = try! (self.compile_0 (compilation, guard));
				(compilation_1, Some ((guard, false)))
			} else {
				(compilation, None)
			};
			
			if (statements.len () >= 1) && (statements[0].is (ValueClass::Symbol) && Symbol::as_ref (&statements[0]) .string_eq ("=>")) {
				fail_unimplemented! (0xfa332991); // deferred
			}
			let (compilation_1, statements) = try! (self.compile_vec_0 (compilation_1, statements));
			
			let clause = if !statements.is_empty () {
				let statements = Expression::Sequence (statements.into_boxed_slice ());
				(guard, Some (statements))
			} else {
				(guard, None)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let expression = Expression::ConditionalIf (clauses.into_boxed_slice ());
		
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
			
			let expected = if ! (expected.is (ValueClass::Symbol) && Symbol::as_ref (&expected) .string_eq ("else")) {
				let expected = try! (vec_list_clone (&expected));
				Some ((expected.into_boxed_slice (), false))
			} else {
				None
			};
			
			if (statements.len () >= 1) && (statements[0].is (ValueClass::Symbol) && Symbol::as_ref (&statements[0]) .string_eq ("=>")) {
				fail_unimplemented! (0xef5d468c); // deferred
			}
			let (compilation_1, statements) = try! (self.compile_vec_0 (compilation, statements));
			
			let clause = if !statements.is_empty () {
				let statements = Expression::Sequence (statements.into_boxed_slice ());
				(expected, Some (statements))
			} else {
				(expected, None)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let expression = Expression::ConditionalMatch (actual.into (), clauses.into_boxed_slice ());
		
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
			let (identifier, initializer, updater) = try! (vec_explode_3 (definition));
			let identifier = try_into_symbol! (identifier);
			identifiers.push (identifier);
			initializers.push (initializer);
			updaters.push (updater);
		}
		
		let (compilation, has_definitions) = if identifiers.is_empty () {
			(compilation, false)
		} else {
			let compilation = try! (compilation.fork_locals (false));
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
				let binding = try! (compilation.bindings.define (identifier));
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
		let (compilation, break_statements) = if break_statements.is_empty () {
			(compilation, None)
		} else {
			let (compilation, break_statements) = try! (self.compile_vec_0 (compilation, break_statements));
			let break_statements = Expression::Sequence (break_statements.into_boxed_slice ());
			(compilation, Some (break_statements))
		};
		let break_clauses = StdBox::new ([(Some ((break_guard, false)), break_statements)]);
		
		let (compilation, loop_statement) = if loop_statements.is_empty () {
			(compilation, None)
		} else {
			let (compilation, loop_statements) = try! (self.compile_vec_0 (compilation, loop_statements));
			let loop_statements = Expression::Sequence (loop_statements.into_boxed_slice ());
			(compilation, Some (loop_statements.into ()))
		};
		
		let expression = Expression::Loop (binding_initializers, binding_updaters, loop_statement, break_clauses);
		
		let (compilation, expression) = if has_definitions {
			let (compilation, registers) = try! (compilation.unfork_locals ());
			let expression = Expression::RegisterClosure (expression.into (), registers.into_boxed_slice ());
			(compilation, expression)
		} else {
			(compilation, expression)
		};
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_locals (&self, compilation : CompilerContext, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = try! (compilation.fork_locals (false));
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (statements.into_boxed_slice ());
		let expression = Expression::RegisterClosure (statements.into (), registers.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_let (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x633b3ed8);
		}
		let (definitions, statements) = try! (vec_explode_1n (tokens));
		
		match definitions.class () {
			ValueClass::Null =>
				return self.compile_syntax_locals (compilation, statements),
			ValueClass::Pair =>
				(),
			_ =>
				fail! (0x825cb457),
		}
		
		let definitions = try! (vec_list_clone (&definitions));
		
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
		
		let mut compilation = try! (compilation.fork_locals (false));
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
					let binding = try! (compilation.bindings.define (identifier));
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetSequential => {
				for (initializer, identifier) in initializers.into_iter ().zip (identifiers.into_iter ()) {
					let (compilation_1, initializer) = try! (self.compile_0 (compilation, initializer));
					compilation = compilation_1;
					let binding = try! (compilation.bindings.define (identifier));
					binding_initializers.push (initializer);
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetRecursiveParallel | SyntaxPrimitiveV::LetRecursiveSequential => {
				for identifier in identifiers.into_iter () {
					let binding = try! (compilation.bindings.define (identifier));
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
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (statements.into_boxed_slice ());
		let expression = Expression::RegisterClosure (statements.into (), registers.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_let_values (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x10672a0d);
		}
		let (definitions, statements) = try! (vec_explode_1n (tokens));
		
		match definitions.class () {
			ValueClass::Null =>
				return self.compile_syntax_locals (compilation, statements),
			ValueClass::Pair =>
				(),
			_ =>
				fail! (0x60cfd87a),
		}
		
		let definitions = try! (vec_list_clone (&definitions));
		
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
		
		let mut compilation = try! (compilation.fork_locals (false));
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
						let binding = try! (compilation.bindings.define (identifier));
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
						let binding = try! (compilation.bindings.define (identifier));
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
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (statements.into_boxed_slice ());
		let expression = Expression::RegisterClosure (statements.into (), registers.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_define (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x4481879c);
		}
		let (signature, statements) = try! (vec_explode_1n (tokens));
		let mut compilation = compilation;
		
		let (compilation, binding, expression) = match signature.class () {
			
			ValueClass::Symbol => {
				
				if statements.len () != 1 {
					fail! (0xc364edf8);
				}
				
				let identifier = try_into_symbol! (signature);
				let statement = try! (vec_explode_1 (statements));
				
				let binding = try! (compilation.bindings.define (identifier));
				let (compilation, expression) = try! (self.compile_0 (compilation, statement));
				
				(compilation, binding, expression)
			},
			
			ValueClass::Pair => {
				
				if statements.len () < 1 {
					fail! (0x48c70de5);
				}
				
				let (signature, argument_rest) = try! (vec_list_clone_dotted (&signature));
				let (identifier, arguments_positional) = try! (vec_explode_1n (signature));
				
				let identifier = try_into_symbol! (identifier);
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				
				let binding = try! (compilation.bindings.define (identifier.clone ()));
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
			let binding = try! (compilation.bindings.define (identifier));
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
		
		let mut compilation = compilation;
		let binding = try! (compilation.bindings.resolve (identifier));
		
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
			let binding = try! (compilation.bindings.resolve (identifier));
			bindings.push (binding);
		}
		
		let (compilation, initializer) = try! (self.compile_0 (compilation, initializer));
		
		let initializer = try! (self.compile_syntax_binding_set_values_1 (bindings, initializer, false));
		
		succeed! ((compilation, initializer));
	}
	
	
	
	
	fn compile_syntax_binding_set_1 (&self, binding : CompilerBinding, expression : Expression, initialize : bool) -> (Outcome<Expression>) {
		match binding {
			
			CompilerBinding::Undefined =>
				fail! (0x42370d15),
			
			CompilerBinding::Binding (binding) => {
				let expression = if initialize {
					Expression::BindingInitialize1 (binding, expression.into ())
				} else {
					Expression::BindingSet1 (binding, expression.into ())
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (index) => {
				let expression = if initialize {
					Expression::RegisterInitialize1 (index, expression.into ())
				} else {
					Expression::RegisterSet1 (index, expression.into ())
				};
				succeed! (expression);
			},
			
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
			
			CompilerBinding::Undefined =>
				fail! (0xac48836a),
			
			CompilerBinding::Binding (_) => {
				let initializers = try_vec_map_into! (
						initializers,
						(binding, expression),
						match binding {
							CompilerBinding::Binding (binding) =>
								succeed! ((binding, expression)),
							_ =>
								fail! (0x31f5b387),
						});
				let expression = if initialize {
					Expression::BindingInitializeN (initializers.into_boxed_slice (), parallel)
				} else {
					Expression::BindingSetN (initializers.into_boxed_slice (), parallel)
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (_) => {
				let initializers = try_vec_map_into! (
						initializers,
						(binding, expression),
						match binding {
							CompilerBinding::Register (index) =>
								succeed! ((index, expression)),
							_ =>
								fail! (0x5627731f),
						});
				let expression = if initialize {
					Expression::RegisterInitializeN (initializers.into_boxed_slice (), parallel)
				} else {
					Expression::RegisterSetN (initializers.into_boxed_slice (), parallel)
				};
				succeed! (expression);
			},
			
		}
	}
	
	
	
	
	fn compile_syntax_binding_set_values_1 (&self, bindings : StdVec<CompilerBinding>, expression : Expression, initialize : bool) -> (Outcome<Expression>) {
		
		if bindings.len () == 0 {
			fail! (0xe6c10f17);
		}
		
		match bindings[0] {
			
			CompilerBinding::Undefined =>
				fail! (0x2d8c07dd),
			
			CompilerBinding::Binding (_) => {
				let bindings = try_vec_map_into! (
						bindings,
						binding,
						match binding {
							CompilerBinding::Binding (binding) =>
								succeed! (binding),
							_ =>
								fail! (0xe59c62c6),
						});
				let expression = if initialize {
					Expression::BindingInitializeValues (bindings.into_boxed_slice (), expression.into ())
				} else {
					Expression::BindingSetValues (bindings.into_boxed_slice (), expression.into ())
				};
				succeed! (expression);
			},
			
			CompilerBinding::Register (_) => {
				let bindings = try_vec_map_into! (
						bindings,
						binding,
						match binding {
							CompilerBinding::Register (index) =>
								succeed! (index),
							_ =>
								fail! (0x5627731f),
						});
				let expression = if initialize {
					Expression::RegisterInitializeValues (bindings.into_boxed_slice (), expression.into ())
				} else {
					Expression::RegisterSetValues (bindings.into_boxed_slice (), expression.into ())
				};
				succeed! (expression);
			},
		
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
		let (arguments_positional, argument_rest) = match arguments.class () {
			ValueClass::Symbol =>
				(StdVec::new (), Some (Symbol::from (arguments))),
			ValueClass::Pair | ValueClass::Null => {
				let (arguments_positional, argument_rest) = try! (vec_list_clone_dotted (&arguments));
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				(arguments_positional, argument_rest)
			},
			_ =>
				fail! (0x70773cab),
		};
		
		return self.compile_syntax_lambda_0 (compilation, identifier, arguments_positional, argument_rest, statements);
	}
	
	
	fn compile_syntax_lambda_0 (&self, compilation : CompilerContext, identifier : Option<Symbol>, arguments_positional : StdVec<Symbol>, argument_rest : Option<Symbol>, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let arguments_count = arguments_positional.len () + if argument_rest.is_some () { 1 } else { 0 };
		
		let compilation = try! (compilation.fork_locals (true));
		
		let mut compilation = try! (compilation.fork_locals (true));
		for identifier in &arguments_positional {
			try! (compilation.bindings.define (identifier.clone ()));
		}
		if let Some (ref identifier) = argument_rest {
			try! (compilation.bindings.define (identifier.clone ()));
		}
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		
		let (compilation, mut registers_local) = try! (compilation.unfork_locals ());
		let (compilation, registers_closure) = try! (compilation.unfork_locals ());
		
		let registers_local = registers_local.split_off (arguments_count);
		
		let statements = Expression::Sequence (statements.into_boxed_slice ());
		
		let template = LambdaTemplate {
				identifier : identifier,
				arguments_positional : arguments_positional,
				argument_rest : argument_rest,
			};
		
		let expression = Expression::Lambda (StdBox::new (template), statements.into (), registers_closure.into_boxed_slice (), registers_local.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_quote (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		succeed! ((compilation, Expression::Value (token)));
	}
	
	
	
	
	fn compile_syntax_quasi_quote (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		return self.compile_syntax_quasi_quote_0 (compilation, token, true, false, 0, 0);
	}
	
	
	fn compile_syntax_quasi_quote_0 (&self, compilation : CompilerContext, token : Value, top : bool, spliceable : bool, quote_depth : usize, unquote_depth : usize) -> (Outcome<(CompilerContext, Expression)>) {
		
		fn splice <ExpressionInto : StdInto<Expression>> (expression : ExpressionInto, spliceable : bool) -> (Expression) {
			let expression = expression.into ();
			if spliceable {
				Expression::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([expression]))
			} else {
				expression
			}
		}
		
		match token.class () {
			
			ValueClass::Null |
			ValueClass::Void |
			ValueClass::Undefined |
			ValueClass::Singleton =>
				succeed! ((compilation, splice (token, spliceable))),
			
			ValueClass::Boolean |
			ValueClass::NumberInteger |
			ValueClass::NumberReal |
			ValueClass::Character =>
				succeed! ((compilation, splice (token, spliceable))),
			
			ValueClass::String |
			ValueClass::Bytes =>
				succeed! ((compilation, splice (token, spliceable))),
			
			ValueClass::Symbol =>
				succeed! ((compilation, splice (token, spliceable))),
			ValueClass::Array =>
				fail_unimplemented! (0x0d99c57b),
			ValueClass::Values =>
				fail_unimplemented! (0x0da960b2),
			
			ValueClass::Error =>
				fail! (0x9681733a),
			
			ValueClass::ProcedurePrimitive =>
				fail! (0x89c49854),
			ValueClass::ProcedureExtended =>
				fail! (0xc3fb9b61),
			ValueClass::ProcedureLambda =>
				fail! (0xf3b07bb7),
			ValueClass::SyntaxPrimitive =>
				fail! (0x251a7fd0),
			ValueClass::SyntaxExtended =>
				fail! (0x567a02a2),
			ValueClass::SyntaxLambda =>
				fail! (0xbe7157a3),
			
			ValueClass::Port =>
				fail! (0x9c039a72),
			
			ValueClass::Binding =>
				fail! (0xdf21f737),
			ValueClass::Context =>
				fail! (0xfa7ef6f6),
			
			ValueClass::Pair => {
				
				let compilation = match try! (self.compile_form_0 (compilation, token.clone () .into ())) {
					
					(compilation, Some ((syntax, tokens))) => {
						let tokens = try! (vec_list_clone (&tokens));
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
										let element = Expression::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("unquote") .into ()), element]));
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
											let element = Expression::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("unquote-splicing") .into ()), element]));
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
									let element = Expression::ProcedureCall (ListPrimitiveV::ListBuild.into (), StdBox::new ([Expression::Value (symbol_clone_str ("quasiquote") .into ()), element]));
									succeed! ((compilation, splice (element, spliceable)));
								} else {
									fail! (0x95565615);
								},
							
							_ =>
								compilation,
						}
					},
					
					(compilation, None) =>
						compilation,
					
				};
				
				let mut compilation = compilation;
				let mut elements = ExpressionVec::new ();
				let mut cursor = &token;
				loop {
					match cursor.class () {
						
						ValueClass::Pair => {
							let pair = cursor.as_ref () as &Pair;
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
				
				let expression = Expression::ProcedureCall (ListPrimitiveV::ListAppend.into (), elements.into_boxed_slice ());
				
				let expression = if top {
					expression
				} else {
					Expression::ProcedureCall (ListPrimitive2::Pair.into (), StdBox::new ([expression, NULL.into ()]))
				};
				
				succeed! ((compilation, expression));
			},
			
		}
	}
	
	
	
	
}




#[ derive (Clone, Debug) ]
struct CompilerContext {
	bindings : CompilerBindings,
}


impl CompilerContext {
	
	fn new (bindings : CompilerBindings) -> (CompilerContext) {
		return CompilerContext {
				bindings : bindings,
			};
	}
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerContext>) {
		let bindings = try! (self.bindings.fork_locals (force));
		succeed! (CompilerContext::new (bindings));
	}
	
	fn unfork_locals (self) -> (Outcome<(CompilerContext, StdVec<RegistersBindingTemplate>)>) {
		let (bindings, registers) = try! (self.bindings.unfork_locals ());
		succeed! ((CompilerContext::new (bindings), registers));
	}
}




#[ derive (Clone, Debug) ]
enum CompilerBindings {
	None,
	Globals1 (Context),
	Globals2 (StdBox<CompilerBindings>, Context),
	Locals (StdBox<CompilerBindings>, StdMap<Symbol, CompilerBinding>, StdVec<RegistersBindingTemplate>, usize),
}


#[ derive (Clone, Debug) ]
enum CompilerBinding {
	Undefined,
	Binding (Binding),
	Register (usize),
}


impl CompilerBindings {
	
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerBindings>) {
		if force {
			succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), 1));
		} else {
			match self {
				CompilerBindings::None =>
					fail! (0xad3e033b),
				CompilerBindings::Globals1 (_) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None))),
				CompilerBindings::Globals2 (_, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None))),
				CompilerBindings::Locals (_, _, _, depth) =>
					succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), depth + 1)),
			}
		}
	}
	
	fn unfork_locals (self) -> (Outcome<(CompilerBindings, StdVec<RegistersBindingTemplate>)>) {
		match self {
			CompilerBindings::None =>
				fail! (0x98657e5a),
			CompilerBindings::Globals1 (_) =>
				fail! (0xdd470d36),
			CompilerBindings::Globals2 (parent, _) =>
				succeed! ((*parent, StdVec::new ())),
			CompilerBindings::Locals (parent, _, registers, _) =>
				succeed! ((*parent, registers)),
		}
	}
	
	
	fn resolve (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				succeed! (CompilerBinding::Undefined),
			CompilerBindings::Globals1 (ref context) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (binding));
				} else {
					succeed! (CompilerBinding::Undefined);
				},
			CompilerBindings::Globals2 (ref mut parent, ref context) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (binding));
				} else {
					return parent.resolve (identifier);
				},
			CompilerBindings::Locals (ref mut parent, ref mut cached, ref mut registers, _depth) => {
				if let Some (binding) = cached.get (&identifier) {
					succeed! (binding.clone ());
				} /*else*/ {
					match try! (parent.resolve (identifier.clone ())) {
						CompilerBinding::Undefined =>
							succeed! (CompilerBinding::Undefined),
						binding @ CompilerBinding::Binding (_) =>
							succeed! (binding),
						CompilerBinding::Register (parent_index) => {
							let self_index = registers.len ();
							let self_binding = CompilerBinding::Register (self_index);
							let template = RegistersBindingTemplate {
									identifier : Some (identifier.clone ()),
									borrow : Some (parent_index),
									value : None,
									immutable : false,
								};
							registers.push (template);
							cached.insert (identifier, self_binding.clone ());
							succeed! (self_binding);
						}
					}
				}
			},
		}
	}
	
	fn define (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				fail! (0xd943456d),
			CompilerBindings::Globals1 (ref context) | CompilerBindings::Globals2 (_, ref context) => {
				let template = ContextBindingTemplate {
						identifier : identifier,
						value : None,
						immutable : false,
					};
				let binding = try! (context.define (&template));
				succeed! (CompilerBinding::Binding (binding));
			},
			CompilerBindings::Locals (ref _parent, ref mut cached, ref mut registers, _depth) => {
				let index = registers.len ();
				let binding = CompilerBinding::Register (index);
				let template = RegistersBindingTemplate {
						identifier : Some (identifier.clone ()),
						borrow : None,
						value : None,
						immutable : false,
					};
				registers.push (template);
				cached.insert (identifier, binding.clone ());
				succeed! (binding);
			},
		}
	}
	
	
	fn resolve_value (&mut self, identifier : Symbol) -> (Outcome<Option<Value>>) {
		match try! (self.resolve (identifier)) {
			CompilerBinding::Undefined =>
				succeed! (None),
			CompilerBinding::Binding (binding) =>
				succeed! (Some (try! (binding.get ()))),
			CompilerBinding::Register (_index) =>
				succeed! (None),
		}
	}
}

