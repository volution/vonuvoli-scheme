

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

use super::prelude::*;




pub mod exports {
	
	pub use super::compile;
	pub use super::compile_script;
	
	pub use super::Compiler;
	pub use super::CompilerConfiguration;
	pub use super::CompilerContext;
	pub use super::CompilerBindings;
	pub use super::CompilerBinding;
	
	#[ cfg ( feature = "vonuvoli_optimizer" ) ]
	pub use super::super::compiler_optimizer::exports::*;
	
}




#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
#[ cfg ( feature = "vonuvoli_transcript" ) ]
def_transcript! (transcript);




pub fn compile (context : &Context, token : &Value, configuration : Option<&CompilerConfiguration>) -> (Outcome<Expression>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return Compiler::new (configuration) .compile (Some (context), token);
}

pub fn compile_script (context : &Context, tokens : &[Value], configuration : Option<&CompilerConfiguration>) -> (Outcome<ExpressionVec>) {
	let configuration = configuration.cloned () .unwrap_or_default ();
	return Compiler::new (configuration) .compile_slice (Some (context), tokens);
}




pub struct Compiler {
	configuration : CompilerConfiguration,
}


impl Compiler {
	
	
	
	
	pub fn new (configuration : CompilerConfiguration) -> (Compiler) {
		return Compiler {
				configuration : configuration,
			};
	}
	
	
	
	
	pub fn compile (&self, context : Option<&Context>, token : &Value) -> (Outcome<Expression>) {
		let token = token.clone ();
		let compilation = CompilerContext::new_with_context (context);
		let (_compilation, expression) = r#try! (self.compile_0 (compilation, token));
		succeed! (expression);
	}
	
	pub fn compile_slice (&self, context : Option<&Context>, tokens : &[Value]) -> (Outcome<ExpressionVec>) {
		let tokens = vec_clone_slice (tokens);
		let compilation = CompilerContext::new_with_context (context);
		let (_compilation, expressions) = r#try! (self.compile_0_vec (compilation, tokens));
		succeed! (expressions);
	}
	
	
	
	
	fn compile_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
		#[ cfg ( feature = "vonuvoli_transcript" ) ]
		{ if self.configuration.is_trace_enabled () {
			
			let token_input = token.clone ();
			
			if self.configuration.should_trace_input () {
				trace_debugging! (transcript, 0x1d44f3ad => "compiling:\u{1e}{}" => (&token_input));
			}
			
			let outcome = self.compile_00 (compilation, token);
			
			match outcome {
				Ok ((_, ref expression)) if self.configuration.should_trace_output () =>
					trace_debugging! (transcript, 0x1307865e => "compiling succeeded:\u{1e}{}\u{1e}{:#?}" => (&token_input, expression)),
				Ok (_) =>
					(),
				Err (ref error) if self.configuration.should_trace_output_or_error () && error.is_traceable () && !error.was_reported () =>
					trace_error! (transcript, 0xb1511d7c => "compiling failed:\u{1e}{}" => (&token_input), error = error),
				Err (_) =>
					(),
			}
			
			return outcome;
			
		} else {
			
			return self.compile_00 (compilation, token);
		} }
		
		#[ cfg ( not ( all ( feature = "vonuvoli_compiler_trace_enabled", feature = "vonuvoli_transcript" ) ) ) ]
		return self.compile_00 (compilation, token);
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::Character (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Symbol (value) =>
				return self.compile_symbol (compilation, value),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueClassMatchInto::Keyword (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueClassMatchInto::Unique (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::String (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::Bytes (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::StringRegex (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::BytesRegex (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Pair (class) =>
				match class {
					PairMatchInto::Immutable (value) =>
						return self.compile_form (compilation, value),
					#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
					PairMatchInto::Mutable (value) =>
						return self.compile_form (compilation, r#try! (value.into_immutable ())),
				},
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueClassMatchInto::Array (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueClassMatchInto::Values (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::RecordKind (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::Record (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueClassMatchInto::Error (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Procedure (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Syntax (_class) =>
				fail! (0xc617f3c7),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueClassMatchInto::Path (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueClassMatchInto::Port (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
			ValueClassMatchInto::Resource (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			ValueClassMatchInto::Internal (class) =>
				return self.compile_syntax_quote_0 (compilation, class.value ()),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueClassMatchInto::Opaque (value) =>
				return self.compile_syntax_quote_0 (compilation, value.into ()),
			
		}
	}
	
	
	
	
	fn compile_0_vec (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, ExpressionVec)>) {
		let mut expressions = ExpressionVec::with_capacity (tokens.len ());
		let mut compilation = compilation;
		for token in tokens {
			let (compilation_1, expression) = r#try! (self.compile_0 (compilation, token));
			compilation = compilation_1;
			expressions.push (expression);
		}
		succeed! ((compilation, expressions));
	}
	
	
	
	
	fn compile_symbol (&self, compilation : CompilerContext, identifier : Symbol) -> (Outcome<(CompilerContext, Expression)>) {
		let (compilation, binding) = r#try! (compilation.resolve (identifier));
		let expression = r#try! (self.compile_syntax_binding_get (binding));
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_form (&self, compilation : CompilerContext, form : PairImmutable) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (callable, arguments) = form.left_and_right_into ();
		
		match r#try! (self.compile_form_match_callable (compilation, callable)) {
			
			(compilation, Alternative2::Variant1 (syntax)) =>
				return self.compile_syntax_call (compilation, syntax, arguments),
			
			(compilation, Alternative2::Variant2 (callable)) =>
				return self.compile_procedure_call (compilation, callable, arguments),
			
		}
	}
	
	
	fn compile_form_match_callable (&self, compilation : CompilerContext, callable : Value) -> (Outcome<(CompilerContext, Alternative2<SyntaxPrimitive, Value>)>) {
		
		match callable.class_match_into () {
			
			ValueClassMatchInto::Symbol (symbol) => {
				let (compilation, callable) = r#try! (compilation.resolve_value (symbol.clone ()));
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
					#[ cfg ( feature = "vonuvoli_values_extended" ) ]
					SyntaxMatchInto::Extended (_syntax) =>
						fail_unimplemented! (0xb9915ee0, (github_issue, 45)),
					#[ cfg ( feature = "vonuvoli_values_native" ) ]
					SyntaxMatchInto::Native (_syntax) =>
						fail_unimplemented! (0x09bdd94b, (github_issue, 45)),
					#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
					SyntaxMatchInto::Lambda (_syntax) =>
						fail_unimplemented! (0xd3517f42, (github_issue, 46)),
				},
			
			class =>
				succeed! ((compilation, Alternative2::Variant2 (class.value ()))),
			
		}
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::needless_pass_by_value) ) ]
	fn compile_procedure_call (&self, compilation : CompilerContext, procedure : Value, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		let arguments = r#try! (vec_list_clone (&arguments));
		return self.compile_procedure_call_0 (compilation, procedure, arguments);
	}
	
	
	fn compile_procedure_call_0 (&self, compilation : CompilerContext, procedure : Value, arguments : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, procedure) = r#try! (self.compile_0 (compilation, procedure));
		let (compilation, arguments) = r#try! (self.compile_0_vec (compilation, arguments));
		
		let expression = ExpressionForProcedureGenericCall::ProcedureCall (procedure.into (), arguments.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::needless_pass_by_value) ) ]
	fn compile_syntax_call (&self, compilation : CompilerContext, syntax : SyntaxPrimitive, tokens : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens = r#try! (vec_list_clone (&tokens));
		
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
					
					#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
					SyntaxPrimitiveV::Case =>
						return self.compile_syntax_case (compilation, tokens),
					
					SyntaxPrimitiveV::Do =>
						return self.compile_syntax_do (compilation, tokens, false),
					
					SyntaxPrimitiveV::DoCond =>
						return self.compile_syntax_do (compilation, tokens, true),
					
					SyntaxPrimitiveV::While =>
						return self.compile_syntax_while (compilation, tokens, false, false),
					
					SyntaxPrimitiveV::Until =>
						return self.compile_syntax_while (compilation, tokens, true, false),
					
					SyntaxPrimitiveV::WhileCond =>
						return self.compile_syntax_while (compilation, tokens, false, true),
					
					SyntaxPrimitiveV::UntilCond =>
						return self.compile_syntax_while (compilation, tokens, true, true),
					
					SyntaxPrimitiveV::Loop =>
						return self.compile_syntax_loop (compilation, tokens),
					
					#[ cfg ( feature = "vonuvoli_values_error" ) ]
					SyntaxPrimitiveV::Guard =>
						return self.compile_syntax_guard (compilation, tokens),
					
					#[ cfg ( feature = "vonuvoli_values_error" ) ]
					SyntaxPrimitiveV::GuardCond =>
						return self.compile_syntax_guard_cond (compilation, tokens),
					
					SyntaxPrimitiveV::Locals =>
						return self.compile_syntax_locals (compilation, tokens),
					
					SyntaxPrimitiveV::LetParallel |
					SyntaxPrimitiveV::LetSequential |
					SyntaxPrimitiveV::LetRecursiveParallel |
					SyntaxPrimitiveV::LetRecursiveSequential =>
						return self.compile_syntax_let (compilation, syntax, tokens),
					
					#[ cfg ( feature = "vonuvoli_values_values" ) ]
					SyntaxPrimitiveV::LetValuesParallel |
					SyntaxPrimitiveV::LetValuesSequential =>
						return self.compile_syntax_let_values (compilation, syntax, tokens),
					
					#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
					SyntaxPrimitiveV::LetParameters =>
						return self.compile_syntax_let_parameters (compilation, tokens),
					
					SyntaxPrimitiveV::Define =>
						return self.compile_syntax_define (compilation, tokens, false),
					
					SyntaxPrimitiveV::ReDefine =>
						return self.compile_syntax_define (compilation, tokens, true),
					
					#[ cfg ( feature = "vonuvoli_values_values" ) ]
					SyntaxPrimitiveV::DefineValues =>
						return self.compile_syntax_define_values (compilation, tokens, false),
					
					#[ cfg ( feature = "vonuvoli_values_values" ) ]
					SyntaxPrimitiveV::ReDefineValues =>
						return self.compile_syntax_define_values (compilation, tokens, true),
					
					SyntaxPrimitiveV::Set =>
						return self.compile_syntax_set (compilation, tokens),
					
					#[ cfg ( feature = "vonuvoli_values_values" ) ]
					SyntaxPrimitiveV::SetValues =>
						return self.compile_syntax_set_values (compilation, tokens),
					
					#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
					SyntaxPrimitiveV::Lambda =>
						return self.compile_syntax_lambda (compilation, None, tokens),
					
					#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
					SyntaxPrimitiveV::DefineRecord =>
						return self.compile_syntax_define_record (compilation, tokens),
					
				},
			
			SyntaxPrimitive::Auxiliary =>
				fail! (0xc64adbb8),
			
			SyntaxPrimitive::Unimplemented =>
				fail_unimplemented! (0xa4e41f62, github_issue_new),
			
			SyntaxPrimitive::Unsupported =>
				fail_unimplemented! (0x175a7f9e, github_issue_new),
			
			SyntaxPrimitive::Reserved =>
				fail! (0x1aed14f3),
			
		}
	}
	
	
	
	
	fn compile_syntax_sequence (&self, compilation : CompilerContext, operator : ExpressionSequenceOperator, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = match operator {
			ExpressionSequenceOperator::ReturnFirst | ExpressionSequenceOperator::ReturnLast =>
				r#try! (self.compile_0_vec (compilation, tokens)),
			ExpressionSequenceOperator::And | ExpressionSequenceOperator::Or => {
				let compilation = r#try! (compilation.define_disable ());
				let (compilation, statements) = r#try! (self.compile_0_vec (compilation, tokens));
				let compilation = r#try! (compilation.define_enable ());
				(compilation, statements)
			},
		};
		
		let expression = Expression::Sequence (operator, statements.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_if (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if (tokens_count != 2) && (tokens_count != 3) {
			fail! (0xe34389a7);
		}
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, tokens));
		let compilation = r#try! (compilation.define_enable ());
		
		let clauses = if tokens_count == 3 {
			let (guard, if_true, if_false) = r#try! (vec_explode_3 (statements));
			vec! [
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::Expression (guard, false),
							ExpressionValueConsumer::Ignore,
							if_true),
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::Expression (TRUE_VALUE.into (), false),
							ExpressionValueConsumer::Ignore,
							if_false),
				]
		} else if tokens_count == 2 {
			let (guard, if_true) = r#try! (vec_explode_2 (statements));
			vec! [
					ExpressionConditionalIfClause::GuardAndExpression (
							ExpressionConditionalIfGuard::Expression (guard, false),
							ExpressionValueConsumer::Ignore,
							if_true),
				]
		} else {
			fail_unreachable! (0xbc801c5d, github_issue_new);
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
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, tokens));
		let compilation = r#try! (compilation.define_enable ());
		
		let (guard, statements) = r#try! (vec_explode_1n (statements));
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		
		let negated = match syntax {
			SyntaxPrimitiveV::When =>
				false,
			SyntaxPrimitiveV::Unless =>
				true,
			_ =>
				fail_unreachable! (0x500d298f, github_issue_new),
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
		
		let (compilation, clauses) = r#try! (self.compile_syntax_cond_clauses (compilation, tokens, false));
		
		let clauses = ExpressionConditionalIfClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalIf (clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	fn compile_syntax_cond_clauses (&self, compilation : CompilerContext, tokens : ValueVec, negated : bool) -> (Outcome<(CompilerContext, StdVec<ExpressionConditionalIfClause>)>) {
		
		let mut compilation = r#try! (compilation.define_disable ());
		let mut clauses = StdVec::new ();
		
		for tokens in tokens {
			
			let tokens = r#try! (vec_list_clone (&tokens));
			if tokens.is_empty () {
				fail! (0x86331f4b);
			}
			let (guard, statements) = r#try! (vec_explode_1n (tokens));
			
			let (compilation_1, guard) = if ! is_symbol_eq ("else", &guard) {
				let (compilation_1, guard) = r#try! (self.compile_0 (compilation, guard));
				let guard = ExpressionConditionalIfGuard::Expression (guard, negated);
				(compilation_1, guard)
			} else {
				let guard = ExpressionConditionalIfGuard::Expression (TRUE_VALUE.into (), false);
				(compilation, guard)
			};
			
			let (compilation_1, guard_consumer) = if ! statements.is_empty () && is_symbol_eq ("=>", &statements[0]) {
				fail_unimplemented! (0xfa332991, (github_issue, 24));
			} else {
				(compilation_1, ExpressionValueConsumer::Ignore)
			};
			
			let (compilation_1, statements) = r#try! (self.compile_0_vec (compilation_1, statements));
			
			let clause = if ! statements.is_empty () {
				let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
				ExpressionConditionalIfClause::GuardAndExpression (guard, guard_consumer, statements)
			} else {
				ExpressionConditionalIfClause::GuardOnly (guard, ExpressionValueConsumer::Return)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let compilation = r#try! (compilation.define_enable ());
		
		succeed! ((compilation, clauses));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	fn compile_syntax_case (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.is_empty () {
			fail! (0xeb8569a2);
		}
		
		let (actual, tokens) = r#try! (vec_explode_1n (tokens));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, actual) = r#try! (self.compile_0 (compilation, actual));
		let compilation = r#try! (compilation.define_enable ());
		
		let (compilation, clauses) = r#try! (self.compile_syntax_case_clauses (compilation, tokens, false));
		
		let clauses = ExpressionConditionalMatchClauses::Multiple (clauses.into_boxed_slice ());
		
		let expression = Expression::ConditionalMatch (actual.into (), clauses);
		
		succeed! ((compilation, expression));
	}
	
	
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	fn compile_syntax_case_clauses (&self, compilation : CompilerContext, tokens : ValueVec, negated : bool) -> (Outcome<(CompilerContext, StdVec<ExpressionConditionalMatchClause>)>) {
		
		let mut compilation = r#try! (compilation.define_disable ());
		let mut clauses = StdVec::new ();
		
		for tokens in tokens {
			
			let tokens = r#try! (vec_list_clone (&tokens));
			if tokens.is_empty () {
				fail! (0x17388f6a);
			}
			let (expected, statements) = r#try! (vec_explode_1n (tokens));
			
			let guard = if ! is_symbol_eq ("else", &expected) {
				let expected = r#try! (vec_list_clone (&expected));
				ExpressionConditionalMatchGuard::Values (expected.into_boxed_slice (), negated)
			} else {
				ExpressionConditionalMatchGuard::True
			};
			
			let (compilation_1, guard_consumer) = if ! statements.is_empty () && is_symbol_eq ("=>", &statements[0]) {
				fail_unimplemented! (0xef5d468c, (github_issue, 24));
			} else {
				(compilation, ExpressionValueConsumer::Ignore)
			};
			
			let (compilation_1, statements) = r#try! (self.compile_0_vec (compilation_1, statements));
			
			let clause = if ! statements.is_empty () {
				let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
				ExpressionConditionalMatchClause::GuardAndExpression (guard, guard_consumer, statements)
			} else {
				ExpressionConditionalMatchClause::GuardOnly (guard, ExpressionValueConsumer::Return)
			};
			
			clauses.push (clause);
			compilation = compilation_1;
		}
		
		let compilation = r#try! (compilation.define_enable ());
		
		succeed! ((compilation, clauses));
	}
	
	
	
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::cyclomatic_complexity) ) ]
	fn compile_syntax_do (&self, compilation : CompilerContext, tokens : ValueVec, break_uses_cond : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (definitions, break_statements, loop_statements) = r#try! (vec_explode_2n (tokens));
		
		let definitions = r#try! (vec_list_clone (&definitions));
		
		let mut identifiers = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		let mut updaters = StdVec::with_capacity (definitions.len ());
		for definition in definitions {
			let definition = r#try! (vec_list_clone (&definition));
			let (identifier, initializer, updater) = match definition.len () {
				2 => {
					let (identifier, initializer) = r#try! (vec_explode_2 (definition));
					(identifier, initializer, None)
				},
				3 => {
					let (identifier, initializer, updater) = r#try! (vec_explode_3 (definition));
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
			let compilation = r#try! (compilation.fork_locals (true));
			(compilation, true)
		};
		
		let (compilation, binding_initializers, binding_updaters) = if has_definitions {
			
			let mut binding_templates = StdVec::new ();
			let mut binding_initializers = StdVec::new ();
			let mut binding_updaters = StdVec::new ();
			let mut compilation = compilation;
			
			for initializer in initializers {
				let compilation_1 = r#try! (compilation.define_disable ());
				let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
				compilation = r#try! (compilation_1.define_enable ());
				binding_initializers.push (initializer);
			}
			
			for identifier in identifiers {
				let (compilation_1, binding) = r#try! (compilation.define (identifier));
				compilation = compilation_1;
				binding_templates.push (binding);
			}
			
			for updater in updaters {
				let compilation_1 = r#try! (compilation.define_disable ());
				let (compilation_1, updater) = r#try! (self.compile_0 (compilation_1, updater));
				compilation = r#try! (compilation_1.define_enable ());
				binding_updaters.push (updater);
			}
			
			let binding_initializers = r#try! (self.compile_syntax_binding_set_n (binding_templates.clone (), binding_initializers, true, true));
			let binding_updaters = r#try! (self.compile_syntax_binding_set_n (binding_templates.clone (), binding_updaters, true, false));
			
			(compilation, Some (binding_initializers.into ()), Some (binding_updaters.into ()))
			
		} else {
			
			(compilation, None, None)
		};
		
		
		let break_statements = r#try! (vec_list_clone (&break_statements));
		let (compilation, break_clauses) = if break_uses_cond {
			
			let (compilation, break_clauses) = r#try! (self.compile_syntax_cond_clauses (compilation, break_statements, false));
			let break_clauses = ExpressionConditionalIfClauses::Multiple (break_clauses.into_boxed_slice ());
			
			(compilation, Some (break_clauses))
			
		} else  if ! break_statements.is_empty () {
			
			let (break_guard, break_statements) = r#try! (vec_explode_1n (break_statements));
			
			let compilation = r#try! (compilation.define_disable ());
			let (compilation, break_guard) = r#try! (self.compile_0 (compilation, break_guard));
			let compilation = r#try! (compilation.define_enable ());
			let break_guard = ExpressionConditionalIfGuard::Expression (break_guard, false);
			
			let (compilation, break_clause) = if break_statements.is_empty () {
				let clause = ExpressionConditionalIfClause::GuardOnly (break_guard, ExpressionValueConsumer::Return);
				(compilation, clause)
			} else {
				FIXME! ("add support for `(guard => expression)` just like for `cond`");
				let compilation = r#try! (compilation.define_disable ());
				let (compilation, break_statements) = r#try! (self.compile_0_vec (compilation, break_statements));
				let compilation = r#try! (compilation.define_enable ());
				let break_statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, break_statements.into_boxed_slice ());
				let clause = ExpressionConditionalIfClause::GuardAndExpression (break_guard, ExpressionValueConsumer::Ignore, break_statements);
				(compilation, clause)
			};
			
			let break_clauses = ExpressionConditionalIfClauses::Multiple (StdBox::new ([break_clause]));
			
			(compilation, Some (break_clauses))
			
		} else {
			
			(compilation, None)
		};
		
		let (compilation, loop_statement) = r#try! (self.compile_syntax_loop_statements (compilation, loop_statements));
		let loop_statement = option_box_new (loop_statement);
		
		let expression = Expression::Loop (binding_initializers, binding_updaters, loop_statement, break_clauses);
		
		let (compilation, expression) = if has_definitions {
			let (compilation, registers) = r#try! (compilation.unfork_locals ());
			let expression = ExpressionForContexts::RegisterClosure (expression.into (), registers.into_boxed_slice ()) .into ();
			(compilation, expression)
		} else {
			(compilation, expression)
		};
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_while (&self, compilation : CompilerContext, tokens : ValueVec, negated : bool, break_uses_cond : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (break_guard, loop_statements) = r#try! (vec_explode_1n (tokens));
		
		let (compilation, break_clauses) = if break_uses_cond {
			
			let break_clauses = r#try! (vec_list_clone (&break_guard));
			let (compilation, break_clauses) = r#try! (self.compile_syntax_cond_clauses (compilation, break_clauses, ! negated));
			
			let break_clauses = ExpressionConditionalIfClauses::Multiple (break_clauses.into_boxed_slice ());
			
			(compilation, break_clauses)
			
		} else {
			
			let compilation = r#try! (compilation.define_disable ());
			let (compilation, break_guard) = r#try! (self.compile_0 (compilation, break_guard));
			let compilation = r#try! (compilation.define_enable ());
			let break_guard = ExpressionConditionalIfGuard::Expression (break_guard, ! negated);
			
			let break_clause = ExpressionConditionalIfClause::GuardOnly (break_guard, ExpressionValueConsumer::Return);
			let break_clauses = ExpressionConditionalIfClauses::Multiple (StdBox::new ([break_clause]));
			
			(compilation, break_clauses)
			
		};
		
		let (compilation, loop_statement) = r#try! (self.compile_syntax_loop_statements (compilation, loop_statements));
		let loop_statement = option_box_new (loop_statement);
		
		let expression = Expression::Loop (None, None, loop_statement, Some (break_clauses));
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_loop (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, loop_statement) = r#try! (self.compile_syntax_loop_statements (compilation, tokens));
		let loop_statement = option_box_new (loop_statement);
		
		let expression = Expression::Loop (None, None, loop_statement, None);
		
		succeed! ((compilation, expression));
	}
	
	
	fn compile_syntax_loop_statements (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Option<Expression>)>) {
		
		let (compilation, loop_statement) = if tokens.is_empty () {
			(compilation, None)
		} else {
			let compilation = r#try! (compilation.define_disable ());
			let (compilation, loop_statements) = r#try! (self.compile_0_vec (compilation, tokens));
			let compilation = r#try! (compilation.define_enable ());
			let loop_statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, loop_statements.into_boxed_slice ());
			(compilation, Some (loop_statements))
		};
		
		succeed! ((compilation, loop_statement));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	fn compile_syntax_guard (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		match tokens.len () {
			0 =>
				fail! (0x4065b3e7),
			1 => {
				let token = r#try! (vec_explode_1 (tokens));
				return self.compile_syntax_guard_return (compilation, token);
			},
			3 =>
				(),
			_ =>
				fail! (0xa6056cfd),
		}
		
		let (error_identifier, statement, error_statement) = r#try! (vec_explode_3 (tokens));
		let error_identifier = try_into_symbol! (error_identifier);
		
		let compilation = r#try! (compilation.fork_locals (true));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, statement) = r#try! (self.compile_0 (compilation, statement));
		let compilation = r#try! (compilation.define_enable ());
		
		let (compilation, error_binding) = r#try! (compilation.define (error_identifier));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, error_statement) = r#try! (self.compile_0 (compilation, error_statement));
		let compilation = r#try! (compilation.define_enable ());
		
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		
		let error_consumer = r#try! (self.compile_syntax_binding_consumer (error_binding));
		
		let expression = Expression::ErrorCatch (statement.into (), error_consumer, error_statement.into ());
		let expression = ExpressionForContexts::RegisterClosure (expression.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	fn compile_syntax_guard_cond (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		match tokens.len () {
			0 =>
				fail! (0xa10959b0),
			1 => {
				let token = r#try! (vec_explode_1 (tokens));
				return self.compile_syntax_guard_return (compilation, token);
			},
			_ =>
				(),
		}
		
		let (error_handler, statements) = r#try! (vec_explode_1n (tokens));
		let error_handler = r#try! (vec_list_clone (&error_handler));
		let (error_identifier, error_clauses) = r#try! (vec_explode_1n (error_handler));
		let error_identifier = try_into_symbol! (error_identifier);
		
		let compilation = r#try! (compilation.fork_locals (true));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		let compilation = r#try! (compilation.define_enable ());
		
		let (compilation, error_binding) = r#try! (compilation.define (error_identifier.clone ()));
		let (compilation, error_reference) = r#try! (self.compile_symbol (compilation, error_identifier));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, error_clauses) = r#try! (self.compile_syntax_cond_clauses (compilation, error_clauses, false));
		let compilation = r#try! (compilation.define_enable ());
		
		let mut error_clauses = error_clauses;
		error_clauses.push (ExpressionConditionalIfClause::GuardAndExpression (
				ExpressionConditionalIfGuard::Expression (TRUE_VALUE.into (), false),
				ExpressionValueConsumer::Ignore,
				Expression::ErrorThrow (error_reference.into ())));
		
		let error_clauses = ExpressionConditionalIfClauses::Multiple (error_clauses.into_boxed_slice ());
		let error_statement = Expression::ConditionalIf (error_clauses);
		
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		
		let error_consumer = r#try! (self.compile_syntax_binding_consumer (error_binding));
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = Expression::ErrorCatch (statements.into (), error_consumer, error_statement.into ());
		let expression = ExpressionForContexts::RegisterClosure (expression.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	fn compile_syntax_guard_return (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, statement) = r#try! (self.compile_0 (compilation, token));
		let compilation = r#try! (compilation.define_enable ());
		
		let expression = Expression::ErrorReturn (statement.into ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_locals (&self, compilation : CompilerContext, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = r#try! (compilation.fork_locals (true));
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_let (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x633b3ed8);
		}
		let (definitions, statements) = r#try! (vec_explode_1n (tokens));
		
		let definitions = match definitions.kind_match_into () {
			ValueKindMatchInto::Null =>
				return self.compile_syntax_locals (compilation, statements),
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueKindMatchInto::Symbol (symbol) => {
				let (definitions, statements) = r#try! (vec_explode_1n (statements));
				return self.compile_syntax_lambda_let (compilation, symbol, definitions, statements);
			},
			#[ cfg ( not ( feature = "vonuvoli_values_lambda" ) ) ]
			ValueKindMatchInto::Symbol (_) =>
				fail! (0xf38dbfa0),
			ValueKindMatchInto::PairImmutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::PairMutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			_ =>
				fail! (0x825cb457),
		};
		
		let mut identifiers = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions {
			let definition = r#try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x190d57f8);
			}
			let (identifier, initializer) = r#try! (vec_explode_2 (definition));
			let identifier = try_into_symbol! (identifier);
			identifiers.push (identifier);
			initializers.push (initializer);
		}
		
		let mut compilation = r#try! (compilation.fork_locals (true));
		let mut binding_templates = StdVec::new ();
		let mut binding_initializers = StdVec::new ();
		
		match syntax {
			
			SyntaxPrimitiveV::LetParallel => {
				for initializer in initializers {
					let compilation_1 = r#try! (compilation.define_disable ());
					let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
					compilation = r#try! (compilation_1.define_enable ());
					binding_initializers.push (initializer);
				}
				for identifier in identifiers {
					let (compilation_1, binding) = r#try! (compilation.define (identifier));
					compilation = compilation_1;
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetSequential => {
				for (initializer, identifier) in initializers.into_iter ().zip (identifiers.into_iter ()) {
					let compilation_1 = r#try! (compilation.define_disable ());
					let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
					let compilation_1 = r#try! (compilation_1.define_enable ());
					let (compilation_1, binding) = r#try! (compilation_1.define (identifier));
					compilation = compilation_1;
					binding_initializers.push (initializer);
					binding_templates.push (binding);
				}
			},
			
			SyntaxPrimitiveV::LetRecursiveParallel | SyntaxPrimitiveV::LetRecursiveSequential => {
				for identifier in identifiers {
					let (compilation_1, binding) = r#try! (compilation.define (identifier));
					compilation = compilation_1;
					binding_templates.push (binding);
				}
				for initializer in initializers {
					let compilation_1 = r#try! (compilation.define_disable ());
					let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
					compilation = r#try! (compilation_1.define_enable ());
					binding_initializers.push (initializer);
				}
			},
			
			_ =>
				fail_unreachable! (0xa1c3e4ac, github_issue_new),
			
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
				fail_unreachable! (0xa615e40c, github_issue_new),
		};
		
		let binding_initializers = r#try! (self.compile_syntax_binding_set_n (binding_templates, binding_initializers, parallel, true));
		let binding_initializers = vec! [ binding_initializers ];
		
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	fn compile_syntax_let_values (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveV, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x10672a0d);
		}
		let (definitions, statements) = r#try! (vec_explode_1n (tokens));
		
		let definitions = match definitions.kind_match_into () {
			ValueKindMatchInto::Null =>
				return self.compile_syntax_locals (compilation, statements),
			ValueKindMatchInto::PairImmutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::PairMutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			_ =>
				fail! (0x60cfd87a),
		};
		
		let mut identifiers_n = StdVec::with_capacity (definitions.len ());
		let mut initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions {
			let definition = r#try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x6cbd574f);
			}
			let (identifiers, initializer) = r#try! (vec_explode_2 (definition));
			let identifiers = r#try! (vec_list_clone (&identifiers));
			let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
			identifiers_n.push (identifiers);
			initializers.push (initializer);
		}
		
		let mut compilation = r#try! (compilation.fork_locals (true));
		let mut binding_templates_n = StdVec::new ();
		let mut binding_initializers = StdVec::new ();
		
		match syntax {
			
			SyntaxPrimitiveV::LetValuesParallel => {
				for initializer in initializers {
					let compilation_1 = r#try! (compilation.define_disable ());
					let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
					compilation = r#try! (compilation_1.define_enable ());
					binding_initializers.push (initializer);
				}
				for identifiers in identifiers_n {
					let mut binding_templates = StdVec::new ();
					for identifier in identifiers {
						let (compilation_1, binding) = r#try! (compilation.define (identifier));
						compilation = compilation_1;
						binding_templates.push (binding);
					}
					binding_templates_n.push (binding_templates);
				}
			},
			
			SyntaxPrimitiveV::LetValuesSequential => {
				for (initializer, identifiers) in initializers.into_iter ().zip (identifiers_n.into_iter ()) {
					let compilation_1 = r#try! (compilation.define_disable ());
					let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
					compilation = r#try! (compilation_1.define_enable ());
					let mut binding_templates = StdVec::new ();
					for identifier in identifiers {
						let (compilation_1, binding) = r#try! (compilation.define (identifier));
						compilation = compilation_1;
						binding_templates.push (binding);
					}
					binding_templates_n.push (binding_templates);
					binding_initializers.push (initializer);
				}
			},
			
			_ =>
				fail_unreachable! (0x7498ded2, github_issue_new),
			
		}
		
		let binding_initializers = r#try! (self.compile_syntax_binding_set_values_n (binding_templates_n, binding_initializers, true));
		
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		
		let statements = vec_append_2 (binding_initializers, statements);
		
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	fn compile_syntax_let_parameters (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0xf101e358);
		}
		let (definitions, statements) = r#try! (vec_explode_1n (tokens));
		
		let definitions = match definitions.kind_match_into () {
			ValueKindMatchInto::Null =>
				StdVec::new (),
			ValueKindMatchInto::PairImmutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			ValueKindMatchInto::PairMutable (pair) =>
				r#try! (vec_list_clone (&pair.into ())),
			_ =>
				fail! (0xb3f3b18a),
		};
		
		let mut compilation = compilation;
		let mut initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions {
			let definition = r#try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0xc077e4a3);
			}
			let (parameter, initializer) = r#try! (vec_explode_2 (definition));
			let (compilation_1, parameter) = r#try! (self.compile_0 (compilation, parameter));
			let (compilation_1, initializer) = r#try! (self.compile_0 (compilation_1, initializer));
			compilation = compilation_1;
			let initializer = ExpressionForProcedureGenericCall::ProcedureCall (RuntimePrimitiveV::ParameterConfigure.into (), StdBox::new ([parameter, initializer]));
			initializers.push (initializer.into ());
		}
		
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		
		let statements = vec_append_2 (initializers, statements);
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::ParameterClosure (statements.into ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_define (&self, compilation : CompilerContext, tokens : ValueVec, redefine : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x4481879c);
		}
		let (signature, statements) = r#try! (vec_explode_1n (tokens));
		
		let (compilation, binding, expression) = match signature.class_match_into () {
			
			ValueClassMatchInto::Symbol (identifier) => {
				
				if statements.len () != 1 {
					fail! (0xc364edf8);
				}
				
				let statement = r#try! (vec_explode_1 (statements));
				
				let (compilation, binding) = if ! redefine {
					let (compilation, binding) = r#try! (compilation.define_or_redefine (identifier.clone (), redefine));
					(compilation, Some (binding))
				} else {
					(compilation, None)
				};
				
				let compilation = r#try! (compilation.define_disable ());
				let (compilation, expression) = r#try! (self.compile_0 (compilation, statement));
				let compilation = r#try! (compilation.define_enable ());
				
				let (compilation, binding) = if redefine {
					r#try! (compilation.define_or_redefine (identifier.clone (), redefine))
				} else {
					let binding = try_some_or_panic! (binding, 0x1dedc516, github_issue_new);
					(compilation, binding)
				};
				
				(compilation, binding, expression)
			},
			
			#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
			ValueClassMatchInto::Pair (signature) => {
				
				if statements.is_empty () {
					fail! (0x48c70de5);
				}
				
				if redefine {
					fail! (0xb09e1636);
				}
				
				let (signature, argument_rest) = r#try! (vec_list_clone_dotted (&signature.value ()));
				let (identifier, arguments_positional) = r#try! (vec_explode_1n (signature));
				
				let identifier = try_into_symbol! (identifier);
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				
				let (compilation, binding) = r#try! (compilation.define_or_redefine (identifier.clone (), redefine));
				let (compilation, expression) = r#try! (self.compile_syntax_lambda_0 (compilation, Some (identifier), arguments_positional, argument_rest, statements));
				
				(compilation, binding, expression)
			},
			
			#[ cfg ( not ( feature = "vonuvoli_values_lambda" ) ) ]
			ValueClassMatchInto::Pair (_) =>
				fail! (0x6d722cc0),
			
			_ =>
				fail! (0x0f0edc26),
		};
		
		let expression = r#try! (self.compile_syntax_binding_set_1 (binding, expression, true));
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	fn compile_syntax_define_values (&self, compilation : CompilerContext, tokens : ValueVec, redefine : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0x5d801f2e);
		}
		
		let (identifiers, initializer) = r#try! (vec_explode_2 (tokens));
		let identifiers = r#try! (vec_list_clone (&identifiers));
		let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
		
		let mut binding_templates = StdVec::new ();
		
		let mut compilation = compilation;
		if ! redefine {
			for identifier in identifiers.clone () {
				let (compilation_1, binding) = r#try! (compilation.define_or_redefine (identifier, redefine));
				compilation = compilation_1;
				binding_templates.push (binding);
			}
		}
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, binding_initializer) = r#try! (self.compile_0 (compilation, initializer));
		let compilation = r#try! (compilation.define_enable ());
		
		let mut compilation = compilation;
		if redefine {
			for identifier in identifiers.clone () {
				let (compilation_1, binding) = r#try! (compilation.define_or_redefine (identifier, redefine));
				compilation = compilation_1;
				binding_templates.push (binding);
			}
		}
		
		let expression = r#try! (self.compile_syntax_binding_set_values_1 (binding_templates, binding_initializer, true));
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_set (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0x2573a064);
		}
		
		let (identifier, initializer) = r#try! (vec_explode_2 (tokens));
		let identifier = try_into_symbol! (identifier);
		
		let (compilation, binding) = r#try! (compilation.resolve (identifier));
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, initializer) = r#try! (self.compile_0 (compilation, initializer));
		let compilation = r#try! (compilation.define_enable ());
		
		let initializer = r#try! (self.compile_syntax_binding_set_1 (binding, initializer, false));
		
		succeed! ((compilation, initializer));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	fn compile_syntax_set_values (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () != 2 {
			fail! (0xecf87cfa);
		}
		
		let (identifiers, initializer) = r#try! (vec_explode_2 (tokens));
		let identifiers = r#try! (vec_list_clone (&identifiers));
		let identifiers = try_vec_map_into! (identifiers, identifier, Symbol::try_from (identifier));
		
		let mut compilation = compilation;
		let mut bindings = StdVec::new ();
		for identifier in identifiers {
			let (compilation_1, binding) = r#try! (compilation.resolve (identifier));
			compilation = compilation_1;
			bindings.push (binding);
		}
		
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, initializer) = r#try! (self.compile_0 (compilation, initializer));
		let compilation = r#try! (compilation.define_enable ());
		
		let initializer = r#try! (self.compile_syntax_binding_set_values_1 (bindings, initializer, false));
		
		succeed! ((compilation, initializer));
	}
	
	
	
	
	fn compile_syntax_binding_get (&self, binding : CompilerBinding) -> (Outcome<Expression>) {
		match binding {
			CompilerBinding::Binding (_, binding, _) =>
				succeed! (ExpressionForContexts::BindingGet1 (binding) .into ()),
			CompilerBinding::Register (_, index, _) =>
				succeed! (ExpressionForContexts::RegisterGet1 (index) .into ()),
			CompilerBinding::Undefined =>
				fail! (0xc6825cfd),
		}
	}
	
	
	fn compile_syntax_binding_consumer (&self, binding : CompilerBinding) -> (Outcome<ExpressionValueConsumer>) {
		match binding {
			CompilerBinding::Binding (_, binding, _) =>
				succeed! (ExpressionValueConsumer::BindingInitialize (binding)),
			CompilerBinding::Register (_, index, _) =>
				succeed! (ExpressionValueConsumer::RegisterInitialize (index)),
			CompilerBinding::Undefined =>
				fail_panic! (0xce4f018f, github_issue_new),
		}
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
		
		if bindings.is_empty () {
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
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	fn compile_syntax_binding_set_values_1 (&self, bindings : StdVec<CompilerBinding>, expression : Expression, initialize : bool) -> (Outcome<Expression>) {
		
		if bindings.is_empty () {
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
	
	
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	fn compile_syntax_binding_set_values_n (&self, bindings : StdVec<StdVec<CompilerBinding>>, expressions : StdVec<Expression>, initialize : bool) -> (Outcome<StdVec<Expression>>) {
		
		if bindings.is_empty () {
			fail! (0x28e6a67b);
		}
		if bindings.len () != expressions.len () {
			fail! (0x3ab734fa);
		}
		
		let mut initializers = StdVec::with_capacity (bindings.len ());
		
		for (bindings, expression) in vec_zip_2 (bindings, expressions) {
			let initializer = r#try! (self.compile_syntax_binding_set_values_1 (bindings, expression, initialize));
			initializers.push (initializer);
		}
		
		succeed! (initializers);
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	fn compile_syntax_lambda (&self, compilation : CompilerContext, identifier : Option<Symbol>, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x2dfd91d1);
		}
		
		let (arguments, statements) = r#try! (vec_explode_1n (tokens));
		let (arguments_positional, argument_rest) = match arguments.class_match_into () {
			ValueClassMatchInto::Null =>
				(StdVec::new (), None),
			ValueClassMatchInto::Symbol (argument) =>
				(StdVec::new (), Some (argument)),
			ValueClassMatchInto::Pair (arguments) => {
				let (arguments_positional, argument_rest) = r#try! (vec_list_clone_dotted (&arguments.value ()));
				let arguments_positional = try_vec_map_into! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				(arguments_positional, argument_rest)
			},
			_ =>
				fail! (0x70773cab),
		};
		
		return self.compile_syntax_lambda_0 (compilation, identifier, arguments_positional, argument_rest, statements);
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::needless_pass_by_value) ) ]
	fn compile_syntax_lambda_let (&self, compilation : CompilerContext, identifier : Symbol, definitions : Value, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let definitions = r#try! (vec_list_clone (&definitions));
		
		let compilation = r#try! (compilation.fork_locals (true));
		
		let mut compilation = r#try! (compilation.define_disable ());
		let mut argument_identifiers = StdVec::with_capacity (definitions.len ());
		let mut argument_initializers = StdVec::with_capacity (definitions.len ());
		for definition in definitions {
			let definition = r#try! (vec_list_clone (&definition));
			if definition.len () != 2 {
				fail! (0x4ad3c4b8);
			}
			let (identifier, initializer) = r#try! (vec_explode_2 (definition));
			let identifier = try_into_symbol! (identifier);
			let (compilation_1, initializer) = r#try! (self.compile_0 (compilation, initializer));
			argument_identifiers.push (identifier);
			argument_initializers.push (initializer);
			compilation = compilation_1;
		}
		let compilation = r#try! (compilation.define_enable ());
		
		let (compilation, lambda_binding) = r#try! (compilation.define (identifier.clone ()));
		let (compilation, lambda_value) = r#try! (self.compile_syntax_lambda_0 (compilation, Some (identifier.clone ()), argument_identifiers, None, statements));
		let (compilation, lambda_reference) = r#try! (self.compile_symbol (compilation, identifier.clone ()));
		
		let lambda_initializer = r#try! (self.compile_syntax_binding_set_1 (lambda_binding, lambda_value, true));
		let lambda_call = ExpressionForProcedureGenericCall::ProcedureCall (lambda_reference.into (), argument_initializers.into_boxed_slice ()) .into ();
		let statements = vec! [ lambda_initializer, lambda_call ];
		
		let (compilation, registers) = r#try! (compilation.unfork_locals ());
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		let expression = ExpressionForContexts::RegisterClosure (statements.into (), registers.into_boxed_slice ()) .into ();
		
		succeed! ((compilation, expression));
	}
	
	
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	fn compile_syntax_lambda_0 (&self, compilation : CompilerContext, identifier : Option<Symbol>, arguments_positional : StdVec<Symbol>, argument_rest : Option<Symbol>, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = r#try! (compilation.fork_locals_with_bindings ());
		
		let mut compilation = r#try! (compilation.fork_locals (true));
		for identifier in &arguments_positional {
			let (compilation_1, _) = r#try! (compilation.define (identifier.clone ()));
			compilation = compilation_1;
		}
		if let Some (ref identifier) = argument_rest {
			let (compilation_1, _) = r#try! (compilation.define (identifier.clone ()));
			compilation = compilation_1;
		}
		
		let (compilation, statements) = r#try! (self.compile_0_vec (compilation, statements));
		
		let (compilation, registers_local) = r#try! (compilation.unfork_locals ());
		let (compilation, registers_closure) = r#try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		
		let template = LambdaTemplate::new (identifier, arguments_positional.into_boxed_slice (), argument_rest);
		
		let template = StdRc::new (template);
		let statements = StdRc::new (statements);
		let registers_closure = StdBox::from (registers_closure);
		let registers_local = StdRc::from (registers_local);
		let expression = Expression::Lambda (template, statements, registers_closure, registers_local);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::cyclomatic_complexity) ) ]
	fn compile_syntax_define_record (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 3 {
			fail! (0x69953d45);
		}
		
		let (type_identifier, constructor, predicate, fields) = r#try! (vec_explode_3n (tokens));
		
		let type_identifier = match type_identifier.class_match_into () {
			ValueClassMatchInto::Symbol (type_identifier) =>
				Some (type_identifier),
			ValueClassMatchInto::Boolean (type_identifier) =>
				if ! type_identifier.value () {
					None
				} else {
					fail! (0xa0795b37);
				},
			_ =>
				fail! (0x6c3b98ed),
		};
		
		let (constructor_identifier, constructor_fields) = match constructor.class_match_into () {
			ValueClassMatchInto::Symbol (constructor_identifier) =>
				(Some (constructor_identifier.into ()), None),
			ValueClassMatchInto::Boolean (constructor_identifier) =>
				if ! constructor_identifier.value () {
					(None, None)
				} else {
					(Some (constructor_identifier.into ()), None)
				},
			ValueClassMatchInto::Pair (tokens) => {
				let tokens = r#try! (vec_list_clone (& tokens.value ()));
				let (constructor_identifier, constructor_fields) = r#try! (vec_explode_1n (tokens));
				(Some (constructor_identifier), Some (constructor_fields))
			},
			_ =>
				fail! (0xc5304162),
		};
		
		let constructor_identifier = if let Some (constructor_identifier) = constructor_identifier {
			match constructor_identifier.class_match_into () {
				ValueClassMatchInto::Symbol (constructor_identifier) =>
					Some (constructor_identifier),
				ValueClassMatchInto::Boolean (constructor_identifier) =>
					if ! constructor_identifier.value () {
						None
					} else if let Some (ref type_identifier) = type_identifier {
						Some (symbol_new (string_concatenate! ("make-", type_identifier.string_as_str ())))
					} else {
						fail! (0xb9fa57b0);
					},
				_ =>
					fail! (0xa29b8e8a),
			}
		} else {
			None
		};
		
		let predicate_identifier = match predicate.class_match_into () {
			ValueClassMatchInto::Symbol (predicate_identifier) =>
				Some (predicate_identifier),
			ValueClassMatchInto::Boolean (predicate_identifier) =>
				if ! predicate_identifier.value () {
					None
				} else if let Some (ref type_identifier) = type_identifier {
					Some (symbol_new (string_concatenate! (type_identifier.string_as_str (), "?")))
				} else {
					fail! (0x2420b024);
				},
			_ =>
				fail! (0xecfa6ca2),
		};
		
		let fields = try_vec_map_into! (fields, field,
				{
					let (field_identifier, field_accessor_identifier, field_mutator_identifier) = match field.class_match_into () {
						ValueClassMatchInto::Symbol (field_identifier) =>
							(Some (field_identifier.into ()), Some (TRUE_VALUE), Some (TRUE_VALUE)),
						ValueClassMatchInto::Boolean (field_identifier) =>
							if ! field_identifier.value () {
								(None, None, None)
							} else {
								fail! (0x06d78cb8);
							},
						ValueClassMatchInto::Pair (tokens) => {
							let tokens = r#try! (vec_list_clone (& tokens.value ()));
							match tokens.len () {
								1 => {
									let field_identifier = r#try! (vec_explode_1 (tokens));
									(Some (field_identifier), None, None)
								},
								2 => {
									let (field_identifier, field_accessor_identifier) = r#try! (vec_explode_2 (tokens));
									(Some (field_identifier), Some (field_accessor_identifier), None)
								},
								3 => {
									let (field_identifier, field_accessor_identifier, field_mutator_identifier) = r#try! (vec_explode_3 (tokens));
									(Some (field_identifier), Some (field_accessor_identifier), Some (field_mutator_identifier))
								},
								_ =>
									fail! (0x87ca8214),
							}
						},
						_ =>
							fail! (0xdcf687a0),
					};
					let field_identifier = if let Some (field_identifier) = field_identifier {
						match field_identifier.class_match_into () {
							ValueClassMatchInto::Symbol (field_identifier) =>
								Some (field_identifier),
							ValueClassMatchInto::Boolean (field_identifier) =>
								if ! field_identifier.value () {
									None
								} else {
									fail! (0x10a7989e);
								},
							_ =>
								fail! (0xb2676cd0),
						}
					} else {
						None
					};
					let field_accessor_identifier = if let Some (field_accessor_identifier) = field_accessor_identifier {
						match field_accessor_identifier.class_match_into () {
							ValueClassMatchInto::Symbol (field_accessor_identifier) =>
								Some (field_accessor_identifier),
							ValueClassMatchInto::Boolean (field_accessor_identifier) =>
								if ! field_accessor_identifier.value () {
									None
								} else if let (&Some (ref type_identifier), &Some (ref field_identifier)) = (&type_identifier, &field_identifier) {
									Some (symbol_new (string_concatenate! (type_identifier.string_as_str (), "-", field_identifier.string_as_str ())))
								} else {
									fail! (0xe445ce71);
								},
							_ =>
								fail! (0x7a86855f),
						}
					} else {
						None
					};
					let field_mutator_identifier = if let Some (field_mutator_identifier) = field_mutator_identifier {
						match field_mutator_identifier.class_match_into () {
							ValueClassMatchInto::Symbol (field_mutator_identifier) =>
								Some (field_mutator_identifier),
							ValueClassMatchInto::Boolean (field_mutator_identifier) =>
								if ! field_mutator_identifier.value () {
									None
								} else if let Some (ref field_accessor_identifier) = field_accessor_identifier {
									Some (symbol_new (string_concatenate! (field_accessor_identifier.string_as_str (), "-set!")))
								} else {
									fail! (0xa509987a);
								},
							_ =>
								fail! (0x63cffa4c),
						}
					} else {
						None
					};
					succeed! ((field_identifier, field_accessor_identifier, field_mutator_identifier))
				}
			);
		
		let fields_count = fields.len ();
		
		let (fields, fields_specification) = {
			let fields_specification = vec_map! (fields.iter (), &(ref field_identifier, ref _field_accessor, ref field_mutator),
					{
						let field_identifier = if let Some (ref field_identifier) = *field_identifier {
							field_identifier.clone () .into ()
						} else {
							FALSE_VALUE
						};
						let field_mutable = field_mutator.is_some () .into ();
						let field_specification = pair_immutable_new (field_identifier, field_mutable) .into ();
						field_specification
					});
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			let fields_specification = array_immutable_new (fields_specification);
			#[ cfg ( not ( feature = "vonuvoli_values_array" ) ) ]
			let fields_specification = list_collect (fields_specification.into_iter (), Some (true));
			(fields, fields_specification)
		};
		
		let constructor_fields = if let Some (constructor_fields) = constructor_fields {
			let constructor_fields = try_vec_map_into! (constructor_fields, constructor_field,
					match constructor_field.kind_match_into () {
						ValueKindMatchInto::Symbol (ref constructor_field) => {
							let field_index = fields.iter () .position (
									|&(ref field_identifier, _, _)|
										if let Some (ref field_identifier) = *field_identifier {
											Symbol::eq (field_identifier, constructor_field)
										} else {
											false
										});
							if let Some (field_index) = field_index {
								succeed! (field_index);
							} else {
								fail! (0x4794d106);
							}
						},
						ValueKindMatchInto::NumberInteger (constructor_field) => {
							let constructor_field_index = r#try! (constructor_field.try_to_usize ());
							if constructor_field_index < fields_count {
								succeed! (constructor_field_index);
							} else {
								fail! (0x2af3e21a);
							}
						},
						_ =>
							fail! (0x3c782983),
					}
				);
			Some (constructor_fields)
		} else {
			None
		};
		
		let mut statements : StdVec<Expression> = StdVec::new ();
		
		let (compilation, type_binding) = if let Some (ref type_identifier) = type_identifier {
			r#try! (compilation.define (type_identifier.clone ()))
		} else {
			r#try! (compilation.define_anonymous ())
		};
		
		{
			let expression = if let Some (ref type_identifier) = type_identifier {
				ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordKindBuild.into (), StdBox::new ([type_identifier.clone () .into (), fields_specification.into ()])) .into ()
			} else {
				ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordKindBuild.into (), StdBox::new ([fields_specification.into ()])) .into ()
			};
			let expression = r#try! (self.compile_syntax_binding_set_1 (type_binding.clone (), expression, true));
			statements.push (expression);
		}
		
		let mut compilation = compilation;
		
		if let Some (constructor_identifier) = constructor_identifier {
			let (compilation_1, constructor_binding) = r#try! (compilation.define (constructor_identifier));
			let type_binding_get = r#try! (self.compile_syntax_binding_get (type_binding.clone ()));
			let expression = if let Some (constructor_fields) = constructor_fields {
				let constructor_fields = try_vec_map_into! (constructor_fields, constructor_field, NumberInteger::try_from (constructor_field) .into_0 ());
				#[ cfg ( feature = "vonuvoli_values_array" ) ]
				let constructor_fields = array_immutable_new (constructor_fields);
				#[ cfg ( not ( feature = "vonuvoli_values_array" ) ) ]
				let constructor_fields = list_collect (constructor_fields.into_iter (), Some (true));
				ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordBuildFnN.into (), StdBox::new ([type_binding_get, constructor_fields.into ()])) .into ()
			} else {
				ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordBuildFnN.into (), StdBox::new ([type_binding_get])) .into ()
			};
			let expression = r#try! (self.compile_syntax_binding_set_1 (constructor_binding, expression, true));
			statements.push (expression);
			compilation = compilation_1;
		}
		
		if let Some (predicate_identifier) = predicate_identifier {
			let (compilation_1, predicate_binding) = r#try! (compilation.define (predicate_identifier));
			let type_binding_get = r#try! (self.compile_syntax_binding_get (type_binding.clone ()));
			let expression = ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordKindIsFn.into (), StdBox::new ([type_binding_get])) .into ();
			let expression = r#try! (self.compile_syntax_binding_set_1 (predicate_binding, expression, true));
			statements.push (expression);
			compilation = compilation_1;
		}
		
		for (field_index, &(_, ref field_accessor_identifier, ref field_mutator_identifier)) in fields.iter () .enumerate () {
			let field_index = r#try! (NumberInteger::try_from (field_index));
			if let Some (ref field_accessor_identifier) = *field_accessor_identifier {
				let (compilation_1, field_accessor_binding) = r#try! (compilation.define (field_accessor_identifier.clone ()));
				let type_binding_get = r#try! (self.compile_syntax_binding_get (type_binding.clone ()));
				let expression = ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordGetFn.into (), StdBox::new ([type_binding_get, field_index.clone () .into ()])) .into ();
				let expression = r#try! (self.compile_syntax_binding_set_1 (field_accessor_binding, expression, true));
				statements.push (expression);
				compilation = compilation_1;
			}
			#[ cfg ( feature = "vonuvoli_values_mutable" ) ]
			{ if let Some (ref field_mutator_identifier) = *field_mutator_identifier {
				let (compilation_1, field_mutator_binding) = r#try! (compilation.define (field_mutator_identifier.clone ()));
				let type_binding_get = r#try! (self.compile_syntax_binding_get (type_binding.clone ()));
				let expression = ExpressionForProcedureGenericCall::ProcedureCall (RecordPrimitiveV::RecordSetFn.into (), StdBox::new ([type_binding_get, field_index.clone () .into ()])) .into ();
				let expression = r#try! (self.compile_syntax_binding_set_1 (field_mutator_binding, expression, true));
				statements.push (expression);
				compilation = compilation_1;
			} }
			#[ cfg ( not ( feature = "vonuvoli_values_mutable" ) ) ]
			{ if let Some (_) = *field_mutator_identifier {
				fail! (0x0e100970);
			} }
		}
		
		let expression = Expression::Sequence (ExpressionSequenceOperator::ReturnLast, statements.into_boxed_slice ());
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	fn compile_syntax_quote (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		let token = r#try! (vec_explode_1 (tokens));
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, expression) = r#try! (self.compile_syntax_quote_0 (compilation, token));
		let compilation = r#try! (compilation.define_enable ());
		succeed! ((compilation, expression));
	}
	
	fn compile_syntax_quote_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		succeed! ((compilation, Expression::Value (token)));
	}
	
	
	
	
	fn compile_syntax_quasi_quote (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		let token = r#try! (vec_explode_1 (tokens));
		let compilation = r#try! (compilation.define_disable ());
		let (compilation, expression) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, 0, 0));
		let compilation = r#try! (compilation.define_enable ());
		succeed! ((compilation, expression));
	}
	
	
	#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (clippy::cyclomatic_complexity) ) ]
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
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::Character (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Symbol (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_keyword" ) ]
			ValueClassMatchInto::Keyword (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_unique" ) ]
			ValueClassMatchInto::Unique (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::String (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::Bytes (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_string" ) ]
			ValueClassMatchInto::StringRegex (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_builtins_regex" ) ]
			#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
			ValueClassMatchInto::BytesRegex (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_array" ) ]
			ValueClassMatchInto::Array (class) =>
				{ FIXME! ("add support for quasi-quotation");
				succeed! ((compilation, splice (class.value (), spliceable))) },
			
			#[ cfg ( feature = "vonuvoli_values_values" ) ]
			ValueClassMatchInto::Values (value) =>
				{ FIXME! ("add support for quasi-quotation");
				succeed! ((compilation, splice (value, spliceable))) },
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::RecordKind (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
			ValueClassMatchInto::Record (class) =>
				{ FIXME! ("add support for quasi-quotation");
				succeed! ((compilation, splice (class.value (), spliceable))) },
			
			#[ cfg ( feature = "vonuvoli_values_error" ) ]
			ValueClassMatchInto::Error (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Procedure (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			#[ cfg ( feature = "vonuvoli_builtins_filesystem" ) ]
			ValueClassMatchInto::Path (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			#[ cfg ( feature = "vonuvoli_builtins_ports" ) ]
			ValueClassMatchInto::Port (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClassMatchInto::Resource (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			ValueClassMatchInto::Internal (class) =>
				succeed! ((compilation, splice (class.value (), spliceable))),
			
			#[ cfg ( feature = "vonuvoli_values_opaque" ) ]
			ValueClassMatchInto::Opaque (value) =>
				succeed! ((compilation, splice (value, spliceable))),
			
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
			
			ValueClassMatchInto::Pair (class) => {
				
				let (callable, arguments) = r#try! (class.left_and_right_into ());
				
				let (compilation, callable) = match r#try! (self.compile_form_match_callable (compilation, callable)) {
					
					(compilation, Alternative2::Variant1 (syntax)) => {
						let tokens = r#try! (vec_list_clone (&arguments));
						let tokens_count = tokens.len ();
						match syntax {
							
							SyntaxPrimitive::PrimitiveV (SyntaxPrimitiveV::UnQuote) =>
								if tokens_count == 1 {
									let token = r#try! (vec_explode_1 (tokens));
									let (compilation, element) = if quote_depth == unquote_depth {
										r#try! (self.compile_0 (compilation, token))
									} else {
										let (compilation, element) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth, unquote_depth + 1));
										TODO! ("eliminate dynamic creation of symbol");
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
										let token = r#try! (vec_explode_1 (tokens));
										let (compilation, element) = if quote_depth == unquote_depth {
											r#try! (self.compile_0 (compilation, token))
										} else {
											let (compilation, element) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth, unquote_depth + 1));
											TODO! ("eliminate dynamic creation of symbol");
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
									let token = r#try! (vec_explode_1 (tokens));
									let (compilation, element) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, token, true, false, quote_depth + 1, unquote_depth));
									TODO! ("eliminate dynamic creation of symbol");
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
					FIXME! ("use `ListIterator`");
					match cursor.class () {
						
						ValueClass::Pair => {
							let pair = try_as_pair_immutable_ref! (cursor);
							let (compilation_1, element) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, pair.left () .clone (), false, true, quote_depth, unquote_depth));
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
							let (compilation_1, element) = r#try! (self.compile_syntax_quasi_quote_0 (compilation, cursor.clone (), false, true, quote_depth, unquote_depth));
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
					ExpressionForProcedureGenericCall::ProcedureCall (ListPrimitive2::PairCons.into (), StdBox::new ([expression, NULL.into ()])) .into ()
				};
				
				succeed! ((compilation, expression));
			},
			
		}
	}
	
	
	
	
}




pub struct CompilerContext {
	bindings : CompilerBindings,
}


impl CompilerContext {
	
	
	fn new_with_context (context : Option<&Context>) -> (CompilerContext) {
		if let Some (context) = context {
			return CompilerContext::new_with_bindings (CompilerBindings::Globals1 (context.clone (), 0));
		} else {
			return CompilerContext::new_with_bindings (CompilerBindings::None (0));
		}
	}
	
	fn new_with_bindings (bindings : CompilerBindings) -> (CompilerContext) {
		return CompilerContext {
				bindings : bindings,
			};
	}
	
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerContext>) {
		let bindings = r#try! (self.bindings.fork_locals (force));
		succeed! (CompilerContext::new_with_bindings (bindings));
	}
	
	fn fork_locals_with_bindings (self) -> (Outcome<CompilerContext>) {
		let bindings = r#try! (self.bindings.fork_locals_with_bindings ());
		succeed! (CompilerContext::new_with_bindings (bindings));
	}
	
	fn unfork_locals (self) -> (Outcome<(CompilerContext, StdVec<RegisterTemplate>)>) {
		let (bindings, registers) = r#try! (self.bindings.unfork_locals ());
		succeed! ((CompilerContext::new_with_bindings (bindings), registers));
	}
	
	
	fn resolve (self, identifier : Symbol) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = r#try! (this.bindings.resolve (identifier));
		succeed! ((this, binding));
	}
	
	fn resolve_value (self, identifier : Symbol) -> (Outcome<(CompilerContext, Option<Value>)>) {
		let mut this = self;
		let value = r#try! (this.bindings.resolve_value (identifier));
		succeed! ((this, value));
	}
	
	fn define (self, identifier : Symbol) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = r#try! (this.bindings.define (identifier));
		succeed! ((this, binding));
	}
	
	fn define_anonymous (self) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = r#try! (this.bindings.define_anonymous ());
		succeed! ((this, binding));
	}
	
	fn define_or_redefine (self, identifier : Symbol, redefine : bool) -> (Outcome<(CompilerContext, CompilerBinding)>) {
		let mut this = self;
		let binding = r#try! (this.bindings.define_or_redefine (identifier, redefine));
		succeed! ((this, binding));
	}
	
	fn define_enable (self) -> (Outcome<CompilerContext>) {
		let mut this = self;
		r#try! (this.bindings.define_enable ());
		succeed! (this);
	}
	
	fn define_disable (self) -> (Outcome<CompilerContext>) {
		let mut this = self;
		r#try! (this.bindings.define_disable ());
		succeed! (this);
	}
}




#[ allow (variant_size_differences) ] // OK
pub enum CompilerBindings {
	None (usize),
	Globals1 (Context, usize),
	Globals2 (StdBox<CompilerBindings>, Context, usize),
	Locals (StdBox<CompilerBindings>, StdMap<Symbol, CompilerBinding>, StdMap<Symbol, CompilerBinding>, StdVec<RegisterTemplate>, bool, usize),
}


#[ derive ( Clone ) ] // OK ??
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub enum CompilerBinding {
	Undefined,
	Binding (Option<Symbol>, Binding, Option<BindingTemplate>),
	Register (Option<Symbol>, usize, RegisterTemplate),
}


impl CompilerBindings {
	
	
	fn fork_locals (self, force : bool) -> (Outcome<CompilerBindings>) {
		if force {
			succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdMap::new (), StdVec::new (), false, 0));
		} else {
			match self {
				CompilerBindings::None (_) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None), 0)),
				CompilerBindings::Globals1 (_, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None), 0)),
				CompilerBindings::Globals2 (_, _, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None), 0)),
				CompilerBindings::Locals (_, _, _, _, _, _) =>
					succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdMap::new (), StdVec::new (), false, 0)),
			}
		}
	}
	
	fn fork_locals_with_bindings (self) -> (Outcome<CompilerBindings>) {
		succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdMap::new (), StdVec::new (), true, 0));
	}
	
	
	fn unfork_locals (self) -> (Outcome<(CompilerBindings, StdVec<RegisterTemplate>)>) {
		match self {
			CompilerBindings::None (_) =>
				fail_panic! (0x98657e5a, github_issue_new),
			CompilerBindings::Globals1 (_, _) =>
				fail_panic! (0xdd470d36, github_issue_new),
			CompilerBindings::Globals2 (parent, _, _) =>
				succeed! ((*parent, StdVec::new ())),
			CompilerBindings::Locals (parent, _, _, registers, _, _) =>
				succeed! ((*parent, registers)),
		}
	}
	
	
	fn resolve (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		if str::ne (identifier.string_as_str (), "_") {
			return self.resolve_0 (identifier, false);
		} else {
			fail! (0x42267a32);
		}
	}
	
	fn resolve_0 (&mut self, identifier : Symbol, force_binding : bool) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None (_) =>
				succeed! (CompilerBinding::Undefined),
			CompilerBindings::Globals1 (ref context, _) =>
				if let Some (binding) = r#try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (Some (identifier), binding, None));
				} else {
					succeed! (CompilerBinding::Undefined);
				},
			CompilerBindings::Globals2 (ref mut parent, ref context, _) =>
				if let Some (binding) = r#try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (Some (identifier), binding, None));
				} else {
					return parent.resolve_0 (identifier, force_binding);
				},
			CompilerBindings::Locals (ref mut parent, _, ref mut cached, ref mut registers, _, _) => {
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
					let binding = CompilerBinding::Register (Some (identifier.clone ()), index, template.clone ());
					registers[index] = template;
					cached.insert (identifier, binding.clone ());
					succeed! (binding);
				}
				{
					match r#try! (parent.resolve_0 (identifier.clone (), true)) {
						CompilerBinding::Undefined =>
							succeed! (CompilerBinding::Undefined),
						binding @ CompilerBinding::Binding (_, _, _) => {
							cached.insert (identifier, binding.clone ());
							succeed! (binding);
						},
						CompilerBinding::Register (_, parent_index, _) => {
							let self_index = registers.len ();
							let template = RegisterTemplate::Borrow (parent_index);
							let self_binding = CompilerBinding::Register (Some (identifier.clone ()), self_index, template.clone ());
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
		match r#try! (self.resolve (identifier)) {
			CompilerBinding::Binding (_, binding, _) =>
				succeed! (r#try! (binding.get_option ())),
			CompilerBinding::Register (_, _, _) =>
				succeed! (None),
			CompilerBinding::Undefined =>
				succeed! (None),
		}
	}
	
	
	fn define (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		if str::ne (identifier.string_as_str (), "_") {
			return self.define_0 (Some ((identifier, false)));
		} else {
			return self.define_0 (None);
		}
	}
	
	fn define_anonymous (&mut self) -> (Outcome<CompilerBinding>) {
		return self.define_0 (None);
	}
	
	fn define_or_redefine (&mut self, identifier : Symbol, redefine : bool) -> (Outcome<CompilerBinding>) {
		if str::ne (identifier.string_as_str (), "_") {
			return self.define_0 (Some ((identifier, redefine)));
		} else {
			return self.define_0 (None);
		}
	}
	
	fn define_0 (&mut self, identifier : Option<(Symbol, bool)>) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None (_) =>
				fail! (0xd943456d),
			CompilerBindings::Globals1 (ref context, define_allowed) | CompilerBindings::Globals2 (_, ref context, define_allowed) => {
				if define_allowed > 0 {
					fail! (0x0d9133ab);
				}
				let (identifier, redefine) = if let Some ((identifier, redefine)) = identifier {
					(Some (identifier), redefine)
				} else {
					(None, false)
				};
				let template = BindingTemplate {
						identifier : identifier.clone (),
						value : None,
						immutable : false,
					};
				let binding = if identifier.is_some () {
					if redefine {
						fail_panic! (0x6daa843d, github_issue_new);
					}
					r#try! (context.define (&template))
				} else {
					Binding::new_from_template (&template)
				};
				succeed! (CompilerBinding::Binding (identifier, binding, Some (template)));
			},
			CompilerBindings::Locals (_, ref mut locals, ref mut cached, ref mut registers, force_binding, define_allowed) => {
				if define_allowed > 0 {
					fail! (0xaf5074cb);
				}
				let (identifier, redefine) = if let Some ((identifier, redefine)) = identifier {
					(Some (identifier), redefine)
				} else {
					(None, false)
				};
				let index = registers.len ();
				let template = if force_binding {
					let template = BindingTemplate {
							identifier : identifier.clone (),
							value : None,
							immutable : false,
						};
					RegisterTemplate::LocalBinding (template)
				} else {
					RegisterTemplate::LocalValue (None, false)
				};
				let binding = CompilerBinding::Register (identifier.clone (), index, template.clone ());
				registers.push (template);
				if let Some (identifier) = identifier {
					if redefine {
						locals.remove (&identifier);
						cached.remove (&identifier);
					} else {
						if locals.contains_key (&identifier) {
							fail! (0x97d3fa0c);
						}
					}
					locals.insert (identifier.clone (), binding.clone ());
					cached.insert (identifier.clone (), binding.clone ());
				}
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
				if let Some (ref identifier) = *identifier {
					return self.undefine_0 (identifier);
				} else {
					succeed! (());
				},
		}
	}
	
	fn undefine_0 (&mut self, identifier : &Symbol) -> (Outcome<()>) {
		match *self {
			CompilerBindings::None (_) =>
				fail_panic! (0x3a1f3306, github_issue_new),
			CompilerBindings::Globals1 (_, _) =>
				fail_panic! (0xcdf142d8, github_issue_new),
			CompilerBindings::Globals2 (_, _, _) =>
				fail_panic! (0x382ba35e, github_issue_new),
			CompilerBindings::Locals (_, ref mut locals, ref mut cached, _, _, _) => {
				locals.remove (identifier);
				cached.remove (identifier);
				succeed! (());
			},
		}
	}
	
	
	fn define_disable (&mut self) -> (Outcome<()>) {
		let define_allowed = match *self {
			CompilerBindings::None (ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Globals1 (_, ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Globals2 (_, _, ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Locals (_, _, _, _, _, ref mut define_allowed) =>
				define_allowed,
		};
		*define_allowed += 1;
		succeed! (());
	}
	
	fn define_enable (&mut self) -> (Outcome<()>) {
		let define_allowed = match *self {
			CompilerBindings::None (ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Globals1 (_, ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Globals2 (_, _, ref mut define_allowed) =>
				define_allowed,
			CompilerBindings::Locals (_, _, _, _, _, ref mut define_allowed) =>
				define_allowed,
		};
		if *define_allowed == 0 {
			fail_panic! (0x987257ca, github_issue_new);
		}
		*define_allowed -= 1;
		succeed! (());
	}
}




#[ derive ( Clone, Default ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK ??
pub struct CompilerConfiguration {
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_input : Option<bool>,
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_output : Option<bool>,
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub trace_error : Option<bool>,
}


impl CompilerConfiguration {
	
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub fn should_trace_input (&self) -> (bool) {
		self.trace_input.unwrap_or (COMPILER_TRACE_INPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub fn should_trace_output (&self) -> (bool) {
		self.trace_output.unwrap_or (COMPILER_TRACE_OUTPUT)
	}
	
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub fn should_trace_error (&self) -> (bool) {
		self.trace_error.unwrap_or (COMPILER_TRACE_ERROR)
	}
	
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub fn should_trace_output_or_error (&self) -> (bool) {
		self.should_trace_output () || self.should_trace_error ()
	}
	
	#[ cfg ( feature = "vonuvoli_compiler_trace_enabled" ) ]
	#[ cfg ( feature = "vonuvoli_transcript" ) ]
	pub fn is_trace_enabled (&self) -> (bool) {
		self.should_trace_input () || self.should_trace_output () || self.should_trace_error ()
	}
}

