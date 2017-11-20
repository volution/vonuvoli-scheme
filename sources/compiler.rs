

use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::operators::exports::*;
use super::primitives::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::compile;
	pub use super::Compiler;
}




pub fn compile (context : &Context, value : &Value) -> (Outcome<Expression>) {
	return Compiler::new () .compile (context, value);
}




pub struct Compiler {}


impl Compiler {
	
	
	
	
	pub fn new () -> (Compiler) {
		return Compiler {};
	}
	
	pub fn compile (&self, context : &Context, value : &Value) -> (Outcome<Expression>) {
		let compilation = CompilerContext::new (CompilerBindings::Globals1 (context.clone ()));
		let (_compilation, expression) = try! (self.compile_0 (compilation, value.clone ()));
		succeed! (expression);
	}
	
	
	
	
	fn compile_0 (&self, compilation : CompilerContext, value : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		match value.class () {
			
			ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
				succeed! ((compilation, value.into ())),
			ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
				succeed! ((compilation, value.into ())),
			ValueClass::String | ValueClass::Bytes =>
				succeed! ((compilation, value.into ())),
			
			ValueClass::Symbol =>
				return self.compile_symbol (compilation, value.into ()),
			ValueClass::Pair =>
				return self.compile_form (compilation, value.into ()),
			ValueClass::Array =>
				fail_unimplemented! (0xe7db25d8),
			
			ValueClass::Error =>
				fail! (0x2aa7bc60),
			ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
				fail! (0xaf6f1288),
			
			ValueClass::Binding | ValueClass::Context =>
				fail! (0x5f0d7003),
			
			ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
				fail! (0x841d4d00),
			
		}
	}
	
	
	
	
	fn compile_vec (&self, compilation : CompilerContext, values : ValueVec) -> (Outcome<(CompilerContext, ExpressionVec)>) {
		let mut expressions = ExpressionVec::with_capacity (values.len ());
		let mut compilation = compilation;
		for value in values.into_iter () {
			let (compilation_1, expression) = try! (self.compile_0 (compilation, value));
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
				succeed! ((compilation, Expression::BindingGet (binding))),
			CompilerBinding::Register (index) =>
				succeed! ((compilation, Expression::RegisterGet (index))),
		}
	}
	
	
	
	
	fn compile_form (&self, compilation : CompilerContext, form : Pair) -> (Outcome<(CompilerContext, Expression)>) {
		
		match try! (self.compile_form_1 (compilation, form.clone ())) {
			
			(compilation, Some ((primitive, arguments))) =>
				return self.compile_syntax_call (compilation, primitive, arguments),
			
			(compilation, None) =>
				return self.compile_procedure_call (compilation, form.left () .clone (), form.right () .clone ()),
		}
	}
	
	
	fn compile_form_1 (&self, compilation : CompilerContext, value : Pair) -> (Outcome<(CompilerContext, Option<(SyntaxPrimitive, Value)>)>) {
		
		let mut compilation = compilation;
		let callable = value.left () .clone ();
		let arguments = value.right () .clone ();
		
		match callable.class () {
			
			ValueClass::Symbol => {
				if let Some (callable) = try! (compilation.bindings.resolve_value (callable.into ())) {
					match callable.class () {
						ValueClass::SyntaxPrimitive =>
							succeed! ((compilation, Some ((callable.into (), arguments.clone ())))),
						_ =>
							succeed! ((compilation, None)),
					}
				} else {
					succeed! ((compilation, None));
				}
			},
			
			ValueClass::SyntaxPrimitive =>
				succeed! ((compilation, Some ((callable.clone () .into (), arguments.clone ())))),
			
			_ =>
				succeed! ((compilation, None)),
			
		}
	}
	
	
	
	
	fn compile_procedure_call (&self, compilation : CompilerContext, procedure : Value, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		let (compilation, procedure) = try! (self.compile_0 (compilation, procedure));
		let (compilation, arguments) = try! (self.compile_vec (compilation, try! (vec_clone_list (&arguments))));
		succeed! ((compilation, Expression::ProcedureCall (procedure.into (), arguments)));
	}
	
	
	
	
	fn compile_syntax_call (&self, compilation : CompilerContext, syntax : SyntaxPrimitive, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let mut compilation = compilation;
		let arguments = try! (vec_clone_list (&arguments));
		let arguments_count = arguments.len ();
		
		match syntax {
			
			SyntaxPrimitive::Primitive1 (syntax) =>
				if arguments_count == 1 {
					let arguments = vec_explode_1! (arguments);
					match syntax {
						
						SyntaxPrimitive1::Quote =>
							succeed! ((compilation, Expression::Value (arguments.clone ()))),
						
						SyntaxPrimitive1::QuasiQuote =>
							return self.compile_syntax_quasy_quote_value (compilation, arguments, false),
						
						SyntaxPrimitive1::UnQuote | SyntaxPrimitive1::UnQuoteSplicing =>
							fail! (0x99b4857b),
						
					}
				} else {
					fail! (0x421da1f1);
				},
			
			SyntaxPrimitive::Primitive2 (syntax) =>
				if arguments_count == 2 {
					match syntax {
						
						SyntaxPrimitive2::Define => {
							let (identifier, value) = vec_explode_2! (arguments);
							match identifier.class () {
								ValueClass::Symbol =>
									match try! (compilation.bindings.define (try_into_symbol! (identifier))) {
										CompilerBinding::Undefined =>
											fail! (0x1e75333d),
										CompilerBinding::Binding (binding) => {
											let (compilation, value) = try! (self.compile_0 (compilation, value));
											succeed! ((compilation, Expression::BindingInitialize (binding, value.into ())));
										},
										CompilerBinding::Register (index) => {
											let (compilation, value) = try! (self.compile_0 (compilation, value));
											succeed! ((compilation, Expression::RegisterInitialize (index, value.into ())));
										},
									},
								ValueClass::Pair =>
									fail_unimplemented! (0xfc72467c),
								_ =>
									fail! (0x404d24c7),
							}
						},
						
						SyntaxPrimitive2::DefineValues =>
							fail_unimplemented! (0x2f87acf0),
						
						SyntaxPrimitive2::Set =>
							fail_unimplemented! (0x19d7ebad),
						
						SyntaxPrimitive2::SetValues =>
							fail_unimplemented! (0xeea43320),
						
					}
				} else {
					fail! (0x9d9b6a94);
				},
			
			SyntaxPrimitive::Primitive3 (syntax) =>
				if arguments_count == 3 {
					match syntax {
						
						SyntaxPrimitive3::If => {
							let (compilation, arguments) = try! (self.compile_vec (compilation, arguments));
							succeed! ((compilation, Expression::SyntaxPrimitiveCall (SyntaxPrimitive3::If.into (), arguments)));
						},
						
					}
				} else {
					fail! (0xd76f0ad2);
				},
			
			SyntaxPrimitive::PrimitiveN (syntax) =>
				match syntax {
					
					SyntaxPrimitiveN::Begin => {
						let (compilation, arguments) = try! (self.compile_vec (compilation, arguments));
						succeed! ((compilation, Expression::Sequence (arguments)));
					},
					
					SyntaxPrimitiveN::And | SyntaxPrimitiveN::Or => {
						let (compilation, arguments) = try! (self.compile_vec (compilation, arguments));
						succeed! ((compilation, Expression::SyntaxPrimitiveCall (syntax.into (), arguments)));
					},
					
					SyntaxPrimitiveN::When | SyntaxPrimitiveN::Unless =>
						if arguments_count >= 2 {
							let (compilation, arguments) = try! (self.compile_vec (compilation, arguments));
							succeed! ((compilation, Expression::SyntaxPrimitiveCall (syntax.into (), arguments)));
						} else {
							fail! (0x3c364a9f);
						},
					
					SyntaxPrimitiveN::Locals =>
						return self.compile_syntax_locals (compilation, arguments),
					
					_ =>
						fail_unimplemented! (0x73d95eb5),
					
				},
			
			SyntaxPrimitive::Auxiliary =>
				fail! (0x1aed14f3),
			
			SyntaxPrimitive::Reserved =>
				fail! (0x1aed14f3),
			
			SyntaxPrimitive::Unimplemented =>
				fail_unimplemented! (0xa4e41f62),
			
		}
	}
	
	
	
	
	fn compile_syntax_locals (&self, compilation : CompilerContext, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		let compilation = try! (compilation.fork_locals (true));
		let (compilation, statements) = try! (self.compile_vec (compilation, statements));
		let (compilation, registers) = try! (compilation.unfork_locals ());
		let registers_count = registers.len ();
		let statements = Expression::Sequence (statements);
		if registers_count == 0 {
			succeed! ((compilation, statements));
		} else {
			succeed! ((compilation, Expression::RegisterClosure (statements.into (), registers)));
		}
	}
	
	
	
	
	fn compile_syntax_quasy_quote_value (&self, compilation : CompilerContext, value : Value, spliceable : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		fn splice <ExpressionInto : StdInto<Expression>> (expression : ExpressionInto, spliceable : bool) -> (Expression) {
			let expression = expression.into ();
			if spliceable {
				Expression::ProcedureCall (ListPrimitive1::List.into (), vec! [expression])
			} else {
				expression
			}
		}
		
		match value.class () {
			
			ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
				succeed! ((compilation, splice (value, spliceable))),
			ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
				succeed! ((compilation, splice (value, spliceable))),
			ValueClass::String | ValueClass::Bytes =>
				succeed! ((compilation, splice (value, spliceable))),
			
			ValueClass::Symbol =>
				succeed! ((compilation, splice (value, spliceable))),
			ValueClass::Array =>
				fail_unimplemented! (0x0d99c57b),
			
			ValueClass::Error =>
				fail! (0x9681733a),
			ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
				fail! (0x251a7fd0),
			
			ValueClass::Binding | ValueClass::Context =>
				fail! (0xfa7ef6f6),
			
			ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
				fail! (0x841d4d00),
			
			ValueClass::Pair => {
				let compilation = match try! (self.compile_form_1 (compilation, value.clone () .into ())) {
					
					(compilation, Some ((primitive, arguments))) => {
						let arguments = try! (vec_clone_list (&arguments));
						let arguments_count = arguments.len ();
						match primitive {
							
							SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuote) =>
								if arguments_count == 1 {
									let arguments = vec_explode_1! (arguments);
									let (compilation, element) = try! (self.compile_syntax_quasy_quote_value (compilation, arguments, false));
									succeed! ((compilation, splice (element, spliceable)));
								} else {
									fail! (0x9dc44267);
								},
							
							SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuoteSplicing) =>
								if arguments_count == 1 {
									if spliceable {
										let arguments = vec_explode_1! (arguments);
										let (compilation, element) = try! (self.compile_syntax_quasy_quote_value (compilation, arguments, false));
										succeed! ((compilation, element));
									} else {
										fail! (0x47356961);
									}
								} else {
									fail! (0xe0c45124);
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
				let mut cursor = &value;
				loop {
					match cursor.class () {
						
						ValueClass::Pair => {
							let pair = cursor.as_ref () as &Pair;
							let (compilation_1, element) = try! (self.compile_syntax_quasy_quote_value (compilation, pair.left () .clone (), true));
							compilation = compilation_1;
							elements.push (element);
							cursor = pair.right ();
						},
						
						ValueClass::Null =>
							break,
						
						_ => {
							let (compilation_1, element) = try! (self.compile_syntax_quasy_quote_value (compilation, cursor.clone (), true));
							compilation = compilation_1;
							elements.push (element);
							break;
						},
						
					}
				}
				
				succeed! ((compilation, Expression::ProcedureCall (ListPrimitiveN::Append.into (), elements)));
			},
			
		}
	}
	
	
}




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
	
	fn unfork_locals (self) -> (Outcome<(CompilerContext, StdVec<Option<usize>>)>) {
		let (bindings, registers) = try! (self.bindings.unfork_locals ());
		succeed! ((CompilerContext::new (bindings), registers));
	}
}




#[ allow (dead_code) ]
enum CompilerBindings {
	None,
	Globals1 (Context),
	Globals2 (StdBox<CompilerBindings>, Context),
	Locals (StdBox<CompilerBindings>, StdMap<Symbol, CompilerBinding>, StdVec<Option<usize>>, usize),
}


#[ derive (Clone) ]
#[ allow (dead_code) ]
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
	
	fn unfork_locals (self) -> (Outcome<(CompilerBindings, StdVec<Option<usize>>)>) {
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
			CompilerBindings::Locals (ref mut parent, ref mut locals, ref mut registers, _depth) => {
				if let Some (binding) = locals.get (&identifier) {
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
							registers.push (Some (parent_index));
							locals.insert (identifier, self_binding.clone ());
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
			CompilerBindings::Globals1 (ref context) => {
				let binding = try! (context.define (&identifier));
				succeed! (CompilerBinding::Binding (binding));
			},
			CompilerBindings::Globals2 (ref _parent, ref context) => {
				let binding = try! (context.define (&identifier));
				succeed! (CompilerBinding::Binding (binding));
			},
			CompilerBindings::Locals (ref _parent, ref mut locals, ref mut registers, _depth) => {
				let index = registers.len ();
				let binding = CompilerBinding::Register (index);
				registers.push (None);
				locals.insert (identifier, binding.clone ());
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

